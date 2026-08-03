//! Jetstream per-size metric helpers. Split out of `presentation/mod.rs`
//! (god-file decomposition); pure scalar functions.

use poodle_specs::{ControlDensity, ControlSize};

/// Candidate-item horizontal padding in rem (`--relation-picker-item-x`).
pub fn relation_picker_item_x_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        ControlSize::Sm => 0.5,
        ControlSize::Md => 0.5,
        ControlSize::Lg => 0.625,
        ControlSize::Xl => 0.75,
    }
}

/// Candidate-item inner gap in rem (`--relation-picker-item-gap`).
pub fn relation_picker_item_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        _ => 0.5,
    }
}

/// Candidate-copy title (`strong`) font size in rem (`--relation-picker-title-size`).
pub fn relation_picker_title_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.75,
        ControlSize::Md => 0.8125,
        ControlSize::Lg => 0.875,
        ControlSize::Xl => 0.9375,
    }
}

/// Candidate-copy description (`small`) font size in rem (`--relation-picker-desc-size`).
pub fn relation_picker_desc_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.5625,
        ControlSize::Sm => 0.625,
        ControlSize::Md => 0.6875,
        ControlSize::Lg => 0.75,
        ControlSize::Xl => 0.8125,
    }
}

/// Candidate-list inter-row gap in rem (`--relation-picker-list-gap`).
pub fn relation_picker_list_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.1875,
        ControlDensity::Default => 0.25,
        ControlDensity::Comfortable => 0.3125,
    }
}

// ── TokenInput size / density scales ─────────────────────────────
//
// Contract §8 + the Svelte `.poodle-token-input[data-size]`/`[data-density]`
// overrides. Mirrors `poodle_gpui::presentation` exactly.

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

// ── Breadcrumbs size / density scales ────────────────────────────
//
// Contract §8 size + density tables (mirrors the Svelte
// `.poodle-breadcrumbs[data-size]` / `[data-density]` overrides). The list/item
// gap is size-driven; density overrides it when not `default`. Font-size is a
// breadcrumbs-specific ladder where md == `typography.body.size` (0.875rem).
// Mirrors `poodle_gpui::presentation` exactly.

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

/// List font-size in rem per size. Contract §8: xs 0.6875, sm 0.78125, md
/// `typography.body.size` (0.875), lg 1, xl 1.125.
pub fn breadcrumbs_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.6875,
        ControlSize::Sm => 0.78125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 1.0,
        ControlSize::Xl => 1.125,
    }
}

// ── CodeInput size / density scales ──────────────────────────────
//
// Contract §7 size + density tables (mirrors the Svelte
// `.poodle-code-input[data-size]` / `[data-density]` overrides). Slots are
// square at every size; the gap is density-driven. Mirrors
// `poodle_gpui::presentation` exactly.

/// Square slot width/height in rem per size. Contract §7: xs 1.5, sm 1.75,
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

/// Slot digit font-size in rem per size. Contract §7: xs 0.8125, sm 0.875,
/// md 1, lg 1.125, xl 1.25.
pub fn code_input_slot_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    }
}

// ── EditableList size / density scales ───────────────────────────
//
// Contract §8 "Size Adjustments" / "Density Adjustments". Mirrors
// `poodle_gpui::presentation::editable_list_*` exactly so both Rust targets
// resolve identical row geometry from a single source of truth.

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

// ── DataTable size scale ─────────────────────────────────────────
//
// Contract §11 "Size Variants": the selection column width per size
// (md == `3.25rem`) and the fixed actions column width (`3.5rem`, all
// sizes). Mirrors `poodle_gpui::presentation::data_table_*`.

/// Selection (checkbox) column width in rem per size. Contract §11 size
/// table: xs 2.5, sm 2.75, md 3.25, lg 3.625, xl 4.
pub fn data_table_selection_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 2.5,
        ControlSize::Sm => 2.75,
        ControlSize::Md => 3.25,
        ControlSize::Lg => 3.625,
        ControlSize::Xl => 4.0,
    }
}

/// Row-actions column width in rem. Contract `.data-table__actions`
/// is a fixed `3.5rem` across all sizes.
pub fn data_table_actions_width_rem() -> f32 {
    3.5
}

// ── Drawer size scale ────────────────────────────────────────────
//
// Contract §8 "Size adjustments": the drawer header title font-size per
// size. md == `1rem`. Mirrors `poodle_gpui::presentation::drawer_title_font_rem`.

/// Drawer header title font-size in rem per size. Contract §8 size table:
/// xs 0.8125, sm 0.875, md 1, lg 1.0625, xl 1.125.
pub fn drawer_title_font_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.8125,
        ControlSize::Sm => 0.875,
        ControlSize::Md => 1.0,
        ControlSize::Lg => 1.0625,
        ControlSize::Xl => 1.125,
    }
}

// ── Table size / density scales ──────────────────────────────────
//
// Contract `table.md` §8 "Size adjustments" / "Density adjustments".
// Size scales the table font-size, header font-size, and vertical cell
// `padding-block` (md is the baseline from the cell/header values). Density
// scales horizontal cell `padding-inline` only (never height). Mirrors the
// Svelte `.poodle-table-shell[data-size]` / `[data-density]` overrides, and
// `poodle_gpui::presentation::table_*` exactly.
