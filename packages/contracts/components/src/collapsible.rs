use crate::types::{ControlDensity, ControlSize, SemanticControlSizeRole};
use poodle_tokens::semantic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollapsibleSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_disabled: bool,
    pub highlighted: bool,
    pub aria_label: Option<String>,
    pub size: ControlSize,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for CollapsibleSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            title: None,
            description: None,
            is_disabled: false,
            highlighted: false,
            aria_label: None,
            size: ControlSize::Md,
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl CollapsibleSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn activation_allowed(&self) -> bool {
        !self.is_disabled
    }

    pub fn requires_accessible_name(&self) -> bool {
        self.title
            .as_ref()
            .map(|title| title.trim().is_empty())
            .unwrap_or(true)
            && self
                .aria_label
                .as_ref()
                .map(|label| label.trim().is_empty())
                .unwrap_or(true)
    }

    pub fn content_gap_token(&self) -> &'static str {
        semantic::SPACE_STACK_SM
    }

    /// Accent token sourcing the highlighted Root border + halo (contract §8).
    pub fn highlight_accent_token(&self) -> &'static str {
        semantic::COLOR_ACCENT_BASE
    }

    /// Alpha fraction applied to the accent for the highlighted border-color
    /// (`accent-base 55%`, contract §8 Root highlighted).
    pub fn highlight_border_alpha(&self) -> f32 {
        0.55
    }

    /// Alpha fraction applied to the accent for the highlighted halo
    /// (`accent-base 12%`, contract §8 Root highlighted).
    pub fn highlight_halo_alpha(&self) -> f32 {
        0.12
    }

    /// Border-subtle alpha fraction for the Root border
    /// (`border-subtle 36%`, contract §8 Root).
    pub fn border_subtle_alpha(&self) -> f32 {
        0.36
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
