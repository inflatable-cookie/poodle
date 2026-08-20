use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Calendar, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CalendarMode, CalendarSpec, DateRangeValue, EyebrowSpec};
use std::sync::{Arc, Mutex};

fn text_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.to_string(),
            value: value.to_string(),
        });
    })
}

fn range_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
) -> Arc<dyn Fn(&DateRangeValue) + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move |range| {
        let mut events = events.lock().unwrap();
        events.push(NodeSpecimenEvent::SetOptionalText {
            key: "calendar-range-start".to_string(),
            value: range.start.clone(),
        });
        events.push(NodeSpecimenEvent::SetOptionalText {
            key: "calendar-range-end".to_string(),
            value: range.end.clone(),
        });
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_primary = theme.resolve_color("color.text.primary");

    let selected_date = state.specimens.text.get("calendar-selected").cloned();
    let nav_month = state.specimens.text.get("calendar-nav-month").cloned();
    let range_start = state.specimens.text.get("calendar-range-start").cloned();
    let range_end = state.specimens.text.get("calendar-range-end").cloned();

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default ---
        .child({
            let mut section = div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child({
                    let mut spec = CalendarSpec::new().with_today("2026-03-12");
                    spec.aria_label = Some("Select a date".to_string());
                    if let Some(ref date) = selected_date {
                        spec.value = Some(date.clone());
                    }
                    if let Some(ref month) = nav_month {
                        spec.visible_month = Some(month.clone());
                    }
                    Calendar::from_spec(spec, theme)
                        .with_id("interactive")
                        .on_select(text_handler(&state.node_events, "calendar-selected"))
                        .on_navigate(text_handler(&state.node_events, "calendar-nav-month"))
                });
            if let Some(ref date) = selected_date {
                section = section.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!("Selected: {date}")),
                );
            }
            section
        })
        // --- With pre-selected date ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With pre-selected date"),
                    theme,
                ))
                .child({
                    let mut spec = CalendarSpec::new().with_today("2026-03-12");
                    spec.default_value = Some("2026-03-14".to_string());
                    spec.aria_label = Some("Calendar with default".to_string());
                    Calendar::from_spec(spec, theme).with_id("preselected")
                }),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child({
                    let mut spec = CalendarSpec::new().with_today("2026-03-12");
                    spec.default_value = Some("2026-03-01".to_string());
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled calendar".to_string());
                    Calendar::from_spec(spec, theme).with_id("disabled")
                }),
        )
        // --- Range selection ---
        .child({
            let mut section = div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Range selection"),
                    theme,
                ))
                .child({
                    let mut spec = CalendarSpec::new()
                        .with_today("2026-03-12")
                        .with_mode(CalendarMode::Range);
                    if range_start.is_some() || range_end.is_some() {
                        spec.range_value =
                            Some(DateRangeValue::new(range_start.clone(), range_end.clone()));
                    }
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Select a date range".to_string());

                    Calendar::from_spec(spec, theme)
                        .with_id("range")
                        .on_range_select(range_handler(&state.node_events))
                });
            if let Some(ref start) = range_start {
                section = section.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(color_to_hsla(text_primary))
                        .child(format!(
                            "{start} → {}",
                            range_end.as_deref().unwrap_or("...")
                        )),
                );
            }
            section
        })
        // --- Range with pre-selected range ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Range with pre-selected range"),
                    theme,
                ))
                .child({
                    let mut spec = CalendarSpec::new()
                        .with_today("2026-03-12")
                        .with_mode(CalendarMode::Range);
                    spec.default_range_value = DateRangeValue::new(
                        Some("2026-03-05".to_string()),
                        Some("2026-03-12".to_string()),
                    );
                    spec.visible_month = Some("2026-03".to_string());
                    spec.aria_label = Some("Pre-selected range".to_string());
                    Calendar::from_spec(spec, theme).with_id("range-preselected")
                }),
        )
        // --- Range disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Range disabled"),
                    theme,
                ))
                .child({
                    let mut spec = CalendarSpec::new()
                        .with_today("2026-03-12")
                        .with_mode(CalendarMode::Range);
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range calendar".to_string());
                    Calendar::from_spec(spec, theme).with_id("range-disabled")
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "calendar",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let mut spec = CalendarSpec::new().with_today("2026-03-12");
                spec.default_value = Some("2026-03-14".to_string());
                spec.aria_label = Some("Calendar".to_string());
                Calendar::from_spec(spec, theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let mut spec = CalendarSpec::new().with_today("2026-03-12");
                spec.default_value = Some("2026-03-14".to_string());
                spec.aria_label = Some("Calendar".to_string());
                Calendar::from_spec(spec, theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
