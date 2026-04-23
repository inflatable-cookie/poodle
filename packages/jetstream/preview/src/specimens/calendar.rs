//! Calendar specimen — date grid display.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::calendar::js_calendar;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CalendarMode, CalendarSpec};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Default", secondary,
            js_calendar(&CalendarSpec::new(), theme)
        ))
        // With selected date
        .child(group("With selected date", secondary,
            js_calendar(&CalendarSpec::new().with_value("2026-04-15"), theme)
        ))
        // Range mode
        .child(group("Range mode", secondary,
            js_calendar(&CalendarSpec::new().with_mode(CalendarMode::Range), theme)
        ))
        // Disabled
        .child(group("Disabled", secondary, {
            let mut spec = CalendarSpec::new();
            spec.is_disabled = true;
            js_calendar(&spec, theme)
        }))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
