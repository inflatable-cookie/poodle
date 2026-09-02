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
    /// The mount host's own drag controller. Production hosts wrap their root
    /// in exactly one provider; a regression that needs two independent
    /// sessions mounts two provider elements instead (see
    /// `HeadlessContent::Element`).
    pub drag: poodle_gpui_node_backend::DragDropController,
    /// This window's provider census, exactly as a production root wires it.
    /// One per window and never shared: two mounted windows own two hosts,
    /// which is what stops one window's frame from sweeping the other's
    /// controllers.
    pub drag_host: poodle_gpui_node_backend::DragDropWindowHost,
}

impl Focusable for HeadlessRoot {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for HeadlessRoot {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Same production frame lifetime as PreviewRoot: begin at render,
        // end after this effect cycle so a removed continuous-value host
        // cancels in the removal frame without a next-frame delay.
        poodle_gpui_node_backend::overlay_frame_begin();
        cx.defer(|_cx| {
            poodle_gpui_node_backend::overlay_frame_end();
        });
        // The same per-frame reset the production root performs
        // (`main.rs`): without it, generated element ids mint fresh every
        // frame, and an unstamped node's identity — click state, focus
        // handle, ring registry key — changes between frames.
        poodle_gpui_node_backend::reset_element_ids();
        // The window-level overlay host: every pointer press and Escape is
        // routed through the node backend's layer registry (generic — no
        // component identifier here), so overlay dismissal executes through
        // the real event tree. The production preview root uses the same
        // wiring, so dismissal behaves identically here and in the real app.
        // The drag provider wraps the overlay host, exactly as the production
        // preview root does: every source and target built below belongs to
        // this host's controller, and release, Escape, keyboard, and the
        // rebuild sweep run through it rather than a backend global.
        let box_width = self.box_width;
        let box_height = self.box_height;
        let focus = self.focus.clone();
        // The node tree is converted INSIDE the provider closure: drag sources
        // and drop targets register while this controller is current, and a
        // conversion that happened before the push would register with nobody.
        let content = match &self.content {
            HeadlessContent::Node(node) => {
                let node = Arc::clone(node);
                HeadlessContent::Node(node)
            }
            HeadlessContent::Element(build) => HeadlessContent::Element(Rc::clone(build)),
        };
        let drag_host = self.drag_host.clone();
        poodle_gpui_node_backend::drag_drop_window_host(&drag_host, || {
            poodle_gpui_node_backend::drag_drop_provider(&self.drag, || {
                let content = match &content {
                    HeadlessContent::Node(node) => {
                        let node = node.lock().expect("node lock").clone();
                        poodle_gpui_node_backend::to_gpui(&node)
                    }
                    HeadlessContent::Element(build) => build(),
                };
                poodle_gpui_node_backend::attach_overlay_host(
                    div().size_full().track_focus(&focus).child(
                        div()
                            .w(px(box_width))
                            .h(px(box_height))
                            .ml(px(MOUNT_BOX_LEFT))
                            .mt(px(MOUNT_BOX_TOP))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(content),
                    ),
                )
            })
        })
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
    painted_frames: usize,
    input_dispatches: usize,
}

/// Evidence that this driver actually painted and dispatched input through a
/// mounted GPUI tree. The fields stay crate-private so receipt emitters cannot
/// manufacture production-path observation tokens.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MountedObservation {
    painted_frames: usize,
    input_dispatches: usize,
}

impl MountedObservation {
    pub(crate) fn is_valid(self) -> bool {
        self.painted_frames > 0 && self.input_dispatches > 0
    }
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
                drag: poodle_gpui_node_backend::DragDropController::new(),
                drag_host: poodle_gpui_node_backend::DragDropWindowHost::new(),
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
            painted_frames: 0,
            input_dispatches: 0,
        };
        driver.draw_frame();
        driver
    }

    /// Mount an element factory when a regression owns runtime state outside
    /// the renderer-neutral node tree (scroll handles, for example).
    pub fn new_element(cx: &'a mut TestAppContext, build: Rc<dyn Fn() -> AnyElement>) -> Self {
        Self::new_element_in_box(cx, build, MOUNT_BOX_WIDTH, MOUNT_BOX_HEIGHT)
    }

    /// Mount an element factory in a box of the given size.
    pub fn new_element_in_box(
        cx: &'a mut TestAppContext,
        build: Rc<dyn Fn() -> AnyElement>,
        box_width: f32,
        box_height: f32,
    ) -> Self {
        let (root, cx) = cx.add_window_view(|window, cx| {
            let root = HeadlessRoot {
                content: HeadlessContent::Element(build),
                focus: cx.focus_handle(),
                box_width,
                box_height,
                drag: poodle_gpui_node_backend::DragDropController::new(),
                drag_host: poodle_gpui_node_backend::DragDropWindowHost::new(),
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
            painted_frames: 0,
            input_dispatches: 0,
        };
        driver.draw_frame();
        driver
    }

    /// The mount host's drag controller — the one every source and target in
    /// a mounted node tree registers with.
    pub fn drag(&mut self) -> poodle_gpui_node_backend::DragDropController {
        self.cx
            .update(|_window, cx| self.root.read(cx).drag.clone())
    }

    /// This window's provider census — the seam that notices an unmounted
    /// provider and stops the runtime's own drag on its behalf.
    pub fn drag_host(&mut self) -> poodle_gpui_node_backend::DragDropWindowHost {
        self.cx
            .update(|_window, cx| self.root.read(cx).drag_host.clone())
    }

    /// Whether GPUI itself still believes a drag is in flight.
    ///
    /// Distinct from the semantic phase on purpose: a session can reach idle
    /// with registries empty while the runtime keeps painting its own drag,
    /// and that gap is exactly what the provider-unmount proof is about.
    pub fn has_active_native_drag(&mut self) -> bool {
        self.cx.update(|_window, cx| cx.has_active_drag())
    }

    /// Run one closure with a real `App`, without drawing.
    ///
    /// For the cases that install or replace window-level wiring mid-test,
    /// where drawing would confound what the test is measuring.
    pub fn update_app<R>(&mut self, body: impl FnOnce(&mut gpui::App) -> R) -> R {
        self.cx.update(|_window, cx| body(cx))
    }

    /// Advance the test executor without holding the app's RefCell borrow.
    /// Async window tasks must be able to re-enter the app when their timer
    /// wakes; advancing through `update_app` would make that re-entry fail.
    pub fn advance_clock(&mut self, duration: std::time::Duration) {
        self.cx.background_executor.advance_clock(duration);
    }

    pub fn with_window<R>(&mut self, body: impl FnOnce(&mut Window, &mut gpui::App) -> R) -> R {
        self.cx.update(|window, cx| body(window, cx))
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
        self.paint_frame();
    }

    /// Draw without notifying the root or refreshing the window first.
    /// Animation tests use this to prove the production scheduler performed
    /// the invalidation rather than letting the harness supply it.
    pub fn draw_if_invalidated(&mut self) {
        self.cx.update(|window, cx| {
            let _ = window.draw(cx);
        });
        self.cx.run_until_parked();
    }

    /// Same production frame lifetime as the preview root: `overlay_frame_begin`
    /// during render and `overlay_frame_end` deferred to the end of this cycle.
    pub fn draw_preview_frame(&mut self) {
        self.paint_frame();
    }

    fn paint_frame(&mut self) {
        self.painted_frames += 1;
        self.root.update(self.cx, |_root, cx| cx.notify());
        self.cx.update(|window, cx| {
            window.refresh();
            let _ = window.draw(cx);
        });
        self.cx.run_until_parked();
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
        self.input_dispatches += 1;
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
        self.input_dispatches += 1;
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

    /// One key **press**, with no matching release. Focus is untouched.
    ///
    /// The pair below exists because press and release are separate dispatched
    /// events with separate state: GPUI synthesizes Enter/Space clicks on
    /// key-up, so anything that suppresses that has to survive whatever
    /// arrives between the two.
    pub fn dispatch_key_press(&mut self, key: &str) {
        self.input_dispatches += 1;
        let keystroke = Keystroke::parse(key).expect("keystroke parses");
        self.cx.simulate_event(KeyDownEvent {
            keystroke,
            is_held: false,
        });
        self.cx.run_until_parked();
        self.draw_frame();
    }

    /// One key **release**, with no preceding press.
    pub fn dispatch_key_release(&mut self, key: &str) {
        self.input_dispatches += 1;
        let keystroke = Keystroke::parse(key).expect("keystroke parses");
        self.cx.simulate_event(KeyUpEvent { keystroke });
        self.cx.run_until_parked();
        self.draw_frame();
    }

    /// The keystroke half of [`Self::dispatch_key`], with focus untouched —
    /// callers that already focused the target use this so the mount host
    /// never steals focus.
    pub fn dispatch_key_raw(&mut self, key: &str) {
        self.input_dispatches += 2;
        let keystroke = Keystroke::parse(key).expect("keystroke parses");
        self.cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
        });
        self.cx.simulate_event(KeyUpEvent { keystroke });
        self.cx.run_until_parked();
        self.draw_frame();
    }

    /// Dispatch a harmless key without the driver's forced repaint. GPUI's
    /// event path draws first only when production code already invalidated
    /// the window, making this a scheduler-invalidation oracle.
    pub fn dispatch_probe_key(&mut self, key: &str) {
        self.input_dispatches += 2;
        let keystroke = Keystroke::parse(key).expect("keystroke parses");
        self.cx.simulate_event(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
        });
        self.cx.simulate_event(KeyUpEvent { keystroke });
        self.cx.run_until_parked();
    }

    /// Return the private observation token used by the receipt emitter. A
    /// receipt can only be requested from a driver that has painted a frame
    /// and sent real input through the GPUI test platform.
    pub(crate) fn mounted_observation(&self) -> MountedObservation {
        MountedObservation {
            painted_frames: self.painted_frames,
            input_dispatches: self.input_dispatches,
        }
    }
}
