//! Handler-backed Knob, Fader, and XYPad.
//!
//! Handler structs expose the four contract effects plus a required
//! lifetime-stable `instance_id`. Machine state lives in a renderer-owned
//! registry keyed by that id, not on the callback surface.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use poodle_headless::audio::{
    fader_transition, format_value, knob_point_to_norm, knob_transition, xy_pad_transition,
    AudioPoint, AudioRect, AudioValueContext, AudioValueEffect, AudioValueEvent, DragState,
    FaderContext, FaderOrientation, KnobContext, KnobDragMode, ValueBound, XYPadAxis, XYPadContext,
    XYPadEffect, XYPadEvent,
};
use poodle_node::{
    ContinuousValuePhase, FocusRing, Node, NodeContinuousValueEvent, NodeKey, NodeRole,
    NodeWheelEvent,
};
use poodle_specs::{FaderSpec, KnobSpec, Orientation, XYPadSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PendingFocus {
    None,
    Entry,
    Root,
}

struct ScalarRuntime<C> {
    machine: C,
    draft: String,
    draft_replace: bool,
    pointer: f64,
    pending_focus: PendingFocus,
}

thread_local! {
    static FADERS: RefCell<HashMap<String, Arc<Mutex<ScalarRuntime<FaderContext>>>>> =
        RefCell::new(HashMap::new());
    static KNOBS: RefCell<HashMap<String, Arc<Mutex<ScalarRuntime<KnobContext>>>>> =
        RefCell::new(HashMap::new());
    static PADS: RefCell<HashMap<String, Arc<Mutex<XYPadContext>>>> =
        RefCell::new(HashMap::new());
}

fn audio_focus_ring(ctx: &RenderContext<'_>) -> FocusRing {
    FocusRing {
        color: with_alpha(ctx.theme().resolve_color("color.accent.base"), 0.32),
        width: rem_to_px(0.1875),
        offset: 0.0,
    }
}

fn entry_focus_ring(ctx: &RenderContext<'_>) -> FocusRing {
    FocusRing {
        color: ctx.theme().resolve_color("color.accent.base"),
        width: rem_to_px(0.125),
        offset: rem_to_px(0.125),
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

/// Root Node id for a handler-backed audio control.
pub fn audio_root_id(instance_id: &str) -> String {
    instance_id.to_owned()
}

/// Type-in field id for a Knob or Fader instance.
pub fn audio_entry_id(instance_id: &str) -> String {
    format!("{instance_id}:entry")
}

/// XYPad X-axis slider id.
pub fn xy_pad_x_id(instance_id: &str) -> String {
    format!("{instance_id}:x")
}

/// XYPad Y-axis slider id.
pub fn xy_pad_y_id(instance_id: &str) -> String {
    format!("{instance_id}:y")
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

fn apply_draft_insert(draft: &mut String, replace: &mut bool, text: &str) {
    if *replace {
        draft.clear();
        *replace = false;
    }
    draft.push_str(text);
}

fn apply_draft_key(draft: &mut String, replace: &mut bool, key: &str, accel: bool) {
    if accel && key == "a" {
        *replace = true;
        return;
    }
    if key == "backspace" {
        if *replace {
            draft.clear();
            *replace = false;
        } else {
            draft.pop();
        }
        return;
    }
    if key.len() == 1 && !accel {
        apply_draft_insert(draft, replace, key);
    }
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

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone)]
pub struct FaderHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl FaderHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_value_change: None,
            on_value_commit: None,
            on_gesture_begin: None,
            on_gesture_end: None,
        }
    }

    pub fn on_value_change(mut self, handler: Arc<dyn Fn(f64) + Send + Sync>) -> Self {
        self.on_value_change = Some(handler);
        self
    }

    pub fn on_value_commit(mut self, handler: Arc<dyn Fn(f64) + Send + Sync>) -> Self {
        self.on_value_commit = Some(handler);
        self
    }

    pub fn on_gesture_begin(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_gesture_begin = Some(handler);
        self
    }

    pub fn on_gesture_end(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_gesture_end = Some(handler);
        self
    }
}

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone)]
pub struct KnobHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl KnobHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_value_change: None,
            on_value_commit: None,
            on_gesture_begin: None,
            on_gesture_end: None,
        }
    }

    pub fn on_value_change(mut self, handler: Arc<dyn Fn(f64) + Send + Sync>) -> Self {
        self.on_value_change = Some(handler);
        self
    }

    pub fn on_value_commit(mut self, handler: Arc<dyn Fn(f64) + Send + Sync>) -> Self {
        self.on_value_commit = Some(handler);
        self
    }

    pub fn on_gesture_begin(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_gesture_begin = Some(handler);
        self
    }

    pub fn on_gesture_end(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_gesture_end = Some(handler);
        self
    }
}

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone)]
pub struct XYPadHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl XYPadHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            on_value_change: None,
            on_value_commit: None,
            on_gesture_begin: None,
            on_gesture_end: None,
        }
    }

    pub fn on_value_change(mut self, handler: Arc<dyn Fn(f64, f64) + Send + Sync>) -> Self {
        self.on_value_change = Some(handler);
        self
    }

    pub fn on_value_commit(mut self, handler: Arc<dyn Fn(f64, f64) + Send + Sync>) -> Self {
        self.on_value_commit = Some(handler);
        self
    }

    pub fn on_gesture_begin(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_gesture_begin = Some(handler);
        self
    }

    pub fn on_gesture_end(mut self, handler: Arc<dyn Fn() + Send + Sync>) -> Self {
        self.on_gesture_end = Some(handler);
        self
    }
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

fn apply_host_fader(machine: &mut FaderContext, spec: &FaderSpec) {
    machine.orientation = fader_orientation(spec.orientation);
    machine.detents = spec.detents.clone();
    machine.detent_snap = spec.detent_snap;
    machine.base.min = spec.min;
    machine.base.max = spec.max;
    machine.base.law = spec.law;
    machine.base.default_value = spec.default_value;
    machine.base.keyboard_step = spec.keyboard_step;
    machine.base.format = spec.format;
    machine.base.disabled = !spec.visual_state.enabled;
    machine.base.hover = spec.visual_state.hover;
    machine.base.automation = spec.visual_state.automation;
    if machine.base.drag == DragState::None && !machine.base.entry_open {
        machine.base.value = spec.visual_state.raw_value;
        machine.base.focus = spec.visual_state.focus;
    }
}

fn apply_host_knob(machine: &mut KnobContext, spec: &KnobSpec) {
    machine.drag_mode = spec.drag_mode;
    machine.drag_sensitivity = spec.drag_sensitivity;
    machine.base.min = spec.min;
    machine.base.max = spec.max;
    machine.base.law = spec.law;
    machine.base.default_value = spec.default_value;
    machine.base.keyboard_step = spec.keyboard_step;
    machine.base.format = spec.format;
    machine.base.disabled = !spec.visual_state.enabled;
    machine.base.hover = spec.visual_state.hover;
    machine.base.automation = spec.visual_state.automation;
    if machine.base.drag == DragState::None && !machine.base.entry_open {
        machine.base.value = spec.visual_state.raw_value;
        machine.base.focus = spec.visual_state.focus;
    }
}

fn apply_host_xy(machine: &mut XYPadContext, spec: &XYPadSpec) {
    machine.min_x = spec.min_x;
    machine.max_x = spec.max_x;
    machine.min_y = spec.min_y;
    machine.max_y = spec.max_y;
    machine.law_x = spec.law_x;
    machine.law_y = spec.law_y;
    machine.default_x = spec.default_x;
    machine.default_y = spec.default_y;
    machine.keyboard_step_x = spec.keyboard_step_x;
    machine.keyboard_step_y = spec.keyboard_step_y;
    machine.disabled = !spec.visual_state.enabled;
    machine.hover = spec.visual_state.hover;
    machine.automation = spec.visual_state.automation;
    if machine.drag == DragState::None {
        machine.x = spec.visual_state.raw_x;
        machine.y = spec.visual_state.raw_y;
        machine.focus = spec.visual_state.focus;
    }
}

fn retain_fader(instance_id: &str, spec: &FaderSpec) -> Arc<Mutex<ScalarRuntime<FaderContext>>> {
    FADERS.with(|slot| {
        let mut map = slot.borrow_mut();
        if let Some(existing) = map.get(instance_id) {
            {
                let mut runtime = existing.lock().expect("fader machine");
                apply_host_fader(&mut runtime.machine, spec);
            }
            existing.clone()
        } else {
            let live = Arc::new(Mutex::new(ScalarRuntime {
                machine: fader_context_from_spec(spec),
                draft: spec.entry_draft.clone(),
                draft_replace: true,
                pointer: 0.0,
                pending_focus: PendingFocus::None,
            }));
            map.insert(instance_id.to_owned(), Arc::clone(&live));
            live
        }
    })
}

fn retain_knob(instance_id: &str, spec: &KnobSpec) -> Arc<Mutex<ScalarRuntime<KnobContext>>> {
    KNOBS.with(|slot| {
        let mut map = slot.borrow_mut();
        if let Some(existing) = map.get(instance_id) {
            {
                let mut runtime = existing.lock().expect("knob machine");
                apply_host_knob(&mut runtime.machine, spec);
            }
            existing.clone()
        } else {
            let live = Arc::new(Mutex::new(ScalarRuntime {
                machine: knob_context_from_spec(spec),
                draft: spec.entry_draft.clone(),
                draft_replace: true,
                pointer: spec.pointer_position,
                pending_focus: PendingFocus::None,
            }));
            map.insert(instance_id.to_owned(), Arc::clone(&live));
            live
        }
    })
}

fn retain_xy(instance_id: &str, spec: &XYPadSpec) -> Arc<Mutex<XYPadContext>> {
    PADS.with(|slot| {
        let mut map = slot.borrow_mut();
        if let Some(existing) = map.get(instance_id) {
            {
                let mut machine = existing.lock().expect("xy pad machine");
                apply_host_xy(&mut machine, spec);
            }
            existing.clone()
        } else {
            let live = Arc::new(Mutex::new(xy_pad_context_from_spec(spec)));
            map.insert(instance_id.to_owned(), Arc::clone(&live));
            live
        }
    })
}

/// Snapshot of the retained Fader machine, when one exists.
pub fn fader_retained_spec(instance_id: &str, aria_label: impl Into<String>) -> Option<FaderSpec> {
    FADERS.with(|slot| {
        slot.borrow().get(instance_id).map(|live| {
            fader_spec_from_context(&live.lock().expect("fader machine").machine, aria_label)
        })
    })
}

/// Snapshot of the retained Knob machine, when one exists.
pub fn knob_retained_spec(instance_id: &str, aria_label: impl Into<String>) -> Option<KnobSpec> {
    KNOBS.with(|slot| {
        slot.borrow().get(instance_id).map(|live| {
            knob_spec_from_context(&live.lock().expect("knob machine").machine, aria_label)
        })
    })
}

/// Snapshot of the retained XYPad machine, when one exists.
pub fn xy_pad_retained_spec(instance_id: &str, aria_label: impl Into<String>) -> Option<XYPadSpec> {
    PADS.with(|slot| {
        slot.borrow()
            .get(instance_id)
            .map(|live| xy_pad_spec_from_context(&live.lock().expect("xy pad machine"), aria_label))
    })
}

/// Drop retained machines. Tests use this so instance ids do not leak across
/// cases; hosts do not need it for ordinary rebuilds.
pub fn reset_audio_runtime() {
    FADERS.with(|slot| slot.borrow_mut().clear());
    KNOBS.with(|slot| slot.borrow_mut().clear());
    PADS.with(|slot| slot.borrow_mut().clear());
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

pub fn bind_fader(
    node: &mut Node,
    spec: &FaderSpec,
    ctx: &RenderContext<'_>,
    handlers: &FaderHandlers,
) {
    let instance_id = handlers.instance_id.as_str();
    node.id = Some(audio_root_id(instance_id));
    let live = retain_fader(instance_id, spec);
    let (enabled, value, min, max, value_text, orientation, entry_open, pending) = {
        let runtime = live.lock().expect("fader machine");
        let machine = &runtime.machine;
        (
            !machine.base.disabled,
            machine.base.value,
            machine.base.min,
            machine.base.max,
            machine.base.value_text(),
            spec.orientation,
            machine.base.entry_open,
            runtime.pending_focus,
        )
    };
    bind_slider_a11y(
        node,
        &spec.aria_label,
        value,
        min,
        max,
        &value_text,
        Some(orientation_name(orientation)),
        enabled,
        enabled.then(|| audio_focus_ring(ctx)),
    );
    if pending == PendingFocus::Root {
        node.interaction.request_focus = true;
        live.lock().expect("fader machine").pending_focus = PendingFocus::None;
    }
    let scalar = ScalarHandlers {
        on_value_change: handlers.on_value_change.clone(),
        on_value_commit: handlers.on_value_commit.clone(),
        on_gesture_begin: handlers.on_gesture_begin.clone(),
        on_gesture_end: handlers.on_gesture_end.clone(),
    };
    bind_fader_pointer(node, Arc::clone(&live), scalar.clone(), orientation);
    bind_fader_wheel(node, Arc::clone(&live), scalar.clone());
    bind_fader_reset(node, Arc::clone(&live), scalar.clone());
    bind_fader_keys(node, Arc::clone(&live), scalar.clone(), entry_open);
    if entry_open {
        bind_fader_entry(
            node,
            spec,
            live,
            scalar,
            ctx,
            pending == PendingFocus::Entry,
        );
    }
}

fn run_fader(
    live: &Mutex<ScalarRuntime<FaderContext>>,
    event: AudioValueEvent,
    handlers: &ScalarHandlers,
) {
    let current = live.lock().expect("fader machine").machine.clone();
    let (next, effects) = fader_transition(current, event.clone());
    {
        let mut runtime = live.lock().expect("fader machine");
        runtime.machine = next;
        if effects
            .iter()
            .any(|effect| matches!(effect, AudioValueEffect::RequestEntryFocus))
        {
            runtime.pending_focus = PendingFocus::Entry;
            runtime.draft = format_value(runtime.machine.base.value, runtime.machine.base.format);
            runtime.draft_replace = true;
        }
        if matches!(
            event,
            AudioValueEvent::EntryCancel | AudioValueEvent::EntryCommit { .. }
        ) {
            runtime.pending_focus = PendingFocus::Root;
            runtime.draft.clear();
        }
    }
    apply_scalar_effects(&effects, handlers);
}

fn bind_fader_pointer(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<FaderContext>>>,
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

fn bind_fader_wheel(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<FaderContext>>>,
    handlers: ScalarHandlers,
) {
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

fn bind_fader_reset(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<FaderContext>>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_fader(&live, AudioValueEvent::Reset, &handlers);
    }));
}

fn bind_fader_keys(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<FaderContext>>>,
    handlers: ScalarHandlers,
    entry_open: bool,
) {
    let submit_live = Arc::clone(&live);
    let submit_handlers = handlers.clone();
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
    if !entry_open {
        node.interaction.on_submit = Some(Arc::new(move || {
            run_fader(&submit_live, AudioValueEvent::EntryOpen, &submit_handlers);
        }));
    }
}

fn bind_fader_entry(
    node: &mut Node,
    spec: &FaderSpec,
    live: Arc<Mutex<ScalarRuntime<FaderContext>>>,
    handlers: ScalarHandlers,
    ctx: &RenderContext<'_>,
    request_focus: bool,
) {
    let (text, instance_id) = {
        let runtime = live.lock().expect("fader machine");
        let text = if runtime.draft.is_empty() {
            format_value(runtime.machine.base.value, runtime.machine.base.format)
        } else {
            runtime.draft.clone()
        };
        (text, node.id.clone().unwrap_or_default())
    };
    let mut entry = Node::input(text, "");
    entry.id = Some(audio_entry_id(&instance_id));
    entry.interaction.focusable = true;
    entry.a11y.role = Some(NodeRole::TextInput);
    entry.a11y.label = Some(format!("{} value", spec.aria_label));
    entry.a11y.tab_index = Some(0);
    entry.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(rem_to_px(4.5));
    entry.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(rem_to_px(1.5));
    entry.style.focus_ring = Some(entry_focus_ring(ctx));
    entry.interaction.request_focus = request_focus;
    if request_focus {
        live.lock().expect("fader machine").pending_focus = PendingFocus::None;
    }
    let edit_live = Arc::clone(&live);
    entry.interaction.on_text_change = Some(Arc::new(move |value: &str| {
        let mut runtime = edit_live.lock().expect("fader machine");
        runtime.draft = value.to_owned();
        runtime.draft_replace = false;
    }));
    let key_live = Arc::clone(&live);
    entry.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
        let mut runtime = key_live.lock().expect("fader machine");
        let mut draft = std::mem::take(&mut runtime.draft);
        let mut replace = runtime.draft_replace;
        apply_draft_key(&mut draft, &mut replace, key, mods.accel);
        runtime.draft = draft;
        runtime.draft_replace = replace;
    }));
    let insert_live = Arc::clone(&live);
    entry.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
        let mut runtime = insert_live.lock().expect("fader machine");
        let mut draft = std::mem::take(&mut runtime.draft);
        let mut replace = runtime.draft_replace;
        apply_draft_insert(&mut draft, &mut replace, text);
        runtime.draft = draft;
        runtime.draft_replace = replace;
    }));
    let commit_live = Arc::clone(&live);
    let commit_handlers = handlers.clone();
    entry.interaction.on_submit = Some(Arc::new(move || {
        let text = commit_live.lock().expect("fader machine").draft.clone();
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
    let blur_handlers = handlers;
    entry.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused
            || !blur_live
                .lock()
                .expect("fader machine")
                .machine
                .base
                .entry_open
        {
            return;
        }
        let text = blur_live.lock().expect("fader machine").draft.clone();
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
    let instance_id = handlers.instance_id.as_str();
    node.id = Some(audio_root_id(instance_id));
    let live = retain_knob(instance_id, spec);
    let (enabled, value, min, max, value_text, entry_open, pending) = {
        let runtime = live.lock().expect("knob machine");
        let machine = &runtime.machine;
        (
            !machine.base.disabled,
            machine.base.value,
            machine.base.min,
            machine.base.max,
            machine.base.value_text(),
            machine.base.entry_open,
            runtime.pending_focus,
        )
    };
    bind_slider_a11y(
        node,
        &spec.aria_label,
        value,
        min,
        max,
        &value_text,
        None,
        enabled,
        enabled.then(|| audio_focus_ring(ctx)),
    );
    if pending == PendingFocus::Root {
        node.interaction.request_focus = true;
        live.lock().expect("knob machine").pending_focus = PendingFocus::None;
    }
    let scalar = ScalarHandlers {
        on_value_change: handlers.on_value_change.clone(),
        on_value_commit: handlers.on_value_commit.clone(),
        on_gesture_begin: handlers.on_gesture_begin.clone(),
        on_gesture_end: handlers.on_gesture_end.clone(),
    };
    bind_knob_pointer(node, Arc::clone(&live), scalar.clone());
    bind_knob_wheel(node, Arc::clone(&live), scalar.clone());
    bind_knob_reset(node, Arc::clone(&live), scalar.clone());
    bind_knob_keys(node, Arc::clone(&live), scalar.clone(), entry_open);
    if entry_open {
        bind_knob_entry(
            node,
            spec,
            live,
            scalar,
            ctx,
            pending == PendingFocus::Entry,
        );
    }
}

fn run_knob(
    live: &Mutex<ScalarRuntime<KnobContext>>,
    event: AudioValueEvent,
    handlers: &ScalarHandlers,
) {
    let current = live.lock().expect("knob machine").machine.clone();
    let (next, effects) = knob_transition(current, event.clone());
    {
        let mut runtime = live.lock().expect("knob machine");
        runtime.machine = next;
        if effects
            .iter()
            .any(|effect| matches!(effect, AudioValueEffect::RequestEntryFocus))
        {
            runtime.pending_focus = PendingFocus::Entry;
            runtime.draft = format_value(runtime.machine.base.value, runtime.machine.base.format);
            runtime.draft_replace = true;
        }
        if matches!(
            event,
            AudioValueEvent::EntryCancel | AudioValueEvent::EntryCommit { .. }
        ) {
            runtime.pending_focus = PendingFocus::Root;
            runtime.draft.clear();
        }
    }
    apply_scalar_effects(&effects, handlers);
}

fn bind_knob_pointer(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<KnobContext>>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let fine = event.modifiers.shift;
            let mode = live.lock().expect("knob machine").machine.drag_mode;
            match event.phase {
                ContinuousValuePhase::Press => {
                    if mode == KnobDragMode::Vertical {
                        live.lock().expect("knob machine").pointer = 0.0;
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
                        let position = {
                            let mut runtime = live.lock().expect("knob machine");
                            runtime.pointer -= event.delta_y as f64;
                            runtime.pointer
                        };
                        run_knob(
                            &live,
                            AudioValueEvent::DragMove { position, fine },
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

fn bind_knob_wheel(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<KnobContext>>>,
    handlers: ScalarHandlers,
) {
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

fn bind_knob_reset(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<KnobContext>>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_knob(&live, AudioValueEvent::Reset, &handlers);
    }));
}

fn bind_knob_keys(
    node: &mut Node,
    live: Arc<Mutex<ScalarRuntime<KnobContext>>>,
    handlers: ScalarHandlers,
    entry_open: bool,
) {
    let submit_live = Arc::clone(&live);
    let submit_handlers = handlers.clone();
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
    if !entry_open {
        node.interaction.on_submit = Some(Arc::new(move || {
            run_knob(&submit_live, AudioValueEvent::EntryOpen, &submit_handlers);
        }));
    }
}

fn bind_knob_entry(
    node: &mut Node,
    spec: &KnobSpec,
    live: Arc<Mutex<ScalarRuntime<KnobContext>>>,
    handlers: ScalarHandlers,
    ctx: &RenderContext<'_>,
    request_focus: bool,
) {
    let (text, instance_id) = {
        let runtime = live.lock().expect("knob machine");
        let text = if runtime.draft.is_empty() {
            format_value(runtime.machine.base.value, runtime.machine.base.format)
        } else {
            runtime.draft.clone()
        };
        (text, node.id.clone().unwrap_or_default())
    };
    let mut entry = Node::input(text, "");
    entry.id = Some(audio_entry_id(&instance_id));
    entry.interaction.focusable = true;
    entry.a11y.role = Some(NodeRole::TextInput);
    entry.a11y.label = Some(format!("{} value", spec.aria_label));
    entry.a11y.tab_index = Some(0);
    entry.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(rem_to_px(4.5));
    entry.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(rem_to_px(1.5));
    entry.style.focus_ring = Some(entry_focus_ring(ctx));
    entry.interaction.request_focus = request_focus;
    if request_focus {
        live.lock().expect("knob machine").pending_focus = PendingFocus::None;
    }
    let edit_live = Arc::clone(&live);
    entry.interaction.on_text_change = Some(Arc::new(move |value: &str| {
        let mut runtime = edit_live.lock().expect("knob machine");
        runtime.draft = value.to_owned();
        runtime.draft_replace = false;
    }));
    let key_live = Arc::clone(&live);
    entry.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
        let mut runtime = key_live.lock().expect("knob machine");
        let mut draft = std::mem::take(&mut runtime.draft);
        let mut replace = runtime.draft_replace;
        apply_draft_key(&mut draft, &mut replace, key, mods.accel);
        runtime.draft = draft;
        runtime.draft_replace = replace;
    }));
    let insert_live = Arc::clone(&live);
    entry.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
        let mut runtime = insert_live.lock().expect("knob machine");
        let mut draft = std::mem::take(&mut runtime.draft);
        let mut replace = runtime.draft_replace;
        apply_draft_insert(&mut draft, &mut replace, text);
        runtime.draft = draft;
        runtime.draft_replace = replace;
    }));
    let commit_live = Arc::clone(&live);
    let commit_handlers = handlers.clone();
    entry.interaction.on_submit = Some(Arc::new(move || {
        let text = commit_live.lock().expect("knob machine").draft.clone();
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
    entry.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused
            || !blur_live
                .lock()
                .expect("knob machine")
                .machine
                .base
                .entry_open
        {
            return;
        }
        let text = blur_live.lock().expect("knob machine").draft.clone();
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
    let instance_id = handlers.instance_id.as_str();
    node.id = Some(audio_root_id(instance_id));
    let enabled = spec.visual_state.enabled;
    node.a11y.role = Some(NodeRole::Group);
    node.a11y.label = Some(spec.aria_label.clone());
    node.interaction.disabled = !enabled;
    node.interaction.focusable = false;
    node.a11y.tab_index = None;
    node.style.focus_ring = None;
    let live = retain_xy(instance_id, spec);
    bind_xy_pointer(node, Arc::clone(&live), handlers);
    bind_xy_reset(node, Arc::clone(&live), handlers);
    let ring = enabled.then(|| audio_focus_ring(ctx));
    let (x_value, y_value, x_text, y_text) = {
        let machine = live.lock().expect("xy pad machine");
        (
            machine.x,
            machine.y,
            format_value(machine.x, spec.format_x),
            format_value(machine.y, spec.format_y),
        )
    };
    let mut x = Node::container();
    x.id = Some(xy_pad_x_id(instance_id));
    x.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(1.0);
    x.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(1.0);
    bind_slider_a11y(
        &mut x,
        &format!("{} {}", spec.aria_label, spec.x_label),
        x_value,
        spec.min_x,
        spec.max_x,
        &x_text,
        Some("horizontal"),
        enabled,
        ring,
    );
    bind_xy_axis_keys(&mut x, Arc::clone(&live), handlers.clone(), XYPadAxis::X);
    let mut y = Node::container();
    y.id = Some(xy_pad_y_id(instance_id));
    y.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(1.0);
    y.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(1.0);
    bind_slider_a11y(
        &mut y,
        &format!("{} {}", spec.aria_label, spec.y_label),
        y_value,
        spec.min_y,
        spec.max_y,
        &y_text,
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

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_headless::audio::AudioValueLaw;

    struct Theme;
    impl ThemeProvider for Theme {
        fn resolve_color(&self, _: &str) -> poodle_node::ColorValue {
            poodle_node::ColorValue(0.5, 0.5, 0.5, 1.0)
        }
        fn resolve_space(&self, _: &str) -> f32 {
            8.0
        }
        fn resolve_border_width(&self, _: &str) -> f32 {
            1.0
        }
        fn resolve_radius(&self, _: &str) -> f32 {
            4.0
        }
        fn resolve_opacity(&self, _: &str) -> f32 {
            1.0
        }
    }

    #[test]
    fn instance_ids_scope_roots_and_entry() {
        reset_audio_runtime();
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let left = crate::audio::fader_with_handlers(
            &FaderSpec::new(0.2, 0.0, 1.0, AudioValueLaw::Linear),
            &ctx,
            &FaderHandlers::new("left"),
        );
        let right = crate::audio::fader_with_handlers(
            &FaderSpec::new(0.8, 0.0, 1.0, AudioValueLaw::Linear),
            &ctx,
            &FaderHandlers::new("right"),
        );
        assert_eq!(left.id.as_deref(), Some("left"));
        assert_eq!(right.id.as_deref(), Some("right"));
        assert_ne!(left.id, right.id);
    }

    #[test]
    fn host_value_applies_only_when_idle() {
        reset_audio_runtime();
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let spec = FaderSpec::new(0.2, 0.0, 1.0, AudioValueLaw::Linear);
        let handlers = FaderHandlers::new("idle-host");
        let mut node = crate::audio::fader_with_handlers(&spec, &ctx, &handlers);
        (node.interaction.on_continuous_value.as_ref().unwrap())(&NodeContinuousValueEvent {
            phase: ContinuousValuePhase::Press,
            x: 0.5,
            y: 0.5,
            delta_x: 0.0,
            delta_y: 0.0,
            modifiers: poodle_node::NodeModifiers::default(),
        });
        let mut replaced = spec.clone();
        replaced.visual_state.raw_value = 0.9;
        node = crate::audio::fader_with_handlers(&replaced, &ctx, &handlers);
        let during = fader_retained_spec("idle-host", "Level").unwrap();
        assert!(
            (during.visual_state.raw_value - 0.9).abs() > 0.1,
            "host replacement must not wipe an open drag"
        );
        (node.interaction.on_continuous_value.as_ref().unwrap())(&NodeContinuousValueEvent {
            phase: ContinuousValuePhase::Release,
            x: 0.5,
            y: 0.5,
            delta_x: 0.0,
            delta_y: 0.0,
            modifiers: poodle_node::NodeModifiers::default(),
        });
        replaced.visual_state.raw_value = 0.1;
        let _ = crate::audio::fader_with_handlers(&replaced, &ctx, &handlers);
        let idle = fader_retained_spec("idle-host", "Level").unwrap();
        assert!((idle.visual_state.raw_value - 0.1).abs() < 1e-9);
    }

    #[test]
    fn entry_blur_commits_once() {
        reset_audio_runtime();
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let handlers = FaderHandlers::new("blur-host");
        let node = crate::audio::fader_with_handlers(
            &FaderSpec::new(0.2, 0.0, 1.0, AudioValueLaw::Linear),
            &ctx,
            &handlers,
        );
        (node.interaction.on_submit.as_ref().unwrap())();
        let open = crate::audio::fader_with_handlers(
            &fader_retained_spec("blur-host", "Level").unwrap(),
            &ctx,
            &handlers,
        );
        let entry = open
            .find(&|n| n.id.as_deref() == Some("blur-host:entry"))
            .expect("entry");
        (entry.interaction.on_text_change.as_ref().unwrap())("0.4");
        (entry.interaction.on_focus_change.as_ref().unwrap())(false);
        let committed = fader_retained_spec("blur-host", "Level").unwrap();
        assert!(!committed.entry_open);
        assert!((committed.visual_state.raw_value - 0.4).abs() < 1e-9);
        let closed = crate::audio::fader_with_handlers(&committed, &ctx, &handlers);
        assert!(closed
            .find(&|n| n.id.as_deref() == Some("blur-host:entry"))
            .is_none());
    }
}
