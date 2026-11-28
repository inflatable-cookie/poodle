use std::collections::{BTreeMap, BTreeSet};
use std::{error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{cases, vocabulary_fingerprint, Backend, Case, Proof, Reading};

const SCHEMA: &str = "poodle-node-conformance.v1";

/// Return the lower-case SHA-256 identity of the supplied bytes.
pub fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Reproducible identity of a backend execution.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub poodle_commit: String,
    pub backend_commit: String,
    pub lock_sha256: String,
    pub platform: String,
}

/// A case verdict. Unsupported and pending probes do not earn a pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pass,
    Fail,
    Unsupported,
    NotRun,
}

/// The verdict and observed values for one case, in fixture check order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultEntry {
    pub id: String,
    pub proof: Proof,
    pub status: Status,
    pub observation: Value,
}

impl Case {
    /// Execute the case against mounted backend output. Dispatch is pending.
    pub fn run(&self, backend: &mut impl Backend) -> ResultEntry {
        let (status, observation) = if self.actions.is_empty() {
            backend.mount(&self.root, self.viewport);
            let mut status = Status::Pass;
            let mut observations = Vec::new();
            for check in &self.checks {
                match backend.read(check.target, check.probe) {
                    Reading::Observed(actual) if check.matches(&actual) => {
                        observations.push(actual);
                    }
                    Reading::Observed(actual) => {
                        status = Status::Fail;
                        observations.push(json!({
                            "target": check.target, "probe": check.probe,
                            "expected": check.expected, "actual": actual,
                        }));
                    }
                    Reading::Unsupported(reason) => {
                        if status == Status::Pass {
                            status = Status::Unsupported;
                        }
                        observations.push(json!({
                            "target": check.target, "probe": check.probe, "unsupported": reason,
                        }));
                    }
                }
            }
            (status, Value::Array(observations))
        } else {
            (Status::NotRun, json!("dispatch is outside the pure pilot"))
        };
        ResultEntry {
            id: self.id.clone(),
            proof: self.proof,
            status,
            observation,
        }
    }
}

/// The versioned result for the complete, pinned case manifest.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    schema: String,
    vocabulary_sha256: String,
    case_manifest_sha256: String,
    identity: Identity,
    results: Vec<ResultEntry>,
}

/// Failure to load or validate a conformance receipt.
#[derive(Debug)]
pub enum ReceiptError {
    Json(serde_json::Error),
    Invalid(String),
}

impl fmt::Display for ReceiptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(f, "invalid receipt json: {error}"),
            Self::Invalid(message) => write!(f, "invalid receipt: {message}"),
        }
    }
}

impl error::Error for ReceiptError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Invalid(_) => None,
        }
    }
}

impl From<serde_json::Error> for ReceiptError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

fn manifest_digest(corpus: &[Case]) -> String {
    let manifest: Vec<_> = corpus
        .iter()
        .map(|case| {
            (
                &case.id,
                case.proof,
                case.viewport,
                &case.checks,
                &case.actions,
                case.expected_events,
            )
        })
        .collect();
    // The Rust fixture source binds tree inputs and recording closures
    // without inventing a second serialization format for Node.
    let fixture_source = digest(include_bytes!("fixtures.rs"));
    digest(
        &serde_json::to_vec(&(manifest, fixture_source))
            .expect("serialize the authored case manifest"),
    )
}

impl Receipt {
    /// Stamp results against the current corpus and reject incomplete input.
    ///
    /// # Errors
    /// Returns an error if provenance, case IDs, or proof levels are invalid.
    pub fn new(identity: Identity, results: Vec<ResultEntry>) -> Result<Self, ReceiptError> {
        let corpus: Vec<_> = cases().collect();
        let receipt = Self {
            schema: SCHEMA.into(),
            vocabulary_sha256: vocabulary_fingerprint(),
            case_manifest_sha256: manifest_digest(&corpus),
            identity,
            results,
        };
        receipt.validate(&corpus)?;
        Ok(receipt)
    }

    /// Decode a receipt and validate it against this kit's pinned manifest.
    ///
    /// # Errors
    /// Returns an error on malformed JSON, stale stamps, or invalid case coverage.
    pub fn from_json(bytes: &[u8]) -> Result<Self, ReceiptError> {
        let receipt: Self = serde_json::from_slice(bytes)?;
        receipt.validate(&cases().collect::<Vec<_>>())?;
        Ok(receipt)
    }

    /// Serialize the validated receipt for storage or exchange.
    ///
    /// # Errors
    /// Returns an error if JSON serialization fails.
    pub fn to_json(&self) -> Result<String, ReceiptError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Case evidence in the order supplied by the runner.
    pub fn results(&self) -> &[ResultEntry] {
        &self.results
    }

    fn validate(&self, corpus: &[Case]) -> Result<(), ReceiptError> {
        let invalid = |message| Err(ReceiptError::Invalid(message));
        if self.schema != SCHEMA
            || self.vocabulary_sha256 != vocabulary_fingerprint()
            || self.case_manifest_sha256 != manifest_digest(corpus)
        {
            return invalid("schema, vocabulary or case manifest does not match this kit".into());
        }
        let hex = |value: &str, length| {
            value.len() == length
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        };
        if !hex(&self.identity.poodle_commit, 40)
            || !hex(&self.identity.backend_commit, 40)
            || !hex(&self.identity.lock_sha256, 64)
            || self.identity.platform.trim().is_empty()
        {
            return invalid("full commits, lock SHA-256 and platform are required".into());
        }
        let expected: BTreeMap<_, _> = corpus
            .iter()
            .map(|case| (case.id.as_str(), case.proof))
            .collect();
        let mut seen = BTreeSet::new();
        for entry in &self.results {
            let Some(proof) = expected.get(entry.id.as_str()) else {
                return invalid(format!("unknown case {}", entry.id));
            };
            if !seen.insert(entry.id.as_str()) {
                return invalid(format!("duplicate case {}", entry.id));
            }
            if *proof != entry.proof {
                return invalid(format!("wrong proof level for {}", entry.id));
            }
        }
        if let Some(missing) = expected.keys().find(|id| !seen.contains(*id)) {
            return invalid(format!("missing case {missing}"));
        }
        Ok(())
    }
}
