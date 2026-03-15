use pug_tokens::semantic;

use crate::types::Orientation;

#[derive(Clone, Debug, PartialEq)]
pub struct SliderSpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub orientation: Orientation,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub value_text: Option<String>,
}

impl Default for SliderSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
            value_text: None,
        }
    }
}

impl SliderSpec {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    pub fn with_bounds(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn clamped_value(&self) -> f64 {
        self.value.max(self.min).min(self.max)
    }

    pub fn normalized_progress(&self) -> f64 {
        if self.max <= self.min {
            0.0
        } else {
            (self.clamped_value() - self.min) / (self.max - self.min)
        }
    }

    pub fn range_fill_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }
}
