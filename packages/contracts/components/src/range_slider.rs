use poodle_headless::audio::AudioValueLaw;
use poodle_tokens::semantic;

use crate::types::{ControlDensity, ControlSize, Orientation, SemanticControlSizeRole};
use crate::{SliderAppearance, SliderDirection, SliderPolarity, SliderVariant};

#[derive(Clone, Debug, PartialEq)]
pub struct RangeSliderSpec {
    pub low: f64,
    pub high: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub variant: SliderVariant,
    pub appearance: SliderAppearance,
    pub direction: SliderDirection,
    pub polarity: SliderPolarity,
    pub center_value: Option<f64>,
    pub law: AudioValueLaw,
    pub orientation: Orientation,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
    /// Human-readable value for the lower thumb (`aria-valuetext`). Without it
    /// a screen reader announces the raw number, which is meaningless for a
    /// scale like "Small / Medium / Large".
    pub lower_value_text: Option<String>,
    /// Human-readable value for the upper thumb (`aria-valuetext`).
    pub upper_value_text: Option<String>,
    pub visible_label: Option<String>,
    pub visible_lower_text: Option<String>,
    pub visible_upper_text: Option<String>,
    pub visible_range_text: Option<String>,
}

impl Default for RangeSliderSpec {
    fn default() -> Self {
        Self {
            low: 0.0,
            high: 100.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            variant: SliderVariant::Standard,
            appearance: SliderAppearance::Track,
            direction: SliderDirection::Ltr,
            polarity: SliderPolarity::Unipolar,
            center_value: None,
            law: AudioValueLaw::Linear,
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
            lower_value_text: None,
            upper_value_text: None,
            visible_label: None,
            visible_lower_text: None,
            visible_upper_text: None,
            visible_range_text: None,
        }
    }
}

impl RangeSliderSpec {
    pub fn with_value_text(mut self, lower: impl Into<String>, upper: impl Into<String>) -> Self {
        self.lower_value_text = Some(lower.into());
        self.upper_value_text = Some(upper.into());
        self
    }

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

    pub fn with_embedded_control(mut self, polarity: SliderPolarity) -> Self {
        self.variant = SliderVariant::Embedded;
        self.polarity = polarity;
        self
    }

    pub fn with_law(mut self, law: AudioValueLaw) -> Self {
        self.law = law;
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

    pub fn with_appearance(mut self, appearance: SliderAppearance) -> Self {
        self.appearance = appearance;
        self
    }

    pub fn with_direction(mut self, direction: SliderDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_visible_label(mut self, label: impl Into<String>) -> Self {
        self.visible_label = Some(label.into());
        self
    }

    pub fn with_visible_lower_text(mut self, text: impl Into<String>) -> Self {
        self.visible_lower_text = Some(text.into());
        self
    }

    pub fn with_visible_upper_text(mut self, text: impl Into<String>) -> Self {
        self.visible_upper_text = Some(text.into());
        self
    }

    pub fn with_visible_range_text(mut self, text: impl Into<String>) -> Self {
        self.visible_range_text = Some(text.into());
        self
    }
}
