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
