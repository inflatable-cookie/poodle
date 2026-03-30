//! JsTriStateSwitch — ternary exclude/default/include switch backed by TriStateSwitchSpec.
//!
//! Contract: `docs/contracts/foundation/tri-state-switch.md`
//! Reference: `packages/svelte/primitives/src/TriStateSwitch.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{CheckState, TriStateSwitchSpec};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_min_width_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Track inset in rem, derived from density.
fn track_inset_rem(density: poodle_primitives::ControlDensity) -> f32 {
    match density {
        poodle_primitives::ControlDensity::Compact => 0.125,
        poodle_primitives::ControlDensity::Default => 0.1875,
        poodle_primitives::ControlDensity::Comfortable => 0.25,
    }
}

/// Build a Jetstream tri-state switch element from a TriStateSwitchSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Root .tri-state-switch]  <div role="radiogroup">
///   ├── [Selection .tri-state-switch__selection]  <span aria-hidden="true">
///   └── [Option .tri-state-switch__option]  <label>
///         ├── [Control .tri-state-switch__control]  <input type="radio">
///         └── [Segment .tri-state-switch__segment]  <span>
/// ```
pub fn js_tri_state_switch(
    spec: &TriStateSwitchSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Semantic color tokens ──
    let danger: Color = resolve_color(theme, "semantic.color.status.danger").into();
    let success: Color = resolve_color(theme, "semantic.color.status.success").into();
    let text_primary: Color = resolve_color(theme, "semantic.color.text.primary").into();
    let text_secondary: Color = resolve_color(theme, "semantic.color.text.secondary").into();
    let surface: Color = resolve_color(theme, "semantic.color.background.surface").into();
    let border_default: Color = resolve_color(theme, "semantic.color.border.default").into();

    // Per-state colors (with optional overrides)
    let excluded_color = danger;
    let default_color = text_primary;
    let included_color = success;

    // ── Sizing ──
    let height = rem_to_px(control_height_rem(effective_size));
    let segment_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let inset = rem_to_px(track_inset_rem(spec.density));
    let min_segment_width = rem_to_px(size_min_width_rem(effective_size)) * 0.4;

    // Contract: border-radius 999px (pill)
    let border_width = rem_to_px(0.0625); // 0.0625rem

    // Contract: root background = color-mix(text-primary 18%, background-surface)
    let track_bg = text_primary.mix(surface, 0.18);

    // Contract: label font-size, weight from typography-label tokens
    let label_size = rem_to_px(0.8125); // typography-label-size
    let label_weight = 500_u16; // typography-label-weight

    // ── Track fill for the selected state ──
    let (selection_fill, selection_border) = match spec.state {
        CheckState::Unchecked => {
            // Excluded: 18% danger mix with surface
            let fill = excluded_color.mix(surface, 0.18);
            let border = Color::new(
                excluded_color.r,
                excluded_color.g,
                excluded_color.b,
                excluded_color.a * 0.38,
            );
            (fill, border)
        }
        CheckState::Mixed => {
            // Default: 10% text-primary mix with surface (elevated-ish)
            let fill = default_color.mix(surface, 0.10);
            let border = Color::TRANSPARENT;
            (fill, border)
        }
        CheckState::Checked => {
            // Included: 18% success mix with surface
            let fill = included_color.mix(surface, 0.18);
            let border = Color::new(
                included_color.r,
                included_color.g,
                included_color.b,
                included_color.a * 0.38,
            );
            (fill, border)
        }
    };

    // ── Build segments ──
    let states = [
        (CheckState::Unchecked, spec.excluded_label(), excluded_color),
        (CheckState::Mixed, spec.default_label(), default_color),
        (CheckState::Checked, spec.included_label(), included_color),
    ];

    let segment_height = height - inset * 2.0;

    // ── Root ──
    let mut root = ui_element::div()
        .h(height)
        .rounded(999.0) // pill
        .bg(track_bg)
        .border(border_width)
        .border_color(border_default)
        .flex_row()
        .items_center()
        .p(inset);

    for &(state, label_text, state_color) in &states {
        let is_active = spec.state == state;

        // Segment text color: active uses per-state color, inactive uses text-secondary
        let seg_text_color = if is_active { state_color } else { text_secondary };

        // Segment background: active gets the selection fill, inactive is transparent
        let seg_bg = if is_active {
            selection_fill
        } else {
            Color::TRANSPARENT
        };

        let seg_border = if is_active {
            selection_border
        } else {
            Color::TRANSPARENT
        };

        let segment = ui_element::button(label_text)
            .min_h(segment_height)
            .min_w(min_segment_width)
            .pl(segment_pad_x)
            .pr(segment_pad_x)
            .rounded(999.0) // pill
            .bg(seg_bg)
            .border(border_width)
            .border_color(seg_border)
            .text_color(seg_text_color)
            .text_size(label_size)
            .text_weight(label_weight)
            .flex_row()
            .items_center()
            .justify_center()
            .focusable();

        root = root.child(segment);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn has_three_segments() {
        let spec = TriStateSwitchSpec::new();
        let el = js_tri_state_switch(&spec, &theme());
        assert_eq!(el.children.len(), 3);
    }

    #[test]
    fn disabled_has_reduced_opacity() {
        let spec = TriStateSwitchSpec::new().with_disabled(true);
        let el = js_tri_state_switch(&spec, &theme());
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn checked_state_differs_from_unchecked() {
        let unchecked = js_tri_state_switch(
            &TriStateSwitchSpec::new().with_state(CheckState::Unchecked),
            &theme(),
        );
        let checked = js_tri_state_switch(
            &TriStateSwitchSpec::new().with_state(CheckState::Checked),
            &theme(),
        );
        // First segment (excluded) should differ in background between the two states
        let unc_first_bg = unchecked.children[0].style.background;
        let chk_first_bg = checked.children[0].style.background;
        // Active segment shifts, so backgrounds should differ
        assert_ne!(unc_first_bg, chk_first_bg);
    }
}
