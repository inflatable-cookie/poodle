use pug_tokens::semantic;

use crate::types::{ButtonTone, ButtonVariant, ControlSize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplitButtonSpec {
    pub variant: ButtonVariant,
    pub size: ControlSize,
    pub label: Option<String>,
    pub is_disabled: bool,
    pub is_open: bool,
}

impl Default for SplitButtonSpec {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            size: ControlSize::Md,
            label: None,
            is_disabled: false,
            is_open: false,
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

    pub fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_disabled(mut self, is_disabled: bool) -> Self {
        self.is_disabled = is_disabled;
        self
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        self.variant.fill_token(ButtonTone::Default)
    }

    pub fn border_token(&self) -> &'static str {
        self.variant.border_token(ButtonTone::Default)
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
}
