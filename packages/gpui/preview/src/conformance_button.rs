//! Button conformance adapter (spec 066, g14.001).
//!
//! Thin layer over the generic driver: fixture → ButtonSpec, case iteration,
//! and observe_tree with the Button interface document.

use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, host_activate, observe_tree, InterfaceDoc,
};
use poodle_specs::ButtonSpec;
use serde_json::{json, Value};

use super::conformance_driver::{
    blur_element_focus, drain_event_queue, focus_element, keyboard_activate, mount_node,
    pointer_activate, wait_for_focus_handle, warmup_and_calibrate,
};

/// The stable element id the mounted button node carries.
pub const BUTTON_ELEMENT_ID: &str = "conformance-button";

/// One case's host state, shared with the activation handler the backend binds.
struct CaseHost {
    spec: ButtonSpec,
    node: Arc<Mutex<Node>>,
    pressed: Arc<Mutex<Option<bool>>>,
    trace: Arc<Mutex<Vec<Value>>>,
    theme: GpuiThemeProvider,
}

impl CaseHost {
    fn make_handler(
        pressed: Arc<Mutex<Option<bool>>>,
        trace: Arc<Mutex<Vec<Value>>>,
        toggle_mode: bool,
    ) -> Arc<dyn Fn() + Send + Sync> {
        Arc::new(move || {
            host_activate(
                toggle_mode,
                &mut *pressed.lock().expect("pressed lock"),
                &mut *trace.lock().expect("trace lock"),
            );
        })
    }

    fn rebuild(&mut self, handler: Arc<dyn Fn() + Send + Sync>, toggle_mode: bool) {
        if toggle_mode {
            self.spec.pressed = *self.pressed.lock().expect("pressed lock");
        }
        let mut node = poodle_render::button(&self.spec, &self.theme, Some(handler));
        node.id = Some(BUTTON_ELEMENT_ID.to_owned());
        *self.node.lock().expect("node lock") = node;
    }

    fn initial_node(&self, handler: Arc<dyn Fn() + Send + Sync>) -> Node {
        let mut node = poodle_render::button(&self.spec, &self.theme, Some(handler));
        node.id = Some(BUTTON_ELEMENT_ID.to_owned());
        node
    }
}

fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let backend_focus = poodle_gpui_node_backend::focus_state_for(BUTTON_ELEMENT_ID);
    let mut observation = observe_tree("gpui", "button", iface, &node, backend_focus);
    observation["trace"] = json!(host.trace.lock().expect("trace lock").clone());
    observation
}

pub struct CaseOutcome {
    pub case_id: String,
    pub pass: bool,
    pub failures: Vec<Value>,
    pub assertions: Vec<Value>,
    pub observations: Vec<Value>,
}

pub async fn drive_button_cases(
    cx: &mut AsyncWindowContext,
    iface: InterfaceDoc,
    cases: Vec<Value>,
    only: Option<String>,
    spec_from_fixture: impl Fn(&Value) -> ButtonSpec,
) -> Vec<CaseOutcome> {
    let calibration = warmup_and_calibrate(cx).await;

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

        let spec = spec_from_fixture(&fixture);
        let toggle_mode = spec.is_toggle_mode();
        let pressed = Arc::new(Mutex::new(spec.pressed));
        let trace = Arc::new(Mutex::new(Vec::<Value>::new()));
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let node = Arc::new(Mutex::new(Node::container()));
        let host = Arc::new(Mutex::new(CaseHost {
            spec,
            node: Arc::clone(&node),
            pressed: Arc::clone(&pressed),
            trace: Arc::clone(&trace),
            theme,
        }));

        {
            let host = host.lock().expect("host lock");
            let handler = CaseHost::make_handler(Arc::clone(&pressed), Arc::clone(&trace), toggle_mode);
            let initial = host.initial_node(handler);
            *host.node.lock().expect("node lock") = initial;
        }
        mount_node(cx, Arc::clone(&node));
        blur_element_focus(cx, BUTTON_ELEMENT_ID).await;
        wait_for_focus_handle(cx, BUTTON_ELEMENT_ID).await;

        let mount_observation = cx
            .update(|_window, _cx| {
                let host = host.lock().expect("host lock");
                observe_case(&host, &iface)
            })
            .unwrap_or_else(|_| json!({}));

        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = vec![mount_observation];

        for (index, step) in steps.iter().enumerate() {
            let kind = step.get("kind").and_then(Value::as_str).unwrap_or("");
            match kind {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let input = step.get("input").and_then(Value::as_str).unwrap_or("pointer");
                    if name == "press" {
                        if input == "keyboard" {
                            keyboard_activate(cx, BUTTON_ELEMENT_ID).await;
                        } else {
                            let before = trace.lock().expect("trace lock").len();
                            for _click_pass in 0..3 {
                                pointer_activate(cx, calibration).await;
                                if trace.lock().expect("trace lock").len() > before {
                                    break;
                                }
                            }
                        }
                        if toggle_mode {
                            cx.update(|window, _cx| {
                                let mut host = host.lock().expect("host lock");
                                let handler = CaseHost::make_handler(
                                    Arc::clone(&pressed),
                                    Arc::clone(&trace),
                                    toggle_mode,
                                );
                                host.rebuild(handler, toggle_mode);
                                window.refresh();
                            })
                            .ok();
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(150))
                                .await;
                        }
                        let action_observation = cx
                            .update(|_window, _cx| {
                                let host = host.lock().expect("host lock");
                                observe_case(&host, &iface)
                            })
                            .unwrap_or_else(|_| json!({}));
                        observations.push(action_observation);
                    } else if name == "focus" {
                        focus_element(cx, BUTTON_ELEMENT_ID).await;
                        let focus_observation = cx
                            .update(|_window, _cx| {
                                let host = host.lock().expect("host lock");
                                observe_case(&host, &iface)
                            })
                            .unwrap_or_else(|_| json!({}));
                        observations.push(focus_observation);
                    }
                }
                "expectPart" => {
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                    let observation = cx
                        .update(|_window, _cx| {
                            let host = host.lock().expect("host lock");
                            observe_case(&host, &iface)
                        })
                        .unwrap_or_else(|_| json!({}));
                    let mut results = Vec::new();
                    assert_part(&iface, part, &expect, index, observation, "gpui", &mut results);
                    for r in &results {
                        assertions.push(serde_json::to_value(r).expect("result serializes"));
                        if r.verdict == "fail" {
                            failures.push(serde_json::to_value(r).expect("result serializes"));
                        }
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
                    let actual: Vec<String> = trace
                        .lock()
                        .expect("trace lock")
                        .iter()
                        .filter_map(|entry| entry.get("event").and_then(Value::as_str).map(str::to_owned))
                        .collect();
                    let mut results = Vec::new();
                    assert_events(&expected, &actual, index, &mut results);
                    for r in &results {
                        assertions.push(serde_json::to_value(r).expect("result serializes"));
                        if r.verdict == "fail" {
                            failures.push(serde_json::to_value(r).expect("result serializes"));
                        }
                    }
                }
                _ => {}
            }
        }

        drain_event_queue(cx).await;

        outcomes.push(CaseOutcome {
            pass: failures.is_empty(),
            failures,
            assertions,
            observations,
            case_id,
        });
    }

    outcomes
}

pub fn button_report(component: &str, outcomes: &[CaseOutcome]) -> Value {
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

pub fn registry_has_button() -> bool {
    #[path = "component_registry.rs"]
    #[allow(dead_code)]
    mod component_registry;
    component_registry::find_component("button").is_some()
}
