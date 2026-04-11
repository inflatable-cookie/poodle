//! TextInput — Jetstream text input backed by TextInputSpec.
//!
//! Contract: `docs/contracts/components/text-input.md`
//! Reference: `packages/svelte/primitives/src/TextInput.svelte`
//!
//! Uses focus ring and Color::mix for hover state.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::TextInputSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_text_input(spec: &TextInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let fill = resolve_color(theme, spec.fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let placeholder_color = resolve_color(theme, "color.text.secondary");

    // Hover border: contract color-mix(border 78%, text-primary)
    let border_c: Color = border_color.into();
    let text_primary: Color = resolve_color(theme, "color.text.primary").into();
    let hover_border = border_c.mix(text_primary, 0.78);

    let display_value = spec.value.as_deref()
        .or(Some(spec.default_value.as_str()))
        .unwrap_or("");
    let is_placeholder = display_value.is_empty() || spec.value.is_none();
    let show_text = if is_placeholder {
        spec.placeholder.as_deref().unwrap_or("")
    } else {
        display_value
    };
    let show_color = if is_placeholder { placeholder_color } else { text_color };

    let mut el = ui_element::div()
        .h(height)
        .bg(fill)
        .border(1.0).border_color(border_color)
        .rounded(radius)
        .pl(pad_x).pr(pad_x)
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.5))
        .focusable()
        .hover(|s| s.border_color(hover_border))
        .child(
            ui_element::label(show_text)
                .text_color(show_color)
                .text_size(font_size)
                .grow()
        );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    el
}
