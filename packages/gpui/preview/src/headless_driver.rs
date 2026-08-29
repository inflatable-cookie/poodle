//! Generic headless GPUI test driver (g14.023, retained by g14.021).
//!
//! Reusable mount, frame, focus, pointer, drag, and keyboard machinery for
//! native regressions on GPUI 0.2.2's in-memory test platform
//! (`TestAppContext`, `VisualTestContext`, `TestWindow`). No component
//! identifier, part list, or fixture corpus lives here.
//!
//! All input goes through GPUI's real dispatch tree: mouse events are
//! hit-tested against the rendered frame, keys walk the focus chain, and the
//! node backend's listeners are the ones that react. No component handler is
//! ever invoked as a test shortcut, and no OS window is created or activated.
//!
//! This is infrastructure the rejected conformance pilot paid for and
//! `g14.008` ruled worth keeping. It is not a parity architecture: it mounts a
//! `poodle-node` tree and drives real input at it.

use std::rc::Rc;
use std::sync::{Arc, Mutex};

use gpui::*;
use poodle_node::Node;

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
/// node. The root carries its own focus handle so tests can blur whatever the
/// backend holds.
enum HeadlessContent {
    Node(Arc<Mutex<Node>>),
    Element(Rc<dyn Fn() -> AnyElement>),
}

pub struct HeadlessRoot {
    content: HeadlessContent,
    pub focus: FocusHandle,
    box_width: f32,
    box_height: f32,
}

impl Focusable for HeadlessRoot {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for HeadlessRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // The same per-frame reset the production root performs
        // (`main.rs`): without it, generated element ids mint fresh every
        // frame, and an unstamped node's identity — click state, focus
        // handle, ring registry key — changes between frames.
        poodle_gpui_node_backend::reset_element_ids();
        let content = match &self.content {
            HeadlessContent::Node(node) => {
                let node = node.lock().expect("node lock").clone();
                poodle_gpui_node_backend::to_gpui(&node)
            }
            HeadlessContent::Element(build) => build(),
        };
        // The window-level overlay host: every pointer press and Escape is
        // routed through the node backend's layer registry (generic — no
        // component identifier here), so overlay dismissal executes through
        // the real event tree. The production preview root uses the same
        // wiring, so dismissal behaves identically here and in the real app.
        poodle_gpui_node_backend::attach_overlay_host(
            div().size_full().track_focus(&self.focus).child(
                div()
                    .w(px(self.box_width))
                    .h(px(self.box_height))
                    .ml(px(MOUNT_BOX_LEFT))
                    .mt(px(MOUNT_BOX_TOP))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(content),
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
    root: Entity<HeadlessRoot>,
    root_focus: FocusHandle,
    box_width: f32,
    box_height: f32,
}

impl<'a> HeadlessDriver<'a> {
    /// Mount the given node into a fresh test-platform window.
    pub fn new(cx: &'a mut TestAppContext, node: Arc<Mutex<Node>>) -> Self {
        Self::new_in_box(cx, node, MOUNT_BOX_WIDTH, MOUNT_BOX_HEIGHT)
    }

    /// Mount the node in a box of the given size. Vertical Slider needs a
    /// taller host than the default 160×60 fixture.
    pub fn new_in_box(
        cx: &'a mut TestAppContext,
        node: Arc<Mutex<Node>>,
        box_width: f32,
        box_height: f32,
    ) -> Self {
        let (root, cx) = cx.add_window_view(|window, cx| {
            let root = HeadlessRoot {
                content: HeadlessContent::Node(Arc::clone(&node)),
                focus: cx.focus_handle(),
                box_width,
                box_height,
            };
            window.refresh();
            root
        });
        let root_focus = cx.update(|_window, cx| root.read(cx).focus.clone());
        let mut driver = Self {
            cx,
            root,
            root_focus,
            box_width,
            box_height,
        };
        driver.draw_frame();
        driver
    }

    /// Mount an element factory when a regression owns runtime state outside
    /// the renderer-neutral node tree (scroll handles, for example).
    pub fn new_element(cx: &'a mut TestAppContext, build: Rc<dyn Fn() -> AnyElement>) -> Self {
        let (root, cx) = cx.add_window_view(|window, cx| {
            let root = HeadlessRoot {
                content: HeadlessContent::Element(build),
                focus: cx.focus_handle(),
                box_width: MOUNT_BOX_WIDTH,
                box_height: MOUNT_BOX_HEIGHT,
            };
            window.refresh();
            root
        });
        let root_focus = cx.update(|_window, cx| root.read(cx).focus.clone());
        let mut driver = Self {
            cx,
            root,
            root_focus,
            box_width: MOUNT_BOX_WIDTH,
            box_height: MOUNT_BOX_HEIGHT,
        };
        driver.draw_frame();
        driver
    }

    /// Swap in a new node and repaint.
    pub fn mount_node(&mut self, node: Arc<Mutex<Node>>) {
        self.root.update(self.cx, |root, cx| {
            root.content = HeadlessContent::Node(Arc::clone(&node));
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
        self.root.update(self.cx, |_root, cx| cx.notify());
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

    /// Move focus to the next tab stop through the window's real traversal —
    /// the native counterpart of pressing Tab, with no pointer involved.
    pub fn focus_next_tab_stop(&mut self) {
        self.cx.update(|window, _cx| {
            window.focus_next();
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
        self.pointer_press_details(position, 1, Modifiers::none());
    }

    /// Pointer press with an explicit click count and modifiers.
    pub fn pointer_press_details(
        &mut self,
        position: Point<Pixels>,
        click_count: usize,
        modifiers: Modifiers,
    ) {
        self.pointer_event(PlatformInput::MouseDown(MouseDownEvent {
            position,
            modifiers,
            button: MouseButton::Left,
            click_count,
            first_mouse: false,
        }));
    }

    /// Pointer drag: a move while the left button is held.
    pub fn pointer_drag(&mut self, position: Point<Pixels>) {
        self.pointer_drag_details(position, Modifiers::none());
    }

    /// Pointer drag with modifiers (Shift for fine movement).
    pub fn pointer_drag_details(&mut self, position: Point<Pixels>, modifiers: Modifiers) {
        self.pointer_event(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            modifiers,
            pressed_button: Some(MouseButton::Left),
        }));
    }

    /// Pointer hover: a move with no button held.
    pub fn pointer_hover(&mut self, position: Point<Pixels>) {
        self.pointer_event(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            modifiers: Modifiers::none(),
            pressed_button: None,
        }));
    }

    /// Pointer release (left button up) at the given position.
    pub fn pointer_release(&mut self, position: Point<Pixels>) {
        self.pointer_release_details(position, 1, Modifiers::none());
    }

    /// Pointer release with an explicit click count and modifiers.
    pub fn pointer_release_details(
        &mut self,
        position: Point<Pixels>,
        click_count: usize,
        modifiers: Modifiers,
    ) {
        self.pointer_event(PlatformInput::MouseUp(MouseUpEvent {
            position,
            modifiers,
            button: MouseButton::Left,
            click_count,
        }));
    }

    /// One press/release at the mount box center.
    pub fn pointer_activate(&mut self) {
        let target = mount_box_center();
        self.pointer_press(target);
        self.pointer_release(target);
    }

    /// One press/release at a fraction along the mount box (0 = left, 1 = right).
    pub fn pointer_activate_at(&mut self, fraction: f32) {
        let x = MOUNT_BOX_LEFT + fraction.clamp(0.0, 1.0) * self.box_width;
        let y = MOUNT_BOX_TOP + self.box_height / 2.0;
        let target = point(px(x), px(y));
        self.pointer_press(target);
        self.pointer_release(target);
    }

    /// One press/release at the last-painted bounds of a named element.
    pub fn pointer_activate_id(&mut self, element_id: &str) {
        match poodle_gpui_node_backend::bounds_for(element_id) {
            Some(bounds) => {
                self.pointer_press(bounds.center());
                self.pointer_release(bounds.center());
            }
            None => self.pointer_activate_at(0.92),
        }
    }

    /// Scroll the mounted box through GPUI's real wheel dispatch.
    pub fn scroll_vertical(&mut self, delta_y: f32) {
        self.scroll_vertical_at(mount_box_center(), delta_y);
    }

    /// Wheel at a named element's painted center, or the mount box if it has
    /// no bounds yet.
    pub fn scroll_vertical_id(&mut self, element_id: &str, delta_y: f32) {
        let position = poodle_gpui_node_backend::bounds_for(element_id)
            .map(|bounds| bounds.center())
            .unwrap_or_else(mount_box_center);
        self.scroll_vertical_at(position, delta_y);
    }

    fn scroll_vertical_at(&mut self, position: Point<Pixels>, delta_y: f32) {
        self.cx.simulate_event(ScrollWheelEvent {
            position,
            delta: ScrollDelta::Pixels(point(px(0.0), px(delta_y))),
            ..Default::default()
        });
        self.cx.run_until_parked();
        self.draw_frame();
    }

    /// Pointer scrub at a fraction along the mount box (0 = left, 1 = right).
    ///
    /// Matches GPUI scrub wiring: mouse-down → Press, move while held →
    /// Drag, mouse-up → Release. The mount box flex-centers the control, so Y
    /// targets the box mid-line (same as Button activation).
    pub fn pointer_scrub_at(&mut self, fraction: f32, phase: &str) {
        let x = MOUNT_BOX_LEFT + fraction.clamp(0.0, 1.0) * self.box_width;
        let y = MOUNT_BOX_TOP + self.box_height / 2.0;
        let target = point(px(x), px(y));
        match phase {
            "press" => self.pointer_press(target),
            "drag" => self.pointer_drag(target),
            _ => self.pointer_release(target),
        }
    }

    /// Pointer scrub along a vertical mount box (0 = bottom, 1 = top).
    pub fn pointer_scrub_vertical_at(&mut self, fraction: f32, phase: &str) {
        let x = MOUNT_BOX_LEFT + self.box_width / 2.0;
        let y = MOUNT_BOX_TOP + self.box_height * (1.0 - fraction.clamp(0.0, 1.0));
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
    /// the event tree resolves the focus target itself.
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
    /// callers that already focused the target use this so the mount host
    /// never steals focus.
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
}
