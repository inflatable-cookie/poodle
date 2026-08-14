//! Headless GPUI conformance board (spec 066, g14.023).
//!
//! Executes the whole active cohort — Button, RangeSlider, Tabs, and the
//! primitive substrate — on GPUI 0.2.2's in-memory test platform
//! (`TestAppContext`, `VisualTestContext`, `TestWindow`). No OS window is
//! created, no application is activated, and no keyboard focus is taken.
//!
//! The board writes the same report files the windowed runner produced
//! (`test/conformance/web/out/gpui*.json`, `primitive-gpui.json`), so the
//! cross-runtime comparator and primitive report gates are unchanged. The
//! focused tests below prove the driver's real event tree and the planted
//! failures prove the runner catches each defect class.

#![recursion_limit = "512"]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// Explicit import only: `use gpui::*` would glob in gpui's `test` proc macro
// and shadow the built-in `#[test]` attribute (gpui-macros 0.2.2's `test`
// crashes on current rustc).
use gpui::TestAppContext;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, observe_tree_with_focus, InterfaceDoc,
};
use poodle_render::{range_slider, tabs_with_panel, RangeSliderHandlers, TabsHandlers};
use poodle_specs::{RangeSliderSpec, TabDefinition, TabsSpec};
use serde_json::{json, Value};

#[path = "../src/conformance_button.rs"]
mod conformance_button;
#[path = "../src/conformance_driver.rs"]
mod conformance_driver;
#[path = "../src/conformance_range_slider.rs"]
mod conformance_range_slider;
#[path = "../src/conformance_support.rs"]
mod conformance_support;
#[path = "../src/conformance_tabs.rs"]
mod conformance_tabs;
#[path = "../src/primitive_probes_gpui.rs"]
mod primitive_probes_gpui;

use conformance_driver::HeadlessDriver;
use conformance_support::spec_from_fixture;

/// Shared in-memory test-platform harness. The `#[gpui::test]` macro from
/// gpui-macros 0.2.2 crashes on current rustc, so this mirrors its teardown
/// (parked queue, forbidden parking, app shutdown) in a plain `#[test]`.
fn run_headless(body: impl FnOnce(&mut TestAppContext)) {
    let mut cx = TestAppContext::single();
    body(&mut cx);
    cx.dispatcher.run_until_parked();
    cx.background_executor.forbid_parking();
    cx.quit();
    cx.dispatcher.run_until_parked();
}

fn report_out_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test/conformance/web/out")
}

fn write_report(file_name: &str, report: &Value) -> PathBuf {
    let path = report_out_dir().join(file_name);
    conformance_driver::write_or_print_report(Some(&path), report);
    path
}

fn parse_corpus(suffix: &str) -> (String, Vec<Value>) {
    let raw = match suffix {
        "" => conformance_support::CASES,
        "-range-slider" => conformance_support::RANGE_SLIDER_CASES,
        "-tabs" => conformance_support::TABS_CASES,
        _ => panic!("unknown corpus suffix {suffix}"),
    };
    let cases: Value = serde_json::from_str(raw).expect("committed corpus parses");
    let component = cases
        .get("component")
        .and_then(Value::as_str)
        .unwrap_or("?")
        .to_owned();
    let case_list = cases
        .get("cases")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    (component, case_list)
}

fn parse_interface(suffix: &str) -> InterfaceDoc {
    let raw = match suffix {
        "" => conformance_support::INTERFACE,
        "-range-slider" => conformance_support::RANGE_SLIDER_INTERFACE,
        "-tabs" => conformance_support::TABS_INTERFACE,
        _ => panic!("unknown interface suffix {suffix}"),
    };
    let value: Value = serde_json::from_str(raw).expect("committed interface parses");
    InterfaceDoc::parse(&value).expect("interface parses")
}

/// The complete active-cohort board: every landed GPUI case executes on the
/// in-memory test platform and the reports land where the comparator reads
/// them. This one test owns the report files, so parallel focused tests never
/// race the writer.
#[test]
fn complete_board() {
    run_headless(run_complete_board);
}

fn run_complete_board(cx: &mut TestAppContext) {
    if !conformance_button::registry_has_button() {
        panic!("completion: button registration missing from the GPUI registry");
    }

    let node = Arc::new(Mutex::new(Node::container()));
    let mut driver = HeadlessDriver::new(cx, node);

    let mut failures = Vec::new();

    let (component, cases) = parse_corpus("");
    let results = conformance_button::drive_button_cases(
        &mut driver,
        parse_interface(""),
        cases,
        None,
        spec_from_fixture,
    );
    write_report("gpui.json", &conformance_button::button_report(&component, &results));
    failures.extend(results.iter().filter(|o| !o.pass).map(|o| o.case_id.clone()));

    let (component, cases) = parse_corpus("-range-slider");
    let results = conformance_range_slider::drive_range_slider_cases(
        &mut driver,
        parse_interface("-range-slider"),
        cases,
        None,
    );
    write_report(
        "gpui-range-slider.json",
        &conformance_range_slider::range_slider_report(&component, &results),
    );
    failures.extend(results.iter().filter(|o| !o.pass).map(|o| o.case_id.clone()));

    let (component, cases) = parse_corpus("-tabs");
    let results =
        conformance_tabs::drive_tabs_cases(&mut driver, parse_interface("-tabs"), cases, None);
    write_report("gpui-tabs.json", &conformance_tabs::tabs_report(&component, &results));
    failures.extend(results.iter().filter(|o| !o.pass).map(|o| o.case_id.clone()));

    let probes = primitive_probes_gpui::drive_primitive_probes(&mut driver);
    write_report(
        "primitive-gpui.json",
        &conformance_driver::primitive_evidence_report(&probes),
    );
    let failed_probes: Vec<_> = probes
        .iter()
        .filter(|p| p.verdict == "fail")
        .map(|p| format!("{} ({})", p.capability_id, p.probe_id))
        .collect();

    if !failures.is_empty() || !failed_probes.is_empty() {
        panic!(
            "headless GPUI board failed: {} failing case(s) {failures:?}, {} failing primitive probe(s) {failed_probes:?}",
            failures.len(),
            failed_probes.len()
        );
    }
}

// ── Focused driver tests ───────────────────────────────────────────────────

fn button_node(
    spec: poodle_specs::ButtonSpec,
    handler: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
    let mut node = poodle_render::button(&spec, &theme, handler);
    node.id = Some(conformance_button::BUTTON_ELEMENT_ID.to_owned());
    node
}

#[test]
fn driver_mounts_and_tracks_backend_focus() {
    run_headless(run_driver_mounts_and_tracks_backend_focus);
}

fn run_driver_mounts_and_tracks_backend_focus(cx: &mut TestAppContext) {
    let spec = poodle_specs::ButtonSpec::new().with_label("focus");
    let node = Arc::new(Mutex::new(button_node(spec, None)));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle(conformance_button::BUTTON_ELEMENT_ID);
    driver.focus_element(conformance_button::BUTTON_ELEMENT_ID);
    assert_eq!(
        poodle_gpui_node_backend::focus_state_for(conformance_button::BUTTON_ELEMENT_ID),
        Some(true)
    );

    driver.blur_element_focus(conformance_button::BUTTON_ELEMENT_ID);
    assert_eq!(
        poodle_gpui_node_backend::focus_state_for(conformance_button::BUTTON_ELEMENT_ID),
        Some(false)
    );
}

#[test]
fn driver_pointer_activation_fires_backend_listener() {
    run_headless(run_driver_pointer_activation_fires_backend_listener);
}

fn run_driver_pointer_activation_fires_backend_listener(cx: &mut TestAppContext) {
    let clicks = Arc::new(Mutex::new(0usize));
    let clicks_for_handler = Arc::clone(&clicks);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *clicks_for_handler.lock().expect("clicks lock") += 1;
    });
    let node = Arc::new(Mutex::new(button_node(
        poodle_specs::ButtonSpec::new().with_label("click"),
        Some(handler),
    )));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle(conformance_button::BUTTON_ELEMENT_ID);
    driver.pointer_activate();
    assert_eq!(*clicks.lock().expect("clicks lock"), 1);
}

#[test]
fn driver_keyboard_activation_fires_backend_listener() {
    run_headless(run_driver_keyboard_activation_fires_backend_listener);
}

fn run_driver_keyboard_activation_fires_backend_listener(cx: &mut TestAppContext) {
    let presses = Arc::new(Mutex::new(0usize));
    let presses_for_handler = Arc::clone(&presses);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *presses_for_handler.lock().expect("presses lock") += 1;
    });
    let node = Arc::new(Mutex::new(button_node(
        poodle_specs::ButtonSpec::new().with_label("enter"),
        Some(handler),
    )));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle(conformance_button::BUTTON_ELEMENT_ID);
    driver.keyboard_activate(conformance_button::BUTTON_ELEMENT_ID);
    assert_eq!(*presses.lock().expect("presses lock"), 1);
}

fn scrub_host() -> (
    Arc<Mutex<Node>>,
    Arc<Mutex<Vec<Value>>>,
    Arc<Mutex<(f64, f64)>>,
) {
    let mut spec = RangeSliderSpec::default();
    spec.low = 20.0;
    spec.high = 80.0;
    let value = Arc::new(Mutex::new((spec.low, spec.high)));
    let trace = Arc::new(Mutex::new(Vec::<Value>::new()));
    let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
    let live = Arc::clone(&value);
    let trace_change = Arc::clone(&trace);
    let on_change = Arc::new(move |low, high| {
        *live.lock().expect("value lock") = (low, high);
        trace_change.lock().expect("trace lock").push(json!({
            "event": "valueChange",
            "payload": { "value": [low, high] },
        }));
    });
    let live_commit = Arc::clone(&value);
    let trace_commit = Arc::clone(&trace);
    let on_value_commit = Arc::new(move |low, high| {
        *live_commit.lock().expect("value lock") = (low, high);
        trace_commit.lock().expect("trace lock").push(json!({
            "event": "valueCommit",
            "payload": { "value": [low, high] },
        }));
    });
    let mut node = range_slider(
        &spec,
        &theme,
        RangeSliderHandlers {
            on_change: Some(on_change),
            on_value_commit: Some(on_value_commit),
        },
    );
    node.id = Some(conformance_range_slider::ROOT_ELEMENT_ID.to_owned());
    (Arc::new(Mutex::new(node)), trace, value)
}

#[test]
fn driver_scrub_press_drag_release_order() {
    run_headless(run_driver_scrub_press_drag_release_order);
}

fn run_driver_scrub_press_drag_release_order(cx: &mut TestAppContext) {
    let (node, trace, value) = scrub_host();
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle("range-slider-lower");
    driver.pointer_scrub_at(0.9, "press");
    // A real drag moves while held — gpui arms the drag once the pointer
    // exceeds its movement threshold, then dispatches drag moves.
    driver.pointer_scrub_at(0.95, "drag");
    driver.pointer_scrub_at(0.95, "release");

    let events: Vec<String> = trace
        .lock()
        .expect("trace lock")
        .iter()
        .filter_map(|e| e.get("event").and_then(Value::as_str).map(str::to_owned))
        .collect();
    assert_eq!(events, ["valueChange", "valueChange", "valueCommit"]);
    assert_eq!(*value.lock().expect("value lock"), (20.0, 95.0));
}

// ── Planted failures (Batch B): each defect class must fail through the
// ── real event tree, naming runtime/case/step/field.

/// Inert listener: the backend binding does nothing, so a press case leaves
/// no trace and the event assertion fails.
#[test]
fn planted_inert_listener_fails() {
    run_headless(run_planted_inert_listener_fails);
}

fn run_planted_inert_listener_fails(cx: &mut TestAppContext) {
    // A disabled button is an inert activation target: the backend skips the
    // click binding, exactly the "bound but never fires" defect class.
    let spec = poodle_specs::ButtonSpec::new().with_label("inert").with_disabled(true);
    let node = Arc::new(Mutex::new(button_node(spec, Some(Arc::new(|| {})))));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.pointer_activate();

    let mut results = Vec::new();
    assert_events(&["press".to_owned()], &[], 0, &mut results);
    assert_eq!(results[0].verdict, "fail");
    assert_eq!(results[0].step_index, 0);
    assert_eq!(results[0].field, "events");
}

/// Wrong focus target: the Enter key is dispatched while the mount root holds
/// focus, so the button's activation path never runs.
#[test]
fn planted_wrong_focus_target_fails() {
    run_headless(run_planted_wrong_focus_target_fails);
}

fn run_planted_wrong_focus_target_fails(cx: &mut TestAppContext) {
    let presses = Arc::new(Mutex::new(0usize));
    let presses_for_handler = Arc::clone(&presses);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *presses_for_handler.lock().expect("presses lock") += 1;
    });
    let node = Arc::new(Mutex::new(button_node(
        poodle_specs::ButtonSpec::new().with_label("focus"),
        Some(handler),
    )));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle(conformance_button::BUTTON_ELEMENT_ID);
    driver.blur_element_focus(conformance_button::BUTTON_ELEMENT_ID);
    driver.dispatch_key("enter");

    assert_eq!(*presses.lock().expect("presses lock"), 0);
    let mut results = Vec::new();
    assert_events(&["press".to_owned()], &[], 0, &mut results);
    assert_eq!(results[0].verdict, "fail");
    assert_eq!(results[0].step_index, 0);
}

/// Missing selected state: a Tabs tree with no selection cannot satisfy a
/// `selected` expectation, and the failure names the trigger part and field.
#[test]
fn planted_missing_selected_state_fails() {
    run_headless(run_planted_missing_selected_state_fails);
}

fn run_planted_missing_selected_state_fails(cx: &mut TestAppContext) {
    let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
    let spec = TabsSpec::new(vec![
        TabDefinition::new("1", "One"),
        TabDefinition::new("2", "Two"),
    ]);
    let panel = Node::text("panel");
    let mut node = tabs_with_panel(&spec, &theme, TabsHandlers::default(), panel);
    node.id = Some("planted-tabs".to_owned());
    let node = Arc::new(Mutex::new(node));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.draw_frame();
    let iface = parse_interface("-tabs");
    let focus_by_id = |id: &str| poodle_gpui_node_backend::focus_state_for(id);
    let observation = observe_tree_with_focus("gpui", "tabs", &iface, &node.lock().expect("node lock"), &focus_by_id);

    let mut results = Vec::new();
    assert_part(&iface, "trigger:2", &json!({ "selected": true }), 0, observation, "gpui", &mut results);
    assert!(
        results
            .iter()
            .any(|r| r.verdict == "fail" && r.part.as_deref() == Some("trigger:2") && r.field == "selected"),
        "missing selected state did not fail on trigger:2/selected: {results:?}"
    );
}

/// Broken drag order: the release phase arrives without a press, so the
/// expected press→commit sequence never happens and the event assertion fails.
#[test]
fn planted_broken_drag_order_fails() {
    run_headless(run_planted_broken_drag_order_fails);
}

fn run_planted_broken_drag_order_fails(cx: &mut TestAppContext) {
    let (node, trace, _value) = scrub_host();
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle("range-slider-lower");
    driver.pointer_scrub_at(0.9, "release");

    let events: Vec<String> = trace
        .lock()
        .expect("trace lock")
        .iter()
        .filter_map(|e| e.get("event").and_then(Value::as_str).map(str::to_owned))
        .collect();
    let mut results = Vec::new();
    assert_events(&["valueChange".to_owned(), "valueCommit".to_owned()], &events, 0, &mut results);
    assert_eq!(results[0].verdict, "fail");
    assert_eq!(results[0].step_index, 0);
    assert_eq!(results[0].field, "events");
}

/// Broken keyboard order: the key-up half of the Enter pair never arrives, so
/// GPUI's keyboard-activation synthesis cannot fire the click listener.
#[test]
fn planted_broken_keyboard_order_fails() {
    run_headless(run_planted_broken_keyboard_order_fails);
}

fn run_planted_broken_keyboard_order_fails(cx: &mut TestAppContext) {
    let presses = Arc::new(Mutex::new(0usize));
    let presses_for_handler = Arc::clone(&presses);
    let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        *presses_for_handler.lock().expect("presses lock") += 1;
    });
    let node = Arc::new(Mutex::new(button_node(
        poodle_specs::ButtonSpec::new().with_label("enter"),
        Some(handler),
    )));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

    driver.wait_for_focus_handle(conformance_button::BUTTON_ELEMENT_ID);
    driver.focus_element(conformance_button::BUTTON_ELEMENT_ID);
    driver.dispatch_key_down_only("enter");

    assert_eq!(*presses.lock().expect("presses lock"), 0);
    let mut results = Vec::new();
    assert_events(&["press".to_owned()], &[], 0, &mut results);
    assert_eq!(results[0].verdict, "fail");
    assert_eq!(results[0].step_index, 0);
}
