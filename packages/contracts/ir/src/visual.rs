//! VisualState — the serializable projection shape drawing consumes.
//!
//! Serves `CROSS-14` (drawing consumes a serializable state projection;
//! exact fields per component, `RNG-16` plus Button/TextInput equivalents)
//! per spec 063 `IR-06` ("VisualState purity"): drawing consumes serializable
//! state and does not read machine state or own hit-testing/input (`NEG-04`).
//!
//! A [`VisualState`] declares the projection *shape* (field names and value
//! kinds), not values: the runtime machine computes the projection
//! (`rangeSliderVisualState` in `core/src/slider.ts`; `RangeSliderVisualState`
//! in `contracts/headless/src/slider.rs`).

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A serializable VisualState projection shape (`CROSS-14`; `RNG-16` the
/// fourteen-field RangeSlider projection; `BTN-19`/`TXT-19` visual-state
/// equivalents).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualState {
    /// Stable identifier for the projection, e.g. `range-slider`
    /// (`CROSS-14`).
    pub id: Identifier,
    /// Projection name, e.g. `RangeSliderVisualState` (`RNG-16`).
    pub name: String,
    /// Declared fields of the projection, in declaration order.
    pub fields: Vec<VisualStateField>,
    /// What the projection feeds and who computes it, citing the contract
    /// and machine evidence.
    pub description: String,
}

/// One field of a VisualState projection (`CROSS-14`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualStateField {
    /// Stable identifier for the field, e.g. `lowerNorm` (`RNG-16`).
    pub id: Identifier,
    /// Field name as the projection carries it.
    pub name: String,
    /// Value kind of the field.
    pub kind: VisualFieldKind,
    /// What the field means, citing the contract section.
    pub description: String,
}

/// Value kind of a VisualState field (`CROSS-14`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualFieldKind {
    /// A numeric value, e.g. `lowerNorm`, `upperNorm`, `centerNorm`
    /// (`RNG-16`).
    #[serde(rename = "number")]
    Number,
    /// A boolean value, e.g. `pointerActive`, `enabled`, `fillSplitAtCenter`
    /// (`RNG-16`, `RNG-23`).
    #[serde(rename = "boolean")]
    Bool,
    /// A string value.
    #[serde(rename = "string")]
    String,
    /// A member of a shared enumerated type, e.g. `polarity`
    /// (unipolar/bipolar, `RNG-16` `RNG-04`).
    #[serde(rename = "enum")]
    Enum(Identifier),
    /// A start/span geometry pair, e.g. `fillStartNorm`/`fillSpanNorm`
    /// (`RNG-16`).
    #[serde(rename = "pair")]
    Pair,
}
