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
// square at every size; the gap is density-driven.

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

// ── RelationPicker size / density scales ─────────────────────────
//
// Contract §8 "Size adjustments" / "Density adjustments" + the Svelte
// `.poodle-relation-picker[data-size]` / `[data-density]` custom-property
// overrides. Candidate-item geometry (`--relation-picker-item-y/x/gap`),
// copy font sizes, and list gap are all size/density-driven; these mirror the
// authoritative Svelte values so the component resolves geometry from one
// source of truth instead of flat rem literals.

/// Candidate-item vertical padding in rem (`--relation-picker-item-y`).
/// Svelte: xs 0.25, sm/md 0.375, lg/xl 0.5.
pub fn relation_picker_item_y_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.25,
        ControlSize::Sm => 0.375,
        ControlSize::Md => 0.375,
        ControlSize::Lg => 0.5,
        ControlSize::Xl => 0.5,
    }
}

/// Candidate-item horizontal padding in rem (`--relation-picker-item-x`).
/// Svelte: xs 0.375, sm/md 0.5, lg 0.625, xl 0.75.
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
/// Svelte: xs 0.375, sm/md/lg/xl 0.5.
pub fn relation_picker_item_gap_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.375,
        _ => 0.5,
    }
}

/// Candidate-copy title (`strong`) font size in rem (`--relation-picker-title-size`).
/// Svelte: xs 0.6875, sm 0.75, md 0.8125, lg 0.875, xl 0.9375.
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
/// Svelte: xs 0.5625, sm 0.625, md 0.6875, lg 0.75, xl 0.8125.
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
/// Svelte density: compact 0.1875, default 0.25, comfortable 0.3125.
pub fn relation_picker_list_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.1875,
        ControlDensity::Default => 0.25,
        ControlDensity::Comfortable => 0.3125,
    }
}

// ── TokenInput size / density scales ─────────────────────────────
//
// Contract §8 + the Svelte `.poodle-token-input[data-size]` / `[data-density]`
// custom-property overrides. Padding-block/inline derive from `space.control.y/x`
// with per-size offsets; font-size and the wrap-row gap are size/density-driven.

/// Token-row padding-block offset in rem added to `space.control.y`.
/// Svelte: xs -0.125, sm -0.0625, md 0, lg +0.0625, xl +0.125.
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
/// Svelte: xs -0.25, sm -0.125, md 0, lg +0.125, xl +0.1875.
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
/// Svelte: xs 0.75, sm 0.8125, md body-size (returns None → use token), lg 0.9375, xl 1.
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
/// Svelte density: compact 0.25, default 0.375, comfortable 0.5.
pub fn token_input_gap_rem(density: ControlDensity) -> f32 {
    match density {
        ControlDensity::Compact => 0.25,
        ControlDensity::Default => 0.375,
        ControlDensity::Comfortable => 0.5,
    }
}

// ── DataTable size scale ─────────────────────────────────────────
//
// Contract §11 "Size Variants": the selection column width per size
// (md == `3.25rem`) and the fixed actions column width (`3.5rem`, all
// sizes). Mirrors `poodle_jetstream::presentation::data_table_*`.

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
// size. md == `1rem`. Mirrors `poodle_jetstream::presentation::drawer_title_font_rem`.

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
// Svelte `.poodle-table-shell[data-size]` / `[data-density]` overrides.

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

