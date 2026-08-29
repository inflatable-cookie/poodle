//! The twelve audio-family specimen pages.
//!
//! Content comes from `poodle_render::audio_specimens`, which is shared with
//! Jetstream; the Examples / Sizes / Densities structure around it is GPUI's
//! own. Every one of these controls takes both `size` and `density`, so every
//! page admits both axis panes. Knob, Fader, and XYPad Examples own live
//! machines so interaction rebuilds the page.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_headless::audio::{FaderContext, FaderOrientation, KnobContext, XYPadContext};
use poodle_render::audio_specimens::AudioSpecimen;
use poodle_render::{
    fader_spec_from_context, fader_with_handlers, knob_spec_from_context, knob_with_handlers,
    xy_pad_spec_from_context, xy_pad_with_handlers, FaderHandlers, KnobHandlers, RenderContext,
    XYPadHandlers,
};

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use poodle_adapter::ThemeProvider;

fn to_element(node: poodle_node::Node) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&node)
}

thread_local! {
    static FADERS: RefCell<HashMap<String, Arc<Mutex<FaderContext>>>> =
        RefCell::new(HashMap::new());
    static KNOBS: RefCell<HashMap<String, Arc<Mutex<KnobContext>>>> =
        RefCell::new(HashMap::new());
    static PADS: RefCell<HashMap<String, Arc<Mutex<XYPadContext>>>> =
        RefCell::new(HashMap::new());
}

fn fader_machine(key: &str, seed: impl FnOnce() -> FaderContext) -> Arc<Mutex<FaderContext>> {
    FADERS.with(|machines| {
        machines
            .borrow_mut()
            .entry(key.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(seed())))
            .clone()
    })
}

fn knob_machine(key: &str, seed: impl FnOnce() -> KnobContext) -> Arc<Mutex<KnobContext>> {
    KNOBS.with(|machines| {
        machines
            .borrow_mut()
            .entry(key.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(seed())))
            .clone()
    })
}

fn pad_machine(key: &str, seed: impl FnOnce() -> XYPadContext) -> Arc<Mutex<XYPadContext>> {
    PADS.with(|machines| {
        machines
            .borrow_mut()
            .entry(key.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(seed())))
            .clone()
    })
}

fn live_fader(state: &AppState, key: &str, orientation: FaderOrientation) -> AnyElement {
    let machine = fader_machine(key, || {
        let mut context = FaderContext::default();
        context.base.value = 0.65;
        context.orientation = orientation;
        context
    });
    let spec = fader_spec_from_context(&machine.lock().expect("fader machine"), "Fader");
    let events = Arc::clone(&state.node_events);
    let value_key = key.to_owned();
    to_element(fader_with_handlers(
        &spec,
        &RenderContext::new(&state.theme),
        &FaderHandlers {
            on_value_change: Some(Arc::new(move |value| {
                events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                    key: value_key.clone(),
                    value: format!("{value:.2}"),
                });
            })),
            on_value_commit: Some(Arc::new(|_| {})),
            on_gesture_begin: Some(Arc::new(|| {})),
            on_gesture_end: Some(Arc::new(|| {})),
            machine: Some(machine),
        },
    ))
}

fn live_knob(state: &AppState, key: &str) -> AnyElement {
    let machine = knob_machine(key, || {
        let mut context = KnobContext::default();
        context.base.value = 0.6;
        context
    });
    let spec = knob_spec_from_context(&machine.lock().expect("knob machine"), "Knob");
    let events = Arc::clone(&state.node_events);
    let value_key = key.to_owned();
    to_element(knob_with_handlers(
        &spec,
        &RenderContext::new(&state.theme),
        &KnobHandlers {
            on_value_change: Some(Arc::new(move |value| {
                events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                    key: value_key.clone(),
                    value: format!("{value:.2}"),
                });
            })),
            on_value_commit: Some(Arc::new(|_| {})),
            on_gesture_begin: Some(Arc::new(|| {})),
            on_gesture_end: Some(Arc::new(|| {})),
            machine: Some(machine),
        },
    ))
}

fn live_pad(state: &AppState, key: &str) -> AnyElement {
    let machine = pad_machine(key, || {
        let mut context = XYPadContext::default();
        context.x = 0.4;
        context.y = 0.6;
        context
    });
    let spec = xy_pad_spec_from_context(&machine.lock().expect("xy pad machine"), "Pad");
    let events = Arc::clone(&state.node_events);
    let value_key = key.to_owned();
    to_element(xy_pad_with_handlers(
        &spec,
        &RenderContext::new(&state.theme),
        &XYPadHandlers {
            on_value_change: Some(Arc::new(move |x, y| {
                events.lock().unwrap().push(NodeSpecimenEvent::SetText {
                    key: value_key.clone(),
                    value: format!("{x:.2},{y:.2}"),
                });
            })),
            on_value_commit: Some(Arc::new(|_, _| {})),
            on_gesture_begin: Some(Arc::new(|| {})),
            on_gesture_end: Some(Arc::new(|| {})),
            machine: Some(machine),
        },
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
