use poodle_tokens::semantic;

pub use crate::generated::popover::PopoverSpec;

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
    use crate::types::Dimension;

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
}
