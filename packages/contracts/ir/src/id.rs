//! Identity — stable identifiers, component layer, and contract references.
//!
//! Serves `CROSS-01` (component identity: name, layer, contract reference,
//! and a stable identifier the IR and every generated artifact can cite) and
//! `CROSS-02` (stable identifiers for props), per spec 063 "Component IR"
//! ("Each component definition carries stable identifiers for: public props,
//! defaults, types, controlled state, events, slots, and parts").

use serde::{Deserialize, Serialize};

/// Stable identifier for a component, scene, shared type, part, prop, event,
/// attribute, or state (`CROSS-01`, `CROSS-02`; spec 063 "stable
/// identifiers"). Identifiers are the citation surface for every generated
/// artifact and for validation findings.
///
/// Serializes transparently as a plain string. Uniqueness and format are
/// enforced by [`validate`](crate::validate), not by construction, so
/// hand-authored and machine-read definitions share one rule set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identifier(String);

impl Identifier {
    /// Builds an identifier from a string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the identifier string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Component layer. The corpus names exactly one layer for the pilot
/// components — `foundation` (`CROSS-01`). Kept as an enum rather than a
/// free string so layers stay a closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layer {
    /// Foundation-layer component (`CROSS-01`; the layer of Button,
    /// RangeSlider, and TextInput).
    #[serde(rename = "foundation")]
    Foundation,
}

/// Reference to a governing contract document section (`CROSS-01`; the
/// contracts `B/R/T §1` name each component's contract; `S063 "Component IR"`
/// requires a "contract reference").
///
/// The path is repo-relative and the section follows the contract's own
/// numbering, e.g. `docs/contracts/components/button.md` §3.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractRef {
    /// Repo-relative path of the contract document, e.g.
    /// `docs/contracts/components/button.md` (`CROSS-01`).
    pub path: String,
    /// Section number or name within the contract, e.g. `"§3"` or
    /// `"Portable Spec"` (`CROSS-01`, `CROSS-02`).
    pub section: Option<String>,
}

impl ContractRef {
    /// Builds a contract reference from a path and optional section.
    pub fn new(path: impl Into<String>, section: Option<impl Into<String>>) -> Self {
        Self {
            path: path.into(),
            section: section.map(|s| s.into()),
        }
    }
}
