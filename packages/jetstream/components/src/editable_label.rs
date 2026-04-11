//! EditableLabel — Jetstream click-to-edit label backed by EditableLabelSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::EditableLabelSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub fn js_editable_label(spec: &EditableLabelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let text_color = resolve_color(theme, spec.text_color_token());
    let font_size = rem_to_px(size_font_rem(effective_size));

    let display = if spec.value.is_empty() {
        spec.placeholder.as_deref().unwrap_or("")
    } else {
        spec.value.as_str()
    };

    let is_placeholder = spec.value.is_empty() && spec.placeholder.is_some();
    let color = if is_placeholder {
        resolve_color(theme, spec.placeholder_color_token())
    } else {
        text_color
    };

    let mut el = if spec.is_editing {
        let border_color = resolve_color(theme, spec.edit_border_token());
        let bg = resolve_color(theme, spec.fill_token());
        let radius = resolve_radius(theme, spec.radius_token());
        let body_size = resolve_px(theme, spec.body_size_token());
        // Editing mode: show as an input-like container
        ui_element::label(display)
            .text_color(color)
            .text_size(body_size)
            .bg(bg)
            .rounded(radius)
            .border(1.0).border_color(border_color)
    } else {
        ui_element::label(display)
            .text_color(color)
            .text_size(font_size)
            .cursor_pointer()
    };

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        el = el.opacity(opacity);
    }

    el
}
