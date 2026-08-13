//! Anatomy parts — parent/child structure with conditional, documented,
//! and identified-instance nodes.
//!
//! Serves `CROSS-12` (anatomy parts with parent/child constraints and
//! conditional/repeated composition: Button spinner/icons/label/chevron;
//! RangeSlider track/fill/two identified controls; TextInput
//! affixes/affordances/indicator/count) and the per-component anatomy rows
//! (`BTN-17`, `RNG-14`, `RNG-15`, `TXT-17`), per spec 063 "Component IR"
//! ("semantic anatomy and parent/child constraints" and "renderer-neutral
//! render nodes and conditional/repeated composition"). The `Repeated`
//! kind was retired by `g13.018` — a fixed set of identified instances is
//! [`PartKind::Identified`], whose count and identities come from the
//! definition (`g13.018` R5).

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
    /// Whether the part is static, conditional, documented-conditional, or
    /// an identified instance (`CROSS-12` "conditional/repeated nodes").
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
    /// Present only under a documented condition — the condition is prose
    /// vocabulary, not an expression tree (g13.017 R1 bucket 2: the
    /// anatomy fact "this part renders only when X" is kept, the
    /// expression form is gone). No runtime evaluates a part condition;
    /// every runtime renders the part from its own logic and reads only
    /// the anatomy names from the artifact.
    #[serde(rename = "conditional-documented")]
    ConditionalDocumented {
        /// The documented condition under which the part renders, e.g.
        /// "standard variant only" — prose kept from the removed
        /// expression tree.
        #[serde(rename = "condition")]
        condition: String,
        /// Why the part is conditional, citing the contract.
        description: String,
    },
    /// A fixed set of identified instances, e.g. the two RangeSlider thumbs
    /// (`RNG-14` "two overlapping native range inputs"). The count and the
    /// identities come from the definition: each named instance is its own
    /// part in the same anatomy, carrying its own identity and declared
    /// semantics, and a runtime derives the instances from the list rather
    /// than hard-coding the count (`g13.018` R5 — the replacement for the
    /// retired `Repeated` kind, which required a `List` prop and yielded
    /// identical instances with no per-item identity).
    #[serde(rename = "identified")]
    Identified {
        /// Instance part ids, in declaration order. Each must exist as a
        /// part in the same component (validation enforces it).
        #[serde(rename = "instances")]
        instances: Vec<Identifier>,
        /// What the instances are, citing the contract.
        description: String,
    },
}
