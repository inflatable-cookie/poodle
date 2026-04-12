use poodle_tokens::semantic;

use crate::types::{
    ButtonTone, ButtonVariant, ControlDensity, ControlSize, SemanticControlSizeRole,
};

/// A menu item for the split button dropdown.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SplitMenuItem {
    Action {
        value: String,
        label: String,
        is_disabled: bool,
    },
    Separator,
}

impl SplitMenuItem {
    pub fn action(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self::Action {
            value: value.into(),
            label: label.into(),
            is_disabled: false,
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        if let Self::Action {
            is_disabled: ref mut d,
            ..
        } = self
        {
            *d = is_disabled;
        }
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitButtonSpec {
    pub variant: ButtonVariant,
    pub tone: ButtonTone,
    pub size: ControlSize,
    pub label: Option<String>,
    pub items: Vec<SplitMenuItem>,
    pub is_disabled: bool,
    pub is_loading: bool,
    pub is_open: bool,
    pub aria_label: Option<String>,
    pub menu_aria_label: String,
    pub size_role: SemanticControlSizeRole,
    pub density: ControlDensity,
}

impl Default for SplitButtonSpec {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            tone: ButtonTone::Default,
            size: ControlSize::Md,
            label: None,
            items: Vec::new(),
            is_disabled: false,
            is_loading: false,
            is_open: false,
            aria_label: None,
            menu_aria_label: "More actions".to_string(),
            size_role: SemanticControlSizeRole::Control,
            density: ControlDensity::Default,
        }
    }
}

impl SplitButtonSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_tone(mut self, tone: ButtonTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_items(mut self, items: Vec<SplitMenuItem>) -> Self {
        self.items = items;
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn with_menu_aria_label(mut self, label: impl Into<String>) -> Self {
        self.menu_aria_label = label.into();
        self
    }

    pub fn is_unavailable(&self) -> bool {
        self.is_disabled || self.is_loading
    }

    pub fn fill_token(&self) -> &'static str {
        self.variant.fill_token(self.tone)
    }

    pub fn border_token(&self) -> &'static str {
        self.variant.border_token(self.tone)
    }

    pub fn text_token(&self) -> &'static str {
        self.variant.text_token(self.tone)
    }

    pub fn separator_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    pub fn overlay_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }

    pub fn control_height_token(&self) -> &'static str {
        self.size.control_height_token()
    }

    pub fn radius_token(&self) -> &'static str {
        semantic::RADIUS_CONTROL
    }

    pub fn disabled_opacity_token(&self) -> &'static str {
        semantic::STATE_OPACITY_DISABLED
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
