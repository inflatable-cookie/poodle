use crate::types::{ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct MeterSpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub optimum: Option<f64>,
    pub aria_label: Option<String>,
    /// Explicit track-thickness size override. When `None`, resolves from
    /// `size_role` against the inherited presentation scale.
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
}

impl Default for MeterSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            // Contract §3: max default is 100 (matches Svelte).
            max: 100.0,
            low: None,
            high: None,
            optimum: None,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
        }
    }
}

impl MeterSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_low(mut self, low: f64) -> Self {
        self.low = Some(low);
        self
    }

    pub fn with_high(mut self, high: f64) -> Self {
        self.high = Some(high);
        self
    }

    pub fn with_optimum(mut self, optimum: f64) -> Self {
        self.optimum = Some(optimum);
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_size_role(mut self, size_role: SemanticControlSizeRole) -> Self {
        self.size_role = size_role;
        self
    }

    pub fn normalized_progress(&self) -> f64 {
        let range = self.max - self.min;
        if range <= 0.0 {
            0.0
        } else {
            ((self.value - self.min) / range).clamp(0.0, 1.0)
        }
    }

    /// Contract §8: fill base is `--poodle-color-status-success` (gradient
    /// endpoint + base). Previously returned accent-base — that was a bug
    /// inherited by any target trusting the spec.
    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_STATUS_SUCCESS
    }

    /// Track background base. Contract §8: `color-mix(in srgb,
    /// var(--poodle-surface) 96%, var(--poodle-color-text-primary))`.
    /// This is the base side of that mix.
    pub fn track_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    /// The token mixed into the track base (contract §8 mix partner).
    pub fn track_mix_token(&self) -> &'static str {
        semantic::COLOR_TEXT_PRIMARY
    }

    /// Track-background mix ratio for the base token (contract §8: 96%).
    pub fn track_mix_ratio(&self) -> f32 {
        0.96
    }

    /// Contract §8 Size Variants: track thickness in rem per size.
    ///
    /// | xs 0.25 | sm 0.375 | md 0.5 | lg 0.625 | xl 0.75 |
    pub fn track_thickness_rem(&self, size: ControlSize) -> f32 {
        match size {
            ControlSize::Xs => 0.25,
            ControlSize::Sm => 0.375,
            ControlSize::Md => 0.5,
            ControlSize::Lg => 0.625,
            ControlSize::Xl => 0.75,
        }
    }

    /// Track height for the default (md) size. Retained for back-compat;
    /// prefer `track_thickness_rem(size)` once a size is resolved.
    pub fn track_height_rem(&self) -> f32 {
        self.track_thickness_rem(ControlSize::Md)
    }
}
