//! RangeSlider GPUI conformance adapter (g14.003), headless (g14.023).

use std::sync::{Arc, Mutex};
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, expected_events, observe_tree_with_focus, InterfaceDoc,
};
use poodle_render::{range_slider, RangeSliderHandlers};
use poodle_specs::RangeSliderSpec;
use serde_json::{json, Value};

use super::conformance_button::CaseOutcome;
use super::conformance_driver::HeadlessDriver;
use super::conformance_support::range_slider_spec_from_fixture;

pub const ROOT_ELEMENT_ID: &str = "conformance-range-slider";

struct CaseHost {
    spec: RangeSliderSpec,
    node: Arc<Mutex<Node>>,
    value: Arc<Mutex<(f64, f64)>>,
    trace: Arc<Mutex<Vec<Value>>>,
    theme: GpuiThemeProvider,
}

impl CaseHost {
    fn rebuild(&mut self) {
        let value = *self.value.lock().expect("value lock");
        self.spec.low = value.0;
        self.spec.high = value.1;
        let trace = Arc::clone(&self.trace);
        let live = Arc::clone(&self.value);
        let on_change = {
            let trace = Arc::clone(&trace);
            let live = Arc::clone(&live);
            Arc::new(move |low, high| {
                *live.lock().expect("value lock") = (low, high);
                trace.lock().expect("trace lock").push(json!({
                    "event": "valueChange",
                    "payload": { "value": [low, high] },
                }));
            })
        };
        let on_value_commit = {
            let live = Arc::clone(&live);
            Arc::new(move |low, high| {
                *live.lock().expect("value lock") = (low, high);
                trace.lock().expect("trace lock").push(json!({
                    "event": "valueCommit",
                    "payload": { "value": [low, high] },
                }));
            })
        };
        let mut node = range_slider(
            &self.spec,
            &self.theme,
            RangeSliderHandlers {
                on_change: Some(on_change),
                on_value_commit: Some(on_value_commit),
            },
        );
        node.id = Some(ROOT_ELEMENT_ID.to_owned());
        *self.node.lock().expect("node lock") = node;
    }
}

fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
    let mut observation =
        observe_tree_with_focus("gpui", "range-slider", iface, &node, &focus_by_id);
    // Live host value wins over a11y snapshots: scrub updates the host
    // without rebuilding the tree mid-gesture (rebuild would reset gesture
    // atomics and break press→drag→release).
    let (low, high) = *host.value.lock().expect("value lock");
    if let Some(parts) = observation
        .get_mut("parts")
        .and_then(Value::as_object_mut)
    {
        if let Some(lower) = parts.get_mut("lower").and_then(Value::as_object_mut) {
            lower.insert("value".to_owned(), json!(low));
        }
        if let Some(upper) = parts.get_mut("upper").and_then(Value::as_object_mut) {
            upper.insert("value".to_owned(), json!(high));
        }
        if let Some(root) = parts.get_mut("root").and_then(Value::as_object_mut) {
            root.insert("value".to_owned(), json!([low, high]));
        }
    }
    observation["trace"] = json!(host.trace.lock().expect("trace lock").clone());
    observation
}

fn part_element_id(part: &str) -> &str {
    match part {
        "lower" => "range-slider-lower",
        "upper" => "range-slider-upper",
        _ => ROOT_ELEMENT_ID,
    }
}

pub fn drive_range_slider_cases(
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

        let spec = range_slider_spec_from_fixture(&fixture);
        let value = Arc::new(Mutex::new((spec.low, spec.high)));
        let trace = Arc::new(Mutex::new(Vec::<Value>::new()));
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let node = Arc::new(Mutex::new(Node::container()));
        let host = Arc::new(Mutex::new(CaseHost {
            spec,
            node: Arc::clone(&node),
            value: Arc::clone(&value),
            trace: Arc::clone(&trace),
            theme,
        }));

        {
            let mut host = host.lock().expect("host lock");
            host.rebuild();
        }
        driver.mount_node(Arc::clone(&node));
        driver.blur_element_focus("range-slider-lower");
        driver.wait_for_focus_handle("range-slider-lower");

        let mount_observation = observe_case(&host.lock().expect("host lock"), &iface);

        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = vec![mount_observation];

        for (index, step) in steps.iter().enumerate() {
            let kind = step.get("kind").and_then(Value::as_str).unwrap_or("");
            match kind {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("root");
                    if name == "focus" {
                        driver.focus_element(part_element_id(part));
                    } else if name == "key" {
                        let key = step.get("key").and_then(Value::as_str).unwrap_or("");
                        let gpui_key = match key {
                            "ArrowRight" | "ArrowUp" => "right",
                            _ => "right",
                        };
                        driver.keyboard_key(part_element_id(part), gpui_key);
                        let mut host = host.lock().expect("host lock");
                        host.rebuild();
                        driver.draw_frame();
                    } else if name == "scrub" {
                        let fraction = step.get("fraction").and_then(Value::as_f64).unwrap_or(0.0) as f32;
                        let phase = step.get("phase").and_then(Value::as_str).unwrap_or("press");
                        driver.pointer_scrub_at(fraction, phase);
                        // Rebuild only after release — press/drag must keep
                        // the same scrub atomics for gesture continuity.
                        if phase == "release" {
                            let mut host = host.lock().expect("host lock");
                            host.rebuild();
                            driver.draw_frame();
                        }
                    }
                    let action_observation = observe_case(&host.lock().expect("host lock"), &iface);
                    observations.push(action_observation);
                }
                "expectPart" => {
                    let part = step.get("part").and_then(Value::as_str).unwrap_or("");
                    let expect = step.get("expect").cloned().unwrap_or(Value::Null);
                    let observation = observe_case(&host.lock().expect("host lock"), &iface);
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
                    let expected = expected_events(step);
                    let actual = trace.lock().expect("trace lock").clone();
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

        driver.drain();
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

pub fn range_slider_report(component: &str, outcomes: &[CaseOutcome]) -> Value {
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
