//! DateTimeZonePicker specimen — with value, placeholder.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_time_zone_picker::js_date_time_zone_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::DateTimeZonePickerSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // With value and timezone
        .child(group("With value", secondary,
            div().w(360.0)
                .child(js_date_time_zone_picker(
                    &DateTimeZonePickerSpec::new()
                        .with_value("2026-03-30T14:30")
                        .with_time_zone("America/New_York"),
                    theme,
                ))
        ))
        // Placeholder (no value)
        .child(group("Placeholder", secondary,
            div().w(360.0)
                .child(js_date_time_zone_picker(&DateTimeZonePickerSpec::new(), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary,
            div().w(360.0)
                .child(js_date_time_zone_picker(
                    &DateTimeZonePickerSpec::new()
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
