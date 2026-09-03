//! Popover — an anchored overlay surface with its own dismiss layer.
//!
//! This module is the single authority for the Popover declaration surface:
//! the struct, its defaults and builders, then the token recipes and derived
//! queries beside them. `g14.005` briefly generated the first half from a
//! TypeScript interface; `g14.008` rejected that path and `g14.021` restored
//! the hand-written declaration.
//!
//! Contract: `docs/contracts/components/popover.md`

use poodle_tokens::semantic;

#[derive(Clone, Debug, PartialEq)]
pub struct PopoverSpec {
    pub open: Option<bool>,
    pub default_open: bool,
    pub placement: crate::types::OverlayPlacement,
    pub offset: f32,
    pub dismiss_on_outside_interact: bool,
    pub initial_focus: crate::types::PopoverInitialFocus,
    pub aria_label: Option<String>,
    pub block: bool,
    pub disabled: bool,
    pub surface_width: crate::types::PopoverSurfaceWidth,
    pub surface_min_width: Option<crate::types::Dimension>,
    pub surface_max_width: Option<crate::types::Dimension>,
}

impl Default for PopoverSpec {
    fn default() -> Self {
        Self {
            open: None,
            default_open: false,
            placement: crate::types::OverlayPlacement::BottomStart,
            offset: 8.0,
            dismiss_on_outside_interact: true,
            initial_focus: crate::types::PopoverInitialFocus::FirstFocusable,
            aria_label: None,
            block: false,
            disabled: false,
            surface_width: crate::types::PopoverSurfaceWidth::Content,
            surface_min_width: None,
            surface_max_width: None,
        }
    }
}

impl PopoverSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_open(mut self, value: bool) -> Self {
        self.open = Some(value);
        self
    }
    pub fn with_default_open(mut self, value: bool) -> Self {
        self.default_open = value;
        self
    }
    pub fn with_placement(mut self, value: crate::types::OverlayPlacement) -> Self {
        self.placement = value;
        self
    }
    pub fn with_offset(mut self, value: f32) -> Self {
        self.offset = value;
        self
    }
    pub fn with_dismiss_on_outside_interact(mut self, value: bool) -> Self {
        self.dismiss_on_outside_interact = value;
        self
    }
    pub fn with_initial_focus(mut self, value: crate::types::PopoverInitialFocus) -> Self {
        self.initial_focus = value;
        self
    }
    pub fn with_aria_label(mut self, value: impl Into<String>) -> Self {
        self.aria_label = Some(value.into());
        self
    }
    pub fn with_block(mut self, value: bool) -> Self {
        self.block = value;
        self
    }
    pub fn with_disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }
    pub fn with_surface_width(mut self, value: crate::types::PopoverSurfaceWidth) -> Self {
        self.surface_width = value;
        self
    }
    pub fn with_surface_min_width(mut self, value: impl Into<crate::types::Dimension>) -> Self {
        self.surface_min_width = Some(value.into());
        self
    }
    pub fn with_surface_max_width(mut self, value: impl Into<crate::types::Dimension>) -> Self {
        self.surface_max_width = Some(value.into());
        self
    }
}


/// Default surface min-width in rem (contract §7 / §8 `14rem`).
pub const POPOVER_SURFACE_MIN_WIDTH_REM: f32 = 14.0;
/// Default surface max-width in rem (contract §7 / §8 `min(24rem, 90vw)`;
/// the `90vw` clamp is viewport-relative and not expressible in the Rust
/// targets, so they resolve the `24rem` arm).
pub const POPOVER_SURFACE_MAX_WIDTH_REM: f32 = 24.0;

/// Parses a `Dimension` like `"14rem"` into rem. The portable unit is rem
/// (contract §12): a non-rem value is a web-shell CSS length that does not
/// port, and it is an authoring error — never a silent default. The case
/// corpus and the Rust codegen reject such values before they reach this
/// parser, so an assert here means a caller violated the portable contract.
fn rem_of(dimension: &Option<crate::types::Dimension>, default: f32) -> f32 {
    let Some(dimension) = dimension else {
        return default;
    };
    let raw = dimension.as_str();
    let Some(value) = raw.strip_suffix("rem") else {
        panic!(
            "PopoverSpec surface width bound `{raw}` is not a portable rem length (contract §12); arbitrary CSS lengths are a web-shell extension"
        );
    };
    let mut digits = value.split('.');
    let whole = digits.next().unwrap_or_default();
    let shape_is_valid = !whole.is_empty()
        && whole.chars().all(|ch| ch.is_ascii_digit())
        && match digits.next() {
            None => true,
            Some(frac) => {
                digits.next().is_none()
                    && !frac.is_empty()
                    && frac.chars().all(|ch| ch.is_ascii_digit())
            }
        };
    match value.parse::<f32>() {
        Ok(rem) if shape_is_valid && rem.is_finite() => rem,
        _ => panic!(
            "PopoverSpec surface width bound `{raw}` is not a portable rem length (contract §12)"
        ),
    }
}

impl PopoverSpec {
    /// The effective open state: the controlled value when supplied, else the
    /// uncontrolled default.
    pub fn current_open(&self) -> bool {
        self.open.unwrap_or(self.default_open)
    }

    /// Effective surface min-width in rem (override or contract default `14rem`).
    pub fn effective_surface_min_width_rem(&self) -> f32 {
        rem_of(&self.surface_min_width, POPOVER_SURFACE_MIN_WIDTH_REM)
    }

    /// Effective surface max-width in rem (override or contract default `24rem`).
    pub fn effective_surface_max_width_rem(&self) -> f32 {
        rem_of(&self.surface_max_width, POPOVER_SURFACE_MAX_WIDTH_REM)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Dimension, OverlayPlacement, PopoverInitialFocus, PopoverSurfaceWidth,
    };

    #[test]
    fn portable_width_bound_accepts_rem() {
        let spec = PopoverSpec::default().with_surface_min_width(Dimension::new("20.5rem"));
        assert_eq!(spec.effective_surface_min_width_rem(), 20.5);
    }

    #[test]
    #[should_panic(expected = "not a portable rem length")]
    fn portable_width_bound_rejects_css_extensions() {
        let spec = PopoverSpec::default().with_surface_min_width(Dimension::new("320px"));
        let _ = spec.effective_surface_min_width_rem();
    }

    #[test]
    fn defaults_match_the_contract() {
        // Contract §3: `open` null (uncontrolled), `defaultOpen` false,
        // placement bottom-start, offset 8, outside dismissal on,
        // first-focusable initial focus, no label, no block, enabled,
        // content-driven surface width, contract default width bounds.
        let spec = PopoverSpec::default();
        assert_eq!(spec.open, None);
        assert!(!spec.default_open);
        assert_eq!(spec.placement, OverlayPlacement::BottomStart);
        assert_eq!(spec.offset, 8.0);
        assert!(spec.dismiss_on_outside_interact);
        assert_eq!(spec.initial_focus, PopoverInitialFocus::FirstFocusable);
        assert_eq!(spec.aria_label, None);
        assert!(!spec.block);
        assert!(!spec.disabled);
        assert_eq!(spec.surface_width, PopoverSurfaceWidth::Content);
        assert_eq!(spec.surface_min_width, None);
        assert_eq!(spec.surface_max_width, None);
    }

    #[test]
    fn builders_cover_the_public_prop_surface() {
        let spec = PopoverSpec::new()
            .with_open(false)
            .with_default_open(true)
            .with_placement(OverlayPlacement::TopEnd)
            .with_offset(14.0)
            .with_dismiss_on_outside_interact(false)
            .with_initial_focus(PopoverInitialFocus::None)
            .with_aria_label("Quick settings")
            .with_block(true)
            .with_disabled(true)
            .with_surface_width(PopoverSurfaceWidth::Trigger);
        assert_eq!(spec.open, Some(false));
        assert!(spec.default_open);
        assert_eq!(spec.placement, OverlayPlacement::TopEnd);
        assert_eq!(spec.offset, 14.0);
        assert!(!spec.dismiss_on_outside_interact);
        assert_eq!(spec.initial_focus, PopoverInitialFocus::None);
        assert_eq!(spec.aria_label.as_deref(), Some("Quick settings"));
        assert!(spec.block);
        assert!(spec.disabled);
        assert_eq!(spec.surface_width, PopoverSurfaceWidth::Trigger);
    }

    #[test]
    fn current_open_prefers_the_controlled_value() {
        let uncontrolled = PopoverSpec::new().with_default_open(true);
        assert!(uncontrolled.current_open());
        let controlled_closed = PopoverSpec::new().with_default_open(true).with_open(false);
        assert!(!controlled_closed.current_open());
        let controlled_open = PopoverSpec::new().with_open(true);
        assert!(controlled_open.current_open());
    }

    #[test]
    fn surface_token_recipes_match_the_contract() {
        // Contract §8: background = background-elevated; border = border-subtle
        // at 74%; shadow = the overlay elevation recipe.
        let spec = PopoverSpec::new();
        assert_eq!(spec.surface_fill_token(), semantic::COLOR_BACKGROUND_ELEVATED);
        assert_eq!(spec.surface_border_token(), semantic::COLOR_BORDER_SUBTLE);
        assert_eq!(spec.surface_border_alpha(), 0.74);
        assert_eq!(spec.shadow_token(), semantic::ELEVATION_OVERLAY);
    }

    #[test]
    fn effective_width_bounds_fall_back_to_contract_rems() {
        // Contract §7/§8: default surface min-width 14rem, max-width 24rem
        // (the portable arm of `min(24rem, 90vw)`), both overridable.
        let spec = PopoverSpec::new();
        assert_eq!(spec.effective_surface_min_width_rem(), 14.0);
        assert_eq!(spec.effective_surface_max_width_rem(), 24.0);
        let overridden = PopoverSpec::new()
            .with_surface_min_width(Dimension::new("20rem"))
            .with_surface_max_width(Dimension::new("20rem"));
        assert_eq!(overridden.effective_surface_min_width_rem(), 20.0);
        assert_eq!(overridden.effective_surface_max_width_rem(), 20.0);
    }
}
