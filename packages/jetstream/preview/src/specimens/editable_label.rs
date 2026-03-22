//! EditableLabel specimen — editable labels in display mode.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::editable_label::js_editable_label;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::EditableLabelSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // With text
        .child(group("Display mode", secondary,
            div().w(300.0)
                .child(js_editable_label(&EditableLabelSpec::new().with_value("Click to edit"), theme))
        ))
        // With placeholder
        .child(group("Empty (placeholder)", secondary,
            div().w(300.0)
                .child(js_editable_label(&EditableLabelSpec::new().with_placeholder("Enter a title..."), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
