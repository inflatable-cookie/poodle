//! JsTimeField — time input backed by TimeFieldSpec.
//!
//! Contract: `docs/contracts/components/time-field.md`
//! Reference: `packages/svelte/primitives/src/TimeField.svelte`
//!
//! ALL dimensions resolve from tokens. ZERO hardcoded pixel values.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::{ControlSize, TimeFieldSpec};

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_height_offset_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

/// Font size in rem per size override (contract section 8).
fn time_font_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm | ControlSize::Md => 0.8125, // body-size baseline
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    }
}

/// Padding-x offset in rem per size (contract section 8).
fn time_padding_x_offset_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => -0.125,
        ControlSize::Sm => -0.0625,
        ControlSize::Md => 0.0,
        ControlSize::Lg => 0.125,
        ControlSize::Xl => 0.1875,
    }
}

/// Build a Jetstream time field element from a TimeFieldSpec.
///
/// Anatomy (from contract):
/// ```text
/// [Input .time-field]  <input type="time">
/// ```
///
/// In Jetstream there is no native input[type="time"], so the component renders
/// a styled text display of the current time value.
pub fn js_time_field(spec: &TimeFieldSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);

    // ── Token resolution ──
    let fill: Color = resolve_color(theme, spec.fill_token()).into();
    let border_color: Color = resolve_color(theme, spec.border_token()).into();
    let text_color: Color = resolve_color(theme, spec.text_color_token()).into();
    let placeholder_color: Color = resolve_color(theme, spec.placeholder_color_token()).into();
    let radius = resolve_radius(theme, spec.radius_token());

    // ── Sizing (contract section 8) ──
    let min_height = rem_to_px(control_height_rem(effective_size))
        + rem_to_px(size_height_offset_rem(effective_size));
    let base_pad_x = rem_to_px(control_space_x_rem(spec.density));
    let pad_x = base_pad_x + rem_to_px(time_padding_x_offset_rem(effective_size));
    let font_size = rem_to_px(time_font_size_rem(effective_size));
    let border_width = rem_to_px(0.0625); // Contract: 0.0625rem solid

    // ── Display text ──
    let display_text = spec
        .current_value()
        .unwrap_or("--:--");
    let has_value = spec.current_value().is_some();
    let display_color = if has_value { text_color } else { placeholder_color };

    // ── Build element ──
    let mut el = ui_element::button(display_text)
        .min_h(min_height)
        .self_stretch() // Contract: stretches to parent width
        .pl(pad_x)
        .pr(pad_x)
        .rounded(radius)
        .bg(fill)
        .border(border_width)
        .border_color(border_color)
        .text_color(display_color)
        .text_size(font_size)
        .flex_row()
        .items_center()
        .focusable();

    // ── Disabled state ──
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    } else {
        el = el.cursor_pointer();
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn time_field_has_background() {
        let spec = TimeFieldSpec::new().with_default_value("14:30");
        let el = js_time_field(&spec, &theme());
        assert!(el.style.background.is_some());
    }

    #[test]
    fn disabled_has_reduced_opacity() {
        let spec = TimeFieldSpec::new().with_default_value("12:00");
        let mut spec = spec;
        spec.is_disabled = true;
        let el = js_time_field(&spec, &theme());
        assert!(el.style.disabled);
        assert!(el.style.opacity < 1.0);
    }

    #[test]
    fn sizes_produce_different_heights() {
        let sm = js_time_field(
            &{ let mut s = TimeFieldSpec::new(); s.size = ControlSize::Sm; s },
            &theme(),
        );
        let lg = js_time_field(
            &{ let mut s = TimeFieldSpec::new(); s.size = ControlSize::Lg; s },
            &theme(),
        );
        assert_ne!(
            sm.layout.min_size.height, lg.layout.min_size.height,
            "sm and lg should have different min-heights"
        );
    }
}
