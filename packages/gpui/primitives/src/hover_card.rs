use pug_gpui_tokens::semantic;

use crate::types::OverlayPlacement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoverCardSpec {
    pub is_open: bool,
    pub placement: OverlayPlacement,
}

impl Default for HoverCardSpec {
    fn default() -> Self {
        Self {
            is_open: false,
            placement: OverlayPlacement::Bottom,
        }
    }
}

impl HoverCardSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
