//! Handler-backed Knob, Fader, and XYPad. VisualState stays the drawing input;
//! the host may retain a machine across rebuilds.

use std::sync::{Arc, Mutex};

use poodle_headless::audio::{
    fader_transition, format_value, knob_point_to_norm, knob_transition, xy_pad_transition,
    AudioPoint, AudioRect, AudioValueContext, AudioValueEffect, AudioValueEvent, FaderContext,
    FaderOrientation, KnobContext, KnobDragMode, ValueBound, XYPadAxis, XYPadContext, XYPadEffect,
    XYPadEvent,
};
use poodle_node::{
    ContinuousValuePhase, FocusRing, Node, NodeContinuousValueEvent, NodeKey, NodeRole,
    NodeWheelEvent,
};
use poodle_specs::{FaderSpec, KnobSpec, Orientation, XYPadSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

fn audio_focus_ring(ctx: &RenderContext<'_>) -> FocusRing {
    FocusRing {
        color: with_alpha(ctx.theme().resolve_color("color.accent.base"), 0.32),
        width: rem_to_px(0.1875),
        offset: 0.0,
    }
}

fn fader_orientation(orientation: Orientation) -> FaderOrientation {
    match orientation {
        Orientation::Vertical => FaderOrientation::Vertical,
        Orientation::Horizontal => FaderOrientation::Horizontal,
    }
}

fn orientation_name(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Vertical => "vertical",
        Orientation::Horizontal => "horizontal",
    }
}

fn bind_slider_a11y(
    node: &mut Node,
    label: &str,
    value: f64,
    min: f64,
    max: f64,
    value_text: &str,
    orientation: Option<&str>,
    enabled: bool,
    ring: Option<FocusRing>,
) {
    node.a11y.role = Some(NodeRole::Slider);
    node.a11y.label = Some(label.to_owned());
    node.a11y.value = Some(value);
    node.a11y.value_min = Some(min);
    node.a11y.value_max = Some(max);
    node.a11y.value_text = Some(value_text.to_owned());
    node.a11y.orientation = orientation.map(str::to_owned);
    node.interaction.disabled = !enabled;
    if enabled {
        node.interaction.focusable = true;
        node.a11y.tab_index = Some(0);
        node.style.focus_ring = ring;
    } else {
        node.interaction.focusable = false;
        node.a11y.tab_index = None;
        node.style.focus_ring = None;
    }
}

fn audio_nudge(key: NodeKey) -> Option<(i8, f64)> {
    match key {
        NodeKey::ArrowLeft | NodeKey::ArrowDown => Some((-1, 1.0)),
        NodeKey::ArrowRight | NodeKey::ArrowUp => Some((1, 1.0)),
        NodeKey::PageDown => Some((-1, 10.0)),
        NodeKey::PageUp => Some((1, 10.0)),
        _ => None,
    }
}

#[derive(Clone, Default)]
struct ScalarHandlers {
    on_value_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
}

fn apply_scalar_effects(effects: &[AudioValueEffect], handlers: &ScalarHandlers) {
    for effect in effects {
        match effect {
            AudioValueEffect::ValueChange(value) => {
                if let Some(handler) = &handlers.on_value_change {
                    handler(*value);
                }
            }
            AudioValueEffect::ValueCommit(value) => {
                if let Some(handler) = &handlers.on_value_commit {
                    handler(*value);
                }
            }
            AudioValueEffect::GestureBegin => {
                if let Some(handler) = &handlers.on_gesture_begin {
                    handler();
                }
            }
            AudioValueEffect::GestureEnd => {
                if let Some(handler) = &handlers.on_gesture_end {
                    handler();
                }
            }
            AudioValueEffect::RequestEntryFocus => {}
        }
    }
}

/// Contract effects plus optional host-owned machine retained across rebuilds.
#[derive(Clone, Default)]
pub struct FaderHandlers {
    pub on_value_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
    pub machine: Option<Arc<Mutex<FaderContext>>>,
}

/// Contract effects plus optional host-owned machine retained across rebuilds.
#[derive(Clone, Default)]
pub struct KnobHandlers {
    pub on_value_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
    pub machine: Option<Arc<Mutex<KnobContext>>>,
}

/// Contract effects plus optional host-owned machine retained across rebuilds.
#[derive(Clone, Default)]
pub struct XYPadHandlers {
    pub on_value_change: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
    pub machine: Option<Arc<Mutex<XYPadContext>>>,
}

pub fn fader_context_from_spec(spec: &FaderSpec) -> FaderContext {
    FaderContext {
        base: scalar_base(
            spec.visual_state.raw_value,
            spec.min,
            spec.max,
            spec.law,
            spec,
        ),
        orientation: fader_orientation(spec.orientation),
        detents: spec.detents.clone(),
        detent_snap: spec.detent_snap,
    }
}

fn scalar_base(
    value: f64,
    min: f64,
    max: f64,
    law: poodle_headless::audio::AudioValueLaw,
    spec: &FaderSpec,
) -> AudioValueContext {
    AudioValueContext {
        value,
        min,
        max,
        law,
        default_value: spec.default_value,
        keyboard_step: spec.keyboard_step,
        format: spec.format,
        hover: spec.visual_state.hover,
        focus: spec.visual_state.focus,
        drag: spec.visual_state.drag,
        automation: spec.visual_state.automation,
        entry_open: spec.entry_open,
        drag_start_value: spec.drag_start_value,
        drag_start_position: spec.drag_start_position,
        disabled: !spec.visual_state.enabled,
    }
}

pub fn knob_context_from_spec(spec: &KnobSpec) -> KnobContext {
    KnobContext {
        base: AudioValueContext {
            value: spec.visual_state.raw_value,
            min: spec.min,
            max: spec.max,
            law: spec.law,
            default_value: spec.default_value,
            keyboard_step: spec.keyboard_step,
            format: spec.format,
            hover: spec.visual_state.hover,
            focus: spec.visual_state.focus,
            drag: spec.visual_state.drag,
            automation: spec.visual_state.automation,
            entry_open: spec.entry_open,
            drag_start_value: spec.drag_start_value,
            drag_start_position: spec.drag_start_position,
            disabled: !spec.visual_state.enabled,
        },
        drag_mode: spec.drag_mode,
        drag_sensitivity: spec.drag_sensitivity,
    }
}

pub fn fader_spec_from_context(context: &FaderContext, aria_label: impl Into<String>) -> FaderSpec {
    let visual = context.visual_state();
    let mut spec = FaderSpec::new(
        context.base.value,
        context.base.min,
        context.base.max,
        context.base.law,
    );
    spec.visual_state = visual;
    spec.orientation = match context.orientation {
        FaderOrientation::Vertical => Orientation::Vertical,
        FaderOrientation::Horizontal => Orientation::Horizontal,
    };
    spec.detents = context.detents.clone();
    spec.detent_snap = context.detent_snap;
    spec.default_value = context.base.default_value;
    spec.keyboard_step = context.base.keyboard_step;
    spec.format = context.base.format;
    spec.entry_open = context.base.entry_open;
    spec.drag_start_value = context.base.drag_start_value;
    spec.drag_start_position = context.base.drag_start_position;
    spec.value_text = context.base.value_text();
    spec.aria_label = aria_label.into();
    spec
}

pub fn knob_spec_from_context(context: &KnobContext, aria_label: impl Into<String>) -> KnobSpec {
    let visual = context.visual_state();
    let mut spec = KnobSpec::new(
        context.base.value,
        context.base.min,
        context.base.max,
        context.base.law,
    );
    spec.visual_state = visual;
    spec.default_value = context.base.default_value;
    spec.keyboard_step = context.base.keyboard_step;
    spec.format = context.base.format;
    spec.drag_mode = context.drag_mode;
    spec.drag_sensitivity = context.drag_sensitivity;
    spec.entry_open = context.base.entry_open;
    spec.drag_start_value = context.base.drag_start_value;
    spec.drag_start_position = context.base.drag_start_position;
    spec.value_text = context.base.value_text();
    spec.aria_label = aria_label.into();
    spec
}

pub fn xy_pad_spec_from_context(
    context: &XYPadContext,
    aria_label: impl Into<String>,
) -> XYPadSpec {
    let visual = context.visual_state();
    let mut spec = XYPadSpec::new(visual);
    spec.min_x = context.min_x;
    spec.max_x = context.max_x;
    spec.min_y = context.min_y;
    spec.max_y = context.max_y;
    spec.law_x = context.law_x;
    spec.law_y = context.law_y;
    spec.default_x = context.default_x;
    spec.default_y = context.default_y;
    spec.keyboard_step_x = context.keyboard_step_x;
    spec.keyboard_step_y = context.keyboard_step_y;
    spec.drag_start_x = context.drag_start_x;
    spec.drag_start_y = context.drag_start_y;
    spec.drag_start_norm_x = context.drag_start_norm_x;
    spec.drag_start_norm_y = context.drag_start_norm_y;
    spec.aria_label = aria_label.into();
    spec.x_value_text = format_value(context.x, spec.format_x);
    spec.y_value_text = format_value(context.y, spec.format_y);
    spec
}

pub fn xy_pad_context_from_spec(spec: &XYPadSpec) -> XYPadContext {
    XYPadContext {
        x: spec.visual_state.raw_x,
        y: spec.visual_state.raw_y,
        min_x: spec.min_x,
        max_x: spec.max_x,
        min_y: spec.min_y,
        max_y: spec.max_y,
        law_x: spec.law_x,
        law_y: spec.law_y,
        default_x: spec.default_x,
        default_y: spec.default_y,
        keyboard_step_x: spec.keyboard_step_x,
        keyboard_step_y: spec.keyboard_step_y,
        hover: spec.visual_state.hover,
        focus: spec.visual_state.focus,
        drag: spec.visual_state.drag,
        automation: spec.visual_state.automation,
        drag_start_x: spec.drag_start_x,
        drag_start_y: spec.drag_start_y,
        drag_start_norm_x: spec.drag_start_norm_x,
        drag_start_norm_y: spec.drag_start_norm_y,
        disabled: !spec.visual_state.enabled,
    }
}

fn has_scalar_handlers(handlers: &ScalarHandlers) -> bool {
    handlers.on_value_change.is_some()
        || handlers.on_value_commit.is_some()
        || handlers.on_gesture_begin.is_some()
        || handlers.on_gesture_end.is_some()
}

pub fn bind_fader(
    node: &mut Node,
    spec: &FaderSpec,
    ctx: &RenderContext<'_>,
    handlers: &FaderHandlers,
) {
    let enabled = spec.visual_state.enabled;
    bind_slider_a11y(
        node,
        &spec.aria_label,
        spec.visual_state.raw_value,
        spec.min,
        spec.max,
        &spec.value_text,
        Some(orientation_name(spec.orientation)),
        enabled,
        enabled.then(|| audio_focus_ring(ctx)),
    );
    let scalar = ScalarHandlers {
        on_value_change: handlers.on_value_change.clone(),
        on_value_commit: handlers.on_value_commit.clone(),
        on_gesture_begin: handlers.on_gesture_begin.clone(),
        on_gesture_end: handlers.on_gesture_end.clone(),
    };
    if !has_scalar_handlers(&scalar) && handlers.machine.is_none() {
        return;
    }
    let live = handlers
        .machine
        .clone()
        .unwrap_or_else(|| Arc::new(Mutex::new(fader_context_from_spec(spec))));
    let orientation = spec.orientation;
    let draft = Arc::new(Mutex::new(spec.entry_draft.clone()));
    bind_fader_pointer(node, Arc::clone(&live), scalar.clone(), orientation);
    bind_fader_wheel(node, Arc::clone(&live), scalar.clone());
    bind_fader_reset(node, Arc::clone(&live), scalar.clone());
    bind_fader_keys(node, Arc::clone(&live), scalar.clone());
    bind_fader_entry(node, spec, Arc::clone(&live), draft, scalar);
}

fn run_fader(live: &Mutex<FaderContext>, event: AudioValueEvent, handlers: &ScalarHandlers) {
    let current = live.lock().expect("fader machine").clone();
    let (next, effects) = fader_transition(current, event);
    *live.lock().expect("fader machine") = next;
    apply_scalar_effects(&effects, handlers);
}

fn bind_fader_pointer(
    node: &mut Node,
    live: Arc<Mutex<FaderContext>>,
    handlers: ScalarHandlers,
    orientation: Orientation,
) {
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let fine = event.modifiers.shift;
            let value_norm = match orientation {
                Orientation::Horizontal => event.x as f64,
                Orientation::Vertical => event.y as f64,
            };
            match event.phase {
                ContinuousValuePhase::Press => {
                    run_fader(
                        &live,
                        AudioValueEvent::DragBegin {
                            position: value_norm,
                            fine,
                        },
                        &handlers,
                    );
                    run_fader(
                        &live,
                        AudioValueEvent::DragSetNorm { value_norm, fine },
                        &handlers,
                    );
                }
                ContinuousValuePhase::Move => {
                    run_fader(
                        &live,
                        AudioValueEvent::DragSetNorm { value_norm, fine },
                        &handlers,
                    );
                }
                ContinuousValuePhase::Release => {
                    run_fader(&live, AudioValueEvent::DragEnd, &handlers);
                }
                ContinuousValuePhase::Cancel => {
                    run_fader(&live, AudioValueEvent::DragCancel, &handlers);
                }
            }
        }));
}

fn bind_fader_wheel(node: &mut Node, live: Arc<Mutex<FaderContext>>, handlers: ScalarHandlers) {
    node.interaction.on_wheel = Some(Arc::new(move |event: &NodeWheelEvent| {
        if event.dy == 0.0 {
            return;
        }
        run_fader(
            &live,
            AudioValueEvent::Wheel {
                direction: event.dy as i8,
                fine: event.modifiers.shift,
            },
            &handlers,
        );
    }));
}

fn bind_fader_reset(node: &mut Node, live: Arc<Mutex<FaderContext>>, handlers: ScalarHandlers) {
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_fader(&live, AudioValueEvent::Reset, &handlers);
    }));
}

fn bind_fader_keys(node: &mut Node, live: Arc<Mutex<FaderContext>>, handlers: ScalarHandlers) {
    let submit_live = Arc::clone(&live);
    node.interaction.on_key = Some(Arc::new(move |key, mods| {
        let event = if let Some((direction, multiplier)) = audio_nudge(key) {
            AudioValueEvent::KeyNudge {
                direction,
                multiplier,
                fine: mods.shift,
            }
        } else if key == NodeKey::Home {
            AudioValueEvent::KeyBound {
                bound: ValueBound::Min,
            }
        } else if key == NodeKey::End {
            AudioValueEvent::KeyBound {
                bound: ValueBound::Max,
            }
        } else {
            return None;
        };
        run_fader(&live, event, &handlers);
        None
    }));
    node.interaction.on_submit = Some(Arc::new(move || {
        run_fader(
            &submit_live,
            AudioValueEvent::EntryOpen,
            &ScalarHandlers::default(),
        );
    }));
}

fn bind_fader_entry(
    node: &mut Node,
    spec: &FaderSpec,
    live: Arc<Mutex<FaderContext>>,
    draft: Arc<Mutex<String>>,
    handlers: ScalarHandlers,
) {
    let entry_open = live.lock().expect("fader machine").base.entry_open || spec.entry_open;
    if !entry_open {
        return;
    }
    let text = {
        let stored = draft.lock().expect("entry draft");
        if stored.is_empty() {
            let machine = live.lock().expect("fader machine");
            format_value(machine.base.value, machine.base.format)
        } else {
            stored.clone()
        }
    };
    let mut entry = Node::input(text, "");
    entry.id = Some("fader-entry".into());
    entry.interaction.focusable = true;
    let edit_draft = Arc::clone(&draft);
    entry.interaction.on_text_change = Some(Arc::new(move |value: &str| {
        *edit_draft.lock().expect("entry draft") = value.to_owned();
    }));
    let commit_live = Arc::clone(&live);
    let commit_draft = Arc::clone(&draft);
    let commit_handlers = handlers.clone();
    entry.interaction.on_submit = Some(Arc::new(move || {
        let text = commit_draft.lock().expect("entry draft").clone();
        run_fader(
            &commit_live,
            AudioValueEvent::EntryCommit { text },
            &commit_handlers,
        );
    }));
    let cancel_live = Arc::clone(&live);
    entry.interaction.on_cancel = Some(Arc::new(move || {
        run_fader(
            &cancel_live,
            AudioValueEvent::EntryCancel,
            &ScalarHandlers::default(),
        );
    }));
    let blur_live = Arc::clone(&live);
    let blur_draft = Arc::clone(&draft);
    let blur_handlers = handlers;
    entry.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused || !blur_live.lock().expect("fader machine").base.entry_open {
            return;
        }
        let text = blur_draft.lock().expect("entry draft").clone();
        run_fader(
            &blur_live,
            AudioValueEvent::EntryCommit { text },
            &blur_handlers,
        );
    }));
    *node = std::mem::take(node).child(entry);
}

pub fn bind_knob(
    node: &mut Node,
    spec: &KnobSpec,
    ctx: &RenderContext<'_>,
    handlers: &KnobHandlers,
) {
    let enabled = spec.visual_state.enabled;
    bind_slider_a11y(
        node,
        &spec.aria_label,
        spec.visual_state.raw_value,
        spec.min,
        spec.max,
        &spec.value_text,
        None,
        enabled,
        enabled.then(|| audio_focus_ring(ctx)),
    );
    let scalar = ScalarHandlers {
        on_value_change: handlers.on_value_change.clone(),
        on_value_commit: handlers.on_value_commit.clone(),
        on_gesture_begin: handlers.on_gesture_begin.clone(),
        on_gesture_end: handlers.on_gesture_end.clone(),
    };
    if !has_scalar_handlers(&scalar) && handlers.machine.is_none() {
        return;
    }
    let live = handlers
        .machine
        .clone()
        .unwrap_or_else(|| Arc::new(Mutex::new(knob_context_from_spec(spec))));
    let pointer = Arc::new(Mutex::new(spec.pointer_position));
    let draft = Arc::new(Mutex::new(spec.entry_draft.clone()));
    bind_knob_pointer(
        node,
        Arc::clone(&live),
        Arc::clone(&pointer),
        scalar.clone(),
    );
    bind_knob_wheel(node, Arc::clone(&live), scalar.clone());
    bind_knob_reset(node, Arc::clone(&live), scalar.clone());
    bind_knob_keys(node, Arc::clone(&live), scalar.clone());
    bind_knob_entry(node, spec, live, draft, scalar);
}

fn run_knob(live: &Mutex<KnobContext>, event: AudioValueEvent, handlers: &ScalarHandlers) {
    let current = live.lock().expect("knob machine").clone();
    let (next, effects) = knob_transition(current, event);
    *live.lock().expect("knob machine") = next;
    apply_scalar_effects(&effects, handlers);
}

fn bind_knob_pointer(
    node: &mut Node,
    live: Arc<Mutex<KnobContext>>,
    pointer: Arc<Mutex<f64>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let fine = event.modifiers.shift;
            let mode = live.lock().expect("knob machine").drag_mode;
            match event.phase {
                ContinuousValuePhase::Press => {
                    if mode == KnobDragMode::Vertical {
                        *pointer.lock().expect("knob pointer") = 0.0;
                        run_knob(
                            &live,
                            AudioValueEvent::DragBegin {
                                position: 0.0,
                                fine,
                            },
                            &handlers,
                        );
                    } else {
                        let value_norm = circular_norm(event);
                        run_knob(
                            &live,
                            AudioValueEvent::DragBegin {
                                position: value_norm,
                                fine,
                            },
                            &handlers,
                        );
                        run_knob(
                            &live,
                            AudioValueEvent::DragSetNorm { value_norm, fine },
                            &handlers,
                        );
                    }
                }
                ContinuousValuePhase::Move => {
                    if mode == KnobDragMode::Vertical {
                        let mut y = pointer.lock().expect("knob pointer");
                        *y -= event.delta_y as f64;
                        run_knob(
                            &live,
                            AudioValueEvent::DragMove { position: *y, fine },
                            &handlers,
                        );
                    } else {
                        run_knob(
                            &live,
                            AudioValueEvent::DragSetNorm {
                                value_norm: circular_norm(event),
                                fine,
                            },
                            &handlers,
                        );
                    }
                }
                ContinuousValuePhase::Release => {
                    run_knob(&live, AudioValueEvent::DragEnd, &handlers)
                }
                ContinuousValuePhase::Cancel => {
                    run_knob(&live, AudioValueEvent::DragCancel, &handlers)
                }
            }
        }));
}

fn circular_norm(event: &NodeContinuousValueEvent) -> f64 {
    knob_point_to_norm(
        AudioPoint {
            x: event.x as f64,
            y: 1.0 - event.y as f64,
        },
        AudioRect {
            left: 0.0,
            top: 0.0,
            width: 1.0,
            height: 1.0,
        },
    )
}

fn bind_knob_wheel(node: &mut Node, live: Arc<Mutex<KnobContext>>, handlers: ScalarHandlers) {
    node.interaction.on_wheel = Some(Arc::new(move |event: &NodeWheelEvent| {
        if event.dy == 0.0 {
            return;
        }
        run_knob(
            &live,
            AudioValueEvent::Wheel {
                direction: event.dy as i8,
                fine: event.modifiers.shift,
            },
            &handlers,
        );
    }));
}

fn bind_knob_reset(node: &mut Node, live: Arc<Mutex<KnobContext>>, handlers: ScalarHandlers) {
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_knob(&live, AudioValueEvent::Reset, &handlers);
    }));
}

fn bind_knob_keys(node: &mut Node, live: Arc<Mutex<KnobContext>>, handlers: ScalarHandlers) {
    let submit_live = Arc::clone(&live);
    node.interaction.on_key = Some(Arc::new(move |key, mods| {
        let event = if let Some((direction, multiplier)) = audio_nudge(key) {
            AudioValueEvent::KeyNudge {
                direction,
                multiplier,
                fine: mods.shift,
            }
        } else if key == NodeKey::Home {
            AudioValueEvent::KeyBound {
                bound: ValueBound::Min,
            }
        } else if key == NodeKey::End {
            AudioValueEvent::KeyBound {
                bound: ValueBound::Max,
            }
        } else {
            return None;
        };
        run_knob(&live, event, &handlers);
        None
    }));
    node.interaction.on_submit = Some(Arc::new(move || {
        run_knob(
            &submit_live,
            AudioValueEvent::EntryOpen,
            &ScalarHandlers::default(),
        );
    }));
}

fn bind_knob_entry(
    node: &mut Node,
    spec: &KnobSpec,
    live: Arc<Mutex<KnobContext>>,
    draft: Arc<Mutex<String>>,
    handlers: ScalarHandlers,
) {
    let entry_open = live.lock().expect("knob machine").base.entry_open || spec.entry_open;
    if !entry_open {
        return;
    }
    let text = {
        let stored = draft.lock().expect("entry draft");
        if stored.is_empty() {
            let machine = live.lock().expect("knob machine");
            format_value(machine.base.value, machine.base.format)
        } else {
            stored.clone()
        }
    };
    let mut entry = Node::input(text, "");
    entry.id = Some("knob-entry".into());
    entry.interaction.focusable = true;
    let edit_draft = Arc::clone(&draft);
    entry.interaction.on_text_change = Some(Arc::new(move |value: &str| {
        *edit_draft.lock().expect("entry draft") = value.to_owned();
    }));
    let commit_live = Arc::clone(&live);
    let commit_draft = Arc::clone(&draft);
    let commit_handlers = handlers.clone();
    entry.interaction.on_submit = Some(Arc::new(move || {
        let text = commit_draft.lock().expect("entry draft").clone();
        run_knob(
            &commit_live,
            AudioValueEvent::EntryCommit { text },
            &commit_handlers,
        );
    }));
    let cancel_live = Arc::clone(&live);
    entry.interaction.on_cancel = Some(Arc::new(move || {
        run_knob(
            &cancel_live,
            AudioValueEvent::EntryCancel,
            &ScalarHandlers::default(),
        );
    }));
    let blur_live = live;
    let blur_draft = draft;
    entry.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused || !blur_live.lock().expect("knob machine").base.entry_open {
            return;
        }
        let text = blur_draft.lock().expect("entry draft").clone();
        run_knob(&blur_live, AudioValueEvent::EntryCommit { text }, &handlers);
    }));
    *node = std::mem::take(node).child(entry);
}

pub fn bind_xy_pad(
    node: &mut Node,
    spec: &XYPadSpec,
    ctx: &RenderContext<'_>,
    handlers: &XYPadHandlers,
) {
    let enabled = spec.visual_state.enabled;
    node.a11y.role = Some(NodeRole::Group);
    node.a11y.label = Some(spec.aria_label.clone());
    node.interaction.disabled = !enabled;
    let interactive = handlers.on_value_change.is_some()
        || handlers.on_value_commit.is_some()
        || handlers.on_gesture_begin.is_some()
        || handlers.on_gesture_end.is_some()
        || handlers.machine.is_some();
    if !interactive {
        return;
    }
    let live = handlers
        .machine
        .clone()
        .unwrap_or_else(|| Arc::new(Mutex::new(xy_pad_context_from_spec(spec))));
    bind_xy_pointer(node, Arc::clone(&live), handlers);
    bind_xy_reset(node, Arc::clone(&live), handlers);
    let ring = enabled.then(|| audio_focus_ring(ctx));
    let mut x = Node::container();
    x.id = Some("xy-pad-x".into());
    x.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(1.0);
    x.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(1.0);
    bind_slider_a11y(
        &mut x,
        &format!("{} {}", spec.aria_label, spec.x_label),
        spec.visual_state.raw_x,
        spec.min_x,
        spec.max_x,
        &spec.x_value_text,
        Some("horizontal"),
        enabled,
        ring,
    );
    bind_xy_axis_keys(&mut x, Arc::clone(&live), handlers.clone(), XYPadAxis::X);
    let mut y = Node::container();
    y.id = Some("xy-pad-y".into());
    y.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(1.0);
    y.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(1.0);
    bind_slider_a11y(
        &mut y,
        &format!("{} {}", spec.aria_label, spec.y_label),
        spec.visual_state.raw_y,
        spec.min_y,
        spec.max_y,
        &spec.y_value_text,
        Some("vertical"),
        enabled,
        ring,
    );
    bind_xy_axis_keys(&mut y, live, handlers.clone(), XYPadAxis::Y);
    *node = std::mem::take(node).child(x).child(y);
}

fn run_xy(live: &Mutex<XYPadContext>, event: XYPadEvent, handlers: &XYPadHandlers) {
    let current = live.lock().expect("xy pad machine").clone();
    let (next, effects) = xy_pad_transition(current, event);
    *live.lock().expect("xy pad machine") = next;
    for effect in effects {
        match effect {
            XYPadEffect::ValueChange(x, y) => {
                if let Some(handler) = &handlers.on_value_change {
                    handler(x, y);
                }
            }
            XYPadEffect::ValueCommit(x, y) => {
                if let Some(handler) = &handlers.on_value_commit {
                    handler(x, y);
                }
            }
            XYPadEffect::GestureBegin => {
                if let Some(handler) = &handlers.on_gesture_begin {
                    handler();
                }
            }
            XYPadEffect::GestureEnd => {
                if let Some(handler) = &handlers.on_gesture_end {
                    handler();
                }
            }
        }
    }
}

fn bind_xy_pointer(node: &mut Node, live: Arc<Mutex<XYPadContext>>, handlers: &XYPadHandlers) {
    let handlers = handlers.clone();
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let fine = event.modifiers.shift;
            let x_norm = event.x as f64;
            let y_norm = event.y as f64;
            match event.phase {
                ContinuousValuePhase::Press => {
                    run_xy(
                        &live,
                        XYPadEvent::DragBegin {
                            x_norm,
                            y_norm,
                            fine,
                        },
                        &handlers,
                    );
                }
                ContinuousValuePhase::Move => {
                    run_xy(
                        &live,
                        XYPadEvent::DragMove {
                            x_norm,
                            y_norm,
                            fine,
                        },
                        &handlers,
                    );
                }
                ContinuousValuePhase::Release => run_xy(&live, XYPadEvent::DragEnd, &handlers),
                ContinuousValuePhase::Cancel => run_xy(&live, XYPadEvent::DragCancel, &handlers),
            }
        }));
}

fn bind_xy_reset(node: &mut Node, live: Arc<Mutex<XYPadContext>>, handlers: &XYPadHandlers) {
    let handlers = handlers.clone();
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_xy(&live, XYPadEvent::Reset, &handlers);
    }));
}

fn bind_xy_axis_keys(
    node: &mut Node,
    live: Arc<Mutex<XYPadContext>>,
    handlers: XYPadHandlers,
    axis: XYPadAxis,
) {
    node.interaction.on_key = Some(Arc::new(move |key, mods| {
        let event = if let Some((direction, multiplier)) = audio_nudge(key) {
            XYPadEvent::Nudge {
                axis,
                direction,
                multiplier,
                fine: mods.shift,
            }
        } else if key == NodeKey::Home {
            XYPadEvent::Bound {
                axis,
                bound: ValueBound::Min,
            }
        } else if key == NodeKey::End {
            XYPadEvent::Bound {
                axis,
                bound: ValueBound::Max,
            }
        } else {
            return None;
        };
        run_xy(&live, event, &handlers);
        None
    }));
}
