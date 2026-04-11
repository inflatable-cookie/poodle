//! Calendar — Jetstream calendar grid backed by CalendarSpec.

use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::CalendarSpec;

use crate::theme_ext::resolve_color;

pub fn js_calendar(_spec: &CalendarSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.subtle");

    let mut el = ui_element::div()
        .flex_col().gap(4.0)
        .border(1.0).border_color(border)
        .pl(8.0).pr(8.0).pt(8.0).pb(8.0);

    // Day-of-week header
    let mut header = ui_element::div().flex_row().gap(4.0);
    for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
        header = header.child(
            ui_element::label(*day).text_color(muted).text_size(11.0).w(28.0).text_align_center()
        );
    }
    el = el.child(header);

    // Placeholder grid (shows month structure)
    for _week in 0..5 {
        let mut row = ui_element::div().flex_row().gap(4.0);
        for day in 1..=7 {
            row = row.child(
                ui_element::label(&day.to_string()).text_color(text_color).text_size(12.0).w(28.0).text_align_center()
            );
        }
        el = el.child(row);
    }

    el
}
