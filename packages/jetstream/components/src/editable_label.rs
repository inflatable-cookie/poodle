//! EditableLabel — Jetstream click-to-edit label backed by EditableLabelSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::EditableLabelSpec;

use crate::theme_ext::resolve_color;

pub fn js_editable_label(spec: &EditableLabelSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "semantic.color.text.primary");

    let display = Some(spec.value.as_str())
        
        .unwrap_or("");

    ui_element::label(display)
        .text_color(text_color)
        .text_size(13.0)
}
