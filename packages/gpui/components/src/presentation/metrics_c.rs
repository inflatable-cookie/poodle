//! Per-size presentation metric helpers. Split out of
//! `presentation/mod.rs` (god-file decomposition); pure scalar functions.

//! Size-role and density resolution infrastructure for GPUI components.
//!
//! These helpers mirror the Svelte reference implementation in
//! `packages/svelte/components/src/presentation.ts` and ensure all GPUI
//! components resolve size, density, and layout offsets identically.

use poodle_specs::{ControlDensity, ControlSize};

/// Resolve a semantic size role against a base size to get the effective control size.
///
/// `Chrome` resolves one stop smaller (clamped at the minimum),
/// `Prominent` resolves one stop larger (clamped at the maximum),
/// `Control` is the identity mapping.
///
/// This exactly matches the Svelte `resolveSemanticControlSize` function.

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
// `.poodle-toolbar[data-size]` / `[data-density]` overrides.

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

#[cfg(test)]
mod tests {
    use super::*;

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

    // ── editable_list scales ────────────────────────────────────

    #[test]
    fn editable_list_scales_match_contract() {
        // Size adjustments (contract §8).
        assert_eq!(editable_list_handle_size_rem(ControlSize::Xs), 0.875);
        assert_eq!(editable_list_handle_size_rem(ControlSize::Sm), 1.0);
        assert_eq!(editable_list_handle_size_rem(ControlSize::Md), 1.0);
        assert_eq!(editable_list_handle_size_rem(ControlSize::Lg), 1.125);
        assert_eq!(editable_list_handle_size_rem(ControlSize::Xl), 1.25);

        assert_eq!(editable_list_item_x_rem(ControlSize::Xs), 0.5);
        assert_eq!(editable_list_item_x_rem(ControlSize::Md), 0.625);
        assert_eq!(editable_list_item_x_rem(ControlSize::Xl), 0.875);

        assert_eq!(editable_list_item_y_rem(ControlSize::Xs), 0.375);
        assert_eq!(editable_list_item_y_rem(ControlSize::Sm), 0.4375);
        assert_eq!(editable_list_item_y_rem(ControlSize::Md), 0.5);
        assert_eq!(editable_list_item_y_rem(ControlSize::Xl), 0.625);

        assert_eq!(editable_list_font_rem(ControlSize::Xs), 0.6875);
        assert_eq!(editable_list_font_rem(ControlSize::Md), 0.8125);
        assert_eq!(editable_list_font_rem(ControlSize::Xl), 0.9375);

        // Density adjustments (contract §8).
        assert_eq!(editable_list_list_gap_rem(ControlDensity::Compact), 0.0625);
        assert_eq!(editable_list_list_gap_rem(ControlDensity::Default), 0.125);
        assert_eq!(
            editable_list_list_gap_rem(ControlDensity::Comfortable),
            0.1875
        );

        assert_eq!(editable_list_item_gap_rem(ControlDensity::Compact), 0.375);
        assert_eq!(editable_list_item_gap_rem(ControlDensity::Default), 0.5);
        assert_eq!(
            editable_list_item_gap_rem(ControlDensity::Comfortable),
            0.625
        );
    }

    // ── duration_input scales ───────────────────────────────────

    #[test]
    fn duration_field_widths_match_contract() {
        assert_eq!(duration_field_width_rem(ControlSize::Xs), 1.5);
        assert_eq!(duration_field_width_rem(ControlSize::Sm), 1.625);
        assert_eq!(duration_field_width_rem(ControlSize::Md), 1.875);
        assert_eq!(duration_field_width_rem(ControlSize::Lg), 2.0);
        assert_eq!(duration_field_width_rem(ControlSize::Xl), 2.25);
    }

    #[test]
    fn duration_pad_y_matches_contract() {
        assert_eq!(duration_pad_y_rem(ControlSize::Xs), 0.125);
        assert_eq!(duration_pad_y_rem(ControlSize::Sm), 0.1875);
        assert_eq!(duration_pad_y_rem(ControlSize::Md), 0.25);
        assert_eq!(duration_pad_y_rem(ControlSize::Lg), 0.3125);
        assert_eq!(duration_pad_y_rem(ControlSize::Xl), 0.375);
    }

    #[test]
    fn duration_pad_x_offsets_match_contract() {
        assert_eq!(duration_pad_x_offset_rem(ControlSize::Xs), -0.125);
        assert_eq!(duration_pad_x_offset_rem(ControlSize::Sm), -0.0625);
        assert_eq!(duration_pad_x_offset_rem(ControlSize::Md), 0.0);
        assert_eq!(duration_pad_x_offset_rem(ControlSize::Lg), 0.125);
        assert_eq!(duration_pad_x_offset_rem(ControlSize::Xl), 0.1875);
    }

    #[test]
    fn duration_digit_font_overrides_match_contract() {
        assert_eq!(duration_digit_font_rem(ControlSize::Xs), Some(0.75));
        assert_eq!(duration_digit_font_rem(ControlSize::Sm), None);
        assert_eq!(duration_digit_font_rem(ControlSize::Md), None);
        assert_eq!(duration_digit_font_rem(ControlSize::Lg), Some(0.9375));
        assert_eq!(duration_digit_font_rem(ControlSize::Xl), Some(1.0));
    }

    #[test]
    fn duration_label_fonts_match_contract() {
        assert_eq!(duration_label_font_rem(ControlSize::Xs), 0.5);
        assert_eq!(duration_label_font_rem(ControlSize::Sm), 0.5625);
        assert_eq!(duration_label_font_rem(ControlSize::Md), 0.5625);
        assert_eq!(duration_label_font_rem(ControlSize::Xl), 0.5625);
    }

    #[test]
    fn duration_density_adjusts_match_contract() {
        assert_eq!(
            duration_gap_density_adjust_rem(ControlDensity::Comfortable),
            0.25
        );
        assert_eq!(
            duration_gap_density_adjust_rem(ControlDensity::Default),
            0.0
        );
        assert_eq!(
            duration_pad_x_density_adjust_rem(ControlDensity::Compact),
            -0.125
        );
        assert_eq!(
            duration_pad_x_density_adjust_rem(ControlDensity::Comfortable),
            0.125
        );
        assert_eq!(
            duration_pad_x_density_adjust_rem(ControlDensity::Default),
            0.0
        );
    }

    // ── rem_to_px ───────────────────────────────────────────────

    #[test]
    fn rem_to_px_uses_16px_base() {
        assert_eq!(rem_to_px(1.0), 16.0);
        assert_eq!(rem_to_px(0.5), 8.0);
        assert_eq!(rem_to_px(2.25), 36.0);
    }
}

