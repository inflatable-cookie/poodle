use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct RatingSpec {
    pub value: f64,
    /// Uncontrolled initial value (contract `defaultValue`). Display-only seed;
    /// the renderers read `value`, this records the seed for spec parity.
    pub default_value: f64,
    pub max: u8,
    pub is_readonly: bool,
    pub is_disabled: bool,
    /// Display precision (legacy field; retained for callers).
    pub precision: f64,
    /// Interactive input increment (contract `step`). Values below `1` enable
    /// fractional/slider mode; `1` keeps whole-star selection.
    pub step: f64,
    /// Whether clicking the current value deselects it (contract `allowClear`).
    pub allow_clear: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for RatingSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            default_value: 0.0,
            max: 5,
            is_readonly: false,
            is_disabled: false,
            precision: 1.0,
            step: 1.0,
            allow_clear: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl RatingSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn with_max(mut self, max: u8) -> Self {
        self.max = max;
        self
    }

    pub fn with_readonly(mut self, is_readonly: bool) -> Self {
        self.is_readonly = is_readonly;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_precision(mut self, precision: f64) -> Self {
        self.precision = precision;
        self
    }

    pub fn with_default_value(mut self, default_value: f64) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn with_step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn with_allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }

    /// Effective interactive step, clamped to `(0, 1]` (mirrors Svelte
    /// `resolveStep`): non-finite or non-positive falls back to `1`.
    pub fn effective_step(&self) -> f64 {
        if !self.step.is_finite() || self.step <= 0.0 {
            1.0
        } else {
            self.step.min(1.0)
        }
    }

    /// Fractional (slider-semantics) mode when the effective step is below 1.
    pub fn is_fractional(&self) -> bool {
        self.effective_step() < 1.0
    }

    pub fn filled_count(&self) -> u8 {
        let clamped = self.value.clamp(0.0, self.max as f64);
        clamped.floor() as u8
    }

    pub fn partial_fill(&self) -> f64 {
        let clamped = self.value.clamp(0.0, self.max as f64);
        clamped.fract()
    }

    /// Per-star fill ratio for the star at zero-based `index`, in `0.0..=1.0`.
    /// Mirrors Svelte `getFillRatio`: full stars below the value fill to `1`,
    /// the active star fills to its fractional remainder, the rest to `0`.
    pub fn fill_ratio(&self, index: u8) -> f64 {
        let clamped = self.value.clamp(0.0, self.max as f64);
        (clamped - index as f64).clamp(0.0, 1.0)
    }

    pub fn active_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Inactive (unfilled) star color token. Contract §8 unfilled color is
    /// `color-mix(text-secondary 48%, transparent)`, so renderers resolve this
    /// token then apply [`Self::inactive_color_alpha`] as the blend factor.
    pub fn inactive_color_token(&self) -> &'static str {
        semantic::COLOR_TEXT_SECONDARY
    }

    /// Alpha multiplier applied to [`Self::inactive_color_token`] to match the
    /// contract's `text-secondary 48%` mix.
    pub fn inactive_color_alpha(&self) -> f32 {
        0.48
    }

    /// Drop-shadow glow color token for the hovered item (contract §8 hover).
    pub fn hover_glow_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Focus-ring color token for `:focus-visible` (contract §8).
    pub fn focus_ring_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = density;
        self
    }
}
