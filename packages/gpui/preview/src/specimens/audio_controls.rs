//! The twelve audio-family specimen pages.
//!
//! Content comes from `poodle_render::audio_specimens`, which is shared with
//! Jetstream; the Examples / Sizes / Densities structure around it is GPUI's
//! own. Every one of these controls takes both `size` and `density`, so every
//! page admits both axis panes. Knob, Fader, and XYPad Examples bind
//! instance-scoped handlers so interaction rebuilds the page.

use std::sync::Arc;

use gpui::*;
use poodle_headless::audio::{AudioValueLaw, FaderOrientation};
use poodle_render::audio_specimens::AudioSpecimen;
use poodle_render::{
    fader_retained_spec, fader_with_handlers, knob_retained_spec, knob_with_handlers,
    xy_pad_retained_spec, xy_pad_with_handlers, FaderHandlers, KnobHandlers, RenderContext,
    XYPadHandlers,
};
use poodle_specs::{FaderSpec, KnobSpec, Orientation, XYPadSpec};

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use poodle_adapter::ThemeProvider;

fn to_element(node: poodle_node::Node) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&node)
}

fn live_fader(state: &AppState, key: &str, orientation: FaderOrientation) -> AnyElement {
    let spec = fader_retained_spec(key, "Fader").unwrap_or_else(|| {
        let mut spec = FaderSpec::new(0.65, 0.0, 1.0, AudioValueLaw::Linear);
        spec.orientation = match orientation {
            FaderOrientation::Vertical => Orientation::Vertical,
            FaderOrientation::Horizontal => Orientation::Horizontal,
        };
        spec.aria_label = "Fader".into();
        spec
    });
    let events = Arc::clone(&state.node_events);
    let value_key = key.to_owned();
    to_element(fader_with_handlers(
        &spec,
        &RenderContext::new(&state.theme),
        &FaderHandlers::new(key).on_value_change(Arc::new(move |value| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: value_key.clone(),
                value: format!("{value:.2}"),
            });
        })),
    ))
}

fn live_knob(state: &AppState, key: &str) -> AnyElement {
    let spec = knob_retained_spec(key, "Knob").unwrap_or_else(|| {
        let mut spec = KnobSpec::new(0.6, 0.0, 1.0, AudioValueLaw::Linear);
        spec.aria_label = "Knob".into();
        spec
    });
    let events = Arc::clone(&state.node_events);
    let value_key = key.to_owned();
    to_element(knob_with_handlers(
        &spec,
        &RenderContext::new(&state.theme),
        &KnobHandlers::new(key).on_value_change(Arc::new(move |value| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: value_key.clone(),
                value: format!("{value:.2}"),
            });
        })),
    ))
}

fn live_pad(state: &AppState, key: &str) -> AnyElement {
    let spec = xy_pad_retained_spec(key, "Pad").unwrap_or_else(|| {
        let mut spec = XYPadSpec::new(poodle_headless::audio::XYPadVisualState {
            x_norm: 0.4,
            y_norm: 0.6,
            raw_x: 0.4,
            raw_y: 0.6,
            hover: false,
            focus: false,
            drag: poodle_headless::audio::DragState::None,
            automation: poodle_headless::audio::AutomationState::None,
            enabled: true,
        });
        spec.aria_label = "Pad".into();
        spec
    });
    let events = Arc::clone(&state.node_events);
    let value_key = key.to_owned();
    to_element(xy_pad_with_handlers(
        &spec,
        &RenderContext::new(&state.theme),
        &XYPadHandlers::new(key).on_value_change(Arc::new(move |x, y| {
            events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                key: value_key.clone(),
                value: format!("{x:.2},{y:.2}"),
            });
        })),
    ))
}

fn interactive_examples(specimen: AudioSpecimen, state: &AppState) -> AnyElement {
    let theme = &state.theme;
    let muted = color_to_hsla(theme.resolve_color("color.text.secondary"));
    match specimen {
        AudioSpecimen::Fader => {
            let vertical = state
                .specimens
                .text
                .get("fader-live-vertical")
                .cloned()
                .unwrap_or_else(|| "0.65".into());
            let horizontal = state
                .specimens
                .text
                .get("fader-live-horizontal")
                .cloned()
                .unwrap_or_else(|| "0.65".into());
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(
                    div()
                        .flex()
                        .gap(px(24.0))
                        .child(live_fader(
                            state,
                            "fader-live-vertical",
                            FaderOrientation::Vertical,
                        ))
                        .child(live_fader(
                            state,
                            "fader-live-horizontal",
                            FaderOrientation::Horizontal,
                        )),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(muted)
                        .child(format!("Vertical {vertical} · Horizontal {horizontal}")),
                )
                .child(to_element(
                    specimen.examples(&RenderContext::new(&state.theme)),
                ))
                .into_any_element()
        }
        AudioSpecimen::Knob => {
            let value = state
                .specimens
                .text
                .get("knob-live")
                .cloned()
                .unwrap_or_else(|| "0.60".into());
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(live_knob(state, "knob-live"))
                .child(
                    div()
                        .text_sm()
                        .text_color(muted)
                        .child(format!("Value {value}")),
                )
                .child(to_element(
                    specimen.examples(&RenderContext::new(&state.theme)),
                ))
                .into_any_element()
        }
        AudioSpecimen::XyPad => {
            let value = state
                .specimens
                .text
                .get("xy-pad-live")
                .cloned()
                .unwrap_or_else(|| "0.40,0.60".into());
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(live_pad(state, "xy-pad-live"))
                .child(
                    div()
                        .text_sm()
                        .text_color(muted)
                        .child(format!("Position {value}")),
                )
                .child(to_element(
                    specimen.examples(&RenderContext::new(&state.theme)),
                ))
                .into_any_element()
        }
        _ => to_element(specimen.examples(&RenderContext::new(&state.theme))),
    }
}

pub(crate) fn render(
    specimen: AudioSpecimen,
    name: &str,
    state: &AppState,
    cx: &mut Context<PreviewRoot>,
) -> Div {
    let examples = interactive_examples(specimen, state);
    specimen_layout(
        state,
        cx,
        name,
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(move |size, theme| {
                to_element(specimen.size(size, &poodle_render::RenderContext::new(theme)))
            })
            .with_densities(move |density, theme| {
                to_element(specimen.density(density, &poodle_render::RenderContext::new(theme)))
            }),
    )
}
