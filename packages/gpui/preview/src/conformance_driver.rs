//! Generic GPUI conformance driver (spec 066, g14.002).
//!
//! Reusable window, mount, AppKit event, calibration, and evidence machinery
//! for component cases and primitive substrate probes. No component identifier,
//! part list, or case corpus lives here.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_node::Node;
use poodle_render::primitive_probes::ProbeEvidence;
use serde_json::{json, Value};

/// Exit code surfaced after `App::run` returns.
pub static EXIT_CODE: AtomicU8 = AtomicU8::new(0);

/// Content coordinates of the fixed-size box mounted nodes are centered in.
pub const MOUNT_BOX_LEFT: f32 = 32.0;
pub const MOUNT_BOX_TOP: f32 = 32.0;
pub const MOUNT_BOX_WIDTH: f32 = 160.0;
pub const MOUNT_BOX_HEIGHT: f32 = 60.0;

/// Keycode: Enter = 36 (the keyboard confirm the corpus uses).
pub const KEY_ENTER: u16 = 36;

pub fn mount_box_center() -> Point<Pixels> {
    point(
        px(MOUNT_BOX_LEFT + MOUNT_BOX_WIDTH / 2.0),
        px(MOUNT_BOX_TOP + MOUNT_BOX_HEIGHT / 2.0),
    )
}
/// The root view: a fixed-size, center-aligned box containing the current node.
pub struct ConformanceRoot {
    pub node: Arc<Mutex<Node>>,
    pub focus: FocusHandle,
}

impl Focusable for ConformanceRoot {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for ConformanceRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let node = self.node.lock().expect("node lock").clone();
        div()
            .size_full()
            .track_focus(&self.focus)
            .child(
                div()
                    .w(px(MOUNT_BOX_WIDTH))
                    .h(px(MOUNT_BOX_HEIGHT))
                    .ml(px(MOUNT_BOX_LEFT))
                    .mt(px(MOUNT_BOX_TOP))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(poodle_gpui_node_backend::to_gpui(&node)),
            )
    }
}

// ── AppKit event driver ─────────────────────────────────────────────────────

pub fn post_mouse_event(
    window: &mut Window,
    event_type: objc2_app_kit::NSEventType,
    position: Point<Pixels>,
) {
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

pub fn dispatch_press(window: &mut Window, position: Point<Pixels>) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::MouseMoved, position);
    post_mouse_event(window, NSEventType::LeftMouseDown, position);
}

pub fn dispatch_drag(window: &mut Window, position: Point<Pixels>) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::LeftMouseDragged, position);
}

pub fn dispatch_release(window: &mut Window, position: Point<Pixels>) {
    use objc2_app_kit::NSEventType;
    post_mouse_event(window, NSEventType::LeftMouseUp, position);
}

/// Post one named keystroke through the AppKit queue (keycode-based).
pub fn post_key(key_code: u16) {
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

/// Redraw before the next posted event so hit testing sees the current scene.
pub fn post_frame_flush() {
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

// ── Click calibration ───────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct ClickCalibration {
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

    pub fn apply(&self, target: Point<Pixels>) -> Point<Pixels> {
        point(
            px((f32::from(target.x) - self.offset_x) / self.scale_x),
            px((f32::from(target.y) - self.offset_y) / self.scale_y),
        )
    }
}

pub fn activate_app_and_window(_cx: &mut AsyncWindowContext) {
    activate_app_and_window_ns();
}

fn activate_app_and_window_ns() {
    use objc2::MainThreadMarker;
    use objc2_app_kit::NSApplication;
    if let Some(mtm) = MainThreadMarker::new() {
        let app = NSApplication::sharedApplication(mtm);
        #[allow(deprecated)]
        {
            app.activateIgnoringOtherApps(true);
        }
        let windows = app.windows();
        if let Some(win) = windows.firstObject() {
            win.makeKeyAndOrderFront(None);
        }
    }
}

pub fn window_is_active_and_key(cx: &mut AsyncWindowContext) -> bool {
    cx.update(|_window, _cx| {
        use objc2::MainThreadMarker;
        use objc2_app_kit::NSApplication;
        MainThreadMarker::new()
            .map(|mtm| {
                let app = NSApplication::sharedApplication(mtm);
                let key = app
                    .windows()
                    .firstObject()
                    .map(|win| win.isKeyWindow())
                    .unwrap_or(false);
                app.isActive() && key
            })
            .unwrap_or(false)
    })
    .unwrap_or(false)
}

/// Warm up the window server session and calibrate the click affine transform.
pub async fn warmup_and_calibrate(cx: &mut AsyncWindowContext) -> ClickCalibration {
    use objc2_app_kit::NSEventType;

    cx.background_executor()
        .timer(std::time::Duration::from_millis(600))
        .await;

    for _ in 0..30 {
        if window_is_active_and_key(cx) {
            break;
        }
        activate_app_and_window(cx);
        cx.background_executor()
            .timer(std::time::Duration::from_millis(200))
            .await;
    }

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
    ClickCalibration::solve(p1_posted, seen[0], p2_posted, seen[1]).unwrap_or_else(ClickCalibration::identity)
}

pub fn mount_node(cx: &mut AsyncWindowContext, node: Arc<Mutex<Node>>) {
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
}

pub async fn blur_element_focus(cx: &mut AsyncWindowContext, element_id: &str) {
    for _ in 0..20 {
        cx.update(|window, cx| {
            if let Some(Some(root)) = window.root::<ConformanceRoot>() {
                let root_handle = root.focus_handle(cx);
                window.focus(&root_handle);
                window.refresh();
            }
            activate_app_and_window_ns();
        })
        .ok();
        cx.background_executor()
            .timer(std::time::Duration::from_millis(120))
            .await;
        let blurred = cx
            .update(|_window, _cx| poodle_gpui_node_backend::focus_state_for(element_id))
            .ok()
            .flatten()
            == Some(false);
        if blurred {
            break;
        }
    }
}

pub async fn wait_for_focus_handle(cx: &mut AsyncWindowContext, element_id: &str) {
    for _ in 0..20 {
        let ready = cx
            .update(|_window, _cx| poodle_gpui_node_backend::focus_handle_for(element_id).is_some())
            .unwrap_or(false);
        if ready {
            break;
        }
        activate_app_and_window(cx);
        cx.background_executor()
            .timer(std::time::Duration::from_millis(150))
            .await;
    }
    cx.background_executor()
        .timer(std::time::Duration::from_millis(200))
        .await;
}

pub async fn focus_element(cx: &mut AsyncWindowContext, element_id: &str) {
    cx.update(|window, _cx| {
        if let Some(handle) = poodle_gpui_node_backend::focus_handle_for(element_id) {
            handle.focus(window);
            window.refresh();
        }
    })
    .ok();
    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;
}

pub async fn pointer_activate(cx: &mut AsyncWindowContext, calibration: ClickCalibration) {
    let target = calibration.apply(mount_box_center());
    // One press/release. Callers that need a swallowed-click retry (Button)
    // loop on this helper and break when their trace grows.
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
    cx.background_executor()
        .timer(std::time::Duration::from_millis(700))
        .await;
}

pub async fn keyboard_activate(cx: &mut AsyncWindowContext, element_id: &str) {
    focus_element(cx, element_id).await;
    cx.update(|_window, _cx| post_key(KEY_ENTER)).ok();
    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;
}

/// Arrow-right keycode (macOS virtual key).
pub const KEY_RIGHT: u16 = 124;

pub async fn keyboard_key(cx: &mut AsyncWindowContext, element_id: &str, keycode: u16) {
    focus_element(cx, element_id).await;
    cx.update(|_window, _cx| post_key(keycode)).ok();
    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;
}

/// Pointer scrub at a fraction along the mount box (0 = left, 1 = right).
///
/// Matches GPUI scrub wiring: mouse-down → Press, move while held → Drag,
/// mouse-up → Release. The mount box flex-centers the control, so Y targets
/// the box mid-line (same as Button activation).
pub async fn pointer_scrub_at(
    cx: &mut AsyncWindowContext,
    calibration: ClickCalibration,
    fraction: f32,
    phase: &str,
) {
    let x = MOUNT_BOX_LEFT + fraction.clamp(0.0, 1.0) * MOUNT_BOX_WIDTH;
    let y = MOUNT_BOX_TOP + MOUNT_BOX_HEIGHT / 2.0;
    let target = calibration.apply(point(px(x), px(y)));
    match phase {
        "press" => {
            cx.update(|window, _cx| {
                dispatch_press(window, target);
                window.refresh();
                post_frame_flush();
            })
            .ok();
        }
        "drag" => {
            cx.update(|window, _cx| {
                dispatch_drag(window, target);
                window.refresh();
                post_frame_flush();
            })
            .ok();
        }
        _ => {
            cx.update(|window, _cx| {
                dispatch_release(window, target);
                window.refresh();
                post_frame_flush();
            })
            .ok();
        }
    }
    cx.background_executor()
        .timer(std::time::Duration::from_millis(300))
        .await;
}

pub async fn drain_event_queue(cx: &mut AsyncWindowContext) {
    cx.background_executor()
        .timer(std::time::Duration::from_millis(900))
        .await;
}

// ── Asset source + application shell ────────────────────────────────────────

pub struct ConformanceAssets {
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

pub fn conformance_assets() -> ConformanceAssets {
    ConformanceAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    }
}

pub fn primitive_evidence_report(probes: &[ProbeEvidence]) -> Value {
    json!({
        "schema": "primitive-probe-evidence.v1",
        "runtime": "gpui",
        "probes": probes,
    })
}

pub fn write_or_print_report(out: Option<&PathBuf>, report: &Value) {
    match out {
        Some(path) => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).expect("report parent directory creates");
            }
            std::fs::write(
                path,
                serde_json::to_string_pretty(report).expect("report serializes"),
            )
            .expect("report writes");
        }
        None => {
            println!(
                "{}",
                serde_json::to_string_pretty(report).expect("report serializes")
            );
        }
    }
}

pub fn set_exit_from_probes(probes: &[ProbeEvidence]) {
    let failed = probes.iter().any(|probe| probe.verdict == "fail");
    if failed {
        EXIT_CODE.store(1, Ordering::SeqCst);
    }
}
