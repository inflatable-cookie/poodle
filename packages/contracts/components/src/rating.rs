use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_headless::rating::{
    clamp_rating_display_value, rating_fill_ratio, rating_item_count, resolve_rating_step,
};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct RatingSpec {
    pub value: Option<f64>,
    /// Uncontrolled initial value (contract `defaultValue`). Display resolves
    /// `value.or(default_value)`; both absent is empty.
    pub default_value: Option<f64>,
    pub max: u8,
    pub is_disabled: bool,
    /// Interactive input increment (contract `step`). Values below `1` enable
    /// fractional/slider mode; `1` keeps whole-star selection.
    pub step: f64,
    /// Whether selecting the current value deselects it (contract `allowClear`).
    pub allow_clear: bool,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Accessible name (contract §7). `None` falls back to the visible label.
    pub aria_label: Option<String>,
}

impl Default for RatingSpec {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            max: 5,
            is_disabled: false,
            step: 0.5,
            allow_clear: false,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            aria_label: None,
        }
    }
}

impl RatingSpec {
    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn with_max(mut self, max: u8) -> Self {
        self.max = max;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_default_value(mut self, default_value: f64) -> Self {
        self.default_value = Some(default_value);
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

    /// Effective star count: at least one item.
    pub fn item_count(&self) -> u8 {
        rating_item_count(self.max)
    }

    /// Authored or default display value. Both absent is empty (`None`).
    pub fn current_value(&self) -> Option<f64> {
        clamp_rating_display_value(self.value.or(self.default_value), self.item_count() as f64)
    }

    /// Effective interactive step, clamped to `(0, 1]` (mirrors web
    /// `resolveRatingStep`): non-finite or non-positive falls back to `1`.
    pub fn effective_step(&self) -> f64 {
        resolve_rating_step(self.step)
    }

    /// Fractional (slider-semantics) mode when the effective step is below 1.
    pub fn is_fractional(&self) -> bool {
        self.effective_step() < 1.0
    }

    pub fn filled_count(&self) -> u8 {
        self.current_value()
            .map(|value| value.floor() as u8)
            .unwrap_or(0)
    }

    pub fn partial_fill(&self) -> f64 {
        self.current_value()
            .map(|value| value.fract())
            .unwrap_or(0.0)
    }

    /// Per-star fill ratio for the star at zero-based `index`, in `0.0..=1.0`.
    pub fn fill_ratio(&self, index: u8) -> f64 {
        let value = self.current_value().unwrap_or(0.0);
        rating_fill_ratio(index as f64, value)
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
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn with_density(mut self, density: ControlDensity) -> Self {
        self.density = Some(density);
        self
    }
}
