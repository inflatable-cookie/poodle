//! Focused headless GPUI regressions (g14.021).
//!
//! The rejected conformance pilot (`g14.008`) paid for one thing worth
//! keeping: a GPUI board that runs on the in-memory test platform
//! (`TestAppContext` / `VisualTestContext` / `TestWindow`) through the real
//! render, backend, and event tree — no OS window, no application activation,
//! no stolen keyboard focus, about a tenth of a second.
//!
//! What runs here is the set of backend claims the pilot caught and nothing
//! else can own: the corpus, the normalized observation plane, and the planted
//! failures that only tested the harness are gone. Component-level claims live
//! beside their components (`cargo test -p poodle-render`, the Svelte/React
//! component boards); this file is for defects that only appear once a node
//! tree is mounted in a real window and driven with real input.

#![recursion_limit = "512"]

use std::sync::{Arc, Mutex};

// Explicit import only: `use gpui::*` would glob in gpui's `test` proc macro
// and shadow the built-in `#[test]` attribute (gpui-macros 0.2.2's `test`
// crashes on current rustc).
use gpui::TestAppContext;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_specs::{PopoverSpec, RangeSliderSpec};

#[path = "../src/headless_driver.rs"]
mod headless_driver;

use headless_driver::HeadlessDriver;

/// The element id every single-node fixture mounts under.
const FIXTURE_ID: &str = "headless-fixture";

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

fn theme() -> GpuiThemeProvider {
    GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE)
}

fn button_node(
    spec: poodle_specs::ButtonSpec,
    handler: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let mut node = poodle_render::button(&spec, &theme(), handler);
    node.id = Some(FIXTURE_ID.to_owned());
    node
}

fn counting_handler() -> (Arc<dyn Fn() + Send + Sync>, Arc<Mutex<usize>>) {
    let count = Arc::new(Mutex::new(0usize));
    let sink = Arc::clone(&count);
    let handler: Arc<dyn Fn() + Send + Sync> =
        Arc::new(move || *sink.lock().expect("count lock") += 1);
    (handler, count)
}

// ── Driver infrastructure ──────────────────────────────────────────────────

/// The driver mounts through the real backend and reads real focus state.
/// Without this the rest of the file proves nothing: every claim below is only
/// meaningful if the backend — not the test — is the thing reacting.
#[test]
fn the_driver_mounts_and_tracks_real_backend_focus() {
    run_headless(|cx| {
        let node = Arc::new(Mutex::new(button_node(
            poodle_specs::ButtonSpec::new().with_label("focus"),
            None,
        )));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.focus_element(FIXTURE_ID);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(FIXTURE_ID),
            Some(true)
        );

        driver.blur_element_focus(FIXTURE_ID);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(FIXTURE_ID),
            Some(false)
        );
    });
}

/// A pointer press lands through hit testing on the painted frame, not through
/// a direct handler call.
#[test]
fn a_pointer_press_reaches_the_backend_listener_once() {
    run_headless(|cx| {
        let (handler, clicks) = counting_handler();
        let node = Arc::new(Mutex::new(button_node(
            poodle_specs::ButtonSpec::new().with_label("click"),
            Some(handler),
        )));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.pointer_activate();
        assert_eq!(*clicks.lock().expect("clicks lock"), 1);
    });
}

// ── Retained backend regressions ───────────────────────────────────────────

/// g14.001 retained regression. The node backend bound Enter/Space through
/// `on_key_down` while gpui itself synthesizes KeyUp → click on a focused
/// clickable element, so one Enter fired the handler **twice** — every
/// confirm, submit, and destructive action ran doubled under keyboard use.
/// The redundant binding is gone; the click binding is the single activation
/// path. Only a mounted window can prove this: the count is produced by gpui's
/// own dispatch, not by the renderer.
#[test]
fn one_enter_activates_a_focused_control_exactly_once() {
    run_headless(|cx| {
        let (handler, presses) = counting_handler();
        let node = Arc::new(Mutex::new(button_node(
            poodle_specs::ButtonSpec::new().with_label("enter"),
            Some(handler),
        )));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.keyboard_activate(FIXTURE_ID);
        assert_eq!(
            *presses.lock().expect("presses lock"),
            1,
            "one Enter must be one activation",
        );

        driver.keyboard_activate(FIXTURE_ID);
        assert_eq!(*presses.lock().expect("presses lock"), 2);
    });
}

/// g14.003 retained regression. A scrub is press → drag → release, and the
/// drag has to keep arriving after the pointer leaves the thin track. Bound
/// through `on_mouse_move` the gesture detached a few pixels out; the backend
/// uses gpui's captured `on_drag_move`, which keeps delivering anywhere in the
/// window for a gesture that started on the control. The commit fires once, at
/// release.
#[test]
fn a_scrub_reports_change_while_dragging_and_commits_once_at_release() {
    run_headless(|cx| {
        let mut spec = RangeSliderSpec::default();
        spec.low = 20.0;
        spec.high = 80.0;

        let trace: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let value = Arc::new(Mutex::new((spec.low, spec.high)));

        let change_trace = Arc::clone(&trace);
        let change_value = Arc::clone(&value);
        let commit_trace = Arc::clone(&trace);
        let commit_value = Arc::clone(&value);

        let mut node = poodle_render::range_slider(
            &spec,
            &theme(),
            poodle_render::RangeSliderHandlers {
                on_change: Some(Arc::new(move |low, high| {
                    *change_value.lock().expect("value lock") = (low, high);
                    change_trace
                        .lock()
                        .expect("trace lock")
                        .push("valueChange".to_owned());
                })),
                on_value_commit: Some(Arc::new(move |low, high| {
                    *commit_value.lock().expect("value lock") = (low, high);
                    commit_trace
                        .lock()
                        .expect("trace lock")
                        .push("valueCommit".to_owned());
                })),
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());

        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.wait_for_focus_handle("range-slider-lower");

        driver.pointer_scrub_at(0.9, "press");
        // A real drag moves while held — gpui arms the drag once the pointer
        // exceeds its movement threshold, then dispatches drag moves.
        driver.pointer_scrub_at(0.95, "drag");
        driver.pointer_scrub_at(0.95, "release");

        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["valueChange", "valueChange", "valueCommit"],
        );
        assert_eq!(*value.lock().expect("value lock"), (20.0, 95.0));
    });
}

/// g14.005 retained regression. The overlay layer registry is frame-scoped,
/// not conversion-scoped: a real page converts many components independently
/// within one frame, and every open overlay has to register inside that frame
/// or the dismiss stack loses a layer and Escape unwinds the wrong one.
#[test]
fn overlay_layers_survive_independent_conversions_within_one_frame() {
    run_headless(|cx| {
        let _ = cx;
        poodle_gpui_node_backend::overlay_frame_begin();

        let open_popover = |instance: &str, label: &str| {
            poodle_render::popover(
                &PopoverSpec::new().with_open(true),
                &theme(),
                &poodle_render::PopoverHandlers {
                    on_activate: None,
                    on_dismiss: Some(Arc::new(|_| {})),
                    instance_id: Some(instance.to_owned()),
                },
                Some(Node::text(format!("{label} trigger"))),
                Some(Node::text(format!("{label} panel"))),
            )
        };

        // Two independent compositions converted separately — as a real page
        // converts its components — inside ONE frame.
        let first = open_popover("multi-frame-a", "A");
        let second = open_popover("multi-frame-b", "B");
        let _ = poodle_gpui_node_backend::to_gpui(&first);
        let _ = poodle_gpui_node_backend::to_gpui(&second);

        assert_eq!(
            poodle_gpui_node_backend::open_layer_count(),
            2,
            "both independently converted overlays must register in the same frame",
        );
        poodle_gpui_node_backend::overlay_frame_end();
    });
}

/// g14.005 retained regression. GPUI forbids starting a second deferred draw
/// while it is painting the first. A popover nested inside another popover is
/// therefore painted inside the enclosing deferred scope rather than calling
/// `defer_draw` again. This must execute a real paint: converting the tree
/// alone cannot catch the backend panic.
#[test]
fn a_nested_popover_paints_without_nesting_deferred_draws() {
    run_headless(|cx| {
        let inner = poodle_render::popover(
            &PopoverSpec::new().with_open(true),
            &theme(),
            &poodle_render::PopoverHandlers {
                on_activate: None,
                on_dismiss: Some(Arc::new(|_| {})),
                instance_id: Some("nested-paint:inner".to_owned()),
            },
            Some(Node::text("Inner trigger")),
            Some(Node::text("Inner panel")),
        );
        let outer = poodle_render::popover(
            &PopoverSpec::new().with_open(true),
            &theme(),
            &poodle_render::PopoverHandlers {
                on_activate: None,
                on_dismiss: Some(Arc::new(|_| {})),
                instance_id: Some("nested-paint:outer".to_owned()),
            },
            Some(Node::text("Outer trigger")),
            Some(Node::container().child(inner)),
        );
        let node = Arc::new(Mutex::new(outer));
        let mut driver = HeadlessDriver::new(cx, node);

        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::open_layer_count(),
            2,
            "the outer and nested popover must both survive the paint",
        );
    });
}
