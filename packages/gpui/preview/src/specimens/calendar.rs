use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{CalendarMode, CalendarSpec, DateRangeValue, EyebrowSpec};
use poodle_gpui_components::{Calendar, Eyebrow};
use poodle_gpui::GpuiThemeProvider;
use crate::app_state::AppState;
use crate::specimens::specimen_layout::specimen_layout;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let selected_date = state.specimens.text.get("calendar-selected").cloned();
    let nav_month = state.specimens.text.get("calendar-nav-month").cloned();
    let range_start = state.specimens.text.get("calendar-range-start").cloned();
    let range_end = state.specimens.text.get("calendar-range-end").cloned();

    let examples = div().flex().flex_col().gap(px(24.0))
        // --- Default ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default"), theme))
                .child({
                    let mut spec = CalendarSpec::new();
                    spec.aria_label = Some("Select a date".to_string());
                    if let Some(ref date) = selected_date {
                        spec.value = Some(date.clone());
                    }
                    if let Some(ref month) = nav_month {
                        spec.visible_month = Some(month.clone());
                    }
                    Calendar::from_spec(spec, theme)
                        .with_id("interactive")
                        .on_select(cx.listener(|this, date: &str, _w, cx| {
                            this.state.specimens.text.insert("calendar-selected".to_string(), date.to_string());
                            cx.notify();
                        }))
                        .on_navigate(cx.listener(|this, month: &str, _w, cx| {
                            this.state.specimens.text.insert("calendar-nav-month".to_string(), month.to_string());
                            cx.notify();
                        }))
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Selected: {}", selected_date.as_deref().unwrap_or("(none)")))
                )
        )
        // --- With pre-selected date ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("With pre-selected date"), theme))
                .child({
                    let mut spec = CalendarSpec::new();
                    spec.default_value = Some("2026-03-14".to_string());
                    spec.aria_label = Some("Calendar with default".to_string());
                    Calendar::from_spec(spec, theme).with_id("preselected")
                })
        )
        // --- Range selection ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Range selection"), theme))
                .child({
                    // Seed a default range when nothing is picked yet so
                    // the specimen shows range styling immediately.
                    let (eff_start, eff_end) = match (&range_start, &range_end) {
                        (Some(s), Some(e)) => (Some(s.clone()), Some(e.clone())),
                        (Some(s), None) => (Some(s.clone()), None),
                        _ => (Some("2026-03-10".to_string()), Some("2026-03-20".to_string())),
                    };

                    let mut spec = CalendarSpec::new().with_mode(CalendarMode::Range);
                    spec.range_value = Some(DateRangeValue::new(eff_start.clone(), eff_end.clone()));
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Pick a date range".to_string());

                    Calendar::from_spec(spec, theme)
                        .with_id("range")
                        .on_range_select(cx.listener(|this, range: &DateRangeValue, _w, cx| {
                            if let Some(ref start) = range.start {
                                this.state.specimens.text.insert("calendar-range-start".to_string(), start.clone());
                            } else {
                                this.state.specimens.text.remove("calendar-range-start");
                            }
                            if let Some(ref end) = range.end {
                                this.state.specimens.text.insert("calendar-range-end".to_string(), end.clone());
                            } else {
                                this.state.specimens.text.remove("calendar-range-end");
                            }
                            cx.notify();
                        }))
                })
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!(
                            "Start: {} · End: {}",
                            range_start.as_deref().unwrap_or("(none)"),
                            range_end.as_deref().unwrap_or("(none)"),
                        ))
                )
        )
        // --- Range with pre-selected range ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Range with pre-selected range"), theme))
                .child({
                    let mut spec = CalendarSpec::new().with_mode(CalendarMode::Range);
                    spec.default_range_value = DateRangeValue::new(
                        Some("2026-03-05".to_string()),
                        Some("2026-03-12".to_string()),
                    );
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Pre-selected range".to_string());
                    Calendar::from_spec(spec, theme).with_id("range-preselected")
                })
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child({
                    let mut spec = CalendarSpec::new();
                    spec.default_value = Some("2026-03-01".to_string());
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled calendar".to_string());
                    Calendar::from_spec(spec, theme).with_id("disabled")
                })
        )
        // --- Range disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Range disabled"), theme))
                .child({
                    let mut spec = CalendarSpec::new().with_mode(CalendarMode::Range);
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range calendar".to_string());
                    Calendar::from_spec(spec, theme).with_id("range-disabled")
                })
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "calendar",
        examples,
        |size, theme: &GpuiThemeProvider| {
            let mut spec = CalendarSpec::new();
            spec.default_value = Some("2026-03-14".to_string());
            spec.aria_label = Some("Calendar".to_string());
            Calendar::from_spec(spec, theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            let mut spec = CalendarSpec::new();
            spec.default_value = Some("2026-03-14".to_string());
            spec.aria_label = Some("Calendar".to_string());
            Calendar::from_spec(spec, theme)
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
        },
    )
}
