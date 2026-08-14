//! Tabs GPUI conformance adapter (g14.004), headless (g14.023).

use std::sync::{Arc, Mutex};
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, observe_tree_with_focus, InterfaceDoc,
};
use poodle_render::{tabs_with_panel, TabsHandlers};
use poodle_specs::TabsSpec;
use serde_json::{json, Value};

use super::conformance_button::CaseOutcome;
use super::conformance_driver::HeadlessDriver;
use super::conformance_support::tabs_spec_from_fixture;

struct CaseHost {
    spec: TabsSpec,
    node: Arc<Mutex<Node>>,
    value: Arc<Mutex<String>>,
    focused: Arc<Mutex<String>>,
    trace: Arc<Mutex<Vec<Value>>>,
    panel_text: String,
    instance_id: String,
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
                instance_id: Some(self.instance_id.clone()),
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

fn element_id(instance_id: &str, part: &str) -> String {
    part_value(part)
        .map(|value| format!("tabs:{instance_id}:tab:{value}"))
        .unwrap_or_else(|| format!("tabs:{instance_id}:list"))
}

fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
    let mut observation = observe_tree_with_focus("gpui", "tabs", iface, &node, &focus_by_id);
    observation["trace"] = json!(host.trace.lock().expect("trace lock").clone());
    observation
}

fn gpui_key(key: &str) -> Option<&'static str> {
    Some(match key {
        "ArrowRight" => "right",
        "ArrowLeft" => "left",
        "ArrowDown" => "down",
        "ArrowUp" => "up",
        "Home" => "home",
        "End" => "end",
        _ => return None,
    })
}

fn rebuild_and_focus(driver: &mut HeadlessDriver<'_>, host: &Arc<Mutex<CaseHost>>) {
    let target = {
        let mut host = host.lock().expect("host lock");
        host.rebuild();
        let target = format!(
            "tabs:{}:tab:{}",
            host.instance_id,
            host.focused.lock().expect("focus lock")
        );
        driver.draw_frame();
        target
    };
    driver.wait_for_focus_handle(&target);
    driver.focus_element(&target);
}

fn rebuild(driver: &mut HeadlessDriver<'_>, host: &Arc<Mutex<CaseHost>>) {
    host.lock().expect("host lock").rebuild();
    driver.draw_frame();
}

pub fn drive_tabs_cases(
    driver: &mut HeadlessDriver<'_>,
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
            instance_id: format!("conformance-{case_id}"),
            theme: GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE),
        }));
        host.lock().expect("host lock").rebuild();
        driver.mount_node(Arc::clone(&node));
        let initial_id = format!("tabs:conformance-{case_id}:tab:{initial}");
        driver.blur_element_focus(&initial_id);
        // A prior case can briefly repaint the new semantic ids while its old
        // focus handle is still active. Reset the host's semantic focus after
        // the real backend blur so the new case starts from its fixture.
        {
            let mut host = host.lock().expect("host lock");
            *host.focused.lock().expect("focus lock") = initial.clone();
            host.rebuild();
        }
        driver.draw_frame();
        driver.wait_for_focus_handle(&initial_id);

        let mount_observation = observe_case(&host.lock().expect("host lock"), &iface);
        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = vec![mount_observation];

        for (index, step) in steps.iter().enumerate() {
            match step.get("kind").and_then(Value::as_str).unwrap_or("") {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let instance_id = host.lock().expect("host lock").instance_id.clone();
                    let id = element_id(&instance_id, part);
                    match name {
                        "focus" => {
                            if let Some(value) = part_value(part) {
                                let host = host.lock().expect("host lock");
                                *host.focused.lock().expect("focus lock") = value.to_owned();
                            }
                            rebuild_and_focus(driver, &host);
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
                            driver.keyboard_activate(&id);
                            if !disabled {
                                rebuild(driver, &host);
                            }
                        }
                        "key" => {
                            let key = step.get("key").and_then(Value::as_str).unwrap_or("");
                            if key == "Enter" {
                                driver.keyboard_activate(&id);
                            } else if let Some(gpui_key) = gpui_key(key) {
                                driver.keyboard_key(&id, gpui_key);
                            }
                            rebuild(driver, &host);
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
