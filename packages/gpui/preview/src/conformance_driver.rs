//! Generic headless GPUI conformance driver (spec 066, g14.023).
//!
//! Reusable mount, frame, focus, pointer, drag, and keyboard machinery for
//! component cases and primitive substrate probes on GPUI 0.2.2's in-memory
//! test platform (`TestAppContext`, `VisualTestContext`, `TestWindow`).
//! No component identifier, part list, or case corpus lives here.
//!
//! All input goes through GPUI's real dispatch tree: mouse events are
//! hit-tested against the rendered frame, keys walk the focus chain, and the
//! node backend's listeners are the ones that react. No component handler is
//! ever invoked as a test shortcut, and no OS window is created or activated.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_node::Node;
use poodle_render::primitive_probes::ProbeEvidence;
use serde_json::{json, Value};

/// Content coordinates of the fixed-size box mounted nodes are centered in.
pub const MOUNT_BOX_LEFT: f32 = 32.0;
pub const MOUNT_BOX_TOP: f32 = 32.0;
pub const MOUNT_BOX_WIDTH: f32 = 160.0;
pub const MOUNT_BOX_HEIGHT: f32 = 60.0;

pub fn mount_box_center() -> Point<Pixels> {
    point(
        px(MOUNT_BOX_LEFT + MOUNT_BOX_WIDTH / 2.0),
        px(MOUNT_BOX_TOP + MOUNT_BOX_HEIGHT / 2.0),
    )
}

/// The root view: a fixed-size, center-aligned box containing the current
/// node. The root carries its own focus handle so cases can blur whatever the
/// backend holds.
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
        // The window-level overlay host: every pointer press and Escape is
        // routed through the node backend's layer registry (generic — no
        // component identifier here), so overlay dismissal executes through
        // the real event tree. The production preview root uses the same
        // wiring.
        poodle_gpui_node_backend::attach_overlay_host(
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
                ),
        )
    }
}

// ── Headless driver ────────────────────────────────────────────────────────

/// Drives one test-platform window through the real event tree.
///
/// Frames are drawn explicitly (`Window::draw`) because the test platform
/// never requests frames on its own; hit testing and the focus chain read the
/// last painted frame, so every mount, rebuild, and focus change is followed
/// by a draw before input is simulated.
pub struct HeadlessDriver<'a> {
    cx: &'a mut VisualTestContext,
    root: Entity<ConformanceRoot>,
    root_focus: FocusHandle,
}

impl<'a> HeadlessDriver<'a> {
    /// Mount the given node into a fresh test-platform window.
    pub fn new(cx: &'a mut TestAppContext, node: Arc<Mutex<Node>>) -> Self {
        let (root, cx) = cx.add_window_view(|window, cx| {
            let root = ConformanceRoot {
                node: Arc::clone(&node),
                focus: cx.focus_handle(),
            };
            window.refresh();
            root
        });
        let root_focus = cx.update(|_window, cx| root.read(cx).focus.clone());
        let mut driver = Self {
            cx,
            root,
            root_focus,
        };
        driver.draw_frame();
        driver
    }

    /// Swap in a new node and repaint.
    pub fn mount_node(&mut self, node: Arc<Mutex<Node>>) {
        self.root.update(self.cx, |root, cx| {
            root.node = Arc::clone(&node);
            cx.notify();
        });
        self.draw_frame();
    }

    /// Paint one full frame (build, layout, paint, dispatch tree).
    ///
    /// The test platform never requests frames on its own, and gpui reuses
    /// views that were not explicitly invalidated — paint-time side effects
    /// (the node backend's focus canvases) would only run on some frames. The
    /// root view is notified (invalidated) up front so every draw is a full
    /// repaint and the backend observations are deterministic. The overlay
    /// frame boundary (layer registry, bounds, focus queue) is this draw.
    pub fn draw_frame(&mut self) {
        poodle_gpui_node_backend::overlay_frame_begin();
        self.root
            .update(self.cx, |_root, cx| cx.notify());
        self.cx.update(|window, cx| {
            window.refresh();
            let _ = window.draw(cx);
        });
        self.cx.run_until_parked();
        // Focus requests the frame's paint never applied are stale (the
        // target element never appeared).
        poodle_gpui_node_backend::overlay_frame_end();
    }

    /// Drain the executor until every task is parked.
    pub fn drain(&mut self) {
        self.cx.run_until_parked();
    }

    /// Keep painting until the backend owns a focus handle for the element.
    /// The handle is created lazily in the paint pass and attached on the
    /// next build, so two frames are the minimum.
    pub fn wait_for_focus_handle(&mut self, element_id: &str) {
        for _ in 0..16 {
            let ready = poodle_gpui_node_backend::focus_handle_for(element_id).is_some();
            if ready {
                return;
            }
            self.draw_frame();
        }
        panic!("focus handle for `{element_id}` never appeared");
    }

    /// Focus the element through the real backend focus registry.
    pub fn focus_element(&mut self, element_id: &str) {
        self.cx.update(|window, _cx| {
            if let Some(handle) = poodle_gpui_node_backend::focus_handle_for(element_id) {
                handle.focus(window);
            }
        });
        self.draw_frame();
    }

    /// Move focus to the mount root and keep painting until the backend
    /// reports the element as blurred.
    pub fn blur_element_focus(&mut self, element_id: &str) {
        for _ in 0..16 {
            self.cx.update(|window, _cx| {
                window.blur();
            });
            self.draw_frame();
            if poodle_gpui_node_backend::focus_state_for(element_id) == Some(false) {
                return;
            }
        }
        eprintln!("WARN: element `{element_id}` never reported blurred");
    }

    /// Send one platform input through `TestWindow`'s real dispatch callback
    /// (hit testing, focus chain, listeners) and repaint afterwards.
    fn pointer_event(&mut self, event: PlatformInput) {
        match event {
            PlatformInput::MouseDown(down) => self.cx.simulate_event(down),
            PlatformInput::MouseUp(up) => self.cx.simulate_event(up),
            PlatformInput::MouseMove(move_ev) => self.cx.simulate_event(move_ev),
            _ => panic!("pointer_event only takes mouse input"),
        }
        self.cx.run_until_parked();
        self.draw_frame();
    }

    /// Pointer press (left button down) at the given position.
    pub fn pointer_press(&mut self, position: Point<Pixels>) {
        self.pointer_event(PlatformInput::MouseDown(MouseDownEvent {
            position,
            modifiers: Modifiers::none(),
            button: MouseButton::Left,
            click_count: 1,
            first_mouse: false,
        }));
    }

    /// Pointer drag: a move while the left button is held.
    pub fn pointer_drag(&mut self, position: Point<Pixels>) {
        self.pointer_event(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            modifiers: Modifiers::none(),
            pressed_button: Some(MouseButton::Left),
        }));
    }

    /// Pointer release (left button up) at the given position.
    pub fn pointer_release(&mut self, position: Point<Pixels>) {
        self.pointer_event(PlatformInput::MouseUp(MouseUpEvent {
            position,
            modifiers: Modifiers::none(),
            button: MouseButton::Left,
            click_count: 1,
        }));
    }

    /// One press/release at the mount box center.
    pub fn pointer_activate(&mut self) {
        let target = mount_box_center();
        self.pointer_press(target);
        self.pointer_release(target);
    }

    /// Pointer scrub at a fraction along the mount box (0 = left, 1 = right).
    ///
    /// Matches GPUI scrub wiring: mouse-down → Press, move while held →
    /// Drag, mouse-up → Release. The mount box flex-centers the control, so Y
    /// targets the box mid-line (same as Button activation).
    pub fn pointer_scrub_at(&mut self, fraction: f32, phase: &str) {
        let x = MOUNT_BOX_LEFT + fraction.clamp(0.0, 1.0) * MOUNT_BOX_WIDTH;
        let y = MOUNT_BOX_TOP + MOUNT_BOX_HEIGHT / 2.0;
        let target = point(px(x), px(y));
        match phase {
            "press" => self.pointer_press(target),
            "drag" => self.pointer_drag(target),
            _ => self.pointer_release(target),
        }
    }

    /// Focus the element, then send one named keystroke (key down + key up)
    /// through the window's real dispatch tree.
    pub fn keyboard_key(&mut self, element_id: &str, key: &str) {
        self.focus_element(element_id);
        self.dispatch_key_raw(key);
    }

    /// Enter activation on the focused element (keyboard press).
    pub fn keyboard_activate(&mut self, element_id: &str) {
        self.keyboard_key(element_id, "enter");
    }

    /// Send one named keystroke (key down + key up) without moving focus —
    /// the event tree resolves the focus target itself. Used by planted
    /// failures to prove a wrong focus target stays inert.
    ///
    /// An unfocused window — or a focus handle from a previous mount — has an
    /// empty or stale dispatch path, so window-level key handling (Escape →
    /// overlay dismissal) would never fire. The mount host is focused first;
    /// the same guarantee a document-level key listener has on the web.
    pub fn dispatch_key(&mut self, key: &str) {
        self.cx.update(|window, _cx| {
            let handle = self.root_focus.clone();
            handle.focus(window);
        });
        self.dispatch_key_raw(key);
    }

    /// The keystroke half of [`Self::dispatch_key`], with focus untouched —
    /// callers that already focused the target (keyboard activation, tabs
    /// navigation) use this so the mount host never steals focus.
    pub fn dispatch_key_raw(&mut self, key: &str) {
        let keystroke = Keystroke::parse(key).expect("keystroke parses");
        self.cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
        });
        self.cx.simulate_event(KeyUpEvent { keystroke });
        self.cx.run_until_parked();
        self.draw_frame();
    }

    /// Key down without the paired key up — the planted event-order defect.
    pub fn dispatch_key_down_only(&mut self, key: &str) {
        let keystroke = Keystroke::parse(key).expect("keystroke parses");
        self.cx.simulate_event(KeyDownEvent {
            keystroke,
            is_held: false,
        });
        self.cx.run_until_parked();
        self.draw_frame();
    }
}

// ── Report helpers ─────────────────────────────────────────────────────────

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
