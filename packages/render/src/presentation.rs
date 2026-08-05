//! Per-size and per-density metric ladders, shared by every component.
//!
//! Copied from `packages/jetstream/components/src/presentation/` — the values
//! are the Svelte CSS tables, transcribed. The copies there and in
//! `packages/gpui` die as the migration deletes those tiers; this becomes the
//! only Rust transcription. Until then, the parity fixtures keep all three
//! honest: a divergent value shows up as a node/output mismatch, not a guess.

use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

/// Convert rem to pixels at the standard 16px base.
pub fn rem_to_px(rem: f32) -> f32 {
    rem * 16.0
}

/// Resolve a semantic size role against a base size.
/// `Chrome` one stop smaller, `Prominent` one larger, `Control` identity.
pub fn resolve_semantic_size(size: ControlSize, role: SemanticControlSizeRole) -> ControlSize {
    poodle_specs::resolve_semantic_control_size(size, role)
}

/// Control height in rem for a size. Matches the Svelte `controlHeightRem`.
pub fn control_height_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    }
}

/// Font size in rem for a control size. Matches the Svelte per-size tables.
pub fn size_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Supporting visuals render one stop smaller, clamped at the small end.
pub fn resolve_supporting_visual_size(size: ControlSize) -> ControlSize {
    match size {
        ControlSize::Xl => ControlSize::Lg,
        ControlSize::Lg => ControlSize::Md,
        ControlSize::Md => ControlSize::Sm,
        _ => size,
    }
}

/// Horizontal control padding in rem for a density.
pub fn control_space_x_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 1.0,
    }
}

/// Button min-width in rem per size.
pub fn size_min_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 3.75,
        ControlSize::Sm => 4.25,
        ControlSize::Md => 5.0,
        ControlSize::Lg => 5.75,
        ControlSize::Xl => 6.5,
    }
}

/// Button padding-x offset in rem from the density baseline.
pub fn size_padding_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.125,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Switch track width in rem per size (contract §8 flat literals).
pub fn switch_track_w_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.75,
        ControlSize::Sm => 2.0,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.0,
    }
}

/// Switch track height in rem per size.
pub fn switch_track_h_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.0,
        ControlSize::Sm => 1.125,
        ControlSize::Md => 1.375,
        ControlSize::Lg => 1.625,
        ControlSize::Xl => 1.75,
    }
}

/// Switch thumb diameter in rem per size.
pub fn switch_thumb_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.125,
        ControlSize::Lg => 1.375,
        ControlSize::Xl => 1.5,
    }
}

/// Switch thumb travel in rem per size.
pub fn switch_travel_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Switch label font size in rem per size.
pub fn switch_label_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.875,
    }
}

/// IconButton per-size square delta in rem from the md control height.
pub fn icon_button_size_delta_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.25,
        ControlSize::Sm => -0.375,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.5,
    }
}

/// Per-size icon token path.
pub fn icon_token(size: ControlSize) -> &'static str {
    match size {
        ControlSize::Xs => "size.icon.xs",
        ControlSize::Sm => "size.icon.sm",
        ControlSize::Md => "size.icon.md",
        ControlSize::Lg => "size.icon.lg",
        ControlSize::Xl => "size.icon.xl",
    }
}

/// Panel horizontal padding in rem for a density (callout, panels).
pub fn panel_space_x_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.75,
        ControlDensity::Default => 1.0,
        ControlDensity::Comfortable => 1.25,
    }
}

/// Panel vertical padding in rem for a density.
pub fn panel_space_y_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 1.0,
    }
}

/// Breadcrumbs gap in rem per size.
pub fn breadcrumbs_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.625,
        ControlSize::Xl => 0.75,
    }
}

/// Breadcrumbs density gap override in rem.
pub fn breadcrumbs_density_gap_rem(density: ControlDensity) -> Option<f32> {
    match density {
        ControlDensity::Compact => Some(0.25),
        ControlDensity::Default => None,
        ControlDensity::Comfortable => Some(0.75),
    }
}

/// Breadcrumbs font in rem per size (md == typography.body.size).
pub fn breadcrumbs_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.78125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 1.0,
        ControlSize::Xl => 1.125,
    }
}

/// Toolbar block padding in rem per size.
pub fn toolbar_pad_block_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.125,
        ControlSize::Sm => 0.1875,
        ControlSize::Md => 0.25,
        ControlSize::Lg => 0.3125,
        ControlSize::Xl => 0.375,
    }
}

/// Toolbar inline padding in rem per size.
pub fn toolbar_pad_inline_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.3125,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.625,
    }
}

/// Toolbar gap in rem per size.
pub fn toolbar_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.3125,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.625,
    }
}

/// Toolbar density inline-padding override in rem.
pub fn toolbar_density_pad_inline_rem(density: ControlDensity) -> Option<f32> {
    match density {
        ControlDensity::Compact => Some(0.25),
        ControlDensity::Default => None,
        ControlDensity::Comfortable => Some(0.5),
    }
}

/// Toolbar density gap override in rem.
pub fn toolbar_density_gap_rem(density: ControlDensity) -> Option<f32> {
    match density {
        ControlDensity::Compact => Some(0.25),
        ControlDensity::Default => None,
        ControlDensity::Comfortable => Some(0.5),
    }
}

/// Drawer header title font in rem per size.
pub fn drawer_title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    }
}

/// EditableList drag-handle square in rem. Contract: xs 0.875, sm/md 1.0, lg 1.125, xl 1.25.
pub fn editable_list_handle_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.875,
        ControlSize::Sm | ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// EditableList item horizontal padding in rem. Contract: xs 0.5, sm/md 0.625, lg 0.75, xl 0.875.
pub fn editable_list_item_x_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5,
        ControlSize::Sm | ControlSize::Md => 0.625,
        ControlSize::Lg => 0.75,
        ControlSize::Xl => 0.875,
    }
}

/// EditableList item vertical padding in rem. Contract: xs 0.375, sm 0.4375, md 0.5, lg 0.5625, xl 0.625.
pub fn editable_list_item_y_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        ControlSize::Sm => 0.4375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.5625,
        ControlSize::Xl => 0.625,
    }
}

/// EditableList item / content font size in rem. Contract: xs 0.6875, sm 0.75, md 0.8125, lg 0.875, xl 0.9375.
pub fn editable_list_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// EditableList inter-row gap in rem. Contract density: compact 0.0625, default 0.125, comfortable 0.1875.
pub fn editable_list_list_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.0625,
        ControlDensity::Default => 0.125,
        ControlDensity::Comfortable => 0.1875,
    }
}

/// EditableList per-item inner gap (handle↔content↔remove) in rem. Contract
/// density: compact 0.375, default 0.5, comfortable 0.625.
pub fn editable_list_item_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => 0.5,
        ControlDensity::Comfortable => 0.625,
    }
}

/// Calendar day-cell square in rem. Contract §8: xs 1.75, sm 2.0, md 2.25, lg 2.5, xl 2.75.
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
pub fn calendar_day_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs | ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    }
}

/// DatePicker disclosure-chevron font-size in rem per size.
pub fn date_picker_indicator_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.625,
        ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    }
}

/// Control min-height offset in rem from the md baseline (contract §8).
pub fn size_height_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.5,
        ControlSize::Sm => -0.375,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.375,
        ControlSize::Xl => 0.5,
    }
}

/// ToggleGroup inter-item gap in rem per density (contract §8 Root).
pub fn toggle_group_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.1875,
        ControlDensity::Default => 0.25,
        ControlDensity::Comfortable => 0.375,
    }
}

/// Token-row padding-block offset in rem added to `space.control.y`.
pub fn token_input_pad_y_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.0625,
        ControlSize::Xl => 0.125,
    }
}

/// Token-row padding-inline offset in rem added to `space.control.x`.
pub fn token_input_pad_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.25,
        ControlSize::Sm => -0.125,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Token-row / draft-input font size in rem (`--token-input-font-size`).
/// `None` → use the resolved `typography.body.size` token (md base).
pub fn token_input_font_rem(size: ControlSize) -> Option<f32> {
    match size {
        ControlSize::Xs => Some(0.75),
        ControlSize::Sm => Some(0.8125),
        ControlSize::Md => None,
        ControlSize::Lg => Some(0.9375),
        ControlSize::Xl => Some(1.0),
    }
}

/// Token-row wrap gap in rem (`--token-input-gap`).
pub fn token_input_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    }
}

/// SplitButton toggle-half width in rem per size (contract §8).
pub fn split_button_toggle_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.75,
        ControlSize::Sm => 1.875,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.25,
        ControlSize::Xl => 2.5,
    }
}

/// SplitButton chevron icon size in rem per size (contract §8).
pub fn split_button_chevron_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.625,
        ControlSize::Sm => 0.6875,
        ControlSize::Md => 0.75,
        ControlSize::Lg => 0.8125,
        ControlSize::Xl => 0.875,
    }
}

/// CodeInput slot square size in rem per size. Contract §7: xs 1.5, sm 1.75,
/// md 2.25, lg 2.75, xl 3.25.
pub fn code_input_slot_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    }
}

/// CodeInput slot digit font-size in rem per size. Contract §7: xs 0.8125,
/// sm 0.875, md 1, lg 1.125, xl 1.25.
pub fn code_input_slot_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

/// Table body cell font-size in rem per size (contract §8).
pub fn table_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Table header font-size in rem per size (contract §8).
pub fn table_header_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5625,
        ControlSize::Sm => 0.625,
        ControlSize::Md => 0.6875,
        ControlSize::Lg => 0.75,
        ControlSize::Xl => 0.8125,
    }
}

/// Table cell vertical padding in rem per size (contract §8).
pub fn table_cell_pad_block_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.3125,
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.625,
        ControlSize::Xl => 0.75,
    }
}

/// Table cell horizontal padding in rem per density (contract §8).
pub fn table_cell_pad_inline_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 1.125,
    }
}
