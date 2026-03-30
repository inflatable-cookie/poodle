//! DateRangePicker specimen — with range, placeholder, disabled.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::date_range_picker::js_date_range_picker;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{DateRangePickerSpec, DateRangeValue};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // With a range
        .child(group("With range", secondary,
            div().w(320.0)
                .child(js_date_range_picker(
                    &DateRangePickerSpec::new()
                        .with_default_value(DateRangeValue::new(
                            Some("2026-03-01".into()),
                            Some("2026-03-15".into()),
                        )),
                    theme,
                ))
        ))
        // Placeholder
        .child(group("Placeholder", secondary,
            div().w(320.0)
                .child(js_date_range_picker(&DateRangePickerSpec::new(), theme))
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = DateRangePickerSpec::new()
                .with_default_value(DateRangeValue::new(
                    Some("2026-01-01".into()),
                    Some("2026-01-31".into()),
                ));
            spec.is_disabled = true;
            div().w(320.0).child(js_date_range_picker(&spec, theme))
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
