//! Conformance runner for the Jetstream runtime (spec 066, g14.001).
//!
//! Builds the real Button node from each case fixture, renders it through
//! the real backend (`jetstream_poodle::to_js_el` → `GameUi`), dispatches
//! real pointer and keyboard input through `GameUi::process_input`, observes
//! the node tree plus the backend's focus projection, evaluates the case
//! corpus, and emits `component-observation.v1` results.
//!
//! ```text
//!   cargo run --bin conformance                # all cases, JSON report
//!   cargo run --bin conformance -- --case=button/default
//!   cargo run --bin conformance -- --out /tmp/jetstream.json
//! ```
//!
//! Needs the sibling jetstream repo (same constraint as `snap` / `a11y`).

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use jetstream_platform::{KeyCode, MouseButton, PlatformEvent};

use jetstream_ui::{ui_input_context, GameUi, UiEvent};
use jetstream_input::InputSystem;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{evaluate_steps, observe_tree, NativeHarness};
use poodle_specs::ButtonSpec;
use poodle_jetstream_preview::{conformance_support, nel};
use serde_json::{json, Value};


/// Host state behind the main mutex. The trace and pressed state live on
/// separate locks so the activation handler — which fires synchronously
/// inside a backend dispatch — never re-locks the host mutex.
struct HostState {
    spec: ButtonSpec,
    node: Node,
    theme: JetstreamThemeProvider,
    ui: GameUi,
    backend_focused: bool,
    /// The activation handler, bound once per case against the shared state.
    handler: Option<Arc<dyn Fn() + Send + Sync>>,
}

struct HostLocks {
    state: Arc<Mutex<HostState>>,
    pressed: Arc<Mutex<Option<bool>>>,
    trace: Arc<Mutex<Vec<String>>>,
}

impl HostLocks {
    /// Applies the activation handler's toggle, rebuilds the node with the
    /// new spec, and re-renders through the backend. Never called from
    /// inside a backend dispatch.
    fn apply_toggle_and_rebuild(&self, toggle_mode: bool) {
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
        host.render_into_backend();
    }
}

impl HostState {
    fn render_into_backend(&mut self) {
        let el = nel::div().w(480.0).h(320.0).p(24.0).child(self.node.clone());
        let js_el = jetstream_poodle::to_js_el(&el.0);
        self.ui = GameUi::new(480.0, 320.0);
        self.ui.active = true;
        self.ui.render_immediate(&js_el);
        self.backend_focused = self.ui.focus.focused().is_some();
    }

    /// The button's bounds in the backend tree (its accessibility node).
    fn button_bounds(&self) -> Option<(f32, f32, f32, f32)> {
        let update = self.ui.accessibility_update()?;
        for (_, node) in &update.nodes {
            if node.role() == jetstream_ui::accesskit::Role::Button {
                let bounds = node.bounds()?;
                return Some((
                    bounds.x0 as f32,
                    bounds.y0 as f32,
                    bounds.x1 as f32,
                    bounds.y1 as f32,
                ));
            }
        }
        None
    }

    /// Dispatches a real pointer press/release at the button's centre.
    fn pointer_press(&mut self) {
        let Some((x0, y0, x1, y1)) = self.button_bounds() else {
            return;
        };
        let cx = (x0 + x1) / 2.0;
        let cy = (y0 + y1) / 2.0;
        let mut input = InputSystem::new();
        input.add_context(ui_input_context());
        input.push_context("game_ui");
        input.process_events(&[PlatformEvent::MouseButton {
            button: MouseButton::Left,
            pressed: true,
        }]);
        self.ui.process_input(&input, cx, cy);
        input.process_events(&[PlatformEvent::MouseButton {
            button: MouseButton::Left,
            pressed: false,
        }]);
        self.ui.process_input(&input, cx, cy);
    }

    /// Dispatches a real keyboard confirm (focus the button, press ENTER).
    /// The backend emits `UiEvent::Activated` for the confirm path (it does
    /// not re-fire the click handler, unlike pointer), so the caller must
    /// react to the event — the same contract a real host has.
    fn keyboard_press(&mut self) -> Vec<UiEvent> {
        self.focus();
        let mut input = InputSystem::new();
        input.add_context(ui_input_context());
        input.push_context("game_ui");
        input.process_events(&[PlatformEvent::KeyPressed {
            key: KeyCode::ENTER,
            repeat: false,
        }]);
        self.ui.process_input(&input, 0.0, 0.0)
    }

    fn focus(&mut self) {
        self.ui.navigate(jetstream_ui::NavDirection::Next);
        self.backend_focused = self.ui.focus.focused().is_some();
    }
}

struct JetstreamHarness {
    locks: HostLocks,
    toggle_mode: bool,
}

impl NativeHarness for JetstreamHarness {
    fn runtime(&self) -> &'static str {
        "jetstream"
    }

    fn component(&self) -> &'static str {
        "button"
    }

    fn observe(&self) -> Value {
        let state = self.locks.state.lock().expect("host state lock");
        let mut observation = observe_tree("jetstream", "button", &state.node);
        if let Some(parts) = observation.get_mut("parts").and_then(|p| p.get_mut("root")) {
            if let Some(states) = parts.get_mut("states") {
                if let Some(map) = states.as_object_mut() {
                    map.insert("focused".to_owned(), json!(state.backend_focused));
                }
            }
        }
        observation["trace"] = json!(self.locks.trace.lock().expect("trace lock").clone());
        observation
    }

    fn press(&mut self, _part: &str, input: &str) {
        let activated = {
            let mut state = self.locks.state.lock().expect("host state lock");
            if input == "keyboard" {
                state
                    .keyboard_press()
                    .iter()
                    .any(|e| matches!(e, UiEvent::Activated(_)))
            } else {
                state.pointer_press();
                false
            }
        };
        // The real activation path ran. Pointer fires the node's click
        // handler synchronously (which logs press and flips the toggle);
        // keyboard emits `Activated` for the host to handle — invoke the
        // same activation handler here.
        if activated {
            let handler = self
                .locks
                .state
                .lock()
                .expect("host state lock")
                .handler
                .clone()
                .expect("handler bound");
            handler();
        }
        if self.toggle_mode {
            self.locks.apply_toggle_and_rebuild(self.toggle_mode);
        }
    }

    fn focus(&mut self, _part: &str) {
        self.locks
            .state
            .lock()
            .expect("host state lock")
            .focus();
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

    let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);

    // Completion gate: the component must be registered AND have a live
    // specimen renderer — a missing registration fails the run.
    {
        let state = poodle_jetstream_preview::app_state::AppState::new();
        let registered = poodle_jetstream_preview::component_registry::ALL_COMPONENTS
            .iter()
            .any(|entry| entry.slug == "button" && entry.has_specimen);
        let specimen_renders =
            poodle_jetstream_preview::specimens::render_specimen("button", &theme, &state).is_some();
        if !registered || !specimen_renders {
            eprintln!(
                "completion: button registration missing (registered={registered}, specimen={specimen_renders})"
            );
            std::process::exit(1);
        }
    }

    let mut results = Vec::new();
    for case in case_list {
        let case_id = case.get("id").and_then(Value::as_str).unwrap_or("?");
        if let Some(only) = &only
            && only != case_id
        {
            continue;
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
            ui: GameUi::new(1.0, 1.0),
            backend_focused: false,
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
            s.render_into_backend();
        }

        let mut harness = JetstreamHarness {
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
        "runtime": "jetstream",
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
