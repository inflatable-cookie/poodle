//! TextInput GPUI conformance adapter (g14.006), headless (g14.023).
//!
//! Insert goes through real keystrokes (`on_edit_key` → `edit_transition`).
//! IME start/update mark without insert; commit is `on_edit_insert`.

use std::sync::{Arc, Mutex};

use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, observe_tree_with_focus, InterfaceDoc,
};
use poodle_render::{text_input_with_handlers, TextInputHandlers};
use poodle_specs::TextInputSpec;
use serde_json::{json, Value};

use super::conformance_button::CaseOutcome;
use super::conformance_driver::HeadlessDriver;
use super::conformance_support::text_input_spec_from_fixture;

pub const ROOT_ELEMENT_ID: &str = "conformance-text-input";
pub const VALUE_ELEMENT_ID: &str = "poodle-input-conformance-text-input-value";
pub const CLEAR_ELEMENT_ID: &str = "text-input-clear";

/// Planted defects for the failure-proof tests. None of these identifiers
/// belong in the generic driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantedDefect {
    DroppedEdit,
    DroppedSelection,
    DroppedImeCommit,
    SubmitBeforeValueChange,
}

struct CaseHost {
    spec: TextInputSpec,
    node: Arc<Mutex<Node>>,
    value: Arc<Mutex<String>>,
    selection: Arc<Mutex<(usize, usize)>>,
    trace: Arc<Mutex<Vec<Value>>>,
    theme: GpuiThemeProvider,
    defect: Option<PlantedDefect>,
    record_trace: Arc<Mutex<bool>>,
}

impl CaseHost {
    fn rebuild(&mut self) {
        let live = self.value.lock().expect("value lock").clone();
        let selection = *self.selection.lock().expect("selection lock");
        self.spec.value = Some(live);
        self.spec.selection_start = selection.0;
        self.spec.selection_end = selection.1;

        let trace = Arc::clone(&self.trace);
        let live = Arc::clone(&self.value);
        let sel = Arc::clone(&self.selection);
        let record = Arc::clone(&self.record_trace);
        let defect = self.defect;

        let on_change = {
            let trace = Arc::clone(&trace);
            let live = Arc::clone(&live);
            let record = Arc::clone(&record);
            Arc::new(move |next: &str| {
                if defect == Some(PlantedDefect::DroppedEdit) {
                    return;
                }
                *live.lock().expect("value lock") = next.to_owned();
                if !*record.lock().expect("record lock") {
                    return;
                }
                let event = if defect == Some(PlantedDefect::SubmitBeforeValueChange) {
                    "submit"
                } else {
                    "valueChange"
                };
                trace.lock().expect("trace lock").push(json!({
                    "event": event,
                    "payload": { "value": next },
                }));
            })
        };
        let on_selection_change = {
            let sel = Arc::clone(&sel);
            Arc::new(move |start, end| {
                if defect == Some(PlantedDefect::DroppedSelection) {
                    return;
                }
                *sel.lock().expect("selection lock") = (start, end);
            })
        };
        let on_submit = {
            let trace = Arc::clone(&trace);
            let live = Arc::clone(&live);
            Arc::new(move || {
                let event = if defect == Some(PlantedDefect::SubmitBeforeValueChange) {
                    "valueChange"
                } else {
                    "submit"
                };
                let value = live.lock().expect("value lock").clone();
                trace.lock().expect("trace lock").push(json!({
                    "event": event,
                    "payload": { "value": value },
                }));
            })
        };
        let on_cancel = {
            let trace = Arc::clone(&trace);
            Arc::new(move || {
                trace.lock().expect("trace lock").push(json!({
                    "event": "cancel",
                }));
            })
        };
        let on_clear = {
            let trace = Arc::clone(&trace);
            Arc::new(move || {
                trace.lock().expect("trace lock").push(json!({
                    "event": "clear",
                }));
            })
        };

        let mut node = text_input_with_handlers(
            &self.spec,
            &self.theme,
            TextInputHandlers {
                on_change: Some(on_change),
                on_selection_change: Some(on_selection_change),
                on_submit: Some(on_submit),
                on_cancel: Some(on_cancel),
                on_clear: Some(on_clear),
                ..TextInputHandlers::default()
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
        observe_tree_with_focus("gpui", "text-input", iface, &node, &focus_by_id);
    let value = host.value.lock().expect("value lock").clone();
    let (start, end) = *host.selection.lock().expect("selection lock");
    if let Some(parts) = observation
        .get_mut("parts")
        .and_then(Value::as_object_mut)
    {
        if let Some(entry) = parts.get_mut("control").and_then(Value::as_object_mut) {
            entry.insert("value".to_owned(), json!(value));
            entry.insert("selectionStart".to_owned(), json!(start));
            entry.insert("selectionEnd".to_owned(), json!(end));
        }
    }
    observation["trace"] = json!(host.trace.lock().expect("trace lock").clone());
    observation
}

fn part_element_id(part: &str) -> &str {
    match part {
        "clear" => CLEAR_ELEMENT_ID,
        _ => ROOT_ELEMENT_ID,
    }
}

fn drive_insert(driver: &mut HeadlessDriver<'_>, host: &Arc<Mutex<CaseHost>>, text: &str) {
    let inert = {
        let host = host.lock().expect("host lock");
        host.spec.is_disabled || host.spec.is_read_only
    };
    if inert {
        return;
    }
    let (before, record) = {
        let host = host.lock().expect("host lock");
        let before = host.value.lock().expect("value lock").clone();
        let record = Arc::clone(&host.record_trace);
        (before, record)
    };
    *record.lock().expect("record lock") = false;
    for ch in text.chars() {
        driver.keyboard_key(ROOT_ELEMENT_ID, &ch.to_string());
        host.lock().expect("host lock").rebuild();
        driver.draw_frame();
    }
    *record.lock().expect("record lock") = true;
    let host = host.lock().expect("host lock");
    let after = host.value.lock().expect("value lock").clone();
    if after != before && host.defect != Some(PlantedDefect::DroppedEdit) {
        let event = if host.defect == Some(PlantedDefect::SubmitBeforeValueChange) {
            "submit"
        } else {
            "valueChange"
        };
        host.trace.lock().expect("trace lock").push(json!({
            "event": event,
            "payload": { "value": after },
        }));
    }
}

fn drive_select(driver: &mut HeadlessDriver<'_>, host: &Arc<Mutex<CaseHost>>, start: usize, end: usize) {
    driver.keyboard_key(ROOT_ELEMENT_ID, "home");
    host.lock().expect("host lock").rebuild();
    driver.draw_frame();
    for _ in 0..start {
        driver.keyboard_key(ROOT_ELEMENT_ID, "right");
        host.lock().expect("host lock").rebuild();
        driver.draw_frame();
    }
    let span = end.saturating_sub(start);
    for _ in 0..span {
        driver.keyboard_key(ROOT_ELEMENT_ID, "shift-right");
        host.lock().expect("host lock").rebuild();
        driver.draw_frame();
    }
}

fn drive_compose(
    driver: &mut HeadlessDriver<'_>,
    host: &Arc<Mutex<CaseHost>>,
    text: &str,
    phase: &str,
) {
    let selection = *host.lock().expect("host lock").selection.lock().expect("selection lock");
    match phase {
        "start" | "update" => {
            poodle_gpui_node_backend::mark_composing(VALUE_ELEMENT_ID, selection, text);
            driver.draw_frame();
        }
        _ => {
            let committed = poodle_gpui_node_backend::take_composing(VALUE_ELEMENT_ID)
                .unwrap_or_else(|| text.to_owned());
            if host.lock().expect("host lock").defect == Some(PlantedDefect::DroppedImeCommit) {
                driver.draw_frame();
                return;
            }
            let insert = {
                let host = host.lock().expect("host lock");
                let insert = host
                    .node
                    .lock()
                    .expect("node lock")
                    .interaction
                    .on_edit_insert
                    .clone();
                insert
            };
            if let Some(insert) = insert {
                insert(&committed);
            }
            host.lock().expect("host lock").rebuild();
            driver.draw_frame();
        }
    }
}

pub fn drive_text_input_cases(
    driver: &mut HeadlessDriver<'_>,
    iface: InterfaceDoc,
    cases: Vec<Value>,
    only: Option<String>,
    defect: Option<PlantedDefect>,
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

        let spec = text_input_spec_from_fixture(&fixture);
        let value = Arc::new(Mutex::new(spec.current_value().to_owned()));
        let selection = Arc::new(Mutex::new(spec.selection_range()));
        let trace = Arc::new(Mutex::new(Vec::<Value>::new()));
        let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let node = Arc::new(Mutex::new(Node::container()));
        let host = Arc::new(Mutex::new(CaseHost {
            spec,
            node: Arc::clone(&node),
            value: Arc::clone(&value),
            selection: Arc::clone(&selection),
            trace: Arc::clone(&trace),
            theme,
            defect,
            record_trace: Arc::new(Mutex::new(true)),
        }));

        {
            let mut host = host.lock().expect("host lock");
            host.rebuild();
        }
        driver.mount_node(Arc::clone(&node));
        driver.blur_element_focus(ROOT_ELEMENT_ID);
        driver.wait_for_focus_handle(ROOT_ELEMENT_ID);

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
                    match name {
                        "focus" => driver.focus_element(part_element_id(part)),
                        "key" => {
                            let key = step.get("key").and_then(Value::as_str).unwrap_or("");
                            let gpui_key = match key {
                                "Enter" => "enter",
                                "Escape" => "escape",
                                other => other,
                            };
                            driver.keyboard_key(part_element_id(part), gpui_key);
                            host.lock().expect("host lock").rebuild();
                            driver.draw_frame();
                        }
                        "insert" => {
                            let text = step.get("text").and_then(Value::as_str).unwrap_or("");
                            drive_insert(driver, &host, text);
                        }
                        "select" => {
                            let start = step.get("start").and_then(Value::as_u64).unwrap_or(0) as usize;
                            let end = step.get("end").and_then(Value::as_u64).unwrap_or(0) as usize;
                            drive_select(driver, &host, start, end);
                        }
                        "compose" => {
                            let text = step.get("text").and_then(Value::as_str).unwrap_or("");
                            let phase = step.get("phase").and_then(Value::as_str).unwrap_or("commit");
                            drive_compose(driver, &host, text, phase);
                        }
                        "press" => {
                            driver.draw_frame();
                            driver.pointer_activate_id(part_element_id(part));
                            host.lock().expect("host lock").rebuild();
                            driver.draw_frame();
                        }
                        _ => {}
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
                        .filter_map(|entry| {
                            entry.get("event").and_then(Value::as_str).map(str::to_owned)
                        })
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

pub fn text_input_report(component: &str, outcomes: &[CaseOutcome]) -> Value {
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
