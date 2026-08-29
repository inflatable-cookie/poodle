use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, TimeInput};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::time_input::TimeInputContext;
use poodle_specs::{EyebrowSpec, TimeInputSpec};
use std::sync::{Arc, Mutex};

fn change_handler(
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

fn live_time_input(
    state: &AppState,
    key: &str,
    spec: TimeInputSpec,
    theme: &GpuiThemeProvider,
    id: impl Into<String>,
) -> TimeInput {
    let live = {
        let mut map = state.time_input_live.lock().expect("time input live");
        map.entry(key.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(poodle_render::context_from_spec(&spec))))
            .clone()
    };
    let events = Arc::clone(&state.node_events);
    let text_key = key.to_string();
    TimeInput::from_spec(spec, theme)
        .with_id(id)
        .with_context(live)
        .on_context(Arc::new(move |next: TimeInputContext| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: text_key.clone(),
                value: next.committed.clone().unwrap_or_default(),
            });
        }))
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let default_value = state
        .specimens
        .text
        .get("time-input-default")
        .cloned()
        .unwrap_or_default();
    let meeting_value = state
        .specimens
        .text
        .get("time-input-meeting")
        .cloned()
        .unwrap_or_else(|| "14:30".to_string());

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(384.0))
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
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child({
                            let mut spec = TimeInputSpec::new();
                            if !default_value.is_empty() {
                                spec = spec.with_default_value(&default_value);
                            }
                            spec.aria_label = Some("Start time".to_string());
                            live_time_input(state, "time-input-default", spec, theme, "default")
                                .on_change(change_handler(&state.node_events, "time-input-default"))
                        })
                        .when(!default_value.is_empty(), |d| {
                            d.child(
                                div()
                                    .text_sm()
                                    .text_color(color_to_hsla(text_secondary))
                                    .child(format!("Value: {}", default_value)),
                            )
                        }),
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
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child({
                            let mut spec = TimeInputSpec::new().with_default_value(&meeting_value);
                            spec.aria_label = Some("Meeting time".to_string());
                            live_time_input(state, "time-input-meeting", spec, theme, "with-value")
                                .on_change(change_handler(&state.node_events, "time-input-meeting"))
                        })
                        .child(
                            div()
                                .text_sm()
                                .text_color(color_to_hsla(text_secondary))
                                .child(format!("Value: {}", meeting_value)),
                        ),
                ),
        )
        // --- With min/max constraints ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With min/max constraints"),
                    theme,
                ))
                .child({
                    let mut spec = TimeInputSpec::new().with_default_value("09:00");
                    spec.min = Some("08:00".to_string());
                    spec.max = Some("18:00".to_string());
                    spec.aria_label = Some("Office hours".to_string());
                    live_time_input(state, "time-input-constrained", spec, theme, "constrained")
                }),
        )
        // --- Seconds step ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Seconds step"),
                    theme,
                ))
                .child({
                    let mut spec = TimeInputSpec::new()
                        .with_default_value("09:30:15")
                        .with_step(15);
                    spec.aria_label = Some("Cue time".to_string());
                    live_time_input(state, "time-input-seconds", spec, theme, "seconds")
                }),
        )
        // --- Overnight ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Overnight"),
                    theme,
                ))
                .child({
                    let mut spec = TimeInputSpec::new()
                        .with_default_value("23:30")
                        .with_step(1800);
                    spec.min = Some("22:00".to_string());
                    spec.max = Some("06:00".to_string());
                    spec.aria_label = Some("Quiet hours".to_string());
                    live_time_input(state, "time-input-overnight", spec, theme, "overnight")
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
                    let mut spec = TimeInputSpec::new().with_default_value("12:00");
                    spec.is_disabled = true;
                    live_time_input(state, "time-input-disabled", spec, theme, "disabled")
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "time-input",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let mut spec = TimeInputSpec::new().with_default_value("09:30");
                spec.aria_label = Some("Time field".to_string());
                TimeInput::from_spec(spec, theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let mut spec = TimeInputSpec::new().with_default_value("09:30");
                spec.aria_label = Some("Time field".to_string());
                TimeInput::from_spec(spec, theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
