//! Error type shared by the emission core, the gate, and the CLI.

use std::fmt;
use std::path::PathBuf;

use poodle_ir::Finding;

/// Result alias for the crate.
pub type Result<T> = std::result::Result<T, CodegenError>;

/// Every failure mode of the emitter and its gate.
#[derive(Debug)]
pub enum CodegenError {
    /// The fixture could not be read (missing file, permission).
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    /// The fixture is not valid JSON for the current IR schema.
    Malformed { path: PathBuf, detail: String },
    /// The fixture deserialized but failed model validation. Carries every
    /// finding at once, mirroring `poodle-ir`'s all-findings `validate`.
    Invalid {
        path: PathBuf,
        findings: Vec<Finding>,
    },
    /// A declared reference in the model cannot be resolved during emission.
    /// Reached only via a model that skipped validation — a validated model
    /// is guaranteed resolvable.
    UnresolvedReference { what: String },
    /// The filesystem refused an emission write.
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
    /// A target id that is not registered.
    UnknownTarget { id: String, known: Vec<String> },
    /// Check mode found drift: the message is the full report, every
    /// finding at once.
    Gate { message: String },
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::Read { path, source } => {
                write!(f, "cannot read {}: {source}", path.display())
            }
            CodegenError::Malformed { path, detail } => {
                write!(f, "{} is not valid IR JSON: {detail}", path.display())
            }
            CodegenError::Invalid { path, findings } => {
                write!(f, "{} failed IR validation:", path.display())?;
                for finding in findings {
                    write!(
                        f,
                        "\n  - [{:?}] {}: {}",
                        finding.kind, finding.identifier, finding.message
                    )?;
                }
                Ok(())
            }
            CodegenError::UnresolvedReference { what } => {
                write!(f, "emission cannot resolve {what}")
            }
            CodegenError::Write { path, source } => {
                write!(f, "cannot write {}: {source}", path.display())
            }
            CodegenError::UnknownTarget { id, known } => {
                write!(
                    f,
                    "unknown target '{id}'; known targets: {}",
                    known.join(", ")
                )
            }
            CodegenError::Gate { message } => f.write_str(message),
        }
    }
}

impl std::error::Error for CodegenError {}
