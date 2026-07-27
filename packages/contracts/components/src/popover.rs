use poodle_tokens::semantic;

use crate::types::{Dimension, OverlayPlacement, PopoverInitialFocus, PopoverSurfaceWidth};

/// Default surface min-width in rem (contract §7 / §8 `14rem`).
pub const POPOVER_SURFACE_MIN_WIDTH_REM: f32 = 14.0;
/// Default surface max-width in rem (contract §7 / §8 `min(24rem, 90vw)`;
/// the `90vw` clamp is viewport-relative and not expressible in the Rust
/// targets, so they resolve the `24rem` arm).
pub const POPOVER_SURFACE_MAX_WIDTH_REM: f32 = 24.0;

#[derive(Clone, Debug, PartialEq)]
pub struct PopoverSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: OverlayPlacement,
    pub offset: u16,
    pub dismiss_on_outside_interact: bool,
    pub initial_focus: PopoverInitialFocus,
    pub aria_label: Option<String>,
    /// Disables the trigger — blocks open, sets `data-disabled`/`aria-disabled`,
    /// `tabindex=-1`, and `cursor: not-allowed` (contract §3 `disabled`).
    pub disabled: bool,
    /// Expands trigger + root to available width (contract §3 `block`).
    pub block: bool,
    /// Surface width strategy (contract §3 `surfaceWidth`).
    pub surface_width: PopoverSurfaceWidth,
    /// Overrides the default `14rem` surface min-width, in rem
    /// (contract §3 `surfaceMinWidth`). `None` → default.
    pub surface_min_width_rem: Option<f32>,
    /// Overrides the default `24rem` surface max-width, in rem
    /// (contract §3 `surfaceMaxWidth`). `None` → default.
    pub surface_max_width_rem: Option<f32>,

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
            disabled: false,
            block: false,
            surface_width: PopoverSurfaceWidth::Content,
            surface_min_width_rem: None,
            surface_max_width_rem: None,
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

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    pub fn with_surface_width(mut self, surface_width: PopoverSurfaceWidth) -> Self {
        self.surface_width = surface_width;
        self
    }

    pub fn with_surface_min_width_rem(mut self, rem: f32) -> Self {
        self.surface_min_width_rem = Some(rem);
        self
    }

    pub fn with_surface_max_width_rem(mut self, rem: f32) -> Self {
        self.surface_max_width_rem = Some(rem);
        self
    }

    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    /// Effective surface min-width in rem (override or contract default `14rem`).
    pub fn effective_surface_min_width_rem(&self) -> f32 {
        self.surface_min_width_rem.unwrap_or(POPOVER_SURFACE_MIN_WIDTH_REM)
    }

    /// Effective surface max-width in rem (override or contract default `24rem`).
    pub fn effective_surface_max_width_rem(&self) -> f32 {
        self.surface_max_width_rem.unwrap_or(POPOVER_SURFACE_MAX_WIDTH_REM)
    }

    pub fn surface_fill_token(&self) -> &'static str {
        semantic::COLOR_BACKGROUND_ELEVATED
    }

    /// Surface border color token (contract §8: `border-subtle` at 74%).
    pub fn surface_border_token(&self) -> &'static str {
        semantic::COLOR_BORDER_SUBTLE
    }

    /// Alpha multiplier applied to the surface border color (contract §8: 74%).
    pub fn surface_border_alpha(&self) -> f32 {
        0.74
    }

    pub fn shadow_token(&self) -> &'static str {
        semantic::ELEVATION_OVERLAY
    }
}
