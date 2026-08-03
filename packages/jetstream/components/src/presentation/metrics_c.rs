//! Jetstream per-size metric helpers. Split out of `presentation/mod.rs`
//! (god-file decomposition); pure scalar functions.

use poodle_specs::{ControlDensity, ControlSize};

/// Table body font-size in rem per size. Contract §8: xs 0.6875, sm 0.75,
/// md 0.8125, lg 0.875, xl 0.9375.
pub fn table_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Column-header font-size in rem per size. Contract §8: xs 0.5625, sm 0.625,
/// md 0.6875, lg 0.75, xl 0.8125.
pub fn table_header_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5625,
        ControlSize::Sm => 0.625,
        ControlSize::Md => 0.6875,
        ControlSize::Lg => 0.75,
        ControlSize::Xl => 0.8125,
    }
}

/// Header/cell vertical `padding-block` in rem per size. Contract §8: md is the
/// 0.5rem baseline; xs 0.3125, sm 0.375, lg 0.625, xl 0.75.
pub fn table_cell_pad_block_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.3125,
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.625,
        ControlSize::Xl => 0.75,
    }
}

/// Header/cell horizontal `padding-inline` in rem per density. Contract §8:
/// compact 0.5, default 0.75 (baseline), comfortable 1.125.
pub fn table_cell_pad_inline_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 1.125,
    }
}

// ── Toolbar size / density scales ────────────────────────────────
//
// Contract `toolbar.md` §8 "Size Variants" / "Density Variants". Size scales
// both block/inline padding and gap; density overrides only inline padding and
// gap (block padding / height is never touched by density). Mirrors the Svelte
// `.poodle-toolbar[data-size]` / `[data-density]` overrides, and
// `poodle_gpui::presentation::toolbar_*` exactly.

/// Toolbar vertical `padding-block` in rem per size. Contract §8: xs 0.125,
/// sm 0.1875, md 0.25, lg 0.3125, xl 0.375.
pub fn toolbar_pad_block_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.125,
        ControlSize::Sm => 0.1875,
        ControlSize::Md => 0.25,
        ControlSize::Lg => 0.3125,
        ControlSize::Xl => 0.375,
    }
}

/// Toolbar horizontal `padding-inline` in rem per size (before density
/// override). Contract §8: xs 0.25, sm 0.3125, md 0.375, lg 0.5, xl 0.625.
pub fn toolbar_pad_inline_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.3125,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.625,
    }
}

/// Toolbar gap in rem per size (before density override). Contract §8: xs 0.25,
/// sm 0.3125, md 0.375, lg 0.5, xl 0.625.
pub fn toolbar_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.3125,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.625,
    }
}

/// Toolbar horizontal `padding-inline` override in rem per density, or `None`
/// for `default` (use the size value). Contract §8: compact 0.25,
/// comfortable 0.5.
pub fn toolbar_density_pad_inline_rem(density: ControlDensity) -> Option<f32> {
    match density {
        ControlDensity::Compact => Some(0.25),
        ControlDensity::Default => None,
        ControlDensity::Comfortable => Some(0.5),
    }
}

/// Toolbar gap override in rem per density, or `None` for `default` (use the
/// size value). Contract §8: compact 0.25, comfortable 0.5.
pub fn toolbar_density_gap_rem(density: ControlDensity) -> Option<f32> {
    match density {
        ControlDensity::Compact => Some(0.25),
        ControlDensity::Default => None,
        ControlDensity::Comfortable => Some(0.5),
    }
}

/// Convert rem to pixels at the standard 16px base.
pub fn rem_to_px(rem: f32) -> f32 {
    rem * 16.0
}

/// Calendar day-cell / grid-column size (`--calendar-cell-size`) in rem.
///
/// Matches the Svelte Calendar per-size `--calendar-cell-size` values
/// (md `2.25rem`). This is a calendar-specific scale distinct from
/// `control_height_rem`, and also drives the day button `min-height`.
/// Mirrors `poodle_gpui::presentation::calendar_cell_size_rem`.
pub fn calendar_cell_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.75,
        ControlSize::Sm => 2.0,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.5,
        ControlSize::Xl => 2.75,
    }
}

/// Calendar nav button (prev/next month) width/height in rem.
///
/// Matches the Svelte Calendar per-size `.poodle-calendar__nav` sizes
/// (md `2rem`). Mirrors `poodle_gpui::presentation::calendar_nav_size_rem`.
pub fn calendar_nav_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.25,
        ControlSize::Xl => 2.5,
    }
}

/// Calendar day button font-size in rem.
///
/// Matches the Svelte Calendar per-size `.poodle-calendar__day` font-size
/// (md `0.75rem`). Distinct from the month-label font (`size_font_rem`).
/// Mirrors `poodle_gpui::presentation::calendar_day_font_rem`.
pub fn calendar_day_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::*;
    use poodle_specs::SemanticControlSizeRole;

    // ── resolve_semantic_size ────────────────────────────────────

    #[test]
    fn control_role_is_identity() {
        for size in [
            ControlSize::Xs,
            ControlSize::Sm,
            ControlSize::Md,
            ControlSize::Lg,
            ControlSize::Xl,
        ] {
            assert_eq!(
                resolve_semantic_size(size, SemanticControlSizeRole::Control),
                size
            );
        }
    }

    #[test]
    fn chrome_role_shifts_down_one_stop() {
        assert_eq!(
            resolve_semantic_size(ControlSize::Xl, SemanticControlSizeRole::Chrome),
            ControlSize::Lg
        );
        assert_eq!(
            resolve_semantic_size(ControlSize::Lg, SemanticControlSizeRole::Chrome),
            ControlSize::Md
        );
        assert_eq!(
            resolve_semantic_size(ControlSize::Md, SemanticControlSizeRole::Chrome),
            ControlSize::Sm
        );
    }

    #[test]
    fn chrome_role_clamps_at_floor() {
        assert_eq!(
            resolve_semantic_size(ControlSize::Sm, SemanticControlSizeRole::Chrome),
            ControlSize::Sm
        );
        assert_eq!(
            resolve_semantic_size(ControlSize::Xs, SemanticControlSizeRole::Chrome),
            ControlSize::Xs
        );
    }

    #[test]
    fn prominent_role_shifts_up_one_stop() {
        assert_eq!(
            resolve_semantic_size(ControlSize::Xs, SemanticControlSizeRole::Prominent),
            ControlSize::Sm
        );
        assert_eq!(
            resolve_semantic_size(ControlSize::Sm, SemanticControlSizeRole::Prominent),
            ControlSize::Md
        );
        assert_eq!(
            resolve_semantic_size(ControlSize::Md, SemanticControlSizeRole::Prominent),
            ControlSize::Lg
        );
    }

    #[test]
    fn prominent_role_clamps_at_ceiling() {
        assert_eq!(
            resolve_semantic_size(ControlSize::Xl, SemanticControlSizeRole::Prominent),
            ControlSize::Xl
        );
    }

    // ── control_height_rem ──────────────────────────────────────

    #[test]
    fn control_heights_match_svelte() {
        assert_eq!(control_height_rem(ControlSize::Xs), 1.5);
        assert_eq!(control_height_rem(ControlSize::Sm), 1.75);
        assert_eq!(control_height_rem(ControlSize::Md), 2.25);
        assert_eq!(control_height_rem(ControlSize::Lg), 2.75);
        assert_eq!(control_height_rem(ControlSize::Xl), 3.25);
    }

    // ── size_height_offset_rem ──────────────────────────────────

    #[test]
    fn height_offsets_match_svelte_css() {
        assert_eq!(size_height_offset_rem(ControlSize::Xs), -0.5);
        assert_eq!(size_height_offset_rem(ControlSize::Sm), -0.375);
        assert_eq!(size_height_offset_rem(ControlSize::Md), 0.0);
        assert_eq!(size_height_offset_rem(ControlSize::Lg), 0.375);
        assert_eq!(size_height_offset_rem(ControlSize::Xl), 0.5);
    }

    // ── size_font_rem ───────────────────────────────────────────

    #[test]
    fn font_sizes_match_svelte_css() {
        assert_eq!(size_font_rem(ControlSize::Xs), 0.6875);
        assert_eq!(size_font_rem(ControlSize::Sm), 0.75);
        assert_eq!(size_font_rem(ControlSize::Md), 0.8125);
        assert_eq!(size_font_rem(ControlSize::Lg), 0.875);
        assert_eq!(size_font_rem(ControlSize::Xl), 0.9375);
    }

    // ── size_min_width_rem ──────────────────────────────────────

    #[test]
    fn min_widths_match_svelte_css() {
        assert_eq!(size_min_width_rem(ControlSize::Xs), 3.75);
        assert_eq!(size_min_width_rem(ControlSize::Sm), 4.25);
        assert_eq!(size_min_width_rem(ControlSize::Md), 5.0);
        assert_eq!(size_min_width_rem(ControlSize::Lg), 5.75);
        assert_eq!(size_min_width_rem(ControlSize::Xl), 6.5);
    }

    // ── resolve_supporting_visual_size ──────────────────────────

    #[test]
    fn supporting_visual_shifts_down_one_stop() {
        assert_eq!(
            resolve_supporting_visual_size(ControlSize::Xl),
            ControlSize::Lg
        );
        assert_eq!(
            resolve_supporting_visual_size(ControlSize::Lg),
            ControlSize::Md
        );
        assert_eq!(
            resolve_supporting_visual_size(ControlSize::Md),
            ControlSize::Sm
        );
    }

    #[test]
    fn supporting_visual_clamps_at_bottom() {
        assert_eq!(
            resolve_supporting_visual_size(ControlSize::Sm),
            ControlSize::Sm
        );
        assert_eq!(
            resolve_supporting_visual_size(ControlSize::Xs),
            ControlSize::Xs
        );
    }

    // ── density helpers ─────────────────────────────────────────

    #[test]
    fn density_space_values_match_svelte() {
        assert_eq!(control_space_x_rem(ControlDensity::Compact), 0.5);
        assert_eq!(control_space_x_rem(ControlDensity::Default), 0.75);
        assert_eq!(control_space_x_rem(ControlDensity::Comfortable), 1.0);

        assert_eq!(panel_space_x_rem(ControlDensity::Compact), 0.75);
        assert_eq!(panel_space_x_rem(ControlDensity::Default), 1.0);
        assert_eq!(panel_space_x_rem(ControlDensity::Comfortable), 1.25);

        assert_eq!(panel_space_y_rem(ControlDensity::Compact), 0.5);
        assert_eq!(panel_space_y_rem(ControlDensity::Default), 0.75);
        assert_eq!(panel_space_y_rem(ControlDensity::Comfortable), 1.0);
    }

    // ── rem_to_px ───────────────────────────────────────────────

    #[test]
    fn rem_to_px_uses_16px_base() {
        assert_eq!(rem_to_px(1.0), 16.0);
        assert_eq!(rem_to_px(0.5), 8.0);
        assert_eq!(rem_to_px(2.25), 36.0);
    }
}
