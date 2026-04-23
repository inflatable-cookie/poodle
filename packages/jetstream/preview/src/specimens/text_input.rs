//! TextInput specimen — text inputs with placeholder, value, and disabled states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::text_input::js_text_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{TextInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

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
        // Invalid state
        .child(group("Invalid state", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("bad@").with_validation_state(ValidationState::Invalid), theme))
        ))
        // Valid state
        .child(group("Valid state", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("good@email.com").with_validation_state(ValidationState::Valid), theme))
        ))
        // Pending state
        .child(group("Pending state", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("checking...").with_validation_state(ValidationState::Pending), theme))
        ))
        // Leading icon
        .child(group("Leading icon", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Search...").with_leading_icon("search"), theme))
        ))
        // Trailing icon
        .child(group("Trailing icon", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Some text").with_trailing_icon("x-circle"), theme))
        ))
        // With prefix
        .child(group("With prefix", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("42.00").with_prefix("$"), theme))
        ))
        // With suffix
        .child(group("With suffix", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("100").with_suffix("USD"), theme))
        ))
        // Multiline (4 rows)
        .child(group("Multiline (4 rows)", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_placeholder("Enter description...").with_rows(4), theme))
        ))
        // Char count
        .child(group("Char count", secondary,
            div().w(300.0)
                .child(js_text_input(&TextInputSpec::new().with_value("Hello").with_max_length(100).with_show_char_count(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
