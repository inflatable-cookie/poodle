use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DateTimeZonePicker, Eyebrow};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DateTimeZonePickerSpec, EyebrowSpec, ZonedDateTimeValue};
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

    let is_open = state.specimens.is_on("dtz-picker-open");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0))
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
                .child(
                    DateTimeZonePicker::from_spec(
                        DateTimeZonePickerSpec::new().with_open(is_open),
                        theme,
                        "dtz-picker-1",
                    )
                    .on_toggle(toggle_handler(&state.node_events, "dtz-picker-open")),
                ),
        )
        // --- With default value ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With default value"),
                    theme,
                ))
                .child(DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new().with_default_value(ZonedDateTimeValue::new(
                        Some("2026-03-14".into()),
                        Some("10:00".into()),
                        Some("America/Los_Angeles".into()),
                    )),
                    theme,
                    "dtz-picker-2",
                )),
        )
        // --- Open (calendar + time + zone) ---
        // Static open state so the composed real Calendar + TimeInput +
        // TimeZoneSelect fields are always visible for review without interaction.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Open (calendar + time + zone)"),
                    theme,
                ))
                .child(DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new()
                        .with_default_value(ZonedDateTimeValue::new(
                            Some("2026-03-23".into()),
                            Some("14:30".into()),
                            Some("America/New_York".into()),
                        ))
                        .with_open(true),
                    theme,
                    "dtz-picker-3",
                )),
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
                .child(DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new().with_disabled(true),
                    theme,
                    "dtz-picker-4",
                )),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-time-zone-picker",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new().with_value(ZonedDateTimeValue::new(
                        Some("2026-03-23".into()),
                        Some("14:30".into()),
                        Some("America/New_York".into()),
                    )),
                    theme,
                    "dtz-picker-5",
                )
                .size(size)
                .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                DateTimeZonePicker::from_spec(
                    DateTimeZonePickerSpec::new().with_value(ZonedDateTimeValue::new(
                        Some("2026-03-23".into()),
                        Some("14:30".into()),
                        Some("America/New_York".into()),
                    )),
                    theme,
                    "dtz-picker-6",
                )
                .with_density(density)
                .into_any_element()
            }),
    )
}
