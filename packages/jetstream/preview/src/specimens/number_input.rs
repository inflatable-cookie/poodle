//! NumberInput specimen — number entries at default, min, max, and disabled states.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::number_input::js_number_input;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{NumberInputSpec, ValidationState};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Default
        .child(group("Default (50)", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(50.0).with_min(0.0).with_max(100.0).with_step(1.0), theme))
        ))
        // At min
        .child(group("At min (0)", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(0.0).with_min(0.0).with_max(100.0).with_step(1.0), theme))
        ))
        // At max
        .child(group("At max (100)", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(100.0).with_min(0.0).with_max(100.0).with_step(1.0), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(42.0).with_min(0.0).with_max(100.0).with_disabled(true), theme))
        ))
        // Invalid state
        .child(group("Invalid state", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(50.0).with_min(0.0).with_max(100.0).with_validation_state(ValidationState::Invalid), theme))
        ))
        // With prefix ($)
        .child(group("With prefix ($)", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(42.50).with_min(0.0).with_max(1000.0).with_prefix("$").with_precision(2), theme))
        ))
        // With suffix (px)
        .child(group("With suffix (px)", secondary,
            div().w(300.0)
                .child(js_number_input(&NumberInputSpec::new(16.0).with_min(0.0).with_max(100.0).with_suffix("px"), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
