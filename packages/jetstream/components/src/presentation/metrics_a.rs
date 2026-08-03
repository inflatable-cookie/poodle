//! Jetstream per-size metric helpers. Split out of `presentation/mod.rs`
//! (god-file decomposition); pure scalar functions.

use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Resolve a semantic size role against a base size to get the effective control size.
///
/// `Chrome` resolves one stop smaller (clamped at the minimum),
/// `Prominent` resolves one stop larger (clamped at the maximum),
/// `Control` is the identity mapping.
///
/// This exactly matches the Svelte `resolveSemanticControlSize` function.
pub fn resolve_semantic_size(size: ControlSize, role: SemanticControlSizeRole) -> ControlSize {
    poodle_specs::resolve_semantic_control_size(size, role)
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
/// `size_font_rem` ladder. Mirrors `poodle_gpui::presentation::date_picker_indicator_font_rem`.
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
// `poodle_gpui::presentation::switch_*` exactly so both Rust targets resolve
// identical switch geometry from a single source of truth.

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

/// ToggleGroup inter-item gap in rem for a given density.
///
/// Matches the Svelte `.poodle-toggle-group[data-density]` `--poodle-toggle-group-gap`
/// table (compact 0.1875 / default 0.25 / comfortable 0.375 rem). GPUI uses the
/// same values inline.
pub fn toggle_group_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.1875,
        ControlDensity::Default => 0.25,
        ControlDensity::Comfortable => 0.375,
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

// ── RelationPicker size / density scales ─────────────────────────
//
// Contract §8 + the Svelte `.poodle-relation-picker[data-size]`/`[data-density]`
// overrides. Mirrors `poodle_gpui::presentation` exactly.

/// Candidate-item vertical padding in rem (`--relation-picker-item-y`).
pub fn relation_picker_item_y_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.5,
    }
}
