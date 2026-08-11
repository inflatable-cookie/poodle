//! Versioned Rust-authored component and scene IR — data and validation only.
//!
//! Serves [`docs/specs/063-rust-authored-component-and-scene-ir.md`] (`IR-01`
//! Rust authority, `IR-02` serializable boundary, `IR-04` semantic authoring
//! layer above `poodle-node`, `IR-06` VisualState purity, `IR-07` deterministic
//! and versioned artifacts). The vocabulary is the 129 requirements of
//! `docs/roadmaps/g13/pilot-expressiveness-corpus.md`; every public type and
//! field doc-comments the corpus requirement ID (`CROSS-*`, `BTN-*`, `RNG-*`,
//! `TXT-*`, `SHELL-*`) or contract section it serves.
//!
//! Hard boundary (`NEG-01`–`NEG-08`): this crate holds typed, serializable
//! declarations only. No executable Rust is cross-compiled, no framework or
//! runtime type appears here, and no code generation or emission exists.
//! Cross-runtime behavior is expressed as declarative intents, conformance
//! vectors, adapter capabilities, or documented extensions — the four
//! representation slots named by spec 063's "Hard Boundary".
//!
//! # Schema version
//!
//! [`IR_SCHEMA_VERSION`] is the current serialization version. Every
//! [`IrModel`] carries a `schema_version` field that validation checks
//! against it; breaking changes require a migration or a deliberate pre-1.0
//! regeneration (`IR-07`).

mod accessibility;
mod attributes;
mod axes;
mod capabilities;
mod component;
mod conformance;
mod events;
mod expr;
mod id;
mod keyboard;
mod parts;
mod props;
mod scenes;
mod shared;
mod state;
mod tokens;
mod validation;
mod visual;

pub use accessibility::*;
pub use attributes::*;
pub use axes::*;
pub use capabilities::*;
pub use component::*;
pub use conformance::*;
pub use events::*;
pub use expr::*;
pub use id::*;
pub use keyboard::*;
pub use parts::*;
pub use props::*;
pub use scenes::*;
pub use shared::*;
pub use state::*;
pub use tokens::*;
pub use validation::*;
pub use visual::*;

use serde::{Deserialize, Serialize};

/// Serialization schema version for the IR (`IR-07`; spec 063 "Authoring
/// Form — every emitted artifact carries an IR version"). Breaking changes
/// require a migration or a deliberate pre-1.0 regeneration.
pub const IR_SCHEMA_VERSION: u32 = 1;

/// Versioned IR model — the serializable root holding the shared-type layer,
/// component definitions, conformance vectors, and scenes.
///
/// Serves the shared declarative definition vocabulary (`SDD`) of the
/// expressiveness corpus: shared types referenced by many components
/// (g13-b003 R6.1; spec 063 "Shared types and permitted subsets"),
/// component definitions (`CROSS-01`), conformance vectors (`CROSS-18`), and
/// specimen/scene definitions (`CROSS-21`, `SHELL-*`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrModel {
    /// Serialization schema version; must equal [`IR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// First-class shared enumerated types, defined once and referenced by
    /// many components (g13-b003 R6.1; spec 063 "Shared types and permitted
    /// subsets").
    pub shared_types: Vec<SharedType>,
    /// Component definitions (`CROSS-01`; spec 063 "Component IR").
    pub components: Vec<ComponentDefinition>,
    /// Shared conformance vectors executed by each runtime machine
    /// (`CROSS-18`; spec 063 "shared conformance vectors").
    pub conformance_vectors: Vec<ConformanceVector>,
    /// Specimen/scene definitions (`CROSS-21`; `B/R/T §13`; spec 063
    /// "Scene IR").
    pub scenes: Vec<Scene>,
    /// Per-shell specimen registry wiring (`SHELL-10`).
    pub specimen_registry: Option<SpecimenRegistry>,
}

impl IrModel {
    /// Returns the shared type with the given id, if any.
    pub fn shared_type(&self, id: &str) -> Option<&SharedType> {
        self.shared_types.iter().find(|t| t.id.as_str() == id)
    }

    /// Returns the component definition with the given id, if any.
    pub fn component(&self, id: &str) -> Option<&ComponentDefinition> {
        self.components.iter().find(|c| c.id.as_str() == id)
    }

    /// Returns the scene with the given id, if any.
    pub fn scene(&self, id: &str) -> Option<&Scene> {
        self.scenes.iter().find(|s| s.id.as_str() == id)
    }

    /// Returns the conformance vector with the given id, if any.
    pub fn conformance_vector(&self, id: &str) -> Option<&ConformanceVector> {
        self.conformance_vectors
            .iter()
            .find(|v| v.id.as_str() == id)
    }

    /// Validates the whole model, returning **all** findings at once.
    ///
    /// See [`validate`] for the rule set.
    pub fn validate(&self) -> Vec<Finding> {
        validate(self)
    }
}
