use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{DateTimePicker, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DateTimePickerSpec, DateTimeValue, EyebrowSpec};
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

    let default_open = state.specimens.is_on("date-time-picker-default-open");
    let prefilled_open = state.specimens.is_on("date-time-picker-prefilled-open");

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(320.0)) // 20rem
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
                    let mut spec = DateTimePickerSpec::new();
                    spec.open = Some(default_open);
                    spec.aria_label = Some("Select date and time".to_string());
                    DateTimePicker::from_spec(spec, theme)
                        .with_id("default")
                        .on_toggle(toggle_handler(
                            &state.node_events,
                            "date-time-picker-default-open",
                        ))
                }),
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
                .child({
                    let value = DateTimeValue::new(
                        Some("2026-03-14".to_string()),
                        Some("14:30".to_string()),
                    );
                    let mut spec = DateTimePickerSpec::new().with_default_value(value);
                    spec.open = Some(prefilled_open);
                    spec.aria_label = Some("Pre-filled date time".to_string());
                    DateTimePicker::from_spec(spec, theme)
                        .with_id("with-value")
                        .on_toggle(toggle_handler(
                            &state.node_events,
                            "date-time-picker-prefilled-open",
                        ))
                }),
        )
        // --- Open (calendar + time) ---
        // Static open state so the composed real Calendar + TimeInput
        // section is always visible for review without interaction.
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Open (calendar + time)"),
                    theme,
                ))
                .child({
                    let value = DateTimeValue::new(
                        Some("2026-03-14".to_string()),
                        Some("14:30".to_string()),
                    );
                    let mut spec = DateTimePickerSpec::new().with_default_value(value);
                    spec.open = Some(true);
                    spec.aria_label = Some("Open date time picker".to_string());
                    DateTimePicker::from_spec(spec, theme).with_id("open")
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
                    let mut spec = DateTimePickerSpec::new();
                    spec.is_disabled = true;
                    spec.aria_label = Some("Disabled date time picker".to_string());
                    DateTimePicker::from_spec(spec, theme).with_id("disabled")
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "date-time-picker",
        examples,
        |size, theme: &GpuiThemeProvider| {
            let mut spec = DateTimePickerSpec::new();
            spec.aria_label = Some("Date time picker".to_string());
            DateTimePicker::from_spec(spec, theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            let mut spec = DateTimePickerSpec::new();
            spec.aria_label = Some("Date time picker".to_string());
            DateTimePicker::from_spec(spec, theme)
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
        },
    )
}
