//! DateTimePicker specimen — with value, placeholder.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_time_picker::js_date_time_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::{DateTimePickerSpec, DateTimeValue};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // With value
        .child(group("With value", secondary,
            div().w(320.0)
                .child(js_date_time_picker(
                    &DateTimePickerSpec::new()
                        .with_default_value(DateTimeValue::new(
                            Some("2026-03-30".into()),
                            Some("14:30".into()),
                        )),
                    theme,
                ))
        ))
        // Placeholder
        .child(group("Placeholder", secondary,
            div().w(320.0)
                .child(js_date_time_picker(&DateTimePickerSpec::new(), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = DateTimePickerSpec::new()
                .with_default_value(DateTimeValue::new(
                    Some("2026-06-15".into()),
                    Some("09:00".into()),
                ));
            spec.is_disabled = true;
            div().w(320.0).child(js_date_time_picker(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
