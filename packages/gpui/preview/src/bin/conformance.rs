//! Conformance runner for the GPUI runtime (spec 066, g14.001).
//!
//! Opens a real window, mounts the converted Button element, and drives real
//! NSEvents through the AppKit queue (the same calibrated click/key driver
//! shape as the preview's `--click`): hit testing, listener binding, focus,
//! and activation all traverse the actual GPUI path. The runner never calls
//! the node's activation callback directly — a click or Enter keystroke does.
//!
//! Focus is read from the node-backend's focus registry (`focus_state_for`),
//! which records what gpui's focus handles actually did.
//!
//! LOCAL-ONLY: needs a macOS window server, like `test:native-visual`.
//!
//! ```text
//!   cargo run --bin conformance                # all cases, JSON report
//!   cargo run --bin conformance -- --case=button/default
//!   cargo run --bin conformance -- --out=/tmp/gpui.json
//! ```

use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::conformance::{
    assert_events, assert_part, host_activate, observe_tree, InterfaceDoc,
};
use poodle_specs::ButtonSpec;
use serde_json::{json, Value};

#[path = "../conformance_support.rs"]
mod conformance_support;

/// The stable element id the mounted button node carries, so the
/// node-backend's focus registry keys on it.
const BUTTON_ELEMENT_ID: &str = "conformance-button";

/// Content coordinates of the fixed-size box the button is centered in.
const BOX_LEFT: f32 = 32.0;
const BOX_TOP: f32 = 32.0;
const BOX_WIDTH: f32 = 160.0;
const BOX_HEIGHT: f32 = 60.0;

fn box_center() -> Point<Pixels> {
    point(
        px(BOX_LEFT + BOX_WIDTH / 2.0),
        px(BOX_TOP + BOX_HEIGHT / 2.0),
    )
}

/// Exit code surfaced after `App::run` returns.
static EXIT_CODE: AtomicU8 = AtomicU8::new(0);

/// The root view: a fixed-size, center-aligned box containing the current
/// case's converted button element.
struct ConformanceRoot {
    node: Arc<Mutex<Node>>,
}

impl Render for ConformanceRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let node = self.node.lock().expect("node lock").clone();
        div()
            .size_full()
            .child(
                div()
                    .w(px(BOX_WIDTH))
                    .h(px(BOX_HEIGHT))
                    .ml(px(BOX_LEFT))
                    .mt(px(BOX_TOP))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(poodle_gpui_node_backend::to_gpui(&node)),
            )
    }
}

/// One case's host state, shared with the activation handler the backend
/// binds.
struct CaseHost {
    spec: ButtonSpec,
    node: Arc<Mutex<Node>>,
    pressed: Arc<Mutex<Option<bool>>>,
    trace: Arc<Mutex<Vec<String>>>,
    theme: GpuiThemeProvider,
}

impl CaseHost {
    /// The activation handler bound to the real click/key listener path.
    fn make_handler(
        pressed: Arc<Mutex<Option<bool>>>,
        trace: Arc<Mutex<Vec<String>>>,
        toggle_mode: bool,
    ) -> Arc<dyn Fn() + Send + Sync> {
        Arc::new(move || {
            // Mirrors the web host's `pressedChange`-before-`press` order.
            host_activate(
                toggle_mode,
                &mut *pressed.lock().expect("pressed lock"),
                &mut *trace.lock().expect("trace lock"),
            );
        })
    }

    fn rebuild(&mut self, handler: Arc<dyn Fn() + Send + Sync>) {
        if self.spec.pressed.is_some() {
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

/// Observes the current case through the node tree plus the backend focus
/// registry.
fn observe_case(host: &CaseHost, iface: &InterfaceDoc) -> Value {
    let node = host.node.lock().expect("node lock").clone();
    let backend_focus = poodle_gpui_node_backend::focus_state_for(BUTTON_ELEMENT_ID);
    let mut observation = observe_tree("gpui", "button", iface, &node, backend_focus);
    observation["trace"] = json!(host.trace.lock().expect("trace lock").clone());
    observation
}

// ── AppKit event driver (same shape as the preview's `--click` driver) ─────

fn post_mouse_event(window: &mut Window, event_type: objc2_app_kit::NSEventType, position: Point<Pixels>) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::NSPoint;

    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!("click driver: not on the main thread");
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        eprintln!("click driver: no NSWindow to send to");
        return;
    };
    let location = NSPoint {
        x: f64::from(f32::from(position.x)),
        y: f64::from(f32::from(window.viewport_size().height - position.y)),
    };
    let pressure = if event_type == NSEventType::LeftMouseDown {
        1.0
    } else {
        0.0
    };
    let event =
        NSEvent::mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure(
            event_type,
            location,
            NSEventModifierFlags::empty(),
            0.0,
            ns_window.windowNumber(),
            None,
            0,
            1,
            pressure,
        );
    match event {
        Some(event) => app.postEvent_atStart(&event, false),
        None => eprintln!("click driver: NSEvent construction failed"),
    }
}

fn dispatch_press(window: &mut Window, position: Point<Pixels>) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::MouseMoved, position);
    post_mouse_event(window, NSEventType::LeftMouseDown, position);
}

fn dispatch_release(window: &mut Window, position: Point<Pixels>) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::LeftMouseUp, position);
}

/// Post one named keystroke through the AppKit queue (keycode-based; gpui
/// reads the keycode for navigation and activation keys).
fn post_key(key_code: u16) {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventType};
    use objc2_foundation::{NSPoint, NSString};

    let Some(mtm) = MainThreadMarker::new() else {
        eprintln!("key driver: not on the main thread");
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        eprintln!("key driver: no NSWindow to send to");
        return;
    };
    let window_number = ns_window.windowNumber();
    let chars = NSString::from_str("");
    for event_type in [NSEventType::KeyDown, NSEventType::KeyUp] {
        let event = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            event_type,
            NSPoint { x: 0.0, y: 0.0 },
            objc2_app_kit::NSEventModifierFlags::empty(),
            0.0,
            window_number,
            None,
            &chars,
            &chars,
            false,
            key_code,
        );
        match event {
            Some(event) => app.postEvent_atStart(&event, false),
            None => eprintln!("key driver: NSEvent construction failed for keycode {key_code}"),
        }
    }
}

/// Keycode: Enter = 36 (the keyboard confirm the corpus uses).
const KEY_ENTER: u16 = 36;

/// Redraw before the next posted event so hit testing sees the current
/// scene (the preview driver's `post_frame_flush` shape).
fn post_frame_flush() {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_foundation::{NSPoint, NSString};

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let app = NSApplication::sharedApplication(mtm);
    let ns_windows = app.windows();
    let Some(ns_window) = ns_windows.iter().next() else {
        return;
    };
    let chars = NSString::from_str("a");
    let event = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
        NSEventType::KeyUp,
        NSPoint { x: 0.0, y: 0.0 },
        NSEventModifierFlags::empty(),
        0.0,
        ns_window.windowNumber(),
        None,
        &chars,
        &chars,
        false,
        0,
    );
    if let Some(event) = event {
        app.postEvent_atStart(&event, false);
    }
}

// ── Click calibration (the preview driver's two-probe affine solve) ────────

struct ClickCalibration {
    scale_x: f32,
    scale_y: f32,
    offset_x: f32,
    offset_y: f32,
}

impl ClickCalibration {
    fn identity() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    fn solve(
        p1_posted: Point<Pixels>,
        p1_seen: Point<Pixels>,
        p2_posted: Point<Pixels>,
        p2_seen: Point<Pixels>,
    ) -> Option<Self> {
        let dx_posted = f32::from(p2_posted.x - p1_posted.x);
        let dy_posted = f32::from(p2_posted.y - p1_posted.y);
        let dx_seen = f32::from(p2_seen.x - p1_seen.x);
        let dy_seen = f32::from(p2_seen.y - p1_seen.y);
        if dx_posted == 0.0 || dy_posted == 0.0 || dx_seen == 0.0 || dy_seen == 0.0 {
            return None;
        }
        let scale_x = dx_seen / dx_posted;
        let scale_y = dy_seen / dy_posted;
        Some(Self {
            scale_x,
            scale_y,
            offset_x: f32::from(p1_seen.x) - f32::from(p1_posted.x) * scale_x,
            offset_y: f32::from(p1_seen.y) - f32::from(p1_posted.y) * scale_y,
        })
    }

    fn apply(&self, target: Point<Pixels>) -> Point<Pixels> {
        point(
            px((f32::from(target.x) - self.offset_x) / self.scale_x),
            px((f32::from(target.y) - self.offset_y) / self.scale_y),
        )
    }
}

// ── The case driver ────────────────────────────────────────────────────────

struct CaseOutcome {
    case_id: String,
    pass: bool,
    failures: Vec<Value>,
    assertions: Vec<Value>,
    observations: Vec<Value>,
}

async fn drive_cases(
    cx: &mut AsyncWindowContext,
    iface: InterfaceDoc,
    cases: Vec<Value>,
    only: Option<String>,
) -> Vec<CaseOutcome> {
    use objc2_app_kit::NSEventType;

    // Let the first paints land so there is a scene to hit, then activate
    // the app: a script-launched app is not the active app, and macOS
    // swallows its first posted click as an activation instead of
    // delivering it.
    cx.background_executor()
        .timer(std::time::Duration::from_millis(600))
        .await;
    // Activation is not instantaneous and macOS may refuse it once or
    // twice; poll until the app is genuinely active so posted clicks are
    // delivered instead of swallowed as activation beats.
    for _ in 0..20 {
        let active = cx
            .update(|_window, _cx| {
                use objc2::MainThreadMarker;
                use objc2_app_kit::NSApplication;
                MainThreadMarker::new()
                    .map(|mtm| NSApplication::sharedApplication(mtm).isActive())
                    .unwrap_or(false)
            })
            .unwrap_or(false);
        if active {
            break;
        }
        cx.update(|_window, _cx| {
            use objc2::MainThreadMarker;
            use objc2_app_kit::NSApplication;
            if let Some(mtm) = MainThreadMarker::new() {
                let app = NSApplication::sharedApplication(mtm);
                #[allow(deprecated)] { app.activateIgnoringOtherApps(true); }
                let windows = app.windows();
                if let Some(win) = windows.firstObject() {
                    win.makeKeyAndOrderFront(None);
                }
            }
        })
        .ok();
        cx.background_executor()
            .timer(std::time::Duration::from_millis(200))
            .await;
    }

    // Calibrate: two probe moves through the click path, read back where
    // gpui observed them, and solve the affine transform.
    let calibration;
    {
        let p1_posted = point(px(100.0), px(100.0));
        let p2_posted = point(px(500.0), px(400.0));
        let mut seen = [point(px(0.0), px(0.0)); 2];
        for (i, probe) in [p1_posted, p2_posted].into_iter().enumerate() {
            cx.update(|window, _cx| {
                post_mouse_event(window, NSEventType::MouseMoved, probe);
            })
            .ok();
            cx.background_executor()
                .timer(std::time::Duration::from_millis(80))
                .await;
            if let Ok(observed) = cx.update(|window, _cx| window.mouse_position()) {
                seen[i] = observed;
            }
        }
        calibration = ClickCalibration::solve(p1_posted, seen[0], p2_posted, seen[1])
            .unwrap_or_else(ClickCalibration::identity);

    }

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

        let spec = conformance_support::spec_from_fixture(&fixture);
        let toggle_mode = spec.is_toggle_mode();
        let pressed = Arc::new(Mutex::new(spec.pressed));
        let trace = Arc::new(Mutex::new(Vec::<String>::new()));
        let theme =
            GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
        let node = Arc::new(Mutex::new(Node::container()));
        let host = Arc::new(Mutex::new(CaseHost {
            spec,
            node: Arc::clone(&node),
            pressed: Arc::clone(&pressed),
            trace: Arc::clone(&trace),
            theme,
        }));

        // Mount the case's node into the window root and repaint.
        {
            let host = host.lock().expect("host lock");
            let handler = CaseHost::make_handler(Arc::clone(&pressed), Arc::clone(&trace), toggle_mode);
            let initial = host.initial_node(handler);
            *host.node.lock().expect("node lock") = initial;
        }
        cx.update(|window, cx| {
            let Some(Some(root)) = window.root::<ConformanceRoot>() else {
                return;
            };
            root.update(cx, |root, cx| {
                root.node = Arc::clone(&node);
                cx.notify();
            });
            window.refresh();
        })
        .ok();
        // Readiness: the node-backend creates the focus handle in the
        // canvas (paint) pass; drive the case only once the window has
        // actually painted the node. An inactive app can defer painting.
        for _ in 0..20 {
            let ready = cx
                .update(|_window, _cx| {
                    poodle_gpui_node_backend::focus_handle_for(BUTTON_ELEMENT_ID).is_some()
                })
                .unwrap_or(false);
            if ready {
                break;
            }
            cx.update(|_window, _cx| {
                use objc2::MainThreadMarker;
                use objc2_app_kit::NSApplication;
                if let Some(mtm) = MainThreadMarker::new() {
                    let app = NSApplication::sharedApplication(mtm);
                    #[allow(deprecated)] { app.activateIgnoringOtherApps(true); }
                    let windows = app.windows();
                    if let Some(win) = windows.firstObject() {
                        win.makeKeyAndOrderFront(None);
                    }
                }
            })
            .ok();
            cx.background_executor()
                .timer(std::time::Duration::from_millis(150))
                .await;
        }
        cx.background_executor()
            .timer(std::time::Duration::from_millis(200))
            .await;

        let mut failures = Vec::new();
        let mut assertions = Vec::new();
        let mut observations = Vec::new();

        for (index, step) in steps.iter().enumerate() {
            let kind = step.get("kind").and_then(Value::as_str).unwrap_or("");
            match kind {
                "action" => {
                    let name = step.get("name").and_then(Value::as_str).unwrap_or("");
                    let input = step.get("input").and_then(Value::as_str).unwrap_or("pointer");
                    if name == "press" {
                        let target = calibration.apply(box_center());
                        if input == "keyboard" {
                            // Focus through the backend, then Enter — gpui
                            // synthesizes Enter KeyUp → click on the focused
                            // element (the node-backend binds no key handler).
                            cx.update(|window, _cx| {
                                if let Some(handle) =
                                    poodle_gpui_node_backend::focus_handle_for(BUTTON_ELEMENT_ID)
                                {
                                    handle.focus(window);
                                    window.refresh();
                                }
                            })
                            .ok();
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(120))
                                .await;
                            cx.update(|_window, _cx| post_key(KEY_ENTER)).ok();
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(300))
                                .await;
                        } else {
                            let before = trace.lock().expect("trace lock").len();
                            for _click_pass in 0..3 {
                                cx.update(|window, _cx| {
                                    dispatch_press(window, target);
                                    window.refresh();
                                    post_frame_flush();
                                })
                                .ok();
                                cx.background_executor()
                                    .timer(std::time::Duration::from_millis(120))
                                    .await;
                                cx.update(|window, _cx| {
                                    dispatch_release(window, target);
                                    window.refresh();
                                    post_frame_flush();
                                })
                                .ok();
                                // Wait for the queued click to deliver
                                // before judging: a script-launched app's
                                // activation can swallow a posted click even
                                // after the warmup beat; a real user
                                // re-clicks, and so does the driver — once.
                                cx.background_executor()
                                    .timer(std::time::Duration::from_millis(400))
                                    .await;
                                if trace.lock().expect("trace lock").len() > before {
                                    break;
                                }
                            }
                        }
                        // The handler flipped the toggle; rebuild the node so
                        // the pressed state is rendered before observation.
                        if toggle_mode {
                            cx.update(|window, _cx| {
                                let mut host = host.lock().expect("host lock");
                                let handler = CaseHost::make_handler(
                                    Arc::clone(&pressed),
                                    Arc::clone(&trace),
                                    toggle_mode,
                                );
                                host.rebuild(handler);
                                window.refresh();
                            })
                            .ok();
                            cx.background_executor()
                                .timer(std::time::Duration::from_millis(150))
                                .await;
                        }
                    } else if name == "focus" {
                        // Real backend focus: the same FocusHandle the
                        // backend tracks, focused through gpui's own API.
                        // The node-backend observes it both ways.
                        cx.update(|window, _cx| {
                            if let Some(handle) =
                                poodle_gpui_node_backend::focus_handle_for(BUTTON_ELEMENT_ID)
                            {
                                handle.focus(window);
                                window.refresh();
                            }
                        })
                        .ok();
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(300))
                            .await;
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
                    observations.push(observation.clone());
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

        let final_observation = cx
            .update(|_window, _cx| {
                let host = host.lock().expect("host lock");
                observe_case(&host, &iface)
            })
            .unwrap_or_else(|_| json!({}));
        observations.push(final_observation);

        // Let the queue drain before the next case mounts: a queued key
        // event from this case (e.g. Enter) must not activate the next
        // case's node, which shares the element id and any focused state.
        cx.background_executor()
            .timer(std::time::Duration::from_millis(400))
            .await;

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let only: Option<String> = args
        .iter()
        .find_map(|a| a.strip_prefix("--case=").map(str::to_owned));
    let out: Option<PathBuf> = args
        .iter()
        .find_map(|a| a.strip_prefix("--out=").map(PathBuf::from));

    // Completion gate: the component must be registered in the GPUI registry.
    {
        let registered = conformance_support_registry_has_button();
        if !registered {
            eprintln!("completion: button registration missing from the GPUI registry");
            std::process::exit(1);
        }
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

    let outcomes: Arc<Mutex<Vec<CaseOutcome>>> = Arc::new(Mutex::new(Vec::new()));

    struct ConformanceAssets {
        base: PathBuf,
    }

    impl AssetSource for ConformanceAssets {
        fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
            let full_path = path.strip_prefix("assets/icons/").map_or_else(
                || self.base.join(path),
                |name| self.base.join("../../render/assets/icons").join(name),
            );
            match std::fs::read(&full_path) {
                Ok(data) => Ok(Some(std::borrow::Cow::Owned(data))),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(e.into()),
            }
        }

        fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
            let full_path = if path == "assets/icons" {
                self.base.join("../../render/assets/icons")
            } else {
                self.base.join(path)
            };
            match std::fs::read_dir(&full_path) {
                Ok(entries) => Ok(entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|e| e.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
                Err(e) => Err(e.into()),
            }
        }
    }

    let assets = ConformanceAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };

    let outcomes_in_run = Arc::clone(&outcomes);
    Application::new().with_assets(assets).run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(600.0)), cx);
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let window_node = Arc::new(Mutex::new(Node::container()));
                let root_entity = cx.new(|_cx| ConformanceRoot { node: Arc::clone(&window_node) });
                let iface = iface.clone();
                let cases = case_list.clone();
                let only = only.clone();
                window
                    .spawn(cx, async move |cx| {
                        let results = drive_cases(cx, iface, cases, only).await;
                        let report = json!({
                            "runtime": "gpui",
                            "component": component,
                            "results": results.iter().map(|o| json!({
                                "caseId": o.case_id,
                                "pass": o.pass,
                                "failures": o.failures,
                                "assertions": o.assertions,
                                "observations": o.observations,
                            })).collect::<Vec<_>>(),
                        });
                        let failed = results.iter().filter(|o| !o.pass).count();
                        if failed > 0 {
                            EXIT_CODE.store(1, Ordering::SeqCst);
                        }
                        *outcomes_in_run.lock().expect("outcomes lock") = results;
                        match &out {
                            Some(path) => {
                                std::fs::write(
                                    path,
                                    serde_json::to_string_pretty(&report).expect("report serializes"),
                                )
                                .expect("report writes");
                            }
                            None => {
                                println!(
                                    "{}",
                                    serde_json::to_string_pretty(&report).expect("report serializes")
                                );
                            }
                        }
                        if failed > 0 {
                            eprintln!("\n{failed} failing case(s) — see report");
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
        .count();
    let exit_code = EXIT_CODE.load(Ordering::SeqCst);
    if failed > 0 || exit_code != 0 {
        std::process::exit(1);
    }
}

fn conformance_support_registry_has_button() -> bool {
    #[path = "../component_registry.rs"]
    #[allow(dead_code)]
    mod component_registry;
    component_registry::find_component("button").is_some()
}
