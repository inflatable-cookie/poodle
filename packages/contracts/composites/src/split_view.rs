use crate::types::SplitOrientation;
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

#[derive(Clone, Debug, PartialEq)]
pub struct SplitViewSpec {
    pub orientation: SplitOrientation,
    pub ratio: Option<f32>,
    pub default_ratio: f32,
    pub min_primary_size: Option<f32>,
    pub min_secondary_size: Option<f32>,
    pub is_primary_collapsed: bool,
    pub is_secondary_collapsed: bool,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl SplitViewSpec {
    pub fn new(orientation: SplitOrientation) -> Self {
        Self {
            orientation,
            ratio: None,
            default_ratio: 0.5,
            min_primary_size: None,
            min_secondary_size: None,
            is_primary_collapsed: false,
            is_secondary_collapsed: false,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn with_default_ratio(mut self, default_ratio: f32) -> Self {
        self.default_ratio = default_ratio;
        self
    }

    pub fn with_min_primary_size(mut self, min_primary_size: f32) -> Self {
        self.min_primary_size = Some(min_primary_size);
        self
    }

    pub fn with_min_secondary_size(mut self, min_secondary_size: f32) -> Self {
        self.min_secondary_size = Some(min_secondary_size);
        self
    }

    pub fn with_primary_collapsed(mut self, is_primary_collapsed: bool) -> Self {
        self.is_primary_collapsed = is_primary_collapsed;
        self
    }

    pub fn with_secondary_collapsed(mut self, is_secondary_collapsed: bool) -> Self {
        self.is_secondary_collapsed = is_secondary_collapsed;
        self
    }

    pub fn current_ratio(&self) -> f32 {
        self.ratio.unwrap_or(self.default_ratio).clamp(0.0, 1.0)
    }

    pub fn keyboard_resize_supported(&self) -> bool {
        !self.is_primary_collapsed && !self.is_secondary_collapsed
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
