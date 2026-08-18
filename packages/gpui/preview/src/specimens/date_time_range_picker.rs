use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DateTimeRangePicker, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DateTimeRangePickerSpec, DateTimeRangeValue, DateTimeValue, EyebrowSpec};
use std::sync::{Arc, Mutex};

fn toggle_handler(
    events: &Arc<Mutex<Vec<NodeSpecimenEvent>>>,
    key: &'static str,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = Arc::clone(events);
    Arc::new(move || {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Toggle(key.to_string()));
    })
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    let default_open = state.specimens.is_on("datetime-range-default-open");
    let prefilled_open = state.specimens.is_on("datetime-range-prefilled-open");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0)) // 24rem
        // --- Default ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default"),
                    theme,
                ))
                .child({
                    let mut spec = DateTimeRangePickerSpec::new();
                    spec.open = Some(default_open);
                    spec.aria_label = Some("Select date and time range".to_string());
                    DateTimeRangePicker::from_spec(spec, theme)
                        .with_id("default")
                        .on_toggle(toggle_handler(
                            &state.node_events,
                            "datetime-range-default-open",
                        ))
                }),
        )
        // --- With default range ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With default range"),
                    theme,
                ))
                .child({
                    let range = DateTimeRangeValue::new(
                        DateTimeValue::new(
                            Some("2026-03-10".to_string()),
                            Some("09:00".to_string()),
                        ),
                        DateTimeValue::new(
                            Some("2026-03-14".to_string()),
                            Some("17:00".to_string()),
                        ),
                    );
                    let mut spec = DateTimeRangePickerSpec::new().with_default_value(range);
                    spec.open = Some(prefilled_open);
                    spec.aria_label = Some("Pre-filled range".to_string());
                    DateTimeRangePicker::from_spec(spec, theme)
                        .with_id("with-range")
                        .on_toggle(toggle_handler(
                            &state.node_events,
                            "datetime-range-prefilled-open",
                        ))
                }),
        )
        // --- Open (range calendar + start/end time) ---
        // Static open state so the composed real range Calendar plus the
        // paired START/END TimeInput sections are always visible for review.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Open (range calendar + start/end time)"),
                    theme,
                ))
                .child({
                    let range = DateTimeRangeValue::new(
                        DateTimeValue::new(
                            Some("2026-03-10".to_string()),
                            Some("09:00".to_string()),
                        ),
                        DateTimeValue::new(
                            Some("2026-03-14".to_string()),
                            Some("17:00".to_string()),
                        ),
                    );
                    let mut spec = DateTimeRangePickerSpec::new().with_default_value(range);
                    spec.open = Some(true);
                    spec.aria_label = Some("Open range picker".to_string());
                    DateTimeRangePicker::from_spec(spec, theme).with_id("open")
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
                    let mut spec = DateTimeRangePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled range picker".to_string());
                    DateTimeRangePicker::from_spec(spec, theme).with_id("disabled")
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-time-range-picker",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let mut spec = DateTimeRangePickerSpec::new();
                spec.aria_label = Some("Date time range".to_string());
                DateTimeRangePicker::from_spec(spec, theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let mut spec = DateTimeRangePickerSpec::new();
                spec.aria_label = Some("Date time range".to_string());
                DateTimeRangePicker::from_spec(spec, theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
