//! Per-size presentation metric helpers. Split out of
//! `presentation/mod.rs` (god-file decomposition); pure scalar functions.

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

/// DatePicker indicator (disclosure chevron) font-size in rem per size.
///
/// Contract `date-picker.md` §8 size table "indicator `font-size`" column:
/// xs 0.625, sm 0.6875, md 0.75, lg 0.8125, xl 0.875. Distinct from the trigger
/// `size_font_rem` ladder. Mirrors the Svelte
/// `.poodle-date-picker[data-size] .poodle-date-picker__indicator` overrides.
pub fn date_picker_indicator_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.625,
        ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    }
}

// ── Switch size scale ─────────────────────────────────────────────
//
// Contract §8 "Size adjustments" — flat rem literals per size, mirroring the
// Svelte `.poodle-switch[data-size]` overrides (track w/h, thumb diameter,
// thumb translateX travel). Track padding is `0.125rem` at every size. Mirrors
// the Jetstream `presentation::switch_*` table exactly so both Rust targets
// resolve identical switch geometry from a single source of truth.

/// Switch track width in rem per size. Contract §8: xs 1.75, sm 2, md 2.25,
/// lg 2.75, xl 3.
pub fn switch_track_w_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.75,
        ControlSize::Sm => 2.0,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.0,
    }
}

/// Switch track height in rem per size. Contract §8: xs 1, sm 1.125, md 1.375,
/// lg 1.625, xl 1.75.
pub fn switch_track_h_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.0,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.375,
        ControlSize::Lg => 1.625,
        ControlSize::Xl => 1.75,
    }
}

/// Switch thumb diameter in rem per size. Contract §8: xs 0.75, sm 0.875,
/// md 1.125, lg 1.375, xl 1.5.
pub fn switch_thumb_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.125,
        ControlSize::Lg => 1.375,
        ControlSize::Xl => 1.5,
    }
}

/// Switch thumb travel (translateX) in rem per size. Contract §8: xs 0.75,
/// sm 0.875, md 0.875, lg 1.125, xl 1.25.
pub fn switch_travel_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Switch label font-size in rem per size. Contract §8 "Label size variants":
/// xs 0.75, sm 0.75, md 0.8125, lg 0.875, xl 0.875. Distinct from the shared
/// `size_font_rem` ladder (which differs at xs and xl).
pub fn switch_label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.875,
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

/// SplitButton toggle-half base width in rem for a given size.
///
/// Matches the Svelte `--poodle-split-button-toggle-width-base` per-size table
/// (`SplitButton.svelte`). The `md` row (`2rem`) is the unscoped base.
pub fn split_button_toggle_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.75,
        ControlSize::Sm => 1.875,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.25,
        ControlSize::Xl => 2.5,
    }
}

/// SplitButton chevron icon size in rem for a given size.
///
/// Matches the Svelte `--poodle-split-button-chevron-size` per-size table.
pub fn split_button_chevron_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.625,
        ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
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

// ── Breadcrumbs size / density scales ────────────────────────────
//
// Contract §8 size + density tables (mirrors the Svelte
// `.poodle-breadcrumbs[data-size]` / `[data-density]` overrides). The list/item
// gap is size-driven; density overrides it when not `default`. Font-size is a
// breadcrumbs-specific ladder where md == `typography.body.size` (0.875rem).

/// List/item gap in rem per size. Contract §8: xs 0.25, sm 0.375, md
/// `space.inline.sm` (0.5), lg 0.625, xl 0.75.
pub fn breadcrumbs_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.625,
        ControlSize::Xl => 0.75,
    }
}

/// List/item gap override in rem per density, or `None` for `default` (use the
/// size gap). Contract §8: compact 0.25, comfortable 0.75.
pub fn breadcrumbs_density_gap_rem(density: ControlDensity) -> Option<f32> {
    match density {
        ControlDensity::Compact => Some(0.25),
        ControlDensity::Default => None,
        ControlDensity::Comfortable => Some(0.75),
    }
}

