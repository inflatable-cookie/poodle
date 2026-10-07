//! Handler-backed Knob, Fader, XYPad, and AudioSwitch.
//!
//! Handler structs expose the contract effects plus a required
//! lifetime-stable `instance_id`. Machine state lives in host-owned
//! `AudioLive` / `XYPadLive` / `AudioSwitchLive` values the adapter passes
//! into each bind.

use std::sync::{Arc, Mutex};

use poodle_headless::audio::{
    audio_switch_transition, drag_number_transition, fader_transition, format_value,
    keyboard_computer_key_down, keyboard_computer_key_up, keyboard_focus_note, keyboard_hit_test,
    keyboard_move_focus, keyboard_press, keyboard_release, keyboard_retarget,
    keyboard_set_disabled, keyboard_set_octave_shift, keyboard_set_range,
    keyboard_velocity_at_point, keyboard_visual_state, knob_point_to_norm, knob_transition,
    move_envelope_point, normalize_envelope_points, remove_envelope_point, switch_visual_state,
    xy_pad_transition, AudioPoint, AudioRect, AudioSwitchContext, AudioSwitchEffect,
    AudioSwitchEvent, AudioValueContext, AudioValueEffect, AudioValueEvent, DragNumberContext,
    EnvelopePoint, FaderContext, FaderOrientation, KeyboardContext, KeyboardEffect, KnobContext,
    KnobDragMode, ModMatrixCell, ModMatrixContext, ValueBound, WaveformContext, WaveformSelection,
    XYPadAxis, XYPadContext, XYPadEffect, XYPadEvent,
};
use poodle_node::{
    ContinuousValuePhase, FocusRing, Node, NodeContinuousValueEvent, NodeKey, NodeModifiers,
    NodeRole, NodeWheelEvent, ScrubAxis, ScrubPhase,
};
use poodle_specs::{
    AudioSwitchSpec, DragNumberFieldSpec, EnvelopeEditorSpec, FaderSpec, KeyboardSpec, KnobSpec,
    ModMatrixGridSpec, Orientation, WaveformDisplaySpec, XYPadSpec,
};

use crate::audio::keyboard_dimensions;
use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PendingFocus {
    None,
    Entry,
    Root,
}

/// Host-owned adapter state for one Fader/Knob instance. The renderer borrows
/// it for the bind; it does not retain a process-wide map.
pub struct AudioLive<C> {
    pub machine: C,
    draft: String,
    draft_replace: bool,
    pointer: f64,
    pending_focus: PendingFocus,
}

pub type FaderLive = AudioLive<FaderContext>;
pub type KnobLive = AudioLive<KnobContext>;
pub type XYPadLive = XYPadContext;

impl FaderLive {
    pub fn from_spec(spec: &FaderSpec) -> Self {
        Self {
            machine: fader_context_from_spec(spec),
            draft: spec.entry_draft.clone(),
            draft_replace: true,
            pointer: 0.0,
            pending_focus: PendingFocus::None,
        }
    }
}

impl KnobLive {
    pub fn from_spec(spec: &KnobSpec) -> Self {
        Self {
            machine: knob_context_from_spec(spec),
            draft: spec.entry_draft.clone(),
            draft_replace: true,
            pointer: spec.pointer_position,
            pending_focus: PendingFocus::None,
        }
    }
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
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
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
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
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
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
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
    live: &Arc<Mutex<FaderLive>>,
) {
    let instance_id = handlers.instance_id.as_str();
    node.id = Some(audio_root_id(instance_id));
    {
        let mut runtime = live.lock().expect("fader machine");
        apply_host_fader(&mut runtime.machine, spec);
    }
    run_fader(
        live,
        AudioValueEvent::SetValue {
            value: spec.visual_state.raw_value,
        },
        &ScalarHandlers::default(),
    );
    let live = Arc::clone(live);
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
    if entry_open {
        node.a11y.tab_index = Some(-1);
    }
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
    live: &Mutex<AudioLive<FaderContext>>,
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
    live: Arc<Mutex<AudioLive<FaderContext>>>,
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
    live: Arc<Mutex<AudioLive<FaderContext>>>,
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
    live: Arc<Mutex<AudioLive<FaderContext>>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_fader(&live, AudioValueEvent::Reset, &handlers);
    }));
}

fn bind_fader_keys(
    node: &mut Node,
    live: Arc<Mutex<AudioLive<FaderContext>>>,
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
    live: Arc<Mutex<AudioLive<FaderContext>>>,
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
    live: &Arc<Mutex<KnobLive>>,
) {
    let instance_id = handlers.instance_id.as_str();
    node.id = Some(audio_root_id(instance_id));
    {
        let mut runtime = live.lock().expect("knob machine");
        apply_host_knob(&mut runtime.machine, spec);
    }
    run_knob(
        live,
        AudioValueEvent::SetValue {
            value: spec.visual_state.raw_value,
        },
        &ScalarHandlers::default(),
    );
    let live = Arc::clone(live);
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
    if entry_open {
        node.a11y.tab_index = Some(-1);
    }
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
    live: &Mutex<AudioLive<KnobContext>>,
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
    live: Arc<Mutex<AudioLive<KnobContext>>>,
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
    live: Arc<Mutex<AudioLive<KnobContext>>>,
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
    live: Arc<Mutex<AudioLive<KnobContext>>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
        run_knob(&live, AudioValueEvent::Reset, &handlers);
    }));
}

fn bind_knob_keys(
    node: &mut Node,
    live: Arc<Mutex<AudioLive<KnobContext>>>,
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
    live: Arc<Mutex<AudioLive<KnobContext>>>,
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
    live: &Arc<Mutex<XYPadLive>>,
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
    {
        let mut machine = live.lock().expect("xy pad machine");
        apply_host_xy(&mut machine, spec);
    }
    run_xy(
        live,
        XYPadEvent::SetValues {
            x: spec.visual_state.raw_x,
            y: spec.visual_state.raw_y,
        },
        handlers,
    );
    let live = Arc::clone(live);
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

// ── DragNumberField ────────────────────────────────────────────────────────

/// Host-owned adapter state for one DragNumberField instance.
pub type DragNumberLive = AudioLive<DragNumberContext>;

impl DragNumberLive {
    pub fn from_spec(spec: &DragNumberFieldSpec) -> Self {
        Self {
            machine: drag_number_context_from_spec(spec),
            draft: spec.entry_draft.clone(),
            draft_replace: true,
            pointer: 0.0,
            pending_focus: PendingFocus::None,
        }
    }
}

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone)]
pub struct DragNumberHandlers {
    pub instance_id: String,
    pub on_value_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_value_commit: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl DragNumberHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
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

pub fn drag_number_context_from_spec(spec: &DragNumberFieldSpec) -> DragNumberContext {
    DragNumberContext {
        base: AudioValueContext {
            value: spec.visual_state.raw_value,
            min: spec.min,
            max: spec.max,
            law: spec.law(),
            default_value: spec.default_value,
            keyboard_step: spec.step,
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
        drag_sensitivity: spec.drag_sensitivity,
    }
}

pub fn drag_number_spec_from_context(
    context: &DragNumberContext,
    aria_label: impl Into<String>,
) -> DragNumberFieldSpec {
    let value = context.base.value;
    let mut spec = DragNumberFieldSpec::new(
        value,
        context.base.min,
        context.base.max,
        context.base.keyboard_step,
        format_value(value, context.base.format),
    );
    spec.visual_state = context.visual_state();
    spec.drag_sensitivity = context.drag_sensitivity;
    spec.default_value = context.base.default_value;
    spec.format = context.base.format;
    spec.entry_open = context.base.entry_open;
    spec.entry_draft = String::new();
    spec.drag_start_value = context.base.drag_start_value;
    spec.drag_start_position = context.base.drag_start_position;
    spec.text = format_value(value, context.base.format);
    spec.aria_label = aria_label.into();
    spec
}

fn apply_host_drag_number(machine: &mut DragNumberContext, spec: &DragNumberFieldSpec) {
    machine.drag_sensitivity = spec.drag_sensitivity;
    machine.base.min = spec.min;
    machine.base.max = spec.max;
    machine.base.law = spec.law();
    machine.base.default_value = spec.default_value;
    machine.base.keyboard_step = spec.step;
    machine.base.format = spec.format;
    machine.base.disabled = !spec.visual_state.enabled;
    machine.base.hover = spec.visual_state.hover;
    machine.base.automation = spec.visual_state.automation;
}

fn run_drag_number(
    live: &Mutex<DragNumberLive>,
    event: AudioValueEvent,
    handlers: &ScalarHandlers,
) {
    let current = live.lock().expect("drag number machine").machine.clone();
    let (next, effects) = drag_number_transition(current, event.clone());
    {
        let mut runtime = live.lock().expect("drag number machine");
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

/// Bind the scalar channels, a11y surface, and pointer/keyboard/entry routes
/// for a handler-backed DragNumberField.
pub fn bind_drag_number(
    node: &mut Node,
    spec: &DragNumberFieldSpec,
    ctx: &RenderContext<'_>,
    handlers: &DragNumberHandlers,
    live: &Arc<Mutex<DragNumberLive>>,
) {
    let instance_id = handlers.instance_id.as_str();
    node.id = Some(audio_root_id(instance_id));
    {
        let mut runtime = live.lock().expect("drag number machine");
        apply_host_drag_number(&mut runtime.machine, spec);
    }
    run_drag_number(
        live,
        AudioValueEvent::SetValue {
            value: spec.visual_state.raw_value,
        },
        &ScalarHandlers::default(),
    );
    let live = Arc::clone(live);
    let (enabled, value, min, max, value_text, entry_open, pending) = {
        let runtime = live.lock().expect("drag number machine");
        let base = &runtime.machine.base;
        (
            !base.disabled,
            base.value,
            base.min,
            base.max,
            base.value_text(),
            base.entry_open,
            runtime.pending_focus,
        )
    };

    node.a11y.role = Some(NodeRole::SpinButton);
    node.a11y.label = Some(spec.aria_label.clone());
    node.a11y.value = Some(value);
    node.a11y.value_text = Some(value_text);
    node.a11y.value_min = Some(min);
    node.a11y.value_max = Some(max);
    node.interaction.disabled = !enabled;
    if enabled {
        node.interaction.focusable = true;
        node.a11y.tab_index = Some(0);
        node.style.focus_ring = Some(audio_focus_ring(ctx));
    } else {
        node.interaction.focusable = false;
        node.a11y.tab_index = None;
        node.style.focus_ring = None;
    }
    if entry_open {
        node.a11y.tab_index = Some(-1);
    }
    if pending == PendingFocus::Root {
        node.interaction.request_focus = true;
        live.lock().expect("drag number machine").pending_focus = PendingFocus::None;
    }

    let scalar = ScalarHandlers {
        on_value_change: handlers.on_value_change.clone(),
        on_value_commit: handlers.on_value_commit.clone(),
        on_gesture_begin: handlers.on_gesture_begin.clone(),
        on_gesture_end: handlers.on_gesture_end.clone(),
    };
    bind_drag_number_pointer(node, Arc::clone(&live), scalar.clone());
    bind_drag_number_keys(node, Arc::clone(&live), scalar.clone(), entry_open);
    bind_drag_number_focus(node, Arc::clone(&live), scalar.clone());
    if entry_open {
        bind_drag_number_entry(
            node,
            spec,
            live,
            scalar,
            ctx,
            pending == PendingFocus::Entry,
        );
    }
}

fn bind_drag_number_pointer(
    node: &mut Node,
    live: Arc<Mutex<DragNumberLive>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let fine = event.modifiers.shift;
            match event.phase {
                ContinuousValuePhase::Press => {
                    // A press may become a click; the drag is announced on the
                    // first move, exactly as the web adapter waits for travel.
                    live.lock().expect("drag number machine").pointer = 0.0;
                }
                ContinuousValuePhase::Move => {
                    let position = {
                        let mut runtime = live.lock().expect("drag number machine");
                        runtime.pointer += event.delta_x as f64;
                        runtime.pointer
                    };
                    let dragging = live.lock().expect("drag number machine").machine.base.drag
                        != poodle_headless::audio::DragState::None;
                    if !dragging {
                        run_drag_number(
                            &live,
                            AudioValueEvent::DragBegin {
                                position: 0.0,
                                fine,
                            },
                            &handlers,
                        );
                    }
                    run_drag_number(
                        &live,
                        AudioValueEvent::DragMove { position, fine },
                        &handlers,
                    );
                }
                ContinuousValuePhase::Release | ContinuousValuePhase::Cancel => {
                    let dragging = live.lock().expect("drag number machine").machine.base.drag
                        != poodle_headless::audio::DragState::None;
                    if dragging {
                        run_drag_number(&live, AudioValueEvent::DragEnd, &handlers);
                    } else if event.phase == ContinuousValuePhase::Release {
                        run_drag_number(&live, AudioValueEvent::EntryOpen, &handlers);
                    }
                }
            }
        }));
}

fn bind_drag_number_keys(
    node: &mut Node,
    live: Arc<Mutex<DragNumberLive>>,
    handlers: ScalarHandlers,
    entry_open: bool,
) {
    let key_live = Arc::clone(&live);
    let key_handlers = handlers.clone();
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
        run_drag_number(&key_live, event, &key_handlers);
        None
    }));
    if !entry_open {
        node.interaction.on_submit = Some(Arc::new(move || {
            run_drag_number(&live, AudioValueEvent::EntryOpen, &handlers);
        }));
    }
}

fn bind_drag_number_focus(
    node: &mut Node,
    live: Arc<Mutex<DragNumberLive>>,
    handlers: ScalarHandlers,
) {
    node.interaction.on_focus_change = Some(Arc::new(move |focused| {
        run_drag_number(&live, AudioValueEvent::Focus { value: focused }, &handlers);
    }));
}

fn bind_drag_number_entry(
    node: &mut Node,
    spec: &DragNumberFieldSpec,
    live: Arc<Mutex<DragNumberLive>>,
    handlers: ScalarHandlers,
    ctx: &RenderContext<'_>,
    request_focus: bool,
) {
    let (text, instance_id) = {
        let runtime = live.lock().expect("drag number machine");
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
        live.lock().expect("drag number machine").pending_focus = PendingFocus::None;
    }
    let edit_live = Arc::clone(&live);
    entry.interaction.on_text_change = Some(Arc::new(move |value: &str| {
        let mut runtime = edit_live.lock().expect("drag number machine");
        runtime.draft = value.to_owned();
        runtime.draft_replace = false;
    }));
    let key_live = Arc::clone(&live);
    entry.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
        let mut runtime = key_live.lock().expect("drag number machine");
        let mut draft = std::mem::take(&mut runtime.draft);
        let mut replace = runtime.draft_replace;
        apply_draft_key(&mut draft, &mut replace, key, mods.accel);
        runtime.draft = draft;
        runtime.draft_replace = replace;
    }));
    let insert_live = Arc::clone(&live);
    entry.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
        let mut runtime = insert_live.lock().expect("drag number machine");
        let mut draft = std::mem::take(&mut runtime.draft);
        let mut replace = runtime.draft_replace;
        apply_draft_insert(&mut draft, &mut replace, text);
        runtime.draft = draft;
        runtime.draft_replace = replace;
    }));
    let commit_live = Arc::clone(&live);
    let commit_handlers = handlers.clone();
    entry.interaction.on_submit = Some(Arc::new(move || {
        let text = commit_live
            .lock()
            .expect("drag number machine")
            .draft
            .clone();
        run_drag_number(
            &commit_live,
            AudioValueEvent::EntryCommit { text },
            &commit_handlers,
        );
    }));
    let cancel_live = Arc::clone(&live);
    entry.interaction.on_cancel = Some(Arc::new(move || {
        run_drag_number(
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
                .expect("drag number machine")
                .machine
                .base
                .entry_open
        {
            return;
        }
        let text = blur_live.lock().expect("drag number machine").draft.clone();
        run_drag_number(
            &blur_live,
            AudioValueEvent::EntryCommit { text },
            &blur_handlers,
        );
    }));
    *node = std::mem::take(node).child(entry);
}

/// Host-owned adapter state for one Keyboard instance.
pub struct KeyboardLive {
    pub machine: KeyboardContext,
}

impl KeyboardLive {
    pub fn from_context(machine: KeyboardContext) -> Self {
        Self { machine }
    }
}

impl Default for KeyboardLive {
    fn default() -> Self {
        Self {
            machine: KeyboardContext::default(),
        }
    }
}

/// Contract note effects plus a required lifetime-stable instance scope.
#[derive(Clone)]
pub struct KeyboardHandlers {
    pub instance_id: String,
    pub on_note_on: Option<Arc<dyn Fn(u8, u8) + Send + Sync>>,
    pub on_note_off: Option<Arc<dyn Fn(u8) + Send + Sync>>,
}

impl KeyboardHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
            on_note_on: None,
            on_note_off: None,
        }
    }

    pub fn on_note_on(mut self, handler: Arc<dyn Fn(u8, u8) + Send + Sync>) -> Self {
        self.on_note_on = Some(handler);
        self
    }

    pub fn on_note_off(mut self, handler: Arc<dyn Fn(u8) + Send + Sync>) -> Self {
        self.on_note_off = Some(handler);
        self
    }
}

pub fn keyboard_key_id(instance_id: &str, note: u8) -> String {
    format!("{instance_id}:note-{note}")
}

pub fn keyboard_visual_id(instance_id: &str, note: u8) -> String {
    format!("{instance_id}:visual-{note}")
}

pub fn keyboard_spec_from_context(context: &KeyboardContext, aria_label: &str) -> KeyboardSpec {
    let mut spec = KeyboardSpec::new(keyboard_visual_state(context));
    spec.aria_label = aria_label.to_owned();
    spec
}

fn apply_keyboard_effects(effects: &[KeyboardEffect], handlers: &KeyboardHandlers) {
    for effect in effects {
        match effect {
            KeyboardEffect::NoteOn { note, velocity } => {
                if let Some(handler) = &handlers.on_note_on {
                    handler(*note, *velocity);
                }
            }
            KeyboardEffect::NoteOff { note } => {
                if let Some(handler) = &handlers.on_note_off {
                    handler(*note);
                }
            }
        }
    }
}

fn is_reserved_computer_key(key: &str) -> bool {
    matches!(
        key,
        "left"
            | "right"
            | "up"
            | "down"
            | "space"
            | "enter"
            | "tab"
            | "escape"
            | "home"
            | "end"
            | "pageup"
            | "pagedown"
    )
}

fn run_keyboard(
    live: &Mutex<KeyboardLive>,
    apply: impl FnOnce(KeyboardContext) -> (KeyboardContext, Vec<KeyboardEffect>),
    handlers: &KeyboardHandlers,
) {
    let current = live.lock().expect("keyboard machine").machine.clone();
    let (next, effects) = apply(current);
    live.lock().expect("keyboard machine").machine = next;
    apply_keyboard_effects(&effects, handlers);
}

fn apply_host_keyboard(
    mut machine: KeyboardContext,
    spec: &KeyboardSpec,
) -> (KeyboardContext, Vec<KeyboardEffect>) {
    let state = &spec.visual_state;
    let mut effects = Vec::new();
    if machine.first_note != state.first_note || machine.last_note != state.last_note {
        let (next, more) = keyboard_set_range(machine, state.first_note, state.last_note);
        machine = next;
        effects.extend(more);
    }
    if machine.octave_shift != state.octave_shift {
        let (next, more) = keyboard_set_octave_shift(machine, state.octave_shift);
        machine = next;
        effects.extend(more);
    }
    let host_disabled = !state.enabled;
    if machine.disabled != host_disabled {
        let (next, more) = keyboard_set_disabled(machine, host_disabled);
        machine = next;
        effects.extend(more);
    }
    machine.orientation = state.orientation;
    machine.key_layout = state.key_layout;
    machine.row_height_px = state.row_height_px;
    machine.scroll_offset_px = state.scroll_offset_px;
    machine.external_held_notes = state.external_held_notes.clone();
    (machine, effects)
}

pub fn bind_keyboard(
    node: &mut Node,
    spec: &KeyboardSpec,
    ctx: &RenderContext<'_>,
    handlers: &KeyboardHandlers,
    live: &Arc<Mutex<KeyboardLive>>,
) {
    node.id = Some(audio_root_id(&handlers.instance_id));
    run_keyboard(live, |machine| apply_host_keyboard(machine, spec), handlers);
    let enabled = {
        let runtime = live.lock().expect("keyboard machine");
        !runtime.machine.disabled
    };
    if !enabled {
        return;
    }
    let size = ctx.resolve_size(spec.size, spec.size_role);
    let (width, height) = keyboard_dimensions(size, spec.visual_state.orientation);
    let major_axis_px =
        if spec.visual_state.orientation == poodle_headless::audio::KeyboardOrientation::Vertical {
            height
        } else {
            width
        };
    bind_keyboard_pointer(node, Arc::clone(live), handlers.clone(), major_axis_px);
    bind_computer_keys(node, Arc::clone(live), handlers.clone());
    let keys = spec.visual_state.keys.clone();
    let mut key_index = 0usize;
    for visual in &mut node.children {
        let Some(control) = visual
            .children
            .iter_mut()
            .find(|child| child.a11y.role == Some(NodeRole::Button))
        else {
            continue;
        };
        let Some(key) = keys.get(key_index) else {
            break;
        };
        visual.id = Some(keyboard_visual_id(&handlers.instance_id, key.note));
        control.id = Some(keyboard_key_id(&handlers.instance_id, key.note));
        bind_key_control(control, key.note, Arc::clone(live), handlers.clone());
        key_index += 1;
    }
}

fn bind_keyboard_pointer(
    node: &mut Node,
    live: Arc<Mutex<KeyboardLive>>,
    handlers: KeyboardHandlers,
    major_axis_px: f32,
) {
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let x_from_left = event.x as f64;
            let y_from_top = 1.0 - event.y as f64;
            let (orientation, hit) = {
                let runtime = live.lock().expect("keyboard machine");
                (
                    runtime.machine.orientation,
                    keyboard_hit_test(
                        &runtime.machine,
                        x_from_left,
                        y_from_top,
                        f64::from(major_axis_px),
                    ),
                )
            };
            let velocity = keyboard_velocity_at_point(orientation, x_from_left, y_from_top);
            match event.phase {
                ContinuousValuePhase::Press => {
                    if let Some(note) = hit {
                        run_keyboard(
                            &live,
                            |context| keyboard_press(context, "pointer", note, velocity),
                            &handlers,
                        );
                    }
                }
                ContinuousValuePhase::Move => {
                    run_keyboard(
                        &live,
                        |context| keyboard_retarget(context, "pointer", hit, velocity),
                        &handlers,
                    );
                }
                ContinuousValuePhase::Release | ContinuousValuePhase::Cancel => {
                    run_keyboard(
                        &live,
                        |context| keyboard_release(context, "pointer"),
                        &handlers,
                    );
                }
            }
        }));
}

fn bind_computer_keys(node: &mut Node, live: Arc<Mutex<KeyboardLive>>, handlers: KeyboardHandlers) {
    let down_live = Arc::clone(&live);
    let down_handlers = handlers.clone();
    node.interaction.on_edit_key = Some(Arc::new(move |key, _mods| {
        if is_reserved_computer_key(key) {
            return;
        }
        run_keyboard(
            &down_live,
            |context| keyboard_computer_key_down(context, key, 100, false),
            &down_handlers,
        );
    }));
    let up_live = live;
    node.interaction.on_key_up = Some(Arc::new(move |key, _mods| {
        if is_reserved_computer_key(key) {
            return;
        }
        run_keyboard(
            &up_live,
            |context| keyboard_computer_key_up(context, key),
            &handlers,
        );
    }));
}

fn bind_key_control(
    node: &mut Node,
    note: u8,
    live: Arc<Mutex<KeyboardLive>>,
    handlers: KeyboardHandlers,
) {
    let focus_live = Arc::clone(&live);
    node.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused {
            let mut runtime = focus_live.lock().expect("keyboard machine");
            runtime.machine = keyboard_focus_note(runtime.machine.clone(), Some(note));
        }
    }));
    let arrow_live = Arc::clone(&live);
    let instance = handlers.instance_id.clone();
    node.interaction.on_key = Some(Arc::new(move |key, _mods| {
        let direction = match key {
            NodeKey::ArrowRight | NodeKey::ArrowUp => 1,
            NodeKey::ArrowLeft | NodeKey::ArrowDown => -1,
            _ => return None,
        };
        let next = {
            let mut runtime = arrow_live.lock().expect("keyboard machine");
            runtime.machine = keyboard_move_focus(runtime.machine.clone(), direction);
            runtime.machine.focused_note
        };
        next.map(|focused| keyboard_key_id(&instance, focused))
    }));
    let press_live = Arc::clone(&live);
    let press_handlers = handlers.clone();
    node.interaction.on_key_activate = Some(Arc::new(move || {
        run_keyboard(
            &press_live,
            |context| keyboard_press(context, format!("a11y:{note}"), note, 100),
            &press_handlers,
        );
        None
    }));
    let down_live = Arc::clone(&live);
    let down_handlers = handlers.clone();
    node.interaction.on_edit_key = Some(Arc::new(move |key, _mods| {
        if is_reserved_computer_key(key) {
            return;
        }
        run_keyboard(
            &down_live,
            |context| keyboard_computer_key_down(context, key, 100, false),
            &down_handlers,
        );
    }));
    let up_live = Arc::clone(&live);
    let up_handlers = handlers;
    node.interaction.on_key_up = Some(Arc::new(move |key, _mods| {
        if matches!(key, "space" | "enter") {
            run_keyboard(
                &up_live,
                |context| keyboard_release(context, &format!("a11y:{note}")),
                &up_handlers,
            );
        }
        if is_reserved_computer_key(key) {
            return;
        }
        run_keyboard(
            &up_live,
            |context| keyboard_computer_key_up(context, key),
            &up_handlers,
        );
    }));
}

/// Host-owned adapter state for one AudioSwitch instance.
pub struct AudioSwitchLive {
    pub machine: AudioSwitchContext,
}

impl AudioSwitchLive {
    pub fn from_spec(spec: &AudioSwitchSpec) -> Self {
        Self {
            machine: audio_switch_context_from_spec(spec),
        }
    }
}

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone)]
pub struct AudioSwitchHandlers {
    pub instance_id: String,
    pub on_state_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_state_commit: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

impl AudioSwitchHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
            on_state_change: None,
            on_state_commit: None,
        }
    }

    pub fn on_state_change(mut self, handler: Arc<dyn Fn(usize) + Send + Sync>) -> Self {
        self.on_state_change = Some(handler);
        self
    }

    pub fn on_state_commit(mut self, handler: Arc<dyn Fn(usize) + Send + Sync>) -> Self {
        self.on_state_commit = Some(handler);
        self
    }
}

pub fn audio_switch_context_from_spec(spec: &AudioSwitchSpec) -> AudioSwitchContext {
    let follow = spec.visual_state.state > 0;
    AudioSwitchContext {
        mode: spec.mode,
        state: spec.visual_state.state,
        state_count: spec.visual_state.state_count,
        lamp_on: if spec.visual_state.lamp_on == follow {
            None
        } else {
            Some(spec.visual_state.lamp_on)
        },
        pressed: spec.visual_state.pressed,
        disabled: !spec.visual_state.enabled,
    }
}

pub fn audio_switch_spec_from_context(
    context: &AudioSwitchContext,
    aria_label: impl Into<String>,
) -> AudioSwitchSpec {
    let mut spec = AudioSwitchSpec::new(
        switch_visual_state(
            context.mode,
            context.state,
            context.state_count,
            context.pressed,
            context.lamp_on,
            !context.disabled,
        ),
        context.mode,
    );
    spec.aria_label = aria_label.into();
    spec
}

fn apply_host_switch(machine: &mut AudioSwitchContext, spec: &AudioSwitchSpec) {
    *machine = audio_switch_context_from_spec(spec);
}

fn run_switch(
    live: &Arc<Mutex<AudioSwitchLive>>,
    event: AudioSwitchEvent,
    handlers: &AudioSwitchHandlers,
) {
    let effects = {
        let mut runtime = live.lock().expect("audio switch machine");
        let (next, effects) = audio_switch_transition(runtime.machine.clone(), event);
        runtime.machine = next;
        effects
    };
    for effect in effects {
        match effect {
            AudioSwitchEffect::StateChange(state) => {
                if let Some(handler) = &handlers.on_state_change {
                    handler(state);
                }
            }
            AudioSwitchEffect::StateCommit(state) => {
                if let Some(handler) = &handlers.on_state_commit {
                    handler(state);
                }
            }
        }
    }
}

pub fn bind_audio_switch(
    node: &mut Node,
    spec: &AudioSwitchSpec,
    ctx: &RenderContext<'_>,
    handlers: &AudioSwitchHandlers,
    live: &Arc<Mutex<AudioSwitchLive>>,
) {
    node.id = Some(handlers.instance_id.clone());
    {
        let mut runtime = live.lock().expect("audio switch machine");
        apply_host_switch(&mut runtime.machine, spec);
    }
    if spec.visual_state.enabled {
        node.interaction.focusable = true;
        node.a11y.tab_index = Some(0);
        node.style.focus_ring = Some(audio_focus_ring(ctx));
        bind_switch_pointer(node, Arc::clone(live), handlers.clone());
        bind_switch_keys(node, Arc::clone(live), handlers.clone());
        bind_switch_blur(node, Arc::clone(live), handlers.clone());
    } else {
        node.interaction.focusable = false;
        node.interaction.disabled = true;
    }
}

fn bind_switch_pointer(
    node: &mut Node,
    live: Arc<Mutex<AudioSwitchLive>>,
    handlers: AudioSwitchHandlers,
) {
    node.interaction.on_continuous_value = Some(Arc::new(
        move |event: &NodeContinuousValueEvent| match event.phase {
            ContinuousValuePhase::Press => {
                run_switch(&live, AudioSwitchEvent::Press, &handlers);
            }
            ContinuousValuePhase::Release => {
                run_switch(&live, AudioSwitchEvent::Release, &handlers);
            }
            ContinuousValuePhase::Cancel => {
                run_switch(&live, AudioSwitchEvent::Cancel, &handlers);
            }
            ContinuousValuePhase::Move => {}
        },
    ));
}

fn bind_switch_keys(
    node: &mut Node,
    live: Arc<Mutex<AudioSwitchLive>>,
    handlers: AudioSwitchHandlers,
) {
    let press_live = Arc::clone(&live);
    let press_handlers = handlers.clone();
    node.interaction.on_key_activate = Some(Arc::new(move || {
        run_switch(&press_live, AudioSwitchEvent::Press, &press_handlers);
        None
    }));
    node.interaction.on_key_up = Some(Arc::new(move |key, _mods| {
        if matches!(key, "space" | "enter") {
            run_switch(&live, AudioSwitchEvent::Release, &handlers);
        }
    }));
}

fn bind_switch_blur(
    node: &mut Node,
    live: Arc<Mutex<AudioSwitchLive>>,
    handlers: AudioSwitchHandlers,
) {
    node.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if !focused {
            run_switch(&live, AudioSwitchEvent::Cancel, &handlers);
        }
    }));
}

// ── EnvelopeEditor ───────────────────────────────────────────────────────
//
// The GPUI adapter owns point hit testing, drag capture, focus, and key
// translation (contract §10); the shared headless helpers own normalized
// ordering, clamping, and curve evaluation. The event model mirrors the
// Svelte `envelopeTransition` authority: DRAG_BEGIN selects the hit point and
// opens the gesture, DRAG_MOVE reports live change through the snap hook,
// DRAG_END commits and closes it, while arrows/PageUp/PageDown nudge the
// selected point, Delete removes it, and atomic edits emit change plus
// commit. `[`/`]` curve keys and Backspace have no `NodeKey` vocabulary, so
// curve editing stays web-only until the backend carries those keys.

/// Host-owned adapter state for one EnvelopeEditor instance.
#[derive(Clone, Debug)]
pub struct EnvelopeLive {
    pub points: Vec<EnvelopePoint>,
    pub selected: Option<String>,
    pub dragging: Option<String>,
    pub step: f64,
    pub curve_step: f64,
    pub disabled: bool,
    pub aria_label: String,
}

impl EnvelopeLive {
    pub fn new(points: Vec<EnvelopePoint>) -> Self {
        Self {
            points: normalize_envelope_points(&points),
            selected: None,
            dragging: None,
            step: 0.01,
            curve_step: 0.1,
            disabled: false,
            aria_label: "Envelope".into(),
        }
    }
}

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone, Default)]
pub struct EnvelopeHandlers {
    pub instance_id: String,
    pub on_points_change: Option<Arc<dyn Fn(Vec<EnvelopePoint>) + Send + Sync>>,
    pub on_points_commit: Option<Arc<dyn Fn(Vec<EnvelopePoint>) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
    pub snap_point: Option<Arc<dyn Fn(f64, f64) -> (f64, f64) + Send + Sync>>,
}

impl EnvelopeHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
            ..Self::default()
        }
    }

    pub fn on_points_change(
        mut self,
        handler: Arc<dyn Fn(Vec<EnvelopePoint>) + Send + Sync>,
    ) -> Self {
        self.on_points_change = Some(handler);
        self
    }

    pub fn on_points_commit(
        mut self,
        handler: Arc<dyn Fn(Vec<EnvelopePoint>) + Send + Sync>,
    ) -> Self {
        self.on_points_commit = Some(handler);
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

    pub fn snap_point(mut self, hook: Arc<dyn Fn(f64, f64) -> (f64, f64) + Send + Sync>) -> Self {
        self.snap_point = Some(hook);
        self
    }
}

/// Rebuild a spec from host-owned envelope state, so a mounted tree refreshes
/// from the committed machine value like the other audio binds.
pub fn envelope_spec_from_live(live: &EnvelopeLive) -> EnvelopeEditorSpec {
    let mut spec = EnvelopeEditorSpec::new(poodle_headless::audio::EnvelopeVisualState {
        points: live
            .points
            .iter()
            .map(|point| poodle_headless::audio::EnvelopeVisualPoint {
                id: point.id.clone(),
                x_norm: point.x,
                y_norm: point.y,
                curve: point.curve,
                selected: live.selected.as_deref() == Some(point.id.as_str()),
                dragging: live.dragging.as_deref() == Some(point.id.as_str()),
            })
            .collect(),
        hover_point_id: None,
        focus: false,
        enabled: !live.disabled,
    });
    spec.aria_label = live.aria_label.clone();
    spec
}

fn envelope_handle_id(instance_id: &str, point_id: &str) -> String {
    format!("{instance_id}:point:{point_id}")
}

/// Port of the Svelte `envelopeHitTest` authority: closest normalized point
/// within a 10px radius of the pointer position.
fn envelope_hit_test(
    points: &[EnvelopePoint],
    x_px: f32,
    y_px: f32,
    width: f32,
    height: f32,
) -> Option<String> {
    let mut closest: Option<String> = None;
    let mut distance = 10.0_f32.max(0.0);
    for point in points {
        let px = point.x as f32 * width;
        let py = (1.0 - point.y as f32) * height;
        let next = ((x_px - px).powi(2) + (y_px - py).powi(2)).sqrt();
        if next <= distance {
            closest = Some(point.id.clone());
            distance = next;
        }
    }
    closest
}

fn envelope_emit_change(live: &Mutex<EnvelopeLive>, handlers: &EnvelopeHandlers) {
    let points = live.lock().expect("envelope machine").points.clone();
    if let Some(emit) = &handlers.on_points_change {
        emit(points);
    }
}

fn envelope_emit_commit(live: &Mutex<EnvelopeLive>, handlers: &EnvelopeHandlers) {
    let points = live.lock().expect("envelope machine").points.clone();
    if let Some(emit) = &handlers.on_points_commit {
        emit(points);
    }
}

fn envelope_gesture_begin(handlers: &EnvelopeHandlers) {
    if let Some(begin) = &handlers.on_gesture_begin {
        begin();
    }
}

fn envelope_gesture_end(handlers: &EnvelopeHandlers) {
    if let Some(end) = &handlers.on_gesture_end {
        end();
    }
}

/// Wire the production envelope tree: root drag capture with real deltas plus
/// per-handle focus selection and keyboard. `width`/`height` are the resolved
/// fixed geometry the renderer painted, so hit testing matches Svelte's
/// bounding-rect math.
pub fn bind_envelope_editor(
    node: &mut Node,
    spec: &EnvelopeEditorSpec,
    ctx: &RenderContext<'_>,
    handlers: &EnvelopeHandlers,
    live: &Arc<Mutex<EnvelopeLive>>,
    width: f32,
    height: f32,
) {
    let instance_id = handlers.instance_id.clone();
    node.id = Some(audio_root_id(&instance_id));
    node.runtime_id = Some(audio_root_id(&instance_id));
    node.a11y.label = Some(spec.aria_label.clone());
    let enabled = !live.lock().expect("envelope machine").disabled;
    node.interaction.disabled = !enabled;
    if enabled {
        node.style.focus_ring = Some(audio_focus_ring(ctx));
    }
    bind_envelope_drag(node, Arc::clone(live), handlers.clone(), width, height);
    let mut handle_index = 0;
    for child in node
        .children
        .iter_mut()
        .filter(|child| child.a11y.role == Some(NodeRole::Slider))
    {
        let points = live.lock().expect("envelope machine").points.clone();
        if let Some(point) = points.get(handle_index) {
            let id = envelope_handle_id(&instance_id, &point.id);
            child.id = Some(id.clone());
            child.runtime_id = Some(id);
            child.interaction.focusable = enabled;
            child.interaction.disabled = !enabled;
            child.a11y.tab_index = enabled.then_some(0);
            if enabled {
                child.style.focus_ring = Some(audio_focus_ring(ctx));
                bind_envelope_handle_focus(child, Arc::clone(live), point.id.clone());
                bind_envelope_handle_keys(
                    child,
                    Arc::clone(live),
                    handlers.clone(),
                    point.id.clone(),
                );
            }
        }
        handle_index += 1;
    }
    debug_assert_eq!(
        handle_index,
        live.lock().expect("envelope machine").points.len(),
        "every normalized point paints exactly one handle"
    );
}

fn bind_envelope_drag(
    node: &mut Node,
    live: Arc<Mutex<EnvelopeLive>>,
    handlers: EnvelopeHandlers,
    width: f32,
    height: f32,
) {
    node.interaction.on_continuous_value =
        Some(Arc::new(move |event: &NodeContinuousValueEvent| {
            let mut runtime = live.lock().expect("envelope machine");
            if runtime.disabled {
                return;
            }
            match event.phase {
                ContinuousValuePhase::Press => {
                    let x_px = event.x * width;
                    let y_px = (1.0 - event.y) * height;
                    if let Some(id) = envelope_hit_test(&runtime.points, x_px, y_px, width, height)
                    {
                        runtime.selected = Some(id.clone());
                        runtime.dragging = Some(id);
                        drop(runtime);
                        envelope_gesture_begin(&handlers);
                    }
                }
                ContinuousValuePhase::Move => {
                    if let Some(id) = runtime.dragging.clone() {
                        let x = (event.x as f64).clamp(0.0, 1.0);
                        // Continuous norms are y-up with 1 at the top, exactly the
                        // envelope's normalized sense, so no flip is needed here
                        // (the from-top pixel hit test above still flips).
                        let y = (event.y as f64).clamp(0.0, 1.0);
                        let (x, y) = handlers
                            .snap_point
                            .as_ref()
                            .map(|snap| snap(x, y))
                            .unwrap_or((x, y));
                        runtime.points = move_envelope_point(&runtime.points, &id, x, y);
                        drop(runtime);
                        envelope_emit_change(&live, &handlers);
                    }
                }
                ContinuousValuePhase::Release | ContinuousValuePhase::Cancel => {
                    if runtime.dragging.take().is_some() {
                        drop(runtime);
                        envelope_emit_commit(&live, &handlers);
                        envelope_gesture_end(&handlers);
                    }
                }
            }
        }));
}

fn bind_envelope_handle_focus(node: &mut Node, live: Arc<Mutex<EnvelopeLive>>, point_id: String) {
    node.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused {
            live.lock().expect("envelope machine").selected = Some(point_id.clone());
        }
    }));
}

fn bind_envelope_handle_keys(
    node: &mut Node,
    live: Arc<Mutex<EnvelopeLive>>,
    handlers: EnvelopeHandlers,
    point_id: String,
) {
    node.interaction.on_key = Some(Arc::new(move |key, mods: NodeModifiers| {
        let mut runtime = live.lock().expect("envelope machine");
        if runtime.disabled {
            return None;
        }
        runtime.selected = Some(point_id.clone());
        if key == NodeKey::Delete {
            if let Some(selected) = runtime.selected.clone() {
                runtime.points = remove_envelope_point(&runtime.points, &selected);
                runtime.selected = None;
                runtime.dragging = None;
                drop(runtime);
                envelope_emit_change(&live, &handlers);
                envelope_emit_commit(&live, &handlers);
            }
            return None;
        }
        let Some((direction, multiplier)) = audio_nudge(key) else {
            return None;
        };
        let Some(selected) = runtime.selected.clone() else {
            return None;
        };
        let current = runtime
            .points
            .iter()
            .find(|point| point.id == selected)?
            .clone();
        let delta =
            direction as f64 * runtime.step * multiplier * if mods.shift { 0.1 } else { 1.0 };
        let vertical = matches!(
            key,
            NodeKey::ArrowUp | NodeKey::ArrowDown | NodeKey::PageUp | NodeKey::PageDown
        );
        let (x, y) = if vertical {
            (current.x, current.y + delta)
        } else {
            (current.x + delta, current.y)
        };
        runtime.points = move_envelope_point(&runtime.points, &selected, x, y);
        drop(runtime);
        envelope_emit_change(&live, &handlers);
        envelope_emit_commit(&live, &handlers);
        None
    }));
}

// ── WaveformDisplay ──────────────────────────────────────────────────────
//
// The adapter owns viewport hit positions, focus, and key translation
// (contract §10); the Rust core owns pyramid validation, level choice, and
// the 4,096-column reduction. The event model mirrors the Svelte
// `waveformTransition` authority: press opens an ordered selection and
// reports cursor plus selection change, moves extend it live, release commits
// it, arrows move the cursor (Shift extends), Home/End target viewport
// bounds, and Escape clears.

/// Host-owned adapter state for one WaveformDisplay instance: the shared
/// waveform context itself, whose fields the transition owns.
pub type WaveformLive = WaveformContext;

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone, Default)]
pub struct WaveformHandlers {
    pub instance_id: String,
    pub on_cursor_change: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    pub on_selection_change: Option<Arc<dyn Fn(Option<WaveformSelection>) + Send + Sync>>,
    pub on_selection_commit: Option<Arc<dyn Fn(Option<WaveformSelection>) + Send + Sync>>,
}

impl WaveformHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
            ..Self::default()
        }
    }

    pub fn on_cursor_change(mut self, handler: Arc<dyn Fn(usize) + Send + Sync>) -> Self {
        self.on_cursor_change = Some(handler);
        self
    }

    pub fn on_selection_change(
        mut self,
        handler: Arc<dyn Fn(Option<WaveformSelection>) + Send + Sync>,
    ) -> Self {
        self.on_selection_change = Some(handler);
        self
    }

    pub fn on_selection_commit(
        mut self,
        handler: Arc<dyn Fn(Option<WaveformSelection>) + Send + Sync>,
    ) -> Self {
        self.on_selection_commit = Some(handler);
        self
    }
}

/// Rebuild a spec from host-owned waveform state, so a mounted tree refreshes
/// from the committed machine value like the other audio binds.
pub fn waveform_spec_from_context(
    context: &WaveformContext,
    aria_label: &str,
) -> WaveformDisplaySpec {
    let mut spec = WaveformDisplaySpec::new(context.visual_state());
    spec.aria_label = aria_label.to_owned();
    spec
}

/// Port of the Svelte `waveformPointToSample` authority: the scrub fraction
/// resolves to a clamped viewport sample.
fn waveform_sample_at(fraction: f32, visible_start: usize, visible_end: usize) -> usize {
    let span = visible_end.saturating_sub(visible_start).max(1);
    let sample = visible_start + (fraction.clamp(0.0, 1.0) as f64 * span as f64).floor() as usize;
    sample.clamp(
        visible_start,
        visible_end.saturating_sub(1).max(visible_start),
    )
}

fn waveform_clamp_sample(sample: usize, visible_start: usize, visible_end: usize) -> usize {
    sample.clamp(
        visible_start,
        visible_end.saturating_sub(1).max(visible_start),
    )
}

fn waveform_ordered(anchor: usize, sample: usize) -> WaveformSelection {
    WaveformSelection {
        start: anchor.min(sample),
        end: anchor.max(sample),
    }
}

fn waveform_emit_cursor(handlers: &WaveformHandlers, sample: usize) {
    if let Some(emit) = &handlers.on_cursor_change {
        emit(sample);
    }
}

fn waveform_emit_selection(handlers: &WaveformHandlers, selection: Option<WaveformSelection>) {
    if let Some(emit) = &handlers.on_selection_change {
        emit(selection);
    }
}

fn waveform_emit_selection_commit(
    handlers: &WaveformHandlers,
    selection: Option<WaveformSelection>,
) {
    if let Some(emit) = &handlers.on_selection_commit {
        emit(selection);
    }
}

/// Move the cursor by a signed delta, extending the selection from the anchor
/// when asked. Ports `MOVE_CURSOR`: a plain move reports cursor change only,
/// while an extending move also reports selection change and commits the
/// atomic keyboard edit.
fn waveform_move_cursor(
    live: &Mutex<WaveformLive>,
    handlers: &WaveformHandlers,
    delta: isize,
    extend: bool,
) {
    let mut context = live.lock().expect("waveform machine");
    if context.disabled {
        return;
    }
    let origin = context.cursor_sample.unwrap_or(context.visible_start);
    let sample = waveform_clamp_sample(
        origin.saturating_add_signed(delta),
        context.visible_start,
        context.visible_end,
    );
    context.cursor_sample = Some(sample);
    if !extend {
        context.selection_anchor = None;
        drop(context);
        waveform_emit_cursor(handlers, sample);
        return;
    }
    let anchor = context.selection_anchor.unwrap_or(origin);
    let selection = waveform_ordered(anchor, sample);
    context.selection = Some(selection);
    context.selection_anchor = Some(anchor);
    drop(context);
    waveform_emit_cursor(handlers, sample);
    waveform_emit_selection(handlers, Some(selection));
    waveform_emit_selection_commit(handlers, Some(selection));
}

/// Wire the production waveform tree: scrub selection with real pointer
/// fractions plus cursor/selection keyboard over the shared context.
pub fn bind_waveform_display(
    node: &mut Node,
    _spec: &WaveformDisplaySpec,
    ctx: &RenderContext<'_>,
    handlers: &WaveformHandlers,
    live: &Arc<Mutex<WaveformLive>>,
) {
    // The summary label, value range, and value text come from the base
    // render and refresh on every rebuild; the bind only stamps identity.
    let instance_id = handlers.instance_id.clone();
    node.id = Some(audio_root_id(&instance_id));
    node.runtime_id = Some(audio_root_id(&instance_id));
    let enabled = !live.lock().expect("waveform machine").disabled;
    node.interaction.focusable = enabled;
    node.interaction.disabled = !enabled;
    node.a11y.tab_index = enabled.then_some(0);
    if enabled {
        node.style.focus_ring = Some(audio_focus_ring(ctx));
        bind_waveform_scrub(node, Arc::clone(live), handlers.clone());
        bind_waveform_keys(node, Arc::clone(live), handlers.clone());
        let focus_live = Arc::clone(live);
        node.interaction.on_focus_change = Some(Arc::new(move |focused| {
            focus_live.lock().expect("waveform machine").focus = focused;
        }));
    } else {
        node.style.focus_ring = None;
    }
}

fn bind_waveform_scrub(
    node: &mut Node,
    live: Arc<Mutex<WaveformLive>>,
    handlers: WaveformHandlers,
) {
    node.interaction.scrub_axis = ScrubAxis::Horizontal;
    node.interaction.on_scrub = Some(Arc::new(move |fraction: f32, phase: ScrubPhase| {
        let mut context = live.lock().expect("waveform machine");
        if context.disabled {
            return;
        }
        match phase {
            ScrubPhase::Press => {
                let sample =
                    waveform_sample_at(fraction, context.visible_start, context.visible_end);
                context.cursor_sample = Some(sample);
                context.selection = Some(WaveformSelection {
                    start: sample,
                    end: sample,
                });
                context.selection_anchor = Some(sample);
                context.selecting = true;
                let selection = context.selection;
                drop(context);
                waveform_emit_cursor(&handlers, sample);
                waveform_emit_selection(&handlers, selection);
            }
            ScrubPhase::Drag => {
                if !context.selecting {
                    return;
                }
                let anchor = context.selection_anchor;
                let sample =
                    waveform_sample_at(fraction, context.visible_start, context.visible_end);
                if let Some(anchor) = anchor {
                    let selection = waveform_ordered(anchor, sample);
                    context.cursor_sample = Some(sample);
                    context.selection = Some(selection);
                    drop(context);
                    waveform_emit_cursor(&handlers, sample);
                    waveform_emit_selection(&handlers, Some(selection));
                }
            }
            ScrubPhase::Release => {
                if !context.selecting {
                    return;
                }
                context.selecting = false;
                context.selection_anchor = None;
                let selection = context.selection;
                drop(context);
                waveform_emit_selection_commit(&handlers, selection);
            }
        }
    }));
}

fn bind_waveform_keys(node: &mut Node, live: Arc<Mutex<WaveformLive>>, handlers: WaveformHandlers) {
    node.interaction.on_key = Some(Arc::new(move |key, mods: NodeModifiers| {
        let extend = mods.shift;
        match key {
            NodeKey::ArrowLeft => waveform_move_cursor(&live, &handlers, -1, extend),
            NodeKey::ArrowRight => waveform_move_cursor(&live, &handlers, 1, extend),
            NodeKey::Home => {
                let (start, cursor) = {
                    let context = live.lock().expect("waveform machine");
                    (
                        context.visible_start,
                        context.cursor_sample.unwrap_or(context.visible_start),
                    )
                };
                waveform_move_cursor(&live, &handlers, start as isize - cursor as isize, extend);
            }
            NodeKey::End => {
                let (end, cursor) = {
                    let context = live.lock().expect("waveform machine");
                    (
                        context
                            .visible_end
                            .saturating_sub(1)
                            .max(context.visible_start),
                        context.cursor_sample.unwrap_or(context.visible_start),
                    )
                };
                waveform_move_cursor(&live, &handlers, end as isize - cursor as isize, extend);
            }
            NodeKey::Escape => {
                let mut context = live.lock().expect("waveform machine");
                if context.disabled {
                    return None;
                }
                context.selection = None;
                context.selection_anchor = None;
                drop(context);
                waveform_emit_selection(&handlers, None);
                waveform_emit_selection_commit(&handlers, None);
            }
            _ => {}
        }
        None
    }));
}

// ── ModMatrixGrid ────────────────────────────────────────────────────────
//
// The adapter owns grid navigation, enablement, caller IDs, and parameter
// mapping (contract §7); the shared Rust machine owns ID normalization,
// per-cell laws, and the embedded slider value mapping. The event model
// mirrors the Svelte `modMatrixTransition` authority: pointer activation
// focuses a cell, horizontal scrub drags run the embedded slider machine
// with live change and a single release commit bracketed by gesture
// begin/end, Space toggles, arrows move, Home/End target row bounds
// (Control pairs target grid bounds), and PageUp/PageDown nudge by the
// focused cell's step.

/// Host-owned adapter state for one ModMatrixGrid instance.
#[derive(Clone, Debug)]
pub struct ModMatrixLive {
    pub machine: ModMatrixContext,
    pub dragging: bool,
}

impl ModMatrixLive {
    pub fn new(machine: ModMatrixContext) -> Self {
        Self {
            machine,
            dragging: false,
        }
    }
}

/// Contract effects plus a required lifetime-stable instance scope.
#[derive(Clone, Default)]
pub struct ModMatrixHandlers {
    pub instance_id: String,
    pub on_cell_change: Option<Arc<dyn Fn(ModMatrixCell) + Send + Sync>>,
    pub on_cell_commit: Option<Arc<dyn Fn(ModMatrixCell) + Send + Sync>>,
    pub on_gesture_begin: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_gesture_end: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl ModMatrixHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.is_empty(),
            "native audio instance_id must be non-empty and lifetime-stable"
        );
        Self {
            instance_id,
            ..Self::default()
        }
    }

    pub fn on_cell_change(mut self, handler: Arc<dyn Fn(ModMatrixCell) + Send + Sync>) -> Self {
        self.on_cell_change = Some(handler);
        self
    }

    pub fn on_cell_commit(mut self, handler: Arc<dyn Fn(ModMatrixCell) + Send + Sync>) -> Self {
        self.on_cell_commit = Some(handler);
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

/// Rebuild a spec from host-owned matrix state, so a mounted tree refreshes
/// from the committed machine value like the other audio binds.
pub fn mod_matrix_spec_from_context(
    context: &ModMatrixContext,
    aria_label: &str,
) -> ModMatrixGridSpec {
    let mut spec = ModMatrixGridSpec::new(context.visual_state());
    spec.aria_label = aria_label.to_owned();
    spec
}

fn mod_matrix_cell_id(instance_id: &str, source_id: &str, destination_id: &str) -> String {
    format!("{instance_id}:cell:{source_id}:{destination_id}")
}

fn mod_matrix_row_id(instance_id: &str, source_id: &str) -> String {
    format!("{instance_id}:row:{source_id}")
}

fn mod_matrix_focused_cell_id(
    live: &Mutex<ModMatrixLive>,
    handlers: &ModMatrixHandlers,
) -> Option<String> {
    let runtime = live.lock().expect("mod matrix machine");
    let (row, column) = runtime
        .machine
        .focus_row
        .zip(runtime.machine.focus_column)?;
    let destinations = runtime.machine.destinations.len();
    let cell = runtime.machine.cells.get(row * destinations + column)?;
    Some(mod_matrix_cell_id(
        &handlers.instance_id,
        &cell.source_id,
        &cell.destination_id,
    ))
}

fn mod_matrix_emit_commit(live: &Mutex<ModMatrixLive>, handlers: &ModMatrixHandlers) {
    mod_matrix_emit_with(live, handlers, true);
}

fn mod_matrix_emit_with(live: &Mutex<ModMatrixLive>, handlers: &ModMatrixHandlers, commit: bool) {
    let cell = {
        let runtime = live.lock().expect("mod matrix machine");
        runtime
            .machine
            .focus_row
            .zip(runtime.machine.focus_column)
            .and_then(|(row, column)| {
                runtime
                    .machine
                    .cells
                    .get(row * runtime.machine.destinations.len() + column)
                    .cloned()
            })
    };
    if let (Some(cell), Some(emit)) = (
        cell,
        if commit {
            &handlers.on_cell_commit
        } else {
            &handlers.on_cell_change
        },
    ) {
        emit(cell);
    }
}

/// Focus one cell with the Svelte `boundedFocus` clamping: empty axes clear
/// the focus instead of pointing outside the grid.
fn mod_matrix_focus_cell(live: &Mutex<ModMatrixLive>, row: usize, column: usize) {
    let mut runtime = live.lock().expect("mod matrix machine");
    if runtime.machine.disabled
        || runtime.machine.sources.is_empty()
        || runtime.machine.destinations.is_empty()
    {
        return;
    }
    runtime.machine.focus_row = Some(row.min(runtime.machine.sources.len() - 1));
    runtime.machine.focus_column = Some(column.min(runtime.machine.destinations.len() - 1));
}

/// Wire the production matrix tree: per-cell activation focus, horizontal
/// scrub drags through the embedded slider machine, and the full grid
/// keyboard map with roving focus targets.
pub fn bind_mod_matrix_grid(
    node: &mut Node,
    spec: &ModMatrixGridSpec,
    ctx: &RenderContext<'_>,
    handlers: &ModMatrixHandlers,
    live: &Arc<Mutex<ModMatrixLive>>,
) {
    let instance_id = handlers.instance_id.clone();
    node.id = Some(audio_root_id(&instance_id));
    node.runtime_id = Some(audio_root_id(&instance_id));
    node.a11y.label = Some(spec.aria_label.clone());
    let enabled = !live.lock().expect("mod matrix machine").machine.disabled;
    node.interaction.disabled = !enabled;
    let (focus_row, focus_column) = {
        let runtime = live.lock().expect("mod matrix machine");
        (runtime.machine.focus_row, runtime.machine.focus_column)
    };
    let mut rows = node.children.iter_mut();
    let _header = rows.next();
    for (row_index, row) in rows.enumerate() {
        let source_id = live
            .lock()
            .expect("mod matrix machine")
            .machine
            .sources
            .get(row_index)
            .map(|source| source.id.clone())
            .unwrap_or_default();
        row.id = Some(mod_matrix_row_id(&instance_id, &source_id));
        row.runtime_id = Some(mod_matrix_row_id(&instance_id, &source_id));
        let mut cells = row.children.iter_mut();
        let _label = cells.next();
        for (column_index, cell) in cells.enumerate() {
            let destination_id = live
                .lock()
                .expect("mod matrix machine")
                .machine
                .destinations
                .get(column_index)
                .map(|destination| destination.id.clone())
                .unwrap_or_default();
            let id = mod_matrix_cell_id(&instance_id, &source_id, &destination_id);
            cell.id = Some(id.clone());
            cell.runtime_id = Some(id);
            cell.interaction.focusable = enabled;
            cell.interaction.disabled = !enabled;
            let focused = focus_row == Some(row_index) && focus_column == Some(column_index);
            let first_unfocused = focus_row.is_none() && row_index == 0 && column_index == 0;
            cell.a11y.tab_index =
                enabled.then_some(if focused || first_unfocused { 0 } else { -1 });
            if enabled {
                cell.style.focus_ring = Some(audio_focus_ring(ctx));
                bind_mod_matrix_cell_focus(cell, Arc::clone(live), row_index, column_index);
                bind_mod_matrix_cell_pointer(
                    cell,
                    Arc::clone(live),
                    handlers.clone(),
                    row_index,
                    column_index,
                );
                bind_mod_matrix_cell_keys(
                    cell,
                    Arc::clone(live),
                    handlers.clone(),
                    row_index,
                    column_index,
                );
            }
        }
    }
}

fn bind_mod_matrix_cell_focus(
    node: &mut Node,
    live: Arc<Mutex<ModMatrixLive>>,
    row: usize,
    column: usize,
) {
    node.interaction.on_focus_change = Some(Arc::new(move |focused| {
        if focused {
            mod_matrix_focus_cell(&live, row, column);
        }
    }));
}

fn bind_mod_matrix_cell_pointer(
    node: &mut Node,
    live: Arc<Mutex<ModMatrixLive>>,
    handlers: ModMatrixHandlers,
    row: usize,
    column: usize,
) {
    let focus_live = Arc::clone(&live);
    node.interaction.on_activate = Some(Arc::new(move || {
        mod_matrix_focus_cell(&focus_live, row, column);
    }));
    node.interaction.scrub_axis = ScrubAxis::Horizontal;
    node.interaction.on_scrub = Some(Arc::new(move |fraction: f32, phase: ScrubPhase| {
        let mut runtime = live.lock().expect("mod matrix machine");
        if runtime.machine.disabled {
            return;
        }
        match phase {
            ScrubPhase::Press => {
                if runtime.machine.sources.is_empty() || runtime.machine.destinations.is_empty() {
                    return;
                }
                runtime.machine.focus_row = Some(row.min(runtime.machine.sources.len() - 1));
                runtime.machine.focus_column =
                    Some(column.min(runtime.machine.destinations.len() - 1));
                runtime.machine =
                    std::mem::replace(&mut runtime.machine, mod_matrix_empty_context())
                        .set_normalized(fraction as f64);
                runtime.dragging = true;
                drop(runtime);
                mod_matrix_emit_with(&live, &handlers, false);
                if let Some(begin) = &handlers.on_gesture_begin {
                    begin();
                }
            }
            ScrubPhase::Drag => {
                if !runtime.dragging {
                    return;
                }
                runtime.machine =
                    std::mem::replace(&mut runtime.machine, mod_matrix_empty_context())
                        .set_normalized(fraction as f64);
                drop(runtime);
                mod_matrix_emit_with(&live, &handlers, false);
            }
            ScrubPhase::Release => {
                if !runtime.dragging {
                    return;
                }
                runtime.dragging = false;
                drop(runtime);
                mod_matrix_emit_with(&live, &handlers, true);
                if let Some(end) = &handlers.on_gesture_end {
                    end();
                }
            }
        }
    }));
}

/// Placeholder a live matrix context never observes: `set_normalized` takes
/// `self` by value, so the bind swaps the real machine out, runs it, and
/// swaps it back. The empty stand-in is immediately replaced and only exists
/// to satisfy the borrow checker.
fn mod_matrix_empty_context() -> ModMatrixContext {
    ModMatrixContext::new(vec![], vec![], vec![])
}

fn bind_mod_matrix_cell_keys(
    node: &mut Node,
    live: Arc<Mutex<ModMatrixLive>>,
    handlers: ModMatrixHandlers,
    _row: usize,
    _column: usize,
) {
    node.interaction.on_key = Some(Arc::new(move |key, mods: NodeModifiers| {
        let mut runtime = live.lock().expect("mod matrix machine");
        if runtime.machine.disabled {
            return None;
        }
        match key {
            NodeKey::ArrowUp | NodeKey::ArrowDown | NodeKey::ArrowLeft | NodeKey::ArrowRight => {
                let (rows, columns) = match key {
                    NodeKey::ArrowDown => (1, 0),
                    NodeKey::ArrowUp => (-1, 0),
                    NodeKey::ArrowRight => (0, 1),
                    _ => (0, -1),
                };
                runtime.machine =
                    std::mem::replace(&mut runtime.machine, mod_matrix_empty_context())
                        .move_focus(rows, columns);
                drop(runtime);
                mod_matrix_focused_cell_id(&live, &handlers)
            }
            NodeKey::Home | NodeKey::End => {
                let sources = runtime.machine.sources.len();
                let destinations = runtime.machine.destinations.len();
                if sources == 0 || destinations == 0 {
                    return None;
                }
                let grid = mods.accel;
                let at_start = key == NodeKey::Home;
                let (row, column) = if grid {
                    (
                        if at_start { 0 } else { sources - 1 },
                        if at_start { 0 } else { destinations - 1 },
                    )
                } else if at_start {
                    (runtime.machine.focus_row.unwrap_or(0).min(sources - 1), 0)
                } else {
                    (
                        runtime.machine.focus_row.unwrap_or(0).min(sources - 1),
                        destinations - 1,
                    )
                };
                runtime.machine.focus_row = Some(row);
                runtime.machine.focus_column = Some(column);
                drop(runtime);
                mod_matrix_focused_cell_id(&live, &handlers)
            }
            NodeKey::Space => {
                runtime.machine =
                    std::mem::replace(&mut runtime.machine, mod_matrix_empty_context()).toggle();
                drop(runtime);
                mod_matrix_emit_with(&live, &handlers, false);
                mod_matrix_emit_commit(&live, &handlers);
                None
            }
            NodeKey::PageUp | NodeKey::PageDown => {
                let direction = if key == NodeKey::PageUp { 1.0 } else { -1.0 };
                runtime.machine =
                    std::mem::replace(&mut runtime.machine, mod_matrix_empty_context())
                        .nudge(direction, mods.shift);
                drop(runtime);
                mod_matrix_emit_with(&live, &handlers, false);
                mod_matrix_emit_commit(&live, &handlers);
                None
            }
            _ => None,
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_headless::audio::{AudioValueLaw, DragState};

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
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let left_spec = FaderSpec::new(0.2, 0.0, 1.0, AudioValueLaw::Linear);
        let right_spec = FaderSpec::new(0.8, 0.0, 1.0, AudioValueLaw::Linear);
        let left_live = Arc::new(Mutex::new(FaderLive::from_spec(&left_spec)));
        let right_live = Arc::new(Mutex::new(FaderLive::from_spec(&right_spec)));
        let left = crate::audio::fader_with_handlers(
            &left_spec,
            &ctx,
            &FaderHandlers::new("left"),
            &left_live,
        );
        let right = crate::audio::fader_with_handlers(
            &right_spec,
            &ctx,
            &FaderHandlers::new("right"),
            &right_live,
        );
        assert_eq!(left.id.as_deref(), Some("left"));
        assert_eq!(right.id.as_deref(), Some("right"));
        assert_ne!(left.id, right.id);
    }

    #[test]
    fn host_replacement_applies_during_a_gesture() {
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let spec = FaderSpec::new(0.2, 0.0, 1.0, AudioValueLaw::Linear);
        let live = Arc::new(Mutex::new(FaderLive::from_spec(&spec)));
        let handlers = FaderHandlers::new("host-replace");
        let mut node = crate::audio::fader_with_handlers(&spec, &ctx, &handlers, &live);
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
        node = crate::audio::fader_with_handlers(&replaced, &ctx, &handlers, &live);
        assert!(
            (live.lock().expect("fader machine").machine.base.value - 0.9).abs() < 1e-9,
            "SetValue stays live during a gesture"
        );
        assert_ne!(
            live.lock().expect("fader machine").machine.base.drag,
            DragState::None
        );
        (node.interaction.on_continuous_value.as_ref().unwrap())(&NodeContinuousValueEvent {
            phase: ContinuousValuePhase::Release,
            x: 0.5,
            y: 0.5,
            delta_x: 0.0,
            delta_y: 0.0,
            modifiers: poodle_node::NodeModifiers::default(),
        });
    }

    #[test]
    #[should_panic(expected = "non-empty")]
    fn empty_instance_id_is_rejected() {
        let _ = FaderHandlers::new("");
    }
}
