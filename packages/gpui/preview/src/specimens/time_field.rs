use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, TimeField};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EyebrowSpec, TimeFieldSpec};
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

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let default_value = state
        .specimens
        .text
        .get("time-field-default")
        .cloned()
        .unwrap_or_default();
    let meeting_value = state
        .specimens
        .text
        .get("time-field-meeting")
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
                            let mut spec = TimeFieldSpec::new();
                            if !default_value.is_empty() {
                                spec = spec.with_default_value(&default_value);
                            }
                            spec.aria_label = Some("Start time".to_string());
                            TimeField::from_spec(spec, theme)
                                .with_id("default")
                                .on_change(change_handler(&state.node_events, "time-field-default"))
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
                            let mut spec = TimeFieldSpec::new().with_default_value(&meeting_value);
                            spec.aria_label = Some("Meeting time".to_string());
                            TimeField::from_spec(spec, theme)
                                .with_id("with-value")
                                .on_change(change_handler(&state.node_events, "time-field-meeting"))
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
                    let mut spec = TimeFieldSpec::new().with_default_value("09:00");
                    spec.min = Some("08:00".to_string());
                    spec.max = Some("18:00".to_string());
                    spec.aria_label = Some("Office hours".to_string());
                    TimeField::from_spec(spec, theme).with_id("constrained")
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
                    let mut spec = TimeFieldSpec::new().with_default_value("12:00");
                    spec.is_disabled = true;
                    TimeField::from_spec(spec, theme).with_id("disabled")
                }),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "time-field",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let mut spec = TimeFieldSpec::new().with_default_value("09:30");
                spec.aria_label = Some("Time field".to_string());
                TimeField::from_spec(spec, theme)
                    .with_id(format!("specimen-size-{:?}", size))
                    .size(size)
                    .into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let mut spec = TimeFieldSpec::new().with_default_value("09:30");
                spec.aria_label = Some("Time field".to_string());
                TimeField::from_spec(spec, theme)
                    .with_id(format!("specimen-density-{:?}", density))
                    .with_density(density)
                    .into_any_element()
            }),
    )
}
