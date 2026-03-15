use pug_tokens::semantic;

use crate::types::OverlayPlacement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TooltipSpec {
    pub content: Option<String>,
    pub open: Option<bool>,
    pub default_open: bool,
    pub delay_ms: u16,
    pub placement: OverlayPlacement,
    pub aria_label: Option<String>,
}

impl Default for TooltipSpec {
    fn default() -> Self {
        Self {
            content: None,
            open: None,
            default_open: false,
            delay_ms: 400,
            placement: OverlayPlacement::Top,
            aria_label: None,
        }
    }
}

impl TooltipSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_delay_ms(mut self, delay_ms: u16) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn has_content(&self) -> bool {
        self.content
            .as_ref()
            .map(|content| !content.trim().is_empty())
            .unwrap_or(false)
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }
}
