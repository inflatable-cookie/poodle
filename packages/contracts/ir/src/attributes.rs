//! State-derived attributes — the `data-*` emission rules.
//!
//! Serves `CROSS-13` (the `data-*` emission rules: presence-only vs valued,
//! omitted vs always emitted, per component) and the per-component attribute
//! rows (`BTN-18` `data-variant`/`data-tone` omit-when-default and
//! `data-loading` always; `TXT-18` `data-validation-state`; `RNG-*`
//! `data-orientation`/`data-density`/`data-fill-split`), per `B/R/T §9`.

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A state-derived attribute on a component (`CROSS-13`; `B §9`, `R §9`,
/// `T §9`).
///
/// Attributes are declared, never paint-time side effects (`NEG-08`): the
/// emission form and policy are part of the definition so every runtime
/// derives the same `data-*` output. The same mechanism carries computed
/// custom properties such as the RangeSlider fill-geometry hooks
/// `--poodle-range-start/end/center/…` (`RNG-17`) and the TextInput
/// adornment-padding reservation `--poodle-text-input-control-padding-start/
/// end` (`TXT-16`), whose `source` names a VisualState field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateAttribute {
    /// Stable identifier for the attribute, e.g. `data-loading`
    /// (`CROSS-13`).
    pub id: Identifier,
    /// The emitted attribute name, e.g. `data-loading`, `data-tone`
    /// (`BTN-18`, `TXT-18`), or a computed custom property such as
    /// `--poodle-range-start` (`RNG-17`).
    pub name: String,
    /// Presence-only vs valued form (`CROSS-13`).
    pub form: AttributeForm,
    /// Omitted vs always emitted policy (`CROSS-13`).
    pub emission: EmissionPolicy,
    /// Prop, controlled-state, or VisualState-field id the attribute derives
    /// from, e.g. `loading` for `data-loading` (`BTN-08`), `pressed` for
    /// `data-pressed` (`BTN-14`), or `lowerNorm` for the fill-geometry
    /// custom properties (`RNG-17`). `validate` checks the reference
    /// resolves. `None` marks a runtime-derived value (e.g. `data-state`
    /// `active`/`idle`) whose domain the description documents.
    pub source: Option<Identifier>,
    /// What the attribute conveys and when it is emitted, citing the
    /// contract section. The description is the emission condition's home:
    /// e.g. `data-tone` "omitted when the tone is default" is documented
    /// here and in the [`EmissionPolicy`] — there is no expression tree
    /// (g13.017).
    pub description: String,
}

/// Emission form of a state-derived attribute (`CROSS-13`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeForm {
    /// Presence-only: emitted without a value, e.g. `data-pressed`
    /// (`BTN-14`, `BTN-18`).
    #[serde(rename = "presence-only")]
    PresenceOnly,
    /// Valued: emitted with the derived value, e.g. `data-variant="primary"`
    /// (`BTN-18`), `data-validation-state` (`TXT-18`).
    #[serde(rename = "valued")]
    Valued,
}

/// Emission policy of a state-derived attribute (`CROSS-13`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmissionPolicy {
    /// Omitted when the source is at its default, e.g. `data-tone` omitted
    /// for the default tone (`BTN-18` "omit when default").
    #[serde(rename = "omit-when-default")]
    OmitWhenDefault,
    /// Always emitted, e.g. `data-loading` (`BTN-08` "`data-loading` always
    /// emitted", `BTN-18`).
    #[serde(rename = "always")]
    Always,
}
