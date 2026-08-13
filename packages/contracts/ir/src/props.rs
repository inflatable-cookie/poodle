//! Props — the portable spec surface.
//!
//! Serves `CROSS-02` (every public prop with default and type that survives
//! into the Rust spec: `ButtonSpec`, `RangeSliderSpec`, `TextInputSpec`),
//! `CROSS-03` (web-only prop surface excluded from the portable spec:
//! Button form-submission family, TextInput native attributes), and the
//! shared-type/permitted-subset layer (g13-b003 R6.1/R6.2) as it applies to
//! prop values and defaults.

use serde::{Deserialize, Serialize};

use crate::{Identifier, PermittedSubset};

/// A public prop on a component (`CROSS-02`; `B/R/T §3` portable-spec
/// tables). The declaration is portable: name, type, default, and required
/// survive into every generated target, while web-only props are marked and
/// excluded from the portable spec surface (`CROSS-03`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prop {
    /// Stable identifier for the prop, e.g. `tone` (`CROSS-02`; spec 063
    /// "stable identifiers").
    pub id: Identifier,
    /// Public prop name, e.g. `tone` (`CROSS-02`).
    pub name: String,
    /// Declared type, including references to shared enumerated types
    /// (g13-b003 R6.1).
    pub prop_type: PropType,
    /// Default value when the prop is not bound (`CROSS-02` "every public
    /// prop with default and type"). Validation checks a member-valued
    /// default against the permitted subset. Expression defaults were
    /// removed with the expression vocabulary (g13.017).
    pub default: Option<Value>,
    /// Whether the prop is required for the component to be valid.
    pub required: bool,
    /// Web-only surface excluded from the portable spec (`CROSS-03`): the
    /// Button form-submission family (`BTN-06`) and TextInput native
    /// attributes such as `autocomplete`, `pattern`, `spellcheck` (`TXT-04`).
    /// RangeSlider has none.
    pub web_only: bool,
    /// What the prop means, citing the contract section it transcribes.
    pub description: String,
    /// Permitted subset of a shared enumerated type for this prop's values
    /// (g13-b003 R6.2). Must be present iff `prop_type` is a `Shared`
    /// reference; every member value (default or binding) is subset-checked
    /// by [`validate`](crate::validate).
    pub permitted_subset: Option<PermittedSubset>,
}

/// The type of a prop value (`CROSS-02`).
///
/// Framework, DOM, and runtime types never appear here (`NEG-02`): the
/// vocabulary is primitives, opaque payloads, pairs/lists, and references to
/// first-class shared enumerated types (g13-b003 R6.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropType {
    /// A text value, e.g. `ariaLabel` (`BTN-15`), `placeholder` (`TXT-03`).
    String,
    /// A numeric value, e.g. `maxWidth` (`BTN-13`), `debounce` milliseconds
    /// (`TXT-11`), `maxLength` (`TXT-14`).
    Number,
    /// A boolean value, e.g. `disabled` (`BTN-07`), `showClearButton`
    /// (`TXT-08`).
    Bool,
    /// Reference to a first-class shared enumerated type (g13-b003 R6.1),
    /// e.g. `tone` referencing `button-tone` (`BTN-02`), `size` referencing a
    /// control-size shared type (`CROSS-07`).
    Shared(Identifier),
    /// A pair of values of the same inner type, e.g. the RangeSlider
    /// controlled value `[lower, upper]` (`RNG-01`).
    Pair(Box<PropType>),
    /// A list of values of the same inner type, e.g. an icon list or
    /// repeated-content source (`CROSS-12`).
    List(Box<PropType>),
    /// An opaque payload the component carries without interpreting it, e.g.
    /// the validation context (`TXT-12` "opaque"). Opaque values never cross
    /// the serializable boundary as structure.
    Opaque,
}

impl PropType {
    /// Whether this type references a first-class shared enumerated type
    /// (g13-b003 R6.1). Permitted subsets are only meaningful on shared-typed
    /// props.
    pub fn is_shared(&self) -> bool {
        matches!(self, Self::Shared(_))
    }
}

/// A literal prop value, default, or scene binding value (`CROSS-02`;
/// spec 063 "typed prop bindings"). Externally tagged in JSON so string and
/// number values are unambiguous.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// A text literal, e.g. `"primary"` for a non-shared string prop.
    String(String),
    /// A numeric literal, e.g. `100` for `max` (`RNG-02`).
    Number(f64),
    /// A boolean literal.
    Bool(bool),
    /// A shared enumerated member, e.g. `danger` for a `button-tone` prop
    /// (`BTN-02`). The member must exist on the referenced shared type and,
    /// when a [`PermittedSubset`] constrains the prop, must be inside it
    /// (g13-b003 R6.2).
    Member(Identifier),
    /// A pair literal, e.g. `[0, 100]` for the RangeSlider value (`RNG-01`).
    Pair(Box<Value>, Box<Value>),
    /// A list literal, e.g. a repeated-content value (`CROSS-12`).
    List(Vec<Value>),
    /// The null literal — a valid controlled empty state, e.g. TextInput
    /// `value = null` (`TXT-02`).
    Null,
}

impl Value {
    /// Builds a string literal.
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Builds a number literal.
    pub fn number(value: f64) -> Self {
        Self::Number(value)
    }

    /// Builds a boolean literal.
    pub fn boolean(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Builds a shared-member literal.
    pub fn member(value: impl Into<Identifier>) -> Self {
        Self::Member(value.into())
    }

    /// Whether this value is a shared enumerated member.
    pub fn is_member(&self) -> bool {
        matches!(self, Self::Member(_))
    }

    /// The shared member id, if this is a member literal.
    pub fn as_member(&self) -> Option<&str> {
        match self {
            Self::Member(id) => Some(id.as_str()),
            _ => None,
        }
    }
}
