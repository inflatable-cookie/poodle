use poodle_tokens::semantic;

use crate::types::{OverlayPlacement, PopoverInitialFocus};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopoverSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: OverlayPlacement,
    pub offset: u16,
    pub dismiss_on_outside_interact: bool,
    pub initial_focus: PopoverInitialFocus,
    pub aria_label: Option<String>,
}

impl Default for PopoverSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            placement: OverlayPlacement::BottomStart,
            offset: 8,
            dismiss_on_outside_interact: true,
            initial_focus: PopoverInitialFocus::FirstFocusable,
            aria_label: None,
        }
    }
}

impl PopoverSpec {
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

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_offset(mut self, offset: u16) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_dismiss_on_outside_interact(mut self, dismiss_on_outside_interact: bool) -> Self {
        self.dismiss_on_outside_interact = dismiss_on_outside_interact;
        self
    }

    pub fn with_initial_focus(mut self, initial_focus: PopoverInitialFocus) -> Self {
        self.initial_focus = initial_focus;
        self
    }

    pub fn with_aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
