//! Conformance runner for the GPUI runtime (spec 066, g14.001 / g14.002).
//!
//! ```text
//!   cargo run --bin conformance -- --out=test/conformance/web/out/gpui.json
//!   cargo run --bin conformance -- --primitives --out=test/conformance/web/out/primitive-gpui.json
//! ```

use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_node::Node;
use poodle_render::conformance::InterfaceDoc;
use serde_json::Value;

#[path = "../conformance_driver.rs"]
mod conformance_driver;
#[path = "../conformance_button.rs"]
mod conformance_button;
#[path = "../conformance_range_slider.rs"]
mod conformance_range_slider;
#[path = "../conformance_support.rs"]
mod conformance_support;
#[path = "../primitive_probes_gpui.rs"]
mod primitive_probes_gpui;

use conformance_button::{button_report, drive_button_cases, registry_has_button, CaseOutcome};
use conformance_driver::{
    conformance_assets, primitive_evidence_report, set_exit_from_probes, write_or_print_report,
    ConformanceRoot, EXIT_CODE,
};
use conformance_range_slider::{drive_range_slider_cases, range_slider_report};
use primitive_probes_gpui::drive_primitive_probes;

fn parse_args(args: &[String]) -> (bool, Option<String>, Option<PathBuf>) {
    let primitives = args.iter().any(|a| a == "--primitives");
    let only = args
        .iter()
        .find_map(|a| a.strip_prefix("--case=").map(str::to_owned));
    let out = args
        .iter()
        .find_map(|a| a.strip_prefix("--out=").map(PathBuf::from));
    (primitives, only, out)
}

fn run_button_mode(only: Option<String>, out: Option<PathBuf>) {
    if !registry_has_button() {
        eprintln!("completion: button registration missing from the GPUI registry");
        std::process::exit(1);
    }

    let interface: Value = serde_json::from_str(conformance_support::INTERFACE)
        .expect("committed interface parses");
    let iface = InterfaceDoc::parse(&interface).expect("interface parses");
    let cases: Value =
        serde_json::from_str(conformance_support::CASES).expect("committed corpus parses");
    let component = cases
        .get("component")
        .and_then(Value::as_str)
        .unwrap_or("button")
        .to_owned();
    let case_list = cases
        .get("cases")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let range_interface: Value =
        serde_json::from_str(conformance_support::RANGE_SLIDER_INTERFACE)
            .expect("range-slider interface parses");
    let range_iface = InterfaceDoc::parse(&range_interface).expect("range-slider interface parses");
    let range_cases: Value = serde_json::from_str(conformance_support::RANGE_SLIDER_CASES)
        .expect("range-slider corpus parses");
    let range_component = range_cases
        .get("component")
        .and_then(Value::as_str)
        .unwrap_or("range-slider")
        .to_owned();
    let range_case_list = range_cases
        .get("cases")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let outcomes: Arc<Mutex<Vec<CaseOutcome>>> = Arc::new(Mutex::new(Vec::new()));
    let range_outcomes: Arc<Mutex<Vec<CaseOutcome>>> = Arc::new(Mutex::new(Vec::new()));
    let outcomes_in_run = Arc::clone(&outcomes);
    let range_outcomes_in_run = Arc::clone(&range_outcomes);

    Application::new()
        .with_assets(conformance_assets())
        .run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let window_node = Arc::new(Mutex::new(Node::container()));
                    let root_entity = cx.new(|cx| ConformanceRoot {
                        node: Arc::clone(&window_node),
                        focus: cx.focus_handle(),
                    });
                    let iface = iface.clone();
                    let cases = case_list.clone();
                    let range_iface = range_iface.clone();
                    let range_cases = range_case_list.clone();
                    let only = only.clone();
                    let out = out.clone();
                    let range_component = range_component.clone();
                    window
                        .spawn(cx, async move |cx| {
                            let results = drive_button_cases(
                                cx,
                                iface,
                                cases,
                                only.clone(),
                                conformance_support::spec_from_fixture,
                            )
                            .await;
                            let report = button_report(&component, &results);
                            let failed = results.iter().filter(|o| !o.pass).count();
                            if failed > 0 {
                                EXIT_CODE.store(1, Ordering::SeqCst);
                            }
                            *outcomes_in_run.lock().expect("outcomes lock") = results;
                            write_or_print_report(out.as_ref(), &report);

                            let range_results =
                                drive_range_slider_cases(cx, range_iface, range_cases, only).await;
                            let range_report = range_slider_report(&range_component, &range_results);
                            let range_failed = range_results.iter().filter(|o| !o.pass).count();
                            if range_failed > 0 {
                                EXIT_CODE.store(1, Ordering::SeqCst);
                            }
                            *range_outcomes_in_run.lock().expect("range outcomes lock") =
                                range_results;
                            let range_out = out.as_ref().map(|path| {
                                path.with_file_name("gpui-range-slider.json")
                            });
                            write_or_print_report(range_out.as_ref(), &range_report);

                            if failed + range_failed > 0 {
                                eprintln!(
                                    "\n{} failing case(s) — see report",
                                    failed + range_failed
                                );
                            }
                            cx.update(|_window, cx| cx.quit()).ok();
                        })
                        .detach();
                    root_entity
                },
            );
        });

    let failed = outcomes
        .lock()
        .expect("outcomes lock")
        .iter()
        .filter(|o| !o.pass)
        .count()
        + range_outcomes
            .lock()
            .expect("range outcomes lock")
            .iter()
            .filter(|o| !o.pass)
            .count();
    let exit_code = EXIT_CODE.load(Ordering::SeqCst);
    if failed > 0 || exit_code != 0 {
        std::process::exit(1);
    }
}

fn run_primitives_mode(out: Option<PathBuf>) {
    let probes_store: Arc<Mutex<Vec<poodle_render::primitive_probes::ProbeEvidence>>> =
        Arc::new(Mutex::new(Vec::new()));
    let probes_in_run = Arc::clone(&probes_store);

    Application::new()
        .with_assets(conformance_assets())
        .run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let window_node = Arc::new(Mutex::new(Node::container()));
                    let root_entity = cx.new(|cx| ConformanceRoot {
                        node: Arc::clone(&window_node),
                        focus: cx.focus_handle(),
                    });
                    let out = out.clone();
                    window
                        .spawn(cx, async move |cx| {
                            let probes = drive_primitive_probes(cx).await;
                            let failed: Vec<_> = probes
                                .iter()
                                .filter(|p| p.verdict == "fail")
                                .collect();
                            if !failed.is_empty() {
                                EXIT_CODE.store(1, Ordering::SeqCst);
                                for probe in &failed {
                                    eprintln!("{}", probe.failure_message("gpui"));
                                }
                            }
                            set_exit_from_probes(&probes);
                            *probes_in_run.lock().expect("probes lock") = probes.clone();
                            let report = primitive_evidence_report(&probes);
                            write_or_print_report(out.as_ref(), &report);
                            if !failed.is_empty() {
                                eprintln!("\n{} failing primitive probe(s) — see report", failed.len());
                            }
                            cx.update(|_window, cx| cx.quit()).ok();
                        })
                        .detach();
                    root_entity
                },
            );
        });

    let failed = probes_store
        .lock()
        .expect("probes lock")
        .iter()
        .any(|p| p.verdict == "fail");
    let exit_code = EXIT_CODE.load(Ordering::SeqCst);
    if failed || exit_code != 0 {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.iter().any(|arg| arg == "--windowed") {
        eprintln!(
            "GPUI conformance drives a foreground AppKit window. Re-run with --windowed only in an isolated desktop session."
        );
        std::process::exit(2);
    }
    let foreground_allowed = std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true")
        || std::env::var("POODLE_ALLOW_FOREGROUND_CONFORMANCE").as_deref() == Ok("1");
    if !foreground_allowed {
        eprintln!(
            "Foreground GPUI conformance is blocked on local desktops. Use the headless `effigy ci:conformance` board. Only isolated CI or an operator-approved run with POODLE_ALLOW_FOREGROUND_CONFORMANCE=1 may take OS focus."
        );
        std::process::exit(2);
    }
    let (primitives, only, out) = parse_args(&args);

    if primitives {
        run_primitives_mode(out);
    } else {
        run_button_mode(only, out);
    }
}
