use flint_tokens::semantic;

use crate::types::Orientation;

#[derive(Clone, Debug, PartialEq)]
pub struct RangeSliderSpec {
    pub low: f64,
    pub high: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub orientation: Orientation,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
}

impl Default for RangeSliderSpec {
    fn default() -> Self {
        Self {
            low: 0.0,
            high: 100.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
        }
    }
}

impl RangeSliderSpec {
    pub fn new(low: f64, high: f64) -> Self {
        Self {
            low,
            high,
            ..Self::default()
        }
    }

    pub fn with_bounds(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn with_step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn clamped_low(&self) -> f64 {
        self.low.max(self.min).min(self.high)
    }

    pub fn clamped_high(&self) -> f64 {
        self.high.max(self.low).min(self.max)
    }

    pub fn normalized_low(&self) -> f64 {
        if self.max <= self.min {
            0.0
        } else {
            (self.clamped_low() - self.min) / (self.max - self.min)
        }
    }

    pub fn normalized_high(&self) -> f64 {
        if self.max <= self.min {
            0.0
        } else {
            (self.clamped_high() - self.min) / (self.max - self.min)
        }
    }

    pub fn range_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    pub fn track_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_SURFACE
    }

    pub fn focus_ring_color_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_FOCUS_RING
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
    }
}
