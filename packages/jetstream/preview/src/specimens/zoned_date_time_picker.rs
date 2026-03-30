//! ZonedDateTimePicker specimen — with value, placeholder.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::zoned_date_time_picker::js_zoned_date_time_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::ZonedDateTimePickerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // With value and timezone
        .child(group("With value", secondary,
            div().w(360.0)
                .child(js_zoned_date_time_picker(
                    &ZonedDateTimePickerSpec::new()
                        .with_value("2026-03-30T14:30")
                        .with_time_zone("America/New_York"),
                    theme,
                ))
        ))
        // Placeholder (no value)
        .child(group("Placeholder", secondary,
            div().w(360.0)
                .child(js_zoned_date_time_picker(&ZonedDateTimePickerSpec::new(), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(360.0)
                .child(js_zoned_date_time_picker(
                    &ZonedDateTimePickerSpec::new()
                        .with_value("2026-06-15T09:00")
                        .with_time_zone("Europe/London")
                        .with_disabled(true),
                    theme,
                ))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
