//! JsTriStateSwitch — ternary exclude/default/include switch backed by TriStateSwitchSpec.
//!
//! Contract: `docs/contracts/components/tri-state-switch.md`
//! Reference: `packages/svelte/components/src/TriStateSwitch.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, BoxShadow, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, TriStateSwitchSpec, TriStateValue};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
};
use crate::theme_ext::{resolve_color, resolve_opacity};

/// Track inset in rem, derived from density.
///
/// Contract §8: compact `0.0625rem` / default `0.125rem` / comfortable `0.1875rem`.
fn track_inset_rem(density: poodle_specs::ControlDensity) -> f32 {
    match density {
        poodle_specs::ControlDensity::Compact => 0.0625,
        poodle_specs::ControlDensity::Default => 0.125,
        poodle_specs::ControlDensity::Comfortable => 0.1875,
    }
}

/// Per-size `--poodle-tri-state-min-content-width` (contract §8 size scale).
/// Distinct from the text-input `size_min_width_rem` ladder.
fn tri_state_min_content_width_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 2.5,
        ControlSize::Sm => 2.625,
        ControlSize::Md => 3.0,
        ControlSize::Lg => 3.375,
        ControlSize::Xl => 3.75,
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
///
/// JsEl note: the immediate-mode runtime has no abstractly-positioned sliding
/// capsule, so the "Selection" part is realized by painting the active
/// segment's own fill + border + drop shadow (contract §8 selection shadow).
/// Segment click + arrow-key selection lives in the preview event loop.
pub fn js_tri_state_switch(spec: &TriStateSwitchSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Semantic color tokens ──
    let text_secondary: Color = resolve_color(theme, spec.unselected_text_token()).into();
    let border_default: Color = resolve_color(theme, spec.border_token()).into();

    // Per-state colors (with optional hex overrides).
    let excluded_color = override_or(theme, spec.excluded_color_token(), &spec.excluded_color);
    let default_color = override_or(theme, spec.default_color_token(), &spec.default_color);
    let included_color = override_or(theme, spec.included_color_token(), &spec.included_color);

    // ── Track base: color-mix(canvas 70%, black) ──
    let canvas: Color = resolve_color(theme, spec.track_base_token()).into();
    let track_base = canvas.mix(Color::BLACK, 0.70);
    // Root background: color-mix(canvas 75%, black).
    let root_bg = canvas.mix(Color::BLACK, 0.75);

    // ── Sizing ──
    let height = rem_to_px(control_height_rem(effective_size));
    let x = rem_to_px(control_space_x_rem(spec.density));
    let inset = rem_to_px(track_inset_rem(spec.density));
    // Contract: segment min-width = min-content-width + x*2.
    let min_segment_width = rem_to_px(tri_state_min_content_width_rem(effective_size)) + x * 2.0;

    let border_width = rem_to_px(0.0625); // contract hairline

    // Contract §8: segment font-size / weight from typography-label tokens
    // (fixed, not size-scaled).
    let label_size = theme.resolve_space_px(spec.label_size_token());
    let label_weight = theme.resolve_space_px(spec.label_weight_token()) as u16;

    // ── Per-state selection fill + border (14/8/14 over track_base) ──
    let value = spec.value();
    let (selection_fill, selection_border) = match value {
        TriStateValue::Excluded => (
            excluded_color.mix(track_base, 0.14),
            // Contract: color-mix(excluded 58%, border-default).
            excluded_color.mix(border_default, 0.58),
        ),
        TriStateValue::Default => (default_color.mix(track_base, 0.08), border_default),
        TriStateValue::Included => (
            included_color.mix(track_base, 0.14),
            included_color.mix(border_default, 0.58),
        ),
    };

    // ── Build segments ──
    let states = [
        (
            TriStateValue::Excluded,
            spec.excluded_label(),
            excluded_color,
        ),
        (TriStateValue::Default, spec.default_label(), default_color),
        (
            TriStateValue::Included,
            spec.included_label(),
            included_color,
        ),
    ];

    let segment_height = height - inset * 2.0;

    // ── Root ──
    let mut root = ui_element::div()
        .h(height)
        .rounded(999.0) // pill sentinel (max radius; not token-derived)
        .bg(root_bg)
        .border(border_width)
        .border_color(border_default)
        .flex_row()
        .items_center()
        .p(inset);

    for &(state, label_text, state_color) in &states {
        let is_active = value == state;

        // Active uses per-state color; inactive uses text-secondary.
        let seg_text_color = if is_active { state_color } else { text_secondary };

        // Active segment paints the selection fill/border + shadow;
        // inactive is transparent over the shared track.
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

        let mut segment = ui_element::button(label_text)
            .min_h(segment_height)
            .min_w(min_segment_width)
            .pl(x)
            .pr(x)
            .rounded(999.0)
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

        // Selection box-shadow on the active segment (contract §8): an
        // `inset 0 0.0625rem 0 white@8%` top highlight plus a
        // `0 0.125rem 0.5rem black@18%` outset drop.
        if is_active {
            segment = segment.shadow_layers(vec![
                BoxShadow {
                    offset_x: 0.0,
                    offset_y: rem_to_px(0.0625),
                    blur: 0.0,
                    spread: 0.0,
                    color: glam::Vec4::new(1.0, 1.0, 1.0, 0.08),
                    inset: true,
                },
                BoxShadow {
                    offset_x: 0.0,
                    offset_y: rem_to_px(0.125),
                    blur: rem_to_px(0.5),
                    spread: 0.0,
                    color: glam::Vec4::new(0.0, 0.0, 0.0, 0.18),
                    inset: false,
                },
            ]);
        }

        root = root.child(segment);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        root = root.opacity(opacity).disabled(true);
    }

    root
}

/// Resolve a per-state color, preferring an instance hex override.
fn override_or(theme: &JetstreamThemeProvider, token: &str, override_hex: &Option<String>) -> Color {
    if let Some(hex) = override_hex {
        if let Some(rgb) = crate::theme_ext::hex_to_rgb255(hex) {
            return Color::from_u8(rgb.r, rgb.g, rgb.b, 255);
        }
    }
    resolve_color(theme, token).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_probe::probe;
    use poodle_specs::CheckState;

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
    fn three_positions_render_in_fixed_order() {
        let th = theme();
        let spec = TriStateSwitchSpec::new();
        let tree = probe(&js_tri_state_switch(&spec, &th), 400.0, 80.0);
        let texts = tree.texts();
        // Fixed order: excluded, default, included.
        let pos_ex = texts.iter().position(|t| *t == "Exclude");
        let pos_de = texts.iter().position(|t| *t == "Default");
        let pos_in = texts.iter().position(|t| *t == "Include");
        assert!(pos_ex.is_some() && pos_de.is_some() && pos_in.is_some());
        assert!(pos_ex < pos_de && pos_de < pos_in, "segment order wrong: {texts:?}");
    }

    #[test]
    fn active_segment_shifts_per_state() {
        let th = theme();
        // Excluded selected → first segment painted (non-transparent bg).
        let excluded = js_tri_state_switch(
            &TriStateSwitchSpec::new().with_state(CheckState::Unchecked),
            &th,
        );
        let included = js_tri_state_switch(
            &TriStateSwitchSpec::new().with_state(CheckState::Checked),
            &th,
        );
        // First segment fill differs between excluded-selected and included-selected.
        assert_ne!(
            excluded.children[0].style.background,
            included.children[0].style.background
        );
        // Last segment fill differs symmetrically.
        assert_ne!(
            excluded.children[2].style.background,
            included.children[2].style.background
        );
    }

    #[test]
    fn active_segment_has_visible_fill() {
        let th = theme();
        let spec = TriStateSwitchSpec::new().with_state(CheckState::Mixed);
        let el = js_tri_state_switch(&spec, &th);
        // Middle (default) segment is the active one → has a non-transparent bg.
        let active_bg = el.children[1].style.background.expect("active bg set");
        assert!(active_bg.a > 0.0, "active segment fill should be opaque");
        // Inactive segments are transparent.
        let inactive = el.children[0].style.background;
        assert!(inactive.is_none() || inactive.unwrap().a == 0.0);
    }
}
