//! Axes — size, density, orientation, theme, and contrast.
//!
//! Serves `CROSS-07` (size axis: explicit `size` override xs–xl and
//! `sizeRole` resolved from inherited presentation; per-component size
//! ladders), `CROSS-08` (density axis: compact/default/comfortable, explicit
//! override or inherited, per-component adjustments including the two
//! documented exceptions), `CROSS-10` (contrast as a continuous scene axis),
//! `CROSS-11` (orientation/direction axis), and the per-component axis rows
//! (`BTN-03/04/05`, `BTN-23`, `RNG-09`, `TXT-15`). Theme axis resolution
//! against `poodle-tokens` lives in [`crate::tokens`], and the scene-level
//! axis matrix lives in [`crate::scenes`].

use serde::{Deserialize, Serialize};

use crate::{Identifier, MetricValue};

/// The axes a component participates in (`CROSS-07`–`CROSS-11`; spec 063
/// "size, density, orientation, direction, and contrast axes"). Contrast is a
/// scene axis (`CROSS-10`; spec 063 Scene IR "contrast axes") and is modelled
/// in [`crate::scenes`]; the component-level axes are size, density, and
/// orientation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Axes {
    /// Size axis: explicit override and inherited `sizeRole` (`CROSS-07`).
    pub size: Option<SizeAxis>,
    /// Density axis: explicit override or inherited (`CROSS-08`).
    pub density: Option<DensityAxis>,
    /// Orientation axis, e.g. RangeSlider horizontal/vertical (`CROSS-11`,
    /// `RNG-07`).
    pub orientation: Option<OrientationAxis>,
}

/// Control size — the xs–xl ladder (`CROSS-07`; `B/R/T §7/§8` size tables;
/// `SHELL-02`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ControlSize {
    #[serde(rename = "xs")]
    Xs,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "md")]
    Md,
    #[serde(rename = "lg")]
    Lg,
    #[serde(rename = "xl")]
    Xl,
}

/// Semantic control-size role — resolved from inherited presentation when
/// the explicit size is absent (`CROSS-07`, `BTN-04`; `SemanticControlSizeRole`
/// in `docs/contracts/004-shared-control-types.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeRole {
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "control")]
    Control,
    #[serde(rename = "prominent")]
    Prominent,
}

/// The size axis of a component (`CROSS-07`, `BTN-03`, `BTN-04`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeAxis {
    /// Explicit size override; `None` means "resolve from inherited
    /// presentation plus `size_role`" (`CROSS-07`; `BTN-03` default `null`).
    pub explicit: Option<ControlSize>,
    /// Semantic size role consulted when no explicit override is present
    /// (`CROSS-07`, `BTN-04`).
    pub size_role: SizeRole,
    /// Per-component size ladder metrics, one entry per rung (`CROSS-07`
    /// "per-component size ladders"; `BTN-23`, `RNG-09`, `TXT-15` size
    /// tables). The `size ?? resolveSemanticControlSize(sizeRole)` fallback
    /// resolution was an expression (g13.017 R1 bucket 3: derivation) and
    /// is gone — `size_role` remains the declared resolution vocabulary
    /// (CROSS-07).
    pub ladder: Vec<SizeStep>,
}

/// One rung of a per-component size ladder (`BTN-23` fixed rem heights and
/// min-widths; `RNG-09` min-height/track thickness/thumb diameter; `TXT-15`
/// min-height/padding/font-size).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeStep {
    /// The rung this step defines.
    pub size: ControlSize,
    /// Declared metrics for the rung, keyed by metric name (e.g.
    /// `min-height`, `min-width`, `padding-inline`, `font-size`,
    /// `track-thickness`, `thumb-diameter`, `icon-size`, `gap`).
    pub metrics: std::collections::BTreeMap<String, MetricValue>,
    /// What this rung changes, citing the contract size table.
    pub description: String,
}

/// Control density — compact/default/comfortable (`CROSS-08`; `B/R/T §8/§9`
/// density blocks; `SHELL-03`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ControlDensity {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "comfortable")]
    Comfortable,
}

/// The density axis of a component (`CROSS-08`; `BTN-05`, `RNG-09`,
/// `TXT-15`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DensityAxis {
    /// Explicit density override; `None` means "inherited" (`CROSS-08`;
    /// `BTN-05` default `null`).
    pub explicit: Option<ControlDensity>,
    /// Per-component density adjustments, including the two documented
    /// exceptions (`CROSS-08`; `R §8` RangeSlider vertical hit-area padding,
    /// `T §8` TextInput block padding).
    pub adjustments: Vec<DensityAdjustment>,
}

/// One declared density adjustment for a component (`CROSS-08`).
///
/// The two documented exceptions are first-class here: a `DensityAdjustment`
/// with `applies_to` naming a part records the RangeSlider vertical hit-area
/// padding (`RNG-09`) and the TextInput block padding (`TXT-15`) adjustments
/// that the base ladder does not cover.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DensityAdjustment {
    /// The density this adjustment applies to.
    pub density: ControlDensity,
    /// Part id the adjustment applies to; `None` means the whole control.
    /// The documented exceptions use this to name the affected part
    /// (`RNG-09`, `TXT-15`).
    pub applies_to: Option<Identifier>,
    /// Inline-axis delta, e.g. `-0.125rem` (`TXT-15` compact inline).
    pub inline: Option<MetricValue>,
    /// Block-axis delta, e.g. `-0.0625rem` (`TXT-15` compact block).
    pub block: Option<MetricValue>,
    /// What the adjustment changes, citing the contract density section.
    pub description: String,
}

/// Orientation — horizontal/vertical (`CROSS-11`; `R §3/§7/§8`; `RNG-07`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

/// The orientation axis of a component (`CROSS-11`, `RNG-07`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAxis {
    /// Default orientation.
    pub default: Orientation,
    /// Orientations the component supports, e.g. `[horizontal, vertical]`
    /// for RangeSlider (`RNG-07`).
    pub values: Vec<Orientation>,
}
