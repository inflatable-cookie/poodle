//! TextInput — Jetstream text input backed by TextInputSpec.
//!
//! Contract: `docs/contracts/foundation/text-input.md`
//! Reference: `packages/svelte/primitives/src/TextInput.svelte`
//!
//! Uses focus ring and Color::mix for hover state.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use flint_jetstream::JetstreamThemeProvider;
use flint_primitives::TextInputSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_text_input(spec: &TextInputSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let height = resolve_px(theme, spec.control_height_token());
    let fill = resolve_color(theme, spec.fill_token());
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let pad_x = resolve_px(theme, spec.horizontal_padding_token());
    let text_color = resolve_color(theme, spec.text_color_token());
    let placeholder_color = resolve_color(theme, "semantic.color.text.secondary");
    let label_size = resolve_px(theme, "semantic.typography.body.size");

    // Hover border: contract color-mix(border 78%, text-primary)
    let border_c: Color = border_color.into();
    let text_primary: Color = resolve_color(theme, "semantic.color.text.primary").into();
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
        .gap(8.0)
        .focusable()
        .hover(|s| s.border_color(hover_border))
        .child(
            ui_element::label(show_text)
                .text_color(show_color)
                .text_size(label_size)
                .grow()
        );

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity).disabled(true);
    }

    el
}
