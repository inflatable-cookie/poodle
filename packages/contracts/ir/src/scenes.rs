//! Scenes — specimen/scene definitions, axis matrices, and shell vocabulary.
//!
//! Serves `CROSS-21` (contract §13 specimen sets as shared scene definitions
//! rendered by all four shells) and the `SHELL-*` rows (theme/size/density/
//! contrast controls `SHELL-01`–`SHELL-04`, navigation `SHELL-05`, search
//! `SHELL-06`, specimen tabs `SHELL-07`, preview-state serialization
//! `SHELL-08`, parity-harness vocabulary `SHELL-09`, specimen registry
//! `SHELL-10`), per spec 063 "Scene IR" (component references and typed prop
//! bindings, layout nodes, groups, loops, conditions, theme/size/density/
//! orientation/contrast axes, interaction scenarios and stable capture
//! identifiers).
//!
//! Scene IR is not an application framework: routing, persistence, data
//! fetching, authorization, product state, arbitrary host callbacks, and
//! DAW-specific models remain outside it (spec 063 "Scene IR").

use serde::{Deserialize, Serialize};

use crate::{Identifier, RuntimeTarget, Value};

/// A scene or specimen definition (`CROSS-21`; `B/R/T §13` specimen sets;
/// spec 063 "Scene IR").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    /// Stable identifier for the scene, e.g. `button-variants`
    /// (`BTN-28`).
    pub id: Identifier,
    /// Scene name, e.g. `Variants` (`BTN-28`).
    pub name: String,
    /// What the scene demonstrates, citing the contract specimen section.
    pub description: String,
    /// Component instances with typed prop bindings (spec 063 "component
    /// references and typed prop bindings").
    pub instances: Vec<ComponentInstance>,
    /// Declared scene axes — theme, size, density, orientation, contrast
    /// (spec 063 Scene IR "theme, size, density, orientation, and contrast
    /// axes"; `CROSS-10` contrast as a scene axis).
    pub axes: Vec<SceneAxis>,
    /// Shell navigation layout (`SHELL-05`).
    pub layout: Option<SceneLayout>,
    /// Specimen tab matrix (`SHELL-07`).
    pub tabs: Option<SpecimenTabs>,
    /// Component search configuration (`SHELL-06`).
    pub search: Option<SearchConfig>,
    /// Serialized preview state (theme/density/controlSize/contrast)
    /// (`SHELL-08`).
    pub preview_state: Option<PreviewState>,
    /// Parity-harness vocabulary (`SHELL-09`).
    pub parity: Option<ParityHarness>,
    /// Stable capture identifiers for visual evidence (spec 063
    /// "interaction scenarios and stable capture identifiers").
    pub captures: Vec<Identifier>,
}

/// One component instance inside a scene (spec 063 "component references and
/// typed prop bindings").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance {
    /// Id of the referenced component; must resolve in the model.
    pub component: Identifier,
    /// Typed prop bindings; `validate` rejects bindings to unknown props,
    /// type-mismatched values, subset violations, and do-not-mix pairs.
    pub bindings: Vec<PropBinding>,
    /// Optional caption shown next to the instance.
    pub caption: Option<String>,
    /// Optional specimen-section heading the instance belongs to (spec 063
    /// Scene IR "groups" — added by `g14-b005` tranche one, which measured
    /// the static tier against the scene and found grouped sections missing).
    /// Consecutive instances in the same group render under one heading;
    /// validation rejects an empty string.
    #[serde(default)]
    pub group: Option<String>,
}

/// A typed prop binding inside a scene (`CROSS-02`; spec 063 "typed prop
/// bindings"; `SHELL-10` specimen wiring).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropBinding {
    /// Prop id on the referenced component.
    pub prop: Identifier,
    /// Bound value. Member values are checked against the prop's permitted
    /// subset when one is declared (g13-b003 R6.2).
    pub value: Value,
    /// What the binding demonstrates, if anything.
    pub description: Option<String>,
}

/// A declared scene axis (`SHELL-01`–`SHELL-04`; `CROSS-07`–`CROSS-11`;
/// spec 063 Scene IR "theme, size, density, orientation, and contrast
/// axes").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneAxis {
    /// The axis kind.
    pub kind: SceneAxisKind,
    /// Values of the axis — named values or a continuous range.
    pub values: AxisValues,
    /// What the axis controls, citing the contract section.
    pub description: String,
}

/// The scene axes of spec 063 Scene IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SceneAxisKind {
    /// Theme preset selection (`SHELL-01`; `CROSS-09` theme axis).
    #[serde(rename = "theme")]
    Theme,
    /// Control-size selection xs–xl (`SHELL-02`; `CROSS-07`).
    #[serde(rename = "size")]
    Size,
    /// Density selection compact/default/comfortable (`SHELL-03`;
    /// `CROSS-08`).
    #[serde(rename = "density")]
    Density,
    /// Orientation selection (`CROSS-11`; `RNG-07`).
    #[serde(rename = "orientation")]
    Orientation,
    /// Continuous neutral-contrast override (`SHELL-04`; `CROSS-10`;
    /// `T §7`).
    #[serde(rename = "contrast")]
    Contrast,
}

/// Values of a scene axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AxisValues {
    /// Named values — theme presets (`SHELL-01`), sizes (`SHELL-02`),
    /// densities (`SHELL-03`), orientations (`CROSS-11`). Theme names
    /// resolve against `poodle-tokens` theme presets; sizes and densities
    /// against the control-size/density registries.
    #[serde(rename = "named")]
    Named(Vec<Identifier>),
    /// A continuous range, e.g. the neutral-contrast axis (`CROSS-10`,
    /// `SHELL-04`: web 0.4–1.6, GPUI 0..1, Jetstream knob). `min ≤ default
    /// ≤ max` is validated.
    #[serde(rename = "continuous")]
    Continuous {
        /// Inclusive lower bound.
        min: f64,
        /// Inclusive upper bound.
        max: f64,
        /// Default value.
        default: f64,
    },
}

/// Shell navigation layout (`SHELL-05`): top-level sections, component
/// sidebar groups, and route state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneLayout {
    /// Top-level sections, e.g. Components/Tokens (+ Demo on native)
    /// (`SHELL-05`).
    pub sections: Vec<NavSection>,
    /// Route state the shells persist (`SHELL-05` hash + query params on
    /// web).
    pub route_state: RouteState,
}

/// A top-level navigation section (`SHELL-05`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavSection {
    /// Section title, e.g. `Components`.
    pub title: String,
    /// Section kind.
    pub kind: NavSectionKind,
    /// Component sidebar groups inside the section (`SHELL-05`).
    pub groups: Vec<ComponentGroup>,
}

/// Kind of a navigation section (`SHELL-05`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavSectionKind {
    #[serde(rename = "components")]
    Components,
    #[serde(rename = "tokens")]
    Tokens,
    /// Native shells add a Demo section (`SHELL-05`).
    #[serde(rename = "demo")]
    Demo,
}

/// A component sidebar group (`SHELL-05`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentGroup {
    /// Group title, e.g. `Controls`.
    pub title: String,
    /// Component ids in the group.
    pub components: Vec<Identifier>,
}

/// Route state persisted by the shells (`SHELL-05`, `SHELL-08`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteState {
    /// Route parameters persisted across navigation, e.g. hash + query
    /// params on web (`SHELL-08` `syncCurrentLocation`/`replaceState`).
    pub persisted: Vec<String>,
}

/// Specimen tab matrix — Examples / Sizes / Densities in all four shells
/// (`SHELL-07`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecimenTabs {
    /// The tabs, e.g. `examples`, `sizes`, `densities` (`SHELL-07`;
    /// `RNG-25` Sizes tab, densities specimen).
    pub tabs: Vec<Identifier>,
}

/// Component search configuration (`SHELL-06`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Case-insensitive matching (`SHELL-06` "case-insensitive filter").
    pub case_insensitive: bool,
    /// Fields searched, e.g. display name and description (`SHELL-06`).
    pub fields: Vec<SearchField>,
}

/// A searchable field (`SHELL-06`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchField {
    #[serde(rename = "display-name")]
    DisplayName,
    #[serde(rename = "description")]
    Description,
}

/// Serialized preview state — theme/density/controlSize/contrast persisted
/// in URL query + hash on the web shells (`SHELL-08`; spec 063 Scene IR).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreviewState {
    /// Theme preset id (`SHELL-01`, `SHELL-08`).
    pub theme: Option<Identifier>,
    /// Density preset id (`SHELL-03`, `SHELL-08`).
    pub density: Option<Identifier>,
    /// Control-size id (`SHELL-02`, `SHELL-08`).
    pub control_size: Option<Identifier>,
    /// Continuous neutral-contrast value (`SHELL-04`, `SHELL-08`).
    pub contrast: Option<f64>,
}

/// Parity-harness vocabulary — defaults, review presets, targets, visual
/// gate tiers with explicit axes, and the native baseline gate (`SHELL-09`;
/// `IR-10` "executed semantic, interaction, accessibility, recipe, axis, and
/// visual evidence").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParityHarness {
    /// Preview state defaults for parity runs (`SHELL-09`).
    pub defaults: PreviewState,
    /// Review route presets (`SHELL-09`).
    pub review_route_presets: Vec<Identifier>,
    /// Parity targets (`SHELL-09`; `IR-10` four-runtime proof).
    pub targets: Vec<RuntimeTarget>,
    /// Visual gate tiers with explicit axes (`SHELL-09`; `test/visual/config.ts`
    /// `AXIS_TIER_SLUGS`).
    pub visual_gates: Vec<VisualGate>,
    /// Whether the native visual baseline gate is required (`SHELL-09`;
    /// `test/native-visual/config.ts`).
    pub native_visual_baseline: bool,
}

/// A visual gate tier with its axes (`SHELL-09`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualGate {
    /// Gate tier.
    pub tier: GateTier,
    /// Axes the gate exercises (`SHELL-09` "visual-gate tiers … with explicit
    /// axes").
    pub axes: Vec<SceneAxisKind>,
}

/// Visual gate tier (`SHELL-09`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateTier {
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "axis")]
    Axis,
    #[serde(rename = "sweep")]
    Sweep,
}

/// Specimen registry — per-shell wiring so every component renders through
/// the same shell chrome (`SHELL-10`; `CROSS-21` `GTA` surface: the registry
/// itself is a later-card generated target artifact).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecimenRegistry {
    /// Registry entries, one per registered component.
    pub entries: Vec<SpecimenEntry>,
}

/// One specimen-registry entry (`SHELL-10`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecimenEntry {
    /// Stable entry identifier.
    pub id: Identifier,
    /// The registered component id.
    pub component: Identifier,
    /// Scenes the component renders through the shell chrome (`SHELL-10`).
    pub scenes: Vec<Identifier>,
}
