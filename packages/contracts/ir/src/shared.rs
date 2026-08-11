//! First-class shared enumerated types and per-component permitted subsets.
//!
//! This module is the reason the card exists. `g13-b003` R6.1 requires a
//! shared-type layer: a named enumerated type used by more than one component
//! is defined **once** and referenced, instead of being fragmented per
//! contract (`ButtonTone`, `OverlayPlacement`, and eight further enumerated
//! shared types found with no canonical definition). R6.2 requires a
//! component to declare the **subset** of a shared type it permits, and that
//! constraint must survive into every generated artifact — the exact case
//! where `ButtonSpec` accepted `ButtonTone::Success` while `button.md`
//! permitted three tones and the web silently rendered default.
//!
//! Governing refs: `docs/roadmaps/g13/batch-cards/003-crate-placement-ruling-and-schema-handoff.md`
//! (R6.1, R6.2), `docs/specs/063-rust-authored-component-and-scene-ir.md`
//! ("Shared types and permitted subsets"), `docs/contracts/004-shared-control-types.md`.
//!
//! [`PermittedSubset`] is validated by
//! [`validate`](crate::validate): any [`Value`](crate::Value) that names a
//! shared-type member outside the subset is rejected with a
//! `PermittedSubsetViolation` finding.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{ContractRef, Identifier};

/// A named enumerated shared type, defined once and referenced by many
/// components (g13-b003 R6.1; spec 063 "Shared types and permitted subsets";
/// `CROSS-01` stable identifiers).
///
/// Motivating cases: `ButtonTone`, `OverlayPlacement`, and the eight further
/// enumerated shared types that fragmented across contracts with no
/// definition anywhere in `docs/` (g13-b007 evidence).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedType {
    /// Stable identifier cited by component references, e.g. `button-tone`.
    pub id: Identifier,
    /// Rust-style type name, e.g. `ButtonTone`.
    pub name: String,
    /// What the type means and where its members come from.
    pub description: String,
    /// Canonical definition the members are transcribed from
    /// (`docs/contracts/004-shared-control-types.md` for shared control
    /// types; the fragmenting contracts for pilot evidence).
    pub canonical_ref: ContractRef,
    /// The enumerated members, in authoring order (deterministic
    /// serialization; duplicates are a validation finding).
    pub members: Vec<SharedEnumMember>,
}

/// One member of a shared enumerated type, e.g. `ButtonTone::danger`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedEnumMember {
    /// Stable identifier for the member, e.g. `danger`; this is the value
    /// props, defaults, and bindings cite.
    pub id: Identifier,
    /// Rust-style member name, e.g. `Danger`.
    pub name: String,
    /// What the member means, naming the contract union it came from where
    /// applicable.
    pub description: String,
}

/// Reference from a component to a shared type (g13-b003 R6.1). The id must
/// resolve to a [`SharedType`] in the model or validation reports an
/// `InvalidReference` finding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SharedTypeRef {
    /// Id of the referenced [`SharedType`].
    pub id: Identifier,
}

/// First-class permitted subset of a shared type's members (g13-b003 R6.2;
/// spec 063 "the subset of a shared type this component permits").
///
/// A component attaches this to a prop or binding that references a shared
/// type. Values naming members outside `members` are rejected by
/// [`validate`](crate::validate) with a `PermittedSubsetViolation` finding,
/// preventing the silent-render-as-default drift the Button tone case
/// demonstrated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermittedSubset {
    /// Id of the [`SharedType`] being constrained.
    pub shared_type: Identifier,
    /// Member ids the component permits, serialized sorted for deterministic
    /// output (`IR-07`). Non-empty; each id must exist on the shared type.
    pub members: BTreeSet<Identifier>,
}

impl PermittedSubset {
    /// Builds a permitted subset from a shared-type id and member ids.
    pub fn new(
        shared_type: impl Into<Identifier>,
        members: impl IntoIterator<Item = impl Into<Identifier>>,
    ) -> Self {
        Self {
            shared_type: shared_type.into(),
            members: members.into_iter().map(Into::into).collect(),
        }
    }

    /// Whether the given member id is permitted by this subset.
    pub fn permits(&self, member: &str) -> bool {
        self.members.iter().any(|m| m.as_str() == member)
    }
}
