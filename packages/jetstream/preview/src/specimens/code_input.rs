//! CodeInput specimen — partial value, complete, invalid, disabled, masked.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::code_input::js_code_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{CodeInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Partial value
        .child(group("Partial (3 of 6)", secondary,
            div().w(320.0)
                .child(js_code_input(&CodeInputSpec::new().with_value("123"), theme))
        ))
        // Complete
        .child(group("Complete", secondary,
            div().w(320.0)
                .child(js_code_input(&CodeInputSpec::new().with_value("123456"), theme))
        ))
        // Invalid
        .child(group("Invalid", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_value("999999")
                        .with_validation_state(ValidationState::Invalid)
                        .with_error("Invalid code. Please try again."),
                    theme,
                ))
        ))
        // Masked
        .child(group("4-digit Masked", secondary,
            div().w(320.0)
                .child(js_code_input(
                    &CodeInputSpec::new()
                        .with_length(4)
                        .with_value("1234")
                        .with_mask(true),
                    theme,
                ))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(320.0)
                .child(js_code_input(&CodeInputSpec::new().with_disabled(true), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
