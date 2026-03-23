//! TextInput specimen — text inputs with placeholder, value, and disabled states.

use jetstream_runtime::ui_element::*;
use flint_jetstream::JetstreamThemeProvider;
use flint_jetstream_components::text_input::js_text_input;
use flint_jetstream_components::theme_ext::*;
use flint_primitives::TextInputSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default (with placeholder)
        .child(group("Default", secondary,
            div().flex_col().gap(8.0).w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Enter text..."), theme))
        ))
        // With value
        .child(group("With value", secondary,
            div().flex_col().gap(8.0).w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Hello world"), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().flex_col().gap(8.0).w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Disabled").with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
