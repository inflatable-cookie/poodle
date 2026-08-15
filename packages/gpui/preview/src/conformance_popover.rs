//! Popover GPUI conformance adapter (g14.005), headless (g14.023).
//!
//! Thin layer over the generic headless driver: fixture → PopoverSpec, the
//! shared render composition, the Rust machine mirror for the transition
//! logic, and observe_tree_with_context for the overlay observations. All
//! actions go through the real event tree — the trigger click binds through
//! the node backend, Escape and outside pointers route through the mount
//! host's window-level dispatch into the backend layer registry.

use std::sync::{Arc, Mutex};

use poodle_gpui::GpuiThemeProvider;
use poodle_headless::popover::{
    popover_transition, PopoverContext, PopoverEffect, PopoverEvent, PopoverInitialFocus,
    PopoverState,
};
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, observe_tree_with_context, InterfaceDoc, ObserveContext,
};
use poodle_render::popover::{popover, PopoverHandlers};
use poodle_specs::PopoverSpec;
use serde_json::{json, Value};

use super::conformance_button::CaseOutcome;
use super::conformance_driver::HeadlessDriver;
use super::conformance_support::{
    popover_content_from_fixture, popover_spec_from_fixture, popover_trigger_from_fixture,
};

/// One popover instance's host state: the effective open state, the event
/// trace, and the effects the machine asked the host to execute (focus entry
/// and focus restoration are real backend focus operations the driver runs).
struct InstanceState {
    open: Arc<Mutex<bool>>,
    trace: Arc<Mutex<Vec<Value>>>,
    pending: Arc<Mutex<Vec<PopoverEffect>>>,
}

struct CaseHost {
    spec: PopoverSpec,
    node: Arc<Mutex<Node>>,
    instance: Arc<Mutex<InstanceState>>,
    instance_id: String,
    theme: GpuiThemeProvider,
    trigger_text: String,
    content_text: String,
    focusables: Vec<String>,
    nested: Option<(String, String)>,
    /// The nested popover's instance state, persistent across rebuilds (its
    /// own machine transitions must survive the composition's rebuilds).
    nested_state: Option<Arc<InstanceState>>,
}

impl CaseHost {
    fn machine_context(&self) -> PopoverContext {
        PopoverContext {
            disabled: self.spec.disabled,
            dismiss_on_outside_interact: self.spec.dismiss_on_outside_interact,
            initial_focus: match self.spec.initial_focus {
                poodle_specs::PopoverInitialFocus::Content => PopoverInitialFocus::Content,
                poodle_specs::PopoverInitialFocus::None => PopoverInitialFocus::None,
                poodle_specs::PopoverInitialFocus::FirstFocusable => {
                    PopoverInitialFocus::FirstFocusable
                }
            },
        }
    }

    fn build_node(&self) -> Node {
        let instance = self.instance.lock().expect("instance lock");
        let open = *instance.open.lock().expect("open lock");
        let mut spec = self.spec.clone();
        spec.open = Some(open);
        let content = {
            let mut content = Node::container();
            if let Some(node) = popover_content_from_fixture(
                &serde_json::json!({
                    "regions": { "children": self.content_text },
                    "host": { "focusables": self.focusables },
                }),
                &self.instance_id,
            ) {
                content = content.child(node);
            }            if let (Some((trigger, children)), Some(nested_state)) =
                (&self.nested, &self.nested_state)
            {
                // The nested popover opens when the outer's surface mounts
                // (defaultOpen), registering its layer on the stack.
                let nested_instance = format!("{}-nested", self.instance_id);
                let nested_context = PopoverContext {
                    disabled: false,
                    dismiss_on_outside_interact: true,
                    initial_focus: PopoverInitialFocus::FirstFocusable,
                };
                let on_activate = {
                    let state = Arc::clone(&nested_state);
                    Arc::new(move || {
                        let (_, effects) = popover_transition(
                            PopoverState::Open,
                            nested_context,
                            PopoverEvent::Toggle,
                        );
                        for effect in effects {
                            match effect {
                                PopoverEffect::EmitOpenChange { open } => {
                                    *state.open.lock().expect("open lock") = open;
                                    state.trace.lock().expect("trace lock").push(json!({
                                        "event": "openChange",
                                        "payload": { "open": open },
                                    }));
                                }
                                other => state.pending.lock().expect("pending lock").push(other),
                            }
                        }
                    })
                };
                let on_dismiss = {
                    let state = Arc::clone(&nested_state);
                    Arc::new(move |reason| {
                        let (_, effects) = popover_transition(
                            PopoverState::Open,
                            nested_context,
                            match reason {
                                poodle_node::DismissReason::Escape => PopoverEvent::Escape,
                                poodle_node::DismissReason::Outside => PopoverEvent::OutsideInteract,
                            },
                        );
                        for effect in effects {
                            match effect {
                                PopoverEffect::EmitOpenChange { open } => {
                                    *state.open.lock().expect("open lock") = open;
                                    state.trace.lock().expect("trace lock").push(json!({
                                        "event": "openChange",
                                        "payload": { "open": open },
                                    }));
                                }
                                other => state.pending.lock().expect("pending lock").push(other),
                            }
                        }
                    })
                };
                let nested_open = *nested_state.open.lock().expect("open lock");
                let nested_node = popover(
                    &PopoverSpec::new().with_open(nested_open),
                    &self.theme,
                    &PopoverHandlers {
                        on_activate: Some(on_activate),
                        on_dismiss: Some(on_dismiss),
                        instance_id: Some(nested_instance.clone()),
                    },
                    Some(Node::text(trigger.clone())),
                    popover_content_from_fixture(
                        &serde_json::json!({ "regions": { "children": children.clone() } }),
                        &nested_instance,
                    ),
                );
                content = content.child(nested_node);
            }
            stamp_focus_tracking(&mut content, &self.theme);
            content
        };
        let on_activate = {
            // The handler needs the host's state; capture the instance and
            // context up front (the host outlives the composition).
            let instance = Arc::clone(&self.instance);
            let context = self.machine_context();
            Arc::new(move || {
                let (open, trace, pending) = {
                    let instance = instance.lock().expect("instance lock");
                    (
                        Arc::clone(&instance.open),
                        Arc::clone(&instance.trace),
                        Arc::clone(&instance.pending),
                    )
                };
                let (_, effects) = popover_transition(
                    if *open.lock().expect("open lock") {
                        PopoverState::Open
                    } else {
                        PopoverState::Closed
                    },
                    context,
                    PopoverEvent::Toggle,
                );
                for effect in effects {
                    match effect {
                        PopoverEffect::EmitOpenChange { open: next_open } => {
                            *open.lock().expect("open lock") = next_open;
                            trace.lock().expect("trace lock").push(json!({
                                "event": "openChange",
                                "payload": { "open": next_open },
                            }));
                        }
                        other => pending.lock().expect("pending lock").push(other),
                    }
                }
            })
        };
        let on_dismiss = {
            let instance = Arc::clone(&self.instance);
            let context = self.machine_context();
            Arc::new(move |reason| {
                let (open, trace, pending) = {
                    let instance = instance.lock().expect("instance lock");
                    (
                        Arc::clone(&instance.open),
                        Arc::clone(&instance.trace),
                        Arc::clone(&instance.pending),
                    )
                };
                let (_, effects) = popover_transition(
                    if *open.lock().expect("open lock") {
                        PopoverState::Open
                    } else {
                        PopoverState::Closed
                    },
                    context,
                    match reason {
                        poodle_node::DismissReason::Escape => PopoverEvent::Escape,
                        poodle_node::DismissReason::Outside => PopoverEvent::OutsideInteract,
                    },
                );
                for effect in effects {
                    match effect {
                        PopoverEffect::EmitOpenChange { open: next_open } => {
                            *open.lock().expect("open lock") = next_open;
                            trace.lock().expect("trace lock").push(json!({
                                "event": "openChange",
                                "payload": { "open": next_open },
                            }));
                        }
                        other => pending.lock().expect("pending lock").push(other),
                    }
                }
            })
        };
        popover(
            &spec,
            &self.theme,
            &PopoverHandlers {
                on_activate: Some(on_activate),
                on_dismiss: Some(on_dismiss),
                instance_id: Some(self.instance_id.clone()),
            },
            popover_trigger_from_fixture(&serde_json::json!({
                "regions": { "trigger": self.trigger_text.clone() },
            })),
            Some(content),
        )
    }

    fn drain_pending(&self) -> Vec<PopoverEffect> {
        std::mem::take(
            &mut *self
                .instance
                .lock()
                .expect("instance lock")
                .pending
                .lock()
                .expect("pending lock"),
        )
    }
}

/// Stamp a focus patch on every focusable content node that lacks one, so
/// the real backend focus path can target it (the focus-entry strategy's
/// first-focusable target is a content-owned node).
fn stamp_focus_tracking(node: &mut Node, theme: &GpuiThemeProvider) {
    if node.interaction.focusable && node.style.focus.is_none() {
        node.style.focus = Some(poodle_node::StylePatch {
            border_color: Some(theme.resolve_color_value("color.accent.focusRing")),
            ..poodle_node::StylePatch::default()
        });
    }
    for child in &mut node.children {
        stamp_focus_tracking(child, theme);
    }
}

fn element_id(instance_id: &str, part: &str) -> String {
    format!("{instance_id}:{part}")
}

/// Execute the machine's focus effects through the shared overlay-host
/// focus queue: the target element's paint-time focus canvas applies the
/// request after the frame that mounts it. Same production path the preview
/// host uses.
fn apply_focus_effects(
    instance_id: &str,
    node: &Node,
    effects: Vec<PopoverEffect>,
) {
    for effect in effects {
        match effect {
            PopoverEffect::RestoreTriggerFocus => {
                poodle_gpui_node_backend::request_focus(&element_id(
                    instance_id,
                    "popover-trigger",
                ));
            }
            PopoverEffect::FocusOnOpen { strategy } => {
                let surface = node.find(&|n| n.id.as_deref() == Some("popover-surface"));
                match strategy {
                    PopoverInitialFocus::Content => {
                        if let Some(surface) = surface {
                            let id = surface
                                .runtime_id
                                .clone()
                                .or_else(|| surface.id.clone())
                                .unwrap_or_default();
                            if !id.is_empty() {
                                poodle_gpui_node_backend::request_focus(&id);
                            }
                        }
                    }
                    PopoverInitialFocus::FirstFocusable => {
                        let target = surface
                            .and_then(|surface| surface.find(&|n| n.interaction.focusable));
                        if let Some(target) = target {
                            let id = target
                                .runtime_id
                                .clone()
                                .or_else(|| target.id.clone())
                                .unwrap_or_default();
                            if !id.is_empty() {
                                poodle_gpui_node_backend::request_focus(&id);
                            }
                        }
                    }
                    PopoverInitialFocus::None => {}
                }
            }
            // EmitOpenChange is applied by the transition runner itself.
            PopoverEffect::EmitOpenChange { .. } => {}
        }
    }
}

fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
    let bounds_by_id = |id: &str| {
        poodle_gpui_node_backend::bounds_for(id).map(|bounds| {
            (
                f32::from(bounds.origin.y),
                f32::from(bounds.origin.x),
                f32::from(bounds.size.width),
                f32::from(bounds.size.height),
            )
        })
    };
    let layer_count = poodle_gpui_node_backend::open_layer_count();
    let mut observation = observe_tree_with_context(
        "gpui",
        "popover",
        iface,
        &node,
        &ObserveContext {
            focus_by_id: &focus_by_id,
            layer_count: &(|| Some(layer_count)),
            bounds_by_id: &bounds_by_id,
        },
    );
    observation["trace"] = json!(
        host.instance
            .lock()
            .expect("instance lock")
            .trace
            .lock()
            .expect("trace lock")
            .clone()
    );
    observation
}

/// A pointer position inside `outer` but outside every bounds union in
/// `inner` — the nested outside-interaction proof's target.
fn point_inside_outside(outer: [f32; 4], inner: &[[f32; 4]]) -> gpui::Point<gpui::Pixels> {
    let [top, left, width, height] = outer;
    let candidates = [
        (left + 4.0, top + 4.0),
        (left + 4.0, top + height - 4.0),
        (left + width - 4.0, top + 4.0),
        (left + width - 4.0, top + height - 4.0),
    ];
    let inside_any = |point: (f32, f32)| {
        inner.iter().any(|[it, il, iw, ih]| {
            point.0 >= *il
                && point.0 <= il + iw
                && point.1 >= *it
                && point.1 <= it + ih
        })
    };
    let (x, y) = candidates
        .into_iter()
        .find(|candidate| !inside_any(*candidate))
        .unwrap_or((left + 4.0, top + 4.0));
    gpui::point(gpui::px(x), gpui::px(y))
}

fn bounds_of(instance_id: &str, part: &str) -> Option<[f32; 4]> {
    let full_part = match part {
        "trigger" => "popover-trigger",
        "surface" => "popover-surface",
        other => other,
    };
    poodle_gpui_node_backend::bounds_for(&element_id(instance_id, full_part)).map(|bounds| {
        [
            f32::from(bounds.origin.y),
            f32::from(bounds.origin.x),
            f32::from(bounds.size.width),
            f32::from(bounds.size.height),
        ]
    })
}

pub fn drive_popover_cases(
    driver: &mut HeadlessDriver<'_>,
    iface: InterfaceDoc,
    cases: Vec<Value>,
    only: Option<String>,
) -> Vec<CaseOutcome> {
    let mut outcomes = Vec::new();
    for case in &cases {
        let case_id = case.get("id").and_then(Value::as_str).unwrap_or("?").to_owned();
        if let Some(only) = &only {
            if only != &case_id {
                continue;
            }
        }
        let fixture = case.get("fixture").cloned().unwrap_or_else(|| json!({}));
        let steps = case
            .get("steps")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let spec = popover_spec_from_fixture(&fixture);
        let initial_open = spec.current_open();
        let instance_id = format!("conformance-{case_id}");
        let trigger_text = fixture
            .pointer("/regions/trigger")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let content_text = fixture
            .pointer("/regions/children")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let nested = fixture
            .pointer("/host/nested")
            .and_then(Value::as_object)
            .map(|nested| {
                (
                    nested
                        .get("trigger")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_owned(),
                    nested
                        .get("children")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_owned(),
                )
            });
        let focusables = fixture
            .pointer("/host/focusables")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let node = Arc::new(Mutex::new(Node::container()));
        let outer_trace = Arc::new(Mutex::new(Vec::new()));
        let host = Arc::new(Mutex::new(CaseHost {
            spec,
            node: Arc::clone(&node),
            instance: Arc::new(Mutex::new(InstanceState {
                open: Arc::new(Mutex::new(initial_open)),
                trace: Arc::clone(&outer_trace),
                pending: Arc::new(Mutex::new(Vec::new())),
            })),
            instance_id: instance_id.clone(),
            theme: GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE),
            trigger_text,
            content_text,
            focusables,
            nested_state: nested.as_ref().map(|_| {
                Arc::new(InstanceState {
                    open: Arc::new(Mutex::new(true)),
                    trace: Arc::clone(&outer_trace),
                    pending: Arc::new(Mutex::new(Vec::new())),
                })
            }),
            nested,
        }));
        {
            let host = host.lock().expect("host lock");
            let built = host.build_node();
            *host.node.lock().expect("node lock") = built;
        }
        driver.mount_node(Arc::clone(&node));
        driver.draw_frame();
        driver.blur_element_focus(&element_id(&instance_id, "popover-trigger"));
        driver.wait_for_focus_handle(&element_id(&instance_id, "popover-trigger"));

        let mount_observation = observe_case(&host.lock().expect("host lock"), &iface);

        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = vec![mount_observation];

        for (index, step) in steps.iter().enumerate() {
            let kind = step.get("kind").and_then(Value::as_str).unwrap_or("");
            match kind {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let instance_id = host.lock().expect("host lock").instance_id.clone();
                    match name {
                        "press" => {
                            let input = step
                                .get("input")
                                .and_then(Value::as_str)
                                .unwrap_or("pointer");
                            let disabled = host
                                .lock()
                                .expect("host lock")
                                .spec
                                .disabled;
                            if input == "keyboard" {
                                if disabled {
                                    // A disabled trigger never receives
                                    // focus; the key goes nowhere.
                                    driver.dispatch_key_raw("enter");
                                } else {
                                    driver.keyboard_activate(&element_id(
                                        &instance_id,
                                        "popover-trigger",
                                    ));
                                }
                            } else {
                                // Click-to-focus is the browser default; the
                                // harness performs it, like the web adapters —
                                // except on a disabled trigger.
                                if !disabled {
                                    driver.focus_element(&element_id(
                                        &instance_id,
                                        "popover-trigger",
                                    ));
                                }
                                driver.pointer_activate();
                            }
                            rebuild_and_apply(driver, &host);
                        }
                        "dismiss" => {
                            driver.dispatch_key("escape");
                            rebuild_and_apply(driver, &host);
                        }
                        "pointer" => {
                            let target = step
                                .get("target")
                                .and_then(Value::as_str)
                                .unwrap_or("inside");
                            if target == "outside" {
                                // A real outside pointer press: the window's
                                // top-left corner, outside the mount box.
                                driver.pointer_press(gpui::point(gpui::px(2.0), gpui::px(2.0)));
                            } else if let Some(bounds) = bounds_of(&instance_id, part) {
                                let inner = [
                                    bounds_of(
                                        &format!("{instance_id}-nested"),
                                        "popover-trigger",
                                    ),
                                    bounds_of(
                                        &format!("{instance_id}-nested"),
                                        "popover-surface",
                                    ),
                                ]
                                .into_iter()
                                .flatten()
                                .collect::<Vec<_>>();
                                let position = point_inside_outside(bounds, &inner);
                                driver.pointer_press(position);
                            }
                            rebuild_and_apply(driver, &host);
                        }
                        "focus" => {
                            driver.focus_element(&element_id(&instance_id, part));
                        }
                        _ => {}
                    }
                    let observation = observe_case(&host.lock().expect("host lock"), &iface);
                    observations.push(observation);
                }
                "expectPart" => {
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                    let observation = observe_case(&host.lock().expect("host lock"), &iface);
                    let mut results = Vec::new();
                    assert_part(&iface, part, &expect, index, observation, "gpui", &mut results);
                    for result in results {
                        let value = serde_json::to_value(&result).expect("result serializes");
                        if result.verdict == "fail" {
                            failures.push(value.clone());
                        }
                        assertions.push(value);
                    }
                }
                "expectEvents" => {
                    let expected = step
                        .get("events")
                        .and_then(Value::as_array)
                        .map(|events| {
                            events
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_owned)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let actual: Vec<String> = host
                        .lock()
                        .expect("host lock")
                        .instance
                        .lock()
                        .expect("instance lock")
                        .trace
                        .lock()
                        .expect("trace lock")
                        .iter()
                        .filter_map(|entry| {
                            entry
                                .get("event")
                                .and_then(Value::as_str)
                                .map(str::to_owned)
                        })
                        .collect();
                    let mut results = Vec::new();
                    assert_events(&expected, &actual, index, &mut results);
                    for result in results {
                        let value = serde_json::to_value(&result).expect("result serializes");
                        if result.verdict == "fail" {
                            failures.push(value.clone());
                        }
                        assertions.push(value);
                    }
                }
                _ => {}
            }
        }

        driver.drain();
        outcomes.push(CaseOutcome {
            case_id,
            pass: failures.is_empty(),
            failures,
            assertions,
            observations,
        });
    }
    outcomes
}

/// Rebuild the composition from the host state after a real interaction,
/// then execute the machine's pending focus effects (focus entry / restore)
/// through the real backend focus API — for the outer popover and the nested
/// instance alike.
fn rebuild_and_apply(driver: &mut HeadlessDriver<'_>, host: &Mutex<CaseHost>) {
    let host_guard = host.lock().expect("host lock");
    let built = host_guard.build_node();
    let pending = host_guard.drain_pending();
    let nested_pending = host_guard
        .nested_state
        .as_ref()
        .map(|state| std::mem::take(&mut *state.pending.lock().expect("pending lock")))
        .unwrap_or_default();
    *host_guard.node.lock().expect("node lock") = built;
    driver.draw_frame();
    let node = host_guard.node.lock().expect("node lock").clone();
    apply_focus_effects(&host_guard.instance_id, &node, pending);
    apply_focus_effects(
        &format!("{}-nested", host_guard.instance_id),
        &node,
        nested_pending,
    );
    driver.draw_frame();
}

pub fn popover_report(component: &str, outcomes: &[CaseOutcome]) -> Value {
    json!({
        "runtime": "gpui",
        "component": component,
        "results": outcomes.iter().map(|o| json!({
            "caseId": o.case_id,
            "pass": o.pass,
            "failures": o.failures,
            "assertions": o.assertions,
            "observations": o.observations,
        })).collect::<Vec<_>>(),
    })
}

pub fn registry_has_popover() -> bool {
    #[path = "component_registry.rs"]
    #[allow(dead_code)]
    mod component_registry;
    component_registry::find_component("popover").is_some()
}
