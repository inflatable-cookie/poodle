//! RangeCalendar specimen — default month view, with range.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::range_calendar::js_range_calendar;
use poodle_jetstream_components::theme_ext::*;
use poodle_primitives::{DateRangeValue, RangeCalendarSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "semantic.color.text.secondary");

    div().flex_col().gap(24.0)
        // Default (no selection)
        .child(group("Default month view", secondary,
            js_range_calendar(&RangeCalendarSpec::new(), theme)
        ))
        // With selected range
        .child(group("With selected range", secondary,
            js_range_calendar(
                &RangeCalendarSpec::new()
                    .with_default_value(DateRangeValue::new(
                        Some("2026-03-10".into()),
                        Some("2026-03-20".into()),
                    )),
                theme,
            )
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = RangeCalendarSpec::new();
            spec.is_disabled = true;
            js_range_calendar(&spec, theme)
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
