//! DateTimeRangePicker specimen — with range, placeholder.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_time_range_picker::js_date_time_range_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // With range
        .child(group("With range", secondary,
            div().w(400.0)
                .child(js_date_time_range_picker(
                    &DateTimeRangePickerSpec::new()
                        .with_default_value(DateTimeRangeValue::new(
                            DateTimeValue::new(Some("2026-03-01".into()), Some("09:00".into())),
                            DateTimeValue::new(Some("2026-03-15".into()), Some("17:00".into())),
                        )),
                    theme,
                ))
        ))
        // Placeholder
        .child(group("Placeholder", secondary,
            div().w(400.0)
                .child(js_date_time_range_picker(&DateTimeRangePickerSpec::new(), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = DateTimeRangePickerSpec::new()
                .with_default_value(DateTimeRangeValue::new(
                    DateTimeValue::new(Some("2026-01-01".into()), Some("08:00".into())),
                    DateTimeValue::new(Some("2026-01-31".into()), Some("18:00".into())),
                ));
            spec.is_disabled = true;
            div().w(400.0).child(js_date_time_range_picker(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
