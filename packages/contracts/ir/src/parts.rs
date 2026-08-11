//! Anatomy parts — parent/child structure with conditional and repeated
//! nodes.
//!
//! Serves `CROSS-12` (anatomy parts with parent/child constraints and
//! conditional/repeated nodes: Button spinner/icons/label/chevron; RangeSlider
//! track/fill/two controls; TextInput affixes/affordances/indicator/count) and
//! the per-component anatomy rows (`BTN-17`, `RNG-14`, `RNG-15`, `TXT-17`),
//! per spec 063 "Component IR" ("semantic anatomy and parent/child
//! constraints" and "renderer-neutral render nodes and conditional/repeated
//! composition").

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// One part of a component's anatomy (`CROSS-12`; `B §2`, `R §2`, `T §2`).
///
/// Parts form a forest: `parent` points at another part in the same
/// component. `validate` rejects unknown parents and parent/child cycles
/// (`NEG-*`-safe declarative structure; spec 063 "parent/child constraints").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Part {
    /// Stable identifier for the part, e.g. `spinner`, `track`, `fill`,
    /// `field`, `char-count` (`CROSS-12`).
    pub id: Identifier,
    /// Display name of the part, e.g. `Spinner` (`BTN-17`).
    pub name: String,
    /// Parent part id, or `None` for a root part (`CROSS-12` "parent/child
    /// constraints").
    pub parent: Option<Identifier>,
    /// Whether the part is static, conditional, or repeated (`CROSS-12`
    /// "conditional/repeated nodes").
    pub kind: PartKind,
    /// What the part is, citing the contract anatomy section.
    pub description: String,
}

/// Composition kind of a part (`CROSS-12` "conditional/repeated composition";
/// `B §2`: spinner when loading; `R §2`: two overlapping controls; `T §2`:
/// affordance slots).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartKind {
    /// Always present, e.g. the Button label when children exist
    /// (`BTN-17`), the RangeSlider track (`RNG-14`).
    #[serde(rename = "static")]
    Static,
    /// Present only when the named boolean prop is true, e.g. the spinner
    /// when `loading` (`BTN-08`, `BTN-17`), the clear button when
    /// `showClearButton` (`TXT-08`, `TXT-17`).
    #[serde(rename = "conditional")]
    Conditional {
        /// Prop id whose truthiness gates the part; must be a boolean prop.
        #[serde(rename = "when")]
        when: Identifier,
        /// Why the part is conditional, citing the contract.
        description: String,
    },
    /// Repeated once per element of the named list prop, e.g. the two
    /// RangeSlider thumbs (`RNG-14` "two overlapping native range inputs").
    #[serde(rename = "repeated")]
    Repeated {
        /// List prop id the part repeats over.
        #[serde(rename = "over")]
        over: Identifier,
        /// What each repetition is, citing the contract.
        description: String,
    },
}
