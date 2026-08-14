//! Tabs GPUI conformance adapter (g14.004).

use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, observe_tree_with_focus, InterfaceDoc,
};
use poodle_render::{tabs_with_panel, TabsHandlers};
use poodle_specs::TabsSpec;
use serde_json::{json, Value};

use super::conformance_button::CaseOutcome;
use super::conformance_driver::{
    blur_element_focus, drain_event_queue, focus_element, keyboard_activate, keyboard_key,
    mount_node, wait_for_focus_handle, KEY_DOWN, KEY_END, KEY_HOME, KEY_LEFT, KEY_RIGHT, KEY_UP,
};
use super::conformance_support::tabs_spec_from_fixture;

struct CaseHost {
    spec: TabsSpec,
    node: Arc<Mutex<Node>>,
    value: Arc<Mutex<String>>,
    focused: Arc<Mutex<String>>,
    trace: Arc<Mutex<Vec<Value>>>,
    panel_text: String,
    theme: GpuiThemeProvider,
}

impl CaseHost {
    fn rebuild(&mut self) {
        let value = self.value.lock().expect("value lock").clone();
        let focused = self.focused.lock().expect("focus lock").clone();
        self.spec.value = Some(value.clone());
        let trace = Arc::clone(&self.trace);
        let live_value = Arc::clone(&self.value);
        let on_change = Arc::new(move |next: &str| {
            *live_value.lock().expect("value lock") = next.to_owned();
            trace.lock().expect("trace lock").push(json!({
                "event": "valueChange",
                "payload": { "value": next },
            }));
        });
        let live_focus = Arc::clone(&self.focused);
        let on_focus = Arc::new(move |next: &str| {
            *live_focus.lock().expect("focus lock") = next.to_owned();
        });
        let panel = Node::text(format!("{} · {value}", self.panel_text));
        let node = tabs_with_panel(
            &self.spec,
            &self.theme,
            TabsHandlers {
                on_change: Some(on_change),
                on_focus: Some(on_focus),
                focused_value: Some(focused),
                ..TabsHandlers::default()
            },
            panel,
        );
        *self.node.lock().expect("node lock") = node;
    }
}

fn part_value(part: &str) -> Option<&str> {
    part.strip_prefix("trigger:")
}

fn element_id(part: &str) -> String {
    part_value(part)
        .map(|value| format!("tabs:{value}"))
        .unwrap_or_else(|| "tabs-list".to_owned())
}

fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
    let mut observation = observe_tree_with_focus("gpui", "tabs", iface, &node, &focus_by_id);
    observation["trace"] = json!(host.trace.lock().expect("trace lock").clone());
    observation
}

fn keycode(key: &str) -> Option<u16> {
    Some(match key {
        "ArrowRight" => KEY_RIGHT,
        "ArrowLeft" => KEY_LEFT,
        "ArrowDown" => KEY_DOWN,
        "ArrowUp" => KEY_UP,
        "Home" => KEY_HOME,
        "End" => KEY_END,
        _ => return None,
    })
}

async fn rebuild_and_focus(cx: &mut AsyncWindowContext, host: &Arc<Mutex<CaseHost>>) {
    let target = cx
        .update(|window, _cx| {
            let mut host = host.lock().expect("host lock");
            host.rebuild();
            let target = format!("tabs:{}", host.focused.lock().expect("focus lock"));
            window.refresh();
            target
        })
        .unwrap_or_default();
    wait_for_focus_handle(cx, &target).await;
    focus_element(cx, &target).await;
}

pub async fn drive_tabs_cases(
    cx: &mut AsyncWindowContext,
    iface: InterfaceDoc,
    cases: Vec<Value>,
    only: Option<String>,
) -> Vec<CaseOutcome> {
    let mut outcomes = Vec::new();
    for case in &cases {
        let case_id = case
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("?")
            .to_owned();
        if only.as_deref().is_some_and(|only| only != case_id.as_str()) {
            continue;
        }
        let fixture = case.get("fixture").cloned().unwrap_or_else(|| json!({}));
        let steps = case
            .get("steps")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let spec = tabs_spec_from_fixture(&fixture);
        let initial = spec.current_value().unwrap_or_default().to_owned();
        let node = Arc::new(Mutex::new(Node::container()));
        let host = Arc::new(Mutex::new(CaseHost {
            spec,
            node: Arc::clone(&node),
            value: Arc::new(Mutex::new(initial.clone())),
            focused: Arc::new(Mutex::new(initial.clone())),
            trace: Arc::new(Mutex::new(Vec::new())),
            panel_text: fixture
                .get("regions")
                .and_then(|regions| regions.get("panel"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned(),
            theme: GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE),
        }));
        host.lock().expect("host lock").rebuild();
        mount_node(cx, Arc::clone(&node));
        let initial_id = format!("tabs:{initial}");
        blur_element_focus(cx, &initial_id).await;
        // A prior case can briefly repaint the new semantic ids while its old
        // focus handle is still active. Reset the host's semantic focus after
        // the real backend blur so the new case starts from its fixture.
        {
            let mut host = host.lock().expect("host lock");
            *host.focused.lock().expect("focus lock") = initial.clone();
            host.rebuild();
        }
        cx.update(|window, _cx| window.refresh()).ok();
        wait_for_focus_handle(cx, &initial_id).await;

        let mount_observation = cx
            .update(|_window, _cx| observe_case(&host.lock().expect("host lock"), &iface))
            .unwrap_or_else(|_| json!({}));
        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = vec![mount_observation];

        for (index, step) in steps.iter().enumerate() {
            match step.get("kind").and_then(Value::as_str).unwrap_or("") {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let id = element_id(part);
                    match name {
                        "focus" => {
                            if let Some(value) = part_value(part) {
                                let host = host.lock().expect("host lock");
                                *host.focused.lock().expect("focus lock") = value.to_owned();
                            }
                            rebuild_and_focus(cx, &host).await;
                        }
                        "press" => {
                            let disabled = part_value(part).is_some_and(|value| {
                                host.lock()
                                    .expect("host lock")
                                    .spec
                                    .tabs
                                    .iter()
                                    .any(|tab| tab.value == value && tab.is_disabled)
                            });
                            keyboard_activate(cx, &id).await;
                            if !disabled {
                                rebuild_and_focus(cx, &host).await;
                            }
                        }
                        "key" => {
                            let key = step.get("key").and_then(Value::as_str).unwrap_or("");
                            if key == "Enter" {
                                keyboard_activate(cx, &id).await;
                            } else if let Some(code) = keycode(key) {
                                keyboard_key(cx, &id, code).await;
                            }
                            rebuild_and_focus(cx, &host).await;
                        }
                        _ => {}
                    }
                    let observation = cx
                        .update(|_window, _cx| {
                            observe_case(&host.lock().expect("host lock"), &iface)
                        })
                        .unwrap_or_else(|_| json!({}));
                    observations.push(observation);
                }
                "expectPart" => {
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                    let observation = cx
                        .update(|_window, _cx| {
                            observe_case(&host.lock().expect("host lock"), &iface)
                        })
                        .unwrap_or_else(|_| json!({}));
                    let mut results = Vec::new();
                    assert_part(
                        &iface,
                        part,
                        &expect,
                        index,
                        observation,
                        "gpui",
                        &mut results,
                    );
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
                    let actual = host
                        .lock()
                        .expect("host lock")
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
                        .collect::<Vec<_>>();
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
        drain_event_queue(cx).await;
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

pub fn tabs_report(component: &str, outcomes: &[CaseOutcome]) -> Value {
    json!({
        "runtime": "gpui",
        "component": component,
        "results": outcomes.iter().map(|outcome| json!({
            "caseId": outcome.case_id,
            "pass": outcome.pass,
            "failures": outcome.failures,
            "assertions": outcome.assertions,
            "observations": outcome.observations,
        })).collect::<Vec<_>>(),
    })
}
