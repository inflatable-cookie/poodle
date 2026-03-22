//! TextArea specimen — text areas with default, value, and disabled states.

use jetstream_runtime::ui_element::*;
use pug_jetstream::JetstreamThemeProvider;
use pug_jetstream_components::text_area::js_text_area;
use pug_jetstream_components::theme_ext::*;
use pug_primitives::TextAreaSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default", secondary,
            div().w(300.0)
                .child(js_text_area(&TextAreaSpec::new().with_placeholder("Enter text...").with_rows(3), theme))
        ))
        // With value
        .child(group("With value", secondary,
            div().w(300.0)
                .child(js_text_area(&TextAreaSpec::new().with_value("This is some multiline\ncontent in a text area.").with_rows(3), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(300.0)
                .child(js_text_area(&TextAreaSpec::new().with_value("Disabled content").with_rows(3).with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
