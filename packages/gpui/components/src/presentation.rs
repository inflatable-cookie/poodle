//! Size-role and density resolution infrastructure for GPUI components.
//!
//! These helpers mirror the Svelte reference implementation in
//! `packages/svelte/components/src/presentation.ts` and ensure all GPUI
//! components resolve size, density, and layout offsets identically.

use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Resolve a semantic size role against a base size to get the effective control size.
///
/// `Chrome` resolves one stop smaller (clamped at the minimum),
/// `Prominent` resolves one stop larger (clamped at the maximum),
/// `Control` is the identity mapping.
///
/// This exactly matches the Svelte `resolveSemanticControlSize` function.
pub fn resolve_semantic_size(size: ControlSize, role: SemanticControlSizeRole) -> ControlSize {
    match (size, role) {
        (_, SemanticControlSizeRole::Control) => size,

        // Chrome: one stop smaller, clamped at the bottom of the scale
        (ControlSize::Xs, SemanticControlSizeRole::Chrome) => ControlSize::Xs,
        (ControlSize::Sm, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
        (ControlSize::Md, SemanticControlSizeRole::Chrome) => ControlSize::Sm,
        (ControlSize::Lg, SemanticControlSizeRole::Chrome) => ControlSize::Md,
        (ControlSize::Xl, SemanticControlSizeRole::Chrome) => ControlSize::Lg,

        // Prominent: one stop larger, clamped at the top of the scale
        (ControlSize::Xs, SemanticControlSizeRole::Prominent) => ControlSize::Sm,
        (ControlSize::Sm, SemanticControlSizeRole::Prominent) => ControlSize::Md,
        (ControlSize::Md, SemanticControlSizeRole::Prominent) => ControlSize::Lg,
        (ControlSize::Lg, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
        (ControlSize::Xl, SemanticControlSizeRole::Prominent) => ControlSize::Xl,
    }
}

/// Get the absolute control height in rem for a given size.
///
/// Matches the Svelte `controlHeightRem` function.
pub fn control_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    }
}

/// Get the control height offset in rem from the Md baseline token.
///
/// Matches the Svelte Button CSS `calc(var(--poodle-size-control-height) +/- offset)`.
pub fn size_height_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.5,
        ControlSize::Sm => -0.375,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.5,
    }
}

/// Get the control padding-x offset in rem from the baseline token.
///
/// Matches the Svelte Button CSS `calc(var(--poodle-space-control-x) +/- offset)`.
pub fn size_padding_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.125,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Get the font size in rem for a given control size.
///
/// Matches the Svelte Button CSS per-size font-size declarations.
/// Md uses `typography-label-size` (0.8125rem / 13px at 16px base).
pub fn size_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Get the min-width in rem for a given control size.
///
/// Matches the Svelte Button CSS per-size min-width declarations.
pub fn size_min_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 3.75,
        ControlSize::Sm => 4.25,
        ControlSize::Md => 5.0,
        ControlSize::Lg => 5.75,
        ControlSize::Xl => 6.5,
    }
}

/// Resolve a supporting visual size (icons inside controls).
///
/// Supporting visuals render one stop smaller than the control, clamped
/// at xs/sm (the smallest sizes keep their own supporting visual size).
///
/// Matches the Svelte `resolveSupportingVisualSize` function.
pub fn resolve_supporting_visual_size(size: ControlSize) -> ControlSize {
    match size {
        ControlSize::Xl => ControlSize::Lg,
        ControlSize::Lg => ControlSize::Md,
        ControlSize::Md => ControlSize::Sm,
        _ => size,
    }
}

/// Get the control horizontal space in rem for a given density.
///
/// Matches the Svelte `controlSpaceXRem` function.
pub fn control_space_x_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 1.0,
    }
}

/// Get the panel horizontal space in rem for a given density.
///
/// Matches the Svelte `panelSpaceXRem` function.
pub fn panel_space_x_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

/// Get the panel vertical space in rem for a given density.
///
/// Matches the Svelte `panelSpaceYRem` function.
pub fn panel_space_y_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 1.0,
    }
}

/// Callout outer gap in rem for a given size.
///
/// Matches Svelte `.callout { gap: space.inline.md }` base, overridden per-size:
/// xs→0.375 sm→0.5 md→0.75 lg→0.875 xl→1.0.
pub fn callout_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        ControlSize::Sm => 0.5,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 1.0,
    }
}

/// Callout icon container size in rem for a given size.
///
/// Matches Svelte per-size `.callout__icon { width/height }` values.
pub fn callout_icon_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.875,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.375,
        ControlSize::Lg => 1.75,
        ControlSize::Xl => 2.0,
    }
}

/// Callout dismiss button size in rem for a given size.
///
/// Matches Svelte per-size `.callout__dismiss { width/height }` values.
pub fn callout_dismiss_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.5,
        ControlSize::Md => 1.75,
        ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.25,
    }
}

/// Calendar day-cell / grid-column size (`--calendar-cell-size`) in rem.
///
/// Matches the Svelte Calendar per-size `--calendar-cell-size` values
/// (md `2.25rem`). This is a calendar-specific scale distinct from
/// `control_height_rem`, and also drives the day button `min-height`.
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
/// (md `2rem`).
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
pub fn calendar_day_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    }
}

// ── EditableList size / density scales ───────────────────────────
//
// Contract §8 "Size Adjustments" / "Density Adjustments". These mirror the
// Svelte `.poodle-editable-list[data-size]` / `[data-density]` custom-property
// overrides exactly, giving the GPUI composite a single source of truth for
// row geometry instead of inline literals.

/// Handle (grip) square size in rem. Contract: xs 0.875, sm/md 1, lg 1.125, xl 1.25.
pub fn editable_list_handle_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.875,
        ControlSize::Sm => 1.0,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Item horizontal padding in rem. Contract: xs 0.5, sm/md 0.625, lg 0.75, xl 0.875.
pub fn editable_list_item_x_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5,
        ControlSize::Sm => 0.625,
        ControlSize::Md => 0.625,
        ControlSize::Lg => 0.75,
        ControlSize::Xl => 0.875,
    }
}

/// Item vertical padding in rem. Contract: xs 0.375, sm 0.4375, md 0.5, lg 0.5625, xl 0.625.
pub fn editable_list_item_y_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        ControlSize::Sm => 0.4375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.5625,
        ControlSize::Xl => 0.625,
    }
}

/// Item / content font size in rem. Contract: xs 0.6875, sm 0.75, md 0.8125, lg 0.875, xl 0.9375.
pub fn editable_list_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// List inter-row gap in rem. Contract density: compact 0.0625, default 0.125, comfortable 0.1875.
pub fn editable_list_list_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.0625,
        ControlDensity::Default => 0.125,
        ControlDensity::Comfortable => 0.1875,
    }
}

/// Per-item inner gap (handle↔content↔remove) in rem. Contract density:
/// compact 0.375, default 0.5, comfortable 0.625.
pub fn editable_list_item_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => 0.5,
        ControlDensity::Comfortable => 0.625,
    }
}

// ── DurationInput size scales ────────────────────────────────────
//
// Contract §8 "Size adjustments". These mirror the Svelte
// `.poodle-duration-input[data-size]` custom-property overrides exactly so
// the GPUI component drives field geometry from a single source of truth
// instead of inline literals. Base (md path) values come from the contract
// base column / size table.

/// Field width in rem. Contract: xs 1.5, sm 1.625, md 1.875, lg 2, xl 2.25.
/// (Base when no size is `1.75`, but the component always resolves a size; md
/// in the size table is `1.875`.)
pub fn duration_field_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.625,
        ControlSize::Md => 1.875,
        ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.25,
    }
}

/// Root padding-block in rem. Contract: xs 0.125, sm 0.1875, md 0.25, lg 0.3125, xl 0.375.
pub fn duration_pad_y_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.125,
        ControlSize::Sm => 0.1875,
        ControlSize::Md => 0.25,
        ControlSize::Lg => 0.3125,
        ControlSize::Xl => 0.375,
    }
}

/// Root padding-inline size adjust in rem (added to `space.control.x`).
/// Contract: xs -0.125, sm -0.0625, md 0, lg +0.125, xl +0.1875.
pub fn duration_pad_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Field / separator-glyph (digit) font-size in rem. Contract size table:
/// xs 0.75, sm base (body-size), md body-size, lg 0.9375, xl 1.
/// Body-size resolves from the token; this returns the per-size override or
/// `None` to signal "use the resolved body-size token".
pub fn duration_digit_font_rem(size: ControlSize) -> Option<f32> {
    match size {
        ControlSize::Xs => Some(0.75),
        ControlSize::Sm => None, // base = typography.body.size
        ControlSize::Md => None, // typography.body.size
        ControlSize::Lg => Some(0.9375),
        ControlSize::Xl => Some(1.0),
    }
}

/// Label font-size in rem. Contract size table: xs 0.5, others base 0.5625rem.
pub fn duration_label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5,
        _ => 0.5625,
    }
}

/// Inter-segment gap density adjust in rem. Contract: comfortable adds 0.25rem;
/// compact/default add nothing.
pub fn duration_gap_density_adjust_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Comfortable => 0.25,
        _ => 0.0,
    }
}

/// Root padding-inline density adjust in rem. Contract: compact -0.125,
/// comfortable +0.125, default 0.
pub fn duration_pad_x_density_adjust_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Comfortable => 0.125,
        ControlDensity::Default => 0.0,
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
