//! SplitButton — Jetstream split button backed by SplitButtonSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::SplitButtonSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_split_button(spec: &SplitButtonSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));
    let separator_h = rem_to_px(control_height_rem(effective_size) * 0.56);
    let trigger_px = rem_to_px(0.375);

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, spec.border_token());
    let separator = resolve_color(theme, spec.separator_token());
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");

    let label = spec.label.as_deref().unwrap_or("");

    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center()
        .h(height);

    // Primary action
    el = el.child(
        ui_element::button(label)
            .text_color(text_color).text_size(font_size).text_weight(500)
            .pl(pad_x).pr(pad_x)
            .focusable()
    );

    // Separator
    el = el.child(ui_element::div().w(1.0).h(separator_h).bg(separator));

    // Dropdown trigger
    el = el.child(
        ui_element::button("")
            .child(ui_element::icon("chevron-down").w(icon_size).h(icon_size).text_color(text_color))
            .pl(trigger_px).pr(trigger_px)
            .focusable()
    );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        el = el.opacity(opacity);
    }

    el
}

#[cfg(test)]
mod tests {
    use super::*;
    use jetstream_runtime::game_ui::Color;
    use poodle_specs::{ButtonTone, ButtonVariant};

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    #[test]
    fn success_tone_round_trips_through_split_button() {
        let th = theme();
        let success: Color = resolve_color(&th, "color.status.success").into();
        // Primary + Success resolves the success fill via the shared
        // `ButtonVariant::fill_token(tone)` path, proving the new tone round-trips
        // through SplitButtonSpec.
        let el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Success)
                .with_label("Save"),
            &th,
        );
        let bg = el.style.background.expect("split-button bg set");
        assert!(
            (bg.r - success.r).abs() < 0.02
                && (bg.g - success.g).abs() < 0.02
                && (bg.b - success.b).abs() < 0.02,
            "primary success split-button fill should be status-success, got {bg:?}"
        );
    }

    #[test]
    fn default_and_success_fills_differ() {
        let th = theme();
        let default_el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_label("Save"),
            &th,
        );
        let success_el = js_split_button(
            &SplitButtonSpec::new()
                .with_variant(ButtonVariant::Primary)
                .with_tone(ButtonTone::Success)
                .with_label("Save"),
            &th,
        );
        let d = default_el.style.background.expect("default bg");
        let s = success_el.style.background.expect("success bg");
        assert!(
            (d.r - s.r).abs() > 0.001 || (d.g - s.g).abs() > 0.001 || (d.b - s.b).abs() > 0.001,
            "success split-button fill must differ from accent default"
        );
    }
}
