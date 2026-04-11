use poodle_tokens::semantic;

use crate::types::OverlayPlacement;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoverCardSpec {
    pub is_open: bool,
    /// When true, the card starts open. Distinct from `is_open` —
    /// this is an initial-state signal for uncontrolled usage.
    pub default_open: bool,
    pub placement: OverlayPlacement,
    /// Hover-in delay in milliseconds before the card appears.
    /// Matches Svelte `openDelayMs` (default 400).
    pub open_delay_ms: u32,
    /// Hover-out delay in milliseconds before the card dismisses.
    /// Matches Svelte `closeDelayMs` (default 150).
    pub close_delay_ms: u32,
}

impl Default for HoverCardSpec {
    fn default() -> Self {
        Self {
            is_open: false,
            default_open: false,
            placement: OverlayPlacement::Bottom,
            open_delay_ms: 400,
            close_delay_ms: 150,
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

    pub fn with_default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn with_placement(mut self, placement: OverlayPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_open_delay_ms(mut self, delay: u32) -> Self {
        self.open_delay_ms = delay;
        self
    }

    pub fn with_close_delay_ms(mut self, delay: u32) -> Self {
        self.close_delay_ms = delay;
        self
    }

    /// Resolved effective open state honouring the default when no
    /// controlled value has been set.
    pub fn current_open(&self) -> bool {
        // With bool `is_open`, we can't distinguish "explicitly
        // false" from "uncontrolled" — callers that want controlled
        // behaviour set is_open directly. Default state is read when
        // the consumer chooses to honour it.
        self.is_open || self.default_open
    }

    pub fn fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
