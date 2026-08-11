//! Accessibility intent — role, accessible-name rule, ARIA state mapping.
//!
//! Serves `CROSS-15` (role, accessible-name rule, ARIA state mapping,
//! native-attribute projection per component) and the per-component
//! accessibility rows (`BTN-21`, `RNG-10/19`, `TXT-03/26`), per spec 063
//! "Component IR" ("accessibility intent"). Accessibility projection onto a
//! runtime remains adapter-owned (`NEG-03`, `IR-05`); this module declares
//! the intent the projection implements.

use serde::{Deserialize, Serialize};

use crate::Identifier;

/// Accessibility intent of a component (`CROSS-15`).
///
/// Roles are semantic vocabulary, not framework types: `button`, `group`,
/// `slider`, `textbox` name the accessibility role the contract documents
/// (`BTN-21` native button role; `RNG-14` root `role="group"`; `TXT-26`
/// native input role). Native projection onto each runtime stays adapter-owned
/// (`NEG-03`, `IR-05`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Accessibility {
    /// The declared accessibility role (`CROSS-15`).
    pub role: A11yRole,
    /// How the accessible name is derived (`CROSS-15`; `BTN-21` icon-only
    /// requires a name; `TXT-03` placeholder never counts as the name).
    pub name_rule: NameRule,
    /// Where the accessible name comes from when a rule requires or allows a
    /// named source (`BTN-21`, `TXT-26` `aria-label` required without
    /// external label).
    pub name_source: Option<NameSource>,
    /// Declared ARIA state mappings, e.g. `aria-pressed` ← `pressed`
    /// (`BTN-14`), `aria-busy` ← `loading` (`BTN-08`), `aria-valuetext` ←
    /// `lowerValueText` (`RNG-10`), `aria-invalid` ← validation state
    /// (`TXT-26`).
    pub aria: Vec<AriaMapping>,
    /// Native attribute projections the contract mandates, e.g. native
    /// `disabled` (`BTN-07`), native `readonly` (`TXT-05` — not
    /// `aria-readonly`).
    pub native: Vec<NativeAttr>,
    /// What the accessibility model guarantees, citing the contract section.
    pub description: String,
}

/// Declared accessibility role (`CROSS-15`; `B §6`, `R §6`, `T §6`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum A11yRole {
    /// Native button role (`BTN-21`).
    #[serde(rename = "button")]
    Button,
    /// Group role, e.g. the RangeSlider root (`RNG-14`).
    #[serde(rename = "group")]
    Group,
    /// Slider role, e.g. the embedded focus stops (`RNG-15`).
    #[serde(rename = "slider")]
    Slider,
    /// Textbox role for text input (`TXT-26` native input role).
    #[serde(rename = "textbox")]
    Textbox,
    /// Multiline textarea (`TXT-07`).
    #[serde(rename = "textbox-multiline")]
    TextboxMultiline,
    /// Search input mode (`TXT-08`).
    #[serde(rename = "searchbox")]
    Searchbox,
}

/// How the accessible name is derived (`CROSS-15`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NameRule {
    /// From the content/label, e.g. button children (`BTN-16`,
    /// `BTN-21`).
    #[serde(rename = "from-content")]
    FromContent,
    /// From a dedicated prop, e.g. `ariaLabel` (`BTN-15`, `TXT-26`).
    #[serde(rename = "from-prop")]
    FromProp(Identifier),
    /// An accessible name is required; missing it is a validation finding
    /// ("missing accessibility data") — icon-only buttons must carry one
    /// (`BTN-21`).
    #[serde(rename = "required")]
    Required,
    /// The name must never be derived from the placeholder (`TXT-03`
    /// "placeholder never counts as the accessible name").
    #[serde(rename = "never-placeholder")]
    NeverPlaceholder,
}

/// Source of an accessible name (`CROSS-15`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NameSource {
    /// The content/label part (`BTN-16`).
    #[serde(rename = "content")]
    Content,
    /// A named prop carrying the name, e.g. `ariaLabel` (`BTN-15`,
    /// `TXT-26`).
    #[serde(rename = "prop")]
    Prop(Identifier),
    /// An external label associated by element id (`TXT-01` `id` required
    /// for label association; `TXT-26`).
    #[serde(rename = "external-label")]
    ExternalLabel,
}

/// A declared ARIA state mapping (`CROSS-15`). `source` must resolve to a
/// prop or controlled state on the component; `validate` checks it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AriaMapping {
    /// The ARIA attribute, e.g. `aria-pressed` (`BTN-14`), `aria-busy`
    /// (`BTN-08`), `aria-valuetext` (`RNG-10`), `aria-invalid` (`TXT-26`).
    pub aria_attr: String,
    /// Prop or state id the attribute derives from.
    pub source: Identifier,
    /// What the mapping conveys, citing the contract section.
    pub description: String,
}

/// A mandated native attribute projection (`CROSS-15`; `B §6`, `R §6`,
/// `T §6`). Native behavior is retained above the IR (`NEG-02`); this
/// declares which native attributes the projection must emit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeAttr {
    /// Attribute name, e.g. `disabled`, `readonly`, `id`, `required`,
    /// `pattern`, `autocomplete`, `inputmode` (`BTN-07`, `TXT-01`,
    /// `TXT-04`, `TXT-05`, `TXT-26`).
    pub name: String,
    /// Why the projection is mandated, citing the contract section.
    pub description: String,
}
