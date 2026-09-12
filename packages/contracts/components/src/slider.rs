use poodle_headless::audio::AudioValueLaw;
use poodle_tokens::semantic;

pub use poodle_headless::slider::SliderPolarity;

use crate::types::{ControlDensity, ControlSize, Orientation, SemanticControlSizeRole};

/// g18.022: `block` is the ordinary standalone capsule and the public
/// default; `embedded` keeps the dense track-and-thumb treatment for
/// composite use. There are no aliases for the removed track/standard
/// vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderVariant {
    #[default]
    Block,
    Embedded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SliderDirection {
    #[default]
    Ltr,
    Rtl,
}

impl SliderDirection {
    pub fn is_rtl(self) -> bool {
        self == Self::Rtl
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderSpec {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub variant: SliderVariant,
    pub direction: SliderDirection,
    pub polarity: SliderPolarity,
    pub center_value: Option<f64>,
    pub law: AudioValueLaw,
    pub orientation: Orientation,
    pub is_disabled: bool,
    pub aria_label: Option<String>,
    pub value_text: Option<String>,
    pub visible_label: Option<String>,
    pub visible_value_text: Option<String>,
    pub size: Option<ControlSize>,
    pub size_role: SemanticControlSizeRole,
    pub density: Option<ControlDensity>,
}

impl Default for SliderSpec {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            variant: SliderVariant::Block,
            direction: SliderDirection::Ltr,
            polarity: SliderPolarity::Unipolar,
            center_value: None,
            law: AudioValueLaw::Linear,
            orientation: Orientation::Horizontal,
            is_disabled: false,
            aria_label: None,
            value_text: None,
            visible_label: None,
            visible_value_text: None,
            size: None,
            size_role: SemanticControlSizeRole::Control,
            density: None,
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

    pub fn with_embedded_control(mut self, polarity: SliderPolarity) -> Self {
        self.variant = SliderVariant::Embedded;
        self.polarity = polarity;
        self
    }

    pub fn with_law(mut self, law: AudioValueLaw) -> Self {
        self.law = law;
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

    pub fn with_variant(mut self, variant: SliderVariant) -> Self {
        self.variant = variant;
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

    pub fn with_visible_value_text(mut self, text: impl Into<String>) -> Self {
        self.visible_value_text = Some(text.into());
        self
    }
}
