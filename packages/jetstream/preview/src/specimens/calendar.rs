//! Calendar specimen — month grid for single-date and range selection.
//!
//! Every group renders the real `js_calendar` builder; all visuals resolve
//! from `CalendarSpec` + tokens. Specimens render static state, so selection
//! and range fields are seeded directly on the spec.

use jetstream_ui::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::calendar::js_calendar;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{CalendarMode, CalendarSpec, ControlDensity, ControlSize, DateRangeValue};

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Default — current month, no selection, today border treatment.
        .child(group(
            "Default",
            secondary,
            js_calendar(
                &{
                    let mut spec = CalendarSpec::new();
                    spec.aria_label = Some("Select a date".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Pre-selected date — March 14 shows selected treatment.
        .child(group(
            "With pre-selected date",
            secondary,
            js_calendar(
                &{
                    let mut spec = CalendarSpec::new();
                    spec.default_value = Some("2026-03-14".to_string());
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Calendar with default".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Range selection — start, in-range fill, and end across the month.
        .child(group(
            "Range selection",
            secondary,
            js_calendar(
                &{
                    let mut spec = CalendarSpec::new().with_mode(CalendarMode::Range);
                    spec.range_value = Some(DateRangeValue::new(
                        Some("2026-03-10".to_string()),
                        Some("2026-03-20".to_string()),
                    ));
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Pick a date range".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Range with pre-selected range — uncontrolled default range.
        .child(group(
            "Range with pre-selected range",
            secondary,
            js_calendar(
                &{
                    let mut spec = CalendarSpec::new().with_mode(CalendarMode::Range);
                    spec.default_range_value = DateRangeValue::new(
                        Some("2026-03-05".to_string()),
                        Some("2026-03-12".to_string()),
                    );
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Pre-selected range".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Disabled — selected day with reduced opacity on the root.
        .child(group(
            "Disabled",
            secondary,
            js_calendar(
                &{
                    let mut spec = CalendarSpec::new();
                    spec.default_value = Some("2026-03-01".to_string());
                    spec.visible_month = Some("2026-03".to_string());
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled calendar".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Range disabled.
        .child(group(
            "Range disabled",
            secondary,
            js_calendar(
                &{
                    let mut spec = CalendarSpec::new().with_mode(CalendarMode::Range);
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range calendar".to_string());
                    spec
                },
                theme,
            ),
        ))
        // Sizes — xs through xl drive cell-size, nav-button, day, month label.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(sized_calendar(theme, ControlSize::Xs))
                .child(sized_calendar(theme, ControlSize::Sm))
                .child(sized_calendar(theme, ControlSize::Md))
                .child(sized_calendar(theme, ControlSize::Lg))
                .child(sized_calendar(theme, ControlSize::Xl)),
        ))
        // Densities — spacing between siblings only.
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(16.0)
                .child(dense_calendar(theme, ControlDensity::Compact))
                .child(dense_calendar(theme, ControlDensity::Default))
                .child(dense_calendar(theme, ControlDensity::Comfortable)),
        ))
}

fn sized_calendar(theme: &JetstreamThemeProvider, size: ControlSize) -> JsEl {
    let mut spec = CalendarSpec::new().with_size(size);
    spec.default_value = Some("2026-03-14".to_string());
    spec.visible_month = Some("2026-03".to_string());
    spec.aria_label = Some("Calendar".to_string());
    js_calendar(&spec, theme)
}

fn dense_calendar(theme: &JetstreamThemeProvider, density: ControlDensity) -> JsEl {
    let mut spec = CalendarSpec::new().with_density(density);
    spec.default_value = Some("2026-03-14".to_string());
    spec.visible_month = Some("2026-03".to_string());
    spec.aria_label = Some("Calendar".to_string());
    js_calendar(&spec, theme)
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
