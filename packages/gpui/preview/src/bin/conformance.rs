//! Conformance runner for the GPUI runtime (spec 066, g14.001).
//!
//! Builds the real Button node from each case fixture and projects it
//! through the real backend conversion (`poodle_gpui_node_backend::to_gpui`)
//! — the same conversion the preview mounts — then evaluates the corpus
//! against the node tree. GPUI element dispatch needs a live window, so the
//! runner dispatches the node-level activation the backend binds to
//! `on_click` (the Jetstream runner proves the full backend input path
//! headlessly). Emits `component-observation.v1` results.
//!
//! ```text
//!   cargo run --bin conformance                # all cases, JSON report
//!   cargo run --bin conformance -- --case=button/default
//!   cargo run --bin conformance -- --out=/tmp/gpui.json
//! ```

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use poodle_node::Node;
use poodle_render::conformance::{evaluate_steps, find_part, observe_tree, NativeHarness};
use poodle_specs::ButtonSpec;
use serde_json::{json, Value};

#[path = "../conformance_support.rs"]
mod conformance_support;

#[path = "../component_registry.rs"]
#[allow(dead_code)]
mod component_registry;

/// Host state behind the main mutex; trace and pressed state live on their
/// own locks so the activation handler never re-locks the host mutex.
struct HostState {
    spec: ButtonSpec,
    node: Node,
    theme: poodle_gpui::GpuiThemeProvider,
    handler: Option<Arc<dyn Fn() + Send + Sync>>,
}

struct HostLocks {
    state: Arc<Mutex<HostState>>,
    pressed: Arc<Mutex<Option<bool>>>,
    trace: Arc<Mutex<Vec<String>>>,
}

impl HostLocks {
    fn rebuild_with_toggle(&self, toggle_mode: bool) {
        let mut host = self.state.lock().expect("host state lock");
        if toggle_mode {
            let pressed = *self.pressed.lock().expect("pressed lock");
            host.spec.pressed = pressed;
        }
        let handler = host
            .handler
            .clone()
            .expect("handler bound before first render");
        host.node = poodle_render::button(&host.spec, &host.theme, Some(handler));
    }
}

struct GpuiHarness {
    locks: HostLocks,
    toggle_mode: bool,
}

impl NativeHarness for GpuiHarness {
    fn runtime(&self) -> &'static str {
        "gpui"
    }

    fn component(&self) -> &'static str {
        "button"
    }

    fn observe(&self) -> Value {
        let state = self.locks.state.lock().expect("host state lock");
        let mut observation = observe_tree("gpui", "button", &state.node);
        observation["trace"] = json!(self.locks.trace.lock().expect("trace lock").clone());
        observation
    }

    fn press(&mut self, part: &str, _input: &str) {
        // The node-level activation the backend binds to `on_click`
        // (interaction.rs). A missing handler (inert button) yields no
        // trace event and the case's press expectation fails.
        let handler = {
            let state = self.locks.state.lock().expect("host state lock");
            find_part(&state.node, part)
                .and_then(|node| node.interaction.on_activate.clone())
        };
        if let Some(handler) = handler {
            handler();
        }
        if self.toggle_mode {
            self.locks.rebuild_with_toggle(self.toggle_mode);
        }
    }

    fn focus(&mut self, _part: &str) {
        // No window: focus is not observable on the GPUI runner; the state
        // assertion is covered by the web and Jetstream runtimes.
    }

    fn trace(&self) -> Vec<String> {
        self.locks.trace.lock().expect("trace lock").clone()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let only: Option<String> = args
        .iter()
        .find_map(|a| a.strip_prefix("--case=").map(str::to_owned));
    let out: Option<PathBuf> = args
        .iter()
        .find_map(|a| a.strip_prefix("--out=").map(PathBuf::from));

    let cases: Value = serde_json::from_str(conformance_support::CASES).expect("committed corpus parses");
    let component = cases
        .get("component")
        .and_then(Value::as_str)
        .unwrap_or("button");
    let case_list = cases
        .get("cases")
        .and_then(Value::as_array)
        .expect("corpus has cases");
    let _interface: Value = serde_json::from_str(conformance_support::INTERFACE).expect("committed interface parses");

    let theme = poodle_gpui::GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);

    // Completion gate: the component must be registered — a missing
    // registration fails the run.
    {
        let registered = crate::component_registry::find_component("button").is_some();
        if !registered {
            eprintln!("completion: button registration missing from the GPUI registry");
            std::process::exit(1);
        }
    }

    let mut results = Vec::new();
    for case in case_list {
        let case_id = case.get("id").and_then(Value::as_str).unwrap_or("?");
        if let Some(only) = &only {
            if only != case_id {
                continue;
            }
        }
        let fixture = case.get("fixture").cloned().unwrap_or_else(|| json!({}));
        let steps = case
            .get("steps")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let spec = conformance_support::spec_from_fixture(&fixture);
        let toggle_mode = spec.is_toggle_mode();
        let pressed = Arc::new(Mutex::new(spec.pressed));
        let trace = Arc::new(Mutex::new(Vec::<String>::new()));
        let host = HostState {
            spec,
            node: Node::container(),
            theme: theme.clone(),
            handler: None,
        };
        let locks = HostLocks {
            state: Arc::new(Mutex::new(host)),
            pressed: Arc::clone(&pressed),
            trace: Arc::clone(&trace),
        };
        let handler_pressed = Arc::clone(&pressed);
        let handler_trace = Arc::clone(&trace);
        let handler: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            // Mirrors the web host's `pressedChange`-before-`press` order.
            let mut trace = handler_trace.lock().expect("trace lock");
            if toggle_mode {
                let mut pressed = handler_pressed.lock().expect("pressed lock");
                let next = !pressed.unwrap_or(false);
                *pressed = Some(next);
                trace.push("pressedChange".to_owned());
            }
            trace.push("press".to_owned());
        });
        {
            let mut s = locks.state.lock().expect("host state lock");
            s.handler = Some(Arc::clone(&handler));
            s.node = poodle_render::button(&s.spec, &s.theme, Some(handler));
        }

        // Backend projection: the real conversion executes on the tree the
        // preview would mount.
        {
            let s = locks.state.lock().expect("host state lock");
            let _element = poodle_gpui_node_backend::to_gpui(&s.node);
        }

        let mut harness = GpuiHarness {
            locks,
            toggle_mode,
        };
        let assertion_results = evaluate_steps(component, &steps, &mut harness);

        let observations = vec![harness.observe()];
        let failures: Vec<Value> = assertion_results
            .iter()
            .filter(|r| r.verdict == "fail")
            .map(|r| serde_json::to_value(r).expect("result serializes"))
            .collect();
        let assertions: Vec<Value> = assertion_results
            .iter()
            .map(|r| serde_json::to_value(r).expect("result serializes"))
            .collect();
        results.push(json!({
            "caseId": case_id,
            "pass": failures.is_empty(),
            "failures": failures,
            "assertions": assertions,
            "observations": observations,
        }));
    }

    let report = json!({
        "runtime": "gpui",
        "component": component,
        "results": results,
    });

    if let Some(out) = out {
        std::fs::write(out, serde_json::to_string_pretty(&report).expect("report serializes"))
            .expect("report writes");
    } else {
        println!("{}", serde_json::to_string_pretty(&report).expect("report serializes"));
    }

    let failed = results
        .iter()
        .filter(|r| r.get("pass") != Some(&json!(true)))
        .count();
    if failed > 0 {
        eprintln!("\n{failed} failing case(s) — see report");
        std::process::exit(1);
    }
}
