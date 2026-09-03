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

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Explicit import only: `use gpui::*` would glob in gpui's `test` proc macro
// and shadow the built-in `#[test]` attribute (gpui-macros 0.2.2's `test`
// crashes on current rustc).
use gpui::{point, px, Modifiers, Pixels, Point, TestAppContext};
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::audio::{AudioValueLaw, KnobDragMode, XYPadVisualState};
use poodle_headless::time_input::{
    time_input_invalid, time_input_transition, TimeInputContext, TimeInputEvent,
};
use poodle_node::{
    AnimEasing, AnimKeyframe, AnimLoop, AnimProperty, ColorValue, ContinuousValuePhase, DismissReason,
    DragSession, DragSessionPhase, DragSubject, DragTerminalOutcome, DropEligibility, FocusRing,
    LayoutDirection, LayoutOverflow, LayoutSizing, Node, NodeAnimation, NodeContinuousValueEvent,
    NodeDragInputKind, NodeDragSource, NodeDropCommit, NodeDropCommitEvent, NodeDropIntentEvent,
    NodeDropTarget, NodeKind, NodePosition, NodeRole, NodeWheelEvent,
};
use poodle_render::{
    audio_entry_id, fader_spec_from_context, fader_with_handlers, history_center,
    knob_spec_from_context, knob_with_handlers, skeleton, spinner, tabs, time_input_with_persistent_context,
    toast_stack, ui_presentation_provider, xy_pad_spec_from_context, xy_pad_with_handlers, xy_pad_x_id,
    xy_pad_y_id, FaderHandlers, FaderLive, HistoryCenterHandlers, HistoryCenterView,
    KnobHandlers, KnobLive, RadioGroupHandlers, RatingHandlers, RenderContext, SliderHandlers,
    TabsHandlers, ToastStackHandlers, ToggleGroupHandlers, TriStateSwitchHandlers, XYPadHandlers, XYPadLive,
};
use poodle_specs::{
    AccordionSelectionValue, ActiveEdge, AgentTranscriptSpec, ControlDensity, ControlSize, FaderSpec,
    HistoryCenterRejection, HistoryCenterSpec, KnobSpec, Orientation, PopoverSpec, RangeSliderSpec,
    RatingSpec, SkeletonSpec, SliderAppearance, SliderDirection, SliderSpec, SpinnerSpec, TabActivationMode, TabDefinition, TabVariant,
    TabsSpec, TimeInputSpec, Toast, ToastStackSpec, ToastTone, TriStateSwitchSpec, TriStateValue,
    UiPresentationProviderSpec, XYPadSpec,
};

#[path = "../src/headless_driver.rs"]
mod headless_driver;

#[path = "../src/nucleus_receipts.rs"]
mod nucleus_receipts;

#[path = "../src/block_slider_host.rs"]
mod block_slider_host;

// The preview-local axis decision (g15.019). Pure data, no GPUI: which axis
// tabs a specimen page publishes, and which tab a retained selection resolves
// to once the available set shrinks.
#[path = "../src/specimens/specimen_axes.rs"]
mod specimen_axes;

use headless_driver::HeadlessDriver;
use specimen_axes::{
    density_key, size_key, AxisAdmission, DENSITIES_TAB, EXAMPLES_TAB, EYEBROW_SIZES, SIZES_TAB,
    TEXT_SIZES,
};

/// The element id every single-node fixture mounts under.
const FIXTURE_ID: &str = "headless-fixture";

/// Shared in-memory test-platform harness. The `#[gpui::test]` macro from
/// gpui-macros 0.2.2 crashes on current rustc, so this mirrors its teardown
/// (parked queue, forbidden parking, app shutdown) in a plain `#[test]`.
fn run_headless(body: impl FnOnce(&mut TestAppContext)) {
    poodle_gpui_node_backend::reset_focus_registry();
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

fn fader_live(spec: &FaderSpec) -> Arc<Mutex<FaderLive>> {
    Arc::new(Mutex::new(FaderLive::from_spec(spec)))
}

fn fader_now(live: &Arc<Mutex<FaderLive>>, aria: &str) -> FaderSpec {
    fader_spec_from_context(&live.lock().expect("fader machine").machine, aria)
}

fn knob_live(spec: &KnobSpec) -> Arc<Mutex<KnobLive>> {
    Arc::new(Mutex::new(KnobLive::from_spec(spec)))
}

fn knob_now(live: &Arc<Mutex<KnobLive>>, aria: &str) -> KnobSpec {
    knob_spec_from_context(&live.lock().expect("knob machine").machine, aria)
}

fn xy_live(spec: &XYPadSpec) -> Arc<Mutex<XYPadLive>> {
    Arc::new(Mutex::new(poodle_render::xy_pad_context_from_spec(spec)))
}

fn xy_now(live: &Arc<Mutex<XYPadLive>>, aria: &str) -> XYPadSpec {
    xy_pad_spec_from_context(&live.lock().expect("xy pad machine"), aria)
}

fn click_away_target() -> Node {
    let mut away = Node::container();
    away.id = Some("away".into());
    away.interaction.focusable = true;
    away.a11y.tab_index = Some(0);
    away.style.descriptor.layout.width = LayoutSizing::Fixed(24.0);
    away.style.descriptor.layout.height = LayoutSizing::Fixed(24.0);
    away.style.focus_ring = Some(FocusRing {
        color: ColorValue(0.2, 0.4, 0.8, 1.0),
        width: 2.0,
        offset: 0.0,
    });
    away
}

fn with_click_away(control: Node) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.width = LayoutSizing::Fixed(200.0);
    row.style.descriptor.layout.height = LayoutSizing::Fixed(60.0);
    row.child(control).child(click_away_target())
}


fn button_node(
    spec: poodle_specs::ButtonSpec,
    handler: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    let mut node = poodle_render::button(&spec, &RenderContext::new(&theme()), handler);
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

/// g15.043 (architecture 010): the UiPresentationProvider cascade is
/// construction-time and layout-neutral. A button that omits size and density
/// inside an xl/comfortable scope mounts at the inherited xl geometry (52px
/// control height), the mounted node IS the button (no provider wrapper exists
/// to paint, lay out, or hold focus), and the backend's real focus machinery
/// reaches it directly.
#[test]
fn a_provider_scope_cascades_to_mounted_geometry_without_a_wrapper_node() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let scope = UiPresentationProviderSpec::new()
            .with_size_scale(ControlSize::Xl)
            .with_density(ControlDensity::Comfortable);
        let mut scoped_button = ui_presentation_provider(&scope, &ctx, |scoped| {
            poodle_render::button(
                &poodle_specs::ButtonSpec::new().with_label("scoped"),
                scoped,
                None,
            )
        });
        // No wrapper: the returned node is the button itself.
        assert!(matches!(scoped_button.kind, poodle_node::NodeKind::Button { .. }));
        assert_eq!(scoped_button.a11y.role, Some(poodle_node::NodeRole::Button));
        scoped_button.id = Some(FIXTURE_ID.to_owned());
        let mut root_button = poodle_render::button(
            &poodle_specs::ButtonSpec::new().with_label("root"),
            &ctx,
            None,
        );
        root_button.id = Some("headless-fixture-root".to_owned());
        let pair = Node::container().child(scoped_button).child(root_button);
        let node = Arc::new(Mutex::new(pair));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        driver.draw_frame();

        // Mounted paint bounds observe the inherited xl geometry against the
        // root-default md sibling (recorded bounds exclude the 1px border per
        // side: 52→50 and 36→34). The scope, not the host, did the work.
        let scoped = poodle_gpui_node_backend::bounds_for(FIXTURE_ID).expect("scoped bounds");
        let root =
            poodle_gpui_node_backend::bounds_for("headless-fixture-root").expect("root bounds");
        assert_eq!(f32::from(scoped.size.height), 50.0);
        assert_eq!(f32::from(root.size.height), 34.0);

        // The accessibility surface is the button's own: a plain sequential
        // focus stop reached by the backend's real focus machinery.
        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.focus_element(FIXTURE_ID);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(FIXTURE_ID),
            Some(true)
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
        nucleus_receipts::emit_if_configured(
            "Button",
            "nucleus.shell.button",
            driver.mounted_observation(),
            &["mount Button through HeadlessDriver", "pointer press and release through GPUI dispatch"],
            &["the mounted Button listener fired exactly once"],
        );
    });
}

/// g15.041: Button disclosure targets (contract §3 `controls`) ride the same
/// renderer-neutral node channel as IconButton's — a Button built with
/// `with_controls(...)` mounts through the real backend carrying
/// `a11y.controls`. Structural evidence only: gpui 0.2.2 projects no
/// platform accessibility attributes from this field.
#[test]
fn a_mounted_button_carries_its_controls_target() {
    run_headless(|cx| {
        let node = Arc::new(Mutex::new(button_node(
            poodle_specs::ButtonSpec::new()
                .with_label("Details")
                .with_controls("details"),
            None,
        )));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle(FIXTURE_ID);
        assert_eq!(
            node.lock().expect("node lock").a11y.controls.as_deref(),
            Some("details"),
        );

        // Absence stays absence: a bare spec mounts carrying no target.
        let bare = button_node(poodle_specs::ButtonSpec::new().with_label("Save"), None);
        assert_eq!(bare.a11y.controls, None);
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
            &RenderContext::new(&theme()),
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
        // exceeds its movement threshold. That arming move establishes the
        // payload; the following move is the first captured drag dispatch.
        driver.pointer_scrub_at(0.93, "drag");
        driver.pointer_scrub_at(0.95, "drag");
        driver.pointer_scrub_at(0.95, "release");

        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["valueChange", "valueChange", "valueCommit"],
        );
        assert_eq!(*value.lock().expect("value lock"), (20.0, 95.0));
    });
}

/// g16.032. One renderer-neutral continuous-value gesture: press, moves,
/// exactly one release or cancel. Wheel and double-activation are separate
/// Node routes. Existing `on_scrub` stays on Slider/RangeSlider.
#[test]
fn a_continuous_value_gesture_releases_once_and_cancels_on_lost_host() {
    fn fixture(
        trace: Arc<Mutex<Vec<String>>>,
        last: Arc<Mutex<Option<NodeContinuousValueEvent>>>,
    ) -> Node {
        let phases = Arc::clone(&trace);
        let wheel_trace = Arc::clone(&trace);
        let double_trace = Arc::clone(&trace);
        let stored = Arc::clone(&last);
        let mut node = Node::container();
        node.id = Some(FIXTURE_ID.to_owned());
        node.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
        node.style.descriptor.layout.height = LayoutSizing::Fixed(60.0);
        node.interaction.on_continuous_value = Some(Arc::new(move |event| {
            *stored.lock().expect("event lock") = Some(*event);
            phases
                .lock()
                .expect("trace lock")
                .push(format!("{:?}", event.phase));
        }));
        node.interaction.on_wheel = Some(Arc::new(move |event: &NodeWheelEvent| {
            wheel_trace
                .lock()
                .expect("trace lock")
                .push(format!("wheel:{}:{}", event.dx, event.dy));
        }));
        node.interaction.on_double_activate = Some(Arc::new(move |_mods| {
            double_trace
                .lock()
                .expect("trace lock")
                .push("double".to_owned());
        }));
        node
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let last = Arc::new(Mutex::new(None));
        let mounted = Arc::new(Mutex::new(fixture(Arc::clone(&trace), Arc::clone(&last))));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let center = headless_driver::mount_box_center();
        let outside = point(px(400.0), px(400.0));

        driver.pointer_press(center);
        // GPUI arms `on_drag` after the movement threshold; the first held
        // move establishes the payload, the second is the captured Move.
        driver.pointer_drag(point(center.x + px(8.0), center.y));
        driver.pointer_drag(point(center.x + px(24.0), center.y));
        driver.pointer_release(outside);
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press", "Move", "Release"]
        );
        let released = last.lock().expect("event lock").expect("release event");
        assert_eq!(released.phase, ContinuousValuePhase::Release);
        assert!(released.x >= 0.0 && released.x <= 1.0);
        assert!(released.y >= 0.0 && released.y <= 1.0);

        trace.lock().expect("trace lock").clear();
        driver.pointer_press(center);
        driver.pointer_press(center);
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press"],
            "a second press while the gesture is open is inert"
        );
        driver.pointer_release(center);
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press", "Release"]
        );

        trace.lock().expect("trace lock").clear();
        driver.pointer_press(center);
        *mounted.lock().expect("mount lock") = Node::container();
        driver.draw_frame();
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press", "Cancel"]
        );
        driver.draw_frame();
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press", "Cancel"],
            "lost-host cancel is exactly once"
        );
    });

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let last = Arc::new(Mutex::new(None));
        let mounted = Arc::new(Mutex::new(fixture(Arc::clone(&trace), Arc::clone(&last))));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let center = headless_driver::mount_box_center();
        driver.pointer_press(center);
        *mounted.lock().expect("mount lock") = Node::container();
        driver.draw_preview_frame();
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press", "Cancel"],
            "one production frame after removal must cancel"
        );
        let replacement = fixture(Arc::clone(&trace), Arc::clone(&last));
        *mounted.lock().expect("mount lock") = replacement;
        driver.draw_preview_frame();
        trace.lock().expect("trace lock").clear();
        driver.pointer_press(center);
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["Press"],
            "a newly mounted control accepts its first press immediately"
        );
    });

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let last = Arc::new(Mutex::new(None));
        let mut driver =
            HeadlessDriver::new(cx, Arc::new(Mutex::new(fixture(Arc::clone(&trace), last))));
        let center = headless_driver::mount_box_center();
        driver.pointer_hover(center);
        driver.scroll_vertical(-12.0);
        driver.pointer_press_details(center, 2, Modifiers::none());
        let events = trace.lock().expect("trace lock").clone();
        assert!(
            events.iter().any(|event| event.starts_with("wheel:")),
            "wheel dispatch: {events:?}"
        );
        assert!(
            events.iter().any(|event| event == "double"),
            "double activation: {events:?}"
        );
        assert!(
            !events.iter().any(|event| event == "Press"),
            "double activation must not open a value gesture: {events:?}"
        );
    });
}

/// g16.032. Fader through production GPUI dispatch.
#[test]
fn fader_mounted_parity_through_production_dispatch() {
    fn handlers(id: &str) -> FaderHandlers {
        FaderHandlers::new(id)
    }
    fn seed(value: f64, orientation: Orientation) -> FaderSpec {
        let mut spec = FaderSpec::new(value, 0.0, 1.0, AudioValueLaw::Linear);
        spec.orientation = orientation;
        spec.default_value = 0.0;
        spec.detents = vec![0.5];
        spec.aria_label = "Level".into();
        spec
    }

    run_headless(|cx| {
        let id = "fader-main";
        let spec0 = seed(0.2, Orientation::Horizontal);
        let live = fader_live(&spec0);
        let trace = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        let events = Arc::clone(&trace);
        let node = fader_with_handlers(
            &spec0,
            &RenderContext::new(&theme()),
            &handlers(id)
                .on_value_change({
                    let events = Arc::clone(&events);
                    Arc::new(move |_| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueChange".into());
                    })
                })
                .on_value_commit({
                    let events = Arc::clone(&events);
                    Arc::new(move |_| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueCommit".into());
                    })
                })
                .on_gesture_begin({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("gestureBegin".into());
                    })
                })
                .on_gesture_end({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events.lock().expect("trace lock").push("gestureEnd".into());
                    })
                }),
            &live,
        );
        *mounted.lock().unwrap() = node;
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(id);
        driver.pointer_scrub_at(0.5, "press");
        driver.pointer_scrub_at(0.52, "drag");
        driver.pointer_scrub_at(0.5, "drag");
        driver.pointer_scrub_at(0.5, "release");
        assert_eq!(fader_now(&live, "Level").visual_state.raw_value, 0.5);
        let log = trace.lock().expect("trace lock").clone();
        assert!(log.contains(&"gestureBegin".to_string()));
        assert!(log.contains(&"valueChange".to_string()));
        assert!(log.contains(&"valueCommit".to_string()));
        assert!(log.contains(&"gestureEnd".to_string()));
        *mounted.lock().unwrap() = fader_with_handlers(
            &fader_now(&live, "Level"),
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        let control = mounted.lock().unwrap();
        assert_eq!(control.a11y.role, Some(NodeRole::Slider));
        assert_eq!(control.a11y.orientation.as_deref(), Some("horizontal"));
        drop(control);

        driver.wait_for_focus_handle(id);
        tab_until_focused(&mut driver, id);
        driver.dispatch_key_raw("end");
        assert_eq!(fader_now(&live, "Level").visual_state.raw_value, 1.0);
        driver.dispatch_key_raw("pagedown");
        assert!((fader_now(&live, "Level").visual_state.raw_value - 0.9).abs() < 1e-9);
        driver.scroll_vertical(-12.0);
        assert!(fader_now(&live, "Level").visual_state.raw_value > 0.9);
        let center = headless_driver::mount_box_center();
        driver.pointer_press_details(center, 2, Modifiers::none());
        assert_eq!(fader_now(&live, "Level").visual_state.raw_value, 0.0);

        tab_until_focused(&mut driver, id);
        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = fader_with_handlers(
            &fader_now(&live, "Level"),
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        let entry_id = audio_entry_id(id);
        driver.wait_for_focus_handle(&entry_id);
        driver.focus_element(&entry_id);
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("0");
        driver.dispatch_key_raw(".");
        driver.dispatch_key_raw("2");
        driver.dispatch_key_raw("5");
        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = fader_with_handlers(
            &fader_now(&live, "Level"),
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        let after_commit = fader_now(&live, "Level");
        assert!((after_commit.visual_state.raw_value - 0.25).abs() < 1e-9);
        assert!(!after_commit.entry_open);
        driver.wait_for_focus_handle(id);

        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = fader_with_handlers(
            &fader_now(&live, "Level"),
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        driver.wait_for_focus_handle(&entry_id);
        driver.focus_element(&entry_id);
        driver.dispatch_key_raw("escape");
        *mounted.lock().unwrap() = fader_with_handlers(
            &fader_now(&live, "Level"),
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        assert!(!fader_now(&live, "Level").entry_open);
        driver.wait_for_focus_handle(id);

        let commits = Arc::new(Mutex::new(0usize));
        let bind_blur = |commits: &Arc<Mutex<usize>>| {
            let c = Arc::clone(commits);
            with_click_away(fader_with_handlers(
                &fader_now(&live, "Level"),
                &RenderContext::new(&theme()),
                &handlers(id).on_value_commit(Arc::new(move |_| {
                    *c.lock().expect("commit") += 1;
                })),
                &live,
            ))
        };
        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = bind_blur(&commits);
        driver.draw_frame();
        driver.wait_for_focus_handle(&entry_id);
        *mounted.lock().unwrap() = bind_blur(&commits);
        driver.draw_frame();
        driver.focus_element(&entry_id);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&entry_id),
            Some(true),
            "type-in must hold focus before Tab"
        );
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("0");
        driver.dispatch_key_raw(".");
        driver.dispatch_key_raw("4");
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&entry_id),
            Some(false),
            "Tab must leave the type-in field"
        );
        assert!(
            !live.lock().expect("fader machine").machine.base.entry_open,
            "Tab blur must commit once"
        );
        *mounted.lock().unwrap() = bind_blur(&commits);
        driver.draw_frame();
        assert!((fader_now(&live, "Level").visual_state.raw_value - 0.4).abs() < 1e-9);
        let after = *commits.lock().expect("commit");
        driver.draw_frame();
        driver.draw_frame();
        assert_eq!(
            *commits.lock().expect("commit"),
            after,
            "Tab blur must not commit again after rebuild"
        );

        let before = fader_now(&live, "Level").visual_state.raw_value;
        driver.pointer_scrub_at(0.2, "press");
        driver.pointer_scrub_at(0.22, "drag");
        *mounted.lock().unwrap() = fader_with_handlers(
            &fader_now(&live, "Level"),
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        driver.pointer_scrub_at(0.8, "drag");
        driver.pointer_scrub_at(0.8, "release");
        assert_ne!(fader_now(&live, "Level").visual_state.raw_value, before);

        driver.pointer_scrub_at(0.3, "press");
        driver.pointer_scrub_at(0.32, "drag");
        let mut host_spec = fader_now(&live, "Level");
        host_spec.visual_state.raw_value = 0.15;
        *mounted.lock().unwrap() = fader_with_handlers(
            &host_spec,
            &RenderContext::new(&theme()),
            &handlers(id),
            &live,
        );
        driver.draw_frame();
        assert!(
            (fader_now(&live, "Level").visual_state.raw_value - 0.15).abs() < 1e-9,
            "host SetValue applies during a gesture"
        );
        driver.pointer_scrub_at(0.3, "release");
    });

    run_headless(|cx| {
        let mut spec = seed(0.2, Orientation::Vertical);
        spec.detents.clear();
        let live = fader_live(&spec);
        let node = fader_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &handlers("fader-vertical"),
            &live,
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(node)), 80.0, 160.0);
        driver.wait_for_focus_handle("fader-vertical");
        driver.pointer_scrub_vertical_at(0.8, "press");
        driver.pointer_scrub_vertical_at(0.82, "drag");
        driver.pointer_scrub_vertical_at(0.8, "drag");
        driver.pointer_scrub_vertical_at(0.8, "release");
        assert!(fader_now(&live, "Level").visual_state.raw_value > 0.5);
    });

    run_headless(|cx| {
        let parent_wheels = Arc::new(Mutex::new(0u32));
        let count = Arc::clone(&parent_wheels);
        let spec = seed(0.4, Orientation::Horizontal);
        let live = fader_live(&spec);
        let fader = fader_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &handlers("fader-wheel"),
            &live,
        );
        let mut root = Node::container();
        root.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
        root.style.descriptor.layout.height = LayoutSizing::Fixed(60.0);
        root.interaction.on_wheel = Some(Arc::new(move |_event: &NodeWheelEvent| {
            *count.lock().expect("wheel lock") += 1;
        }));
        root = root.child(fader);
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(root)));
        driver.wait_for_focus_handle("fader-wheel");
        driver.scroll_vertical_id("fader-wheel", -12.0);
        assert!(fader_now(&live, "Level").visual_state.raw_value > 0.4);
        assert_eq!(*parent_wheels.lock().expect("wheel lock"), 0);
    });

    run_headless(|cx| {
        let left_spec = seed(0.2, Orientation::Horizontal);
        let right_spec = seed(0.2, Orientation::Horizontal);
        let left_live = fader_live(&left_spec);
        let right_live = fader_live(&right_spec);
        let mut left = fader_with_handlers(
            &left_spec,
            &RenderContext::new(&theme()),
            &handlers("fader-left"),
            &left_live,
        );
        left.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        let mut right = fader_with_handlers(
            &right_spec,
            &RenderContext::new(&theme()),
            &handlers("fader-right"),
            &right_live,
        );
        right.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row = row.child(left).child(right);
        let mounted = Arc::new(Mutex::new(row));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 200.0, 80.0);
        driver.wait_for_focus_handle("fader-left");
        let left_bounds = poodle_gpui_node_backend::bounds_for("fader-left").expect("left bounds");
        driver.pointer_press(left_bounds.center());
        driver.pointer_drag(point(
            left_bounds.origin.x + px(8.0),
            left_bounds.center().y,
        ));
        let mut rebuilt_left = fader_with_handlers(
            &fader_now(&left_live, "Level"),
            &RenderContext::new(&theme()),
            &handlers("fader-left"),
            &left_live,
        );
        rebuilt_left.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        let mut rebuilt_right = fader_with_handlers(
            &fader_now(&right_live, "Level"),
            &RenderContext::new(&theme()),
            &handlers("fader-right"),
            &right_live,
        );
        rebuilt_right.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        *mounted.lock().unwrap() = row.child(rebuilt_left).child(rebuilt_right);
        driver.draw_frame();
        let left_bounds = poodle_gpui_node_backend::bounds_for("fader-left").expect("left bounds");
        driver.pointer_drag(point(
            left_bounds.origin.x + px(40.0),
            left_bounds.center().y,
        ));
        driver.pointer_release(left_bounds.center());
        assert_ne!(
            fader_now(&left_live, "Level").visual_state.raw_value,
            fader_now(&right_live, "Level").visual_state.raw_value
        );
        assert_eq!(fader_now(&right_live, "Level").visual_state.raw_value, 0.2);
    });

    run_headless(|cx| {
        let mut spec = seed(0.4, Orientation::Horizontal);
        spec.visual_state.enabled = false;
        let live = fader_live(&spec);
        let sink = Arc::new(Mutex::new(0.4f64));
        let reported = Arc::clone(&sink);
        let node = fader_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &handlers("fader-disabled").on_value_change(Arc::new(move |next| {
                *reported.lock().expect("value lock") = next;
            })),
            &live,
        );
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.pointer_scrub_at(0.9, "press");
        driver.pointer_scrub_at(0.9, "release");
        driver.dispatch_key("right");
        assert_eq!(*sink.lock().expect("value lock"), 0.4);
    });
}

/// g16.032. Knob vertical and circular mapping.
#[test]
fn knob_mounted_parity_through_production_dispatch() {
    fn seed(value: f64, mode: KnobDragMode) -> KnobSpec {
        let mut spec = KnobSpec::new(value, 0.0, 1.0, AudioValueLaw::Linear);
        spec.drag_mode = mode;
        spec.default_value = 0.0;
        spec.aria_label = "Gain".into();
        spec
    }

    run_headless(|cx| {
        let id = "knob-main";
        let spec0 = seed(0.4, KnobDragMode::Vertical);
        let live = knob_live(&spec0);
        let trace = Arc::new(Mutex::new(Vec::new()));
        let events = Arc::clone(&trace);
        let node = knob_with_handlers(
            &spec0,
            &RenderContext::new(&theme()),
            &KnobHandlers::new(id)
                .on_value_change({
                    let events = Arc::clone(&events);
                    Arc::new(move |_| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueChange".into());
                    })
                })
                .on_value_commit({
                    let events = Arc::clone(&events);
                    Arc::new(move |_| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueCommit".into());
                    })
                })
                .on_gesture_begin({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("gestureBegin".into());
                    })
                })
                .on_gesture_end({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events.lock().expect("trace lock").push("gestureEnd".into());
                    })
                }),
            &live,
        );
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(id);
        let center = headless_driver::mount_box_center();
        driver.pointer_press(center);
        driver.pointer_drag(point(center.x, center.y - px(16.0)));
        driver.pointer_drag(point(center.x, center.y - px(24.0)));
        *mounted.lock().unwrap() = knob_with_handlers(
            &knob_now(&live, "Gain"),
            &RenderContext::new(&theme()),
            &KnobHandlers::new(id)
                .on_value_change({
                    let events = Arc::clone(&events);
                    Arc::new(move |_| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueChange".into());
                    })
                })
                .on_value_commit({
                    let events = Arc::clone(&events);
                    Arc::new(move |_| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueCommit".into());
                    })
                })
                .on_gesture_end({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events.lock().expect("trace lock").push("gestureEnd".into());
                    })
                }),
            &live,
        );
        driver.draw_frame();
        driver.pointer_drag(point(center.x, center.y - px(32.0)));
        driver.pointer_release(center);
        assert!(knob_now(&live, "Gain").visual_state.raw_value > 0.4);
        let log = trace.lock().expect("trace lock").clone();
        assert!(log.contains(&"gestureBegin".to_string()));
        assert!(log.contains(&"valueChange".to_string()));
        assert!(log.contains(&"valueCommit".to_string()));
        tab_until_focused(&mut driver, id);
        driver.dispatch_key_raw("home");
        assert_eq!(knob_now(&live, "Gain").visual_state.raw_value, 0.0);
        driver.dispatch_key_raw("end");
        assert_eq!(knob_now(&live, "Gain").visual_state.raw_value, 1.0);
        driver.dispatch_key_raw("pagedown");
        assert!((knob_now(&live, "Gain").visual_state.raw_value - 0.9).abs() < 1e-9);
        driver.scroll_vertical(-12.0);
        assert!(knob_now(&live, "Gain").visual_state.raw_value > 0.9);
        driver.pointer_press_details(center, 2, Modifiers::none());
        assert_eq!(knob_now(&live, "Gain").visual_state.raw_value, 0.0);
        tab_until_focused(&mut driver, id);
        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = knob_with_handlers(
            &knob_now(&live, "Gain"),
            &RenderContext::new(&theme()),
            &KnobHandlers::new(id),
            &live,
        );
        driver.draw_frame();
        let entry_id = audio_entry_id(id);
        driver.wait_for_focus_handle(&entry_id);
        driver.focus_element(&entry_id);
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("0");
        driver.dispatch_key_raw(".");
        driver.dispatch_key_raw("3");
        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = knob_with_handlers(
            &knob_now(&live, "Gain"),
            &RenderContext::new(&theme()),
            &KnobHandlers::new(id),
            &live,
        );
        driver.draw_frame();
        assert!((knob_now(&live, "Gain").visual_state.raw_value - 0.3).abs() < 1e-9);
        driver.wait_for_focus_handle(id);

        let commits = Arc::new(Mutex::new(0usize));
        let bind_blur = |commits: &Arc<Mutex<usize>>| {
            let c = Arc::clone(commits);
            with_click_away(knob_with_handlers(
                &knob_now(&live, "Gain"),
                &RenderContext::new(&theme()),
                &KnobHandlers::new(id).on_value_commit(Arc::new(move |_| {
                    *c.lock().expect("commit") += 1;
                })),
                &live,
            ))
        };
        tab_until_focused(&mut driver, id);
        driver.dispatch_key_raw("enter");
        *mounted.lock().unwrap() = bind_blur(&commits);
        driver.draw_frame();
        driver.wait_for_focus_handle(&entry_id);
        *mounted.lock().unwrap() = bind_blur(&commits);
        driver.draw_frame();
        driver.focus_element(&entry_id);
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("0");
        driver.dispatch_key_raw(".");
        driver.dispatch_key_raw("4");
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&entry_id),
            Some(false),
            "Tab must leave the type-in field"
        );
        assert!(
            !live.lock().expect("knob machine").machine.base.entry_open,
            "Tab blur must commit once"
        );
        *mounted.lock().unwrap() = bind_blur(&commits);
        driver.draw_frame();
        assert!((knob_now(&live, "Gain").visual_state.raw_value - 0.4).abs() < 1e-9);
        let after = *commits.lock().expect("commit");
        driver.draw_frame();
        driver.draw_frame();
        assert_eq!(
            *commits.lock().expect("commit"),
            after,
            "Tab blur must not commit again after rebuild"
        );
    });

    run_headless(|cx| {
        let spec = seed(0.0, KnobDragMode::Circular);
        let live = knob_live(&spec);
        let node = knob_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &KnobHandlers::new("knob-circular"),
            &live,
        );
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        let center = headless_driver::mount_box_center();
        driver.pointer_press(center);
        driver.pointer_drag(point(center.x + px(20.0), center.y - px(20.0)));
        driver.pointer_drag(point(center.x + px(24.0), center.y - px(24.0)));
        driver.pointer_release(center);
        assert!(knob_now(&live, "Gain").visual_state.raw_value > 0.0);
    });

    run_headless(|cx| {
        let mut spec = seed(0.4, KnobDragMode::Vertical);
        spec.visual_state.enabled = false;
        let live = knob_live(&spec);
        let sink = Arc::new(Mutex::new(0.4f64));
        let reported = Arc::clone(&sink);
        let node = knob_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &KnobHandlers::new("knob-disabled").on_value_change(Arc::new(move |next| {
                *reported.lock().expect("value lock") = next;
            })),
            &live,
        );
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.pointer_scrub_at(0.9, "press");
        driver.pointer_scrub_at(0.9, "release");
        driver.dispatch_key("right");
        assert_eq!(*sink.lock().expect("value lock"), 0.4);
    });
}

/// g16.032. XYPad atomic pair.
#[test]
fn xy_pad_mounted_parity_through_production_dispatch() {
    fn seed(x: f64, y: f64) -> XYPadSpec {
        let mut spec = XYPadSpec::new(XYPadVisualState {
            x_norm: x,
            y_norm: y,
            raw_x: x,
            raw_y: y,
            hover: false,
            focus: false,
            drag: poodle_headless::audio::DragState::None,
            automation: poodle_headless::audio::AutomationState::None,
            enabled: true,
        });
        spec.default_x = 0.5;
        spec.default_y = 0.5;
        spec.aria_label = "Pad".into();
        spec
    }

    run_headless(|cx| {
        let id = "xy-main";
        let spec0 = seed(0.2, 0.3);
        let live = xy_live(&spec0);
        let trace = Arc::new(Mutex::new(Vec::new()));
        let events = Arc::clone(&trace);
        let node = xy_pad_with_handlers(
            &spec0,
            &RenderContext::new(&theme()),
            &XYPadHandlers::new(id)
                .on_value_change({
                    let events = Arc::clone(&events);
                    Arc::new(move |_, _| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueChange".into());
                    })
                })
                .on_value_commit({
                    let events = Arc::clone(&events);
                    Arc::new(move |_, _| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueCommit".into());
                    })
                })
                .on_gesture_begin({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("gestureBegin".into());
                    })
                })
                .on_gesture_end({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events.lock().expect("trace lock").push("gestureEnd".into());
                    })
                }),
            &live,
        );
        let x_id = xy_pad_x_id(id);
        let y_id = xy_pad_y_id(id);
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 200.0, 200.0);
        driver.wait_for_focus_handle(&x_id);
        driver.pointer_scrub_at(0.8, "press");
        driver.pointer_scrub_at(0.82, "drag");
        driver.pointer_scrub_at(0.84, "drag");
        *mounted.lock().unwrap() = xy_pad_with_handlers(
            &xy_now(&live, "Pad"),
            &RenderContext::new(&theme()),
            &XYPadHandlers::new(id)
                .on_value_change({
                    let events = Arc::clone(&events);
                    Arc::new(move |_, _| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueChange".into());
                    })
                })
                .on_value_commit({
                    let events = Arc::clone(&events);
                    Arc::new(move |_, _| {
                        events
                            .lock()
                            .expect("trace lock")
                            .push("valueCommit".into());
                    })
                })
                .on_gesture_end({
                    let events = Arc::clone(&events);
                    Arc::new(move || {
                        events.lock().expect("trace lock").push("gestureEnd".into());
                    })
                }),
            &live,
        );
        driver.draw_frame();
        driver.pointer_scrub_at(0.8, "drag");
        driver.pointer_scrub_at(0.8, "release");
        assert!(xy_now(&live, "Pad").visual_state.raw_x > 0.2);
        let log = trace.lock().expect("trace lock").clone();
        assert!(log.contains(&"gestureBegin".to_string()));
        assert!(log.contains(&"valueChange".to_string()));
        assert!(log.contains(&"valueCommit".to_string()));
        driver.pointer_press_details(headless_driver::mount_box_center(), 2, Modifiers::none());
        let reset = xy_now(&live, "Pad");
        assert!((reset.visual_state.raw_x - 0.5).abs() < 1e-9);
        driver.wait_for_focus_handle(&x_id);
        tab_until_focused(&mut driver, &x_id);
        driver.dispatch_key_raw("end");
        assert_eq!(xy_now(&live, "Pad").visual_state.raw_x, 1.0);
        driver.wait_for_focus_handle(&y_id);
        tab_until_focused(&mut driver, &y_id);
        driver.dispatch_key_raw("home");
        assert_eq!(xy_now(&live, "Pad").visual_state.raw_y, 0.0);
        let node = xy_pad_with_handlers(
            &xy_now(&live, "Pad"),
            &RenderContext::new(&theme()),
            &XYPadHandlers::new(id),
            &live,
        );
        assert_eq!(node.a11y.role, Some(NodeRole::Group));
        assert!(node
            .find(&|n| n.id.as_deref() == Some(x_id.as_str()))
            .is_some());
        assert!(node
            .find(&|n| n.id.as_deref() == Some(y_id.as_str()))
            .is_some());
    });

    run_headless(|cx| {
        let left_spec = seed(0.2, 0.2);
        let right_spec = seed(0.2, 0.2);
        let left_live = xy_live(&left_spec);
        let right_live = xy_live(&right_spec);
        let mut left = xy_pad_with_handlers(
            &left_spec,
            &RenderContext::new(&theme()),
            &XYPadHandlers::new("xy-left"),
            &left_live,
        );
        left.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        let mut right = xy_pad_with_handlers(
            &right_spec,
            &RenderContext::new(&theme()),
            &XYPadHandlers::new("xy-right"),
            &right_live,
        );
        right.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row = row.child(left).child(right);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(row)), 200.0, 120.0);
        driver.wait_for_focus_handle(&xy_pad_x_id("xy-left"));
        let bounds = poodle_gpui_node_backend::bounds_for("xy-left").expect("left pad");
        driver.pointer_press(bounds.center());
        driver.pointer_drag(point(bounds.origin.x + px(20.0), bounds.center().y));
        driver.pointer_release(bounds.center());
        assert_ne!(
            xy_now(&left_live, "Pad").visual_state.raw_x,
            xy_now(&right_live, "Pad").visual_state.raw_x
        );
    });

    run_headless(|cx| {
        let mut spec = seed(0.4, 0.4);
        spec.visual_state.enabled = false;
        let live = xy_live(&spec);
        let sink = Arc::new(Mutex::new((0.4, 0.4)));
        let reported = Arc::clone(&sink);
        let node = xy_pad_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &XYPadHandlers::new("xy-disabled").on_value_change(Arc::new(move |x, y| {
                *reported.lock().expect("value lock") = (x, y);
            })),
            &live,
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(node)), 200.0, 200.0);
        driver.pointer_scrub_at(0.9, "press");
        driver.pointer_scrub_at(0.9, "release");
        assert_eq!(*sink.lock().expect("value lock"), (0.4, 0.4));
    });
}

fn payload_paint_box(id: &str) -> Node {
    let mut node = Node::container();
    node.id = Some(id.to_owned());
    node.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
    node.style.descriptor.layout.height = LayoutSizing::Fixed(80.0);
    node.style.descriptor.background = Some(ColorValue(0.2, 0.3, 0.4, 1.0));
    node
}

fn payload_frac(id: &str, x_frac: f32, y_frac: f32) -> Point<Pixels> {
    let bounds = poodle_gpui_node_backend::bounds_for(id).expect(id);
    point(
        px(f32::from(bounds.origin.x) + f32::from(bounds.size.width) * x_frac),
        px(f32::from(bounds.origin.y) + f32::from(bounds.size.height) * y_frac),
    )
}

// ── Custom drag-and-drop surfaces on the shared kernel (g16.025) ──────────
//
// The substrate must work without a Poodle composite: these fixtures are the
// consumer-built sources and targets a host writes from the public
// registration vocabulary, driven through real mounted GPUI dispatch. Direct
// handler invocation would prove none of it.

const CUSTOM_SUBJECT_KIND: &str = "custom.row";

fn custom_subject(id: &str) -> DragSubject {
    DragSubject {
        kind: CUSTOM_SUBJECT_KIND.to_string(),
        id: id.to_string(),
    }
}

fn push_trace(trace: &Arc<Mutex<Vec<String>>>, entry: String) {
    trace.lock().expect("trace lock").push(entry);
}

fn describe_outcome(outcome: &DragTerminalOutcome) -> String {
    match outcome {
        DragTerminalOutcome::Committed { intent } => {
            format!("committed:{}:{}", intent.target_id, intent.position)
        }
        DragTerminalOutcome::Rejected { reason } => {
            format!("rejected:{}", reason.clone().unwrap_or_default())
        }
        DragTerminalOutcome::Failed { reason } => {
            format!("failed:{}", reason.clone().unwrap_or_default())
        }
        DragTerminalOutcome::Cancelled { reason } => format!("cancelled:{reason:?}"),
    }
}

/// A consumer-built drag source: opaque subject, accessible name, keyboard
/// pickup opted in, and traced start/terminal callbacks.
fn traced_source(id: &str, label: &str, trace: &Arc<Mutex<Vec<String>>>) -> NodeDragSource {
    let mut source = NodeDragSource::new(id, custom_subject(id), label);
    source.keyboard_order = Some(0);
    let start = Arc::clone(trace);
    source.on_drag_start = Some(Arc::new(move |session: &DragSession| {
        push_trace(&start, format!("start:{}", session.subject.id));
    }));
    let end = Arc::clone(trace);
    source.on_drag_end = Some(Arc::new(move |outcome: &DragTerminalOutcome| {
        push_trace(&end, format!("end:{}", describe_outcome(outcome)));
    }));
    source
}

/// A consumer-built drop target. `accepts_inside` picks the band rule; the
/// commit result is whatever `commit` returns, so a fixture can prove the
/// rejected and failed terminals as well as the committed one.
fn traced_target(
    id: &str,
    label: &str,
    trace: &Arc<Mutex<Vec<String>>>,
    accepts_inside: bool,
    order: i32,
    commit: NodeDropCommit,
) -> NodeDropTarget {
    let mut target = NodeDropTarget::new(id, CUSTOM_SUBJECT_KIND, label);
    target.resolve_position = Some(poodle_render::vertical_band_resolver(accepts_inside));
    target.keyboard_order = Some(order);
    target.resolve_keyboard_position = Some(poodle_render::linear_keyboard_resolver());

    let intent_trace = Arc::clone(trace);
    let intent_id = id.to_string();
    target.on_intent = Some(Arc::new(move |event: &NodeDropIntentEvent| {
        push_trace(
            &intent_trace,
            format!("intent:{intent_id}:{}:{}", event.position, event.subject.id),
        );
    }));
    let cleared_trace = Arc::clone(trace);
    let cleared_id = id.to_string();
    target.on_intent_cleared = Some(Arc::new(move || {
        push_trace(&cleared_trace, format!("cleared:{cleared_id}"));
    }));
    let drop_trace = Arc::clone(trace);
    let drop_id = id.to_string();
    target.on_drop = Some(Arc::new(move |event: &NodeDropCommitEvent| {
        push_trace(
            &drop_trace,
            format!(
                "drop:{drop_id}:{}:{}",
                event.intent.position, event.subject.id
            ),
        );
        commit.clone()
    }));
    target
}

fn drag_box(id: &str, width: f32, height: f32) -> Node {
    let mut node = Node::container();
    node.id = Some(id.to_owned());
    node.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    node.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    node.style.descriptor.background = Some(ColorValue(0.2, 0.3, 0.4, 1.0));
    node
}

/// Source plus two flat sibling targets — the smallest complete surface.
fn custom_drag_tree(trace: &Arc<Mutex<Vec<String>>>, disabled_source: bool) -> Node {
    custom_drag_tree_with(trace, disabled_source, NodeDropCommit::Committed, true)
}

fn custom_drag_tree_with(
    trace: &Arc<Mutex<Vec<String>>>,
    disabled_source: bool,
    commit: NodeDropCommit,
    zone_b_present: bool,
) -> Node {
    let mut source = drag_box("custom-source", 80.0, 80.0);
    source.interaction.focusable = true;
    source.a11y.tab_index = Some(0);
    if !disabled_source {
        source.interaction.drag_source = Some(traced_source("custom-source", "Alpha", trace));
    }

    let mut zone_a = drag_box("custom-zone-a", 80.0, 80.0);
    zone_a.interaction.drop_target = Some(traced_target(
        "custom-zone-a",
        "Zone A",
        trace,
        false,
        1,
        commit.clone(),
    ));

    let mut row = Node::container();
    row.id = Some("custom-row".to_owned());
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.width = LayoutSizing::Fixed(240.0);
    row.style.descriptor.layout.height = LayoutSizing::Fixed(80.0);
    let mut row = row.child(source).child(zone_a);
    if zone_b_present {
        let mut zone_b = drag_box("custom-zone-b", 80.0, 80.0);
        zone_b.interaction.drop_target = Some(traced_target(
            "custom-zone-b",
            "Zone B",
            trace,
            false,
            2,
            commit,
        ));
        row = row.child(zone_b);
    }
    row
}

fn trace_of(trace: &Arc<Mutex<Vec<String>>>) -> Vec<String> {
    trace.lock().expect("trace lock").clone()
}

fn count_starting_with(events: &[String], prefix: &str) -> usize {
    events
        .iter()
        .filter(|event| event.starts_with(prefix))
        .count()
}

/// g16.025. A custom Rust/GPUI surface runs the shared semantic kernel: one
/// start after the runtime drag threshold, one current intent carrying the
/// resolved band, one revalidated commit, and exactly one terminal — through
/// real mounted mouse dispatch.
#[test]
fn a_custom_surface_runs_the_shared_kernel_from_pickup_to_commit() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(
            trace_of(&trace),
            ["start:custom-source"],
            "start fires once, after the runtime drag threshold"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // Upper half of a flat target is `before`, lower half `after`; moving
        // between bands on ONE target must re-report the intent.
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.25));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        let events = trace_of(&trace);
        assert!(
            events.contains(&"intent:custom-zone-a:before:custom-source".to_owned()),
            "{events:?}"
        );
        assert!(
            events.contains(&"intent:custom-zone-a:after:custom-source".to_owned()),
            "{events:?}"
        );
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-a")
        );

        // Moving to the neighbour clears the first target exactly once.
        driver.pointer_drag(payload_frac("custom-zone-b", 0.5, 0.75));
        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "cleared:custom-zone-a"),
            1,
            "{events:?}"
        );
        assert!(
            events.contains(&"intent:custom-zone-b:after:custom-source".to_owned()),
            "{events:?}"
        );

        driver.pointer_release(payload_frac("custom-zone-b", 0.5, 0.75));
        let events = trace_of(&trace);
        let start = events.iter().position(|e| e == "start:custom-source");
        let drop = events
            .iter()
            .position(|e| e == "drop:custom-zone-b:after:custom-source");
        let end = events
            .iter()
            .position(|e| e == "end:committed:custom-zone-b:after");
        assert!(start < drop && drop < end, "{events:?}");
        assert_eq!(count_starting_with(&events, "end:"), 1, "{events:?}");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        // A later release must not produce a second terminal.
        driver.pointer_release(payload_frac("custom-zone-b", 0.5, 0.75));
        assert_eq!(
            count_starting_with(&trace_of(&trace), "end:"),
            1,
            "a release after the session ended is inert"
        );
    });
}

/// g16.025. Release outside every target is cancellation, not a drop, and it
/// still ends exactly once. `on_drag_move` is capture-phase and hitbox-free,
/// so the session keeps receiving movement well outside the source — the
/// observable result `in_window_capture` claims.
#[test]
fn releasing_outside_every_target_cancels_once_and_commits_nothing() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        driver.pointer_drag(point(px(4.0), px(4.0)));
        assert_eq!(
            controller.snapshot().pointer,
            Some((4.0, 4.0)),
            "movement outside every target still reaches the session"
        );
        driver.pointer_release(point(px(4.0), px(4.0)));

        let events = trace_of(&trace);
        assert!(events.contains(&"cleared:custom-zone-a".to_owned()), "{events:?}");
        assert_eq!(count_starting_with(&events, "drop:"), 0, "{events:?}");
        assert_eq!(
            count_starting_with(&events, "end:cancelled:"),
            1,
            "{events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.025. Escape cancels once; a second Escape is inert because the phase no
/// longer accepts it — exactly-once is a property of the lifecycle, not of a
/// guard flag the adapter maintains.
#[test]
fn escape_cancels_once_and_a_second_escape_is_inert() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-b", 0.5, 0.75));
        driver.dispatch_key("escape");

        let events = trace_of(&trace);
        assert!(events.contains(&"cleared:custom-zone-b".to_owned()), "{events:?}");
        assert_eq!(count_starting_with(&events, "drop:"), 0, "{events:?}");
        assert_eq!(
            count_starting_with(&events, "end:cancelled:Escape"),
            1,
            "{events:?}"
        );

        driver.dispatch_key("escape");
        assert_eq!(
            count_starting_with(&trace_of(&trace), "end:"),
            1,
            "a repeated Escape must not emit another terminal"
        );
    });
}

/// g16.025. Nested overlapping targets: the deepest live one wins, and when it
/// stops being eligible the shallower ancestor takes the intent — without a
/// pointer move, because a rebuild is enough to change eligibility.
#[test]
fn nested_targets_arbitrate_deepest_first_and_follow_a_live_eligibility_change() {
    fn nested_tree(trace: &Arc<Mutex<Vec<String>>>, inner_enabled: bool) -> Node {
        let mut source = drag_box("nested-source", 60.0, 60.0);
        source.interaction.drag_source = Some(traced_source("nested-source", "Alpha", trace));

        let mut inner = drag_box("nested-inner", 90.0, 60.0);
        let mut inner_target =
            traced_target("nested-inner", "Inner", trace, false, 2, NodeDropCommit::Committed);
        inner_target.disabled = !inner_enabled;
        // Priority deliberately favours the OUTER target: depth must beat
        // priority, or a nested surface can never take its own drop.
        inner_target.priority = -50;
        inner.interaction.drop_target = Some(inner_target);

        let mut outer = drag_box("nested-outer", 90.0, 60.0);
        let mut outer_target =
            traced_target("nested-outer", "Outer", trace, false, 1, NodeDropCommit::Committed);
        outer_target.priority = 50;
        outer.interaction.drop_target = Some(outer_target);
        let outer = outer.child(inner);

        let mut row = Node::container();
        row.id = Some("nested-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(150.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(60.0);
        row.child(source).child(outer)
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(nested_tree(&trace, true)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 200.0, 80.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("nested-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("nested-inner", 0.5, 0.75));

        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("nested-inner"),
            "the deepest live target wins even against a higher-priority ancestor"
        );

        // Rebuild with the inner target disabled: the intent must move to the
        // surviving ancestor without any further pointer input.
        *node.lock().expect("mount lock") = nested_tree(&trace, false);
        driver.draw_frame();

        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_id.as_deref(),
            Some("nested-outer"),
            "a live eligibility change re-arbitrates: {:?}",
            trace_of(&trace)
        );
        assert_eq!(snapshot.phase, DragSessionPhase::Dragging);

        driver.pointer_release(payload_frac("nested-inner", 0.5, 0.75));
        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:nested-inner"),
            0,
            "a stale target must never commit: {events:?}"
        );
        assert_eq!(
            count_starting_with(&events, "drop:nested-outer"),
            1,
            "{events:?}"
        );
        assert_eq!(count_starting_with(&events, "end:"), 1, "{events:?}");
    });
}

/// g16.025. Removing the active source mid-drag cancels the session once and
/// commits nothing. Host rebuild is the ordinary path here: a source that did
/// not re-register this frame is gone, and the kernel's `SourceLost` closes it.
#[test]
fn removing_the_dragged_source_during_a_rebuild_cancels_once() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // Same tree, no source registration: the host rebuilt without it.
        *node.lock().expect("mount lock") = custom_drag_tree(&trace, true);
        driver.draw_frame();

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "end:cancelled:SourceLost"),
            1,
            "{events:?}"
        );
        assert!(events.contains(&"cleared:custom-zone-a".to_owned()), "{events:?}");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        driver.pointer_release(payload_frac("custom-zone-a", 0.5, 0.75));
        let events = trace_of(&trace);
        assert_eq!(count_starting_with(&events, "drop:"), 0, "{events:?}");
        assert_eq!(count_starting_with(&events, "end:"), 1, "{events:?}");
    });
}

/// g16.025. Removing the target that holds the current intent takes the
/// kernel's `TargetLost` path: cancelled once, nothing committed.
#[test]
fn removing_the_current_target_during_a_rebuild_cancels_once() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-b", 0.5, 0.75));
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-b")
        );

        *node.lock().expect("mount lock") =
            custom_drag_tree_with(&trace, false, NodeDropCommit::Committed, false);
        driver.draw_frame();

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "end:cancelled:TargetLost"),
            1,
            "{events:?}"
        );
        // The registration contract promises the target that holds the intent
        // is told when it stops — including when what stopped it was its own
        // removal. Looking the callback up in the swept registry lost it.
        assert_eq!(
            count_starting_with(&events, "cleared:custom-zone-b"),
            1,
            "a removed current target is still told it stopped: {events:?}"
        );
        assert_eq!(count_starting_with(&events, "drop:"), 0, "{events:?}");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.025. Two providers in one window own two sessions. A drag started in
/// one must not move, clear, or commit anything in the other, and the second
/// provider must stay idle throughout.
#[test]
fn two_providers_own_independent_sessions() {
    run_headless(|cx| {
        let left_trace = Arc::new(Mutex::new(Vec::new()));
        let right_trace = Arc::new(Mutex::new(Vec::new()));
        let left_controller = poodle_gpui_node_backend::DragDropController::new();
        let right_controller = poodle_gpui_node_backend::DragDropController::new();

        let left_node = Arc::new(Mutex::new(scoped_drag_tree("left", &left_trace)));
        let right_node = Arc::new(Mutex::new(scoped_drag_tree("right", &right_trace)));

        let build = {
            let left_controller = left_controller.clone();
            let right_controller = right_controller.clone();
            let left_node = Arc::clone(&left_node);
            let right_node = Arc::clone(&right_node);
            Rc::new(move || {
                let left = left_node.lock().expect("left lock").clone();
                let right = right_node.lock().expect("right lock").clone();
                use gpui::{IntoElement as _, ParentElement as _, Styled as _};
                gpui::div()
                    .flex()
                    .flex_col()
                    .child(
                        poodle_gpui_node_backend::drag_drop_provider(&left_controller, || {
                            gpui::div().child(poodle_gpui_node_backend::to_gpui(&left))
                        }),
                    )
                    .child(
                        poodle_gpui_node_backend::drag_drop_provider(&right_controller, || {
                            gpui::div().child(poodle_gpui_node_backend::to_gpui(&right))
                        }),
                    )
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        assert_ne!(left_controller.id(), right_controller.id());

        let source = payload_frac("left-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("left-zone-a", 0.5, 0.75));

        assert_eq!(left_controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            right_controller.snapshot().phase,
            DragSessionPhase::Idle,
            "a neighbouring provider must not join someone else's gesture"
        );

        // Move over the OTHER provider's target. It is not in this session's
        // registry, so no intent may resolve and its callbacks stay silent.
        driver.pointer_drag(payload_frac("right-zone-a", 0.5, 0.75));
        assert_eq!(
            left_controller.snapshot().target_id, None,
            "a target in another provider is not a candidate"
        );
        assert!(
            trace_of(&right_trace).is_empty(),
            "{:?}",
            trace_of(&right_trace)
        );

        driver.pointer_release(payload_frac("right-zone-a", 0.5, 0.75));
        let left = trace_of(&left_trace);
        assert_eq!(count_starting_with(&left, "end:cancelled:"), 1, "{left:?}");
        assert!(trace_of(&right_trace).is_empty());
        assert_eq!(right_controller.snapshot().phase, DragSessionPhase::Idle);

        // The second provider still works afterwards, with its own session id.
        let right_source = payload_frac("right-source", 0.5, 0.5);
        driver.pointer_press(right_source);
        driver.pointer_drag(point(px(f32::from(right_source.x) + 4.0), right_source.y));
        driver.pointer_drag(payload_frac("right-zone-a", 0.5, 0.75));
        driver.pointer_release(payload_frac("right-zone-a", 0.5, 0.75));

        let right = trace_of(&right_trace);
        assert_eq!(
            count_starting_with(&right, "drop:right-zone-a:after"),
            1,
            "{right:?}"
        );
        assert_eq!(count_starting_with(&right, "end:"), 1, "{right:?}");
        assert_eq!(
            count_starting_with(&trace_of(&left_trace), "end:"),
            1,
            "the first provider's session must not reopen"
        );
    });
}

/// A source and two targets under one id prefix, so two providers can mount
/// structurally identical surfaces without sharing an element id.
fn scoped_drag_tree(scope: &str, trace: &Arc<Mutex<Vec<String>>>) -> Node {
    let source_id = format!("{scope}-source");
    let zone_id = format!("{scope}-zone-a");

    let mut source = drag_box(&source_id, 60.0, 40.0);
    source.interaction.focusable = true;
    source.a11y.tab_index = Some(0);
    source.interaction.drag_source = Some(traced_source(&source_id, "Alpha", trace));

    let mut zone = drag_box(&zone_id, 60.0, 40.0);
    zone.interaction.drop_target = Some(traced_target(
        &zone_id,
        "Zone A",
        trace,
        false,
        1,
        NodeDropCommit::Committed,
    ));

    let mut row = Node::container();
    row.id = Some(format!("{scope}-row"));
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
    row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
    row.child(source).child(zone)
}

// ── Cross-window host bridge (g16.026) ─────────────────────────────────────
//
// A stub host, driven by the test. It records every command the controller
// sends it and answers on demand, because most of these claims are about what
// the controller did *not* do — start an unarmed gesture, commit a stale
// projection, act on a completion for a session that is gone.

#[derive(Default)]
struct HostLog {
    prepares: Vec<String>,
    starts: Vec<String>,
    stops: Vec<String>,
    cancels: Vec<(String, poodle_node::DragCancelReason)>,
    commits: Vec<String>,
    picks: Vec<String>,
    aborts: Vec<poodle_node::DragCancelReason>,
}

type PendingPrepare = (
    poodle_node::CrossWindowAbort,
    poodle_node::CrossWindowPrepareComplete,
);

#[derive(Default)]
struct HostStubState {
    log: HostLog,
    pending_prepare: Vec<PendingPrepare>,
    terminal: Option<poodle_node::CrossWindowTerminal>,
    listener: Option<Box<dyn Fn(poodle_node::CrossWindowDragTargetEvent) + Send>>,
    pending_commit: Option<(
        poodle_node::CrossWindowAbort,
        poodle_node::CrossWindowCommitComplete,
    )>,
    pending_pick: Option<(
        poodle_node::CrossWindowDragReceipt,
        poodle_node::CrossWindowAbort,
        Box<dyn FnOnce(Option<poodle_node::CrossWindowDragProjection>) + Send>,
    )>,
}

#[derive(Clone, Default)]
struct HostStub {
    state: Arc<Mutex<HostStubState>>,
    keyboard_picker: bool,
}

impl HostStub {
    fn log<T>(&self, read: impl FnOnce(&HostLog) -> T) -> T {
        read(&self.state.lock().expect("host state").log)
    }

    /// Answer the n-th outstanding preparation.
    fn settle_prepare(&self, index: usize, token: Option<&str>) {
        let entry = {
            let mut state = self.state.lock().expect("host state");
            if index >= state.pending_prepare.len() {
                return;
            }
            state.pending_prepare.remove(index)
        };
        let (abort, complete) = entry;
        // A real host watches the signal; this one records that it fired and
        // answers anyway, so the *controller's* handling of a late answer is
        // what the test measures.
        if let Some(reason) = abort.reason() {
            self.state.lock().expect("host state").log.aborts.push(reason);
        }
        complete(token.map(|token| poodle_node::CrossWindowDragReceipt {
            protocol_version: poodle_node::CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
            token: token.to_string(),
        }));
    }

    fn report_terminal(&self, outcome: poodle_node::DragTerminalOutcome) {
        let terminal = {
            let state = self.state.lock().expect("host state");
            state.terminal.is_some()
        };
        if !terminal {
            return;
        }
        let state = self.state.lock().expect("host state");
        if let Some(callback) = state.terminal.as_ref() {
            callback(outcome);
        }
    }

    fn project(&self, projection: poodle_node::CrossWindowDragProjection) {
        let state = self.state.lock().expect("host state");
        if let Some(listener) = state.listener.as_ref() {
            listener(poodle_node::CrossWindowDragTargetEvent::Projection { projection });
        }
    }

    fn cancel_from_host(
        &self,
        receipt: poodle_node::CrossWindowDragReceipt,
        reason: poodle_node::DragCancelReason,
    ) {
        let state = self.state.lock().expect("host state");
        if let Some(listener) = state.listener.as_ref() {
            listener(poodle_node::CrossWindowDragTargetEvent::Cancelled { receipt, reason });
        }
    }

    fn settle_commit(&self, result: poodle_node::DragDropCommitResult) {
        let entry = self.state.lock().expect("host state").pending_commit.take();
        if let Some((abort, complete)) = entry {
            if let Some(reason) = abort.reason() {
                self.state.lock().expect("host state").log.aborts.push(reason);
            }
            complete(result);
        }
    }

    fn settle_pick(&self, projection: Option<poodle_node::CrossWindowDragProjection>) {
        let entry = self.state.lock().expect("host state").pending_pick.take();
        if let Some((_receipt, abort, complete)) = entry {
            if let Some(reason) = abort.reason() {
                self.state.lock().expect("host state").log.aborts.push(reason);
            }
            complete(projection);
        }
    }
}

impl poodle_node::CrossWindowDragSourceBridge for HostStub {
    fn capabilities(&self) -> poodle_node::CrossWindowDragCapabilities {
        poodle_node::CrossWindowDragCapabilities {
            pointer: true,
            touch: false,
            keyboard_target_picker: self.keyboard_picker,
        }
    }

    fn prepare(
        &self,
        request: poodle_node::CrossWindowDragPrepareRequest,
        abort: poodle_node::CrossWindowAbort,
        complete: poodle_node::CrossWindowPrepareComplete,
    ) {
        let mut state = self.state.lock().expect("host state");
        state.log.prepares.push(request.session_id);
        state.pending_prepare.push((abort, complete));
    }

    fn start(
        &self,
        receipt: poodle_node::CrossWindowDragReceipt,
        transport: poodle_node::CrossWindowDragTransport,
        on_terminal: poodle_node::CrossWindowTerminal,
    ) -> poodle_node::CrossWindowCleanup {
        let token = receipt.token.clone();
        {
            let mut state = self.state.lock().expect("host state");
            state.log.starts.push(format!("{token}:{transport:?}"));
            state.terminal = Some(on_terminal);
        }
        let stub = self.clone();
        Box::new(move || {
            let mut state = stub.state.lock().expect("host state");
            state.terminal = None;
            state.log.stops.push(token);
        })
    }

    fn cancel(
        &self,
        receipt: poodle_node::CrossWindowDragReceipt,
        reason: poodle_node::DragCancelReason,
    ) {
        self.state
            .lock()
            .expect("host state")
            .log
            .cancels
            .push((receipt.token, reason));
    }
}

impl poodle_node::CrossWindowDragTargetBridge for HostStub {
    fn capabilities(&self) -> poodle_node::CrossWindowDragCapabilities {
        poodle_node::CrossWindowDragCapabilities {
            pointer: true,
            touch: false,
            keyboard_target_picker: self.keyboard_picker,
        }
    }

    fn subscribe(
        &self,
        listener: Box<dyn Fn(poodle_node::CrossWindowDragTargetEvent) + Send>,
    ) -> poodle_node::CrossWindowCleanup {
        self.state.lock().expect("host state").listener = Some(listener);
        let stub = self.clone();
        Box::new(move || {
            stub.state.lock().expect("host state").listener = None;
        })
    }

    fn commit(
        &self,
        request: poodle_node::CrossWindowDragCommitRequest,
        abort: poodle_node::CrossWindowAbort,
        complete: poodle_node::CrossWindowCommitComplete,
    ) {
        let mut state = self.state.lock().expect("host state");
        state.log.commits.push(format!(
            "{}:{}:{}",
            request.receipt.token, request.intent.target_id, request.intent.position
        ));
        state.pending_commit = Some((abort, complete));
    }

    fn pick_target(
        &self,
        receipt: poodle_node::CrossWindowDragReceipt,
        abort: poodle_node::CrossWindowAbort,
        complete: Box<dyn FnOnce(Option<poodle_node::CrossWindowDragProjection>) + Send>,
    ) -> bool {
        if !self.keyboard_picker {
            return false;
        }
        // No special case for a receipt that names nothing: a host should never
        // be asked to pick outside a transaction, and a stub that tolerated it
        // would hide exactly that.
        let mut state = self.state.lock().expect("host state");
        state.log.picks.push(receipt.token.clone());
        state.pending_pick = Some((receipt, abort, complete));
        true
    }
}

fn receipt_for(token: &str) -> poodle_node::CrossWindowDragReceipt {
    poodle_node::CrossWindowDragReceipt {
        protocol_version: poodle_node::CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
        token: token.to_string(),
    }
}

fn projection_for(
    token: &str,
    target: Option<&str>,
) -> poodle_node::CrossWindowDragProjection {
    poodle_node::CrossWindowDragProjection {
        receipt: poodle_node::CrossWindowDragReceipt {
            protocol_version: poodle_node::CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
            token: token.to_string(),
        },
        source_id: "remote-source".to_string(),
        source_label: "Remote".to_string(),
        subject: poodle_node::DragSubject {
            kind: CUSTOM_SUBJECT_KIND.to_string(),
            id: "remote-row".to_string(),
        },
        operation: poodle_node::DragOperation::Move,
        input_kind: poodle_node::CrossWindowDragInputKind::Pointer,
        target_id: target.map(|value| value.to_string()),
        position: target.map(|_| "after".to_string()),
    }
}

/// g16.026. The source half of the split, end to end: preparation runs before
/// activation, the gesture cannot start until the receipt arms, `start`
/// installs the one authoritative terminal, and the host's refusal — not the
/// native end — is what ends the session.
#[test]
fn a_bridged_gpui_source_prepares_before_activation_and_ends_on_the_host_terminal() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("xw", &trace);
        attach_bridge(&mut node, "xw-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("xw-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));

        // Preparation ran, and the gesture is *not* dragging: the receipt has
        // not armed, so nothing may start.
        assert_eq!(host.log(|log| log.prepares.len()), 1);
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Preparing);
        assert!(host.log(|log| log.starts.is_empty()));

        host.settle_prepare(0, Some("lease-1"));
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            host.log(|log| log.starts.clone()),
            vec!["lease-1:WindowCapture".to_string()],
            "one authoritative terminal subscription"
        );

        // A release is not a result. The host still owns the transaction.
        driver.pointer_release(payload_frac("xw-zone-a", 0.5, 0.75));
        driver.draw_frame();

        host.report_terminal(poodle_node::DragTerminalOutcome::Rejected {
            reason: Some("lease expired".to_string()),
        });
        driver.draw_frame();

        let entries = trace.lock().expect("trace").clone();
        let terminals: Vec<&String> = entries
            .iter()
            .filter(|entry| entry.starts_with("end:"))
            .collect();
        assert_eq!(terminals.len(), 1, "exactly one terminal: {entries:?}");
        assert!(
            terminals[0].contains("rejected"),
            "the host's refusal is the result, not a cancellation: {:?}",
            terminals[0]
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(
            host.log(|log| log.cancels.is_empty()),
            "the host closed its own transaction; Poodle does not cancel it again"
        );
        assert_eq!(host.log(|log| log.stops.clone()), vec!["lease-1".to_string()]);

        // A repeat is inert.
        host.report_terminal(poodle_node::DragTerminalOutcome::Committed {
            intent: poodle_node::DropIntent {
                target_id: "xw-zone-a".to_string(),
                position: "after".to_string(),
                operation: poodle_node::DragOperation::Move,
                destination: None,
            },
        });
        driver.draw_frame();
        assert_eq!(
            trace
                .lock()
                .expect("trace")
                .iter()
                .filter(|entry| entry.starts_with("end:"))
                .count(),
            1
        );
    });
}

/// g16.026. A superseded preparation is aborted, its late receipt is handed
/// straight back, and it cannot arm the session that replaced it.
#[test]
fn a_superseded_gpui_preparation_is_aborted_and_its_late_receipt_returned() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("xw", &trace);
        attach_bridge(&mut node, "xw-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("xw-source", 0.5, 0.5);

        // A first gesture prepares, then is abandoned before it ever arms.
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(host.log(|log| log.prepares.len()), 1);
        driver.dispatch_key_raw("escape");
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        // A second gesture prepares afresh.
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(host.log(|log| log.prepares.len()), 2);
        let first_session = host.log(|log| log.prepares[0].clone());
        let second_session = host.log(|log| log.prepares[1].clone());
        assert_ne!(first_session, second_session);

        // The first host answer arrives late. It cannot arm the second session,
        // and the lease it allocated is handed back rather than leaked.
        host.settle_prepare(0, Some("stale-lease"));
        driver.draw_frame();

        assert!(
            host.log(|log| log.aborts.contains(&poodle_node::DragCancelReason::Escape)),
            "the abandoned preparation was told to stop: {:?}",
            host.log(|log| log.aborts.clone())
        );
        assert_eq!(
            host.log(|log| log.cancels.clone()),
            vec![(
                "stale-lease".to_string(),
                poodle_node::DragCancelReason::Superseded
            )],
            "the late lease is returned exactly once"
        );
        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Preparing,
            "the live session is still waiting for its own receipt"
        );
        assert!(host.log(|log| log.starts.is_empty()));
    });
}

/// g16.026. The window half of the split: an incoming projection starts one
/// session, this window re-runs its own eligibility, and the commit goes
/// through the host bridge — never through a local drop callback.
#[test]
fn an_incoming_gpui_projection_revalidates_locally_and_commits_through_the_host() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_cross_window_target_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(scoped_drag_tree("xw", &trace)));
        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        // A target this window does not have: refused, and no session survives
        // holding an intent for it.
        host.project(projection_for("lease-1", Some("not-in-this-window")));
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            controller.snapshot().target_id, None,
            "a target this window does not have resolves to no intent at all"
        );

        // A target it does have.
        host.project(projection_for("lease-1", Some("xw-zone-a")));
        driver.draw_frame();
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("xw-zone-a")
        );
        assert_eq!(
            controller.snapshot().target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Accepted),
            "accepted, not merely named: the snapshot reports a refusal the same way"
        );

        driver.pointer_release(payload_frac("xw-zone-a", 0.5, 0.75));
        driver.draw_frame();

        assert_eq!(
            host.log(|log| log.commits.clone()),
            vec!["lease-1:xw-zone-a:after".to_string()],
            "the host bridge commits, exactly once"
        );

        host.settle_commit(poodle_node::DragDropCommitResult::Committed);
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        let entries = trace.lock().expect("trace").clone();
        assert!(
            !entries.iter().any(|entry| entry.starts_with("drop:")),
            "a cross-window drop never reaches a local drop callback: {entries:?}"
        );
    });
}

/// g16.026. A commit the window abandoned mid-flight, and a pick that comes
/// back naming another transaction. Both are refused, and cleanup runs once.
#[test]
fn a_cancelled_gpui_commit_and_a_mismatched_pick_are_both_inert() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub {
            keyboard_picker: true,
            ..HostStub::default()
        };
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_cross_window_target_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(scoped_drag_tree("xw", &trace)));
        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        // A keyboard projection asks the host to pick, bound to this receipt.
        let mut keyboard = projection_for("lease-1", None);
        keyboard.input_kind = poodle_node::CrossWindowDragInputKind::Keyboard;
        host.project(keyboard);
        driver.draw_frame();
        assert_eq!(
            host.log(|log| log.picks.clone()),
            vec!["lease-1".to_string()],
            "the picker is bound to the exact receipt"
        );

        let live_session = controller.snapshot().session_id.clone();

        // The host answers with a projection for a *different* transaction.
        // Refusing it at the picker is what leaves the live transaction alone;
        // letting it through would supersede a transaction this window is
        // still holding, which is the damage the receipt binding prevents.
        host.settle_pick(Some(projection_for("someone-elses-lease", Some("xw-zone-a"))));
        driver.draw_frame();
        assert_eq!(
            controller.snapshot().target_id,
            None,
            "a pick naming another receipt is refused, not trusted"
        );
        assert_eq!(
            controller.snapshot().session_id, live_session,
            "and the live transaction is untouched"
        );

        // Proof that it really is untouched: the host can still project into it.
        let mut still_live = projection_for("lease-1", Some("xw-zone-a"));
        still_live.input_kind = poodle_node::CrossWindowDragInputKind::Keyboard;
        host.project(still_live);
        driver.draw_frame();
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("xw-zone-a"),
            "the transaction the mismatched pick did not disturb still resolves"
        );

        // A keyboard transaction is not released by a mouse-up, so the commit
        // case is its own pointer transaction — a different receipt, as it
        // would be in a real host.
        host.cancel_from_host(receipt_for("lease-1"), poodle_node::DragCancelReason::Explicit);
        driver.draw_frame();

        host.project(projection_for("lease-2", Some("xw-zone-a")));
        driver.draw_frame();
        driver.pointer_release(payload_frac("xw-zone-a", 0.5, 0.75));
        driver.draw_frame();
        assert_eq!(
            host.log(|log| log.commits.clone()),
            vec!["lease-2:xw-zone-a:after".to_string()]
        );

        host.cancel_from_host(receipt_for("lease-2"), poodle_node::DragCancelReason::WindowLost);
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        // The host's late answer to the abandoned commit changes nothing — and,
        // more to the point, the host was *told* to stop. A window that
        // abandons a request without signalling it leaves the host working on
        // a transaction nobody is waiting for.
        host.settle_commit(poodle_node::DragDropCommitResult::Committed);
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(
            host.log(|log| log.aborts.clone()),
            vec![poodle_node::DragCancelReason::WindowLost],
            "the abandoned commit carried the reason it was abandoned for"
        );

        let entries = trace.lock().expect("trace").clone();
        assert!(
            !entries.iter().any(|entry| entry.starts_with("drop:")),
            "no local drop callback ran: {entries:?}"
        );
    });
}


/// g16.026 round 2. A late preparation receipt goes back to the host that
/// allocated it, and to no other.
///
/// Two sources with two *different* hosts. A prepares and is abandoned; B
/// prepares and is still live when A's host finally answers. Returning A's
/// lease through B would both leak A's and issue a command B never made — and
/// a single shared stub cannot tell the two apart, which is why this one uses
/// two.
#[test]
fn a_late_receipt_is_returned_to_the_host_that_allocated_it() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host_a = HostStub::default();
        let host_b = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut row = Node::container();
        row.id = Some("xw-pair".to_string());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(240.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        let mut left = scoped_drag_tree("a", &trace);
        attach_bridge(&mut left, "a-source", Arc::new(host_a.clone()));
        let mut right = scoped_drag_tree("b", &trace);
        attach_bridge(&mut right, "b-source", Arc::new(host_b.clone()));
        let node = Arc::new(Mutex::new(row.child(left).child(right)));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        // A prepares, then is abandoned before it arms.
        let a = payload_frac("a-source", 0.5, 0.5);
        driver.pointer_press(a);
        driver.pointer_drag(point(px(f32::from(a.x) + 4.0), a.y));
        assert_eq!(host_a.log(|log| log.prepares.len()), 1);
        driver.dispatch_key_raw("escape");
        driver.draw_frame();

        // B prepares and stays live.
        let b = payload_frac("b-source", 0.5, 0.5);
        driver.pointer_press(b);
        driver.pointer_drag(point(px(f32::from(b.x) + 4.0), b.y));
        assert_eq!(host_b.log(|log| log.prepares.len()), 1);
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Preparing);

        // A's host answers late.
        host_a.settle_prepare(0, Some("lease-a"));
        driver.drain();

        assert_eq!(
            host_a.log(|log| log.cancels.clone()),
            vec![(
                "lease-a".to_string(),
                poodle_node::DragCancelReason::Superseded
            )],
            "the allocating host gets its lease back, exactly once"
        );
        assert!(
            host_b.log(|log| log.cancels.is_empty()),
            "the live host is issued no command it never made: {:?}",
            host_b.log(|log| log.cancels.clone())
        );
        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Preparing,
            "B's preparation is untouched and still waiting for its own receipt"
        );
        assert!(host_b.log(|log| log.starts.is_empty()));
    });
}

/// g16.026 round 2. An asynchronous host answer wakes the window by itself.
///
/// The contract explicitly permits a host to answer whenever its lease
/// resolves. Queueing the answer and waiting for the next incidental frame
/// leaves an otherwise idle window in `Preparing` forever, so the only thing
/// this test does after the answer is let the async runtime run — no draw, no
/// unrelated input.
#[test]
fn an_asynchronous_host_answer_advances_the_session_without_a_manufactured_frame() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("xw", &trace);
        attach_bridge(&mut node, "xw-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("xw-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Preparing);

        host.settle_prepare(0, Some("lease-1"));

        // The only thing that happens next is the async runtime running.
        driver.drain();

        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Dragging,
            "the host's own answer woke the window"
        );
        assert_eq!(
            host.log(|log| log.starts.clone()),
            vec!["lease-1:WindowCapture".to_string()]
        );

        // And the terminal takes the same route.
        host.report_terminal(poodle_node::DragTerminalOutcome::Rejected {
            reason: Some("lease expired".to_string()),
        });
        driver.drain();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.026 round 2. Installing a target bridge asks the host for nothing.
///
/// A capability probe at installation is an observable request outside any
/// transaction, and it forces implementations to special-case a receipt that
/// names nothing. The declared capability is trusted until a real keyboard
/// pick needs it.
#[test]
fn installing_a_target_bridge_makes_no_host_request() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub {
            keyboard_picker: true,
            ..HostStub::default()
        };
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_cross_window_target_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(scoped_drag_tree("xw", &trace)));
        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();
        driver.drain();

        assert!(
            host.log(|log| log.picks.is_empty()),
            "installation picked nothing: {:?}",
            host.log(|log| log.picks.clone())
        );

        // The picker is reached on a real request, bound to a live receipt.
        let mut keyboard = projection_for("lease-1", None);
        keyboard.input_kind = poodle_node::CrossWindowDragInputKind::Keyboard;
        host.project(keyboard);
        driver.drain();
        assert_eq!(host.log(|log| log.picks.clone()), vec!["lease-1".to_string()]);
    });
}

/// g16.026 round 2. Replacing the window's bridge mid-projection ends the
/// transaction the outgoing host owned.
///
/// The outgoing host is about to stop being subscribed, so a projection left
/// open behind it is a session nothing can cancel — and its receipt must never
/// reach the incoming host, which never issued it.
#[test]
fn replacing_the_target_bridge_ends_the_outgoing_transaction() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host_a = HostStub::default();
        let host_b = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_cross_window_target_bridge(Arc::new(host_a.clone()), cx));

        let node = Arc::new(Mutex::new(scoped_drag_tree("xw", &trace)));
        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        host_a.project(projection_for("lease-a", Some("xw-zone-a")));
        driver.drain();
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("xw-zone-a")
        );

        // A publishes again — this one is only *queued*, not yet drained.
        host_a.project(projection_for("lease-a-late", Some("xw-zone-a")));

        // Swap the window's bridge while A's transaction is live and A's
        // second message is still in flight.
        driver.update_app(|cx| {
            controller.set_cross_window_target_bridge(Arc::new(host_b.clone()), cx)
        });
        driver.drain();

        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Idle,
            "the outgoing host's transaction is ended, not stranded"
        );
        assert_eq!(
            controller.snapshot().target_id, None,
            "and A's queued news did not start a transaction under B"
        );

        // A release now cannot commit A's receipt anywhere, least of all to B.
        driver.pointer_release(payload_frac("xw-zone-a", 0.5, 0.75));
        driver.drain();
        assert!(
            host_b.log(|log| log.commits.is_empty()),
            "B is never sent a receipt it did not issue: {:?}",
            host_b.log(|log| log.commits.clone())
        );
        assert!(host_a.log(|log| log.commits.is_empty()));

        // B still works on its own terms, with its own receipt.
        host_b.project(projection_for("lease-b", Some("xw-zone-a")));
        driver.drain();
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("xw-zone-a"),
            "the replacement is live, not merely inert"
        );
        driver.pointer_release(payload_frac("xw-zone-a", 0.5, 0.75));
        driver.drain();
        assert_eq!(
            host_b.log(|log| log.commits.clone()),
            vec!["lease-b:xw-zone-a:after".to_string()]
        );
        assert!(host_a.log(|log| log.commits.is_empty()));
    });
}

/// Attach a host source bridge to one registered source in a built tree.
fn attach_bridge(
    node: &mut Node,
    source_id: &str,
    bridge: Arc<dyn poodle_node::CrossWindowDragSourceBridge>,
) {
    if let Some(source) = node.interaction.drag_source.as_mut() {
        if source.source_id == source_id {
            source.cross_window_source_bridge = Some(bridge);
            return;
        }
    }
    for child in node.children.iter_mut() {
        attach_bridge(child, source_id, Arc::clone(&bridge));
    }
}


/// g16.026, carried from g16.025. **Two windows, no false cancel.**
///
/// This is the counterexample that sank the first attempt. That version kept a
/// thread-global "did this controller sweep this frame" mark, so rendering
/// window A reset and swept controllers owned by window B — and cancelled a
/// live drag in B merely because B did not render during A's frame.
///
/// Here B starts a real drag and then simply stops being drawn, which is what
/// a background window does. A is then mounted and drawn repeatedly. B's
/// session, registrations, and census must all survive it.
#[test]
fn one_window_frame_cannot_cancel_another_windows_live_drag() {
    run_headless(|cx| {
        let background_trace = Arc::new(Mutex::new(Vec::new()));
        let background = poodle_gpui_node_backend::DragDropController::new();
        let background_host;

        {
            let node = Arc::new(Mutex::new(scoped_drag_tree("bg", &background_trace)));
            let build = {
                let controller = background.clone();
                let node = Arc::clone(&node);
                Rc::new(move || {
                    let tree = node.lock().expect("bg lock").clone();
                    use gpui::{IntoElement as _, ParentElement as _};
                    gpui::div()
                        .child(poodle_gpui_node_backend::drag_drop_provider(
                            &controller,
                            || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                        ))
                        .into_any_element()
                }) as Rc<dyn Fn() -> gpui::AnyElement>
            };

            let mut driver = HeadlessDriver::new_element(cx, build);
            driver.draw_frame();
            background_host = driver.drag_host();

            let source = payload_frac("bg-source", 0.5, 0.5);
            driver.pointer_press(source);
            driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
            driver.pointer_drag(payload_frac("bg-zone-a", 0.5, 0.75));

            assert_eq!(
                background.snapshot().phase,
                DragSessionPhase::Dragging,
                "the background window is mid-drag before the other window opens"
            );
            assert_eq!(
                background.snapshot().target_id.as_deref(),
                Some("bg-zone-a")
            );
            // The mount host's own provider plus the nested one under test.
            assert_eq!(background_host.census_len(), 2);
        }
        // The driver is gone; the window and its live drag are not. From here
        // the background window never draws another frame.

        let foreground_trace = Arc::new(Mutex::new(Vec::new()));
        let foreground = poodle_gpui_node_backend::DragDropController::new();
        let node = Arc::new(Mutex::new(scoped_drag_tree("fg", &foreground_trace)));
        let build = {
            let controller = foreground.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("fg lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        for _ in 0..5 {
            driver.draw_frame();
        }

        assert_eq!(
            background.snapshot().phase,
            DragSessionPhase::Dragging,
            "another window's frames must not cancel a live drag; trace={:?} census={}",
            background_trace.lock().expect("trace"),
            background_host.census_len()
        );
        assert_eq!(
            background.snapshot().target_id.as_deref(),
            Some("bg-zone-a"),
            "another window's frames must not prune the live registrations"
        );
        assert_eq!(
            background_host.census_len(),
            2,
            "a window's census belongs to that window and nothing else may empty it"
        );
        assert!(
            !background_trace
                .lock()
                .expect("trace")
                .iter()
                .any(|entry| entry.starts_with("end:")),
            "no terminal ran in the background window: {:?}",
            background_trace.lock().expect("trace")
        );

        // And the foreground window built and swept normally while it did so.
        // Its own drag is not started here: GPUI's active drag is app-wide, so
        // a second simultaneous native gesture would be testing the runtime
        // rather than the census.
        assert_eq!(driver.drag_host().census_len(), 2);
        assert_eq!(foreground.source_ids(), vec!["fg-source".to_string()]);
        assert_eq!(foreground.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.026, carried from g16.025. **Native drag actually stops.**
///
/// A controller can only close a session during its own per-frame sweep, and
/// an unmounted provider never sweeps again — so removing a provider mid-drag
/// used to leave a `Dragging` session, live registrations, no terminal, and
/// GPUI's own drag still in flight. Semantic idle is not enough here: the
/// runtime's active drag and preview have to be gone too, which is the second
/// claim the earlier attempt could not make.
#[test]
fn unmounting_a_provider_mid_drag_cancels_it_and_stops_the_native_drag() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let inner = poodle_gpui_node_backend::DragDropController::new();
        let mounted = Arc::new(Mutex::new(true));
        let node = Arc::new(Mutex::new(scoped_drag_tree("inner", &trace)));

        let build = {
            let controller = inner.clone();
            let node = Arc::clone(&node);
            let mounted = Arc::clone(&mounted);
            Rc::new(move || {
                use gpui::{IntoElement as _, ParentElement as _};
                let root = gpui::div();
                if !*mounted.lock().expect("mounted lock") {
                    return root.into_any_element();
                }
                let tree = node.lock().expect("inner lock").clone();
                root.child(poodle_gpui_node_backend::drag_drop_provider(
                    &controller,
                    || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                ))
                .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();
        let host = driver.drag_host();
        // Two providers in this window: the mount host's own, and the nested
        // one under test.
        assert_eq!(host.census_len(), 2);

        let source = payload_frac("inner-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("inner-zone-a", 0.5, 0.75));

        assert_eq!(inner.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(inner.source_ids(), vec!["inner-source".to_string()]);
        assert!(
            driver.has_active_native_drag(),
            "the runtime owns a drag before the provider disappears"
        );

        // The host removes the provider mid-drag.
        *mounted.lock().expect("mounted lock") = true;
        *mounted.lock().expect("mounted lock") = false;
        driver.draw_frame();

        assert_eq!(
            inner.snapshot().phase,
            DragSessionPhase::Idle,
            "an absent provider's session is cancelled once"
        );
        assert!(
            inner.source_ids().is_empty() && inner.target_ids().is_empty(),
            "its registrations are dropped: {:?} {:?}",
            inner.source_ids(),
            inner.target_ids()
        );
        assert!(
            !driver.has_active_native_drag(),
            "semantic idle is not enough: GPUI's own drag and preview must be gone too"
        );
        assert_eq!(host.census_len(), 1, "the host forgets the departed provider");

        let entries = trace.lock().expect("trace").clone();
        let terminals: Vec<&String> = entries
            .iter()
            .filter(|entry| entry.starts_with("end:"))
            .collect();
        assert_eq!(
            terminals.len(),
            1,
            "exactly one terminal, and it is a cancellation: {entries:?}"
        );
        assert!(terminals[0].contains("cancel"), "{:?}", terminals[0]);

        // A second frame with the provider still absent changes nothing.
        driver.draw_frame();
        assert_eq!(
            trace
                .lock()
                .expect("trace")
                .iter()
                .filter(|entry| entry.starts_with("end:"))
                .count(),
            1,
            "the cancellation does not repeat once the provider is forgotten"
        );
    });
}


/// g16.025. The keyboard route creates the same semantic session as the
/// pointer: pickup on a focused opted-in source, ordered traversal, one
/// revalidated commit, and the same terminal cleanup. It never calls the
/// component handler directly.
#[test]
fn keyboard_pickup_traversal_and_drop_use_the_same_session() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.wait_for_focus_handle("custom-source");
        driver.focus_element("custom-source");
        let controller = driver.drag();

        driver.dispatch_key_raw("space");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            controller.snapshot().input_kind,
            Some(NodeDragInputKind::Keyboard)
        );
        assert_eq!(trace_of(&trace), ["start:custom-source"]);

        driver.dispatch_key_raw("down");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-a"),
            "the first Next step lands on the first ordered target"
        );
        driver.dispatch_key_raw("down");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-b")
        );
        driver.dispatch_key_raw("up");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-a")
        );
        driver.dispatch_key_raw("end");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-b"),
            "End is explicit, not inferred from a synthetic point"
        );

        driver.dispatch_key_raw("enter");
        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:custom-zone-b:after"),
            1,
            "{events:?}"
        );
        assert_eq!(
            count_starting_with(&events, "end:committed:"),
            1,
            "{events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        let announcements = controller.announcements();
        assert!(
            announcements.first().is_some_and(|text| text.contains("Picked up Alpha")),
            "{announcements:?}"
        );
        assert!(
            announcements.last().is_some_and(|text| text.starts_with("Dropped Alpha")),
            "{announcements:?}"
        );
    });
}

/// g16.025. A target that rejects at commit time is a terminal rejection, not
/// a silent no-op: the drop handler ran, nothing committed, the session ended
/// once, and the reason reaches the announcement.
#[test]
fn a_rejected_commit_ends_the_session_with_its_reason() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree_with(
            &trace,
            false,
            NodeDropCommit::Rejected {
                reason: Some("Zone is locked".to_string()),
            },
            true,
        )));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        driver.pointer_release(payload_frac("custom-zone-a", 0.5, 0.75));

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:custom-zone-a:after"),
            1,
            "{events:?}"
        );
        assert_eq!(
            count_starting_with(&events, "end:rejected:Zone is locked"),
            1,
            "{events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(
            controller
                .announcements()
                .iter()
                .any(|text| text.contains("Zone is locked")),
            "{:?}",
            controller.announcements()
        );
    });
}

/// g16.025. The capability matrix is a statement about crates.io GPUI 0.2.2,
/// not about the fixture. Driving a complete mouse drag must leave `pen`,
/// `touch`, and `device_cancel` false and report the input kind honestly as
/// mouse: synthesized mouse input is never evidence for a device the crate
/// does not expose.
#[test]
fn a_mouse_fixture_cannot_make_an_unsupported_capability_true() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let before = controller.capabilities();
        assert_eq!(before, poodle_gpui_node_backend::GPUI_DRAG_CAPABILITIES);

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        assert_eq!(
            controller.snapshot().input_kind,
            Some(NodeDragInputKind::Mouse),
            "a synthesized mouse gesture reports itself as mouse"
        );
        driver.pointer_release(payload_frac("custom-zone-a", 0.5, 0.75));

        let after = controller.capabilities();
        assert_eq!(after, before, "capabilities are immutable");
        assert!(after.mouse && after.keyboard && after.in_window_capture);
        assert!(
            !after.pen && !after.touch && !after.device_cancel,
            "stock GPUI 0.2.2 exposes no pen identity, touch contact, or device cancel"
        );
    });
}

/// g16.025. A disabled source registers nothing, so no press, threshold, or
/// release can open a session.
#[test]
fn a_source_without_a_registration_is_completely_inert() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, true)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        driver.pointer_release(payload_frac("custom-zone-a", 0.5, 0.75));

        assert!(trace_of(&trace).is_empty(), "{:?}", trace_of(&trace));
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(controller.source_ids().is_empty());
        assert_eq!(controller.target_ids().len(), 2);
    });
}

/// g16.025. Duplicate live ids are errors, not last-writer-wins: the second
/// registration is refused and recorded, so two rows sharing an id cannot
/// silently steal one another's callbacks.
#[test]
fn a_duplicate_live_target_id_is_recorded_and_refused() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let mut row = custom_drag_tree(&trace, false);
        let mut clash = drag_box("custom-zone-clash", 40.0, 40.0);
        clash.interaction.drop_target = Some(traced_target(
            "custom-zone-a",
            "Clashing zone",
            &trace,
            false,
            3,
            NodeDropCommit::Committed,
        ));
        row = row.child(clash);

        let node = Arc::new(Mutex::new(row));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 320.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        assert_eq!(controller.target_ids().len(), 2, "the clash is refused");
        assert!(
            controller
                .conflicts()
                .iter()
                .any(|entry| entry.contains("duplicate live drop target id `custom-zone-a`")),
            "{:?}",
            controller.conflicts()
        );
    });
}

fn tab_at<'a>(root: &'a Node, runtime_id: &str) -> &'a Node {
    root.find(&|node| node.runtime_id.as_deref() == Some(runtime_id))
        .unwrap_or_else(|| panic!("{runtime_id} exists"))
}

/// g16.006. Tabs selection, focus, close, keyboard reorder, and pointer
/// reorder through real mounted GPUI input and controlled host rebuilds.
#[test]
fn tabs_drag_keyboard_and_identity_rebuild_the_host_spec() {
    #[derive(Clone)]
    struct TabsState {
        items: Vec<TabDefinition>,
        value: String,
        focused: Option<String>,
        drag: Option<String>,
        drop: Option<String>,
        orientation: Orientation,
        activation: TabActivationMode,
        instance: String,
    }

    fn definitions() -> Vec<TabDefinition> {
        vec![
            TabDefinition::new("one", "One"),
            TabDefinition::new("skip", "Skip").with_disabled(true),
            TabDefinition::new("two", "Two").with_closable(true),
            TabDefinition::new("three", "Three").with_closable(true),
        ]
    }

    run_headless(|cx| {
        fn build(
            state: &TabsState,
            mounted: &Arc<Mutex<Node>>,
            live: &Arc<Mutex<TabsState>>,
            changes: &Arc<Mutex<Vec<String>>>,
            closes: &Arc<Mutex<Vec<String>>>,
            orders: &Arc<Mutex<Vec<Vec<String>>>>,
            starts: &Arc<Mutex<Vec<String>>>,
            ends: &Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let live_state = Arc::clone(live);
            let mount = Arc::clone(mounted);
            let rebuild = {
                let live_state = Arc::clone(&live_state);
                let mount = Arc::clone(&mount);
                let changes = Arc::clone(changes);
                let closes = Arc::clone(closes);
                let orders = Arc::clone(orders);
                let starts = Arc::clone(starts);
                let ends = Arc::clone(ends);
                move |next: TabsState| {
                    *live_state.lock().expect("state lock") = next.clone();
                    *mount.lock().expect("mount lock") = build(
                        &next,
                        &mount,
                        &live_state,
                        &changes,
                        &closes,
                        &orders,
                        &starts,
                        &ends,
                    );
                }
            };
            let mut spec = TabsSpec::new(state.items.clone())
                .with_value(&state.value)
                .with_orientation(state.orientation)
                .with_activation_mode(state.activation)
                .with_reorderable(true)
                .with_drag_value(state.drag.clone())
                .with_drop_target_value(state.drop.clone())
                .with_aria_label("Files");
            spec.activation_mode = state.activation;
            let mut node = poodle_render::tabs_with_panel(
                &spec,
                &RenderContext::new(&theme()),
                TabsHandlers {
                    on_change: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        let changes = Arc::clone(changes);
                        Arc::new(move |value: &str| {
                            changes.lock().expect("changes").push(value.to_owned());
                            let mut next = live_state.lock().expect("state lock").clone();
                            next.value = value.to_owned();
                            next.focused = Some(value.to_owned());
                            rebuild(next);
                        })
                    }),
                    on_close: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        let closes = Arc::clone(closes);
                        Arc::new(move |value: &str| {
                            closes.lock().expect("closes").push(value.to_owned());
                            let mut next = live_state.lock().expect("state lock").clone();
                            next.items.retain(|item| item.value != value);
                            if next.value == value {
                                next.value = next
                                    .items
                                    .iter()
                                    .find(|item| !item.is_disabled)
                                    .map(|item| item.value.clone())
                                    .unwrap_or_default();
                                next.focused = Some(next.value.clone());
                            }
                            rebuild(next);
                        })
                    }),
                    on_focus: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        let instance = state.instance.clone();
                        Arc::new(move |value: &str| {
                            poodle_gpui_node_backend::request_focus(&format!(
                                "tabs:{instance}:tab:{value}"
                            ));
                            let mut next = live_state.lock().expect("state lock").clone();
                            next.focused = Some(value.to_owned());
                            rebuild(next);
                        })
                    }),
                    on_reorder: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        let orders = Arc::clone(orders);
                        Arc::new(move |order: Vec<String>| {
                            let mut next = live_state.lock().expect("state lock").clone();
                            let mut by_value = next
                                .items
                                .iter()
                                .cloned()
                                .map(|item| (item.value.clone(), item))
                                .collect::<std::collections::BTreeMap<_, _>>();
                            next.items = order
                                .iter()
                                .filter_map(|value| by_value.remove(value))
                                .collect();
                            orders.lock().expect("orders").push(order);
                            rebuild(next);
                        })
                    }),
                    on_drag_start: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        let starts = Arc::clone(starts);
                        Arc::new(move |value: &str| {
                            starts.lock().expect("starts").push(value.to_owned());
                            let mut next = live_state.lock().expect("state lock").clone();
                            next.drag = Some(value.to_owned());
                            rebuild(next);
                        })
                    }),
                    on_drag_end: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        let ends = Arc::clone(ends);
                        Arc::new(move |value: &str| {
                            ends.lock().expect("ends").push(value.to_owned());
                            let mut next = live_state.lock().expect("state lock").clone();
                            next.drag = None;
                            next.drop = None;
                            rebuild(next);
                        })
                    }),
                    on_drop_target_change: Some({
                        let rebuild = rebuild.clone();
                        let live_state = Arc::clone(&live_state);
                        Arc::new(move |value: Option<&str>| {
                            let mut next = live_state.lock().expect("state lock").clone();
                            next.drop = value.map(str::to_owned);
                            rebuild(next);
                        })
                    }),
                    focused_value: state.focused.clone(),
                    instance_id: Some(state.instance.clone()),
                    has_panel: true,
                    ..TabsHandlers::default()
                },
                {
                    let mut panel = Node::text(format!("{} panel", state.value));
                    panel.a11y.role = Some(NodeRole::TabPanel);
                    panel
                },
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let live = Arc::new(Mutex::new(TabsState {
            items: definitions(),
            value: "one".into(),
            focused: Some("one".into()),
            drag: None,
            drop: None,
            orientation: Orientation::Horizontal,
            activation: TabActivationMode::Automatic,
            instance: "mounted".into(),
        }));
        let changes = Arc::new(Mutex::new(Vec::new()));
        let closes = Arc::new(Mutex::new(Vec::new()));
        let orders = Arc::new(Mutex::new(Vec::new()));
        let starts = Arc::new(Mutex::new(Vec::new()));
        let ends = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            &live.lock().unwrap().clone(),
            &mounted,
            &live,
            &changes,
            &closes,
            &orders,
            &starts,
            &ends,
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 80.0);
        driver.wait_for_focus_handle("tabs:mounted:tab:one");

        let two = payload_frac("tabs:mounted:tab:two", 0.2, 0.5);
        driver.pointer_press(two);
        driver.pointer_release(two);
        assert_eq!(*changes.lock().unwrap(), vec!["two".to_string()]);
        assert_eq!(live.lock().unwrap().value, "two");
        {
            let root = mounted.lock().unwrap();
            let list = root
                .find(&|node| node.a11y.role == Some(NodeRole::TabList))
                .expect("tablist");
            assert_eq!(list.a11y.orientation.as_deref(), Some("horizontal"));
            let selected = tab_at(&root, "tabs:mounted:tab:two");
            assert_eq!(selected.a11y.role, Some(NodeRole::Tab));
            assert_eq!(selected.a11y.selected, Some(true));
            assert_eq!(selected.a11y.tab_index, Some(0));
            assert_eq!(selected.a11y.controls.as_deref(), Some("tabs-panel:two"));
            let skip = tab_at(&root, "tabs:mounted:tab:skip");
            assert!(skip.interaction.disabled);
            assert!(!skip.interaction.focusable);
            let panel = root
                .find(&|node| node.a11y.role == Some(NodeRole::TabPanel))
                .expect("panel");
            assert_eq!(panel.a11y.labelled_by.as_deref(), Some("tabs:two"));
            assert_eq!(panel.a11y.tab_index, Some(0));
        }

        driver.wait_for_focus_handle("tabs:mounted:tab:two");
        driver.keyboard_key("tabs:mounted:tab:two", "right");
        assert_eq!(live.lock().unwrap().value, "three");
        assert_eq!(
            tab_at(&mounted.lock().unwrap(), "tabs:mounted:tab:three")
                .a11y
                .tab_index,
            Some(0)
        );

        driver.keyboard_key("tabs:mounted:tab:three", "left");
        assert_eq!(
            live.lock().unwrap().value,
            "two",
            "left skips the disabled tab"
        );

        driver.keyboard_key("tabs:mounted:tab:two", "alt-right");
        assert_eq!(
            orders.lock().unwrap().last().map(Vec::as_slice),
            Some(
                [
                    "one".to_string(),
                    "skip".to_string(),
                    "three".to_string(),
                    "two".to_string()
                ]
                .as_slice()
            )
        );
        assert_eq!(
            live.lock().unwrap().focused.as_deref(),
            Some("two"),
            "keyboard reorder keeps focus on the moved tab"
        );

        driver.keyboard_key("tabs:mounted:tab:two", "delete");
        assert_eq!(*closes.lock().unwrap(), vec!["two".to_string()]);
        assert!(mounted
            .lock()
            .unwrap()
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:mounted:tab:two"))
            .is_none());
        driver.pointer_activate_id("tabs:mounted:close:three");
        assert_eq!(
            closes.lock().unwrap().as_slice(),
            ["two".to_string(), "three".to_string()]
        );
    });

    run_headless(|cx| {
        let mut spec = TabsSpec::new(definitions())
            .with_value("one")
            .with_activation_mode(TabActivationMode::Manual)
            .with_reorderable(true);
        spec.activation_mode = TabActivationMode::Manual;
        let live = Arc::new(Mutex::new("one".to_string()));
        let focused = Arc::new(Mutex::new(Some("one".to_string())));
        let sink = Arc::clone(&live);
        let focus_sink = Arc::clone(&focused);
        let mut node = poodle_render::tabs_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            TabsHandlers {
                on_change: Some(Arc::new(move |value: &str| {
                    *sink.lock().unwrap() = value.to_owned();
                })),
                on_focus: Some(Arc::new(move |value: &str| {
                    *focus_sink.lock().unwrap() = Some(value.to_owned());
                    poodle_gpui_node_backend::request_focus(&format!("tabs:manual:tab:{value}"));
                })),
                focused_value: Some("one".into()),
                instance_id: Some("manual".into()),
                ..TabsHandlers::default()
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(node)), 420.0, 80.0);
        driver.wait_for_focus_handle("tabs:manual:tab:one");
        driver.keyboard_key("tabs:manual:tab:one", "right");
        assert_eq!(live.lock().unwrap().as_str(), "one");
        assert_eq!(focused.lock().unwrap().as_deref(), Some("two"));
        driver.keyboard_key("tabs:manual:tab:two", "enter");
        assert_eq!(live.lock().unwrap().as_str(), "two");
    });

    run_headless(|cx| {
        let mut spec = TabsSpec::new(definitions())
            .with_value("one")
            .with_orientation(Orientation::Vertical);
        spec.orientation = Orientation::Vertical;
        let live = Arc::new(Mutex::new("one".to_string()));
        let sink = Arc::clone(&live);
        let mut node = poodle_render::tabs_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            TabsHandlers {
                on_change: Some(Arc::new(move |value: &str| {
                    *sink.lock().unwrap() = value.to_owned();
                })),
                instance_id: Some("vertical".into()),
                focused_value: Some("one".into()),
                ..TabsHandlers::default()
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(node)), 80.0, 220.0);
        driver.wait_for_focus_handle("tabs:vertical:tab:one");
        driver.keyboard_key("tabs:vertical:tab:one", "right");
        assert_eq!(live.lock().unwrap().as_str(), "one");
        driver.keyboard_key("tabs:vertical:tab:one", "down");
        assert_eq!(live.lock().unwrap().as_str(), "two");
    });

    run_headless(|cx| {
        fn build(
            state: TabsState,
            mounted: Arc<Mutex<Node>>,
            live: Arc<Mutex<TabsState>>,
            orders: Arc<Mutex<Vec<Vec<String>>>>,
            starts: Arc<Mutex<Vec<String>>>,
            ends: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let mut node = poodle_render::tabs_with_handlers(
                &TabsSpec::new(state.items.clone())
                    .with_value(&state.value)
                    .with_reorderable(true)
                    .with_drag_value(state.drag.clone())
                    .with_drop_target_value(state.drop.clone()),
                &RenderContext::new(&theme()),
                TabsHandlers {
                    on_reorder: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let orders = Arc::clone(&orders);
                        let starts = Arc::clone(&starts);
                        let ends = Arc::clone(&ends);
                        Arc::new(move |order: Vec<String>| {
                            let mut next = live.lock().unwrap().clone();
                            let mut by_value = next
                                .items
                                .iter()
                                .cloned()
                                .map(|item| (item.value.clone(), item))
                                .collect::<std::collections::BTreeMap<_, _>>();
                            next.items = order
                                .iter()
                                .filter_map(|value| by_value.remove(value))
                                .collect();
                            orders.lock().unwrap().push(order);
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&orders),
                                Arc::clone(&starts),
                                Arc::clone(&ends),
                            );
                        })
                    }),
                    on_drag_start: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let orders = Arc::clone(&orders);
                        let starts = Arc::clone(&starts);
                        let ends = Arc::clone(&ends);
                        Arc::new(move |value: &str| {
                            starts.lock().unwrap().push(value.to_owned());
                            let mut next = live.lock().unwrap().clone();
                            next.drag = Some(value.to_owned());
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&orders),
                                Arc::clone(&starts),
                                Arc::clone(&ends),
                            );
                        })
                    }),
                    on_drag_end: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let orders = Arc::clone(&orders);
                        let starts = Arc::clone(&starts);
                        let ends = Arc::clone(&ends);
                        Arc::new(move |value: &str| {
                            ends.lock().unwrap().push(value.to_owned());
                            let mut next = live.lock().unwrap().clone();
                            next.drag = None;
                            next.drop = None;
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&orders),
                                Arc::clone(&starts),
                                Arc::clone(&ends),
                            );
                        })
                    }),
                    on_drop_target_change: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let orders = Arc::clone(&orders);
                        let starts = Arc::clone(&starts);
                        let ends = Arc::clone(&ends);
                        Arc::new(move |value: Option<&str>| {
                            let mut next = live.lock().unwrap().clone();
                            next.drop = value.map(str::to_owned);
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&orders),
                                Arc::clone(&starts),
                                Arc::clone(&ends),
                            );
                        })
                    }),
                    on_focus: Some({
                        let live = Arc::clone(&live);
                        Arc::new(move |value: &str| {
                            live.lock().unwrap().focused = Some(value.to_owned());
                            poodle_gpui_node_backend::request_focus(&format!(
                                "tabs:drag:tab:{value}"
                            ));
                        })
                    }),
                    focused_value: state.focused.clone(),
                    instance_id: Some("drag".into()),
                    ..TabsHandlers::default()
                },
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let live = Arc::new(Mutex::new(TabsState {
            items: definitions(),
            value: "one".into(),
            focused: Some("one".into()),
            drag: None,
            drop: None,
            orientation: Orientation::Horizontal,
            activation: TabActivationMode::Automatic,
            instance: "drag".into(),
        }));
        let orders = Arc::new(Mutex::new(Vec::new()));
        let starts = Arc::new(Mutex::new(Vec::new()));
        let ends = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            live.lock().unwrap().clone(),
            Arc::clone(&mounted),
            Arc::clone(&live),
            Arc::clone(&orders),
            Arc::clone(&starts),
            Arc::clone(&ends),
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 80.0);
        driver.draw_frame();

        let source = payload_frac("tabs:drag:tab:one", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(*starts.lock().unwrap(), vec!["one".to_string()]);
        assert_eq!(live.lock().unwrap().drag.as_deref(), Some("one"));

        driver.pointer_drag(payload_frac("tabs:drag:tab:three", 0.5, 0.5));
        assert_eq!(live.lock().unwrap().drop.as_deref(), Some("three"));
        // Back over the dragged tab itself: a self-drop is *rejected*, so the
        // drop-target indicator clears rather than pointing a tab at itself.
        driver.pointer_drag(payload_frac("tabs:drag:tab:one", 0.5, 0.5));
        assert_eq!(live.lock().unwrap().drop.as_deref(), None);
        driver.pointer_drag(payload_frac("tabs:drag:tab:three", 0.5, 0.5));
        driver.pointer_release(payload_frac("tabs:drag:tab:three", 0.5, 0.5));
        assert_eq!(
            orders.lock().unwrap().last().map(Vec::as_slice),
            Some(
                [
                    "skip".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "one".to_string()
                ]
                .as_slice()
            )
        );
        assert_eq!(*ends.lock().unwrap(), vec!["one".to_string()]);
        assert!(live.lock().unwrap().drag.is_none());
        assert!(live.lock().unwrap().drop.is_none());
        assert_eq!(live.lock().unwrap().focused.as_deref(), Some("one"));
    });

    run_headless(|cx| {
        let live = Arc::new(Mutex::new(TabsState {
            items: definitions(),
            value: "one".into(),
            focused: Some("one".into()),
            drag: None,
            drop: None,
            orientation: Orientation::Horizontal,
            activation: TabActivationMode::Automatic,
            instance: "cancel".into(),
        }));
        let ends = Arc::new(Mutex::new(Vec::new()));
        let orders = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        fn build_cancel(
            state: TabsState,
            mounted: Arc<Mutex<Node>>,
            live: Arc<Mutex<TabsState>>,
            ends: Arc<Mutex<Vec<String>>>,
            orders: Arc<Mutex<Vec<Vec<String>>>>,
        ) -> Node {
            let mut node = poodle_render::tabs_with_handlers(
                &TabsSpec::new(state.items.clone())
                    .with_value(&state.value)
                    .with_reorderable(true)
                    .with_drag_value(state.drag.clone())
                    .with_drop_target_value(state.drop.clone()),
                &RenderContext::new(&theme()),
                TabsHandlers {
                    on_reorder: Some({
                        let orders = Arc::clone(&orders);
                        Arc::new(move |order: Vec<String>| {
                            orders.lock().unwrap().push(order);
                        })
                    }),
                    on_drag_start: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let ends = Arc::clone(&ends);
                        let orders = Arc::clone(&orders);
                        Arc::new(move |value: &str| {
                            let mut next = live.lock().unwrap().clone();
                            next.drag = Some(value.to_owned());
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build_cancel(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&ends),
                                Arc::clone(&orders),
                            );
                        })
                    }),
                    on_drag_end: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let ends = Arc::clone(&ends);
                        let orders = Arc::clone(&orders);
                        Arc::new(move |value: &str| {
                            ends.lock().unwrap().push(value.to_owned());
                            let mut next = live.lock().unwrap().clone();
                            next.drag = None;
                            next.drop = None;
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build_cancel(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&ends),
                                Arc::clone(&orders),
                            );
                        })
                    }),
                    on_drop_target_change: Some({
                        let mounted = Arc::clone(&mounted);
                        let live = Arc::clone(&live);
                        let ends = Arc::clone(&ends);
                        let orders = Arc::clone(&orders);
                        Arc::new(move |value: Option<&str>| {
                            let mut next = live.lock().unwrap().clone();
                            next.drop = value.map(str::to_owned);
                            *live.lock().unwrap() = next.clone();
                            *mounted.lock().unwrap() = build_cancel(
                                next,
                                Arc::clone(&mounted),
                                Arc::clone(&live),
                                Arc::clone(&ends),
                                Arc::clone(&orders),
                            );
                        })
                    }),
                    instance_id: Some("cancel".into()),
                    ..TabsHandlers::default()
                },
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }
        *mounted.lock().unwrap() = build_cancel(
            live.lock().unwrap().clone(),
            Arc::clone(&mounted),
            Arc::clone(&live),
            Arc::clone(&ends),
            Arc::clone(&orders),
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 80.0);
        driver.draw_frame();
        let source = payload_frac("tabs:cancel:tab:one", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("tabs:cancel:tab:three", 0.5, 0.5));
        driver.dispatch_key("escape");
        assert!(orders.lock().unwrap().is_empty());
        assert_eq!(*ends.lock().unwrap(), vec!["one".to_string()]);
        assert!(live.lock().unwrap().drag.is_none());
        assert!(live.lock().unwrap().drop.is_none());
    });

    run_headless(|cx| {
        let spec = TabsSpec::new(vec![
            TabDefinition::new("shared", "Shared"),
            TabDefinition::new("other", "Other"),
        ])
        .with_value("shared");
        let first = poodle_render::tabs_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            TabsHandlers {
                instance_id: Some("alpha".into()),
                focused_value: Some("shared".into()),
                ..TabsHandlers::default()
            },
        );
        let second = poodle_render::tabs_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            TabsHandlers {
                instance_id: Some("beta".into()),
                focused_value: Some("shared".into()),
                ..TabsHandlers::default()
            },
        );
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root = root.child(first).child(second);
        root.id = Some(FIXTURE_ID.to_owned());
        assert!(root
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:alpha:tab:shared"))
            .is_some());
        assert!(root
            .find(&|node| node.runtime_id.as_deref() == Some("tabs:beta:tab:shared"))
            .is_some());
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(root)), 280.0, 120.0);
        driver.wait_for_focus_handle("tabs:alpha:tab:shared");
        driver.wait_for_focus_handle("tabs:beta:tab:shared");
        assert!(
            poodle_gpui_node_backend::focus_handle_for("tabs:alpha:tab:shared").is_some()
                && poodle_gpui_node_backend::focus_handle_for("tabs:beta:tab:shared").is_some()
        );
    });
}

fn stamp_slider_id(node: &mut Node, id: &str) {
    if node.a11y.role == Some(NodeRole::Slider) {
        node.id = Some(id.to_owned());
        return;
    }
    for child in &mut node.children {
        stamp_slider_id(child, id);
    }
}

fn slider_control(node: &Node) -> &Node {
    node.find(&|n| n.a11y.role == Some(NodeRole::Slider))
        .expect("one slider node")
}

/// g16.005. Slider pointer, keyboard, disabled, and vertical paths through
/// real backend input and a controlled host rebuild after commit.
#[test]
fn slider_axis_keyboard_and_disabled_rebuild_the_host_spec() {
    run_headless(|cx| {
        fn build(
            value: f64,
            mounted: Arc<Mutex<Node>>,
            trace: Arc<Mutex<Vec<String>>>,
            live: Arc<Mutex<f64>>,
        ) -> Node {
            let events = Arc::clone(&trace);
            let state = Arc::clone(&live);
            let commit_events = Arc::clone(&trace);
            let commit_state = Arc::clone(&live);
            let commit_mount = Arc::clone(&mounted);
            let mut spec = SliderSpec::new(value).with_bounds(0.0, 100.0);
            spec.step = 1.0;
            spec.aria_label = Some("Volume".into());
            spec.value_text = Some(format!("{value:.0}%"));
            let mut node = poodle_render::slider(
                &spec,
                &RenderContext::new(&theme()),
                &SliderHandlers {
                    on_change: Some(Arc::new(move |next| {
                        *state.lock().expect("value lock") = next;
                        events.lock().expect("trace lock").push("valueChange".into());
                    })),
                    on_value_commit: Some(Arc::new(move |next| {
                        *commit_state.lock().expect("value lock") = next;
                        commit_events
                            .lock()
                            .expect("trace lock")
                            .push("valueCommit".into());
                        *commit_mount.lock().expect("mount lock") = build(
                            next,
                            Arc::clone(&commit_mount),
                            Arc::clone(&commit_events),
                            Arc::clone(&commit_state),
                        );
                    })),
                },
            );
            stamp_slider_id(&mut node, FIXTURE_ID);
            node
        }

        let trace = Arc::new(Mutex::new(Vec::new()));
        let live = Arc::new(Mutex::new(20.0f64));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            20.0,
            Arc::clone(&mounted),
            Arc::clone(&trace),
            Arc::clone(&live),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(FIXTURE_ID);

        driver.pointer_scrub_at(0.9, "press");
        driver.pointer_scrub_at(0.93, "drag");
        driver.pointer_scrub_at(0.95, "drag");
        driver.pointer_scrub_at(0.95, "release");
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["valueChange", "valueChange", "valueCommit"]
        );
        assert_eq!(*live.lock().expect("value lock"), 95.0);

        let rebuilt = mounted.lock().unwrap();
        let control = slider_control(&rebuilt);
        assert_eq!(control.a11y.role, Some(NodeRole::Slider));
        assert_eq!(control.a11y.label.as_deref(), Some("Volume"));
        assert_eq!(control.a11y.value, Some(95.0));
        assert_eq!(control.a11y.value_min, Some(0.0));
        assert_eq!(control.a11y.value_max, Some(100.0));
        assert_eq!(control.a11y.value_text.as_deref(), Some("95%"));
        assert_eq!(control.a11y.orientation.as_deref(), Some("horizontal"));
        assert!(control.interaction.focusable);
        let ring = control.style.focus_ring.expect("standard thumb ring");
        assert!((ring.width - poodle_render::presentation::rem_to_px(0.1875)).abs() < 1e-6);
        assert!((ring.offset - 0.0).abs() < 1e-6);
        assert!((ring.color.3 - 0.32).abs() < 1e-6);
        assert!(
            rebuilt.style.focus_ring.is_none(),
            "standard focus belongs on the thumb"
        );
        drop(rebuilt);

        driver.wait_for_focus_handle(FIXTURE_ID);
        tab_until_focused(&mut driver, FIXTURE_ID);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(FIXTURE_ID),
            Some(true),
            "Tab traversal must reach the enabled slider before a key"
        );
        driver.dispatch_key_raw("right");
        assert_eq!(*live.lock().expect("value lock"), 96.0);
        driver.dispatch_key_raw("home");
        assert_eq!(*live.lock().expect("value lock"), 0.0);
        driver.dispatch_key_raw("end");
        assert_eq!(*live.lock().expect("value lock"), 100.0);
        assert_eq!(
            slider_control(&mounted.lock().unwrap()).a11y.value,
            Some(100.0),
            "the rebuilt node carries the committed value"
        );
        let keys = trace.lock().expect("trace lock").clone();
        assert_eq!(
            &keys[keys.len() - 6..],
            [
                "valueChange",
                "valueCommit",
                "valueChange",
                "valueCommit",
                "valueChange",
                "valueCommit"
            ]
        );
    });

    run_headless(|cx| {
        let mut spec = SliderSpec::new(0.0)
            .with_bounds(0.0, 100.0)
            .with_orientation(Orientation::Vertical);
        spec.step = 1.0;
        spec.aria_label = Some("Level".into());
        spec.value_text = Some("min".into());
        let live = Arc::new(Mutex::new(0.0f64));
        let sink = Arc::clone(&live);
        let mut node = poodle_render::slider(
            &spec,
            &RenderContext::new(&theme()),
            &SliderHandlers {
                on_change: Some(Arc::new(move |next| {
                    *sink.lock().expect("value lock") = next;
                })),
                on_value_commit: None,
            },
        );
        stamp_slider_id(&mut node, FIXTURE_ID);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(node)), 48.0, 200.0);
        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.pointer_scrub_vertical_at(0.25, "press");
        driver.pointer_scrub_vertical_at(0.28, "drag");
        driver.pointer_scrub_vertical_at(0.75, "drag");
        driver.pointer_scrub_vertical_at(0.75, "release");
        assert_eq!(*live.lock().expect("value lock"), 75.0);
    });

    run_headless(|cx| {
        let mut spec = SliderSpec::new(40.0).with_bounds(0.0, 100.0);
        spec.is_disabled = true;
        spec.aria_label = Some("Muted".into());
        let live = Arc::new(Mutex::new(40.0f64));
        let sink = Arc::clone(&live);
        let mut node = poodle_render::slider(
            &spec,
            &RenderContext::new(&theme()),
            &SliderHandlers {
                on_change: Some(Arc::new(move |next| {
                    *sink.lock().expect("value lock") = next;
                })),
                on_value_commit: Some(Arc::new(|_| {})),
            },
        );
        stamp_slider_id(&mut node, "disabled-slider");
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.pointer_scrub_at(0.9, "press");
        driver.pointer_scrub_at(0.95, "drag");
        driver.pointer_scrub_at(0.95, "release");
        driver.dispatch_key("right");
        assert_eq!(*live.lock().expect("value lock"), 40.0);
        let disabled = mounted.lock().unwrap();
        let control = slider_control(&disabled);
        assert!(control.interaction.disabled);
        assert!(!control.interaction.focusable);
        assert_eq!(control.a11y.tab_index, None);
        assert!(control.interaction.on_key.is_none());
        assert!(control.style.focus_ring.is_none());
        assert!(
            disabled
                .find(&|n| n.interaction.on_scrub.is_some())
                .is_none()
        );
    });
}

/// g16.046. Block Slider hit is a real 44×44 target; RTL remaps scrub; a
/// second terminal is inert. Vertical block panics in the renderer, not here.
#[test]
fn block_slider_hit_rtl_and_terminal_on_the_mounted_host() {
    run_headless(|cx| {
        let live = Arc::new(Mutex::new(0.0f64));
        let sink = Arc::clone(&live);
        let commits = Arc::new(Mutex::new(0u32));
        let commit_count = Arc::clone(&commits);
        let spec = SliderSpec::new(0.0)
            .with_bounds(0.0, 100.0)
            .with_appearance(SliderAppearance::Block)
            .with_direction(SliderDirection::Rtl)
            .with_size(ControlSize::Xs)
            .with_visible_label("Volume");
        let mut spec = spec;
        spec.aria_label = Some("Volume".into());
        spec.step = 1.0;
        let theme = theme();
        let layout_root = RenderContext::new(&theme);
        let ctx = layout_root.with_block_layout_width(160.0);
        let mut node = poodle_render::slider(
            &spec,
            &ctx,
            &SliderHandlers {
                on_change: Some(Arc::new(move |next| {
                    *sink.lock().expect("value lock") = next;
                })),
                on_value_commit: Some(Arc::new(move |_| {
                    *commit_count.lock().expect("commit lock") += 1;
                })),
            },
        );
        stamp_slider_id(&mut node, FIXTURE_ID);
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 160.0, 80.0);
        driver.wait_for_focus_handle(FIXTURE_ID);
        let bounds = poodle_gpui_node_backend::bounds_for(FIXTURE_ID).expect("block hit bounds");
        assert_eq!(f32::from(bounds.size.width), 44.0);
        assert_eq!(f32::from(bounds.size.height), 44.0);
        driver.pointer_scrub_at(0.2, "press");
        driver.pointer_scrub_at(0.2, "drag");
        driver.pointer_scrub_at(0.2, "release");
        assert_eq!(*live.lock().expect("value lock"), 80.0);
        driver.pointer_scrub_at(0.2, "release");
        assert_eq!(*commits.lock().expect("commit lock"), 1);
        tab_until_focused(&mut driver, FIXTURE_ID);
        driver.dispatch_key_raw("right");
        assert_eq!(*live.lock().expect("value lock"), 81.0);
    });

    run_headless(|cx| {
        let live = Arc::new(Mutex::new((50.0f64, 50.0f64)));
        let sink = Arc::clone(&live);
        let spec = RangeSliderSpec::new(50.0, 50.0)
            .with_bounds(0.0, 100.0)
            .with_appearance(SliderAppearance::Block)
            .with_size(ControlSize::Xs)
            .with_aria_label("Range");
        let theme = theme();
        let layout_root = RenderContext::new(&theme);
        let ctx = layout_root.with_block_layout_width(160.0);
        let node = poodle_render::range_slider(
            &spec,
            &ctx,
            poodle_render::RangeSliderHandlers {
                on_change: Some(Arc::new(move |lo, hi| {
                    *sink.lock().expect("value lock") = (lo, hi);
                })),
                on_value_commit: None,
            },
        );
        let mounted = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 160.0, 80.0);
        driver.wait_for_focus_handle("range-slider-lower");
        let lower = poodle_gpui_node_backend::bounds_for("range-slider-lower").expect("lower hit");
        let upper = poodle_gpui_node_backend::bounds_for("range-slider-upper").expect("upper hit");
        assert_eq!(f32::from(lower.size.width), 44.0);
        assert_eq!(f32::from(lower.size.height), 44.0);
        assert_eq!(f32::from(upper.size.width), 44.0);
        assert_eq!(f32::from(upper.size.height), 44.0);
        driver.pointer_scrub_at(0.5, "press");
        driver.pointer_scrub_at(0.48, "drag");
        driver.pointer_scrub_at(0.2, "drag");
        assert_eq!(*live.lock().expect("value lock"), (20.0, 50.0));
        tab_until_focused(&mut driver, "range-slider-upper");
        driver.dispatch_key_raw("right");
        assert_eq!(*live.lock().expect("value lock"), (20.0, 51.0));
    });
}

fn mount_block_slider_host<'a>(
    cx: &'a mut TestAppContext,
    spec: SliderSpec,
    width: f32,
) -> HeadlessDriver<'a> {
    let theme = theme();
    HeadlessDriver::new_element_in_box(
        cx,
        Rc::new(move || {
            block_slider_host::slider_element(
                spec.clone(),
                theme.clone(),
                SliderHandlers::default(),
                None,
            )
        }),
        width,
        120.0,
    )
}

fn block_stack_sibling(id: &str) -> Node {
    let mut next = Node::container();
    next.id = Some(id.to_owned());
    next.style.descriptor.layout.height = LayoutSizing::Fixed(8.0);
    next.style.fill_width = true;
    next
}

fn mount_production_block_stack<'a>(
    cx: &'a mut TestAppContext,
    host: Rc<dyn Fn() -> gpui::AnyElement>,
    next_id: &'static str,
    width: f32,
) -> HeadlessDriver<'a> {
    HeadlessDriver::new_element_in_box(
        cx,
        Rc::new(move || {
            use gpui::{IntoElement as _, ParentElement as _, Styled as _};
            gpui::div()
                .flex()
                .flex_col()
                .w(px(width))
                .child(host())
                .child(poodle_gpui_node_backend::to_gpui(&block_stack_sibling(next_id)))
                .into_any_element()
        }),
        width,
        200.0,
    )
}

fn bounds_contain(outer: gpui::Bounds<Pixels>, inner: gpui::Bounds<Pixels>) -> bool {
    inner.origin.x >= outer.origin.x - px(0.5)
        && inner.origin.y >= outer.origin.y - px(0.5)
        && inner.bottom() <= outer.bottom() + px(0.5)
        && inner.right() <= outer.right() + px(0.5)
}

/// g16.046 repair. Fit uses the mounted parent width and GPUI shaped advance,
/// not a fixed 160px span or `chars * font * 0.5`.
#[test]
fn block_slider_fit_uses_parent_width_and_shaped_advance() {
    let label = SliderSpec::new(50.0)
        .with_bounds(0.0, 100.0)
        .with_appearance(SliderAppearance::Block)
        .with_visible_label("ABCDEFGH")
        .with_visible_value_text("50");
    run_headless(|cx| {
        let _driver = mount_block_slider_host(cx, label.clone(), 80.0);
        assert!(
            poodle_gpui_node_backend::bounds_for("block-slider-fallback").is_some(),
            "narrow parent-owned span must miss and paint fallback"
        );
    });
    run_headless(|cx| {
        let _driver = mount_block_slider_host(cx, label.clone(), 400.0);
        assert!(
            poodle_gpui_node_backend::bounds_for("block-slider-fallback").is_none(),
            "wide parent-owned span must fit inline"
        );
    });

    let font_px = poodle_render::presentation::rem_to_px(poodle_render::slider_block::font_size_rem(
        ControlSize::Md,
    ));
    let (shaped, heuristic) = {
        let mut measured = (0.0f32, 0.0f32);
        run_headless(|cx| {
            let mut driver = mount_block_slider_host(cx, label, 200.0);
            measured = driver.with_window(|window, _| {
                let text = "iii";
                (
                    poodle_gpui_node_backend::shaped_block_advance(window, text, font_px),
                    text.chars().count() as f32 * font_px * 0.5,
                )
            });
        });
        measured
    };
    let shaped_need = shaped.ceil();
    let heuristic_need = heuristic.ceil();
    assert_ne!(
        shaped_need, heuristic_need,
        "this platform's shaped advance must disagree with chars*font*0.5 (shaped={shaped}, heuristic={heuristic}, font={font_px})"
    );
    let available = shaped_need.min(heuristic_need);
    let width = 2.0 * (available + 16.0);
    let sample = SliderSpec::new(50.0)
        .with_bounds(0.0, 100.0)
        .with_appearance(SliderAppearance::Block)
        .with_visible_label("iii")
        .with_visible_value_text("");
    let mut missed = false;
    run_headless(|cx| {
        let _driver = mount_block_slider_host(cx, sample, width);
        missed = poodle_gpui_node_backend::bounds_for("block-slider-fallback").is_some();
    });
    if shaped_need > heuristic_need {
        assert!(
            missed,
            "available={available} at {width}px fits heuristic {heuristic_need} and must miss shaped {shaped_need} (shaped={shaped})"
        );
    } else {
        assert!(
            !missed,
            "available={available} at {width}px misses heuristic {heuristic_need} and must fit shaped {shaped_need} (shaped={shaped})"
        );
    }
}

/// g16.046 repair. Production GPUI host height follows the fit decision:
/// inline reserves the 44px surface; fallback reserves the surface plus its
/// line. A following sibling must sit below the fallback, not under it.
#[test]
fn block_slider_production_host_height_contains_fallback_and_not_wide_inline() {
    let slider = SliderSpec::new(50.0)
        .with_bounds(0.0, 100.0)
        .with_appearance(SliderAppearance::Block)
        .with_visible_label("ABCDEFGH")
        .with_visible_value_text("50");
    let range = RangeSliderSpec::new(20.0, 80.0)
        .with_bounds(0.0, 100.0)
        .with_appearance(SliderAppearance::Block)
        .with_visible_label("ABCDEFGH");
    let slider_surface = block_slider_host::block_slider_surface_height(&slider);
    let range_surface = block_slider_host::block_range_slider_surface_height(&range);

    let mut slider_narrow_h = 0.0f32;
    let mut slider_wide_h = 0.0f32;
    run_headless(|cx| {
        let theme = theme();
        let spec = slider.clone();
        let _driver = mount_production_block_stack(
            cx,
            Rc::new(move || {
                block_slider_host::slider_element(
                    spec.clone(),
                    theme.clone(),
                    SliderHandlers::default(),
                    None,
                )
            }),
            "block-slider-next",
            80.0,
        );
        let host = poodle_gpui_node_backend::bounds_for("block-slider-host").expect("slider host");
        let fallback =
            poodle_gpui_node_backend::bounds_for("block-slider-fallback").expect("slider fallback");
        let next = poodle_gpui_node_backend::bounds_for("block-slider-next").expect("slider sibling");
        assert!(
            bounds_contain(host, fallback),
            "fallback {fallback:?} must sit inside production host {host:?}"
        );
        assert!(
            next.origin.y >= fallback.bottom() - px(0.5),
            "sibling {next:?} must sit below fallback {fallback:?}"
        );
        slider_narrow_h = f32::from(host.size.height);
        assert!(
            slider_narrow_h > slider_surface + 4.0,
            "narrow host {slider_narrow_h} must reserve more than surface {slider_surface}"
        );
    });
    run_headless(|cx| {
        let theme = theme();
        let spec = slider.clone();
        let _driver = mount_production_block_stack(
            cx,
            Rc::new(move || {
                block_slider_host::slider_element(
                    spec.clone(),
                    theme.clone(),
                    SliderHandlers::default(),
                    None,
                )
            }),
            "block-slider-next",
            400.0,
        );
        let host = poodle_gpui_node_backend::bounds_for("block-slider-host").expect("slider host");
        assert!(
            poodle_gpui_node_backend::bounds_for("block-slider-fallback").is_none(),
            "wide production host must inline"
        );
        let next = poodle_gpui_node_backend::bounds_for("block-slider-next").expect("slider sibling");
        slider_wide_h = f32::from(host.size.height);
        assert!(
            (slider_wide_h - slider_surface).abs() <= 1.0,
            "wide host {slider_wide_h} must reserve only the surface {slider_surface}, not fallback height"
        );
        assert!(
            next.origin.y >= host.bottom() - px(0.5),
            "wide sibling {next:?} must sit below the surface host {host:?}"
        );
        assert!(
            slider_wide_h + 4.0 < slider_narrow_h,
            "wide {slider_wide_h} must not retain narrow fallback height {slider_narrow_h}"
        );
    });

    let mut range_narrow_h = 0.0f32;
    let mut range_wide_h = 0.0f32;
    run_headless(|cx| {
        let theme = theme();
        let spec = range.clone();
        let _driver = mount_production_block_stack(
            cx,
            Rc::new(move || {
                block_slider_host::range_slider_element(
                    spec.clone(),
                    theme.clone(),
                    poodle_render::RangeSliderHandlers::default(),
                    None,
                )
            }),
            "block-range-slider-next",
            80.0,
        );
        let host =
            poodle_gpui_node_backend::bounds_for("block-range-slider-host").expect("range host");
        let fallback = poodle_gpui_node_backend::bounds_for("block-range-slider-fallback")
            .expect("range fallback");
        let next = poodle_gpui_node_backend::bounds_for("block-range-slider-next")
            .expect("range sibling");
        assert!(
            bounds_contain(host, fallback),
            "range fallback {fallback:?} must sit inside production host {host:?}"
        );
        assert!(
            next.origin.y >= fallback.bottom() - px(0.5),
            "range sibling {next:?} must sit below fallback {fallback:?}"
        );
        range_narrow_h = f32::from(host.size.height);
        assert!(
            range_narrow_h > range_surface + 4.0,
            "narrow range host {range_narrow_h} must reserve more than surface {range_surface}"
        );
    });
    run_headless(|cx| {
        let theme = theme();
        let spec = range;
        let _driver = mount_production_block_stack(
            cx,
            Rc::new(move || {
                block_slider_host::range_slider_element(
                    spec.clone(),
                    theme.clone(),
                    poodle_render::RangeSliderHandlers::default(),
                    None,
                )
            }),
            "block-range-slider-next",
            400.0,
        );
        let host =
            poodle_gpui_node_backend::bounds_for("block-range-slider-host").expect("range host");
        assert!(
            poodle_gpui_node_backend::bounds_for("block-range-slider-fallback").is_none(),
            "wide range host must inline"
        );
        let next = poodle_gpui_node_backend::bounds_for("block-range-slider-next")
            .expect("range sibling");
        range_wide_h = f32::from(host.size.height);
        assert!(
            (range_wide_h - range_surface).abs() <= 1.0,
            "wide range host {range_wide_h} must reserve only the surface {range_surface}"
        );
        assert!(
            next.origin.y >= host.bottom() - px(0.5),
            "wide range sibling {next:?} must sit below the surface host {host:?}"
        );
        assert!(
            range_wide_h + 4.0 < range_narrow_h,
            "wide range {range_wide_h} must not retain narrow fallback height {range_narrow_h}"
        );
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
                &RenderContext::new(&theme()),
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

/// g15.037: AgentTranscript's native viewport uses GPUI's real scroll handle.
/// The reader can detach, append without being pulled away, jump to the real
/// bottom, and resume following. This runs entirely on GPUI's in-memory test
/// platform; a source token or specimen counter cannot satisfy it.
#[test]
fn agent_transcript_detaches_jumps_and_resumes_following_on_a_real_viewport() {
    use poodle_headless::agent_transcript::{TranscriptItem, TranscriptMessage};

    fn message(index: usize) -> TranscriptItem {
        TranscriptItem::Message(TranscriptMessage {
            id: format!("message-{index}"),
            markdown: format!(
                "Transcript block {index} has enough mixed-height copy to overflow the viewport."
            ),
            ..Default::default()
        })
    }

    run_headless(|cx| {
        let items = Rc::new(RefCell::new((0..24).map(message).collect::<Vec<_>>()));
        let scroll = poodle_gpui_node_backend::TrackedScrollState::new();
        let build_items = Rc::clone(&items);
        let build_scroll = scroll.clone();
        let build_theme = theme();
        let build: Rc<dyn Fn() -> gpui::AnyElement> = Rc::new(move || {
            let spec = AgentTranscriptSpec::new(build_items.borrow().clone());
            let ctx = RenderContext::new(&build_theme);
            let content = poodle_render::agent_transcript(
                &spec,
                &ctx,
                poodle_render::AgentTranscriptHandlers::default(),
            );
            let mut jump = poodle_render::agent_transcript::agent_transcript_jump(
                &spec,
                &ctx,
                Some(build_scroll.jump_handler()),
            );
            jump.id = Some("transcript-headless-jump-control".to_owned());
            poodle_gpui_node_backend::tracked_vertical_scroll(
                &content,
                &jump,
                &build_scroll,
                poodle_gpui_node_backend::TrackedScrollOptions {
                    viewport_id: "transcript-headless-viewport",
                    jump_id: "transcript-headless-jump",
                    pin_threshold: spec.pin_threshold,
                    auto_follow: spec.is_auto_scroll,
                    is_empty: spec.is_empty(),
                },
            )
        });

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();
        assert!(scroll.max_offset_y() > 0.0, "fixture must overflow");
        assert!(scroll.is_pinned(), "initial render follows the latest block");
        assert!(scroll.remaining_to_bottom() <= 0.5);

        driver.scroll_vertical(240.0);
        assert!(!scroll.is_pinned(), "scrolling up detaches the reader");
        let detached_offset = scroll.offset_y();
        assert!(scroll.remaining_to_bottom() > 32.0);

        items.borrow_mut().push(message(24));
        driver.draw_frame();
        assert_eq!(
            scroll.offset_y(),
            detached_offset,
            "an append must not move a detached reader",
        );
        assert!(
            poodle_gpui_node_backend::bounds_for("transcript-headless-jump-control").is_some(),
            "detached state mounts the real jump control",
        );

        driver.pointer_activate_id("transcript-headless-jump-control");
        driver.draw_frame();
        assert!(scroll.is_pinned(), "jump re-arms following");
        assert!(scroll.remaining_to_bottom() <= 0.5, "jump reaches the bottom");
        assert!(
            poodle_gpui_node_backend::bounds_for("transcript-headless-jump-control").is_none(),
            "the jump control leaves the mounted tree once pinned",
        );

        let followed_offset = scroll.offset_y();
        items.borrow_mut().push(message(25));
        driver.draw_frame();
        assert!(scroll.offset_y() < followed_offset, "a pinned append follows");
        assert!(scroll.remaining_to_bottom() <= 0.5);
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
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let inner = poodle_render::popover(
            &PopoverSpec::new().with_open(true),
            &ctx,
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
            &ctx,
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

/// g16.019. A deferred overlay row must receive a real pointer press/release
/// after the host rebuilds the tree open. Direct handler invocation is not
/// this proof. The fixture is generic — no Select identifier — so the repair
/// stays on the layer/hit-test seam.
#[test]
fn a_deferred_overlay_row_receives_pointer_after_host_rebuild() {
    use poodle_adapter::ThemeProvider;

    #[derive(Clone)]
    struct Host {
        open: bool,
        activations: Vec<String>,
        dismissals: Vec<DismissReason>,
    }

    fn build(host: Arc<Mutex<Host>>, mounted: Arc<Mutex<Node>>) -> Node {
        let state = host.lock().expect("host lock").clone();
        let trigger_host = Arc::clone(&host);
        let trigger_mount = Arc::clone(&mounted);
        let option_host = Arc::clone(&host);
        let option_mount = Arc::clone(&mounted);
        let dismiss_host = Arc::clone(&host);
        let dismiss_mount = Arc::clone(&mounted);

        let mut trigger = Node::container();
        trigger.runtime_id = Some("overlay-trigger".to_owned());
        trigger.interaction.focusable = true;
        trigger.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);
        trigger.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
        trigger.style.focus_ring = Some(poodle_node::FocusRing {
            color: theme().resolve_color("color.accent.focusRing"),
            width: theme().resolve_border_width("border.width.focus"),
            offset: 2.0,
        });
        trigger = trigger.child(Node::text("Open"));
        trigger.interaction.on_activate = Some(Arc::new(move || {
            let mut host = trigger_host.lock().expect("host lock");
            host.open = !host.open;
            drop(host);
            *trigger_mount.lock().expect("mount lock") =
                build(Arc::clone(&trigger_host), Arc::clone(&trigger_mount));
        }));

        let under_host = Arc::clone(&host);
        let mut under = Node::container();
        under.runtime_id = Some("overlay-under".to_owned());
        under.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);
        under.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
        under = under.child(Node::text("Covered"));
        under.interaction.on_activate = Some(Arc::new(move || {
            under_host
                .lock()
                .expect("host lock")
                .activations
                .push("under".to_owned());
        }));

        let mut root = Node::container().child(trigger).child(under);
        root.position = NodePosition::Relative;
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        if state.open {
            let mut option = Node::container();
            option.runtime_id = Some("overlay-option".to_owned());
            option.style.descriptor.layout.height = LayoutSizing::Fixed(32.0);
            option.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
            option = option.child(Node::text("Choose me"));
            option.interaction.on_activate = Some(Arc::new(move || {
                let mut host = option_host.lock().expect("host lock");
                host.activations.push("option".to_owned());
                host.open = false;
                drop(host);
                *option_mount.lock().expect("mount lock") =
                    build(Arc::clone(&option_host), Arc::clone(&option_mount));
            }));

            let mut panel = Node::container().child(option);
            panel.runtime_id = Some("overlay-panel".to_owned());
            panel.style.overlay = true;
            panel.style.descriptor.layout.direction = LayoutDirection::Column;
            panel.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            panel.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            panel.position = NodePosition::Absolute {
                top: Some(40.0),
                left: Some(0.0),
                right: None,
                bottom: None,
            };
            panel.interaction.dismiss_layer = Some("overlay-layer".to_owned());

            trigger_dismiss(&mut root, &dismiss_host, &dismiss_mount);
            root = root.child(panel);
        }
        root
    }

    fn trigger_dismiss(
        root: &mut Node,
        host: &Arc<Mutex<Host>>,
        mounted: &Arc<Mutex<Node>>,
    ) {
        let host = Arc::clone(host);
        let mounted = Arc::clone(mounted);
        if let Some(trigger) = root.children.first_mut() {
            trigger.interaction.dismiss_layer = Some("overlay-layer".to_owned());
            trigger.interaction.on_dismiss = Some(Arc::new(move |reason| {
                let mut state = host.lock().expect("host lock");
                state.dismissals.push(reason);
                state.open = false;
                drop(state);
                *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
            }));
        }
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            open: false,
            activations: Vec::new(),
            dismissals: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 240.0, 180.0);
        driver.wait_for_focus_handle("overlay-trigger");
        driver.pointer_activate_id("overlay-trigger");
        assert!(
            host.lock().expect("host lock").open,
            "pointer on the trigger must rebuild the overlay open"
        );
        driver.draw_frame();
        assert!(
            poodle_gpui_node_backend::bounds_for("overlay-option").is_some(),
            "deferred option must record painted bounds after the open rebuild"
        );
        driver.pointer_activate_id("overlay-option");
        let state = host.lock().expect("host lock").clone();
        assert_eq!(
            state.activations,
            ["option"],
            "pointer on a deferred option row must fire on_activate, not the in-flow widget it covers"
        );
        assert!(
            state.dismissals.is_empty(),
            "an inside option click must not first dispatch outside dismissal"
        );
        assert!(!state.open);
    });
}

/// g16.019. Declared overlay overflow still clips pointer targets that sit
/// past `max_height`. Skipping Taffy overflow would let those rows receive
/// input outside the surface.
#[test]
fn a_capped_deferred_overlay_clips_overflowing_rows() {
    use poodle_adapter::ThemeProvider;

    #[derive(Clone)]
    struct Host {
        open: bool,
        activations: Vec<String>,
    }

    fn build(host: Arc<Mutex<Host>>, mounted: Arc<Mutex<Node>>) -> Node {
        let state = host.lock().expect("host lock").clone();
        let trigger_host = Arc::clone(&host);
        let trigger_mount = Arc::clone(&mounted);
        let mut trigger = Node::container();
        trigger.runtime_id = Some("clip-trigger".to_owned());
        trigger.interaction.focusable = true;
        trigger.style.focus_ring = Some(poodle_node::FocusRing {
            color: theme().resolve_color("color.accent.focusRing"),
            width: theme().resolve_border_width("border.width.focus"),
            offset: 2.0,
        });
        trigger.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);
        trigger.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
        trigger = trigger.child(Node::text("Open"));
        trigger.interaction.on_activate = Some(Arc::new(move || {
            let mut host = trigger_host.lock().expect("host lock");
            host.open = !host.open;
            drop(host);
            *trigger_mount.lock().expect("mount lock") =
                build(Arc::clone(&trigger_host), Arc::clone(&trigger_mount));
        }));

        let mut root = Node::container().child(trigger);
        root.position = NodePosition::Relative;
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        if state.open {
            let mut panel = Node::container();
            panel.runtime_id = Some("clip-panel".to_owned());
            panel.style.overlay = true;
            panel.style.max_height = Some(40.0);
            panel.style.descriptor.layout.direction = LayoutDirection::Column;
            panel.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            panel.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            panel.position = NodePosition::Absolute {
                top: Some(40.0),
                left: Some(0.0),
                right: None,
                bottom: None,
            };
            panel.interaction.dismiss_layer = Some("clip-layer".to_owned());
            for (index, label) in ["one", "two", "three"].iter().enumerate() {
                let option_host = Arc::clone(&host);
                let mut option = Node::container();
                option.runtime_id = Some(format!("clip-option-{index}"));
                option.style.descriptor.layout.height = LayoutSizing::Fixed(32.0);
                option.style.descriptor.layout.width = LayoutSizing::Fixed(160.0);
                option = option.child(Node::text(*label));
                let name = (*label).to_string();
                option.interaction.on_activate = Some(Arc::new(move || {
                    option_host
                        .lock()
                        .expect("host lock")
                        .activations
                        .push(name.clone());
                }));
                panel = panel.child(option);
            }
            root = root.child(panel);
        }
        root
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            open: false,
            activations: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 240.0, 220.0);
        driver.wait_for_focus_handle("clip-trigger");
        driver.pointer_activate_id("clip-trigger");
        driver.draw_frame();
        assert!(poodle_gpui_node_backend::bounds_for("clip-option-0").is_some());
        driver.pointer_activate_id("clip-option-0");
        assert_eq!(
            host.lock().expect("host lock").activations,
            ["one"],
            "the row inside max_height remains a pointer target"
        );
        assert!(poodle_gpui_node_backend::bounds_for("clip-option-2").is_some());
        driver.pointer_activate_id("clip-option-2");
        assert_eq!(
            host.lock().expect("host lock").activations,
            ["one"],
            "a row past max_height is clipped and does not activate"
        );
    });
}

// ── g15.007 Batch A regressions ───────────────────────────────────────────

/// The mounted-window regressions that drive interactive nodes give those
/// nodes explicit ids — the same pattern every retained regression in this
/// file uses. The production preview rebuilds id-less elements within each
/// platform frame; the test platform renders a view several times per draw,
/// so only a declared id keeps an element's state stable across a click.
fn give_first_id(node: &mut Node, id: &str, predicate: &dyn Fn(&Node) -> bool) -> bool {
    if predicate(node) {
        node.id = Some(id.to_owned());
        return true;
    }
    node.children
        .iter_mut()
        .any(|child| give_first_id(child, id, predicate))
}

/// A grouped code input stays one joined value through the real dispatch
/// tree: the separator is presentation-only, so the code reaches the host
/// without hyphens, and a full-length entry completes exactly once.
#[test]
fn a_grouped_code_input_types_and_completes_through_the_real_tree() {
    use poodle_specs::CodeInputSpec;

    run_headless(|cx| {
        let changes = Arc::new(Mutex::new(Vec::new()));
        let completes = Arc::new(Mutex::new(Vec::new()));
        let changes_sink = Arc::clone(&changes);
        let completes_sink = Arc::clone(&completes);

        let mut node = poodle_render::code_input_with_handlers(
            &CodeInputSpec::new()
                .with_length(20)
                .with_groups([5, 5, 5, 5])
                .with_separator("-")
                .with_numbers_only(false),
            &RenderContext::new(&theme()),
            poodle_render::CodeInputHandlers {
                on_value_change: Some(Arc::new(move |value: &str| {
                    changes_sink.lock().unwrap().push(value.to_string())
                })),
                on_complete: Some(Arc::new(move |value: &str| {
                    completes_sink.lock().unwrap().push(value.to_string())
                })),
                ..poodle_render::CodeInputHandlers::default()
            },
        );
        // The slot row takes the keys; give it a stable identity for the
        // mounted window.
        assert!(give_first_id(
            &mut node,
            "code-input-row",
            &|n| n.interaction.focusable,
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // A real pointer press focuses the slot row, then keys walk the focus
        // chain — no handler is invoked as a test shortcut.
        driver.pointer_activate();
        driver.dispatch_key_raw("1");
        driver.dispatch_key_raw("2");
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["1", "2"],
            "each key reaches the row as part of one joined value"
        );

        // Re-mount a full grouped code (the host re-render). Completing a
        // full value through the real tree fires completion exactly once.
        fn build_row(
            value: &str,
            changes_sink: Arc<Mutex<Vec<String>>>,
            completes_sink: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let mut node = poodle_render::code_input_with_handlers(
                &CodeInputSpec::new()
                    .with_length(4)
                    .with_numbers_only(false)
                    .with_value(value),
                &RenderContext::new(&theme()),
                poodle_render::CodeInputHandlers {
                    on_value_change: Some(Arc::new(move |next: &str| {
                        changes_sink.lock().unwrap().push(next.to_string())
                    })),
                    on_complete: Some(Arc::new(move |next: &str| {
                        completes_sink.lock().unwrap().push(next.to_string())
                    })),
                    ..poodle_render::CodeInputHandlers::default()
                },
            );
            // A fresh id: the first mount's row state (and its focus handle)
            // is gone with its element, and the driver keeps one window.
            assert!(give_first_id(
                &mut node,
                "code-input-row-2",
                &|n| n.interaction.focusable,
            ));
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }
        // The value is controlled host state: each keystroke is applied by
        // rebuilding the row with the reported value before the next key —
        // the real host loop, driven through the real dispatch tree.
        let row = Arc::new(Mutex::new(build_row(
            "",
            Arc::clone(&changes),
            Arc::clone(&completes),
        )));
        driver.mount_node(Arc::clone(&row));
        let mut value = String::new();
        for key in ["a", "b", "c", "d"] {
            driver.pointer_activate();
            driver.dispatch_key_raw(key);
            value = changes
                .lock()
                .unwrap()
                .last()
                .cloned()
                .expect("the row reported the keystroke");
            *row.lock().unwrap() = build_row(&value, Arc::clone(&changes), Arc::clone(&completes));
            driver.draw_frame();
        }
        assert_eq!(
            value,
            "abcd",
            "the row accumulates the joined value through the host loop"
        );
        assert_eq!(
            completes.lock().unwrap().as_slice(),
            ["abcd"],
            "completion fires on the transition into a full code, once"
        );
    });
}

/// The completion tick/cross belongs to the exact value it was computed for:
/// a host re-render with an edited value removes the indicator in a mounted
/// window, so a stale result can never render.
#[test]
fn a_stale_completion_result_cannot_render_in_a_mounted_window() {
    use poodle_specs::{CodeInputCompletion, CodeInputSpec};

    fn count_indicators(node: &Node) -> usize {
        fn walk(n: &Node, out: &mut usize) {
            if let poodle_node::NodeKind::Icon { name, .. } = &n.kind {
                if name == "check" || name == "x" {
                    if n.a11y.label.is_some() {
                        *out += 1;
                    }
                }
            }
            for c in &n.children {
                walk(c, out);
            }
        }
        let mut out = 0;
        walk(node, &mut out);
        out
    }

    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let mut checked = poodle_render::code_input_with_handlers(
            &CodeInputSpec::new()
                .with_length(6)
                .with_value("123456")
                .with_completion_result(CodeInputCompletion::Passed("123456".to_string())),
            &ctx,
            poodle_render::CodeInputHandlers::default(),
        );
        checked.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(checked));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        driver.draw_frame();
        assert_eq!(count_indicators(&node.lock().unwrap()), 1, "tick renders");

        // The host edits the value away from the checked one and re-renders
        // through the same mounted node.
        *node.lock().unwrap() = poodle_render::code_input_with_handlers(
            &CodeInputSpec::new()
                .with_length(6)
                .with_value("654321")
                .with_completion_result(CodeInputCompletion::Passed("123456".to_string())),
            &ctx,
            poodle_render::CodeInputHandlers::default(),
        );
        driver.draw_frame();
        assert_eq!(
            count_indicators(&node.lock().unwrap()),
            0,
            "the indicator belongs to the value it was checked against"
        );
    });
}

/// Browse goes through the generic single-file seam: a pointer activation of
/// the dropzone flows fixture bytes through the injected source and the same
/// post-selection pipeline the live OS prompt uses.
#[test]
fn a_dropzone_browse_flows_fixture_bytes_through_the_generic_seam() {
    use poodle_gpui_node_backend::file_capability::{
        InjectedFileSource, PickedFile, SingleFilePickSpec, SingleFileSource, finish_file_pick,
    };

    run_headless(|cx| {
        let outcomes = Arc::new(Mutex::new(Vec::new()));
        let on_browse = {
            let outcomes = Arc::clone(&outcomes);
            Arc::new(move || {
                let mut source = InjectedFileSource::new(Ok(Some(PickedFile {
                    path: "/fixtures/machine.lic".into(),
                    name: "machine.lic".to_string(),
                    bytes: b"fixture payload".to_vec(),
                })));
                let file = source
                    .poll()
                    .expect("fixture resolves immediately")
                    .expect("no read error")
                    .expect("not cancelled");
                let outcome = finish_file_pick(
                    file,
                    &SingleFilePickSpec {
                        prompt: "Choose a licence file".to_string(),
                        accept: Some(".lic".to_string()),
                        max_size: None,
                    },
                );
                outcomes.lock().unwrap().push(outcome);
            })
        };
        let mut node = poodle_render::file_upload_with_handlers(
            &poodle_specs::FileUploadSpec::new().with_accept(".lic"),
            &RenderContext::new(&theme()),
            poodle_render::FileUploadHandlers {
                on_browse: Some(on_browse),
                ..poodle_render::FileUploadHandlers::default()
            },
        );
        // The dropzone carries the browse intent; give it a stable identity
        // for the mounted window.
        assert!(give_first_id(
            &mut node,
            "file-upload-dropzone",
            &|n| n.interaction.on_activate.is_some(),
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));

        driver.pointer_activate();
        let outcomes = outcomes.lock().unwrap();
        assert_eq!(outcomes.len(), 1, "one activation, one pick");
        let selected = match &outcomes[0] {
            poodle_gpui_node_backend::file_capability::FilePickOutcome::Selected {
                name,
                contents_base64,
            } => (name.clone(), contents_base64.clone()),
            other => panic!("expected a selection, got {other:?}"),
        };
        assert_eq!(selected.0, "machine.lic");
        assert_eq!(
            selected.1,
            poodle_headless::file_upload::base64_encode(b"fixture payload"),
            "the same bare-base64 payload the live route produces"
        );
        assert!(!selected.1.starts_with("data:"));
    });
}

/// A dropzone browse that fails the accept rule reports the rejection
/// honestly — GPUI 0.2.2 cannot filter in the OS dialog, so the refusal
/// happens after selection through the same seam.
#[test]
fn a_dropzone_browse_reports_accept_rejection_honestly() {
    use poodle_gpui_node_backend::file_capability::{
        FilePickOutcome, InjectedFileSource, PickedFile, SingleFilePickSpec, SingleFileSource,
        finish_file_pick,
    };

    run_headless(|cx| {
        let outcomes = Arc::new(Mutex::new(Vec::new()));
        let on_browse = {
            let outcomes = Arc::clone(&outcomes);
            Arc::new(move || {
                let mut source = InjectedFileSource::new(Ok(Some(PickedFile {
                    path: "/fixtures/machine.txt".into(),
                    name: "machine.txt".to_string(),
                    bytes: b"x".to_vec(),
                })));
                let file = source
                    .poll()
                    .expect("fixture resolves")
                    .expect("no read error")
                    .expect("not cancelled");
                let outcome = finish_file_pick(
                    file,
                    &SingleFilePickSpec {
                        prompt: "Choose a licence file".to_string(),
                        accept: Some(".lic".to_string()),
                        max_size: None,
                    },
                );
                outcomes.lock().unwrap().push(outcome);
            })
        };
        let mut node = poodle_render::file_upload_with_handlers(
            &poodle_specs::FileUploadSpec::new().with_accept(".lic"),
            &RenderContext::new(&theme()),
            poodle_render::FileUploadHandlers {
                on_browse: Some(on_browse),
                ..poodle_render::FileUploadHandlers::default()
            },
        );
        assert!(give_first_id(
            &mut node,
            "file-upload-dropzone",
            &|n| n.interaction.on_activate.is_some(),
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));

        driver.pointer_activate();
        let outcomes = outcomes.lock().unwrap();
        assert_eq!(outcomes.len(), 1);
        assert_eq!(
            &outcomes[0],
            &FilePickOutcome::Rejected(
                "File type not accepted. Accepted types: .lic".to_string()
            ),
            "the rejection names the accept rule, not a fake OS filter"
        );
    });
}

// ── g15.007 Batch D regressions ───────────────────────────────────────────

/// LicenceActivation's segmented key path: typing a full key through the
/// real dispatch tree drives the composed CodeInput, the injected parser's
/// tick renders at full length, and submit emits the exact structural
/// credential through the shared resolver.
#[test]
fn licence_activation_key_entry_types_and_emits_through_the_real_tree() {
    use poodle_headless::licence::{
        LicenceActivationMode, LicenceActivationRoute, LicenceCredential, LicenceKeyFormat,
        LicenceKeyProblem, LicenceKeyResult, LicenceSubmitDraft, LicenceSubmitResolution,
        resolve_licence_submit,
    };
    use poodle_specs::{LicenceActivationSpec, LicenceKeyCodeInputOptions};

    struct SpecimenKeyFormat;
    impl LicenceKeyFormat for SpecimenKeyFormat {
        fn parse(&self, input: &str) -> LicenceKeyResult {
            let stripped: String = input.chars().filter(|c| *c != '-').collect();
            if stripped.chars().count() < 20 {
                return LicenceKeyResult::Err(LicenceKeyProblem::TooShort {
                    minimum: 20,
                    actual: stripped.chars().count(),
                });
            }
            LicenceKeyResult::Ok {
                key: stripped.clone(),
                grouped: stripped,
            }
        }
        fn is_probably_a_typo(&self, _problem: &LicenceKeyProblem) -> bool {
            false
        }
    }

    run_headless(|cx| {
        let submits = Arc::new(Mutex::new(Vec::new()));
        let changes = Arc::new(Mutex::new(Vec::new()));

        let build = |key: String, submit_sink: Arc<Mutex<Vec<LicenceCredential>>>| {
            let mut node = poodle_render::licence_activation_with_slots(
                &LicenceActivationSpec::new()
                    .with_mode(LicenceActivationMode::Key)
                    .with_key_code_input(
                        LicenceKeyCodeInputOptions::new(20).with_groups([5, 5, 5, 5]),
                    )
                    .with_key_draft(key.clone()),
                &RenderContext::new(&theme()),
                None,
                poodle_render::LicenceActivationHandlers {
                    on_key_change: Some({
                        let changes = Arc::clone(&changes);
                        Arc::new(move |value: &str| {
                            changes.lock().unwrap().push(value.to_string())
                        })
                    }),
                    on_key_check: Some(Arc::new(|input: &str| SpecimenKeyFormat.parse(input))),
                    on_submit: Some({
                        let submit_sink = Arc::clone(&submit_sink);
                        Arc::new(move || {
                            let draft = LicenceSubmitDraft {
                                route: LicenceActivationRoute::Key,
                                key: key.clone(),
                                token: None,
                                file_contents_base64: None,
                                label: String::new(),
                            };
                            if let LicenceSubmitResolution::Emit { credential, .. } =
                                resolve_licence_submit(&draft, Some(&SpecimenKeyFormat))
                            {
                                submit_sink.lock().unwrap().push(credential);
                            }
                        })
                    }),
                    ..poodle_render::LicenceActivationHandlers::default()
                },
            );
            assert!(give_first_id(&mut node, "la-code-row", &|n| n.interaction.focusable));
            assert!(give_first_id(
                &mut node,
                "la-submit",
                &|n| matches!(n.kind, poodle_node::NodeKind::Button { .. }),
            ));
            node.id = Some(FIXTURE_ID.to_owned());
            node
        };

        let node = Arc::new(Mutex::new(build(
            String::new(),
            Arc::clone(&submits),
        )));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // Type a full alphanumeric key through the real dispatch tree, with
        // the host re-rendering the controlled draft after each keystroke.
        let mut value = String::new();
        for ch in "abcdefghijklmnopqrst".chars() {
            driver.pointer_activate();
            driver.dispatch_key_raw(&ch.to_string());
            value = changes
                .lock()
                .unwrap()
                .last()
                .cloned()
                .expect("the row reported the keystroke");
            *node.lock().unwrap() = build(value.clone(), Arc::clone(&submits));
            driver.draw_frame();
        }
        assert_eq!(value, "abcdefghijklmnopqrst");

        // Full length resolves through the injected parser: the tick renders.
        assert!(node
            .lock()
            .unwrap()
            .find(&|n| n.a11y.label.as_deref() == Some("Code check passed"))
            .is_some());

        // Submit emits the exact structural key credential. The submit sits
        // below the mount box, so it is focused and Enter-activated rather
        // than pointer-clicked — the button carries a focus ring, so it
        // tracks focus and gpui synthesizes the click from Enter.
        driver.keyboard_activate("la-submit");
        let submitted = submits.lock().unwrap();
        assert_eq!(
            submitted.as_slice(),
            &[LicenceCredential::Key {
                key: "abcdefghijklmnopqrst".to_string()
            }],
            "the raw accepted key is emitted exactly once"
        );
    });
}

/// LicenceSeats release flows through the composed ConfirmAction in a
/// mounted window: the confirmed release emits the exact machine id and the
/// raw id never appears in rendered or accessible text.
#[test]
fn licence_seats_release_flows_through_confirm_in_a_mounted_window() {
    use poodle_headless::licence::LicenceSeat;
    use poodle_specs::LicenceSeatsSpec;

    run_headless(|cx| {
        let released = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&released);
        let mut node = poodle_render::licence_seats(
            &LicenceSeatsSpec::new()
                .with_seats(vec![
                    LicenceSeat {
                        machine_id: "id-a".to_string(),
                        label: Some("Studio rig".to_string()),
                        this_machine: true,
                    },
                    LicenceSeat {
                        machine_id: "id-b".to_string(),
                        label: None,
                        this_machine: false,
                    },
                ])
                .with_open_confirm(Some("id-b".to_string())),
            &RenderContext::new(&theme()),
            poodle_render::LicenceSeatsHandlers {
                on_release: Some(Arc::new(move |machine_id: &str| {
                    sink.lock().unwrap().push(machine_id.to_string())
                })),
                ..poodle_render::LicenceSeatsHandlers::default()
            },
        );
        // The confirm dialog is open (spec state), so its confirm button —
        // labelled with the release label — is the release affordance.
        assert!(give_first_id(
            &mut node,
            "seats-confirm",
            &|n| matches!(&n.kind, poodle_node::NodeKind::Button { label } if label == "Release"),
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.pointer_activate_id("seats-confirm");
        assert_eq!(
            released.lock().unwrap().as_slice(),
            ["id-b"],
            "the confirm button releases the exact machine id"
        );
        assert!(!node
            .lock()
            .unwrap()
            .texts()
            .iter()
            .any(|t| t.contains("id-a") || t.contains("id-b")),
            "raw machine ids never reach rendered or accessible text"
        );
    });
}

/// LicenceStatus renders the supplied state and authority reads in a mounted
/// window: the calm inGrace treatment, the absolute quiet detail, and the
/// data-state roles that gate nothing.
#[test]
fn licence_status_renders_state_and_authority_reads_in_a_mounted_window() {
    use poodle_headless::licence::{LicenceTrustBasis, LicenceUsability};
    use poodle_specs::LicenceStatusSpec;

    run_headless(|cx| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let mut node = poodle_render::licence_status(
            &LicenceStatusSpec::new()
                .with_usability(LicenceUsability::InGrace { until: now + 86_400 })
                .with_trust_basis(LicenceTrustBasis::OfflineSignature)
                .with_use_until(Some(now + 86_400))
                .with_update_until(None)
                .with_usable(true),
            &RenderContext::new(&theme()),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        driver.draw_frame();

        let node = node.lock().unwrap();
        let texts = node.texts();
        assert!(
            texts.iter().any(|t| *t == "Licence active"),
            "inGrace keeps the calm title"
        );
        assert!(
            texts.iter().any(|t| t.starts_with("Use continues until")),
            "the quiet detail carries the absolute date"
        );
        assert_eq!(node.roles.get("state").map(String::as_str), Some("inGrace"));
        assert_eq!(node.roles.get("usable").map(String::as_str), Some("true"));
        assert_eq!(
            node.a11y.label.as_deref(),
            Some("Licence"),
            "the section carries the accessible name"
        );
    });
}

/// LicenceActivation's account-mode submit is the defining action: pressing
/// the Activate button through the real dispatch tree fires the host-owned
/// acquisition request (the specimen's provider then cancels).
#[test]
fn licence_activation_account_submit_fires_through_the_real_tree() {
    use poodle_headless::licence::LicenceActivationMode;
    use poodle_specs::LicenceActivationSpec;

    run_headless(|cx| {
        let submits = Arc::new(Mutex::new(0usize));
        let sink = Arc::clone(&submits);
        let mut node = poodle_render::licence_activation_with_slots(
            &LicenceActivationSpec::new()
                .with_mode(LicenceActivationMode::Account)
                .with_machine_label(Some("Studio Mac".to_string())),
            &RenderContext::new(&theme()),
            Some(Node::text("host login form")),
            poodle_render::LicenceActivationHandlers {
                on_submit: Some(Arc::new(move || {
                    *sink.lock().unwrap() += 1;
                })),
                ..poodle_render::LicenceActivationHandlers::default()
            },
        );
        // The account-view submit carries the default copy; the header route
        // switch is a different button, so target the submit by its label.
        assert!(give_first_id(
            &mut node,
            "la-account-submit",
            &|n| matches!(&n.kind, poodle_node::NodeKind::Button { label } if label == "Continue with account"),
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, node);

        driver.keyboard_activate("la-account-submit");
        assert_eq!(
            *submits.lock().unwrap(),
            1,
            "the account Activate button fires the host request"
        );
    });
}

/// Editing the key clears the local validation copy: after a rejected
/// submit the message shows, and a new keystroke removes it — the web pair's
/// keyMessage-clearing rule, through the real dispatch tree.
#[test]
fn key_validation_copy_clears_on_edit_in_a_mounted_window() {
    use poodle_headless::licence::LicenceActivationMode;
    use poodle_specs::LicenceActivationSpec;

    run_headless(|cx| {
        let changed = Arc::new(Mutex::new(0usize));
        let sink = Arc::clone(&changed);
        let build = |message: Option<&str>| {
            let mut node = poodle_render::licence_activation(
                &LicenceActivationSpec::new()
                    .with_mode(LicenceActivationMode::Key)
                    .with_key_message(message.map(str::to_string)),
                &RenderContext::new(&theme()),
                poodle_render::LicenceActivationHandlers {
                    on_key_change: Some({
                        let sink = Arc::clone(&sink);
                        Arc::new(move |_value: &str| {
                            *sink.lock().unwrap() += 1;
                        })
                    }),
                    ..poodle_render::LicenceActivationHandlers::default()
                },
            );
            assert!(give_first_id(
                &mut node,
                "la-key-input",
                &|n| n.interaction.on_text_change.is_some(),
            ));
            node.id = Some(FIXTURE_ID.to_owned());
            node
        };
        let node = Arc::new(Mutex::new(build(Some("This key is too short."))));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        driver.draw_frame();
        assert!(
            node.lock().unwrap().has_text("This key is too short."),
            "the rejected submit shows its copy"
        );

        // A new keystroke fires on_key_change; the host clears the stale copy
        // (the web pair's handleKeyChange) and re-renders.
        driver.pointer_activate_id("la-key-input");
        driver.dispatch_key_raw("a");
        assert_eq!(*changed.lock().unwrap(), 1, "the key edit fired");
        *node.lock().unwrap() = build(None);
        driver.draw_frame();
        assert!(
            !node.lock().unwrap().has_text("This key is too short."),
            "editing the key removes the stale validation copy"
        );
        assert!(
            node.lock()
                .unwrap()
                .find(&|n| n.roles.get("validation").map(String::as_str) == Some("invalid"))
                .is_none(),
            "editing the key removes invalid state rather than leaving an empty error"
        );
    });
}

/// Escape on the machine-name edit restores the committed value: after
/// typing a new draft, Escape returns the display to the original label —
/// the web EditableLabel's revert rule, through the real dispatch tree.
#[test]
fn a_machine_name_escape_restores_the_original_in_a_mounted_window() {
    use poodle_headless::licence::LicenceActivationMode;
    use poodle_specs::LicenceActivationSpec;

    run_headless(|cx| {
        let draft = Arc::new(Mutex::new("Studio Mac".to_string()));
        let cancelled = Arc::new(Mutex::new(0usize));
        let build = |label: &str, editing: bool| {
            let len = label.chars().count();
            let mut spec = LicenceActivationSpec::new()
                .with_mode(LicenceActivationMode::Account)
                .with_machine_label(Some(label.to_string()))
                .with_machine_label_editing(editing);
            // Already-editing hosts must project caret. End-caret makes the
            // typed suffix unique for the Escape-restore check below.
            if editing {
                spec = spec
                    .with_machine_label_draft(Some(label.to_string()))
                    .with_machine_label_selection(len, len);
            }
            let mut node = poodle_render::licence_activation(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::LicenceActivationHandlers {
                    on_machine_label_change: Some({
                        let draft = Arc::clone(&draft);
                        Arc::new(move |value: &str| {
                            *draft.lock().unwrap() = value.to_string();
                        })
                    }),
                    on_machine_label_cancel: Some({
                        let cancelled = Arc::clone(&cancelled);
                        Arc::new(move || {
                            *cancelled.lock().unwrap() += 1;
                        })
                    }),
                    ..poodle_render::LicenceActivationHandlers::default()
                },
            );
            if editing {
                assert!(give_first_id(
                    &mut node,
                    "la-machine-input",
                    &|n| n.interaction.on_text_change.is_some(),
                ));
            }
            node.id = Some(FIXTURE_ID.to_owned());
            node
        };
        let node = Arc::new(Mutex::new(build("Studio Mac", true)));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // Type a new draft into the editing input. It now carries a focus
        // ring, so it tracks focus and can be focused by id regardless of
        // where it sits in the wide form.
        driver.focus_element("la-machine-input");
        driver.dispatch_key_raw("2");
        assert_eq!(
            draft.lock().unwrap().as_str(),
            "Studio Mac2",
            "typing edits the draft"
        );

        // Escape fires the cancel channel; the host restores the committed
        // value snapped at edit start and closes editing.
        driver.dispatch_key_raw("escape");
        assert_eq!(*cancelled.lock().unwrap(), 1, "escape reached the cancel channel");
        *node.lock().unwrap() = build("Studio Mac", false);
        driver.draw_frame();
        assert!(
            node.lock().unwrap().has_text("Studio Mac"),
            "the original label is restored"
        );
        assert!(
            !node.lock().unwrap().has_text("Studio Mac2"),
            "the typed draft is discarded on escape"
        );
    });
}

const MACHINE_NAME_FOCUS_ID: &str = "la-machine-name-focus";
const MACHINE_NAME_SUBMIT_ID: &str = "la-machine-name-submit";
const SEAT_RENAME_FOCUS_ID: &str = "seat-rename-focus";

struct MachineNameFocusHost {
    label: Mutex<String>,
    draft: Mutex<String>,
    selection: Mutex<(usize, usize)>,
    editing: Mutex<bool>,
    request_focus: Mutex<bool>,
    log: EventLog,
}

fn stamp_machine_name_focus_ids(node: &mut Node) {
    assert!(
        give_first_id(node, MACHINE_NAME_FOCUS_ID, &|n| {
            n.a11y.label.as_deref() == Some("Edit machine name")
        }),
        "machine-name EditableLabel"
    );
    assert!(
        give_first_id(node, MACHINE_NAME_SUBMIT_ID, &|n| {
            matches!(&n.kind, NodeKind::Button { label } if label == "Continue with account")
        }),
        "account submit"
    );
}

fn machine_name_focus_tree(host: &Arc<MachineNameFocusHost>, mounted: &Arc<Mutex<Node>>) -> Node {
    use poodle_headless::licence::LicenceActivationMode;
    use poodle_specs::LicenceActivationSpec;

    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let editing = *host.editing.lock().expect("editing");
    let label = host.label.lock().expect("label").clone();
    let draft = host.draft.lock().expect("draft").clone();
    let selection = *host.selection.lock().expect("selection");
    let restore = {
        let mut flag = host.request_focus.lock().expect("request_focus");
        let restore = *flag;
        *flag = false;
        restore
    };
    let mut spec = LicenceActivationSpec::new()
        .with_mode(LicenceActivationMode::Account)
        .with_machine_label(Some(label))
        .with_machine_label_editing(editing)
        .with_machine_label_request_focus(restore);
    if editing {
        spec = spec
            .with_machine_label_draft(Some(draft))
            .with_machine_label_selection(selection.0, selection.1);
    }

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let select_host = Arc::clone(host);
    let select_mount = Arc::clone(mounted);
    let commit_host = Arc::clone(host);
    let commit_mount = Arc::clone(mounted);
    let cancel_host = Arc::clone(host);
    let cancel_mount = Arc::clone(mounted);
    let restore_host = Arc::clone(host);
    let restore_mount = Arc::clone(mounted);
    let mut node = poodle_render::licence_activation(
        &spec,
        &ctx,
        poodle_render::LicenceActivationHandlers {
            on_machine_label_change: Some(Arc::new(move |next: &str| {
                *change_host.draft.lock().expect("draft") = next.to_owned();
                note(&change_host.log, format!("machine/change:{next}"));
                let tree = machine_name_focus_tree(&change_host, &change_mount);
                *change_mount.lock().expect("mount") = tree;
            })),
            on_machine_label_selection_change: Some(Arc::new(move |start, end| {
                *select_host.selection.lock().expect("selection") = (start, end);
                let tree = machine_name_focus_tree(&select_host, &select_mount);
                *select_mount.lock().expect("mount") = tree;
            })),
            on_machine_label_commit: Some(Arc::new(move |next: &str| {
                if !*commit_host.editing.lock().expect("editing") {
                    return;
                }
                *commit_host.editing.lock().expect("editing") = false;
                *commit_host.label.lock().expect("label") = next.to_owned();
                note(&commit_host.log, format!("machine/commit:{next}"));
                let tree = machine_name_focus_tree(&commit_host, &commit_mount);
                *commit_mount.lock().expect("mount") = tree;
            })),
            on_machine_label_cancel: Some(Arc::new(move || {
                if !*cancel_host.editing.lock().expect("editing") {
                    return;
                }
                *cancel_host.editing.lock().expect("editing") = false;
                let restored = cancel_host.label.lock().expect("label").clone();
                *cancel_host.draft.lock().expect("draft") = restored;
                note(&cancel_host.log, "machine/cancel".to_owned());
                let tree = machine_name_focus_tree(&cancel_host, &cancel_mount);
                *cancel_mount.lock().expect("mount") = tree;
            })),
            on_machine_label_restore_display_focus: Some(Arc::new(move || {
                *restore_host.request_focus.lock().expect("request_focus") = true;
                note(&restore_host.log, "machine/restore".to_owned());
                let tree = machine_name_focus_tree(&restore_host, &restore_mount);
                *restore_mount.lock().expect("mount") = tree;
            })),
            ..poodle_render::LicenceActivationHandlers::default()
        },
    );
    stamp_machine_name_focus_ids(&mut node);
    routing_column(vec![
        traversal_marker("la-machine-before", &host.log, &ctx),
        node,
        traversal_marker("la-machine-after", &host.log, &ctx),
    ])
}

fn machine_name_focus_host() -> Arc<MachineNameFocusHost> {
    Arc::new(MachineNameFocusHost {
        label: Mutex::new("Studio Mac".to_owned()),
        draft: Mutex::new("Studio Mac".to_owned()),
        selection: Mutex::new((10, 10)),
        editing: Mutex::new(true),
        request_focus: Mutex::new(false),
        log: event_log(),
    })
}

/// g16.045. LicenceActivation `machine_name` carries EditableLabel's Enter/
/// Escape display-focus restore. Tab still commits through blur and advances.
#[test]
fn licence_activation_machine_name_enter_and_escape_restore_display_focus() {
    run_headless(|cx| {
        let host = machine_name_focus_host();
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = machine_name_focus_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 360.0);
        driver.wait_for_focus_handle(MACHINE_NAME_FOCUS_ID);
        driver.focus_element(MACHINE_NAME_FOCUS_ID);
        driver.dispatch_key_raw("2");
        take_events(&host.log);
        driver.dispatch_key_raw("enter");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["machine/commit:Studio Mac2", "machine/restore"]);
        assert_eq!(*host.label.lock().expect("label"), "Studio Mac2");
        assert!(!*host.editing.lock().expect("editing"));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(MACHINE_NAME_FOCUS_ID),
            Some(true),
            "Enter restores the machine-name display focus handle"
        );
    });

    run_headless(|cx| {
        let host = machine_name_focus_host();
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = machine_name_focus_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 360.0);
        driver.wait_for_focus_handle(MACHINE_NAME_FOCUS_ID);
        driver.focus_element(MACHINE_NAME_FOCUS_ID);
        driver.dispatch_key_raw("2");
        take_events(&host.log);
        driver.dispatch_key_raw("escape");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["machine/cancel", "machine/restore"]);
        assert_eq!(*host.label.lock().expect("label"), "Studio Mac");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(MACHINE_NAME_FOCUS_ID),
            Some(true),
            "Escape restores the machine-name display focus handle"
        );
    });

    run_headless(|cx| {
        let host = machine_name_focus_host();
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = machine_name_focus_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 360.0);
        driver.wait_for_focus_handle(MACHINE_NAME_FOCUS_ID);
        driver.focus_element(MACHINE_NAME_FOCUS_ID);
        driver.dispatch_key_raw("2");
        take_events(&host.log);
        driver.dispatch_key_raw("tab");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["machine/commit:Studio Mac2"]);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(MACHINE_NAME_SUBMIT_ID),
            Some(true),
            "Tab advances to the next focusable in the activation form"
        );
        assert_ne!(
            poodle_gpui_node_backend::focus_state_for(MACHINE_NAME_FOCUS_ID),
            Some(true),
            "Tab does not restore machine-name display focus"
        );
    });
}

struct SeatRenameFocusHost {
    machine_id: String,
    label: Mutex<Option<String>>,
    draft: Mutex<String>,
    selection: Mutex<(usize, usize)>,
    editing: Mutex<bool>,
    request_focus: Mutex<bool>,
    log: EventLog,
}

fn seat_rename_focus_tree(host: &Arc<SeatRenameFocusHost>, mounted: &Arc<Mutex<Node>>) -> Node {
    use poodle_headless::licence::LicenceSeat;
    use poodle_specs::LicenceSeatsSpec;

    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let editing = *host.editing.lock().expect("editing");
    let label = host.label.lock().expect("label").clone();
    let draft = host.draft.lock().expect("draft").clone();
    let selection = *host.selection.lock().expect("selection");
    let restore = {
        let mut flag = host.request_focus.lock().expect("request_focus");
        let restore = *flag;
        *flag = false;
        restore
    };
    let spec = LicenceSeatsSpec::new()
        .with_seats(vec![LicenceSeat {
            machine_id: host.machine_id.clone(),
            label,
            this_machine: true,
        }])
        .with_editing_machine(editing.then(|| host.machine_id.clone()))
        .with_editing_draft(editing.then_some(draft))
        .with_editing_selection(selection.0, selection.1)
        .with_request_focus_machine(restore.then(|| host.machine_id.clone()));

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let select_host = Arc::clone(host);
    let select_mount = Arc::clone(mounted);
    let commit_host = Arc::clone(host);
    let commit_mount = Arc::clone(mounted);
    let cancel_host = Arc::clone(host);
    let cancel_mount = Arc::clone(mounted);
    let restore_host = Arc::clone(host);
    let restore_mount = Arc::clone(mounted);
    let mut node = poodle_render::licence_seats(
        &spec,
        &ctx,
        poodle_render::LicenceSeatsHandlers {
            on_rename_change: Some(Arc::new(move |_id, next: &str| {
                *change_host.draft.lock().expect("draft") = next.to_owned();
                note(&change_host.log, format!("seat/change:{next}"));
                let tree = seat_rename_focus_tree(&change_host, &change_mount);
                *change_mount.lock().expect("mount") = tree;
            })),
            on_rename_selection_change: Some(Arc::new(move |_id, start, end| {
                *select_host.selection.lock().expect("selection") = (start, end);
                let tree = seat_rename_focus_tree(&select_host, &select_mount);
                *select_mount.lock().expect("mount") = tree;
            })),
            on_rename: Some(Arc::new(move |_id, next: Option<&str>| {
                if !*commit_host.editing.lock().expect("editing") {
                    return;
                }
                *commit_host.editing.lock().expect("editing") = false;
                *commit_host.label.lock().expect("label") = next.map(str::to_string);
                note(
                    &commit_host.log,
                    format!("seat/commit:{}", next.unwrap_or("")),
                );
                let tree = seat_rename_focus_tree(&commit_host, &commit_mount);
                *commit_mount.lock().expect("mount") = tree;
            })),
            on_rename_cancel: Some(Arc::new(move |_id| {
                if !*cancel_host.editing.lock().expect("editing") {
                    return;
                }
                *cancel_host.editing.lock().expect("editing") = false;
                let restored = cancel_host
                    .label
                    .lock()
                    .expect("label")
                    .clone()
                    .unwrap_or_default();
                *cancel_host.draft.lock().expect("draft") = restored;
                note(&cancel_host.log, "seat/cancel".to_owned());
                let tree = seat_rename_focus_tree(&cancel_host, &cancel_mount);
                *cancel_mount.lock().expect("mount") = tree;
            })),
            on_rename_restore_display_focus: Some(Arc::new(move |_id| {
                *restore_host.request_focus.lock().expect("request_focus") = true;
                note(&restore_host.log, "seat/restore".to_owned());
                let tree = seat_rename_focus_tree(&restore_host, &restore_mount);
                *restore_mount.lock().expect("mount") = tree;
            })),
            ..poodle_render::LicenceSeatsHandlers::default()
        },
    );
    assert!(
        give_first_id(&mut node, SEAT_RENAME_FOCUS_ID, &|n| {
            n.a11y
                .label
                .as_deref()
                .is_some_and(|label| label.starts_with("Rename "))
        }),
        "seat-row EditableLabel"
    );
    routing_column(vec![
        traversal_marker("seat-rename-before", &host.log, &ctx),
        node,
        traversal_marker("seat-rename-after", &host.log, &ctx),
    ])
}

fn seat_rename_focus_host() -> Arc<SeatRenameFocusHost> {
    Arc::new(SeatRenameFocusHost {
        machine_id: "id-a".to_owned(),
        label: Mutex::new(Some("Studio rig".to_owned())),
        draft: Mutex::new("Studio rig".to_owned()),
        selection: Mutex::new((10, 10)),
        editing: Mutex::new(true),
        request_focus: Mutex::new(false),
        log: event_log(),
    })
}

/// g16.045. LicenceSeats `seat_row` carries EditableLabel's Enter/Escape
/// display-focus restore. Tab still commits through blur and advances.
#[test]
fn licence_seats_seat_row_enter_and_escape_restore_display_focus() {
    run_headless(|cx| {
        let host = seat_rename_focus_host();
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = seat_rename_focus_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 240.0);
        driver.wait_for_focus_handle(SEAT_RENAME_FOCUS_ID);
        driver.focus_element(SEAT_RENAME_FOCUS_ID);
        driver.dispatch_key_raw("2");
        take_events(&host.log);
        driver.dispatch_key_raw("enter");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["seat/commit:Studio rig2", "seat/restore"]);
        assert_eq!(
            host.label.lock().expect("label").as_deref(),
            Some("Studio rig2")
        );
        assert!(!*host.editing.lock().expect("editing"));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(SEAT_RENAME_FOCUS_ID),
            Some(true),
            "Enter restores the seat-row display focus handle"
        );
    });

    run_headless(|cx| {
        let host = seat_rename_focus_host();
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = seat_rename_focus_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 240.0);
        driver.wait_for_focus_handle(SEAT_RENAME_FOCUS_ID);
        driver.focus_element(SEAT_RENAME_FOCUS_ID);
        driver.dispatch_key_raw("2");
        take_events(&host.log);
        driver.dispatch_key_raw("escape");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["seat/cancel", "seat/restore"]);
        assert_eq!(
            host.label.lock().expect("label").as_deref(),
            Some("Studio rig")
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(SEAT_RENAME_FOCUS_ID),
            Some(true),
            "Escape restores the seat-row display focus handle"
        );
    });

    run_headless(|cx| {
        let host = seat_rename_focus_host();
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = seat_rename_focus_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 240.0);
        driver.wait_for_focus_handle(SEAT_RENAME_FOCUS_ID);
        driver.focus_element(SEAT_RENAME_FOCUS_ID);
        driver.dispatch_key_raw("2");
        take_events(&host.log);
        driver.dispatch_key_raw("tab");
        driver.draw_frame();
        assert_eq!(
            take_events(&host.log),
            vec!["seat/commit:Studio rig2", "seat-rename-after/focus:true"]
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-seat-rename-after"),
            Some(true),
            "Tab advances past the seat row"
        );
        assert_ne!(
            poodle_gpui_node_backend::focus_state_for(SEAT_RENAME_FOCUS_ID),
            Some(true),
            "Tab does not restore seat-row display focus"
        );
    });
}


// ── Model-connection family (g15.008) ──────────────────────────────────────

/// The picker's roving focus is real backend focus: an arrow key on the
/// mounted option moves the window's focus to the next enabled option and
/// selects it, and the disabled routes in between are skipped.
#[test]
fn model_connection_picker_roving_focus_moves_real_backend_focus() {
    use poodle_headless::model_connection::model_connection_picker_fixtures;
    use poodle_render::model_connection_option_id;
    use poodle_specs::ModelConnectionPickerSpec;

    run_headless(|cx| {
        let chosen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&chosen);
        let mut node = poodle_render::model_connection_picker(
            &ModelConnectionPickerSpec::new()
                .with_options(model_connection_picker_fixtures())
                .with_value(Some("anthropic-messages".to_string())),
            &RenderContext::new(&theme()),
            poodle_render::ModelConnectionPickerHandlers {
                on_value_change: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                ..poodle_render::ModelConnectionPickerHandlers::default()
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        let from = model_connection_option_id("anthropic-messages");
        // `ollama-local` is the next *enabled* option: `codex-app` is checking
        // and disabled, so the roving move must step over it.
        let to = model_connection_option_id("ollama-local");
        driver.wait_for_focus_handle(&from);
        driver.keyboard_key(&from, "down");

        assert_eq!(
            chosen.lock().unwrap().as_slice(),
            ["ollama-local"],
            "the move selects the option it moved to"
        );
        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&to),
            Some(true),
            "the backend moved real focus to the named destination"
        );
    });
}

/// A disabled route is inert in a mounted window: a real pointer click on the
/// unsupported option's rendered bounds selects nothing, while the available
/// one beside it selects on the same gesture.
///
/// Two options only: the mount box centres its child, so a full catalogue
/// overflows above the window and its top rows cannot be hit-tested.
#[test]
fn model_connection_picker_ignores_a_click_on_an_unsupported_route() {
    use poodle_headless::model_connection::{
        ModelConnectionAvailability, ModelConnectionOption,
    };
    use poodle_render::model_connection_option_id;
    use poodle_specs::ModelConnectionPickerSpec;

    run_headless(|cx| {
        let chosen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&chosen);
        let options = vec![
            ModelConnectionOption::new("vendor-legacy", "Legacy Vendor", "Hosted")
                .with_availability(
                    ModelConnectionAvailability::Unsupported,
                    "Unsupported on this platform",
                )
                .with_disabled(true),
            ModelConnectionOption::new("openai-responses", "OpenAI", "Hosted"),
        ];
        let mut node = poodle_render::model_connection_picker(
            &ModelConnectionPickerSpec::new().with_options(options),
            &RenderContext::new(&theme()),
            poodle_render::ModelConnectionPickerHandlers {
                on_value_change: Some(Arc::new(move |id: &str| {
                    sink.lock().unwrap().push(id.to_string())
                })),
                ..poodle_render::ModelConnectionPickerHandlers::default()
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.pointer_activate_id(&model_connection_option_id("vendor-legacy"));
        assert!(
            chosen.lock().unwrap().is_empty(),
            "an unsupported route cannot be chosen by pointer either"
        );

        driver.pointer_activate_id(&model_connection_option_id("openai-responses"));
        assert_eq!(chosen.lock().unwrap().as_slice(), ["openai-responses"]);
    });
}

/// The setup workflow's direct-add path in a mounted window: pressing Add on
/// a route that needs no configuration submits from choose and never asks for
/// a configure stage.
#[test]
fn model_connection_setup_direct_add_submits_from_choose_in_a_mounted_window() {
    use poodle_headless::model_connection::{
        model_connection_picker_fixtures, ModelConnectionAvailability,
    };
    use poodle_specs::ModelConnectionSetupSpec;

    run_headless(|cx| {
        let submits = Arc::new(Mutex::new(Vec::new()));
        let stages = Arc::new(Mutex::new(Vec::new()));
        let submit_sink = Arc::clone(&submits);
        let stage_sink = Arc::clone(&stages);
        let options = model_connection_picker_fixtures()
            .into_iter()
            .map(|option| {
                if option.id == "codex-app" {
                    option
                        .with_availability(ModelConnectionAvailability::Available, "Available")
                        .with_disabled(false)
                } else {
                    option
                }
            })
            .collect();
        let mut node = poodle_render::model_connection_setup(
            &ModelConnectionSetupSpec::new()
                .with_options(options)
                .with_value(Some("codex-app".to_string()))
                .with_can_submit(true),
            &RenderContext::new(&theme()),
            poodle_render::ModelConnectionSetupHandlers {
                on_submit: Some(Arc::new(move |id: &str| {
                    submit_sink.lock().unwrap().push(id.to_string())
                })),
                on_stage_change: Some(Arc::new(move |stage| {
                    stage_sink.lock().unwrap().push(stage)
                })),
                ..poodle_render::ModelConnectionSetupHandlers::default()
            },
        );
        assert!(give_first_id(
            &mut node,
            "setup-add",
            &|n| matches!(&n.kind, poodle_node::NodeKind::Button { label } if label == "Add connection"),
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.pointer_activate_id("setup-add");
        assert_eq!(submits.lock().unwrap().as_slice(), ["codex-app"]);
        assert!(
            stages.lock().unwrap().is_empty(),
            "a direct route skips the configure stage entirely"
        );
    });
}

/// The card's two dimensions stay independent through the real event tree,
/// and closing the details region returns real backend focus to the
/// disclosure control.
#[test]
fn model_connection_card_closes_and_returns_real_focus_to_the_disclosure() {
    use poodle_headless::model_connection::ModelConnectionReadiness;
    use poodle_specs::ModelConnectionCardSpec;

    run_headless(|cx| {
        let opens = Arc::new(Mutex::new(Vec::new()));
        let enables = Arc::new(Mutex::new(Vec::new()));
        let open_sink = Arc::clone(&opens);
        let enable_sink = Arc::clone(&enables);
        let spec = ModelConnectionCardSpec::new("conn-openai-work", "OpenAI · Work", "OpenAI")
            .with_route_label("Responses API")
            .with_access_summary("API key on file")
            .with_readiness(ModelConnectionReadiness::Ready, "Ready")
            .with_open(true);
        let disclosure_id = spec.disclosure_id();
        let mut node = poodle_render::model_connection_card_with_slots(
            &spec,
            &RenderContext::new(&theme()),
            poodle_render::ModelConnectionCardSlots {
                details: Some(poodle_node::Node::text("Host details")),
                ..poodle_render::ModelConnectionCardSlots::default()
            },
            poodle_render::ModelConnectionCardHandlers {
                on_open_change: Some(Arc::new(move |open| open_sink.lock().unwrap().push(open))),
                on_enabled_change: Some(Arc::new(move |enabled| {
                    enable_sink.lock().unwrap().push(enabled)
                })),
                on_focus_request: Some(Arc::new(|id: &str| {
                    // The bridge the preview uses: the component names the
                    // destination, the backend performs the move.
                    poodle_gpui_node_backend::request_focus(id);
                })),
                ..poodle_render::ModelConnectionCardHandlers::default()
            },
        );
        assert!(give_first_id(
            &mut node,
            "card-switch",
            &|n| n.a11y.label.as_deref() == Some("Enable OpenAI · Work"),
        ));
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.pointer_activate_id(&disclosure_id);
        assert_eq!(opens.lock().unwrap().as_slice(), [false]);
        assert!(
            enables.lock().unwrap().is_empty(),
            "disclosing never touches the enable preference"
        );
        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&disclosure_id),
            Some(true),
            "closing returns real focus to the disclosure control"
        );

        driver.pointer_activate_id("card-switch");
        assert_eq!(enables.lock().unwrap().as_slice(), [false]);
        assert_eq!(
            opens.lock().unwrap().as_slice(),
            [false],
            "the enable preference never touches disclosure"
        );
    });
}

/// The catalogue editor's keyboard reorder through the real dispatch tree:
/// activating the handle grabs, an arrow moves the grabbed row and emits the
/// complete shown order, and Escape cancels the grab.
#[test]
fn model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window() {
    use poodle_headless::model_connection::model_catalogue_fixtures;
    use poodle_specs::ModelCatalogueEditorSpec;

    run_headless(|cx| {
        let orders = Arc::new(Mutex::new(Vec::new()));
        let grabs = Arc::new(Mutex::new(Vec::new()));
        let announcements = Arc::new(Mutex::new(Vec::new()));

        let build = |grabbed: Option<String>,
                     orders: Arc<Mutex<Vec<Vec<String>>>>,
                     grabs: Arc<Mutex<Vec<Option<String>>>>,
                     announcements: Arc<Mutex<Vec<String>>>| {
            let mut node = poodle_render::model_catalogue_editor(
                &ModelCatalogueEditorSpec::new()
                    .with_items(model_catalogue_fixtures())
                    .with_grabbed(grabbed),
                &RenderContext::new(&theme()),
                poodle_render::ModelCatalogueEditorHandlers {
                    on_order_change: Some(Arc::new(move |order: &[String]| {
                        orders.lock().unwrap().push(order.to_vec())
                    })),
                    on_grab_change: Some(Arc::new(move |id: Option<&str>| {
                        grabs.lock().unwrap().push(id.map(str::to_string))
                    })),
                    on_announce: Some(Arc::new(move |message: &str| {
                        announcements.lock().unwrap().push(message.to_string())
                    })),
                    ..poodle_render::ModelCatalogueEditorHandlers::default()
                },
            );
            node.id = Some(FIXTURE_ID.to_owned());
            Arc::new(Mutex::new(node))
        };

        let handle = "model-catalogue-editor:model-beta:handle";
        let node = build(
            None,
            Arc::clone(&orders),
            Arc::clone(&grabs),
            Arc::clone(&announcements),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // Enter on the handle grabs the row through the backend's own
        // activation path.
        driver.wait_for_focus_handle(handle);
        driver.keyboard_activate(handle);
        assert_eq!(
            grabs.lock().unwrap().as_slice(),
            [Some("model-beta".to_string())]
        );

        // The host applied the grab; the next render moves on arrow keys.
        let grabbed = build(
            Some("model-beta".to_string()),
            Arc::clone(&orders),
            Arc::clone(&grabs),
            Arc::clone(&announcements),
        );
        driver.mount_node(Arc::clone(&grabbed));
        driver.wait_for_focus_handle(handle);
        driver.keyboard_key(handle, "down");
        assert_eq!(
            orders.lock().unwrap().last().expect("an order").as_slice(),
            [
                "model-alpha".to_string(),
                "model-gamma".to_string(),
                "model-beta".to_string(),
                "model-dup-a".to_string(),
            ],
            "the move emits the complete shown-id order"
        );

        // Escape cancels the live grab through the real key dispatch.
        driver.keyboard_key(handle, "escape");
        assert_eq!(grabs.lock().unwrap().last().expect("a grab"), &None);
        assert!(announcements
            .lock()
            .unwrap()
            .contains(&"Cancelled keyboard move.".to_string()));
    });
}

/// Hiding a shown model in a mounted window emits only a visibility request
/// and moves real backend focus to the next shown model's handle.
///
/// Three rows only, for the same hit-testing reason as the picker above.
#[test]
fn model_catalogue_editor_hide_moves_real_focus_to_the_next_shown_model() {
    use poodle_headless::model_connection::{ModelCatalogueItem, ModelCatalogueVisibilityChange};
    use poodle_specs::ModelCatalogueEditorSpec;

    run_headless(|cx| {
        let changes = Arc::new(Mutex::new(Vec::new()));
        let orders = Arc::new(Mutex::new(Vec::new()));
        let change_sink = Arc::clone(&changes);
        let order_sink = Arc::clone(&orders);
        let items = vec![
            ModelCatalogueItem::new("model-alpha", "Frontier Alpha"),
            ModelCatalogueItem::new("model-beta", "Frontier Beta"),
            ModelCatalogueItem::new("model-gamma", "Gateway Gamma"),
        ];
        let mut node = poodle_render::model_catalogue_editor(
            &ModelCatalogueEditorSpec::new().with_items(items),
            &RenderContext::new(&theme()),
            poodle_render::ModelCatalogueEditorHandlers {
                on_visibility_change: Some(Arc::new(
                    move |change: &ModelCatalogueVisibilityChange| {
                        change_sink
                            .lock()
                            .unwrap()
                            .push((change.id.clone(), change.visible))
                    },
                )),
                on_order_change: Some(Arc::new(move |order: &[String]| {
                    order_sink.lock().unwrap().push(order.to_vec())
                })),
                on_focus_request: Some(Arc::new(|id: &str| {
                    poodle_gpui_node_backend::request_focus(id);
                })),
                ..poodle_render::ModelCatalogueEditorHandlers::default()
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // Keyboard activation, not pointer: the mount box clips hit testing
        // to its own 160x60 content mask, and a three-row editor is taller
        // than that. Enter reaches the button through the real focus chain.
        let hide = "model-catalogue-editor:model-beta:hide";
        driver.wait_for_focus_handle(hide);
        driver.keyboard_activate(hide);
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            [("model-beta".to_string(), false)]
        );
        assert!(
            orders.lock().unwrap().is_empty(),
            "hiding never reorders the catalogue"
        );

        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("model-catalogue-editor:model-gamma:handle"),
            Some(true),
            "focus follows to the next shown model"
        );
    });
}

/// The setup's configure heading and selected option are real focus
/// destinations. The mounted host applies each controlled stage request before
/// the next paint, so both focus moves must come from the component request —
/// never from a test-side focus shortcut.
#[test]
fn model_connection_setup_stage_focus_lands_on_real_handles() {
    use poodle_headless::model_connection::{
        model_connection_picker_fixtures, ModelConnectionSetupStage,
    };
    use poodle_render::{
        model_connection_setup_action_id, model_connection_setup_title_focus_id,
    };
    use poodle_specs::ModelConnectionSetupSpec;

    run_headless(|cx| {
        fn build(
            stage: ModelConnectionSetupStage,
            mounted: Arc<Mutex<Node>>,
            requested: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let stage_mount = Arc::clone(&mounted);
            let stage_requests = Arc::clone(&requested);
            let mut node = poodle_render::model_connection_setup(
                &ModelConnectionSetupSpec::new()
                    .with_options(model_connection_picker_fixtures())
                    .with_stage(stage)
                    .with_value(Some("openai-responses".to_string())),
                &RenderContext::new(&theme()),
                poodle_render::ModelConnectionSetupHandlers {
                    on_stage_change: Some(Arc::new(move |next| {
                        let next_node = build(
                            next,
                            Arc::clone(&stage_mount),
                            Arc::clone(&stage_requests),
                        );
                        *stage_mount.lock().unwrap() = next_node;
                    })),
                    on_focus_request: Some(Arc::new(move |id: &str| {
                        requested.lock().unwrap().push(id.to_string());
                        poodle_gpui_node_backend::request_focus(id);
                    })),
                    instance_id: Some("mounted".to_string()),
                    ..poodle_render::ModelConnectionSetupHandlers::default()
                },
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let requested = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            ModelConnectionSetupStage::Choose,
            Arc::clone(&mounted),
            Arc::clone(&requested),
        );

        let heading = model_connection_setup_title_focus_id(Some("mounted"));
        let continue_id = model_connection_setup_action_id(Some("mounted"), "continue");
        let back_id = model_connection_setup_action_id(Some("mounted"), "back");

        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(&continue_id);
        driver.keyboard_activate(&continue_id);
        assert_eq!(requested.lock().unwrap().as_slice(), [heading.clone()]);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&heading),
            Some(true),
            "the heading actually receives the focus it was sent"
        );

        // configure → choose: the host applies the stage request inside the
        // callback, before the driver's post-activation paint. The selected
        // option therefore exists in time to consume the queued focus request.
        driver.wait_for_focus_handle(&back_id);
        driver.keyboard_activate(&back_id);
        let back_target = requested
            .lock()
            .unwrap()
            .last()
            .cloned()
            .expect("Back names a destination");
        driver.wait_for_focus_handle(&back_target);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&back_target),
            Some(true),
            "Back's request restores real focus after the host applies choose"
        );
    });
}

/// Hiding the sole shown model moves real backend focus onto the
/// hidden-section disclosure — the `Collapsible`'s own focusable trigger, not
/// the outer region it returns.
#[test]
fn model_catalogue_editor_hiding_the_last_row_focuses_the_hidden_disclosure() {
    use poodle_headless::model_connection::ModelCatalogueItem;
    use poodle_render::model_catalogue_hidden_focus_id;
    use poodle_specs::ModelCatalogueEditorSpec;

    run_headless(|cx| {
        let disclosed = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&disclosed);
        let items = vec![
            ModelCatalogueItem::new("model-solo", "Solo"),
            ModelCatalogueItem::new("model-gone", "Gone").with_visible(false),
        ];
        let mut node = poodle_render::model_catalogue_editor(
            &ModelCatalogueEditorSpec::new().with_items(items),
            &RenderContext::new(&theme()),
            poodle_render::ModelCatalogueEditorHandlers {
                on_visibility_change: Some(Arc::new(|_| {})),
                on_hidden_open_change: Some(Arc::new(move |open| {
                    sink.lock().unwrap().push(open)
                })),
                on_focus_request: Some(Arc::new(|id: &str| {
                    poodle_gpui_node_backend::request_focus(id);
                })),
                instance_id: Some("mounted".to_string()),
                ..poodle_render::ModelCatalogueEditorHandlers::default()
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        let hide = "model-catalogue-editor:mounted:model-solo:hide";
        let hidden = model_catalogue_hidden_focus_id(Some("mounted"));
        driver.wait_for_focus_handle(hide);
        driver.keyboard_activate(hide);
        driver.draw_frame();

        assert_eq!(disclosed.lock().unwrap().as_slice(), [true]);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&hidden),
            Some(true),
            "the hidden-section disclosure actually receives the focus it was sent"
        );
    });
}

/// Two mounted pickers over the same routes keep separate backend focus
/// handles: focusing one instance's option leaves the other's alone.
#[test]
fn two_model_connection_pickers_do_not_share_backend_focus_handles() {
    use poodle_headless::model_connection::model_connection_picker_fixtures;
    use poodle_render::model_connection_option_focus_id;
    use poodle_specs::ModelConnectionPickerSpec;

    run_headless(|cx| {
        let picker = |scope: &str| {
            poodle_render::model_connection_picker(
                &ModelConnectionPickerSpec::new()
                    .with_options(model_connection_picker_fixtures()),
                &RenderContext::new(&theme()),
                poodle_render::ModelConnectionPickerHandlers {
                    instance_id: Some(scope.to_string()),
                    ..poodle_render::ModelConnectionPickerHandlers::default()
                },
            )
        };
        let mut node = Node::container()
            .child(picker("left"))
            .child(picker("right"));
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        let left = model_connection_option_focus_id(Some("left"), "openai-responses");
        let right = model_connection_option_focus_id(Some("right"), "openai-responses");
        driver.wait_for_focus_handle(&left);
        driver.wait_for_focus_handle(&right);
        driver.focus_element(&left);

        assert_eq!(poodle_gpui_node_backend::focus_state_for(&left), Some(true));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right),
            Some(false),
            "the second picker's option keeps its own handle"
        );
    });
}

// ── g15.009 Batch C regressions ───────────────────────────────────────────

/// Radio selects on activate and never unchecks itself. Group exclusivity is
/// host-owned on native; this case is the single-option control, not RadioGroup.
#[test]
fn radio_selects_on_activate_and_does_not_uncheck_itself() {
    use poodle_specs::RadioSpec;

    run_headless(|cx| {
        let selected = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&selected);
        let mut node = poodle_render::radio(
            &RadioSpec::new()
                .with_name("shipping")
                .with_value("standard")
                .with_label("Standard shipping"),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |checked| {
                sink.lock().unwrap().push(checked);
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.keyboard_activate(FIXTURE_ID);
        assert_eq!(
            selected.lock().unwrap().as_slice(),
            [true],
            "an unchecked radio selects"
        );
    });

    run_headless(|cx| {
        let selected = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&selected);
        let mut node = poodle_render::radio(
            &RadioSpec::new()
                .with_name("shipping")
                .with_value("standard")
                .with_label("Standard shipping")
                .with_checked(true),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |checked| {
                sink.lock().unwrap().push(checked);
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, node);

        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.keyboard_activate(FIXTURE_ID);
        assert!(
            selected.lock().unwrap().is_empty(),
            "an already-checked radio does not uncheck"
        );
    });
}

/// UpdateStatus's confirm path goes through the real tree: Install opens the
/// host-owned confirm dialog, and confirming emits install.
#[test]
fn update_status_confirm_then_install_through_the_real_tree() {
    use poodle_headless::update::{OfferReason, UpdateAvailabilityProjection, UpdateControllerStatus};
    use poodle_specs::UpdateStatusSpec;

    run_headless(|cx| {
        fn offer() -> UpdateAvailabilityProjection {
            UpdateAvailabilityProjection::Offer {
                version: "1.4.0".to_string(),
                reason: OfferReason::Staged,
                notes: None,
            }
        }

        fn build(
            confirm_open: bool,
            mounted: Arc<Mutex<Node>>,
            installs: Arc<Mutex<usize>>,
            confirms: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let install_sink = Arc::clone(&installs);
            let confirm_sink = Arc::clone(&confirms);
            let mut node = poodle_render::update_status(
                &UpdateStatusSpec::new()
                    .with_status(UpdateControllerStatus::Ready)
                    .with_availability(offer())
                    .with_confirm_open(confirm_open),
                &RenderContext::new(&theme()),
                poodle_render::UpdateStatusHandlers {
                    instance_id: Some("mounted".to_string()),
                    on_install: Some(Arc::new(move || {
                        *install_sink.lock().unwrap() += 1;
                    })),
                    on_confirm_open_change: Some(Arc::new(move |open| {
                        confirm_sink.lock().unwrap().push(open);
                        let next = build(
                            open,
                            Arc::clone(&mount),
                            Arc::clone(&installs),
                            Arc::clone(&confirms),
                        );
                        *mount.lock().unwrap() = next;
                    })),
                    ..poodle_render::UpdateStatusHandlers::default()
                },
            );
            if confirm_open {
                assert!(give_first_id(
                    &mut node,
                    "update-status-confirm",
                    &|n| matches!(
                        &n.kind,
                        poodle_node::NodeKind::Button { label }
                            if label == "Install and restart"
                    ) && n.id.as_deref() != Some("mounted-install"),
                ));
            }
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let installs = Arc::new(Mutex::new(0usize));
        let confirms = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            false,
            Arc::clone(&mounted),
            Arc::clone(&installs),
            Arc::clone(&confirms),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        driver.wait_for_focus_handle("mounted-install");
        driver.keyboard_activate("mounted-install");
        assert_eq!(confirms.lock().unwrap().as_slice(), [true]);
        assert_eq!(*installs.lock().unwrap(), 0, "confirm opens before install");

        driver.wait_for_focus_handle("update-status-confirm");
        driver.keyboard_activate("update-status-confirm");
        assert_eq!(confirms.lock().unwrap().as_slice(), [true, false]);
        assert_eq!(*installs.lock().unwrap(), 1);
    });
}

/// Hidden presence collapses UpdateCenter to an empty container; attention
/// plus open hosts UpdateStatus in the popover.
#[test]
fn update_center_hidden_presence_mounts_nothing_and_open_shows_status() {
    use poodle_headless::update::{
        OfferReason, UpdateAvailabilityProjection, UpdateControllerStatus, UpdatePresence,
    };
    use poodle_specs::UpdateCenterSpec;

    run_headless(|cx| {
        let opens = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&opens);
        let mut closed = poodle_render::update_center(
            &UpdateCenterSpec::new(UpdatePresence::Quiet).with_open(false),
            &RenderContext::new(&theme()),
            poodle_render::UpdateCenterHandlers {
                instance_id: Some("mounted-center".to_string()),
                on_open_change: Some(Arc::new(move |open| {
                    sink.lock().unwrap().push(open);
                })),
                ..poodle_render::UpdateCenterHandlers::default()
            },
        );
        closed.id = Some(FIXTURE_ID.to_owned());
        let closed = Arc::new(Mutex::new(closed));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&closed));

        driver.wait_for_focus_handle("mounted-center-trigger");
        assert_eq!(
            closed
                .lock()
                .unwrap()
                .find(&|node| node.id.as_deref() == Some("mounted-center-trigger"))
                .and_then(|node| node.a11y.expanded),
            Some(false),
        );
        driver.keyboard_activate("mounted-center-trigger");
        assert_eq!(opens.lock().unwrap().as_slice(), [true]);
    });

    run_headless(|cx| {
        let mut hidden = poodle_render::update_center(
            &UpdateCenterSpec::new(UpdatePresence::Hidden)
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(UpdateAvailabilityProjection::WithheldByRollout {
                    version: "2.0.0".to_string(),
                }),
            &RenderContext::new(&theme()),
            poodle_render::UpdateCenterHandlers::default(),
        );
        hidden.id = Some(FIXTURE_ID.to_owned());
        let hidden = Arc::new(Mutex::new(hidden));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&hidden));
        driver.draw_frame();
        let hidden = hidden.lock().unwrap();
        assert!(hidden.texts().is_empty(), "hidden presence paints nothing");
    });

    run_headless(|cx| {
        let mut open = poodle_render::update_center(
            &UpdateCenterSpec::new(UpdatePresence::Attention)
                .with_status(UpdateControllerStatus::Ready)
                .with_availability(UpdateAvailabilityProjection::Offer {
                    version: "1.4.0".to_string(),
                    reason: OfferReason::Staged,
                    notes: None,
                })
                .with_open(true),
            &RenderContext::new(&theme()),
            poodle_render::UpdateCenterHandlers::default(),
        );
        open.id = Some(FIXTURE_ID.to_owned());
        let open = Arc::new(Mutex::new(open));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&open));
        driver.draw_frame();
        let open = open.lock().unwrap();
        let texts = open.texts();
        assert!(
            texts.iter().any(|t| *t == "Version 1.4.0 is available"),
            "attention plus open hosts UpdateStatus; got {texts:?}"
        );
    });
}

/// SettingsShell navigation goes through the real sidebar ids, and a refused
/// close keeps the dialog open.
#[test]
fn settings_shell_navigates_and_refused_close_stays_open() {
    use poodle_specs::{SettingsShellSpec, SidebarNavGroup, SidebarNavItem};

    fn groups() -> Vec<SidebarNavGroup> {
        vec![SidebarNavGroup::new(
            "workspace",
            vec![
                SidebarNavItem::new("general", "General"),
                SidebarNavItem::new("appearance", "Appearance"),
            ],
        )
        .with_label("Workspace")]
    }

    run_headless(|cx| {
        let pages = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&pages);
        let mut node = poodle_render::settings_shell(
            &SettingsShellSpec::new()
                .with_open(true)
                .with_groups(groups())
                .with_active_page_id("general"),
            &RenderContext::new(&theme()),
            poodle_render::SettingsShellHandlers {
                on_navigate: Some(Arc::new(move |id| {
                    sink.lock().unwrap().push(id.to_string());
                })),
                ..poodle_render::SettingsShellHandlers::default()
            },
            Some(Node::text("General page")),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, node);

        driver.wait_for_focus_handle("sidebar-nav-appearance");
        driver.keyboard_activate("sidebar-nav-appearance");
        assert_eq!(
            pages.lock().unwrap().as_slice(),
            ["appearance".to_string()]
        );
    });

    run_headless(|cx| {
        let closes = Arc::new(Mutex::new(0usize));
        let opens = Arc::new(Mutex::new(Vec::new()));
        let close_sink = Arc::clone(&closes);
        let open_sink = Arc::clone(&opens);
        let mut node = poodle_render::settings_shell(
            &SettingsShellSpec::new()
                .with_open(true)
                .with_groups(groups())
                .with_active_page_id("general")
                .with_close_refused_reason("Unsaved changes on this page."),
            &RenderContext::new(&theme()),
            poodle_render::SettingsShellHandlers {
                on_request_close: Some(Arc::new(move || {
                    *close_sink.lock().unwrap() += 1;
                })),
                on_open_change: Some(Arc::new(move |open| {
                    open_sink.lock().unwrap().push(open);
                })),
                ..poodle_render::SettingsShellHandlers::default()
            },
            Some(Node::text("General page")),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle("poodle-dialog-close");
        driver.keyboard_activate("poodle-dialog-close");
        assert_eq!(*closes.lock().unwrap(), 1);
        assert!(
            opens.lock().unwrap().is_empty(),
            "refused close does not emit on_open_change(false)"
        );
        assert!(
            node.lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Unsaved changes on this page."),
            "the refused reason stays in the tree"
        );
    });
}

/// g15.040. The native ResizeHandle was drag-only: its node took no focus,
/// carried no key handler, and declared no value range, so the native page
/// could not teach what the web one teaches. This drives the REAL focused key
/// route — the handler is never called directly — and reads both halves of
/// the result: the host's pane width, and the renderer-neutral current value
/// the next node declares.
#[test]
fn a_focused_resize_handle_steps_the_pane_and_its_declared_value() {
    use poodle_render::ResizePhase;
    use poodle_specs::{Orientation, ResizeHandleSpec};

    const MIN_PX: f32 = 48.0;
    const MAX_PX: f32 = 280.0;

    run_headless(|cx| {
        // The host owns the pane, exactly as the specimen does: it applies the
        // delta, clamps to its own bounds, and supplies the next spec.
        fn build(width: f32, mounted: Arc<Mutex<Node>>, pane: Arc<Mutex<f32>>) -> Node {
            let mount = Arc::clone(&mounted);
            let state = Arc::clone(&pane);
            let gesture = Arc::new(Mutex::new(width));
            poodle_render::resize_handle(
                &ResizeHandleSpec::new("editor:sidebar")
                    .with_orientation(Orientation::Horizontal)
                    .with_aria_label("Resize horizontal")
                    .with_aria_value_now(width)
                    .with_aria_value_min(MIN_PX)
                    .with_aria_value_max(MAX_PX),
                &RenderContext::new(&theme()),
                Some(Arc::new(move |phase, delta| match phase {
                    ResizePhase::Start => {
                        *gesture.lock().expect("gesture lock") =
                            *state.lock().expect("pane lock");
                    }
                    ResizePhase::Move => {
                        let mut at = gesture.lock().expect("gesture lock");
                        *at = (*at + delta).clamp(MIN_PX, MAX_PX);
                        *state.lock().expect("pane lock") = *at;
                        *mount.lock().expect("mount lock") =
                            build(*at, Arc::clone(&mount), Arc::clone(&state));
                    }
                    ResizePhase::End => {}
                })),
            )
        }

        let pane = Arc::new(Mutex::new(120.0f32));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(120.0, Arc::clone(&mounted), Arc::clone(&pane));

        // The host derives the key from the scope it supplied — no orientation,
        // name, or value in it, so a relabelled handle keeps its focus handle.
        let handle_id = poodle_render::resize_handle_focus_id(&ResizeHandleSpec::new(
            "editor:sidebar",
        ));

        let declared_value = || mounted.lock().unwrap().a11y.value;
        let declared_range = || {
            let node = mounted.lock().unwrap();
            (node.a11y.value_min, node.a11y.value_max)
        };

        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(&handle_id);
        driver.focus_element(&handle_id);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&handle_id),
            Some(true),
            "the separator is a real focus target, not a node with a handler nobody can reach",
        );
        assert_eq!(declared_range(), (Some(48.0), Some(280.0)));

        // An axis arrow: contract §6's 8px step, through the focus chain.
        driver.dispatch_key_raw("right");
        assert_eq!(*pane.lock().unwrap(), 128.0);
        assert_eq!(declared_value(), Some(128.0));

        // A cross-axis arrow belongs to whatever owns the surface.
        driver.dispatch_key_raw("up");
        assert_eq!(*pane.lock().unwrap(), 128.0);

        driver.dispatch_key_raw("left");
        assert_eq!(*pane.lock().unwrap(), 120.0);

        // Home and End saturate; the host's clamp decides where they land.
        driver.dispatch_key_raw("home");
        assert_eq!(*pane.lock().unwrap(), MIN_PX);
        assert_eq!(declared_value(), Some(48.0));

        driver.dispatch_key_raw("end");
        assert_eq!(*pane.lock().unwrap(), MAX_PX);
        assert_eq!(declared_value(), Some(280.0));
        assert_eq!(
            declared_range(),
            (Some(48.0), Some(280.0)),
            "the range survives every rebuild",
        );
    });
}

/// g15.040. The disabled section of the same page must stay out of the focus
/// order entirely — a disabled separator that still answers keys is worse
/// than one that never moved.
#[test]
fn a_disabled_resize_handle_takes_no_focus_and_answers_no_key() {
    use poodle_specs::{Orientation, ResizeHandleSpec};

    run_headless(|cx| {
        let spec = ResizeHandleSpec::new("editor:sidebar")
            .with_orientation(Orientation::Horizontal)
            .with_disabled(true)
            .with_aria_label("Disabled resize");
        let moves = Arc::new(Mutex::new(0usize));
        let sink = Arc::clone(&moves);
        let node = poodle_render::resize_handle(
            &spec,
            &RenderContext::new(&theme()),
            Some(Arc::new(move |_phase, _delta| {
                *sink.lock().expect("count lock") += 1;
            })),
        );
        let handle_id = poodle_render::resize_handle_focus_id(&spec);
        assert_eq!(node.runtime_id.as_deref(), Some(handle_id.as_str()));

        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.focus_element(&handle_id);
        driver.dispatch_key_raw("right");

        assert!(
            poodle_gpui_node_backend::focus_handle_for(&handle_id).is_none(),
            "a disabled separator never becomes a focus target",
        );
        assert_eq!(*moves.lock().unwrap(), 0);
    });
}

/// g15.040 review. Two ordinary `SplitView`s on one page compose two dividers.
/// While the handle keyed itself on orientation and accessible name, both
/// derived the same key and resolved ONE backend focus handle: focusing one
/// divider focused the other, and keys landed on whichever painted last. Each
/// split now states its own scope and derives the divider's from it.
#[test]
fn two_composed_split_views_do_not_share_a_divider_focus_handle() {
    use poodle_specs::{ResizeHandleSpec, SplitOrientation, SplitViewSpec};

    run_headless(|cx| {
        // Same orientation, same (absent) label, same ratio — everything a
        // derived key could see is identical.
        let left = SplitViewSpec::new("workspace:left", SplitOrientation::Horizontal);
        let right = SplitViewSpec::new("workspace:right", SplitOrientation::Horizontal);
        let divider_id = |spec: &SplitViewSpec| {
            poodle_render::resize_handle_focus_id(&ResizeHandleSpec::new(
                spec.divider_instance_id(),
            ))
        };
        let (left_id, right_id) = (divider_id(&left), divider_id(&right));
        assert_ne!(left_id, right_id);

        let build = |spec: &SplitViewSpec| {
            poodle_render::split_view(
                spec,
                &RenderContext::new(&theme()),
                Some(Node::text("primary")),
                Some(Node::text("secondary")),
                poodle_render::SplitViewHandlers {
                    on_resize: Some(Arc::new(|_phase, _delta| {})),
                    ..poodle_render::SplitViewHandlers::default()
                },
            )
        };
        let tree = Node::container().child(build(&left)).child(build(&right));

        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(tree)));
        driver.wait_for_focus_handle(&left_id);
        driver.wait_for_focus_handle(&right_id);

        driver.focus_element(&left_id);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&left_id),
            Some(true),
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right_id),
            Some(false),
            "the other split's divider is a different control and stays blurred",
        );

        driver.focus_element(&right_id);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right_id),
            Some(true),
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&left_id),
            Some(false),
        );
    });
}

// ── g15.010 Batch A regressions ───────────────────────────────────────────

/// Callout dismiss is a focusable button. Keyboard activation reaches the
/// host, which stores dismissed state and supplies the next spec.
#[test]
fn callout_dismiss_rebuilds_the_host_spec_through_mounted_input() {
    use poodle_specs::CallOutSpec;

    run_headless(|cx| {
        fn build(dismissed: bool, mounted: Arc<Mutex<Node>>, flag: Arc<Mutex<bool>>) -> Node {
            if dismissed {
                return Node::text("Dismissed");
            }
            let mount = Arc::clone(&mounted);
            let flag = Arc::clone(&flag);
            poodle_render::callout(
                &CallOutSpec::new()
                    .with_title("Dismissible callout")
                    .with_content("This callout can be dismissed by the user.")
                    .dismissible(true),
                &RenderContext::new(&theme()),
                poodle_render::CalloutHandlers {
                    on_dismiss: Some(Arc::new(move || {
                        *flag.lock().unwrap() = true;
                        *mount.lock().unwrap() =
                            build(true, Arc::clone(&mount), Arc::clone(&flag));
                    })),
                    ..poodle_render::CalloutHandlers::default()
                },
            )
        }

        let dismissed = Arc::new(Mutex::new(false));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, Arc::clone(&mounted), Arc::clone(&dismissed));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        driver.wait_for_focus_handle("poodle-callout-dismiss");
        driver.keyboard_activate("poodle-callout-dismiss");
        assert!(*dismissed.lock().unwrap(), "dismiss reached the host");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Dismissed"),
            "the next spec reflects dismissed host state"
        );
    });
}

/// RemediationBanner action and dismiss both travel through mounted input.
/// The host stores the requested action id, then applies dismiss by omitting
/// the banner from the next spec.
#[test]
fn remediation_banner_action_and_dismiss_rebuild_the_host_spec() {
    use poodle_specs::{ButtonVariant, RemediationAction, RemediationBannerSpec, StatusTone};

    run_headless(|cx| {
        fn build(
            dismissed: bool,
            last_action: Option<String>,
            mounted: Arc<Mutex<Node>>,
            actions: Arc<Mutex<Vec<String>>>,
            flag: Arc<Mutex<bool>>,
        ) -> Node {
            if dismissed {
                let mut root = Node::container().child(Node::text("Dismissed"));
                if let Some(action) = last_action {
                    root = root.child(Node::text(format!("Last request: {action}")));
                }
                return root;
            }
            let mount = Arc::clone(&mounted);
            let action_sink = Arc::clone(&actions);
            let flag = Arc::clone(&flag);
            let mut node = poodle_render::remediation_banner(
                &RemediationBannerSpec::new(
                    "We could not save your changes",
                    "Your edits are still local. Retry the save or inspect the error details.",
                )
                .with_tone(StatusTone::Danger)
                .with_primary_action(
                    RemediationAction::new("retry", "Try again")
                        .with_variant(ButtonVariant::Primary),
                )
                .with_dismissible(true),
                &RenderContext::new(&theme()),
                poodle_render::RemediationBannerHandlers {
                    on_action: Some(Arc::new({
                        let mount = Arc::clone(&mount);
                        let action_sink = Arc::clone(&action_sink);
                        let flag = Arc::clone(&flag);
                        move |id| {
                            action_sink.lock().unwrap().push(id.to_string());
                            *mount.lock().unwrap() = build(
                                false,
                                Some(id.to_string()),
                                Arc::clone(&mount),
                                Arc::clone(&action_sink),
                                Arc::clone(&flag),
                            );
                        }
                    })),
                    on_dismiss: Some(Arc::new(move || {
                        *flag.lock().unwrap() = true;
                        let last = action_sink.lock().unwrap().last().cloned();
                        *mount.lock().unwrap() = build(
                            true,
                            last,
                            Arc::clone(&mount),
                            Arc::clone(&action_sink),
                            Arc::clone(&flag),
                        );
                    })),
                    ..poodle_render::RemediationBannerHandlers::default()
                },
            );
            if let Some(action) = last_action {
                node = Node::container()
                    .child(node)
                    .child(Node::text(format!("Last request: {action}")));
            }
            node
        }

        let actions = Arc::new(Mutex::new(Vec::new()));
        let dismissed = Arc::new(Mutex::new(false));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            false,
            None,
            Arc::clone(&mounted),
            Arc::clone(&actions),
            Arc::clone(&dismissed),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        driver.wait_for_focus_handle("remediation-action-retry");
        driver.keyboard_activate("remediation-action-retry");
        assert_eq!(actions.lock().unwrap().as_slice(), ["retry".to_string()]);
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Last request: retry"),
            "action id is stored on the host and painted into the next spec"
        );

        driver.wait_for_focus_handle("remediation-banner-dismiss");
        driver.keyboard_activate("remediation-banner-dismiss");
        assert!(*dismissed.lock().unwrap(), "dismiss reached the host");
        let texts: Vec<String> = mounted
            .lock()
            .unwrap()
            .texts()
            .into_iter()
            .map(str::to_string)
            .collect();
        assert!(
            texts.iter().any(|t| t == "Dismissed"),
            "dismissed host state omits the banner"
        );
        assert!(
            texts.iter().any(|t| t == "Last request: retry"),
            "the stored action survives dismiss"
        );
    });
}

// ── g15.010 Batch B regressions ───────────────────────────────────────────

/// ActionDiscoveryPanel selection travels through mounted keyboard input.
/// The host stores the chosen action id and supplies it on the next spec.
#[test]
fn action_discovery_selection_rebuilds_the_host_spec_through_mounted_input() {
    use poodle_specs::{ActionDiscoveryPanelSpec, ActionDiscoverySection, CommandActionItem};

    run_headless(|cx| {
        fn build(active: String, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = ActionDiscoveryPanelSpec::new(vec![ActionDiscoverySection::new(
                "file",
                "File",
                vec![
                    CommandActionItem::new("save", "Save"),
                    CommandActionItem::new("open-file", "Open File"),
                ],
            )])
            .with_active_id(&active);
            let mount = Arc::clone(&mounted);
            let panel = poodle_render::action_discovery_panel(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::ActionDiscoveryPanelHandlers {
                    on_select: Some(Arc::new(move |id| {
                        *mount.lock().unwrap() = build(id.to_string(), Arc::clone(&mount));
                    })),
                    ..poodle_render::ActionDiscoveryPanelHandlers::default()
                },
            );
            Node::container()
                .child(panel)
                .child(Node::text(format!("Active: {active}")))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build("save".to_string(), Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        driver.wait_for_focus_handle("open-file");
        driver.keyboard_activate("open-file");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Active: open-file"),
            "the next spec reflects the host-owned active action"
        );
    });
}

/// DockRegion tab selection and collapse both travel through mounted input.
/// The host stores the chosen tab and the collapsed flag, then paints them.
#[test]
fn dock_region_tab_and_collapse_rebuild_the_host_spec_through_mounted_input() {
    use poodle_specs::{DockEdge, DockRegionSpec, PanelTabItem};

    run_headless(|cx| {
        fn build(tab: String, collapsed: bool, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = DockRegionSpec::new(
                DockEdge::Left,
                vec![
                    PanelTabItem::new("explorer", "Explorer"),
                    PanelTabItem::new("search", "Search"),
                ],
            )
            .with_collapsible(true)
            .with_collapsed(collapsed)
            .with_value(&tab);
            let tab_mount = Arc::clone(&mounted);
            let collapse_mount = Arc::clone(&mounted);
            let tab_for_collapse = tab.clone();
            let collapsed_for_tab = collapsed;
            let dock = poodle_render::dock_region(
                &spec,
                &RenderContext::new(&theme()),
                Some(Node::text(format!("Panel: {tab}"))),
                poodle_render::DockRegionHandlers {
                    on_tab_change: Some(Arc::new(move |value| {
                        *tab_mount.lock().unwrap() = build(
                            value.to_string(),
                            collapsed_for_tab,
                            Arc::clone(&tab_mount),
                        );
                    })),
                    on_collapse_toggle: Some(Arc::new(move |next| {
                        *collapse_mount.lock().unwrap() = build(
                            tab_for_collapse.clone(),
                            next,
                            Arc::clone(&collapse_mount),
                        );
                    })),
                    ..poodle_render::DockRegionHandlers::default()
                },
            );
            Node::container()
                .child(dock)
                .child(Node::text(format!("Tab: {tab}")))
                .child(Node::text(if collapsed {
                    "Collapsed"
                } else {
                    "Expanded"
                }))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build("explorer".to_string(), false, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        driver.wait_for_focus_handle("dock-tab-search");
        driver.keyboard_activate("dock-tab-search");
        let after_tab: Vec<String> = mounted
            .lock()
            .unwrap()
            .texts()
            .into_iter()
            .map(str::to_string)
            .collect();
        assert!(
            after_tab.iter().any(|t| t == "Tab: search"),
            "tab change reached the host and painted the next spec"
        );
        assert!(
            after_tab.iter().any(|t| t == "Expanded"),
            "tab change leaves the dock expanded"
        );

        driver.wait_for_focus_handle("dock-collapse");
        driver.keyboard_activate("dock-collapse");
        let after_collapse: Vec<String> = mounted
            .lock()
            .unwrap()
            .texts()
            .into_iter()
            .map(str::to_string)
            .collect();
        assert!(
            after_collapse.iter().any(|t| t == "Collapsed"),
            "collapse reached the host and painted the next spec"
        );
        assert!(
            after_collapse.iter().any(|t| t == "Tab: search"),
            "the stored tab survives collapse"
        );
    });
}

/// Hovered-tab `accept_panel` and static-stack before/after insert run through
/// mounted pointer dispatch, not a direct handler poke. Top docks resolve X;
/// side docks resolve Y, matching their stack axis.
#[test]
fn dock_region_hovered_tab_policy_and_static_insert_run_through_mounted_input() {
    use poodle_render::DockPanelDrop;
    use poodle_specs::{DockEdge, DockRegionSpec, DockSizing, PanelTabItem};

    fn sized(id: &str, width: f32, height: f32, child: Node) -> Node {
        let mut wrap = Node::container();
        wrap.id = Some(id.to_owned());
        wrap.style.descriptor.layout.width = LayoutSizing::Fixed(width);
        wrap.style.descriptor.layout.height = LayoutSizing::Fixed(height);
        wrap.child(child)
    }

    run_headless(|cx| {
        let refused = Arc::new(Mutex::new(Vec::new()));
        let left = DockRegionSpec::new(
            DockEdge::Left,
            vec![PanelTabItem::new("explorer", "Explorer")],
        )
        .with_can_accept_panel(true)
        .with_drag_zone_id("zone-a")
        .with_value("explorer");
        let right = DockRegionSpec::new(
            DockEdge::Left,
            vec![PanelTabItem::new("outline", "Outline")],
        )
        .with_can_accept_panel(true)
        .with_drag_zone_id("zone-b")
        .with_value("outline");

        let left_dock = poodle_render::dock_region(
            &left,
            &RenderContext::new(&theme()),
            None,
            poodle_render::DockRegionHandlers {
                on_panel_drop: {
                    let refused = Arc::clone(&refused);
                    Some(Arc::new(move |drop: &DockPanelDrop| {
                        refused.lock().unwrap().push(drop.clone());
                    }))
                },
                accept_panel: Some(Arc::new(|panel_id: &str, _edge: &str| {
                    panel_id != "explorer"
                })),
                ..poodle_render::DockRegionHandlers::default()
            },
        );
        let right_dock = poodle_render::dock_region(
            &right,
            &RenderContext::new(&theme()),
            None,
            poodle_render::DockRegionHandlers {
                on_panel_drop: {
                    let refused = Arc::clone(&refused);
                    Some(Arc::new(move |drop: &DockPanelDrop| {
                        refused.lock().unwrap().push(drop.clone());
                    }))
                },
                accept_panel: Some(Arc::new(|panel_id: &str, _edge: &str| {
                    panel_id != "explorer"
                })),
                ..poodle_render::DockRegionHandlers::default()
            },
        );
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(400.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(160.0);
        let node = Arc::new(Mutex::new(
            row.child(sized("dock-a", 200.0, 160.0, left_dock))
                .child(sized("dock-b", 200.0, 160.0, right_dock)),
        ));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 400.0, 160.0);
        driver.draw_frame();

        let source = payload_frac("dock-tab-explorer", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 8.0), source.y));
        driver.pointer_drag(payload_frac("dock-tab-outline", 0.5, 0.5));
        driver.pointer_release(payload_frac("dock-tab-outline", 0.5, 0.5));
        assert!(
            refused.lock().unwrap().is_empty(),
            "accept_panel must refuse explorer on the hovered tab"
        );
    });

    run_headless(|cx| {
        let moved = Arc::new(Mutex::new(Vec::new()));
        let source_spec = DockRegionSpec::new(
            DockEdge::Top,
            vec![PanelTabItem::new("explorer", "Explorer")],
        )
        .with_sizing(DockSizing::Static)
        .with_can_accept_panel(true)
        .with_drag_zone_id("stack-a");
        let target_spec = DockRegionSpec::new(
            DockEdge::Top,
            vec![
                PanelTabItem::new("outline", "Outline"),
                PanelTabItem::new("inspector", "Inspector"),
            ],
        )
        .with_sizing(DockSizing::Static)
        .with_can_accept_panel(true)
        .with_drag_zone_id("stack-b");

        let source_dock = poodle_render::dock_region(
            &source_spec,
            &RenderContext::new(&theme()),
            None,
            poodle_render::DockRegionHandlers::default(),
        );
        assert_eq!(
            source_dock.style.descriptor.layout.direction,
            LayoutDirection::Row,
            "a top static stack is a row"
        );
        let target_dock = poodle_render::dock_region(
            &target_spec,
            &RenderContext::new(&theme()),
            None,
            poodle_render::DockRegionHandlers {
                on_panel_drop: {
                    let moved = Arc::clone(&moved);
                    Some(Arc::new(move |drop: &DockPanelDrop| {
                        moved.lock().unwrap().push(drop.clone());
                    }))
                },
                ..poodle_render::DockRegionHandlers::default()
            },
        );
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(400.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(160.0);
        let node = Arc::new(Mutex::new(
            row.child(sized("stack-a-host", 200.0, 160.0, source_dock))
                .child(sized("stack-b-host", 200.0, 160.0, target_dock)),
        ));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 400.0, 160.0);
        driver.draw_frame();

        let source = payload_frac("dock-stack-explorer", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 8.0), source.y));
        driver.pointer_drag(payload_frac("dock-stack-outline", 0.25, 0.5));
        driver.pointer_release(payload_frac("dock-stack-outline", 0.25, 0.5));
        assert_eq!(
            moved.lock().unwrap().last().map(|drop| drop.index),
            Some(0),
            "left half of a top static stack item inserts before"
        );

        moved.lock().unwrap().clear();
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 8.0), source.y));
        driver.pointer_drag(payload_frac("dock-stack-outline", 0.75, 0.5));
        driver.pointer_release(payload_frac("dock-stack-outline", 0.75, 0.5));
        assert_eq!(
            moved.lock().unwrap().last().map(|drop| drop.index),
            Some(1),
            "right half of a top static stack item inserts after"
        );
    });

    run_headless(|cx| {
        let moved = Arc::new(Mutex::new(Vec::new()));
        let source_spec = DockRegionSpec::new(
            DockEdge::Left,
            vec![PanelTabItem::new("explorer", "Explorer")],
        )
        .with_sizing(DockSizing::Static)
        .with_can_accept_panel(true)
        .with_drag_zone_id("side-a");
        let target_spec = DockRegionSpec::new(
            DockEdge::Left,
            vec![
                PanelTabItem::new("outline", "Outline"),
                PanelTabItem::new("inspector", "Inspector"),
            ],
        )
        .with_sizing(DockSizing::Static)
        .with_can_accept_panel(true)
        .with_drag_zone_id("side-b");

        let source_dock = poodle_render::dock_region(
            &source_spec,
            &RenderContext::new(&theme()),
            None,
            poodle_render::DockRegionHandlers::default(),
        );
        assert_eq!(
            source_dock.style.descriptor.layout.direction,
            LayoutDirection::Column,
            "a side static stack is a column"
        );
        let target_dock = poodle_render::dock_region(
            &target_spec,
            &RenderContext::new(&theme()),
            None,
            poodle_render::DockRegionHandlers {
                on_panel_drop: {
                    let moved = Arc::clone(&moved);
                    Some(Arc::new(move |drop: &DockPanelDrop| {
                        moved.lock().unwrap().push(drop.clone());
                    }))
                },
                ..poodle_render::DockRegionHandlers::default()
            },
        );
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(400.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(200.0);
        let node = Arc::new(Mutex::new(
            row.child(sized("side-a-host", 200.0, 200.0, source_dock))
                .child(sized("side-b-host", 200.0, 200.0, target_dock)),
        ));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 400.0, 200.0);
        driver.draw_frame();

        let source = payload_frac("dock-stack-explorer", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 8.0), source.y));
        driver.pointer_drag(payload_frac("dock-stack-outline", 0.5, 0.75));
        driver.pointer_release(payload_frac("dock-stack-outline", 0.5, 0.75));
        assert_eq!(
            moved.lock().unwrap().last().map(|drop| drop.index),
            Some(1),
            "lower half of a side static stack item inserts after"
        );

        moved.lock().unwrap().clear();
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 8.0), source.y));
        driver.pointer_drag(payload_frac("dock-stack-outline", 0.5, 0.25));
        driver.pointer_release(payload_frac("dock-stack-outline", 0.5, 0.25));
        assert_eq!(
            moved.lock().unwrap().last().map(|drop| drop.index),
            Some(0),
            "upper half of a side static stack item inserts before"
        );
    });
}

// ── g15.010 Batch C regressions ───────────────────────────────────────────

/// AgentPlan accept/revise/dismiss travel through mounted keyboard input.
#[test]
fn agent_plan_decisions_rebuild_the_host_spec_through_mounted_input() {
    use poodle_headless::agent_plan::AgentPlanStatus;
    use poodle_specs::AgentPlanSpec;

    run_headless(|cx| {
        fn build(status: AgentPlanStatus, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = AgentPlanSpec::new("1. Inspect the contract.\n2. Apply the change.")
                .with_status(status);
            let accept_mount = Arc::clone(&mounted);
            let revise_mount = Arc::clone(&mounted);
            let dismiss_mount = Arc::clone(&mounted);
            let plan = poodle_render::agent_plan(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::AgentPlanHandlers {
                    on_accept: Some(Arc::new(move || {
                        *accept_mount.lock().unwrap() =
                            build(AgentPlanStatus::Accepted, Arc::clone(&accept_mount));
                    })),
                    on_revise: Some(Arc::new(move || {
                        *revise_mount.lock().unwrap() =
                            build(AgentPlanStatus::Revised, Arc::clone(&revise_mount));
                    })),
                    on_dismiss: Some(Arc::new(move || {
                        *dismiss_mount.lock().unwrap() =
                            build(AgentPlanStatus::Dismissed, Arc::clone(&dismiss_mount));
                    })),
                    ..poodle_render::AgentPlanHandlers::default()
                },
            );
            Node::container()
                .child(plan)
                .child(Node::text(format!("Decided: {}", status.as_str())))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(AgentPlanStatus::Pending, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("agent-plan-accept");
        driver.keyboard_activate("agent-plan-accept");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Decided: accepted"),
            "accept reached the host and painted the next spec"
        );

        *mounted.lock().unwrap() = build(AgentPlanStatus::Pending, Arc::clone(&mounted));
        driver.wait_for_focus_handle("agent-plan-revise");
        driver.keyboard_activate("agent-plan-revise");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Decided: revised"),
            "revise reached the host and painted the next spec"
        );

        *mounted.lock().unwrap() = build(AgentPlanStatus::Pending, Arc::clone(&mounted));
        driver.wait_for_focus_handle("agent-plan-dismiss");
        driver.keyboard_activate("agent-plan-dismiss");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Decided: dismissed"),
            "dismiss reached the host and painted the next spec"
        );
    });
}

/// AgentPlanRecord disclosure travels through mounted keyboard input.
#[test]
fn agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input() {
    use poodle_headless::agent_plan::AgentPlanStatus;
    use poodle_specs::AgentPlanRecordSpec;

    run_headless(|cx| {
        fn build(expanded: bool, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = AgentPlanRecordSpec::new(
                "## Proposed plan\n\n1. Wire the host.",
                AgentPlanStatus::Accepted,
            )
            .with_expanded(expanded);
            let mount = Arc::clone(&mounted);
            let record = poodle_render::agent_plan_record(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::AgentPlanRecordHandlers {
                    on_toggle: Some(Arc::new(move |next| {
                        *mount.lock().unwrap() = build(next, Arc::clone(&mount));
                    })),
                    instance_id: Some("mounted".to_string()),
                },
            );
            Node::container()
                .child(record)
                .child(Node::text(if expanded {
                    "Record: open"
                } else {
                    "Record: shut"
                }))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let toggle = poodle_render::agent_plan_record_toggle_focus_id(Some("mounted"));
        driver.wait_for_focus_handle(&toggle);
        driver.keyboard_activate(&toggle);
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Record: open"),
            "disclosure reached the host and painted the next spec"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&toggle),
            Some(true),
            "disclosure keeps the same backend focus handle across the rebuild"
        );
    });
}

/// Two AgentPlanRecords with the same status and no decided_at keep separate
/// backend focus handles. Activating one does not activate the other.
#[test]
fn two_agent_plan_records_do_not_share_backend_focus_handles() {
    use poodle_headless::agent_plan::AgentPlanStatus;
    use poodle_specs::AgentPlanRecordSpec;

    run_headless(|cx| {
        fn record(
            scope: &str,
            expanded: bool,
            mounted: &Arc<Mutex<Node>>,
            left_open: bool,
            right_open: bool,
        ) -> Node {
            let spec = AgentPlanRecordSpec::new(
                "## Proposed plan\n\n1. Wire the host.",
                AgentPlanStatus::Accepted,
            )
            .with_expanded(expanded);
            let mount = Arc::clone(mounted);
            let scope_owned = scope.to_string();
            poodle_render::agent_plan_record(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::AgentPlanRecordHandlers {
                    on_toggle: Some(Arc::new(move |next| {
                        let (left, right) = if scope_owned == "left" {
                            (next, right_open)
                        } else {
                            (left_open, next)
                        };
                        *mount.lock().unwrap() = build(left, right, Arc::clone(&mount));
                    })),
                    instance_id: Some(scope.to_string()),
                },
            )
        }

        fn build(left_open: bool, right_open: bool, mounted: Arc<Mutex<Node>>) -> Node {
            Node::container()
                .child(record("left", left_open, &mounted, left_open, right_open))
                .child(record("right", right_open, &mounted, left_open, right_open))
                .child(Node::text(format!(
                    "left:{} right:{}",
                    if left_open { "open" } else { "shut" },
                    if right_open { "open" } else { "shut" }
                )))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, false, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let left = poodle_render::agent_plan_record_toggle_focus_id(Some("left"));
        let right = poodle_render::agent_plan_record_toggle_focus_id(Some("right"));
        driver.wait_for_focus_handle(&left);
        driver.wait_for_focus_handle(&right);
        driver.focus_element(&left);
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&left), Some(true));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right),
            Some(false),
            "the second record keeps its own handle"
        );

        driver.keyboard_activate(&left);
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "left:open right:shut"),
            "only the focused record activates"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&left),
            Some(true),
            "the activated record retains focus after rebuild"
        );

        driver.keyboard_activate(&right);
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "left:open right:open"),
            "the second record activates independently"
        );
    });
}

/// AgentSubagent disclosure travels through mounted keyboard input.
#[test]
fn agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input() {
    use poodle_headless::agent_subagent::{AgentSubagentItem, AgentSubagentStatus};
    use poodle_specs::AgentSubagentSpec;

    run_headless(|cx| {
        fn build(expanded: bool, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = AgentSubagentSpec::new(AgentSubagentItem {
                id: "scout-running".to_string(),
                label: "Scout".to_string(),
                status: AgentSubagentStatus::Running,
                activity_line: Some("Searching".to_string()),
                summary: None,
            })
            .with_detail_lines(vec!["Matched 41 of 44 vectors".to_string()])
            .with_expanded(expanded);
            let mount = Arc::clone(&mounted);
            let node = poodle_render::agent_subagent(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::AgentSubagentHandlers {
                    on_toggle: Some(Arc::new(move |next| {
                        *mount.lock().unwrap() = build(next, Arc::clone(&mount));
                    })),
                    on_open_child: None,
                    instance_id: None,
                },
            );
            Node::container()
                .child(node)
                .child(Node::text(if expanded {
                    "Child: open"
                } else {
                    "Child: shut"
                }))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("agent-subagent-toggle-scout-running");
        driver.keyboard_activate("agent-subagent-toggle-scout-running");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Child: open"),
            "disclosure reached the host and painted the next spec"
        );
    });
}

/// ChangedFiles disclosure and file selection travel through mounted input.
#[test]
fn changed_files_disclosure_and_selection_rebuild_the_host_spec() {
    use poodle_headless::agent_transcript::ChangedFile;
    use poodle_specs::ChangedFilesSpec;

    run_headless(|cx| {
        fn build(
            expanded: bool,
            selected: Option<String>,
            mounted: Arc<Mutex<Node>>,
        ) -> Node {
            let spec = ChangedFilesSpec::new(
                "worked",
                vec![
                    ChangedFile {
                        path: "cp-api/Cargo.toml".to_string(),
                        additions: 1,
                        deletions: 0,
                        status: None,
                    },
                    ChangedFile {
                        path: "cp-docs/notes.md".to_string(),
                        additions: 1,
                        deletions: 0,
                        status: None,
                    },
                ],
            )
            .with_expanded(expanded);
            let toggle_mount = Arc::clone(&mounted);
            let select_mount = Arc::clone(&mounted);
            let expanded_for_select = expanded;
            let selected_for_toggle = selected.clone();
            let node = poodle_render::changed_files(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::ChangedFilesHandlers {
                    on_toggle: Some(Arc::new(move |_| {
                        *toggle_mount.lock().unwrap() = build(
                            !expanded_for_select,
                            selected_for_toggle.clone(),
                            Arc::clone(&toggle_mount),
                        );
                    })),
                    on_file_select: Some(Arc::new(move |path| {
                        *select_mount.lock().unwrap() = build(
                            true,
                            Some(path.to_string()),
                            Arc::clone(&select_mount),
                        );
                    })),
                    instance_id: None,
                },
            );
            let mut root = Node::container()
                .child(node)
                .child(Node::text(if expanded {
                    "Files: open"
                } else {
                    "Files: shut"
                }));
            if let Some(path) = selected {
                root = root.child(Node::text(format!("selected: {path}")));
            }
            root
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, None, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("changed-files-toggle-worked");
        driver.keyboard_activate("changed-files-toggle-worked");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Files: open"),
            "disclosure reached the host and painted the next spec"
        );

        driver.wait_for_focus_handle("changed-files-file-worked-cp-api:Cargo.toml");
        driver.keyboard_activate("changed-files-file-worked-cp-api:Cargo.toml");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "selected: cp-api/Cargo.toml"),
            "file selection reached the host and painted the next spec"
        );
    });
}

/// ToolCall output disclosure travels through mounted keyboard input.
#[test]
fn tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input() {
    use poodle_specs::ToolCallSpec;

    run_headless(|cx| {
        fn build(expanded: bool, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = ToolCallSpec::new("with-output", "Ran command")
                .with_detail("bun test")
                .with_output("272 pass\n0 fail")
                .with_expanded(expanded);
            let mount = Arc::clone(&mounted);
            let node = poodle_render::tool_call(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::ToolCallHandlers {
                    on_toggle: Some(Arc::new(move |_| {
                        *mount.lock().unwrap() = build(!expanded, Arc::clone(&mount));
                    })),
                    ..poodle_render::ToolCallHandlers::default()
                },
            );
            Node::container()
                .child(node)
                .child(Node::text(if expanded {
                    "Output: open"
                } else {
                    "Output: shut"
                }))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("with-output");
        driver.keyboard_activate("with-output");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Output: open"),
            "disclosure reached the host and painted the next spec"
        );
    });
}

/// ToolCallGroup run disclosure travels through mounted keyboard input.
#[test]
fn tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input() {
    use poodle_headless::agent_transcript::{ToolCallStatus, TranscriptToolCall};
    use poodle_specs::ToolCallGroupSpec;

    run_headless(|cx| {
        fn call(id: &str, detail: &str) -> TranscriptToolCall {
            TranscriptToolCall {
                id: id.to_string(),
                label: "Ran command".to_string(),
                detail: Some(detail.to_string()),
                status: ToolCallStatus::Success,
                icon: None,
                output: None,
            }
        }

        fn build(expanded: bool, mounted: Arc<Mutex<Node>>) -> Node {
            let spec = ToolCallGroupSpec::new("three", vec![call("a", "one"), call("b", "two"), call("c", "three")])
                .with_expanded(expanded);
            let mount = Arc::clone(&mounted);
            let node = poodle_render::tool_call_group(
                &spec,
                &RenderContext::new(&theme()),
                poodle_render::ToolCallGroupHandlers {
                    on_toggle: Some(Arc::new(move |_| {
                        *mount.lock().unwrap() = build(!expanded, Arc::clone(&mount));
                    })),
                    on_call_toggle: None,
                    instance_id: None,
                },
            );
            Node::container()
                .child(node)
                .child(Node::text(if expanded {
                    "Run: open"
                } else {
                    "Run: shut"
                }))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("tool-call-group-toggle-three");
        driver.keyboard_activate("tool-call-group-toggle-three");
        assert!(
            mounted
                .lock()
                .unwrap()
                .texts()
                .iter()
                .any(|t| *t == "Run: open"),
            "run disclosure reached the host and painted the next spec"
        );
    });
}

// ── Specimen axis admission (g15.019) ──────────────────────────────────────
//
// The merged web census decides which axis panes a native page may show. These
// claims are why a page cannot advertise a tab it has no renderer for, and why
// a retained tab cannot strand a page on a pane that no longer exists.

const BOTH: AxisAdmission = AxisAdmission {
    sizes: true,
    densities: true,
};
const SIZES_ONLY: AxisAdmission = AxisAdmission {
    sizes: true,
    densities: false,
};
const DENSITIES_ONLY: AxisAdmission = AxisAdmission {
    sizes: false,
    densities: true,
};
const EXAMPLES_ONLY: AxisAdmission = AxisAdmission {
    sizes: false,
    densities: false,
};

#[test]
fn a_page_publishes_exactly_the_axis_tabs_it_admits() {
    assert_eq!(
        BOTH.tabs(),
        vec![
            (EXAMPLES_TAB, "Examples"),
            (SIZES_TAB, "Sizes"),
            (DENSITIES_TAB, "Densities"),
        ]
    );
    assert_eq!(
        SIZES_ONLY.tabs(),
        vec![(EXAMPLES_TAB, "Examples"), (SIZES_TAB, "Sizes")]
    );
    assert_eq!(
        DENSITIES_ONLY.tabs(),
        vec![(EXAMPLES_TAB, "Examples"), (DENSITIES_TAB, "Densities")]
    );
    assert_eq!(EXAMPLES_ONLY.tabs(), vec![(EXAMPLES_TAB, "Examples")]);
}

#[test]
fn an_admitted_tab_is_the_one_that_renders() {
    assert_eq!(BOTH.resolve_tab(Some(SIZES_TAB)), SIZES_TAB);
    assert_eq!(BOTH.resolve_tab(Some(DENSITIES_TAB)), DENSITIES_TAB);
    assert_eq!(SIZES_ONLY.resolve_tab(Some(SIZES_TAB)), SIZES_TAB);
    assert_eq!(
        DENSITIES_ONLY.resolve_tab(Some(DENSITIES_TAB)),
        DENSITIES_TAB
    );
}

#[test]
fn a_retained_tab_the_page_no_longer_admits_falls_back_to_examples() {
    // Avatar and Progress lost Densities; Tooltip lost both. A page that kept
    // the old selection must not render a blank pane.
    assert_eq!(SIZES_ONLY.resolve_tab(Some(DENSITIES_TAB)), EXAMPLES_TAB);
    assert_eq!(DENSITIES_ONLY.resolve_tab(Some(SIZES_TAB)), EXAMPLES_TAB);
    assert_eq!(EXAMPLES_ONLY.resolve_tab(Some(SIZES_TAB)), EXAMPLES_TAB);
    assert_eq!(EXAMPLES_ONLY.resolve_tab(Some(DENSITIES_TAB)), EXAMPLES_TAB);
    assert_eq!(BOTH.resolve_tab(Some("nonsense")), EXAMPLES_TAB);
    assert_eq!(BOTH.resolve_tab(None), EXAMPLES_TAB);
}

#[test]
fn axis_row_keys_are_distinct_per_step() {
    use poodle_specs::{ControlDensity, ControlSize};

    let sizes: Vec<&str> = [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ]
    .into_iter()
    .map(size_key)
    .collect();
    assert_eq!(sizes, vec!["xs", "sm", "md", "lg", "xl"]);

    let densities: Vec<&str> = [
        ControlDensity::Compact,
        ControlDensity::Default,
        ControlDensity::Comfortable,
    ]
    .into_iter()
    .map(density_key)
    .collect();
    assert_eq!(densities, vec!["compact", "default", "comfortable"]);
}

#[test]
fn empty_state_scene_carries_the_two_value_size_domain() {
    #[path = "../src/generated/specimens/specimens.rs"]
    mod fixture;

    let scene = fixture::SPECIMEN_SCENES
        .iter()
        .find(|scene| scene.id == "empty-state-specimen")
        .expect("empty-state scene");
    assert_eq!(scene.size_axis, &["default", "compact"]);
}

#[test]
fn avatar_scene_matrix_uses_fixture_first_instance_with_xs_default() {
    #[path = "../src/generated/specimens/specimens.rs"]
    mod fixture;

    let scene = fixture::SPECIMEN_SCENES
        .iter()
        .find(|scene| scene.id == "avatar-specimen")
        .expect("avatar scene");
    let first = scene
        .groups
        .first()
        .and_then(|group| group.instances.first())
        .expect("avatar first instance");
    assert_eq!(first.props.iter().find(|p| p.prop == "size").map(|p| p.value), Some("xs"));
    assert_eq!(scene.size_axis, &["xs", "sm", "md", "lg", "xl"]);
}

#[test]
fn text_and_eyebrow_native_specimens_advertise_xs_sm_md_in_order() {
    assert_eq!(TEXT_SIZES, &["xs", "sm", "md"]);
    assert_eq!(EYEBROW_SIZES, &["xs", "sm", "md"]);
}

#[test]
fn icon_size_domain_covers_all_five_control_steps_in_order() {
    use poodle_specs::{ControlSize, IconSize};

    let ordered: Vec<IconSize> = [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ]
    .into_iter()
    .map(IconSize::from)
    .collect();
    assert_eq!(
        ordered,
        [
            IconSize::Xs,
            IconSize::Sm,
            IconSize::Md,
            IconSize::Lg,
            IconSize::Xl,
        ]
    );
}

#[test]
fn empty_state_compact_and_default_render_distinct_geometry() {
    use poodle_node::{LayoutSizing, Node, NodeKind};
    use poodle_render::empty_state;
    use poodle_specs::{EmptyStateSize, EmptyStateSpec};

    fn walk<'a>(node: &'a Node, visit: &mut impl FnMut(&'a Node)) {
        visit(node);
        for child in &node.children {
            walk(child, visit);
        }
    }

    fn title_text_size(node: &Node) -> Option<f32> {
        let mut found = None;
        walk(node, &mut |candidate| {
            if matches!(candidate.kind, NodeKind::Text { .. })
                && candidate.style.text_size.is_some()
                && candidate.style.text_weight == Some(600)
            {
                found = candidate.style.text_size;
            }
        });
        found
    }

    fn icon_container_side(node: &Node) -> Option<f32> {
        let mut found = None;
        walk(node, &mut |candidate| {
            if !matches!(candidate.kind, NodeKind::Container) {
                return;
            }
            let LayoutSizing::Fixed(width) = candidate.style.descriptor.layout.width else {
                return;
            };
            if candidate
                .children
                .iter()
                .any(|child| matches!(child.kind, NodeKind::Icon { .. }))
            {
                found = Some(width);
            }
        });
        found
    }

    let theme = theme();
    let ctx = RenderContext::new(&theme);
    let default = empty_state(&EmptyStateSpec::new("No projects yet"), &ctx);
    let compact = empty_state(
        &EmptyStateSpec::new("No projects yet").with_size(EmptyStateSize::Compact),
        &ctx,
    );

    let default_title = title_text_size(&default).expect("default title");
    let compact_title = title_text_size(&compact).expect("compact title");
    assert!(compact_title < default_title);

    let default_icon = icon_container_side(&default).expect("default icon box");
    let compact_icon = icon_container_side(&compact).expect("compact icon box");
    assert!(compact_icon < default_icon);
}

// ── g15.042 Stepper native interaction ────────────────────────────────────

/// Give each mounted Stepper control a stable id keyed by its step value.
///
/// Bounds are only recorded for identified elements, so this is how the driver
/// addresses the real trigger and the real rerun button instead of calling a
/// closure. The shape is the renderer's: one list item per step, trigger
/// first, rerun — where the contract permits one — second.
fn identify_stepper(root: &mut Node, values: &[&str]) {
    let mut cells = root
        .children
        .iter_mut()
        .filter(|cell| cell.a11y.role == Some(poodle_node::NodeRole::ListItem));
    for value in values {
        let cell = cells.next().expect("one list item per step");
        cell.children[0].id = Some(format!("stepper-trigger-{value}"));
        if let Some(rerun) = cell.children.get_mut(1) {
            rerun.id = Some(format!("stepper-rerun-{value}"));
        }
    }
}

/// g15.042: GPUI wired `on_collapsed_change` alone, so the specimen painted
/// selectable steps and rerun buttons that did nothing.
///
/// Selection and re-run are separate controls because re-running a finished
/// step spends whatever that step costs (`stepper.md` §2), so this drives both
/// through the real mounted tree and checks that neither one stands in for the
/// other. Only a mounted window can prove it: the rerun sits *inside* the
/// clickable step, so an unwired one would let the press bubble into
/// selection, and gpui's own dispatch is what decides.
///
/// Keyboard coverage here is activation of an already-focused control. GPUI's
/// Stepper declares no focus treatment, so nothing registers a focus handle
/// and focus can only arrive by pointer — an open gap with its own row in the
/// g15 release-gap register.
#[test]
fn stepper_selection_and_rerun_reach_separate_mounted_controls() {
    use poodle_specs::{StepStatus, StepperSpec, StepperStep};

    run_headless(|cx| {
        let changes = Arc::new(Mutex::new(Vec::new()));
        let reruns = Arc::new(Mutex::new(Vec::new()));
        let change_sink = Arc::clone(&changes);
        let rerun_sink = Arc::clone(&reruns);

        let mut node = poodle_render::stepper(
            &StepperSpec::new(vec![
                StepperStep::new("read", "Read").with_status(StepStatus::Complete),
                StepperStep::new("apply", "Apply").with_disabled(true),
            ])
            .with_value("apply")
            .with_show_rerun(true),
            &RenderContext::new(&theme()),
            poodle_render::StepperHandlers {
                on_change: Some(Arc::new(move |value: &str| {
                    change_sink.lock().unwrap().push(value.to_string())
                })),
                on_rerun: Some(Arc::new(move |value: &str| {
                    rerun_sink.lock().unwrap().push(value.to_string())
                })),
                on_collapsed_change: None,
            },
        );
        identify_stepper(&mut node, &["read", "apply"]);
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // Pointer: the trigger navigates and does nothing else.
        driver.pointer_activate_id("stepper-trigger-read");
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read"],
            "an enabled trigger emits its own value exactly once",
        );
        assert!(
            reruns.lock().unwrap().is_empty(),
            "selecting a completed step must not re-run it",
        );

        // Keyboard: the press left focus on that same trigger, so Enter walks
        // the real focus chain to the control the pointer just used. This is
        // activation, not entry — see the note above.
        driver.dispatch_key_raw("enter");
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read", "read"],
            "keyboard activation reaches the same mounted trigger",
        );
        assert!(reruns.lock().unwrap().is_empty());

        // Pointer: the rerun control is a different node with a different job.
        driver.pointer_activate_id("stepper-rerun-read");
        assert_eq!(
            reruns.lock().unwrap().as_slice(),
            ["read"],
            "the rerun control emits the completed step's exact value once",
        );
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read", "read"],
            "re-running must not also navigate — the press stopped at the rerun",
        );

        driver.dispatch_key_raw("space");
        assert_eq!(
            reruns.lock().unwrap().as_slice(),
            ["read", "read"],
            "keyboard activation reaches the same mounted rerun control",
        );
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read", "read"],
            "and still does not select the step it re-ran",
        );

        // A disabled step is not a control: it takes neither the click nor the
        // focus the click would have moved.
        driver.pointer_activate_id("stepper-trigger-apply");
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read", "read"],
            "a disabled step cannot select",
        );
        assert_eq!(reruns.lock().unwrap().len(), 2);
    });
}

/// g15.042: collapse is the third action and stays its own. It folds the
/// vertical track, carries the new state, and never selects or re-runs.
#[test]
fn stepper_collapse_stays_independent_in_a_mounted_window() {
    use poodle_node::NodeRole;
    use poodle_specs::{Orientation, StepStatus, StepperSpec, StepperStep};

    const SUMMARY: &str = "poodle-stepper-summary";

    run_headless(|cx| {
        let changes = Arc::new(Mutex::new(Vec::new()));
        let reruns = Arc::new(Mutex::new(Vec::new()));
        let collapses = Arc::new(Mutex::new(Vec::new()));

        let build = |collapsed: bool| {
            let change_sink = Arc::clone(&changes);
            let rerun_sink = Arc::clone(&reruns);
            let collapse_sink = Arc::clone(&collapses);
            let mut node = poodle_render::stepper(
                &StepperSpec::new(vec![
                    StepperStep::new("read", "Read").with_status(StepStatus::Complete),
                    StepperStep::new("apply", "Apply"),
                ])
                .with_orientation(Orientation::Vertical)
                .with_collapsible(true)
                .with_collapsed(collapsed)
                .with_show_rerun(true)
                .with_value("apply"),
                &RenderContext::new(&theme()),
                poodle_render::StepperHandlers {
                    on_change: Some(Arc::new(move |value: &str| {
                        change_sink.lock().unwrap().push(value.to_string())
                    })),
                    on_rerun: Some(Arc::new(move |value: &str| {
                        rerun_sink.lock().unwrap().push(value.to_string())
                    })),
                    on_collapsed_change: Some(Arc::new(move |next: bool| {
                        collapse_sink.lock().unwrap().push(next)
                    })),
                },
            );
            node.id = Some(FIXTURE_ID.to_owned());
            Arc::new(Mutex::new(node))
        };

        let collapsed = build(true);
        assert!(
            !collapsed
                .lock()
                .unwrap()
                .children
                .iter()
                .any(|child| child.a11y.role == Some(NodeRole::ListItem)),
            "collapsed omits the step rows rather than hiding them",
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&collapsed));

        driver.pointer_activate_id(SUMMARY);
        assert_eq!(
            collapses.lock().unwrap().as_slice(),
            [false],
            "the summary carries the state it is moving to",
        );

        // The host owns the state, so the expanded tree is a fresh mount. The
        // same control now asks to fold, and the keyboard reaches it too.
        driver.mount_node(build(false));
        driver.pointer_activate_id(SUMMARY);
        driver.dispatch_key_raw("enter");
        assert_eq!(
            collapses.lock().unwrap().as_slice(),
            [false, true, true],
            "expanded, the summary asks to collapse — by pointer, and by key \
             once the pointer has focused it",
        );

        assert!(
            changes.lock().unwrap().is_empty() && reruns.lock().unwrap().is_empty(),
            "folding the track selects nothing and re-runs nothing",
        );
    });
}

// ── g15.052 native focus ring ───────────────────────────────────────────
//
// The reusable node channel (`NodeStyle::focus_ring`) and its GPUI
// projection: the backend paints the declared ring only while the node's real
// focus handle holds focus, outside layout and without touching the resting
// border. Component adoption (Button, Stepper) is proven separately; these
// are the bordered and borderless proof nodes the channel was built against.

/// A fixed-size proof node with a declared ring. Centered in the driver's
/// 160×60 mount box at (32, 32), a 100×40 node's border box lands at exactly
/// (62, 42), so the painted ring's outer edge is exact: the border box
/// outset by `offset + width` = 4 logical px.
fn ring_proof_node(bordered: bool) -> Node {
    let mut node = Node::container();
    node.id = Some("ring-proof".to_owned());
    node.interaction.focusable = true;
    node.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(100.0);
    node.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(40.0);
    node.style.focus_ring = Some(poodle_node::FocusRing {
        color: poodle_node::ColorValue(0.3, 0.6, 1.0, 1.0),
        width: 2.0,
        offset: 2.0,
    });
    if bordered {
        node.style.descriptor.border.width = 1.0;
        node.style.descriptor.border.color = poodle_node::ColorValue(0.5, 0.5, 0.5, 1.0);
        let radii = &mut node.style.descriptor.corner_radii;
        radii.top_left = 6.0;
        radii.top_right = 6.0;
        radii.bottom_right = 6.0;
        radii.bottom_left = 6.0;
        node.style.shadow_layers = vec![poodle_node::ShadowLayer {
            offset_x: 0.0,
            offset_y: 2.0,
            blur: 8.0,
            spread: 0.0,
            color: poodle_node::ColorValue(0.0, 0.0, 0.0, 0.2),
            inset: false,
        }];
        // A hover patch alongside the ring: gpui refines hover after focus,
        // so this is the composition that used to erase focus treatments.
        node.style.hover = Some(poodle_node::StylePatch {
            background: Some(poodle_node::ColorValue(0.2, 0.2, 0.2, 1.0)),
            ..poodle_node::StylePatch::default()
        });
    }
    node
}

/// The ring painted for `id` matches the expected outer-edge bounds exactly
/// (all proof geometry is integral logical pixels).
fn assert_ring_bounds(id: &str, expected: [f32; 4]) -> poodle_gpui_node_backend::PaintedRing {
    let painted = poodle_gpui_node_backend::painted_ring_for(id)
        .unwrap_or_else(|| panic!("a ring is painted for {id}"));
    for (got, want) in painted.bounds.iter().zip(expected.iter()) {
        assert!(
            (got - want).abs() < 0.01,
            "ring bounds for {id}: got {:?}, want {expected:?}",
            painted.bounds,
        );
    }
    painted
}

/// Bordered node: the ring draws OUTSIDE the resting 1px border — the
/// border is preserved, not widened or recoloured — only while the real
/// handle holds focus, alongside an existing shadow stack, and a hover patch
/// cannot overwrite it.
#[test]
fn a_declared_ring_paints_outside_a_bordered_node_only_while_focused() {
    run_headless(|cx| {
        let node = Arc::new(Mutex::new(ring_proof_node(true)));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for("ring-proof"),
            None,
            "nothing paints before focus arrives",
        );

        driver.wait_for_focus_handle("ring-proof");
        driver.focus_element("ring-proof");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("ring-proof"),
            Some(true),
        );
        let painted = assert_ring_bounds("ring-proof", [58.0, 38.0, 108.0, 48.0]);
        assert_eq!(painted.ring.width, 2.0);
        assert_eq!(painted.ring.offset, 2.0);
        assert_eq!(painted.ring.color, poodle_node::ColorValue(0.3, 0.6, 1.0, 1.0));

        // The resting border is still the descriptor's — the ring did not
        // become a wider replacement border.
        let node = node.lock().unwrap();
        assert_eq!(node.style.descriptor.border.width, 1.0);
        drop(node);

        // Hover applies its own patch and the ring survives it.
        driver.pointer_hover(headless_driver::mount_box_center());
        assert!(
            poodle_gpui_node_backend::painted_ring_for("ring-proof").is_some(),
            "hover must not overwrite the ring",
        );

        driver.blur_element_focus("ring-proof");
        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for("ring-proof"),
            None,
            "blur clears the ring",
        );
    });
}

/// Borderless node: the same ring projects with no resting border at all —
/// the channel's reason to exist (a `StylePatch` focus recolour has nothing
/// to recolour on a borderless control).
#[test]
fn a_borderless_node_paints_the_declared_ring_without_a_resting_border() {
    run_headless(|cx| {
        let node = ring_proof_node(false);
        assert_eq!(node.style.descriptor.border.width, 0.0);
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle("ring-proof");
        driver.focus_element("ring-proof");
        assert_ring_bounds("ring-proof", [58.0, 38.0, 108.0, 48.0]);

        driver.blur_element_focus("ring-proof");
        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for("ring-proof"),
            None,
        );
    });
}

// ── g15.052 Stepper keyboard entry ──────────────────────────────────────
//
// The retained half of the focus gap: the trigger, rerun, and summary
// controls are borderless, so no `StylePatch` focus recolour could ever give
// them a tracked focus handle — keyboard entry only worked after a pointer
// press. The declared focus ring makes the backend track a real handle per
// control, and these tests drive entry through the window's real tab-stop
// traversal with no pointer input at all.

/// Traverse the window's real tab stops until `element_id` holds focus.
/// Fails after a bounded number of hops, so a control that never enters the
/// tab order is a loud failure, not a silent pass.
fn tab_until_focused(driver: &mut HeadlessDriver, element_id: &str) {
    for _ in 0..8 {
        driver.focus_next_tab_stop();
        if poodle_gpui_node_backend::focus_state_for(element_id) == Some(true) {
            return;
        }
    }
    panic!("`{element_id}` never received focus through tab-stop traversal");
}

/// Keyboard entry reaches the trigger and the rerun control in contract
/// order (trigger, then its rerun, then the next step) without any prior
/// pointer press; `Enter`/`Space` activates the focused action; the ring
/// follows focus and clears behind it.
#[test]
fn stepper_keyboard_entry_focuses_and_activates_without_a_pointer_press() {
    use poodle_specs::{StepStatus, StepperSpec, StepperStep};

    const TRIGGER_READ: &str = "poodle-stepper:trigger:read";
    const RERUN_READ: &str = "poodle-stepper:rerun:read";
    const TRIGGER_APPLY: &str = "poodle-stepper:trigger:apply";

    run_headless(|cx| {
        let changes = Arc::new(Mutex::new(Vec::new()));
        let reruns = Arc::new(Mutex::new(Vec::new()));
        let change_sink = Arc::clone(&changes);
        let rerun_sink = Arc::clone(&reruns);

        let mut node = poodle_render::stepper(
            &StepperSpec::new(vec![
                StepperStep::new("read", "Read").with_status(StepStatus::Complete),
                StepperStep::new("apply", "Apply"),
            ])
            .with_value("apply")
            .with_show_rerun(true),
            &RenderContext::new(&theme()),
            poodle_render::StepperHandlers {
                on_change: Some(Arc::new(move |value: &str| {
                    change_sink.lock().unwrap().push(value.to_string())
                })),
                on_rerun: Some(Arc::new(move |value: &str| {
                    rerun_sink.lock().unwrap().push(value.to_string())
                })),
                on_collapsed_change: None,
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        // The declared rings give every contracted control a tracked handle —
        // before g15.052 none of these existed until a pointer press.
        driver.wait_for_focus_handle(TRIGGER_READ);
        driver.wait_for_focus_handle(RERUN_READ);
        driver.wait_for_focus_handle(TRIGGER_APPLY);
        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for(TRIGGER_READ),
            None,
            "no ring is painted before focus arrives",
        );

        // Entry: the trigger is in the window's tab order. No pointer input
        // has occurred anywhere in this test.
        tab_until_focused(&mut driver, TRIGGER_READ);
        let ring = poodle_gpui_node_backend::painted_ring_for(TRIGGER_READ)
            .expect("the focused trigger paints its ring");
        assert_eq!(ring.ring.width, 2.0);
        assert_eq!(ring.ring.offset, 2.0);

        // Activation: Enter on the focused trigger selects its step.
        driver.dispatch_key_raw("enter");
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read"],
            "keyboard activation reaches the trigger with no prior pointer press",
        );
        assert!(reruns.lock().unwrap().is_empty());

        // Contract order: the rerun control is the next stop after its
        // trigger. The ring moves with focus and clears behind it.
        tab_until_focused(&mut driver, RERUN_READ);
        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for(TRIGGER_READ),
            None,
            "the ring clears when focus leaves the trigger",
        );
        assert!(
            poodle_gpui_node_backend::painted_ring_for(RERUN_READ).is_some(),
            "the focused rerun control paints its ring",
        );

        driver.dispatch_key_raw("space");
        assert_eq!(
            reruns.lock().unwrap().as_slice(),
            ["read"],
            "Space activates the focused rerun control",
        );
        assert_eq!(
            changes.lock().unwrap().as_slice(),
            ["read"],
            "re-running still does not select the step",
        );

        // Traversal continues to the next step's trigger.
        tab_until_focused(&mut driver, TRIGGER_APPLY);

        driver.blur_element_focus(TRIGGER_APPLY);
        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for(TRIGGER_APPLY),
            None,
            "blur clears the last ring",
        );
    });
}

/// The collapsible summary is the first stop when collapsible (contract §6)
/// and paints the contracted INSET ring (-0.125rem): the row spans the track
/// edge to edge, so an outset ring would clip against it.
#[test]
fn stepper_summary_takes_keyboard_entry_and_paints_the_inset_ring() {
    use poodle_specs::{Orientation, StepStatus, StepperSpec, StepperStep};

    const SUMMARY: &str = "poodle-stepper-summary";

    run_headless(|cx| {
        let collapses = Arc::new(Mutex::new(Vec::new()));
        let collapse_sink = Arc::clone(&collapses);

        let mut node = poodle_render::stepper(
            &StepperSpec::new(vec![
                StepperStep::new("read", "Read").with_status(StepStatus::Complete),
                StepperStep::new("apply", "Apply"),
            ])
            .with_orientation(Orientation::Vertical)
            .with_collapsible(true)
            .with_collapsed(false)
            .with_value("apply"),
            &RenderContext::new(&theme()),
            poodle_render::StepperHandlers {
                on_change: None,
                on_rerun: None,
                on_collapsed_change: Some(Arc::new(move |next: bool| {
                    collapse_sink.lock().unwrap().push(next)
                })),
            },
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));

        driver.wait_for_focus_handle(SUMMARY);
        tab_until_focused(&mut driver, SUMMARY);
        let ring = poodle_gpui_node_backend::painted_ring_for(SUMMARY)
            .expect("the focused summary paints its ring");
        assert_eq!(ring.ring.width, 2.0);
        assert_eq!(ring.ring.offset, -2.0, "the summary ring is inset");

        driver.dispatch_key_raw("enter");
        assert_eq!(
            collapses.lock().unwrap().as_slice(),
            [true],
            "Enter on the focused summary toggles collapse with no pointer press",
        );
    });
}

// ── g15.052 review: registry identity, tab-stop freshness, frame lifetime ──

/// Two UNSTAMPED production Buttons — `poodle_render::button` mints no id —
/// must not share a focus registry key. Proves separate handles, sequential
/// keyboard entry in tree order, one ring at a time, and independent
/// activation, all through the real traversal with no pointer.
#[test]
fn two_unstamped_buttons_hold_independent_focus_identities() {
    run_headless(|cx| {
        let (handler_one, clicks_one) = counting_handler();
        let (handler_two, clicks_two) = counting_handler();
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let one = poodle_render::button(
            &poodle_specs::ButtonSpec::new()
                .with_label("One")
                .with_size(poodle_specs::ControlSize::Sm),
            &ctx,
            Some(handler_one),
        );
        let two = poodle_render::button(
            &poodle_specs::ButtonSpec::new()
                .with_label("Two")
                .with_size(poodle_specs::ControlSize::Sm),
            &ctx,
            Some(handler_two),
        );
        assert!(
            one.id.is_none() && one.runtime_id.is_none(),
            "the production path stamps no identity — the backend mints it",
        );
        assert!(two.id.is_none() && two.runtime_id.is_none());

        let mut row = Node::container();
        row.style.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
        row.style.descriptor.layout.spacing.gap = 8.0;
        let mut row = row.child(one).child(two);
        row.id = Some(FIXTURE_ID.to_owned());
        let node = Arc::new(Mutex::new(row));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        // The tracked handles are created in the first paint pass and attach
        // from the next build; settle both before traversing.
        driver.draw_frame();
        driver.draw_frame();

        // Keyboard entry: the first button is the first tab stop.
        driver.focus_next_tab_stop();
        let rings = poodle_gpui_node_backend::painted_rings();
        assert_eq!(rings.len(), 1, "exactly one ring is on screen");
        let (first_key, first_ring) = rings[0].clone();
        assert!(
            !first_key.is_empty(),
            "an unstamped control gets a real registry identity, not the shared empty key",
        );

        driver.dispatch_key_raw("enter");
        assert_eq!(*clicks_one.lock().unwrap(), 1);
        assert_eq!(
            *clicks_two.lock().unwrap(),
            0,
            "activation stays with the focused control",
        );

        // The next stop is the second button: the ring moves, and only one
        // control reports focused at a time.
        driver.focus_next_tab_stop();
        let rings = poodle_gpui_node_backend::painted_rings();
        assert_eq!(rings.len(), 1, "one ring at a time");
        let (second_key, second_ring) = rings[0].clone();
        assert_ne!(
            first_key, second_key,
            "two unstamped controls hold separate handles",
        );
        assert!(
            second_ring.bounds[0] > first_ring.bounds[0],
            "the ring moved to the second button: {:?} -> {:?}",
            first_ring.bounds,
            second_ring.bounds,
        );

        driver.dispatch_key_raw("space");
        assert_eq!(*clicks_two.lock().unwrap(), 1);
        assert_eq!(*clicks_one.lock().unwrap(), 1);

        driver.blur_element_focus(&second_key);
        assert!(
            poodle_gpui_node_backend::painted_rings().is_empty(),
            "blur clears the last ring",
        );
    });
}

/// A simple focusable proof node with a declared ring and a caller-chosen
/// roving tab index.
fn roving_proof_node(id: &str, tab_index: i32) -> Node {
    let mut node = Node::container();
    node.id = Some(id.to_owned());
    node.interaction.focusable = true;
    node.a11y.tab_index = Some(tab_index);
    node.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(40.0);
    node.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(20.0);
    node.style.focus_ring = Some(poodle_node::FocusRing {
        color: poodle_node::ColorValue(0.3, 0.6, 1.0, 1.0),
        width: 2.0,
        offset: 2.0,
    });
    node
}

fn roving_pair(a_tab_index: i32) -> Arc<Mutex<Node>> {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = poodle_node::LayoutDirection::Row;
    row.style.descriptor.layout.spacing.gap = 8.0;
    let mut row = row
        .child(roving_proof_node("roving-a", a_tab_index))
        .child(roving_proof_node("roving-b", 0));
    row.id = Some(FIXTURE_ID.to_owned());
    Arc::new(Mutex::new(row))
}

/// A retained handle's tab flags follow the node's CURRENT declaration, not
/// the first frame's: a roving component that moves `a11y.tab_index` 0 → -1
/// drops out of sequential traversal, and 0 again re-enters it.
#[test]
fn a_tracked_handle_follows_roving_tab_index_changes() {
    run_headless(|cx| {
        let mut driver = HeadlessDriver::new(cx, roving_pair(0));
        driver.draw_frame();
        driver.draw_frame();

        tab_until_focused(&mut driver, "roving-a");
        assert!(
            poodle_gpui_node_backend::painted_ring_for("roving-a").is_some(),
            "the first stop paints its ring",
        );

        // Rove A out of the order. Focus still sits on A's handle; the next
        // Tab must skip A and land on B.
        driver.mount_node(roving_pair(-1));
        driver.draw_frame();
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("roving-b"),
            Some(true),
            "with A at tab_index -1, traversal skips it",
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("roving-a"),
            Some(false),
        );

        // Rove A back in: traversal reaches it again — the retained handle's
        // flags were refreshed, not frozen at first paint.
        driver.mount_node(roving_pair(0));
        driver.draw_frame();
        tab_until_focused(&mut driver, "roving-a");
    });
}

/// The painted-ring registry is frame-scoped: a focused node that leaves the
/// tree paints nothing this frame, so its entry must not survive. Before the
/// frame boundary cleared the registry, the entry lived forever and
/// `painted_ring_for` could claim a ring that is no longer on screen.
#[test]
fn a_removed_focused_node_leaves_no_painted_ring() {
    run_headless(|cx| {
        let node = Arc::new(Mutex::new(ring_proof_node(true)));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        driver.wait_for_focus_handle("ring-proof");
        driver.focus_element("ring-proof");
        assert!(poodle_gpui_node_backend::painted_ring_for("ring-proof").is_some());

        let mut empty = Node::container();
        empty.id = Some(FIXTURE_ID.to_owned());
        driver.mount_node(Arc::new(Mutex::new(empty)));
        assert_eq!(
            poodle_gpui_node_backend::painted_ring_for("ring-proof"),
            None,
            "the observation cannot outlive the node that painted it",
        );
        assert!(poodle_gpui_node_backend::painted_rings().is_empty());
    });
}


// ── Inset shadow projection (g16.005) ──────────────────────────────────────
//
// crates.io `gpui::BoxShadow` has no `inset` flag, so the node backend paints
// inset layers itself as per-side bands inside the padding box. Accordion,
// ActionDiscoveryPanel, ListCard, Popover, and Tabs all depend on this; band
// arithmetic is unit-tested in the backend, and what these prove is that the
// real paint pass emits them.

const INSET_ID: &str = "inset-shadow-proof";

/// Stamp the observation id on the first node in the tree that declares an
/// inset layer. Real compositions put the highlight on an inner surface, not
/// on the composition root, and hunting for it by hand would just encode this
/// component's current shape into the test.
fn stamp_first_inset_node(node: &mut Node) -> bool {
    if node.style.shadow_layers.iter().any(|layer| layer.inset) {
        node.id = Some(INSET_ID.to_owned());
        return true;
    }
    for child in &mut node.children {
        if stamp_first_inset_node(child) {
            return true;
        }
    }
    false
}

fn inset_shadow_node(layers: Vec<poodle_node::ShadowLayer>) -> Node {
    let mut node = Node::container();
    node.id = Some(INSET_ID.to_owned());
    node.style.descriptor.layout.width = poodle_node::LayoutSizing::Fixed(120.0);
    node.style.descriptor.layout.height = poodle_node::LayoutSizing::Fixed(48.0);
    node.style.descriptor.background = Some(poodle_node::ColorValue(0.1, 0.1, 0.1, 1.0));
    node.style.shadow_layers = layers;
    node
}

fn painted_inset_bands(
    cx: &mut TestAppContext,
    layers: Vec<poodle_node::ShadowLayer>,
) -> Vec<poodle_gpui_node_backend::PaintedInsetShadow> {
    let node = Arc::new(Mutex::new(inset_shadow_node(layers)));
    let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
    driver.draw_frame();
    poodle_gpui_node_backend::painted_inset_shadows_for(INSET_ID)
}

/// The Popover and Accordion top highlight: `offset (0, 1)`, no spread. The
/// paint pass must emit a 1px band on the top edge only, clipped to the
/// element's own padding box.
#[test]
fn a_top_highlight_inset_layer_paints_a_top_edge_band() {
    run_headless(|cx| {
        let painted = painted_inset_bands(
            cx,
            vec![poodle_node::ShadowLayer {
                offset_x: 0.0,
                offset_y: 1.0,
                blur: 0.0,
                spread: 0.0,
                color: poodle_node::ColorValue(1.0, 1.0, 1.0, 0.08),
                inset: true,
            }],
        );
        assert_eq!(painted.len(), 1, "one inset layer paints one band set");
        let band = painted[0];
        assert_eq!(band.top, 1.0, "the highlight is a 1px top band");
        assert_eq!((band.left, band.right, band.bottom), (0.0, 0.0, 0.0));
        assert_eq!(band.color, poodle_node::ColorValue(1.0, 1.0, 1.0, 0.08));
        assert_eq!(
            [band.bounds[2], band.bounds[3]],
            [120.0, 48.0],
            "the band is clipped to the element's padding box"
        );
    });
}

/// The Tabs drop-target and ActionDiscoveryPanel active ring: spread only, so
/// an even band on all four sides.
#[test]
fn a_spread_inset_layer_paints_an_even_inner_ring() {
    run_headless(|cx| {
        let painted = painted_inset_bands(
            cx,
            vec![poodle_node::ShadowLayer {
                offset_x: 0.0,
                offset_y: 0.0,
                blur: 0.0,
                spread: 2.0,
                color: poodle_node::ColorValue(0.3, 0.6, 1.0, 1.0),
                inset: true,
            }],
        );
        assert_eq!(painted.len(), 1);
        let band = painted[0];
        assert_eq!(
            (band.left, band.right, band.top, band.bottom),
            (2.0, 2.0, 2.0, 2.0)
        );
    });
}

/// ListCard composes a highlight ring and an active leading bar. Both must
/// paint, in declaration order — the regression this whole projection exists
/// to prevent was losing them.
#[test]
fn stacked_inset_layers_all_paint_in_declaration_order() {
    run_headless(|cx| {
        let painted = painted_inset_bands(
            cx,
            vec![
                poodle_node::ShadowLayer {
                    offset_x: 0.0,
                    offset_y: 0.0,
                    blur: 0.0,
                    spread: 1.0,
                    color: poodle_node::ColorValue(0.3, 0.6, 1.0, 0.12),
                    inset: true,
                },
                poodle_node::ShadowLayer {
                    offset_x: 3.0,
                    offset_y: 0.0,
                    blur: 0.0,
                    spread: 0.0,
                    color: poodle_node::ColorValue(0.3, 0.6, 1.0, 1.0),
                    inset: true,
                },
            ],
        );
        assert_eq!(painted.len(), 2, "both layers paint");
        assert_eq!(painted[0].top, 1.0, "the highlight ring is first");
        assert_eq!(painted[1].left, 3.0, "the leading bar is second");
        assert_eq!(painted[1].top, 0.0);
    });
}

/// A drop layer and an inset layer on the same node take different routes —
/// the drop through the shadow refinement, the inset through the painter —
/// and BOTH must survive.
#[test]
fn a_drop_layer_and_an_inset_layer_coexist() {
    run_headless(|cx| {
        let painted = painted_inset_bands(
            cx,
            vec![
                poodle_node::ShadowLayer {
                    offset_x: 0.0,
                    offset_y: 2.0,
                    blur: 8.0,
                    spread: 0.0,
                    color: poodle_node::ColorValue(0.0, 0.0, 0.0, 0.2),
                    inset: false,
                },
                poodle_node::ShadowLayer {
                    offset_x: 0.0,
                    offset_y: 1.0,
                    blur: 0.0,
                    spread: 0.0,
                    color: poodle_node::ColorValue(1.0, 1.0, 1.0, 0.4),
                    inset: true,
                },
            ],
        );
        assert_eq!(painted.len(), 1, "only the inset layer takes this route");
        assert_eq!(painted[0].top, 1.0);
    });
}

/// A node with no inset layer paints no bands, so the registry cannot report
/// a stale entry as evidence.
#[test]
fn a_node_without_inset_layers_paints_no_bands() {
    run_headless(|cx| {
        let painted = painted_inset_bands(
            cx,
            vec![poodle_node::ShadowLayer {
                offset_x: 0.0,
                offset_y: 2.0,
                blur: 8.0,
                spread: 0.0,
                color: poodle_node::ColorValue(0.0, 0.0, 0.0, 0.2),
                inset: false,
            }],
        );
        assert!(painted.is_empty());
    });
}

/// The end-to-end claim: a REAL Accordion, built by `poodle_render`, still
/// paints its contracted item highlight after the crates.io recovery. This is
/// the check that would have caught the regression the synthetic cases above
/// cannot see — a component composing its own tree, not a hand-built node.
#[test]
fn a_real_accordion_still_paints_its_contracted_item_highlight() {
    run_headless(|cx| {
        let mut node = poodle_render::accordion(
            &poodle_specs::AccordionSpec::new(vec![
                poodle_specs::AccordionItemSpec::new("one", "One"),
                poodle_specs::AccordionItemSpec::new("two", "Two"),
            ]),
            &RenderContext::new(&theme()),
            poodle_render::AccordionHandlers::new("inset-shadow-proof"),
        );
        // The highlight lives on an item surface, not the composition root.
        assert!(
            stamp_first_inset_node(&mut node),
            "the accordion composition must declare an inset layer at all"
        );
        let node = Arc::new(Mutex::new(node));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&node));
        driver.draw_frame();

        let painted = poodle_gpui_node_backend::painted_inset_shadows_for(INSET_ID);
        assert_eq!(
            painted.len(),
            1,
            "the accordion's contracted inset highlight must still paint"
        );
        assert!(
            painted[0].top > 0.0,
            "the highlight is a top-edge band, got {painted:?}"
        );
        assert!(
            painted[0].bounds[2] > 0.0 && painted[0].bounds[3] > 0.0,
            "the band must be clipped to a real padding box, got {painted:?}"
        );
    });
}

// ── g16.014 accordion mounted parity ────────────────────────────────────────

fn accordion_trigger_id(scope: &str, value: &str) -> String {
    poodle_render::accordion_trigger_focus_id(scope, value)
}

fn accordion_panel_id(scope: &str, value: &str) -> String {
    poodle_render::accordion_panel_focus_id(scope, value)
}

fn accordion_target<'a>(root: &'a Node, id: &str) -> &'a Node {
    root.find(&|node| {
        node.runtime_id.as_deref() == Some(id) || node.id.as_deref() == Some(id)
    })
    .unwrap_or_else(|| panic!("{id}"))
}

fn spec_from_accordion_result(value: &AccordionSelectionValue) -> AccordionSelectionValue {
    value.clone()
}

/// Accordion resulting-selection, disclosure semantics, keyboard parity, disabled
/// skips, and two-instance identity all travel through the real mounted tree.
#[test]
fn accordion_result_disclosure_focus_identity_and_disabled_paths() {
    use poodle_render::{accordion_with_content, AccordionHandlers};
    use poodle_specs::{AccordionItemSpec, AccordionSelectionMode, AccordionSelectionValue, AccordionSpec};

    run_headless(|cx| {
        fn build(
            value: AccordionSelectionValue,
            collapsible: bool,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<AccordionSelectionValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = AccordionSpec::new(vec![
                AccordionItemSpec::new("first", "First"),
                AccordionItemSpec::new("second", "Second"),
            ])
            .with_collapsible(collapsible)
            .with_value(value);
            let mut node = accordion_with_content(
                &spec,
                &RenderContext::new(&theme()),
                &[(
                    "first".to_string(),
                    Node::text("First panel"),
                )],
                AccordionHandlers::new("single").on_value_change(Arc::new(move |next| {
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() = build(
                        spec_from_accordion_result(&next),
                        collapsible,
                        Arc::clone(&mount),
                        Arc::clone(&sink),
                    );
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            AccordionSelectionValue::Single(Some("first".into())),
            true,
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        let first = accordion_trigger_id("single", "first");
        let second = accordion_trigger_id("single", "second");
        let panel = accordion_panel_id("single", "first");

        {
            let root = mounted.lock().unwrap();
            assert!(root.a11y.role.is_none());
            let trigger = accordion_target(&root, &first);
            assert_eq!(trigger.a11y.role, Some(NodeRole::Button));
            assert_eq!(trigger.a11y.expanded, Some(true));
            assert_eq!(trigger.a11y.controls.as_deref(), Some(panel.as_str()));
            assert!(trigger.style.focus_ring.is_some());
            let region = accordion_target(&root, &panel);
            assert_eq!(region.a11y.role, Some(NodeRole::Region));
            assert_eq!(region.a11y.labelled_by.as_deref(), Some(first.as_str()));
        }

        driver.wait_for_focus_handle(&first);
        driver.pointer_activate_id(&second);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [AccordionSelectionValue::Single(Some("second".into()))]
        );
        assert!(!mounted.lock().unwrap().has_text("First panel"));

        driver.wait_for_focus_handle(&second);
        driver.focus_element(&second);
        driver.dispatch_key_raw("space");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [
                AccordionSelectionValue::Single(Some("second".into())),
                AccordionSelectionValue::Single(None),
            ]
        );
        assert!(
            mounted
                .lock()
                .unwrap()
                .find(&|node| node.a11y.role == Some(NodeRole::Region))
                .is_none()
        );

        driver.pointer_activate_id(&first);
        assert_eq!(
            payloads.lock().unwrap().last(),
            Some(&AccordionSelectionValue::Single(Some("first".into())))
        );
    });

    run_headless(|cx| {
        fn build(
            value: AccordionSelectionValue,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<AccordionSelectionValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = AccordionSpec::new(vec![
                AccordionItemSpec::new("first", "First"),
                AccordionItemSpec::new("second", "Second"),
            ])
            .with_collapsible(false)
            .with_value(value);
            let mut node = accordion_with_content(
                &spec,
                &RenderContext::new(&theme()),
                &[],
                AccordionHandlers::new("locked").on_value_change(Arc::new(move |next| {
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() =
                        build(spec_from_accordion_result(&next), Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            AccordionSelectionValue::Single(Some("first".into())),
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let first = accordion_trigger_id("locked", "first");
        driver.wait_for_focus_handle(&first);
        driver.pointer_activate_id(&first);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [AccordionSelectionValue::Single(Some("first".into()))]
        );
    });

    run_headless(|cx| {
        fn build(
            value: AccordionSelectionValue,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<AccordionSelectionValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = AccordionSpec::new(vec![
                AccordionItemSpec::new("design", "Design"),
                AccordionItemSpec::new("keyboard", "Keyboard"),
            ])
            .with_selection_mode(AccordionSelectionMode::Multiple)
            .with_value(value);
            let mut node = accordion_with_content(
                &spec,
                &RenderContext::new(&theme()),
                &[],
                AccordionHandlers::new("multi").on_value_change(Arc::new(move |next| {
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() =
                        build(spec_from_accordion_result(&next), Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            AccordionSelectionValue::Multiple(vec!["design".into()]),
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        assert_eq!(
            mounted.lock().unwrap().a11y.role,
            Some(NodeRole::Group)
        );
        let design = accordion_trigger_id("multi", "design");
        let keyboard = accordion_trigger_id("multi", "keyboard");
        driver.wait_for_focus_handle(&keyboard);
        driver.focus_element(&keyboard);
        driver.dispatch_key_raw("enter");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [AccordionSelectionValue::Multiple(vec![
                "design".into(),
                "keyboard".into()
            ])],
            "enter adds the closed item to the ordered set"
        );
        driver.pointer_activate_id(&design);
        assert_eq!(
            payloads.lock().unwrap().last(),
            Some(&AccordionSelectionValue::Multiple(vec!["keyboard".into()])),
            "pointer removes an open member from the ordered set"
        );
        driver.pointer_activate_id(&design);
        assert_eq!(
            payloads.lock().unwrap().last(),
            Some(&AccordionSelectionValue::Multiple(vec![
                "keyboard".into(),
                "design".into()
            ])),
            "pointer add after remove rebuilds the complete Multiple result"
        );
    });

    run_headless(|cx| {
        fn marker(id: &str, label: &str) -> Node {
            let mut node = poodle_render::button(
                &poodle_specs::ButtonSpec::new().with_label(label),
                &RenderContext::new(&theme()),
                None,
            );
            node.id = Some(id.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let disabled = accordion_trigger_id("disabled", "locked");
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root = root
            .child(marker("accordion-before", "Before"))
            .child(accordion_with_content(
                &AccordionSpec::new(vec![
                    AccordionItemSpec::new("locked", "Locked").with_disabled(true),
                ])
                .with_value(AccordionSelectionValue::Single(None)),
                &RenderContext::new(&theme()),
                &[],
                AccordionHandlers::new("disabled")
                    .on_value_change(Arc::new(move |next| sink.lock().unwrap().push(next))),
            ))
            .child(marker("accordion-after", "After"));

        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 320.0, 160.0);
        driver.wait_for_focus_handle("accordion-before");
        driver.wait_for_focus_handle("accordion-after");

        let root_guard = mounted.lock().unwrap();
        let disabled_trigger = accordion_target(&root_guard, &disabled);
        assert!(disabled_trigger.interaction.disabled);
        assert!(!disabled_trigger.interaction.focusable);
        assert_eq!(disabled_trigger.a11y.tab_index, Some(-1));
        assert!(disabled_trigger.interaction.on_activate.is_none());
        drop(root_guard);
        assert!(
            poodle_gpui_node_backend::focus_handle_for(&disabled).is_none(),
            "disabled trigger never registers a sequential stop"
        );

        driver.pointer_activate_id(&disabled);
        assert!(payloads.lock().unwrap().is_empty());

        driver.focus_element("accordion-before");
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("accordion-after"),
            Some(true),
            "disabled accordion item is skipped by sequential focus"
        );
    });

    run_headless(|cx| {
        fn build_pair(
            left_value: AccordionSelectionValue,
            mounted: Arc<Mutex<Node>>,
            left_events: Arc<Mutex<Vec<AccordionSelectionValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&left_events);
            let panel_body = Node::text("Shared panel");
            let left = {
                let mut node = accordion_with_content(
                    &AccordionSpec::new(vec![AccordionItemSpec::new("shared", "Shared")])
                        .with_value(left_value),
                    &RenderContext::new(&theme()),
                    &[("shared".to_string(), panel_body.clone())],
                    AccordionHandlers::new("left").on_value_change(Arc::new({
                        let mount = Arc::clone(&mount);
                        let sink = Arc::clone(&sink);
                        move |next| {
                            sink.lock().unwrap().push(next.clone());
                            *mount.lock().unwrap() = build_pair(
                                spec_from_accordion_result(&next),
                                Arc::clone(&mount),
                                Arc::clone(&sink),
                            );
                        }
                    })),
                );
                node.id = Some("accordion-left-host".to_owned());
                node
            };
            let right = {
                let mut node = accordion_with_content(
                    &AccordionSpec::new(vec![AccordionItemSpec::new("shared", "Shared")])
                        .with_value(AccordionSelectionValue::Single(Some("shared".into()))),
                    &RenderContext::new(&theme()),
                    &[("shared".to_string(), panel_body)],
                    AccordionHandlers::new("right"),
                );
                node.id = Some("accordion-right-host".to_owned());
                node
            };
            let mut root = Node::container();
            root.style.descriptor.layout.direction = LayoutDirection::Row;
            root.style.descriptor.layout.spacing.gap = 16.0;
            root = root.child(left).child(right);
            root
        }

        let left_events = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build_pair(
            AccordionSelectionValue::Single(Some("shared".into())),
            Arc::clone(&mounted),
            Arc::clone(&left_events),
        );

        let left_trigger = accordion_trigger_id("left", "shared");
        let right_trigger = accordion_trigger_id("right", "shared");
        let left_panel = accordion_panel_id("left", "shared");
        let right_panel = accordion_panel_id("right", "shared");
        assert_ne!(left_trigger, right_trigger);
        assert_ne!(left_panel, right_panel);

        {
            let root = mounted.lock().unwrap();
            let left_region = accordion_target(&root, &left_panel);
            let right_region = accordion_target(&root, &right_panel);
            assert_eq!(left_region.a11y.role, Some(NodeRole::Region));
            assert_eq!(right_region.a11y.role, Some(NodeRole::Region));
            assert_eq!(
                left_region.a11y.labelled_by.as_deref(),
                Some(left_trigger.as_str())
            );
            assert_eq!(
                right_region.a11y.labelled_by.as_deref(),
                Some(right_trigger.as_str())
            );
            assert_eq!(
                accordion_target(&root, &left_trigger)
                    .a11y
                    .controls
                    .as_deref(),
                Some(left_panel.as_str())
            );
            assert_eq!(
                accordion_target(&root, &right_trigger)
                    .a11y
                    .controls
                    .as_deref(),
                Some(right_panel.as_str())
            );
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 480.0, 120.0);
        driver.wait_for_focus_handle(&left_trigger);
        driver.wait_for_focus_handle(&right_trigger);
        assert_ne!(
            poodle_gpui_node_backend::focus_handle_for(&left_trigger),
            poodle_gpui_node_backend::focus_handle_for(&right_trigger)
        );

        driver.pointer_activate_id(&left_trigger);
        assert_eq!(
            left_events.lock().unwrap().as_slice(),
            [AccordionSelectionValue::Single(None)]
        );
        assert!(
            mounted
                .lock()
                .unwrap()
                .find(&|node| node.runtime_id.as_deref() == Some(left_panel.as_str()))
                .is_none(),
            "left rebuild removes its panel while the right panel stays mounted"
        );
        assert!(
            mounted
                .lock()
                .unwrap()
                .find(&|node| node.runtime_id.as_deref() == Some(right_panel.as_str()))
                .is_some()
        );

        driver.pointer_activate_id(&left_trigger);
        assert_eq!(
            left_events.lock().unwrap().last(),
            Some(&AccordionSelectionValue::Single(Some("shared".into())))
        );
        assert_eq!(
            mounted
                .lock()
                .unwrap()
                .find(&|node| node.runtime_id.as_deref() == Some(left_panel.as_str()))
                .expect("left panel returns after rebuild")
                .a11y
                .labelled_by
                .as_deref(),
            Some(left_trigger.as_str())
        );
    });
}

// ── g16.002 selection-controls mounted parity ─────────────────────────────

fn checkbox_toggled(node: &Node) -> Option<poodle_node::NodeToggled> {
    node.a11y.toggled
}

/// Checkbox activation, mixed-to-checked, readonly, and disabled all travel
/// through the real mounted tree. The host stores the next checked value and
/// supplies the rebuilt spec; mixed resolves to checked on the first accept.
#[test]
fn checkbox_toggle_readonly_and_disabled_rebuild_the_host_spec() {
    use poodle_node::NodeToggled;
    use poodle_specs::CheckboxSpec;

    run_headless(|cx| {
        fn build(
            checked: bool,
            mixed: bool,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let mut spec = CheckboxSpec::new()
                .with_checked(checked)
                .with_label("Notify");
            if mixed {
                spec = spec.with_mixed(true);
            }
            let mut node = poodle_render::checkbox(
                &spec,
                &RenderContext::new(&theme()),
                Some(Arc::new(move |next| {
                    sink.lock().unwrap().push(next);
                    *mount.lock().unwrap() =
                        build(next, false, Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, true, Arc::clone(&mounted), Arc::clone(&payloads));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        assert_eq!(
            checkbox_toggled(&mounted.lock().unwrap()),
            Some(NodeToggled::Mixed)
        );
        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.pointer_activate();
        assert_eq!(payloads.lock().unwrap().as_slice(), [true]);
        assert_eq!(
            checkbox_toggled(&mounted.lock().unwrap()),
            Some(NodeToggled::True),
            "mixed resolves to checked on the first accepted activation"
        );

        driver.pointer_activate();
        assert_eq!(payloads.lock().unwrap().as_slice(), [true, false]);
        assert_eq!(
            checkbox_toggled(&mounted.lock().unwrap()),
            Some(NodeToggled::False)
        );
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let mut node = poodle_render::checkbox(
            &CheckboxSpec::new()
                .with_checked(true)
                .with_read_only(true)
                .with_label("Locked"),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next| sink.lock().unwrap().push(next))),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.focus_element(FIXTURE_ID);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(FIXTURE_ID),
            Some(true),
            "readonly stays focusable"
        );
        driver.dispatch_key_raw("space");
        driver.pointer_activate();
        assert!(
            payloads.lock().unwrap().is_empty(),
            "readonly does not change or emit"
        );
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let mut node = poodle_render::checkbox(
            &CheckboxSpec::new()
                .with_checked(false)
                .with_disabled(true)
                .with_label("Off"),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next| sink.lock().unwrap().push(next))),
        );
        node.id = Some("checkbox-disabled".to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        assert!(
            poodle_gpui_node_backend::focus_handle_for("checkbox-disabled").is_none(),
            "disabled does not accept focus"
        );
        driver.pointer_activate();
        assert!(
            payloads.lock().unwrap().is_empty(),
            "disabled does not accept activation"
        );
    });
}

/// Switch activation, readonly, and disabled match Checkbox's binary rules
/// through the real mounted tree. The host rebuilds from the emitted next value.
#[test]
fn switch_toggle_readonly_and_disabled_rebuild_the_host_spec() {
    use poodle_node::NodeToggled;
    use poodle_specs::SwitchSpec;

    run_headless(|cx| {
        fn build(
            checked: bool,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let mut node = poodle_render::switch(
                &SwitchSpec::new()
                    .with_checked(checked)
                    .with_label("Dark mode"),
                &RenderContext::new(&theme()),
                Some(Arc::new(move |next| {
                    sink.lock().unwrap().push(next);
                    *mount.lock().unwrap() = build(next, Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(false, Arc::clone(&mounted), Arc::clone(&payloads));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.pointer_activate();
        assert_eq!(payloads.lock().unwrap().as_slice(), [true]);
        assert_eq!(
            checkbox_toggled(&mounted.lock().unwrap()),
            Some(NodeToggled::True)
        );
        driver.dispatch_key_raw("enter");
        assert_eq!(payloads.lock().unwrap().as_slice(), [true, false]);
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let mut node = poodle_render::switch(
            &SwitchSpec::new()
                .with_checked(true)
                .with_read_only(true)
                .with_label("Locked"),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next| sink.lock().unwrap().push(next))),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.wait_for_focus_handle(FIXTURE_ID);
        driver.keyboard_key(FIXTURE_ID, "space");
        driver.pointer_activate();
        assert!(payloads.lock().unwrap().is_empty());
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(FIXTURE_ID),
            Some(true)
        );
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let mut node = poodle_render::switch(
            &SwitchSpec::new()
                .with_checked(false)
                .with_disabled(true)
                .with_label("Off"),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next| sink.lock().unwrap().push(next))),
        );
        node.id = Some("switch-disabled".to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        assert!(poodle_gpui_node_backend::focus_handle_for("switch-disabled").is_none());
        driver.pointer_activate();
        assert!(payloads.lock().unwrap().is_empty());
    });
}

fn segment_option_id(scope: &str, value: &str) -> String {
    format!("segmented:{scope}:option:{value}")
}

fn segment_selected(node: &Node, scope: &str, value: &str) -> bool {
    let id = segment_option_id(scope, value);
    node.find(&|n| n.runtime_id.as_deref() == Some(id.as_str()))
        .and_then(|n| n.a11y.selected)
        .unwrap_or(false)
}

fn selection_segment_options() -> Vec<poodle_specs::SegmentedControlOption> {
    vec![
        poodle_specs::SegmentedControlOption::new("grid", "Grid"),
        poodle_specs::SegmentedControlOption::new("list", "List").with_disabled(true),
        poodle_specs::SegmentedControlOption::new("table", "Table"),
    ]
}

/// SegmentedControl exclusive selection, wrap, disabled skip, disabled-group
/// inertia, and independent instance focus identity through the mounted tree.
#[test]
fn segmented_control_exclusive_focus_identity_and_disabled_paths() {
    use poodle_specs::{SegmentedControlOption, SegmentedControlSpec};

    run_headless(|cx| {
        fn build(
            value: &str,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let mut spec = SegmentedControlSpec::new("view", selection_segment_options());
            spec.value = Some(value.to_string());
            let mut node = poodle_render::segmented_control(
                &spec,
                &RenderContext::new(&theme()),
                Some(Arc::new(move |next: &str| {
                    sink.lock().unwrap().push(next.to_string());
                    *mount.lock().unwrap() = build(next, Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build("grid", Arc::clone(&mounted), Arc::clone(&payloads));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        let grid = segment_option_id("view", "grid");
        let list = segment_option_id("view", "list");
        let table = segment_option_id("view", "table");
        driver.wait_for_focus_handle(&grid);
        driver.pointer_activate_id(&table);
        assert_eq!(payloads.lock().unwrap().as_slice(), ["table"]);
        assert!(segment_selected(&mounted.lock().unwrap(), "view", "table"));

        driver.pointer_activate_id(&table);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["table"],
            "same-value selection is inert"
        );
        driver.pointer_activate_id(&list);
        assert_eq!(payloads.lock().unwrap().as_slice(), ["table"]);

        driver.wait_for_focus_handle(&table);
        driver.focus_element(&table);
        driver.dispatch_key_raw("right");
        assert_eq!(payloads.lock().unwrap().as_slice(), ["table", "grid"]);
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&grid), Some(true));
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = SegmentedControlSpec {
            is_disabled: true,
            ..SegmentedControlSpec::new("disabled-view", selection_segment_options())
        };
        let mut spec = spec;
        spec.value = Some("grid".to_string());
        let mut node = poodle_render::segmented_control(
            &spec,
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next: &str| {
                sink.lock().unwrap().push(next.to_string())
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.pointer_activate_id(&segment_option_id("disabled-view", "table"));
        assert!(payloads.lock().unwrap().is_empty());
    });

    run_headless(|cx| {
        let picker = |scope: &str| {
            let mut spec = SegmentedControlSpec::new(
                scope,
                vec![
                    SegmentedControlOption::new("grid", "Grid"),
                    SegmentedControlOption::new("list", "List"),
                ],
            );
            spec.value = Some("grid".to_string());
            poodle_render::segmented_control(&spec, &RenderContext::new(&theme()), None)
        };
        let mut node = Node::container()
            .child(picker("left"))
            .child(picker("right"));
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        let left = segment_option_id("left", "grid");
        let right = segment_option_id("right", "grid");
        driver.wait_for_focus_handle(&left);
        driver.wait_for_focus_handle(&right);
        driver.focus_element(&left);
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&left), Some(true));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right),
            Some(false),
            "two mounted controls keep independent focus identity"
        );
    });
}

fn radio_option_id(scope: &str, value: &str) -> String {
    format!("radio:{scope}:option:{value}")
}

fn radio_selected(node: &Node, scope: &str, value: &str) -> bool {
    let id = radio_option_id(scope, value);
    node.find(&|n| n.runtime_id.as_deref() == Some(id.as_str()))
        .and_then(|n| n.a11y.selected)
        .unwrap_or(false)
}

fn selection_radio_options() -> Vec<poodle_specs::ChoiceOption> {
    vec![
        poodle_specs::ChoiceOption::new("free", "Free"),
        poodle_specs::ChoiceOption::new("pro", "Pro").with_disabled(true),
        poodle_specs::ChoiceOption::new("enterprise", "Enterprise"),
    ]
}

/// RadioGroup exclusive selection, orientation-aware arrows, wrap, disabled
/// skip, disabled-group inertia, and independent instance focus identity
/// through the mounted tree.
#[test]
fn radio_group_exclusive_focus_identity_and_disabled_paths() {
    use poodle_specs::{ChoiceOption, Orientation, RadioGroupSpec};

    run_headless(|cx| {
        fn build(
            value: &str,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let mut spec = RadioGroupSpec::new(selection_radio_options());
            spec.value = Some(value.to_string());
            let mut node = poodle_render::radio_group(
                &spec,
                &RenderContext::new(&theme()),
                RadioGroupHandlers::new("plan").on_change(Arc::new(move |next: &str| {
                    sink.lock().unwrap().push(next.to_string());
                    *mount.lock().unwrap() = build(next, Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build("free", Arc::clone(&mounted), Arc::clone(&payloads));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        let free = radio_option_id("plan", "free");
        let pro = radio_option_id("plan", "pro");
        let enterprise = radio_option_id("plan", "enterprise");
        driver.wait_for_focus_handle(&free);
        driver.pointer_activate_id(&enterprise);
        assert_eq!(payloads.lock().unwrap().as_slice(), ["enterprise"]);
        assert!(radio_selected(
            &mounted.lock().unwrap(),
            "plan",
            "enterprise"
        ));

        driver.pointer_activate_id(&enterprise);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["enterprise"],
            "same-value selection is inert"
        );
        driver.pointer_activate_id(&pro);
        assert_eq!(payloads.lock().unwrap().as_slice(), ["enterprise"]);

        driver.wait_for_focus_handle(&enterprise);
        driver.focus_element(&enterprise);
        driver.dispatch_key_raw("right");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["enterprise"],
            "unrelated-axis arrows are inert"
        );
        driver.dispatch_key_raw("down");
        assert_eq!(payloads.lock().unwrap().as_slice(), ["enterprise", "free"]);
        assert!(radio_selected(&mounted.lock().unwrap(), "plan", "free"));
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&free), Some(true));
    });

    run_headless(|cx| {
        fn build(
            value: &str,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = RadioGroupSpec::new(vec![
                ChoiceOption::new("sm", "Small"),
                ChoiceOption::new("md", "Medium"),
                ChoiceOption::new("lg", "Large"),
            ])
            .with_value(value)
            .with_orientation(Orientation::Horizontal);
            let mut node = poodle_render::radio_group(
                &spec,
                &RenderContext::new(&theme()),
                RadioGroupHandlers::new("size").on_change(Arc::new(move |next: &str| {
                    sink.lock().unwrap().push(next.to_string());
                    *mount.lock().unwrap() = build(next, Arc::clone(&mount), Arc::clone(&sink));
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build("lg", Arc::clone(&mounted), Arc::clone(&payloads));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let lg = radio_option_id("size", "lg");
        let sm = radio_option_id("size", "sm");
        driver.wait_for_focus_handle(&lg);
        driver.focus_element(&lg);
        driver.dispatch_key_raw("down");
        assert!(
            payloads.lock().unwrap().is_empty(),
            "vertical arrows are inert on a horizontal group"
        );
        driver.dispatch_key_raw("right");
        assert_eq!(payloads.lock().unwrap().as_slice(), ["sm"]);
        assert!(radio_selected(&mounted.lock().unwrap(), "size", "sm"));
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&sm), Some(true));
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = RadioGroupSpec {
            is_disabled: true,
            ..RadioGroupSpec::new(selection_radio_options()).with_value("free")
        };
        let mut node = poodle_render::radio_group(
            &spec,
            &RenderContext::new(&theme()),
            RadioGroupHandlers::new("disabled-plan").on_change(Arc::new(move |next: &str| {
                sink.lock().unwrap().push(next.to_string())
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.pointer_activate_id(&radio_option_id("disabled-plan", "enterprise"));
        assert!(payloads.lock().unwrap().is_empty());
    });

    run_headless(|cx| {
        let picker = |scope: &str| {
            poodle_render::radio_group(
                &RadioGroupSpec::new(vec![
                    ChoiceOption::new("free", "Free"),
                    ChoiceOption::new("pro", "Pro"),
                ])
                .with_value("free"),
                &RenderContext::new(&theme()),
                RadioGroupHandlers::new(scope),
            )
        };
        let mut node = Node::container()
            .child(picker("left"))
            .child(picker("right"));
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        let left = radio_option_id("left", "free");
        let right = radio_option_id("right", "free");
        driver.wait_for_focus_handle(&left);
        driver.wait_for_focus_handle(&right);
        driver.focus_element(&left);
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&left), Some(true));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right),
            Some(false),
            "two mounted groups keep independent focus identity"
        );
    });
}

fn tri_state_segment_id(scope: &str, value: TriStateValue) -> String {
    format!("tri-state:{scope}:option:{}", value.as_str())
}

fn tri_state_marker(id: &str, label: &str) -> Node {
    let mut node = poodle_render::button(
        &poodle_specs::ButtonSpec::new().with_label(label),
        &RenderContext::new(&theme()),
        None,
    );
    node.id = Some(id.to_owned());
    node
}

fn tri_state_target<'a>(root: &'a Node, id: &str) -> &'a Node {
    root.find(&|node| {
        node.runtime_id.as_deref() == Some(id) || node.id.as_deref() == Some(id)
    })
    .unwrap_or_else(|| panic!("{id}"))
}

fn tri_state_selected(node: &Node, scope: &str, value: TriStateValue) -> bool {
    tri_state_target(node, &tri_state_segment_id(scope, value))
        .a11y
        .selected
        .unwrap_or(false)
}

fn assert_tri_state_pointer_targets(ids: &[String]) {
    for id in ids {
        assert!(
            poodle_gpui_node_backend::bounds_for(id).is_some(),
            "pointer proof needs a real hit target for {id}"
        );
    }
}

fn assert_tri_state_radio_semantics(
    switch: &Node,
    scope: &str,
    selected: TriStateValue,
    group_label: &str,
) {
    assert_eq!(switch.a11y.role, Some(NodeRole::RadioGroup));
    assert_eq!(switch.a11y.label.as_deref(), Some(group_label));

    let labels = ["Exclude", "Default", "Include"];
    let mut tab_stops = 0;
    for (value, label) in TriStateValue::ALL.iter().zip(labels) {
        let segment = tri_state_target(switch, &tri_state_segment_id(scope, *value));
        assert_eq!(segment.a11y.role, Some(NodeRole::RadioButton));
        assert_eq!(segment.a11y.label.as_deref(), Some(label));
        let is_selected = *value == selected;
        assert_eq!(segment.a11y.selected, Some(is_selected));
        assert_eq!(
            segment.a11y.toggled,
            Some(if is_selected {
                poodle_node::NodeToggled::True
            } else {
                poodle_node::NodeToggled::False
            })
        );
        if segment.a11y.tab_index == Some(0) {
            tab_stops += 1;
        }
        assert!(segment.style.focus_ring.is_some(), "{label} focus ring");
    }
    assert_eq!(tab_stops, 1, "exactly one roving tab stop");
}

/// TriStateSwitch exclusive selection, arrow wrap, same-value inertia, disabled
/// skip, and independent instance focus identity through the mounted tree.
#[test]
fn tri_state_switch_value_focus_identity_and_disabled_paths() {
    run_headless(|cx| {
        fn build(
            value: TriStateValue,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<TriStateValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let mut node = poodle_render::tri_state_switch(
                &TriStateSwitchSpec::new()
                    .with_value(value)
                    .with_aria_label("Filter mode"),
                &RenderContext::new(&theme()),
                TriStateSwitchHandlers::new("filter").on_value_change(Arc::new(
                    move |next: TriStateValue| {
                        sink.lock().unwrap().push(next);
                        *mount.lock().unwrap() =
                            build(next, Arc::clone(&mount), Arc::clone(&sink));
                    },
                )),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            TriStateValue::Default,
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 280.0, 60.0);

        let excluded = tri_state_segment_id("filter", TriStateValue::Excluded);
        let default = tri_state_segment_id("filter", TriStateValue::Default);
        let included = tri_state_segment_id("filter", TriStateValue::Included);
        let pointer_targets = [excluded.clone(), default.clone(), included.clone()];

        assert_tri_state_radio_semantics(
            &mounted.lock().unwrap(),
            "filter",
            TriStateValue::Default,
            "Filter mode",
        );
        assert_tri_state_pointer_targets(&pointer_targets);
        driver.wait_for_focus_handle(&default);
        driver.wait_for_focus_handle(&excluded);
        assert!(tri_state_selected(
            &mounted.lock().unwrap(),
            "filter",
            TriStateValue::Default
        ));

        driver.focus_element(&excluded);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&excluded),
            Some(true),
            "an unselected segment accepts programmatic focus"
        );
        driver.dispatch_key_raw("space");
        assert_eq!(payloads.lock().unwrap().as_slice(), [TriStateValue::Excluded]);
        assert_tri_state_radio_semantics(
            &mounted.lock().unwrap(),
            "filter",
            TriStateValue::Excluded,
            "Filter mode",
        );
        assert_tri_state_pointer_targets(&pointer_targets);

        driver.wait_for_focus_handle(&excluded);
        driver.focus_element(&excluded);
        driver.dispatch_key_raw("space");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [TriStateValue::Excluded],
            "space on the selected segment is inert"
        );

        driver.pointer_activate_id(&included);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [TriStateValue::Excluded, TriStateValue::Included]
        );
        assert!(tri_state_selected(
            &mounted.lock().unwrap(),
            "filter",
            TriStateValue::Included
        ));

        driver.pointer_activate_id(&included);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [TriStateValue::Excluded, TriStateValue::Included],
            "same-value pointer selection is inert"
        );

        driver.wait_for_focus_handle(&included);
        driver.focus_element(&included);
        driver.dispatch_key_raw("right");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [
                TriStateValue::Excluded,
                TriStateValue::Included,
                TriStateValue::Excluded
            ],
            "right wraps from included to excluded"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&excluded),
            Some(true)
        );

        driver.dispatch_key_raw("left");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [
                TriStateValue::Excluded,
                TriStateValue::Included,
                TriStateValue::Excluded,
                TriStateValue::Included
            ],
            "left wraps from excluded to included"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&included),
            Some(true)
        );
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let disabled = tri_state_segment_id("disabled-filter", TriStateValue::Included);
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root = root
            .child(tri_state_marker("tri-state-before", "Before"))
            .child({
                let mut node = poodle_render::tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Default)
                        .with_disabled(true)
                        .with_aria_label("Disabled switch"),
                    &RenderContext::new(&theme()),
                    TriStateSwitchHandlers::new("disabled-filter")
                        .on_value_change(Arc::new(move |next| sink.lock().unwrap().push(next))),
                );
                node.id = Some("tri-state-disabled-host".to_owned());
                node
            })
            .child(tri_state_marker("tri-state-after", "After"));

        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 280.0, 120.0);
        driver.wait_for_focus_handle("tri-state-before");
        driver.wait_for_focus_handle("tri-state-after");

        let root = mounted.lock().unwrap();
        let disabled_segment = tri_state_target(&root, &disabled);
        assert!(disabled_segment.interaction.disabled);
        assert!(!disabled_segment.interaction.focusable);
        assert_eq!(disabled_segment.a11y.tab_index, Some(-1));
        assert!(disabled_segment.interaction.on_activate.is_none());
        drop(root);
        assert!(
            poodle_gpui_node_backend::focus_handle_for(&disabled).is_none(),
            "disabled segment never registers a sequential stop"
        );

        driver.pointer_activate_id(&disabled);
        assert!(payloads.lock().unwrap().is_empty());

        driver.focus_element("tri-state-before");
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("tri-state-after"),
            Some(true),
            "disabled tri-state is skipped by sequential focus"
        );
    });

    run_headless(|cx| {
        fn build_pair(
            left_value: TriStateValue,
            mounted: Arc<Mutex<Node>>,
            left_events: Arc<Mutex<Vec<TriStateValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&left_events);
            let left = {
                let mut node = poodle_render::tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(left_value)
                        .with_aria_label("Left filter"),
                    &RenderContext::new(&theme()),
                    TriStateSwitchHandlers::new("left").on_value_change(Arc::new(
                        move |next: TriStateValue| {
                            sink.lock().unwrap().push(next);
                            *mount.lock().unwrap() = build_pair(
                                next,
                                Arc::clone(&mount),
                                Arc::clone(&sink),
                            );
                        },
                    )),
                );
                node.id = Some("tri-state-left-host".to_owned());
                node
            };
            let right = {
                let mut node = poodle_render::tri_state_switch(
                    &TriStateSwitchSpec::new()
                        .with_value(TriStateValue::Default)
                        .with_aria_label("Right filter"),
                    &RenderContext::new(&theme()),
                    TriStateSwitchHandlers::new("right"),
                );
                node.id = Some("tri-state-right-host".to_owned());
                node
            };
            Node::container().child(left).child(right)
        }

        let left_events = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build_pair(
            TriStateValue::Default,
            Arc::clone(&mounted),
            Arc::clone(&left_events),
        );
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 560.0, 60.0);

        let left_default = tri_state_segment_id("left", TriStateValue::Default);
        let left_excluded = tri_state_segment_id("left", TriStateValue::Excluded);
        let right_default = tri_state_segment_id("right", TriStateValue::Default);
        driver.wait_for_focus_handle(&left_default);
        driver.wait_for_focus_handle(&right_default);
        assert_tri_state_pointer_targets(&[
            left_default.clone(),
            left_excluded.clone(),
            right_default.clone(),
        ]);

        driver.pointer_activate_id(&left_excluded);
        assert_eq!(left_events.lock().unwrap().as_slice(), [TriStateValue::Excluded]);
        let root = mounted.lock().unwrap();
        let left_host = tri_state_target(&root, "tri-state-left-host");
        let right_host = tri_state_target(&root, "tri-state-right-host");
        assert_tri_state_radio_semantics(
            left_host,
            "left",
            TriStateValue::Excluded,
            "Left filter",
        );
        assert_tri_state_radio_semantics(
            right_host,
            "right",
            TriStateValue::Default,
            "Right filter",
        );
        drop(root);

        driver.wait_for_focus_handle(&left_excluded);
        driver.wait_for_focus_handle(&right_default);
        driver.focus_element(&left_excluded);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&left_excluded),
            Some(true)
        );
        driver.focus_element(&right_default);
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right_default),
            Some(true)
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&left_excluded),
            Some(false),
            "rebuilt instances keep independent focus identity"
        );
    });
}

fn toggle_option_id(scope: &str, value: &str) -> String {
    format!("toggle:{scope}:option:{value}")
}

fn toggle_pressed(node: &Node, scope: &str, value: &str) -> bool {
    let id = toggle_option_id(scope, value);
    node.find(&|n| n.runtime_id.as_deref() == Some(id.as_str()))
        .and_then(|n| n.a11y.toggled)
        .map(|toggled| toggled == poodle_node::NodeToggled::True)
        .unwrap_or(false)
}

fn spec_from_result(result: &poodle_headless::toggle_group::ToggleGroupValue) -> Vec<String> {
    match result {
        poodle_headless::toggle_group::ToggleGroupValue::Single(Some(value)) => {
            vec![value.clone()]
        }
        poodle_headless::toggle_group::ToggleGroupValue::Single(None) => Vec::new(),
        poodle_headless::toggle_group::ToggleGroupValue::Multiple(values) => values.clone(),
    }
}

fn selection_toggle_options() -> Vec<poodle_specs::ToggleGroupOption> {
    vec![
        poodle_specs::ToggleGroupOption::new("grid", "Grid"),
        poodle_specs::ToggleGroupOption::new("list", "List").with_disabled(true),
        poodle_specs::ToggleGroupOption::new("board", "Board"),
    ]
}

/// ToggleGroup resulting-selection payloads, same-value emission, wrap,
/// disabled skip, multiple add/remove, disabled-group inertia, and
/// independent instance focus identity through the mounted tree.
#[test]
fn toggle_group_result_focus_identity_and_disabled_paths() {
    use poodle_headless::toggle_group::ToggleGroupValue;
    use poodle_specs::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

    run_headless(|cx| {
        fn build(
            value: Vec<String>,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<ToggleGroupValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = ToggleGroupSpec::new(selection_toggle_options()).with_value(value);
            let mut node = poodle_render::toggle_group(
                &spec,
                &RenderContext::new(&theme()),
                ToggleGroupHandlers::new("view").on_value_change(Arc::new(move |next| {
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() = build(
                        spec_from_result(&next),
                        Arc::clone(&mount),
                        Arc::clone(&sink),
                    );
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            vec!["grid".into()],
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        let grid = toggle_option_id("view", "grid");
        let list = toggle_option_id("view", "list");
        let board = toggle_option_id("view", "board");
        driver.wait_for_focus_handle(&grid);
        driver.pointer_activate_id(&board);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [ToggleGroupValue::Single(Some("board".into()))]
        );
        assert!(toggle_pressed(&mounted.lock().unwrap(), "view", "board"));

        driver.pointer_activate_id(&board);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [
                ToggleGroupValue::Single(Some("board".into())),
                ToggleGroupValue::Single(Some("board".into())),
            ],
            "same-value selection still emits"
        );
        driver.pointer_activate_id(&list);
        assert_eq!(
            payloads.lock().unwrap().len(),
            2,
            "disabled options stay inert"
        );

        driver.wait_for_focus_handle(&board);
        driver.focus_element(&board);
        driver.dispatch_key_raw("right");
        assert_eq!(
            payloads.lock().unwrap().last(),
            Some(&ToggleGroupValue::Single(Some("grid".into())))
        );
        assert!(toggle_pressed(&mounted.lock().unwrap(), "view", "grid"));
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&grid), Some(true));
    });

    run_headless(|cx| {
        fn build(
            value: Vec<String>,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<ToggleGroupValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = ToggleGroupSpec::new(vec![
                ToggleGroupOption::new("grid", "Grid"),
                ToggleGroupOption::new("list", "List"),
                ToggleGroupOption::new("board", "Board"),
            ])
            .with_value(value)
            .with_allow_deactivation(true);
            let mut node = poodle_render::toggle_group(
                &spec,
                &RenderContext::new(&theme()),
                ToggleGroupHandlers::new("optional-view").on_value_change(Arc::new(move |next| {
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() = build(
                        spec_from_result(&next),
                        Arc::clone(&mount),
                        Arc::clone(&sink),
                    );
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            vec!["grid".into()],
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let grid = toggle_option_id("optional-view", "grid");
        driver.wait_for_focus_handle(&grid);
        driver.pointer_activate_id(&grid);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [ToggleGroupValue::Single(None)]
        );
        assert!(!toggle_pressed(
            &mounted.lock().unwrap(),
            "optional-view",
            "grid"
        ));
    });

    run_headless(|cx| {
        fn build(
            value: Vec<String>,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<ToggleGroupValue>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let spec = ToggleGroupSpec::new(vec![
                ToggleGroupOption::new("design", "Design"),
                ToggleGroupOption::new("engineering", "Engineering"),
                ToggleGroupOption::new("docs", "Docs"),
            ])
            .with_value(value)
            .with_selection_mode(ToggleGroupSelectionMode::Multiple);
            let mut node = poodle_render::toggle_group(
                &spec,
                &RenderContext::new(&theme()),
                ToggleGroupHandlers::new("tags").on_value_change(Arc::new(move |next| {
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() = build(
                        spec_from_result(&next),
                        Arc::clone(&mount),
                        Arc::clone(&sink),
                    );
                })),
            );
            node.id = Some(FIXTURE_ID.to_owned());
            node
        }

        let payloads = Arc::new(Mutex::new(Vec::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().unwrap() = build(
            vec!["design".into(), "docs".into()],
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        let design = toggle_option_id("tags", "design");
        let engineering = toggle_option_id("tags", "engineering");
        driver.wait_for_focus_handle(&design);
        driver.pointer_activate_id(&engineering);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [ToggleGroupValue::Multiple(vec![
                "design".into(),
                "docs".into(),
                "engineering".into()
            ])]
        );
        assert!(toggle_pressed(
            &mounted.lock().unwrap(),
            "tags",
            "engineering"
        ));
        driver.pointer_activate_id(&design);
        assert_eq!(
            payloads.lock().unwrap().last(),
            Some(&ToggleGroupValue::Multiple(vec![
                "docs".into(),
                "engineering".into()
            ]))
        );
        assert!(!toggle_pressed(&mounted.lock().unwrap(), "tags", "design"));
        driver.focus_element(&design);
        driver.dispatch_key_raw("right");
        assert_eq!(
            payloads.lock().unwrap().len(),
            2,
            "multiple mode does not intercept arrows"
        );
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = ToggleGroupSpec {
            is_disabled: true,
            ..ToggleGroupSpec::new(selection_toggle_options()).with_value(vec!["grid".into()])
        };
        let mut node = poodle_render::toggle_group(
            &spec,
            &RenderContext::new(&theme()),
            ToggleGroupHandlers::new("disabled-view")
                .on_value_change(Arc::new(move |next| sink.lock().unwrap().push(next))),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.pointer_activate_id(&toggle_option_id("disabled-view", "board"));
        assert!(payloads.lock().unwrap().is_empty());
    });

    run_headless(|cx| {
        let picker = |scope: &str| {
            poodle_render::toggle_group(
                &ToggleGroupSpec::new(vec![
                    ToggleGroupOption::new("grid", "Grid"),
                    ToggleGroupOption::new("list", "List"),
                ])
                .with_value(vec!["grid".into()]),
                &RenderContext::new(&theme()),
                ToggleGroupHandlers::new(scope),
            )
        };
        let mut node = Node::container()
            .child(picker("left"))
            .child(picker("right"));
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        let left = toggle_option_id("left", "grid");
        let right = toggle_option_id("right", "grid");
        driver.wait_for_focus_handle(&left);
        driver.wait_for_focus_handle(&right);
        driver.focus_element(&left);
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&left), Some(true));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right),
            Some(false),
            "two mounted groups keep independent focus identity"
        );
    });
}

// ── g16.007 TextInput controlled editing ───────────────────────────────────

/// One field's worth of host-owned state. The Rust targets have no native
/// editor, so the value, the caret and the focus flag all live here and the
/// public spec is rebuilt from them after every reported callback — the same
/// shape a real host has to implement.
#[derive(Clone)]
struct TextFieldState {
    name: String,
    value: String,
    selection: (usize, usize),
    is_focused: bool,
    input_type: String,
    placeholder: Option<String>,
    is_disabled: bool,
    is_read_only: bool,
    max_length: Option<usize>,
}

impl TextFieldState {
    fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
            selection: (0, 0),
            is_focused: false,
            input_type: "text".to_owned(),
            placeholder: None,
            is_disabled: false,
            is_read_only: false,
            max_length: None,
        }
    }

    fn searchable(mut self) -> Self {
        self.input_type = "search".to_owned();
        self
    }

    fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_owned());
        self
    }

    fn disabled(mut self) -> Self {
        self.is_disabled = true;
        self
    }

    fn read_only(mut self) -> Self {
        self.is_read_only = true;
        self
    }

    fn limited(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }
}

/// The controlled host: what every field currently holds, and every callback
/// it has been told about, in order.
struct TextFieldHost {
    fields: Mutex<Vec<TextFieldState>>,
    log: Mutex<Vec<String>>,
}

impl TextFieldHost {
    fn new(fields: Vec<TextFieldState>) -> Arc<Self> {
        Arc::new(Self {
            fields: Mutex::new(fields),
            log: Mutex::new(Vec::new()),
        })
    }

    fn field(&self, name: &str) -> TextFieldState {
        self.fields
            .lock()
            .expect("fields")
            .iter()
            .find(|field| field.name == name)
            .cloned()
            .unwrap_or_else(|| panic!("{name} is mounted"))
    }

    fn log(&self) -> Vec<String> {
        self.log.lock().expect("log").clone()
    }

    fn take_log(&self) -> Vec<String> {
        std::mem::take(&mut *self.log.lock().expect("log"))
    }
}

/// The element ids the backend keys focus, bounds and history by. Derived from
/// the field's own `id`, which the contract requires every field to carry.
fn field_id(name: &str) -> String {
    format!("poodle-input-{name}")
}
fn field_value_id(name: &str) -> String {
    format!("poodle-input-{name}-value")
}
fn field_clear_id(name: &str) -> String {
    format!("poodle-input-{name}-clear")
}

/// Rebuild the whole mounted tree from host state. Every callback ends here,
/// so nothing an assertion reads was written by the test: the value and the
/// caret in the tree are the ones the host stored and handed back as props.
fn text_field_tree(host: &Arc<TextFieldHost>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let states = host.fields.lock().expect("fields").clone();

    let mut column = Node::container();
    column.id = Some(FIXTURE_ID.to_owned());
    column.style.descriptor.layout.direction = LayoutDirection::Column;
    column.style.descriptor.layout.spacing.gap = 8.0;
    column.style.descriptor.layout.width = LayoutSizing::Fixed(360.0);

    for state in &states {
        let mut spec = poodle_specs::TextInputSpec::new()
            .with_id(&state.name)
            .with_aria_label(&state.name)
            .with_value(&state.value)
            .with_selection(state.selection.0, state.selection.1)
            .with_is_focused(state.is_focused)
            .with_type(&state.input_type)
            .with_disabled(state.is_disabled)
            .with_read_only(state.is_read_only);
        if let Some(placeholder) = &state.placeholder {
            spec = spec.with_placeholder(placeholder);
        }
        if let Some(max) = state.max_length {
            spec = spec.with_max_length(max);
        }
        column = column.child(poodle_render::text_input_with_handlers(
            &spec,
            &ctx,
            text_field_handlers(host, mounted, &state.name),
        ));
    }
    column
}

/// Store one reported result and rebuild. The mutation and the rebuild are
/// separated so the fields lock is never held across the rebuild.
fn text_field_apply(
    host: &Arc<TextFieldHost>,
    mounted: &Arc<Mutex<Node>>,
    name: &str,
    entry: String,
    mutate: impl FnOnce(&mut TextFieldState),
) {
    {
        let mut fields = host.fields.lock().expect("fields");
        let field = fields
            .iter_mut()
            .find(|field| field.name == name)
            .unwrap_or_else(|| panic!("{name} is mounted"));
        mutate(field);
    }
    host.log.lock().expect("log").push(entry);
    let next = text_field_tree(host, mounted);
    *mounted.lock().expect("mount") = next;
}

fn text_field_handlers(
    host: &Arc<TextFieldHost>,
    mounted: &Arc<Mutex<Node>>,
    name: &str,
) -> poodle_render::TextInputHandlers {
    macro_rules! sink {
        () => {{
            (Arc::clone(host), Arc::clone(mounted), name.to_owned())
        }};
    }
    let (change_host, change_mount, change_name) = sink!();
    let (select_host, select_mount, select_name) = sink!();
    let (focus_host, focus_mount, focus_name) = sink!();
    let (submit_host, submit_mount, submit_name) = sink!();
    let (cancel_host, cancel_mount, cancel_name) = sink!();
    let (clear_host, clear_mount, clear_name) = sink!();

    poodle_render::TextInputHandlers {
        on_change: Some(Arc::new(move |value: &str| {
            let value = value.to_owned();
            text_field_apply(
                &change_host,
                &change_mount,
                &change_name,
                format!("{change_name}/change:{value}"),
                |field| field.value = value,
            );
        })),
        on_selection_change: Some(Arc::new(move |start: usize, end: usize| {
            text_field_apply(
                &select_host,
                &select_mount,
                &select_name,
                format!("{select_name}/select:{start}-{end}"),
                |field| field.selection = (start, end),
            );
        })),
        on_focus_change: Some(Arc::new(move |focused: bool| {
            text_field_apply(
                &focus_host,
                &focus_mount,
                &focus_name,
                format!("{focus_name}/focus:{focused}"),
                |field| field.is_focused = focused,
            );
        })),
        on_submit: Some(Arc::new(move || {
            text_field_apply(
                &submit_host,
                &submit_mount,
                &submit_name,
                format!("{submit_name}/submit"),
                |_| {},
            );
        })),
        on_cancel: Some(Arc::new(move || {
            text_field_apply(
                &cancel_host,
                &cancel_mount,
                &cancel_name,
                format!("{cancel_name}/cancel"),
                |_| {},
            );
        })),
        on_clear: Some(Arc::new(move || {
            text_field_apply(
                &clear_host,
                &clear_mount,
                &clear_name,
                format!("{clear_name}/clear"),
                |_| {},
            );
        })),
    }
}

fn mount_text_fields<'a>(
    cx: &'a mut TestAppContext,
    host: &Arc<TextFieldHost>,
) -> (HeadlessDriver<'a>, Arc<Mutex<Node>>) {
    let mounted = Arc::new(Mutex::new(Node::container()));
    *mounted.lock().expect("mount") = text_field_tree(host, &mounted);
    let driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 200.0);
    (driver, mounted)
}

/// The node the backend actually painted for a field, read back out of the
/// rebuilt tree.
fn mounted_field(mounted: &Arc<Mutex<Node>>, name: &str) -> Node {
    mounted
        .lock()
        .expect("mount")
        .find(&|node| node.id.as_deref() == Some(field_id(name).as_str()))
        .cloned()
        .unwrap_or_else(|| panic!("{name} is mounted"))
}

fn mounted_text(mounted: &Arc<Mutex<Node>>, name: &str) -> String {
    let value = mounted
        .lock()
        .expect("mount")
        .find(&|node| node.id.as_deref() == Some(field_value_id(name).as_str()))
        .cloned()
        .unwrap_or_else(|| panic!("{name} has a value node"));
    match &value.kind {
        poodle_node::NodeKind::Text { content } => content.clone(),
        _ => panic!("{name}'s value node is text, so the field root stays the only input"),
    }
}

/// g16.007. TextInput core controlled editing through the real GPUI node,
/// backend and input path: focus is the backend's, the edit rules are shared
/// Rust's, and the value and caret are the host's — restated as props on every
/// frame. Nothing here invokes a handler, a transition or a renderer directly
/// after mount.
///
/// Deliberately not claimed: multiline layout, slug source/generation,
/// validation timing, OS input methods, and NumberInput's value model.
#[test]
fn text_input_controlled_editing_and_identity_rebuild_the_host_spec() {
    // ── Editing one controlled field ───────────────────────────────────────
    run_headless(|cx| {
        let host = TextFieldHost::new(vec![
            TextFieldState::new("name", "kick").limited(6),
            TextFieldState::new("note", "").with_placeholder("Describe it"),
        ]);
        let (mut driver, mounted) = mount_text_fields(cx, &host);
        driver.wait_for_focus_handle(&field_id("name"));

        // A field starts unfocused with a collapsed caret, and the placeholder
        // is drawn as text while being declared as *not* the value — without
        // that flag one layer down cannot tell the prompt from what was typed.
        assert!(!host.field("name").is_focused);
        assert_eq!(mounted_text(&mounted, "note"), "Describe it");
        assert_eq!(
            mounted_field(&mounted, "note")
                .find(&|node| node.id.as_deref() == Some(field_value_id("note").as_str()))
                .and_then(|node| node.caret)
                .map(|caret| caret.showing_placeholder),
            Some(true)
        );

        // Pointer focus: the press reaches the real focus handle, the backend
        // reports the gain, and the rebuilt spec draws the caret where the
        // click landed.
        driver.pointer_press(payload_frac(&field_value_id("name"), 0.0, 0.5));
        driver.pointer_release(payload_frac(&field_value_id("name"), 0.0, 0.5));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("name")),
            Some(true)
        );
        assert!(host.field("name").is_focused);
        assert_eq!(host.field("name").selection, (0, 0));
        assert!(
            host.log().iter().any(|entry| entry == "name/focus:true"),
            "the backend reported the focus gain: {:?}",
            host.log()
        );

        // Caret movement and printable input, both through the real key path.
        host.take_log();
        driver.dispatch_key_raw("end");
        assert_eq!(host.field("name").selection, (4, 4));
        driver.dispatch_key_raw("e");
        driver.dispatch_key_raw("r");
        assert_eq!(host.field("name").value, "kicker");
        assert_eq!(host.field("name").selection, (6, 6));
        assert_eq!(mounted_text(&mounted, "name"), "kicker");

        // The field is now at its declared limit: the key is consumed, and the
        // host is told nothing at all. The whole log, not a filtered view of
        // it — a rejected edit that still reports its unchanged caret is
        // indistinguishable from an accepted one.
        host.take_log();
        driver.dispatch_key_raw("s");
        assert_eq!(host.field("name").value, "kicker");
        assert_eq!(host.field("name").selection, (6, 6));
        assert_eq!(
            host.take_log(),
            Vec::<String>::new(),
            "a full field reports nothing on any channel"
        );

        // Same claim through a key that is ours but has nothing to do: the
        // caret is already at the end, so Delete is swallowed and silent.
        driver.dispatch_key_raw("delete");
        assert_eq!(
            host.take_log(),
            Vec::<String>::new(),
            "a consumed but inert key reports nothing"
        );

        // Shift-extend, then type over the selection.
        driver.dispatch_key_raw("home");
        for _ in 0..4 {
            driver.dispatch_key_raw("shift-right");
        }
        assert_eq!(host.field("name").selection, (0, 4));
        assert_eq!(host.field("name").value, "kicker", "extending never edits");
        driver.dispatch_key_raw("p");
        assert_eq!(host.field("name").value, "per");
        assert_eq!(host.field("name").selection, (1, 1));

        // Deletion in both directions.
        driver.dispatch_key_raw("backspace");
        assert_eq!(host.field("name").value, "er");
        driver.dispatch_key_raw("delete");
        assert_eq!(host.field("name").value, "r");
        assert_eq!(host.field("name").selection, (0, 0));

        // Commands report without touching the controlled value.
        host.take_log();
        driver.dispatch_key_raw("enter");
        driver.dispatch_key_raw("escape");
        assert_eq!(host.take_log(), vec!["name/submit", "name/cancel"]);
        assert_eq!(host.field("name").value, "r");
        assert_eq!(host.field("name").selection, (0, 0));

        // The placeholder is never the value: typing into an empty field
        // reports the typed character alone.
        driver.pointer_press(payload_frac(&field_value_id("note"), 0.0, 0.5));
        driver.pointer_release(payload_frac(&field_value_id("note"), 0.0, 0.5));
        driver.dispatch_key_raw("a");
        assert_eq!(host.field("note").value, "a");
        assert_eq!(mounted_text(&mounted, "note"), "a");
        assert_eq!(
            mounted_field(&mounted, "note")
                .find(&|node| node.id.as_deref() == Some(field_value_id("note").as_str()))
                .and_then(|node| node.caret)
                .map(|caret| caret.showing_placeholder),
            Some(false)
        );

        // Focus moving on reports the loss once, and the rebuild drops the
        // focus state without touching the value or the caret.
        let before = host.field("name");
        host.take_log();
        driver.blur_element_focus(&field_id("note"));
        assert!(!host.field("note").is_focused);
        assert_eq!(host.field("name").value, before.value);
        assert_eq!(host.field("name").selection, before.selection);
        assert_eq!(
            host.log()
                .iter()
                .filter(|entry| *entry == "note/focus:false")
                .count(),
            1,
            "the loss is reported exactly once: {:?}",
            host.log()
        );
    });

    // ── Search clear, disabled, and read-only ──────────────────────────────
    run_headless(|cx| {
        let host = TextFieldHost::new(vec![
            TextFieldState::new("query", "kick").searchable(),
            TextFieldState::new("locked", "sealed").searchable().disabled(),
            TextFieldState::new("frozen", "fixed").searchable().read_only(),
        ]);
        let (mut driver, mounted) = mount_text_fields(cx, &host);
        driver.wait_for_focus_handle(&field_id("query"));

        // Only a search field with a value, enabled and writable, offers the
        // clear control at all.
        assert!(mounted_field(&mounted, "query")
            .find(&|node| node.id.as_deref() == Some(field_clear_id("query").as_str()))
            .is_some());
        for inert in ["locked", "frozen"] {
            assert!(
                mounted_field(&mounted, inert)
                    .find(&|node| node.id.as_deref() == Some(field_clear_id(inert).as_str()))
                    .is_none(),
                "{inert} offers no clear control"
            );
        }

        // Clearing is two signals in one order: the empty value first, then
        // the command. Both kinds of host see the field empty.
        host.take_log();
        driver.pointer_activate_id(&field_clear_id("query"));
        assert_eq!(
            host.take_log()
                .into_iter()
                // The press also focuses the field it belongs to, which the
                // backend reports on the next paint. Everything else the host
                // hears is the clear itself, in order.
                .filter(|entry| !entry.ends_with("/focus:true"))
                .collect::<Vec<_>>(),
            vec!["query/change:".to_string(), "query/clear".to_string()]
        );
        assert_eq!(host.field("query").value, "");
        assert!(
            mounted_field(&mounted, "query")
                .find(&|node| node.id.as_deref() == Some(field_clear_id("query").as_str()))
                .is_none(),
            "an empty search field has nothing to clear"
        );

        // Disabled is inert: no focus handle exists at all, so nothing can be
        // focused, typed into, submitted or cancelled.
        assert!(
            poodle_gpui_node_backend::focus_handle_for(&field_id("locked")).is_none(),
            "a disabled field is not focusable"
        );
        assert!(mounted_field(&mounted, "locked").interaction.disabled);
        host.take_log();
        driver.pointer_activate_id(&field_id("locked"));
        driver.dispatch_key_raw("x");
        driver.dispatch_key_raw("enter");
        assert_eq!(host.field("locked").value, "sealed");
        assert_eq!(
            host.take_log()
                .into_iter()
                .filter(|entry| entry.starts_with("locked/"))
                .collect::<Vec<_>>(),
            Vec::<String>::new(),
            "a disabled field reports nothing"
        );

        // Read-only takes real focus and reports selection, but no keystroke
        // moves its value. Commands still reach the host.
        driver.wait_for_focus_handle(&field_id("frozen"));
        driver.focus_element(&field_id("frozen"));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("frozen")),
            Some(true),
            "a read-only field is still focusable"
        );
        host.take_log();
        driver.dispatch_key_raw("x");
        driver.dispatch_key_raw("backspace");
        assert_eq!(host.field("frozen").value, "fixed");
        driver.dispatch_key_raw("enter");
        driver.pointer_press(payload_frac(&field_value_id("frozen"), 0.9, 0.5));
        driver.pointer_release(payload_frac(&field_value_id("frozen"), 0.9, 0.5));
        let log = host.take_log();
        assert!(
            !log.iter().any(|entry| entry.starts_with("frozen/change")),
            "read-only never mutates: {log:?}"
        );
        assert!(
            log.iter().any(|entry| entry == "frozen/submit"),
            "read-only still submits: {log:?}"
        );
        assert!(
            log.iter().any(|entry| entry.starts_with("frozen/select")),
            "read-only still selects: {log:?}"
        );
    });

    // ── Two fields with equal values keep their own identity ───────────────
    run_headless(|cx| {
        let host = TextFieldHost::new(vec![
            TextFieldState::new("left", "same"),
            TextFieldState::new("right", "same"),
        ]);
        let (mut driver, _mounted) = mount_text_fields(cx, &host);
        driver.wait_for_focus_handle(&field_id("left"));
        driver.wait_for_focus_handle(&field_id("right"));

        driver.focus_element(&field_id("left"));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("left")),
            Some(true)
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("right")),
            Some(false),
            "equal values do not merge focus identity"
        );

        driver.dispatch_key_raw("end");
        driver.dispatch_key_raw("x");
        assert_eq!(host.field("left").value, "samex");
        assert_eq!(host.field("right").value, "same");

        driver.focus_element(&field_id("right"));
        driver.dispatch_key_raw("home");
        driver.dispatch_key_raw("y");
        assert_eq!(host.field("right").value, "ysame");
        assert_eq!(host.field("left").value, "samex");
        assert_eq!(host.field("left").selection, (5, 5));
        assert_eq!(host.field("right").selection, (1, 1));

        // Undo history is per field too — the backend keys it by the value
        // node's id, which is derived from the field's own.
        driver.focus_element(&field_id("left"));
        driver.dispatch_key_raw("cmd-z");
        assert_eq!(host.field("left").value, "same");
        assert_eq!(
            host.field("right").value,
            "ysame",
            "one field's undo leaves the other alone"
        );
    });
}

// ── g16.008 native text event routing ──────────────────────────────────────
//
// Two generic backend defects, proven through mounted dispatch rather than
// through the components that exposed them: Tab was routed to the submit
// channel, and the blur reset cleared transient text state under the focused
// *root* id while a composite field paints its value under a derived child.
//
// Every traversal claim below drives real key dispatch through gpui's own
// tab-stop order. Nothing calls a component handler, a focus helper or a
// transition directly.

/// One ordered sequence of everything the mounted hosts were told.
type EventLog = Arc<Mutex<Vec<String>>>;

fn event_log() -> EventLog {
    Arc::new(Mutex::new(Vec::new()))
}

fn take_events(log: &EventLog) -> Vec<String> {
    std::mem::take(&mut *log.lock().expect("log"))
}

fn note(log: &EventLog, entry: String) {
    log.lock().expect("log").push(entry);
}

/// A TextInput used as an observable traversal bookend. It carries an explicit
/// id, so it owns a retrievable focus handle, and it reports both focus
/// directions and any submit — which is how a Tab-to-submit remap shows up.
fn traversal_marker(name: &str, log: &EventLog, ctx: &RenderContext<'_>) -> Node {
    let spec = poodle_specs::TextInputSpec::new()
        .with_id(name)
        .with_aria_label(name)
        .with_value("");
    let focus_log = Arc::clone(log);
    let focus_name = name.to_owned();
    let submit_log = Arc::clone(log);
    let submit_name = name.to_owned();
    poodle_render::text_input_with_handlers(
        &spec,
        ctx,
        poodle_render::TextInputHandlers {
            on_focus_change: Some(Arc::new(move |focused: bool| {
                note(&focus_log, format!("{focus_name}/focus:{focused}"));
            })),
            on_submit: Some(Arc::new(move || {
                note(&submit_log, format!("{submit_name}/submit"));
            })),
            ..poodle_render::TextInputHandlers::default()
        },
    )
}

fn routing_column(children: Vec<Node>) -> Node {
    let mut column = Node::container();
    column.id = Some(FIXTURE_ID.to_owned());
    column.style.descriptor.layout.direction = LayoutDirection::Column;
    column.style.descriptor.layout.spacing.gap = 8.0;
    column.style.descriptor.layout.width = LayoutSizing::Fixed(360.0);
    for child in children {
        column = column.child(child);
    }
    column
}

// ── CodeInput ──────────────────────────────────────────────────────────────

struct CodeRouting {
    value: Mutex<String>,
    log: EventLog,
}

fn code_routing_tree(host: &Arc<CodeRouting>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let spec = poodle_specs::CodeInputSpec::new()
        .with_length(4)
        .with_value(host.value.lock().expect("code").clone())
        .with_aria_label("code");

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let complete_host = Arc::clone(host);
    let code = poodle_render::code_input_with_handlers(
        &spec,
        &ctx,
        poodle_render::CodeInputHandlers {
            on_value_change: Some(Arc::new(move |next: &str| {
                *change_host.value.lock().expect("code") = next.to_owned();
                note(&change_host.log, format!("code/change:{next}"));
                let tree = code_routing_tree(&change_host, &change_mount);
                *change_mount.lock().expect("mount") = tree;
            })),
            on_complete: Some(Arc::new(move |next: &str| {
                note(&complete_host.log, format!("code/complete:{next}"));
            })),
            ..poodle_render::CodeInputHandlers::default()
        },
    );
    routing_column(vec![
        traversal_marker("code-before", &host.log, &ctx),
        code,
        traversal_marker("code-after", &host.log, &ctx),
    ])
}

// ── DurationInput ──────────────────────────────────────────────────────────

struct DurationRouting {
    segments: Mutex<(u32, u32, u32)>,
    last_total: Mutex<Option<u64>>,
    show_seconds: bool,
    max_hours: u32,
    disabled: bool,
    log: EventLog,
}

fn duration_host(hours: u32, minutes: u32, seconds: u32) -> DurationRouting {
    DurationRouting {
        segments: Mutex::new((hours, minutes, seconds)),
        last_total: Mutex::new(None),
        show_seconds: true,
        max_hours: 99,
        disabled: false,
        log: event_log(),
    }
}

fn duration_routing_tree(host: &Arc<DurationRouting>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let (hours, minutes, seconds) = *host.segments.lock().expect("segments");
    let spec = poodle_specs::DurationInputSpec::new()
        .with_show_seconds(host.show_seconds)
        .with_segments(hours, minutes, seconds)
        .with_max_hours(host.max_hours)
        .with_disabled(host.disabled)
        .with_aria_label("duration");

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let duration = poodle_render::duration_input_with_handlers(
        &spec,
        &ctx,
        poodle_render::DurationInputHandlers {
            on_change: Some(Arc::new(move |h: u32, m: u32, s: u32, total: u64| {
                *change_host.segments.lock().expect("segments") = (h, m, s);
                *change_host.last_total.lock().expect("total") = Some(total);
                note(
                    &change_host.log,
                    format!("duration/change:{h}:{m}:{s}:{total}"),
                );
                let tree = duration_routing_tree(&change_host, &change_mount);
                *change_mount.lock().expect("mount") = tree;
            })),
        },
    );
    routing_column(vec![
        traversal_marker("duration-before", &host.log, &ctx),
        duration,
        traversal_marker("duration-after", &host.log, &ctx),
    ])
}

// ── EditableLabel ──────────────────────────────────────────────────────────

struct LabelRouting {
    /// The element id this mount stamps on the label. Each scenario below uses
    /// its own, because focus handles and painted state are keyed by id and
    /// outlive a single test-platform app.
    id: String,
    /// The committed value, which is what the host owns.
    value: Mutex<String>,
    /// The value the field currently shows — the draft, until it commits.
    draft: Mutex<String>,
    selection: Mutex<(usize, usize)>,
    editing: Mutex<bool>,
    request_focus: Mutex<bool>,
    last_previous: Mutex<Option<String>>,
    log: EventLog,
}

fn label_routing_tree(host: &Arc<LabelRouting>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let editing = *host.editing.lock().expect("editing");
    let value = host.value.lock().expect("value").clone();
    let draft = host.draft.lock().expect("draft").clone();
    let selection = *host.selection.lock().expect("selection");
    let restore = {
        let mut flag = host.request_focus.lock().expect("request_focus");
        let restore = *flag;
        *flag = false;
        restore
    };
    let spec = poodle_specs::EditableLabelSpec::new()
        .with_value(&value)
        .with_draft_value(editing.then_some(draft.clone()))
        .with_editing(editing)
        .with_selection(selection.0, selection.1)
        .with_request_focus(restore)
        .with_aria_label("track name");
    let id = host.id.clone();

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let select_host = Arc::clone(host);
    let select_mount = Arc::clone(mounted);
    let commit_host = Arc::clone(host);
    let commit_mount = Arc::clone(mounted);
    let cancel_host = Arc::clone(host);
    let cancel_mount = Arc::clone(mounted);
    let restore_host = Arc::clone(host);
    let restore_mount = Arc::clone(mounted);
    let mut label = poodle_render::editable_label_with_handlers(
        &spec,
        &ctx,
        poodle_render::EditableLabelHandlers {
            on_change: Some(Arc::new(move |next: &str| {
                *change_host.draft.lock().expect("draft") = next.to_owned();
                note(&change_host.log, format!("label/change:{next}"));
                let tree = label_routing_tree(&change_host, &change_mount);
                *change_mount.lock().expect("mount") = tree;
            })),
            on_selection_change: Some(Arc::new(move |start: usize, end: usize| {
                *select_host.selection.lock().expect("selection") = (start, end);
                let tree = label_routing_tree(&select_host, &select_mount);
                *select_mount.lock().expect("mount") = tree;
            })),
            on_commit: Some(Arc::new(move |next: &str, previous: &str| {
                if !*commit_host.editing.lock().expect("editing") {
                    return;
                }
                *commit_host.editing.lock().expect("editing") = false;
                *commit_host.value.lock().expect("value") = next.to_owned();
                *commit_host.last_previous.lock().expect("previous") = Some(previous.to_owned());
                note(&commit_host.log, format!("label/commit:{next}"));
                let tree = label_routing_tree(&commit_host, &commit_mount);
                *commit_mount.lock().expect("mount") = tree;
            })),
            on_cancel: Some(Arc::new(move || {
                if !*cancel_host.editing.lock().expect("editing") {
                    return;
                }
                *cancel_host.editing.lock().expect("editing") = false;
                let restored = cancel_host.value.lock().expect("value").clone();
                *cancel_host.draft.lock().expect("draft") = restored;
                note(&cancel_host.log, "label/cancel".to_owned());
                let tree = label_routing_tree(&cancel_host, &cancel_mount);
                *cancel_mount.lock().expect("mount") = tree;
            })),
            on_restore_display_focus: Some(Arc::new(move || {
                *restore_host.request_focus.lock().expect("request_focus") = true;
                note(&restore_host.log, "label/restore".to_owned());
                let tree = label_routing_tree(&restore_host, &restore_mount);
                *restore_mount.lock().expect("mount") = tree;
            })),
            ..poodle_render::EditableLabelHandlers::default()
        },
    );
    // EditableLabel carries no id prop, and the backend keys focus, bounds and
    // painted text state by element id. The fixture stamps one so both modes
    // are the same identity, exactly as a host with a real row key would.
    label.id = Some(id.clone());
    routing_column(vec![
        traversal_marker(&format!("{id}-before"), &host.log, &ctx),
        label,
        traversal_marker(&format!("{id}-after"), &host.log, &ctx),
    ])
}

fn label_host(id: &str) -> Arc<LabelRouting> {
    Arc::new(LabelRouting {
        id: id.to_owned(),
        value: Mutex::new("Kick".to_owned()),
        draft: Mutex::new("Kick".to_owned()),
        selection: Mutex::new((4, 4)),
        editing: Mutex::new(true),
        request_focus: Mutex::new(false),
        last_previous: Mutex::new(None),
        log: event_log(),
    })
}

fn painted_label_input(mounted: &Arc<Mutex<Node>>, id: &str) -> String {
    let tree = mounted.lock().expect("mount");
    tree.find(&|node| node.id.as_deref() == Some(id))
        .and_then(|node| match &node.kind {
            NodeKind::Input { value, .. } => Some(value.clone()),
            _ => None,
        })
        .expect("editing label paints an input")
}

// ── A childless editable input ─────────────────────────────────────────────

const DIRECT_ID: &str = "direct-field";

/// The other input shape: a node that draws its own value, with no derived
/// value child. Native `EditableLabel`'s editing field is one, and so is any
/// plain `Node::input` with a caret. Built from the vocabulary directly, so
/// the shape is the subject rather than one component's spelling of it.
struct DirectInput {
    value: Mutex<String>,
    selection: Mutex<(usize, usize)>,
    log: EventLog,
}

fn direct_input_tree(host: &Arc<DirectInput>, mounted: &Arc<Mutex<Node>>) -> Node {
    let value = host.value.lock().expect("value").clone();
    let selection = *host.selection.lock().expect("selection");

    let mut input = Node::input(value.clone(), "");
    input.id = Some(DIRECT_ID.to_owned());
    input.interaction.focusable = true;
    input.style.descriptor.layout.width = LayoutSizing::Fixed(200.0);
    // A focus treatment is what makes the backend track this node's real
    // handle, the same way TextInput's focus ring does.
    input.style.focus = Some(poodle_node::StylePatch {
        border_color: Some(ColorValue(0.3, 0.6, 1.0, 1.0)),
        ..poodle_node::StylePatch::default()
    });
    let mut input = input.with_caret(
        selection,
        ColorValue(1.0, 1.0, 1.0, 1.0),
        ColorValue(0.3, 0.6, 1.0, 0.4),
    );

    let apply = {
        let host = Arc::clone(host);
        let mounted = Arc::clone(mounted);
        move |next: String, next_selection: (usize, usize), entry: String| {
            *host.value.lock().expect("value") = next;
            *host.selection.lock().expect("selection") = next_selection;
            note(&host.log, entry);
            let tree = direct_input_tree(&host, &mounted);
            *mounted.lock().expect("mount") = tree;
        }
    };

    let key_apply = apply.clone();
    let key_value = value.clone();
    input.interaction.on_edit_key = Some(Arc::new(move |key: &str, mods| {
        let state = poodle_headless::text_input::EditState {
            anchor: selection.0,
            head: selection.1,
        };
        let Some(outcome) = poodle_headless::text_input::edit_transition(
            &key_value, state, key, mods.shift, mods.accel, None,
        ) else {
            return;
        };
        let next = outcome.value.clone().unwrap_or_else(|| key_value.clone());
        let next_selection = (outcome.state.anchor, outcome.state.head);
        key_apply(next.clone(), next_selection, format!("direct/key:{next}"));
    }));

    // Undo restores a snapshot: the value and the caret together, reported by
    // the backend on those two channels in that order, from the history it
    // keeps for this field.
    let text_apply = apply.clone();
    input.interaction.on_text_change = Some(Arc::new(move |next: &str| {
        let next = next.to_owned();
        text_apply(next.clone(), selection, format!("direct/text:{next}"));
    }));
    // The caret channel moves the caret and nothing else: an undo reports the
    // restored value first and its selection second, so re-stating the value
    // here would put the pre-undo one back.
    let select_host = Arc::clone(host);
    let select_mount = Arc::clone(mounted);
    input.interaction.on_select_range = Some(Arc::new(
        move |start: usize, end: usize, _granularity| {
            *select_host.selection.lock().expect("selection") = (start, end);
            note(&select_host.log, format!("direct/select:{start}-{end}"));
            let tree = direct_input_tree(&select_host, &select_mount);
            *select_mount.lock().expect("mount") = tree;
        },
    ));

    let log = Arc::clone(&host.log);
    routing_column(vec![
        input,
        traversal_marker("direct-after", &log, &RenderContext::new(&theme())),
    ])
}

/// g16.008. Enter is the submit gesture and Tab is traversal. Two controlled
/// fields, real key dispatch, gpui's own tab-stop order: Tab moves focus and
/// reports nothing else, Shift+Tab moves back, and Enter and Escape still
/// reach the host on the field that holds focus.
///
/// Deliberately not claimed: which visual treatment a traversed field draws,
/// NumberInput's value model, multiline, or IME behavior beyond the marked
/// range asserted below.
#[test]
fn text_input_submits_on_enter_and_traverses_on_tab() {
    run_headless(|cx| {
        let host = TextFieldHost::new(vec![
            TextFieldState::new("first", "kick"),
            TextFieldState::new("second", "snare"),
        ]);
        let (mut driver, _mounted) = mount_text_fields(cx, &host);
        driver.wait_for_focus_handle(&field_id("first"));
        driver.wait_for_focus_handle(&field_id("second"));
        driver.focus_element(&field_id("first"));
        host.take_log();

        // Enter submits, exactly once, and never falls through to the edit
        // transition: the value the host holds is untouched.
        driver.dispatch_key_raw("enter");
        assert_eq!(host.take_log(), vec!["first/submit"]);
        assert_eq!(host.field("first").value, "kick");

        // Tab traverses. The only thing the host hears is the focus moving —
        // no submit on the field being left, and no value change anywhere.
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("first")),
            Some(false)
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("second")),
            Some(true)
        );
        assert_eq!(
            host.take_log(),
            vec!["first/focus:false", "second/focus:true"],
            "Tab is traversal: no submit, no edit, no selection report"
        );
        assert_eq!(host.field("first").value, "kick");
        assert_eq!(host.field("second").value, "snare");

        // Shift+Tab walks the same order backwards.
        driver.dispatch_key_raw("shift-tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("first")),
            Some(true)
        );
        assert_eq!(
            host.take_log(),
            // Tree order, not gain-then-loss order: both fields observe their
            // own handle as the frame paints them, and `first` paints first.
            vec!["first/focus:true", "second/focus:false"]
        );

        // Enter and Escape still belong to whichever field holds focus.
        driver.dispatch_key_raw("enter");
        driver.dispatch_key_raw("escape");
        assert_eq!(host.take_log(), vec!["first/submit", "first/cancel"]);
        assert_eq!(host.field("first").value, "kick");
        assert_eq!(host.field("second").value, "snare");

        // A disabled field is not a tab stop at all: traversal from the last
        // enabled field wraps back to the first rather than stopping on it.
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&field_id("first")),
            Some(true),
            "two stops in a two-field fixture wraps to the start"
        );
    });
}

/// g16.008. Transient text state is keyed by the node that *paints* the value,
/// and the two input shapes disagree about which node that is. Proven from
/// both sides in one mount: a composite `TextInput`, whose value is a derived
/// child, and a childless input, which draws its own.
///
/// Deliberately not claimed: measured-line contents, pixel positions, or that
/// a still-mounted field stops measuring after blur — it repaints, and
/// re-measures, exactly as it should.
#[test]
fn blur_clears_the_painted_field_state_and_keeps_its_undo_history() {
    // ── Composite: the value is a derived child ────────────────────────────
    run_headless(|cx| {
        let host = TextFieldHost::new(vec![TextFieldState::new("name", "kick")]);
        let (mut driver, _mounted) = mount_text_fields(cx, &host);
        driver.wait_for_focus_handle(&field_id("name"));
        driver.focus_element(&field_id("name"));
        driver.dispatch_key_raw("end");
        driver.dispatch_key_raw("e");
        driver.dispatch_key_raw("r");
        assert_eq!(host.field("name").value, "kicker");

        // The field root holds focus and takes the keys; none of the painted
        // state hangs off it.
        let root = poodle_gpui_node_backend::painted_text_state_for(&field_id("name"));
        assert_eq!(
            root,
            poodle_gpui_node_backend::PaintedTextState::default(),
            "the composite root paints no value, so it owns no text state"
        );

        // The value child does, including the history the edits recorded.
        let value = poodle_gpui_node_backend::painted_text_state_for(&field_value_id("name"));
        assert!(value.measured, "the value child is the node that measures");
        assert!(value.blinking, "and the node whose caret blinks");
        assert!(value.history, "and the node that records what was typed");

        // An input method marks the same key. The old blur reset cleared the
        // root id, so this survived a focus change and was spliced over the
        // next field to take the caret.
        poodle_gpui_node_backend::mark_composing(&field_value_id("name"), (6, 6), "\u{3053}");
        assert!(poodle_gpui_node_backend::painted_text_state_for(&field_value_id("name")).composing);

        driver.blur_element_focus(&field_id("name"));
        let after = poodle_gpui_node_backend::painted_text_state_for(&field_value_id("name"));
        assert!(!after.composing && !after.marked, "blur ends the composition");
        assert!(!after.blinking, "and the blink epoch it started");
        assert!(!after.scrolled, "and the scroll it was holding");
        assert!(
            after.history,
            "history is mounted-lifetime state: focus moving is not a reason to forget it"
        );
        assert_eq!(
            poodle_gpui_node_backend::take_composing(&field_value_id("name")),
            None
        );

        // And the retained history is reachable again after refocus.
        driver.focus_element(&field_id("name"));
        host.take_log();
        driver.dispatch_key_raw("cmd-z");
        assert_eq!(
            host.field("name").value,
            // One typing run is one entry, so undo lands on what the field
            // held before it — reached back across the focus excursion.
            "kick",
            "undo reaches back across the focus excursion"
        );
    });

    // ── Childless: the input draws its own value ──────────────────────────
    run_headless(|cx| {
        let host = Arc::new(DirectInput {
            value: Mutex::new("kick".to_owned()),
            selection: Mutex::new((4, 4)),
            log: event_log(),
        });
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = direct_input_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 200.0);
        driver.wait_for_focus_handle(DIRECT_ID);
        driver.focus_element(DIRECT_ID);
        driver.dispatch_key_raw("s");
        assert_eq!(*host.value.lock().expect("value"), "kicks");

        // Nothing is painted under a derived child here, because there is no
        // child: keys, focus and paint all land on the same id. Deriving
        // `<id>-value` addressed a node that never existed, so undo looked up
        // an empty history.
        assert_eq!(
            poodle_gpui_node_backend::painted_text_state_for(&format!("{DIRECT_ID}-value")),
            poodle_gpui_node_backend::PaintedTextState::default()
        );
        let painted = poodle_gpui_node_backend::painted_text_state_for(DIRECT_ID);
        assert!(painted.measured && painted.blinking && painted.history);

        take_events(&host.log);
        driver.dispatch_key_raw("cmd-z");
        assert_eq!(
            *host.value.lock().expect("value"),
            "kick",
            "the keystroke side and the paint side address the same history"
        );

        // The same blur reset, on the same shape's own id.
        poodle_gpui_node_backend::mark_composing(DIRECT_ID, (4, 4), "\u{3053}");
        driver.blur_element_focus(DIRECT_ID);
        let after = poodle_gpui_node_backend::painted_text_state_for(DIRECT_ID);
        assert!(!after.composing && !after.marked && !after.blinking && !after.scrolled);
        assert!(after.history);

        // Two fields with distinct ids never share either.
        assert_eq!(
            poodle_gpui_node_backend::painted_text_state_for("poodle-input-direct-after"),
            poodle_gpui_node_backend::PaintedTextState::default(),
            "the neighbouring field kept none of this field's state"
        );
    });
}

/// g16.008. CodeInput and DurationInput are traversed, not typed into, by Tab.
/// Both put their key handlers on plain containers — a slot row and one
/// container per segment — so a generic Tab remap reaches them through the
/// same channel a real keystroke does.
///
/// Deliberately not claimed: which slot or segment draws the active treatment,
/// paste, or the completion check's own contract.
#[test]
fn code_and_duration_inputs_traverse_on_tab_without_mutating() {
    // ── CodeInput: one stop, no completion, no value change ───────────────
    run_headless(|cx| {
        let host = Arc::new(CodeRouting {
            value: Mutex::new("123".to_owned()),
            log: event_log(),
        });
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = code_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-code-before");
        driver.focus_element("poodle-input-code-before");
        take_events(&host.log);

        // Into the slot row, then straight out of it. One key away from a full
        // code, and neither Tab completes it.
        driver.dispatch_key_raw("tab");
        assert_eq!(take_events(&host.log), vec!["code-before/focus:false"]);
        driver.dispatch_key_raw("tab");
        assert_eq!(take_events(&host.log), vec!["code-after/focus:true"]);
        assert_eq!(*host.value.lock().expect("code"), "123");

        // The row really was the stop in between: coming back to it, a digit
        // types and completes, through the same dispatch path.
        driver.dispatch_key_raw("shift-tab");
        take_events(&host.log);
        driver.dispatch_key_raw("4");
        assert_eq!(*host.value.lock().expect("code"), "1234");
        assert_eq!(
            take_events(&host.log),
            vec!["code/change:1234", "code/complete:1234"]
        );
    });

    // ── DurationInput: through its segments, in order, then out ───────────
    run_headless(|cx| {
        let host = Arc::new(duration_host(1, 2, 3));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");
        take_events(&host.log);

        // Four Tabs cross the whole control: hours, minutes, seconds, then out
        // to the field after it. The root frames the segments and is not a
        // stop of its own, and not one of the four changes a segment.
        for _ in 0..4 {
            driver.dispatch_key_raw("tab");
        }
        assert_eq!(*host.segments.lock().expect("segments"), (1, 2, 3));
        assert_eq!(
            take_events(&host.log),
            vec!["duration-before/focus:false", "duration-after/focus:true"],
            "traversal crosses the segments without any of them reporting"
        );

        // Which stop was which, proven by what the arrow key acts on: back
        // through the same order, incrementing whichever segment holds focus.
        for expected in [(1, 2, 4), (1, 3, 4), (2, 3, 4)] {
            driver.dispatch_key_raw("shift-tab");
            driver.dispatch_key_raw("up");
            assert_eq!(
                *host.segments.lock().expect("segments"),
                expected,
                "one Shift+Tab is one segment, in seconds/minutes/hours order"
            );
        }

        // One more Shift+Tab off Hours leaves the control the way it came in —
        // straight to the field before it, with nothing in between.
        take_events(&host.log);
        driver.dispatch_key_raw("shift-tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-duration-before"),
            Some(true),
            "Hours is the control's entry stop in both directions"
        );
        assert_eq!(*host.segments.lock().expect("segments"), (2, 3, 4));
    });
}

/// g16.009. DurationInput's three segments are the only host value. Display,
/// carry/borrow, digit entry, max-hours swallowing, callback totals, visible
/// traversal, and disabled inertia all go through production focus/key
/// dispatch and a host rebuild from those fields.
///
/// Deliberately not claimed: IME, free-form parsing, selection ranges, native
/// accessibility, visual comparison, or Jetstream admission.
#[test]
fn duration_input_segments_edit_and_rebuild_the_host_spec() {
    fn last_total(host: &DurationRouting) -> u64 {
        host.last_total
            .lock()
            .expect("total")
            .expect("a change reported a total")
    }

    // ── Carry, borrow, digit-shift, max-hours, callback totals ────────────
    run_headless(|cx| {
        let host = Arc::new(duration_host(0, 59, 59));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");
        take_events(&host.log);

        // Hours is the entry stop. One Tab lands there; ArrowUp steps hours.
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(*host.segments.lock().expect("segments"), (1, 59, 59));
        assert_eq!(last_total(&host), 7199);

        // Minutes carry into hours through the same dispatch path.
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(*host.segments.lock().expect("segments"), (2, 0, 59));
        assert_eq!(last_total(&host), 7259);

        // Seconds carry into minutes.
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(*host.segments.lock().expect("segments"), (2, 1, 0));
        assert_eq!(last_total(&host), 7260);
        assert_eq!(
            take_events(&host.log),
            vec![
                "duration-before/focus:false",
                "duration/change:1:59:59:7199",
                "duration/change:2:0:59:7259",
                "duration/change:2:1:0:7260",
            ]
        );

        // One more Tab leaves the control.
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-duration-after"),
            Some(true)
        );
    });

    run_headless(|cx| {
        let host = Arc::new(duration_host(1, 0, 0));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");

        // Borrow: Seconds ArrowDown walks minutes and hours back.
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("down");
        assert_eq!(*host.segments.lock().expect("segments"), (0, 59, 59));
        assert_eq!(last_total(&host), 3599);
    });

    run_headless(|cx| {
        let host = Arc::new(duration_host(0, 4, 0));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("5");
        assert_eq!(*host.segments.lock().expect("segments"), (0, 45, 0));
        assert_eq!(last_total(&host), 2700);
    });

    run_headless(|cx| {
        let mut host = duration_host(9, 59, 0);
        host.max_hours = 9;
        let host = Arc::new(host);
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");

        // Carry at max hours is swallowed: minutes wrap, hours stay.
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(*host.segments.lock().expect("segments"), (9, 0, 0));
        assert_eq!(last_total(&host), 32400);

        // Hours themselves clamp at the bound.
        driver.dispatch_key_raw("shift-tab");
        take_events(&host.log);
        driver.dispatch_key_raw("up");
        assert_eq!(*host.segments.lock().expect("segments"), (9, 0, 0));
        assert!(
            take_events(&host.log).is_empty(),
            "a swallowed hours step reports nothing"
        );
    });

    // ── show_seconds=false: two stops, seconds retained in state/payload ──
    run_headless(|cx| {
        let mut host = duration_host(1, 2, 3);
        host.show_seconds = false;
        let host = Arc::new(host);
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(
            *host.segments.lock().expect("segments"),
            (1, 3, 3),
            "hidden seconds stay in the host value"
        );
        assert_eq!(last_total(&host), 3783);

        take_events(&host.log);
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-duration-after"),
            Some(true),
            "without Seconds, the second Tab leaves the control"
        );
        assert_eq!(
            take_events(&host.log),
            vec!["duration-after/focus:true"],
            "the third stop is the field after, not a hidden Seconds segment"
        );
    });

    // ── Disabled: no segment stops, no change ─────────────────────────────
    run_headless(|cx| {
        let mut host = duration_host(1, 2, 3);
        host.disabled = true;
        let host = Arc::new(host);
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = duration_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-duration-before");
        driver.focus_element("poodle-input-duration-before");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-duration-after"),
            Some(true),
            "a disabled DurationInput exposes no segment stops"
        );
        driver.dispatch_key_raw("up");
        assert_eq!(*host.segments.lock().expect("segments"), (1, 2, 3));
        assert!(host.last_total.lock().expect("total").is_none());
        assert_eq!(
            take_events(&host.log),
            vec!["duration-before/focus:false", "duration-after/focus:true"]
        );
    });
}

struct TimeRouting {
    context: Arc<Mutex<TimeInputContext>>,
    last_emit: Mutex<Option<Option<String>>>,
    log: EventLog,
}

fn time_host(committed: Option<&str>) -> TimeRouting {
    TimeRouting {
        context: Arc::new(Mutex::new(TimeInputContext {
            committed: committed.map(str::to_string),
            ..TimeInputContext::default()
        })),
        last_emit: Mutex::new(None),
        log: event_log(),
    }
}

fn time_routing_tree(host: &Arc<TimeRouting>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let context = host.context.lock().expect("context").clone();
    let spec = TimeInputSpec::new()
        .with_aria_label("time")
        .with_step(context.step as u32)
        .with_disabled(context.disabled);
    let mut spec = spec;
    if let Some(value) = context.committed.as_deref() {
        spec = spec.with_value(value);
    }
    if let Some(min) = context.min.as_deref() {
        spec = spec.with_min(min);
    }
    if let Some(max) = context.max.as_deref() {
        spec = spec.with_max(max);
    }

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let live = Arc::clone(&host.context);
    let time = time_input_with_persistent_context(
        &spec,
        &ctx,
        live,
        Some({
            let host = Arc::clone(host);
            Arc::new(move |value: &str| {
                let value = if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                };
                *host.last_emit.lock().expect("emit") = Some(value.clone());
                note(
                    &host.log,
                    format!("time/change:{}", value.as_deref().unwrap_or("null")),
                );
            })
        }),
        Some(Arc::new(move |_next: TimeInputContext| {
            let tree = time_routing_tree(&change_host, &change_mount);
            *change_mount.lock().expect("mount") = tree;
        })),
    );

    let after_host = Arc::clone(host);
    let after_mount = Arc::clone(mounted);
    let after_log = Arc::clone(&host.log);
    let mut after = traversal_marker("time-after", &host.log, &ctx);
    after.interaction.on_focus_change = Some(Arc::new(move |focused: bool| {
        note(&after_log, format!("time-after/focus:{focused}"));
        if !focused {
            return;
        }
        let current = after_host.context.lock().expect("context").clone();
        if current.draft.is_none() {
            return;
        }
        let (next, effects) = time_input_transition(current, TimeInputEvent::Blur);
        *after_host.context.lock().expect("context") = next;
        for effect in effects {
            let poodle_headless::time_input::TimeInputEffect::EmitValueChange { value } = effect;
            *after_host.last_emit.lock().expect("emit") = Some(value.clone());
            note(
                &after_host.log,
                format!("time/change:{}", value.as_deref().unwrap_or("null")),
            );
        }
        let tree = time_routing_tree(&after_host, &after_mount);
        *after_mount.lock().expect("mount") = tree;
    }));

    routing_column(vec![
        traversal_marker("time-before", &host.log, &ctx),
        time,
        after,
    ])
}

/// g16.029. TimeInput's segmented GPUI editor routes mounted key/focus
/// dispatch through the shared machine: live commit, local invalid drafts,
/// blur/Escape revert, clear, step, bounds, conditional seconds, replacement,
/// Tab traversal, and disabled inertia.
///
/// Deliberately not claimed: IME, locale/12-hour presentation, picker overlays,
/// native accessibility proof, visual comparison, or Jetstream admission.
#[test]
fn time_input_segmented_editor_commits_drafts_and_bounds() {
    fn committed(host: &TimeRouting) -> Option<String> {
        host.context.lock().expect("context").committed.clone()
    }

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("14:30")));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("1");
        assert!(host.last_emit.lock().expect("emit").is_none());
        assert!(time_input_invalid(&host.context.lock().expect("context")));

        driver.dispatch_key_raw("5");
        assert_eq!(committed(&host).as_deref(), Some("15:30"));
        assert_eq!(
            host.last_emit.lock().expect("emit").clone(),
            Some(Some("15:30".into()))
        );
        assert!(!time_input_invalid(&host.context.lock().expect("context")));

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-time-after"),
            Some(true)
        );
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("09:00")));
        {
            let mut context = host.context.lock().expect("context");
            context.step = 300.0;
            context.min = Some("08:00".into());
            context.max = Some("18:00".into());
        }
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("0");
        driver.dispatch_key_raw("7");
        assert!(host.last_emit.lock().expect("emit").is_none());
        assert!(time_input_invalid(&host.context.lock().expect("context")));
        assert_eq!(committed(&host).as_deref(), Some("09:00"));

        driver.dispatch_key_raw("escape");
        assert!(!time_input_invalid(&host.context.lock().expect("context")));
        assert_eq!(committed(&host).as_deref(), Some("09:00"));
        assert!(host.last_emit.lock().expect("emit").is_none());
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("09:00")));
        {
            let mut context = host.context.lock().expect("context");
            context.step = 300.0;
        }
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("0");
        driver.dispatch_key_raw("7");
        assert!(time_input_invalid(&host.context.lock().expect("context")));

        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-time-after"),
            Some(true)
        );
        assert!(!time_input_invalid(&host.context.lock().expect("context")));
        assert_eq!(committed(&host).as_deref(), Some("09:00"));
        assert!(host.last_emit.lock().expect("emit").is_none());
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("14:30")));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(committed(&host).as_deref(), Some("14:31"));

        driver.dispatch_key_raw("backspace");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("backspace");
        assert_eq!(committed(&host).as_deref(), None);
        assert_eq!(
            host.last_emit.lock().expect("emit").clone(),
            Some(None)
        );
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("23:30")));
        {
            let mut context = host.context.lock().expect("context");
            context.min = Some("22:00".into());
            context.max = Some("06:00".into());
            context.step = 1800.0;
        }
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(committed(&host).as_deref(), Some("00:00"));
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("18:00")));
        {
            let mut context = host.context.lock().expect("context");
            context.min = Some("08:00".into());
            context.max = Some("18:00".into());
        }
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(committed(&host).as_deref(), Some("18:00"));
        assert!(
            !take_events(&host.log).iter().any(|entry| entry.starts_with("time/change:")),
            "linear max does not emit a duplicate bound step"
        );
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("09:30:00")));
        host.context.lock().expect("context").step = 15.0;
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("up");
        assert_eq!(committed(&host).as_deref(), Some("09:30:15"));

        take_events(&host.log);
        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-time-after"),
            Some(true)
        );
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("14:30")));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");

        driver.dispatch_key_raw("tab");
        driver.dispatch_key_raw("2");
        assert!(time_input_invalid(&host.context.lock().expect("context")));

        let (next, effects) = time_input_transition(
            host.context.lock().expect("context").clone(),
            TimeInputEvent::Replace {
                value: Some("08:00".into()),
            },
        );
        assert!(effects.is_empty());
        *host.context.lock().expect("context") = next;
        let tree = time_routing_tree(&host, &mounted);
        *mounted.lock().expect("mount") = tree;
        assert_eq!(committed(&host).as_deref(), Some("08:00"));
        assert!(!time_input_invalid(&host.context.lock().expect("context")));
        assert!(host.last_emit.lock().expect("emit").is_none());
    });

    run_headless(|cx| {
        let host = Arc::new(time_host(Some("12:00")));
        host.context.lock().expect("context").disabled = true;
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = time_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("poodle-input-time-before");
        driver.focus_element("poodle-input-time-before");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-time-after"),
            Some(true)
        );
        driver.dispatch_key_raw("up");
        assert_eq!(committed(&host).as_deref(), Some("12:00"));
        assert!(host.last_emit.lock().expect("emit").is_none());
    });
}

// ── g16.030 NumberInput mounted editing ────────────────────────────────────

#[derive(Clone)]
struct NumberFieldState {
    name: String,
    value: Option<f64>,
    draft: Option<String>,
    selection: (usize, usize),
    is_focused: bool,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    precision: Option<u16>,
    show_steppers: bool,
    is_disabled: bool,
    is_read_only: bool,
    validation_state: poodle_specs::ValidationState,
}

impl NumberFieldState {
    fn new(name: &str, value: Option<f64>) -> Self {
        let display = poodle_headless::number_input::format_number_committed(value, None);
        let len = display.chars().count();
        Self {
            name: name.to_owned(),
            value,
            draft: None,
            selection: (len, len),
            is_focused: false,
            min: None,
            max: None,
            step: None,
            precision: None,
            show_steppers: false,
            is_disabled: false,
            is_read_only: false,
            validation_state: poodle_specs::ValidationState::None,
        }
    }

    fn bounded(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    fn stepped(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    fn precise(mut self, precision: u16) -> Self {
        self.precision = Some(precision);
        let display = poodle_headless::number_input::format_number_committed(
            self.value,
            Some(f64::from(precision)),
        );
        let len = display.chars().count();
        self.selection = (len, len);
        self
    }

    fn with_steppers(mut self) -> Self {
        self.show_steppers = true;
        self
    }

    fn disabled(mut self) -> Self {
        self.is_disabled = true;
        self
    }

    fn read_only(mut self) -> Self {
        self.is_read_only = true;
        self
    }
}

struct NumberFieldHost {
    fields: Mutex<Vec<NumberFieldState>>,
    log: Mutex<Vec<String>>,
}

impl NumberFieldHost {
    fn new(fields: Vec<NumberFieldState>) -> Arc<Self> {
        Arc::new(Self {
            fields: Mutex::new(fields),
            log: Mutex::new(Vec::new()),
        })
    }

    fn field(&self, name: &str) -> NumberFieldState {
        self.fields
            .lock()
            .expect("fields")
            .iter()
            .find(|field| field.name == name)
            .cloned()
            .unwrap_or_else(|| panic!("{name} is mounted"))
    }

    fn take_log(&self) -> Vec<String> {
        std::mem::take(&mut *self.log.lock().expect("log"))
    }
}

fn number_field_id(name: &str) -> String {
    format!("poodle-number-input-{name}")
}

fn number_field_inc_id(name: &str) -> String {
    format!("poodle-number-input-{name}-inc")
}

fn number_field_apply(
    host: &Arc<NumberFieldHost>,
    mounted: &Arc<Mutex<Node>>,
    name: &str,
    entry: String,
    mutate: impl FnOnce(&mut NumberFieldState),
) {
    {
        let mut fields = host.fields.lock().expect("fields");
        let field = fields
            .iter_mut()
            .find(|field| field.name == name)
            .unwrap_or_else(|| panic!("{name} is mounted"));
        mutate(field);
    }
    host.log.lock().expect("log").push(entry);
    let next = number_field_tree(host, mounted);
    *mounted.lock().expect("mount") = next;
}

fn number_field_handlers(
    host: &Arc<NumberFieldHost>,
    mounted: &Arc<Mutex<Node>>,
    name: &str,
) -> poodle_render::NumberInputHandlers {
    macro_rules! sink {
        () => {{
            (Arc::clone(host), Arc::clone(mounted), name.to_owned())
        }};
    }
    let (draft_host, draft_mount, draft_name) = sink!();
    let (value_host, value_mount, value_name) = sink!();
    let (commit_host, commit_mount, commit_name) = sink!();
    let (select_host, select_mount, select_name) = sink!();
    let (focus_host, focus_mount, focus_name) = sink!();

    poodle_render::NumberInputHandlers {
        on_draft_value_change: Some(Arc::new(move |draft: Option<String>| {
            let label = match &draft {
                Some(text) => format!("{draft_name}/draft:{text}"),
                None => format!("{draft_name}/draft:null"),
            };
            number_field_apply(&draft_host, &draft_mount, &draft_name, label, |field| {
                field.draft = draft;
            });
        })),
        on_value_change: Some(Arc::new(move |value: Option<f64>| {
            let label = match value {
                Some(v) => format!("{value_name}/value:{v}"),
                None => format!("{value_name}/value:null"),
            };
            number_field_apply(&value_host, &value_mount, &value_name, label, |field| {
                field.value = value;
            });
        })),
        on_commit: Some(Arc::new(move |value: Option<f64>| {
            let label = match value {
                Some(v) => format!("{commit_name}/commit:{v}"),
                None => format!("{commit_name}/commit:null"),
            };
            number_field_apply(&commit_host, &commit_mount, &commit_name, label, |_| {});
        })),
        on_selection_change: Some(Arc::new(move |start: usize, end: usize| {
            number_field_apply(
                &select_host,
                &select_mount,
                &select_name,
                format!("{select_name}/select:{start}-{end}"),
                |field| field.selection = (start, end),
            );
        })),
        on_focus_change: Some(Arc::new(move |focused: bool| {
            number_field_apply(
                &focus_host,
                &focus_mount,
                &focus_name,
                format!("{focus_name}/focus:{focused}"),
                |field| field.is_focused = focused,
            );
        })),
    }
}

fn number_field_tree(host: &Arc<NumberFieldHost>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let states = host.fields.lock().expect("fields").clone();

    let mut column = Node::container();
    column.id = Some(FIXTURE_ID.to_owned());
    column.style.descriptor.layout.direction = LayoutDirection::Column;
    column.style.descriptor.layout.spacing.gap = 8.0;
    column.style.descriptor.layout.width = LayoutSizing::Fixed(360.0);

    for state in &states {
        let mut spec = poodle_specs::NumberInputSpec::new(state.value)
            .with_id(&state.name)
            .with_aria_label(&state.name)
            .with_selection(state.selection.0, state.selection.1)
            .with_focused(state.is_focused)
            .with_disabled(state.is_disabled)
            .with_read_only(state.is_read_only)
            .with_steppers(state.show_steppers)
            .with_validation_state(state.validation_state);
        if let Some(draft) = &state.draft {
            spec = spec.with_draft_value(Some(draft.clone()));
        }
        if let Some(min) = state.min {
            spec = spec.with_min(Some(min));
        }
        if let Some(max) = state.max {
            spec = spec.with_max(Some(max));
        }
        if let Some(step) = state.step {
            spec = spec.with_step(Some(step));
        }
        if let Some(precision) = state.precision {
            spec = spec.with_precision(precision);
        }
        column = column.child(poodle_render::number_input(
            &spec,
            &ctx,
            number_field_handlers(host, mounted, &state.name),
        ));
    }
    column
}

fn mount_number_fields<'a>(
    cx: &'a mut TestAppContext,
    host: &Arc<NumberFieldHost>,
) -> (HeadlessDriver<'a>, Arc<Mutex<Node>>) {
    let mounted = Arc::new(Mutex::new(Node::container()));
    *mounted.lock().expect("mount") = number_field_tree(host, &mounted);
    let driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
    (driver, mounted)
}

fn mounted_number(mounted: &Arc<Mutex<Node>>, name: &str) -> Node {
    mounted
        .lock()
        .expect("mount")
        .find(&|node| node.id.as_deref() == Some(number_field_id(name).as_str()))
        .cloned()
        .unwrap_or_else(|| panic!("{name} is mounted"))
}

/// g16.030. NumberInput routes real mounted text/key/focus/pointer dispatch
/// through the shared machine and specimen-owned rebuild state: valid edits,
/// partial/invalid silence, clear, blur/Escape revert, Enter commit, fractional
/// step, precision, bounds, Home/End, controlled replacement, identity, and
/// disabled/read-only inertia.
#[test]
fn number_input_mounted_valid_direct_editing_rebuilds_host_draft_and_value() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("qty", Some(5.0)).bounded(0.0, 100.0)]);
        let (mut driver, mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));
        host.take_log();

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("1");
        driver.dispatch_key_raw("2");
        assert_eq!(host.field("qty").draft.as_deref(), Some("12"));
        assert_eq!(host.field("qty").value, Some(12.0));
        assert_eq!(
            mounted_number(&mounted, "qty").a11y.value,
            Some(12.0),
            "the rebuilt spin-button projects the live value"
        );
        let log = host.take_log();
        assert!(
            log.iter().any(|entry| entry == "qty/value:12"),
            "valid complete drafts emit value: {log:?}"
        );
    });
}

#[test]
fn number_input_mounted_partial_and_invalid_drafts_emit_no_value() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("qty", Some(5.0)).bounded(0.0, 10.0)]);
        let (mut driver, mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));
        host.take_log();

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("-");
        assert_eq!(host.field("qty").draft.as_deref(), Some("-"));
        assert_eq!(host.field("qty").value, Some(5.0));
        assert!(
            !host.take_log().iter().any(|entry| entry.starts_with("qty/value:")),
            "incomplete drafts stay silent on the value channel"
        );
        assert_eq!(mounted_number(&mounted, "qty").a11y.invalid, Some(true));

        driver.dispatch_key_raw("escape");
        host.take_log();
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("9");
        assert_eq!(host.field("qty").value, Some(9.0));
        host.take_log();
        driver.dispatch_key_raw("9");
        assert_eq!(host.field("qty").draft.as_deref(), Some("99"));
        assert_eq!(host.field("qty").value, Some(9.0));
        assert!(
            !host.take_log().iter().any(|entry| entry.starts_with("qty/value:")),
            "out-of-range complete drafts emit no further value"
        );
        assert_eq!(mounted_number(&mounted, "qty").a11y.invalid, Some(true));
    });
}

#[test]
fn number_input_mounted_empty_clear_emits_null_value() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("qty", Some(5.0))]);
        let (mut driver, _mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));
        host.take_log();

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("backspace");
        assert_eq!(host.field("qty").draft.as_deref(), Some(""));
        assert_eq!(host.field("qty").value, None);
        let log = host.take_log();
        assert!(
            log.iter().any(|entry| entry == "qty/value:null"),
            "clear emits committed null: {log:?}"
        );
    });
}

#[test]
fn number_input_mounted_blur_and_escape_revert_unresolved_drafts() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![
            NumberFieldState::new("qty", Some(5.0)),
            NumberFieldState::new("other", Some(1.0)),
        ]);
        let (mut driver, _mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));
        host.take_log();

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("-");
        assert_eq!(host.field("qty").draft.as_deref(), Some("-"));
        host.take_log();

        driver.dispatch_key_raw("escape");
        assert_eq!(host.field("qty").draft, None);
        assert_eq!(host.field("qty").value, Some(5.0));
        let log = host.take_log();
        assert!(
            log.iter().any(|entry| entry == "qty/draft:null"),
            "Escape discards the draft: {log:?}"
        );
        assert!(
            !log.iter().any(|entry| entry.starts_with("qty/value:") || entry.starts_with("qty/commit:")),
            "Escape emits neither value nor commit: {log:?}"
        );
    });

    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![
            NumberFieldState::new("qty", Some(5.0)),
            NumberFieldState::new("other", Some(1.0)),
        ]);
        let (mut driver, _mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("-");
        host.take_log();

        driver.focus_element(&number_field_id("other"));
        assert_eq!(host.field("qty").draft, None);
        assert_eq!(host.field("qty").value, Some(5.0));
        let log = host.take_log();
        assert!(
            log.iter().any(|entry| entry == "qty/draft:null"),
            "blur discards unresolved drafts: {log:?}"
        );
        assert!(
            !log.iter().any(|entry| entry.starts_with("qty/commit:")),
            "blur on unresolved drafts does not commit: {log:?}"
        );
    });
}

#[test]
fn number_input_mounted_enter_commits_resolved_value() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("qty", Some(5.0))]);
        let (mut driver, _mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));
        host.take_log();

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("8");
        host.take_log();
        driver.dispatch_key_raw("enter");
        let log = host.take_log();
        assert!(
            log.iter().any(|entry| entry == "qty/commit:8"),
            "Enter commits the resolved value: {log:?}"
        );
        assert_eq!(host.field("qty").value, Some(8.0));
        assert_eq!(host.field("qty").draft, None);
    });

    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("qty", Some(5.0))]);
        let (mut driver, _mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("-");
        host.take_log();
        driver.dispatch_key_raw("enter");
        assert_eq!(host.field("qty").draft.as_deref(), Some("-"));
        assert_eq!(host.field("qty").value, Some(5.0));
        assert!(
            !host.take_log().iter().any(|entry| entry.starts_with("qty/commit:")),
            "Enter on an unresolved draft stays silent"
        );
    });
}

#[test]
fn number_input_mounted_fractional_step_precision_bounds_and_home_end() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("price", Some(1.00))
            .bounded(0.0, 5.0)
            .stepped(0.25)
            .precise(2)
            .with_steppers()]);
        let (mut driver, mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("price"));
        driver.focus_element(&number_field_id("price"));
        host.take_log();

        driver.dispatch_key_raw("up");
        assert_eq!(host.field("price").value, Some(1.25));
        let log = host.take_log();
        assert!(
            log.iter().any(|entry| entry == "price/commit:1.25"),
            "a successful step commits: {log:?}"
        );

        driver.pointer_activate_id(&number_field_inc_id("price"));
        assert_eq!(host.field("price").value, Some(1.5));

        driver.focus_element(&number_field_id("price"));
        driver.dispatch_key_raw("end");
        assert_eq!(host.field("price").value, Some(5.0));
        driver.dispatch_key_raw("home");
        assert_eq!(host.field("price").value, Some(0.0));

        host.take_log();
        driver.focus_element(&number_field_id("price"));
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("1");
        assert_eq!(host.field("price").value, Some(1.0));
        host.take_log();
        driver.dispatch_key_raw(".");
        driver.dispatch_key_raw("2");
        driver.dispatch_key_raw("3");
        driver.dispatch_key_raw("4");
        assert_eq!(host.field("price").draft.as_deref(), Some("1.234"));
        assert_eq!(host.field("price").value, Some(1.0));
        assert!(
            !host.take_log().iter().any(|entry| entry.starts_with("price/value:")),
            "over-precision drafts emit no further value"
        );

        let node = mounted_number(&mounted, "price");
        assert_eq!(node.a11y.value_min, Some(0.0));
        assert_eq!(node.a11y.value_max, Some(5.0));
    });
}

#[test]
fn number_input_mounted_controlled_replacement_discards_uncontrolled_draft() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![NumberFieldState::new("qty", Some(5.0))]);
        let (mut driver, mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));
        driver.focus_element(&number_field_id("qty"));

        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("-");
        assert_eq!(host.field("qty").draft.as_deref(), Some("-"));

        number_field_apply(
            &host,
            &mounted,
            "qty",
            "qty/replace:9".into(),
            |field| {
                field.value = Some(9.0);
                field.draft = None;
                let display = poodle_headless::number_input::format_number_committed(Some(9.0), None);
                let len = display.chars().count();
                field.selection = (len, len);
            },
        );
        assert_eq!(host.field("qty").value, Some(9.0));
        assert_eq!(host.field("qty").draft, None);

        host.take_log();
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("4");
        assert_eq!(host.field("qty").value, Some(4.0));
        assert_eq!(host.field("qty").draft.as_deref(), Some("4"));
    });
}

#[test]
fn number_input_mounted_two_instances_keep_independent_identity() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![
            NumberFieldState::new("left", Some(3.0)),
            NumberFieldState::new("right", Some(3.0)),
        ]);
        let (mut driver, _mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("left"));
        driver.wait_for_focus_handle(&number_field_id("right"));

        driver.focus_element(&number_field_id("left"));
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("7");
        assert_eq!(host.field("left").value, Some(7.0));
        assert_eq!(host.field("right").value, Some(3.0));

        driver.focus_element(&number_field_id("right"));
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("8");
        assert_eq!(host.field("left").value, Some(7.0));
        assert_eq!(host.field("right").value, Some(8.0));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&number_field_id("left")),
            Some(false),
            "equal starting values do not merge focus identity"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&number_field_id("right")),
            Some(true)
        );
    });
}

#[test]
fn number_input_mounted_disabled_and_read_only_are_inert() {
    run_headless(|cx| {
        let host = NumberFieldHost::new(vec![
            NumberFieldState::new("locked", Some(5.0)).disabled().with_steppers(),
            NumberFieldState::new("frozen", Some(5.0)).read_only().with_steppers(),
        ]);
        let (mut driver, mounted) = mount_number_fields(cx, &host);

        assert!(
            poodle_gpui_node_backend::focus_handle_for(&number_field_id("locked")).is_none(),
            "disabled fields are not focusable"
        );
        host.take_log();
        driver.pointer_activate_id(&number_field_id("locked"));
        driver.dispatch_key_raw("9");
        driver.pointer_activate_id(&number_field_inc_id("locked"));
        assert_eq!(host.field("locked").value, Some(5.0));
        assert!(
            host.take_log()
                .into_iter()
                .filter(|entry| entry.starts_with("locked/")
                    && !entry.ends_with("/focus:true")
                    && !entry.ends_with("/focus:false"))
                .collect::<Vec<_>>()
                .is_empty(),
            "disabled reports no mutation"
        );

        driver.wait_for_focus_handle(&number_field_id("frozen"));
        driver.focus_element(&number_field_id("frozen"));
        host.take_log();
        driver.dispatch_key_raw("9");
        driver.dispatch_key_raw("up");
        driver.pointer_activate_id(&number_field_inc_id("frozen"));
        assert_eq!(host.field("frozen").value, Some(5.0));
        assert_eq!(host.field("frozen").draft, None);
        let log = host.take_log();
        assert!(
            !log.iter().any(|entry| entry.starts_with("frozen/value:")
                || entry.starts_with("frozen/draft:")
                || entry.starts_with("frozen/commit:")),
            "read-only stays inert on text and step routes: {log:?}"
        );

        let frozen = mounted_number(&mounted, "frozen");
        assert!(frozen.interaction.focusable);
        assert!(frozen.interaction.on_text_change.is_none());
    });
}

/// Accessibility surface for the mounted SpinButton: name, value, bounds,
/// unresolved invalid, validation busy, stepper labels/bounds, and one focus
/// treatment on the field root.
#[test]
fn number_input_mounted_accessibility_projects_spin_button_surface() {
    run_headless(|cx| {
        let mut pending = NumberFieldState::new("pending", Some(2.0)).bounded(0.0, 10.0);
        pending.validation_state = poodle_specs::ValidationState::Pending;
        pending.show_steppers = true;
        let host = NumberFieldHost::new(vec![
            NumberFieldState::new("qty", Some(3.0))
                .bounded(0.0, 10.0)
                .with_steppers(),
            pending,
        ]);
        let (mut driver, mounted) = mount_number_fields(cx, &host);
        driver.wait_for_focus_handle(&number_field_id("qty"));

        let qty = mounted_number(&mounted, "qty");
        assert_eq!(qty.a11y.role, Some(NodeRole::SpinButton));
        assert_eq!(qty.a11y.label.as_deref(), Some("qty"));
        assert_eq!(qty.a11y.value, Some(3.0));
        assert_eq!(qty.a11y.value_min, Some(0.0));
        assert_eq!(qty.a11y.value_max, Some(10.0));
        assert!(qty.style.focus.is_some(), "the field owns one focus treatment");

        let steppers = &qty.children[1];
        let inc = &steppers.children[0];
        let dec = &steppers.children[1];
        assert_eq!(inc.a11y.label.as_deref(), Some("Increment"));
        assert_eq!(dec.a11y.label.as_deref(), Some("Decrement"));
        assert!(!inc.interaction.focusable);
        assert!(!dec.interaction.focusable);
        assert!(inc.style.focus.is_none());
        assert!(dec.style.focus.is_none());
        assert!(!inc.interaction.disabled);
        assert!(!dec.interaction.disabled);

        let pending_node = mounted_number(&mounted, "pending");
        assert_eq!(pending_node.a11y.busy, Some(true));

        driver.focus_element(&number_field_id("qty"));
        driver.dispatch_key_raw("cmd-a");
        driver.dispatch_key_raw("-");
        let invalid = mounted_number(&mounted, "qty");
        assert_eq!(invalid.a11y.invalid, Some(true));
        assert_eq!(invalid.a11y.value, None);

        // At the authored max, Increment is blocked.
        number_field_apply(&host, &mounted, "qty", "qty/bound".into(), |field| {
            field.value = Some(10.0);
            field.draft = None;
            field.selection = (2, 2);
        });
        let at_max = mounted_number(&mounted, "qty");
        let inc_at_max = &at_max.children[1].children[0];
        assert!(inc_at_max.interaction.disabled, "Increment disables at max");
    });
}

/// g16.008. EditableLabel still commits when Tab leaves it — but for the
/// reason its contract gives: Tab moves focus, and the blur commits the draft
/// once. Enter is the direct commit, Escape cancels, and neither leaves a
/// second commit behind for the blur to fire. Enter and Escape restore the
/// real display focus handle; Tab advances past it.
///
/// Deliberately not claimed: activation modes, select-on-focus, or the
/// display mode's own affordances.
#[test]
fn editable_label_commits_on_enter_and_once_through_the_blur_tab_causes() {
    // ── Enter: one commit, and the blur that follows adds nothing ─────────
    run_headless(|cx| {
        let host = label_host("label-enter");
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = label_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 200.0);
        driver.wait_for_focus_handle(&host.id);
        driver.focus_element(&host.id);
        take_events(&host.log);

        driver.dispatch_key_raw("s");
        assert_eq!(*host.draft.lock().expect("draft"), "Kicks");
        take_events(&host.log);

        driver.dispatch_key_raw("enter");
        driver.draw_frame();
        assert_eq!(
            take_events(&host.log),
            vec!["label/commit:Kicks", "label/restore"],
            "Enter commits directly, exactly once"
        );
        assert_eq!(*host.value.lock().expect("value"), "Kicks");
        assert!(!*host.editing.lock().expect("editing"));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&host.id),
            Some(true),
            "Enter restores the display focus handle"
        );
    });

    // ── Escape: cancel, and no commit behind it ───────────────────────────
    run_headless(|cx| {
        let host = label_host("label-escape");
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = label_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 200.0);
        driver.wait_for_focus_handle(&host.id);
        driver.focus_element(&host.id);
        driver.dispatch_key_raw("s");
        take_events(&host.log);

        driver.dispatch_key_raw("escape");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["label/cancel", "label/restore"]);
        assert_eq!(
            *host.value.lock().expect("value"),
            "Kick",
            "a cancelled edit never reaches the committed value"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&host.id),
            Some(true),
            "Escape restores the display focus handle"
        );
    });

    // ── Tab: focus moves, and the blur it causes commits once ─────────────
    run_headless(|cx| {
        let host = label_host("label-tab");
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = label_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 200.0);
        driver.wait_for_focus_handle(&host.id);
        driver.focus_element(&host.id);
        driver.dispatch_key_raw("s");
        assert_eq!(*host.draft.lock().expect("draft"), "Kicks");
        take_events(&host.log);

        driver.dispatch_key_raw("tab");
        driver.draw_frame();
        // The order is the claim: the host hears the commit — reported from
        // this field's blur — before it hears the next field take focus.
        // Nothing here routed Tab to the submit channel.
        assert_eq!(
            take_events(&host.log),
            vec!["label/commit:Kicks", "label-tab-after/focus:true"]
        );
        assert_eq!(*host.value.lock().expect("value"), "Kicks");
        assert!(!*host.editing.lock().expect("editing"));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-label-tab-after"),
            Some(true),
            "and focus really did advance"
        );
        assert_ne!(
            poodle_gpui_node_backend::focus_state_for("poodle-input-label-tab"),
            Some(true),
            "Tab does not restore display focus"
        );

        // Further frames cannot produce a second commit: the edit is over.
        driver.draw_frame();
        driver.draw_frame();
        assert_eq!(take_events(&host.log), Vec::<String>::new());
    });
}

/// g16.045. Live keystrokes paint the session draft while the host's committed
/// `value` and the commit callback's previous snapshot stay on Kick.
#[test]
fn editable_label_live_draft_stays_off_the_committed_value() {
    run_headless(|cx| {
        let host = label_host("label-draft-oracle");
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount") = label_routing_tree(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 200.0);
        driver.wait_for_focus_handle(&host.id);
        driver.focus_element(&host.id);
        take_events(&host.log);

        driver.dispatch_key_raw("s");
        assert_eq!(painted_label_input(&mounted, &host.id), "Kicks");
        assert_eq!(
            *host.value.lock().expect("value"),
            "Kick",
            "live typing never overwrites the committed value"
        );
        assert_eq!(*host.draft.lock().expect("draft"), "Kicks");
        assert!(host.last_previous.lock().expect("previous").is_none());
        take_events(&host.log);

        driver.dispatch_key_raw("enter");
        driver.draw_frame();
        assert_eq!(take_events(&host.log), vec!["label/commit:Kicks", "label/restore"]);
        assert_eq!(
            host.last_previous.lock().expect("previous").as_deref(),
            Some("Kick")
        );
        assert_eq!(*host.value.lock().expect("value"), "Kicks");
        assert!(!*host.editing.lock().expect("editing"));
    });
}

/// g16.010. Linkless Breadcrumbs crumbs call `on_navigate` with their authored
/// value through real pointer and keyboard dispatch. `href`, current, and
/// ellipsis crumbs stay inert and are not sequential stops.
///
/// Deliberately not claimed: native URL routing, assistive-technology
/// coverage, visual comparison, or Jetstream admission.
#[test]
fn breadcrumbs_callback_navigation_through_mounted_pointer_and_keyboard() {
    fn crumb_name(node: &Node) -> Option<String> {
        if let Some(label) = node.a11y.label.clone() {
            return Some(label);
        }
        match &node.kind {
            NodeKind::Text { content } => Some(content.clone()),
            _ => node.children.iter().find_map(|child| match &child.kind {
                NodeKind::Text { content } => Some(content.clone()),
                _ => None,
            }),
        }
    }

    fn stamp_crumb_ids(root: &mut Node) {
        let trail = root
            .children
            .iter_mut()
            .find(|node| node.a11y.label.as_deref() == Some("Trail"))
            .expect("the breadcrumbs root keeps its aria label");
        for child in trail.children.iter_mut().step_by(2) {
            let id = match crumb_name(child).as_deref() {
                Some("Home") => "breadcrumbs-home",
                Some("\u{2026}") => "breadcrumbs-ellipsis",
                Some("Workspace") => "breadcrumbs-workspace",
                Some("Projects") => "breadcrumbs-projects",
                Some("Poodle") => "breadcrumbs-poodle",
                other => panic!("unexpected crumb {other:?}"),
            };
            child.id = Some(id.to_owned());
        }
    }

    fn marker(id: &str, label: &str) -> Node {
        let mut node = poodle_render::button(
            &poodle_specs::ButtonSpec::new().with_label(label),
            &RenderContext::new(&theme()),
            None,
        );
        node.id = Some(id.to_owned());
        node
    }

    fn crumb<'a>(root: &'a Node, id: &str) -> &'a Node {
        root.find(&|node| node.id.as_deref() == Some(id))
            .unwrap_or_else(|| panic!("{id}"))
    }

    run_headless(|cx| {
        let nav = Arc::new(Mutex::new(Vec::<String>::new()));
        let sink = Arc::clone(&nav);
        let trail = poodle_render::breadcrumbs(
            &poodle_specs::BreadcrumbsSpec::new(vec![
                poodle_specs::BreadcrumbItem::new("home", "Home"),
                poodle_specs::BreadcrumbItem::new("hidden", "Hidden"),
                poodle_specs::BreadcrumbItem::new("workspace", "Workspace").with_href("/workspace"),
                poodle_specs::BreadcrumbItem::new("projects", "Projects").with_icon_only("folder"),
                poodle_specs::BreadcrumbItem::new("poodle", "Poodle").with_is_current(true),
            ])
            .with_aria_label("Trail")
            .with_max_visible_items(4),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |value: &str| {
                sink.lock().expect("nav lock").push(value.to_string());
            })),
        );

        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root = root
            .child(marker("breadcrumbs-before", "Before"))
            .child(trail)
            .child(marker("breadcrumbs-after", "After"));
        stamp_crumb_ids(&mut root);

        {
            let home = crumb(&root, "breadcrumbs-home");
            let projects = crumb(&root, "breadcrumbs-projects");
            assert_eq!(home.a11y.role, Some(NodeRole::Button));
            assert_eq!(home.a11y.label.as_deref(), Some("Home"));
            assert!(home.style.focus_ring.is_some());
            assert_eq!(projects.a11y.role, Some(NodeRole::Button));
            assert_eq!(projects.a11y.label.as_deref(), Some("Projects"));
            assert!(projects.style.focus_ring.is_some());
            for id in [
                "breadcrumbs-ellipsis",
                "breadcrumbs-workspace",
                "breadcrumbs-poodle",
            ] {
                let node = crumb(&root, id);
                assert!(node.interaction.on_activate.is_none(), "{id}");
                assert!(!node.interaction.focusable, "{id}");
                assert!(node.style.focus_ring.is_none(), "{id}");
            }
        }

        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 640.0, 160.0);
        driver.wait_for_focus_handle("breadcrumbs-before");
        driver.wait_for_focus_handle("breadcrumbs-home");
        driver.wait_for_focus_handle("breadcrumbs-projects");
        driver.wait_for_focus_handle("breadcrumbs-after");

        assert!(
            poodle_gpui_node_backend::bounds_for("breadcrumbs-home").is_some(),
            "pointer proof needs a real hit target"
        );
        driver.pointer_activate_id("breadcrumbs-home");
        assert_eq!(*nav.lock().expect("nav lock"), ["home"]);

        driver.focus_element("breadcrumbs-projects");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("breadcrumbs-projects"),
            Some(true)
        );
        driver.dispatch_key_raw("enter");
        assert_eq!(*nav.lock().expect("nav lock"), ["home", "projects"]);
        driver.dispatch_key_raw("space");
        assert_eq!(
            *nav.lock().expect("nav lock"),
            ["home", "projects", "projects"]
        );

        for id in [
            "breadcrumbs-ellipsis",
            "breadcrumbs-workspace",
            "breadcrumbs-poodle",
        ] {
            if poodle_gpui_node_backend::bounds_for(id).is_some() {
                driver.pointer_activate_id(id);
            }
            assert!(
                poodle_gpui_node_backend::focus_handle_for(id).is_none(),
                "{id} must not become a sequential stop"
            );
        }
        assert_eq!(
            *nav.lock().expect("nav lock"),
            ["home", "projects", "projects"]
        );

        driver.focus_element("breadcrumbs-before");
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("breadcrumbs-home"),
            Some(true),
            "the first Tab from Before lands on the linkless text crumb"
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("breadcrumbs-projects"),
            Some(true),
            "href, ellipsis, and current crumbs are skipped"
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("breadcrumbs-after"),
            Some(true)
        );
    });
}

/// g16.011. IconButton command and toggle outcomes travel through real pointer
/// and keyboard dispatch. Tooltip text, role, label, tab position, focus ring,
/// and toggled/disclosure state ride the same production target.
///
/// Deliberately not claimed: Tooltip overlay chrome, web timer/Escape
/// lifecycle, aria-describedby, assistive-technology coverage, visual
/// comparison, or Jetstream admission.
#[test]
fn icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard() {
    use poodle_node::NodeToggled;
    use poodle_render::IconButtonHandlers;
    use poodle_specs::IconButtonSpec;

    fn marker(id: &str, label: &str) -> Node {
        let mut node = poodle_render::button(
            &poodle_specs::ButtonSpec::new().with_label(label),
            &RenderContext::new(&theme()),
            None,
        );
        node.id = Some(id.to_owned());
        node
    }

    fn icon(spec: IconButtonSpec, id: &str, handlers: IconButtonHandlers) -> Node {
        let mut node =
            poodle_render::icon_button_with_handlers(&spec, &RenderContext::new(&theme()), handlers);
        node.id = Some(id.to_owned());
        node
    }

    fn target<'a>(root: &'a Node, id: &str) -> &'a Node {
        root.find(&|node| node.id.as_deref() == Some(id))
            .unwrap_or_else(|| panic!("{id}"))
    }

    // ── Command, tooltip projection, semantics, inert skips ──────────────
    run_headless(|cx| {
        let clicks = Arc::new(Mutex::new(Vec::<String>::new()));
        let pressed = Arc::new(Mutex::new(Vec::<bool>::new()));
        let click_sink = Arc::clone(&clicks);
        let pressed_sink = Arc::clone(&pressed);
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root = root
            .child(marker("icon-before", "Before"))
            .child(icon(
                IconButtonSpec::new()
                    .with_icon("plus")
                    .with_aria_label("Add"),
                "icon-command",
                IconButtonHandlers {
                    on_click: Some(Arc::new(move || {
                        click_sink.lock().expect("click lock").push("Add".into());
                    })),
                    on_pressed_change: Some(Arc::new(move |next| {
                        pressed_sink.lock().expect("pressed lock").push(next);
                    })),
                },
            ))
            .child(icon(
                IconButtonSpec::new()
                    .with_icon("save")
                    .with_aria_label("Save")
                    .with_tooltip("Save document"),
                "icon-tooltip",
                IconButtonHandlers::default(),
            ))
            .child(icon(
                IconButtonSpec::new()
                    .with_icon("x")
                    .with_aria_label("Close"),
                "icon-fallback",
                IconButtonHandlers::default(),
            ))
            .child(icon(
                IconButtonSpec::new()
                    .with_icon("chevron-down")
                    .with_aria_label("Details")
                    .with_expanded(true)
                    .with_controls("panel"),
                "icon-disclosure",
                IconButtonHandlers::default(),
            ))
            .child(icon(
                IconButtonSpec::new()
                    .with_icon("ban")
                    .with_aria_label("Block")
                    .with_disabled(true),
                "icon-disabled",
                IconButtonHandlers {
                    on_click: Some(Arc::new(|| panic!("disabled buttons do not fire"))),
                    on_pressed_change: Some(Arc::new(|_| panic!("disabled buttons do not fire"))),
                },
            ))
            .child(icon(
                IconButtonSpec::new()
                    .with_icon("loader")
                    .with_aria_label("Refresh")
                    .with_loading(true),
                "icon-loading",
                IconButtonHandlers {
                    on_click: Some(Arc::new(|| panic!("loading buttons do not fire"))),
                    on_pressed_change: Some(Arc::new(|_| panic!("loading buttons do not fire"))),
                },
            ))
            .child(marker("icon-after", "After"));

        {
            let command = target(&root, "icon-command");
            assert_eq!(command.a11y.role, Some(NodeRole::Button));
            assert_eq!(command.a11y.label.as_deref(), Some("Add"));
            assert_eq!(command.a11y.tab_index, Some(0));
            assert!(command.style.focus_ring.is_some());
            assert!(command.a11y.toggled.is_none());
            assert_eq!(command.tooltip.as_deref(), Some("Add"));

            let explicit = target(&root, "icon-tooltip");
            assert_eq!(explicit.tooltip.as_deref(), Some("Save document"));
            assert_eq!(explicit.a11y.label.as_deref(), Some("Save"));

            let fallback = target(&root, "icon-fallback");
            assert_eq!(fallback.tooltip.as_deref(), Some("Close"));

            let disclosure = target(&root, "icon-disclosure");
            assert_eq!(disclosure.a11y.role, Some(NodeRole::Button));
            assert_eq!(disclosure.a11y.expanded, Some(true));
            assert_eq!(disclosure.a11y.controls.as_deref(), Some("panel"));
            assert!(disclosure.style.focus_ring.is_some());

            for id in ["icon-disabled", "icon-loading"] {
                let node = target(&root, id);
                assert!(node.interaction.disabled, "{id}");
                assert!(!node.interaction.focusable, "{id}");
                assert_eq!(node.a11y.tab_index, None, "{id}");
                assert!(node.style.focus_ring.is_none(), "{id}");
                assert!(node.interaction.on_activate.is_none(), "{id}");
            }
        }

        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 640.0, 420.0);
        driver.wait_for_focus_handle("icon-before");
        driver.wait_for_focus_handle("icon-command");
        driver.wait_for_focus_handle("icon-after");

        assert!(
            poodle_gpui_node_backend::bounds_for("icon-command").is_some(),
            "pointer proof needs a real hit target"
        );
        driver.pointer_activate_id("icon-command");
        assert_eq!(*clicks.lock().expect("click lock"), ["Add"]);
        assert!(
            pressed.lock().expect("pressed lock").is_empty(),
            "command-only activation must not manufacture a pressed change"
        );

        for id in ["icon-disabled", "icon-loading"] {
            if poodle_gpui_node_backend::bounds_for(id).is_some() {
                driver.pointer_activate_id(id);
            }
            assert!(
                poodle_gpui_node_backend::focus_handle_for(id).is_none(),
                "{id} must not become a sequential stop"
            );
        }
        assert_eq!(*clicks.lock().expect("click lock"), ["Add"]);

        driver.focus_element("icon-before");
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("icon-command"),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("icon-tooltip"),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("icon-fallback"),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("icon-disclosure"),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("icon-after"),
            Some(true),
            "disabled and loading targets are skipped"
        );
    });

    // ── Controlled toggle: Enter then Space rebuild the host spec ────────
    run_headless(|cx| {
        fn build(
            pressed: bool,
            mounted: Arc<Mutex<Node>>,
            events: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let event_sink = Arc::clone(&events);
            let click_sink = Arc::clone(&events);
            let mount = Arc::clone(&mounted);
            let mut node = poodle_render::icon_button_with_handlers(
                &IconButtonSpec::new()
                    .with_icon("bold")
                    .with_aria_label("Bold")
                    .with_pressed(pressed),
                &RenderContext::new(&theme()),
                IconButtonHandlers {
                    on_pressed_change: Some(Arc::new(move |next| {
                        event_sink
                            .lock()
                            .expect("event lock")
                            .push(format!("pressed:{next}"));
                        *mount.lock().expect("mount lock") =
                            build(next, Arc::clone(&mount), Arc::clone(&event_sink));
                    })),
                    on_click: Some(Arc::new(move || {
                        click_sink.lock().expect("event lock").push("click".into());
                    })),
                },
            );
            node.id = Some("icon-toggle".to_owned());
            node
        }

        let events = Arc::new(Mutex::new(Vec::<String>::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(false, Arc::clone(&mounted), Arc::clone(&events));
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.toggled, Some(NodeToggled::False));
            assert_eq!(node.a11y.role, Some(NodeRole::Button));
            assert!(node.style.focus_ring.is_some());
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 120.0, 120.0);
        driver.wait_for_focus_handle("icon-toggle");
        driver.focus_element("icon-toggle");
        driver.dispatch_key_raw("enter");
        assert_eq!(
            *events.lock().expect("event lock"),
            ["pressed:true".to_string(), "click".to_string()]
        );
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.toggled,
            Some(NodeToggled::True)
        );

        driver.wait_for_focus_handle("icon-toggle");
        driver.focus_element("icon-toggle");
        driver.dispatch_key_raw("space");
        assert_eq!(
            *events.lock().expect("event lock"),
            [
                "pressed:true".to_string(),
                "click".to_string(),
                "pressed:false".to_string(),
                "click".to_string()
            ]
        );
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.toggled,
            Some(NodeToggled::False)
        );
    });

    // ── Seeded toggle: default_pressed starts on and reports false first ─
    run_headless(|cx| {
        fn build(
            pressed: Option<bool>,
            default_pressed: Option<bool>,
            mounted: Arc<Mutex<Node>>,
            events: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let event_sink = Arc::clone(&events);
            let mount = Arc::clone(&mounted);
            let mut spec = IconButtonSpec::new()
                .with_icon("pin")
                .with_aria_label("Pin");
            if let Some(value) = pressed {
                spec = spec.with_pressed(value);
            }
            if let Some(value) = default_pressed {
                spec = spec.with_default_pressed(value);
            }
            let mut node = poodle_render::icon_button_with_handlers(
                &spec,
                &RenderContext::new(&theme()),
                IconButtonHandlers {
                    on_pressed_change: Some(Arc::new(move |next| {
                        event_sink.lock().expect("event lock").push(next);
                        *mount.lock().expect("mount lock") = build(
                            Some(next),
                            None,
                            Arc::clone(&mount),
                            Arc::clone(&event_sink),
                        );
                    })),
                    on_click: None,
                },
            );
            node.id = Some("icon-seeded".to_owned());
            node
        }

        let events = Arc::new(Mutex::new(Vec::<bool>::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(None, Some(true), Arc::clone(&mounted), Arc::clone(&events));
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.toggled,
            Some(NodeToggled::True)
        );

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 120.0, 120.0);
        driver.wait_for_focus_handle("icon-seeded");
        driver.focus_element("icon-seeded");
        driver.dispatch_key_raw("enter");
        assert_eq!(*events.lock().expect("event lock"), [false]);
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.toggled,
            Some(NodeToggled::False),
            "the host rebuilds from the reported inverse"
        );
    });
}

/// Collapsible disclosure travels through mounted pointer and keyboard input.
///
/// Deliberately not claimed: content height animation, exact web transition
/// timing, trigger snippets, assistive-technology coverage, visual
/// comparison, or Jetstream admission.
#[test]
fn collapsible_disclosure_and_identity_through_mounted_pointer_and_keyboard() {
    use poodle_render::{
        collapsible_content_focus_id, collapsible_trigger_focus_id, collapsible_with_handlers,
        CollapsibleHandlers,
    };
    use poodle_specs::CollapsibleSpec;

    fn marker(id: &str, label: &str) -> Node {
        let mut node = poodle_render::button(
            &poodle_specs::ButtonSpec::new().with_label(label),
            &RenderContext::new(&theme()),
            None,
        );
        node.id = Some(id.to_owned());
        node
    }

    fn target<'a>(root: &'a Node, id: &str) -> &'a Node {
        root.find(&|node| {
            node.runtime_id.as_deref() == Some(id)
                || node.id.as_deref() == Some(id)
        })
        .unwrap_or_else(|| panic!("{id}"))
    }

    // ── Semantics, naming, inert skips ─────────────────────────────────
    run_headless(|cx| {
        let reported = Arc::new(Mutex::new(Vec::<bool>::new()));
        let sink = Arc::clone(&reported);
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root = root
            .child(marker("before", "Before"))
            .child(collapsible_with_handlers(
                &CollapsibleSpec::new().with_title("Project settings"),
                &RenderContext::new(&theme()),
                Some(Node::text("Build target: production")),
                CollapsibleHandlers {
                    instance_id: Some("closed".to_string()),
                    on_open_change: Some(Arc::new(move |next| {
                        sink.lock().expect("report lock").push(next);
                    })),
                },
            ))
            .child(collapsible_with_handlers(
                &CollapsibleSpec::new().with_aria_label("Hidden section"),
                &RenderContext::new(&theme()),
                None,
                CollapsibleHandlers {
                    instance_id: Some("aria".to_string()),
                    ..CollapsibleHandlers::default()
                },
            ))
            .child(collapsible_with_handlers(
                &CollapsibleSpec::new()
                    .with_title("Locked section")
                    .with_disabled(true),
                &RenderContext::new(&theme()),
                Some(Node::text("secret")),
                CollapsibleHandlers {
                    instance_id: Some("disabled".to_string()),
                    on_open_change: Some(Arc::new(|_| panic!("disabled collapsible does not fire"))),
                },
            ))
            .child(marker("after", "After"));

        let closed_trigger = collapsible_trigger_focus_id("closed");
        let aria_trigger = collapsible_trigger_focus_id("aria");
        let disabled_trigger = collapsible_trigger_focus_id("disabled");

        {
            let trigger = target(&root, &closed_trigger);
            assert_eq!(trigger.a11y.role, Some(NodeRole::Button));
            assert_eq!(trigger.a11y.label.as_deref(), Some("Project settings"));
            assert_eq!(trigger.a11y.expanded, Some(false));
            assert_eq!(
                trigger.a11y.controls.as_deref(),
                Some(collapsible_content_focus_id("closed").as_str())
            );
            assert_eq!(trigger.a11y.tab_index, Some(0));
            assert!(trigger.style.focus_ring.is_some());

            let aria = target(&root, &aria_trigger);
            assert_eq!(aria.a11y.label.as_deref(), Some("Hidden section"));

            let disabled = target(&root, &disabled_trigger);
            assert!(disabled.interaction.disabled);
            assert!(!disabled.interaction.focusable);
            assert_eq!(disabled.a11y.tab_index, None);
            assert!(disabled.interaction.on_activate.is_none());
        }

        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 640.0, 420.0);
        driver.wait_for_focus_handle("before");
        driver.wait_for_focus_handle(&closed_trigger);
        driver.wait_for_focus_handle(&aria_trigger);
        driver.wait_for_focus_handle("after");

        assert!(
            poodle_gpui_node_backend::bounds_for(&closed_trigger).is_some(),
            "pointer proof needs a real hit target"
        );
        driver.pointer_activate_id(&closed_trigger);
        assert_eq!(*reported.lock().expect("report lock"), [true]);

        driver.focus_element("before");
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&closed_trigger),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&aria_trigger),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("after"),
            Some(true),
            "disabled collapsible is skipped"
        );
        assert!(
            poodle_gpui_node_backend::focus_handle_for(&disabled_trigger).is_none(),
            "disabled trigger never registers a sequential stop"
        );
    });

    // ── Controlled rebuild: pointer, Enter, Space ─────────────────────
    run_headless(|cx| {
        fn build(
            open: bool,
            mounted: Arc<Mutex<Node>>,
            events: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let event_sink = Arc::clone(&events);
            let mount = Arc::clone(&mounted);
            collapsible_with_handlers(
                &CollapsibleSpec::new()
                    .with_title("Advanced options")
                    .with_open(open),
                &RenderContext::new(&theme()),
                Some(Node::text(if open {
                    "Cache TTL: 3600s"
                } else {
                    "hidden"
                })),
                CollapsibleHandlers {
                    instance_id: Some("controlled".to_string()),
                    on_open_change: Some(Arc::new(move |next| {
                        event_sink
                            .lock()
                            .expect("event lock")
                            .push(format!("open:{next}"));
                        *mount.lock().expect("mount lock") =
                            build(next, Arc::clone(&mount), Arc::clone(&event_sink));
                    })),
                },
            )
        }

        let events = Arc::new(Mutex::new(Vec::<String>::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(false, Arc::clone(&mounted), Arc::clone(&events));
        let trigger = collapsible_trigger_focus_id("controlled");
        {
            let node = mounted.lock().expect("mount lock");
            let trigger_node = node
                .find(&|n| n.runtime_id.as_deref() == Some(trigger.as_str()))
                .expect("trigger");
            assert_eq!(trigger_node.a11y.expanded, Some(false));
            assert!(node
                .find(&|n| n.runtime_id.as_deref() == Some(collapsible_content_focus_id("controlled").as_str()))
                .is_none());
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle(&trigger);
        driver.pointer_activate_id(&trigger);
        assert_eq!(*events.lock().expect("event lock"), ["open:true".to_string()]);
        assert_eq!(
            mounted
                .lock()
                .expect("mount lock")
                .find(&|n| n.runtime_id.as_deref() == Some(trigger.as_str()))
                .expect("trigger")
                .a11y
                .expanded,
            Some(true)
        );
        assert!(mounted
            .lock()
            .expect("mount lock")
            .find(&|n| n.runtime_id.as_deref() == Some(collapsible_content_focus_id("controlled").as_str()))
            .is_some());

        driver.wait_for_focus_handle(&trigger);
        driver.focus_element(&trigger);
        driver.dispatch_key_raw("enter");
        assert_eq!(
            *events.lock().expect("event lock"),
            ["open:true".to_string(), "open:false".to_string()]
        );

        driver.wait_for_focus_handle(&trigger);
        driver.focus_element(&trigger);
        driver.dispatch_key_raw("space");
        assert_eq!(
            *events.lock().expect("event lock"),
            [
                "open:true".to_string(),
                "open:false".to_string(),
                "open:true".to_string()
            ]
        );
    });

    // ── Default-open seed reports false first ───────────────────────────
    run_headless(|cx| {
        fn build(
            open: Option<bool>,
            default_open: bool,
            mounted: Arc<Mutex<Node>>,
            events: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let event_sink = Arc::clone(&events);
            let mount = Arc::clone(&mounted);
            let mut spec = CollapsibleSpec::new()
                .with_title("Advanced options")
                .with_default_open(default_open);
            if let Some(value) = open {
                spec = spec.with_open(value);
            }
            collapsible_with_handlers(
                &spec,
                &RenderContext::new(&theme()),
                Some(Node::text("seeded content")),
                CollapsibleHandlers {
                    instance_id: Some("seeded".to_string()),
                    on_open_change: Some(Arc::new(move |next| {
                        event_sink.lock().expect("event lock").push(next);
                        *mount.lock().expect("mount lock") = build(
                            Some(next),
                            default_open,
                            Arc::clone(&mount),
                            Arc::clone(&event_sink),
                        );
                    })),
                },
            )
        }

        let events = Arc::new(Mutex::new(Vec::<bool>::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(
            None,
            true,
            Arc::clone(&mounted),
            Arc::clone(&events),
        );
        let trigger = collapsible_trigger_focus_id("seeded");
        {
            let node = mounted.lock().expect("mount lock");
            let trigger_node = node
                .find(&|n| n.runtime_id.as_deref() == Some(trigger.as_str()))
                .expect("trigger");
            assert_eq!(trigger_node.a11y.expanded, Some(true));
            let content_id = collapsible_content_focus_id("seeded");
            let region = node
                .find(&|n| n.runtime_id.as_deref() == Some(content_id.as_str()))
                .expect("region");
            assert_eq!(region.a11y.role, Some(NodeRole::Region));
            assert_eq!(
                region.a11y.labelled_by.as_deref(),
                Some(trigger.as_str())
            );
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle(&trigger);
        driver.focus_element(&trigger);
        driver.dispatch_key_raw("enter");
        assert_eq!(*events.lock().expect("event lock"), [false]);
        assert_eq!(
            mounted
                .lock()
                .expect("mount lock")
                .find(&|n| n.runtime_id.as_deref() == Some(trigger.as_str()))
                .expect("trigger")
                .a11y
                .expanded,
            Some(false)
        );
    });

    // ── Two same-titled instances keep separate backend handles ─────────
    run_headless(|cx| {
        fn build(left_open: bool, right_open: bool, mounted: Arc<Mutex<Node>>) -> Node {
            fn one(
                scope: &str,
                open: bool,
                mounted: &Arc<Mutex<Node>>,
                left_open: bool,
                right_open: bool,
            ) -> Node {
                let mount = Arc::clone(mounted);
                let scope_owned = scope.to_string();
                collapsible_with_handlers(
                    &CollapsibleSpec::new()
                        .with_title("Same title")
                        .with_open(open),
                    &RenderContext::new(&theme()),
                    Some(Node::text(format!("{scope} content"))),
                    CollapsibleHandlers {
                        instance_id: Some(scope.to_string()),
                        on_open_change: Some(Arc::new(move |next| {
                            let (left, right) = if scope_owned == "left" {
                                (next, right_open)
                            } else {
                                (left_open, next)
                            };
                            *mount.lock().expect("mount lock") =
                                build(left, right, Arc::clone(&mount));
                        })),
                    },
                )
            }

            Node::container()
                .child(one("left", left_open, &mounted, left_open, right_open))
                .child(one("right", right_open, &mounted, left_open, right_open))
        }

        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(false, false, Arc::clone(&mounted));
        let left = collapsible_trigger_focus_id("left");
        let right = collapsible_trigger_focus_id("right");
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle(&left);
        driver.wait_for_focus_handle(&right);
        driver.focus_element(&left);
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&left), Some(true));
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right),
            Some(false)
        );

        driver.keyboard_activate(&left);
        assert_eq!(
            mounted
                .lock()
                .expect("mount lock")
                .find(&|n| n.runtime_id.as_deref() == Some(left.as_str()))
                .expect("left trigger")
                .a11y
                .expanded,
            Some(true)
        );
        assert_eq!(
            mounted
                .lock()
                .expect("mount lock")
                .find(&|n| n.runtime_id.as_deref() == Some(right.as_str()))
                .expect("right trigger")
                .a11y
                .expanded,
            Some(false),
            "activating one instance does not expand the other"
        );
    });
}

/// Standalone CollapseToggle label, expanded state, direction, host-owned
/// rebuild, focus, and disabled inertia through the production renderer.
#[test]
fn collapse_toggle_disclosure_focus_and_disabled_through_mounted_pointer_and_keyboard() {
    use poodle_node::NodeKind;
    use poodle_specs::{CollapseDirection, CollapseToggleSpec};

    fn marker(id: &str, label: &str) -> Node {
        let mut node = poodle_render::button(
            &poodle_specs::ButtonSpec::new().with_label(label),
            &RenderContext::new(&theme()),
            None,
        );
        node.id = Some(id.to_owned());
        node
    }

    fn toggle(
        spec: CollapseToggleSpec,
        id: &str,
        on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    ) -> Node {
        let mut node =
            poodle_render::collapse_toggle(&spec, &RenderContext::new(&theme()), on_toggle);
        node.id = Some(id.to_owned());
        node
    }

    fn target<'a>(root: &'a Node, id: &str) -> &'a Node {
        root.find(&|node| node.id.as_deref() == Some(id))
            .unwrap_or_else(|| panic!("{id}"))
    }

    fn icon_name(node: &Node) -> &str {
        node.find(&|child| matches!(&child.kind, NodeKind::Icon { .. }))
            .and_then(|child| match &child.kind {
                NodeKind::Icon { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .expect("chevron")
    }

    // ── Semantics, naming, inert skips ─────────────────────────────────
    run_headless(|cx| {
        let reported = Arc::new(Mutex::new(Vec::<bool>::new()));
        let sink = Arc::clone(&reported);
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root = root
            .child(marker("ct-before", "Before"))
            .child(toggle(
                CollapseToggleSpec::new().with_direction(CollapseDirection::Left),
                "ct-enabled",
                Some(Arc::new(move |next| {
                    sink.lock().expect("report lock").push(next);
                })),
            ))
            .child(toggle(
                CollapseToggleSpec::new()
                    .with_collapsed(true)
                    .with_aria_label("Collapse left dock"),
                "ct-labeled",
                None,
            ))
            .child(toggle(
                CollapseToggleSpec::new().with_disabled(true),
                "ct-disabled",
                Some(Arc::new(|_| panic!("disabled collapse toggle does not fire"))),
            ))
            .child(marker("ct-after", "After"));

        {
            let enabled = target(&root, "ct-enabled");
            assert_eq!(enabled.a11y.role, Some(NodeRole::Button));
            assert_eq!(enabled.a11y.label.as_deref(), Some("Collapse"));
            assert_eq!(enabled.a11y.expanded, Some(true));
            assert_eq!(icon_name(enabled), "chevron-left");
            assert_eq!(enabled.a11y.tab_index, Some(0));
            assert!(enabled.style.focus_ring.is_some());

            let labeled = target(&root, "ct-labeled");
            assert_eq!(labeled.a11y.label.as_deref(), Some("Collapse left dock"));
            assert_eq!(labeled.a11y.expanded, Some(false));
            assert_eq!(icon_name(labeled), "chevron-right");

            let disabled = target(&root, "ct-disabled");
            assert!(disabled.interaction.disabled);
            assert!(!disabled.interaction.focusable);
            assert_eq!(disabled.a11y.tab_index, None);
            assert!(disabled.style.focus_ring.is_none());
            assert!(disabled.interaction.on_activate.is_none());
        }

        let mounted = Arc::new(Mutex::new(root));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 640.0, 420.0);
        driver.wait_for_focus_handle("ct-before");
        driver.wait_for_focus_handle("ct-enabled");
        driver.wait_for_focus_handle("ct-labeled");
        driver.wait_for_focus_handle("ct-after");

        assert!(
            poodle_gpui_node_backend::bounds_for("ct-enabled").is_some(),
            "pointer proof needs a real hit target"
        );
        driver.pointer_activate_id("ct-enabled");
        assert_eq!(*reported.lock().expect("report lock"), [true]);

        assert!(
            poodle_gpui_node_backend::bounds_for("ct-disabled").is_some(),
            "disabled pointer proof needs a real hit target"
        );
        driver.pointer_activate_id("ct-disabled");
        assert!(
            poodle_gpui_node_backend::focus_handle_for("ct-disabled").is_none(),
            "disabled toggle never registers a sequential stop"
        );
        assert_eq!(*reported.lock().expect("report lock"), [true]);

        driver.focus_element("ct-before");
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("ct-enabled"),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("ct-labeled"),
            Some(true)
        );
        driver.focus_next_tab_stop();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("ct-after"),
            Some(true),
            "disabled collapse toggle is skipped"
        );
    });

    // ── Host rebuild: pointer, Enter, Space ────────────────────────────
    run_headless(|cx| {
        fn build(
            collapsed: bool,
            mounted: Arc<Mutex<Node>>,
            events: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let event_sink = Arc::clone(&events);
            let mount = Arc::clone(&mounted);
            let mut node = poodle_render::collapse_toggle(
                &CollapseToggleSpec::new()
                    .with_direction(CollapseDirection::Left)
                    .with_collapsed(collapsed),
                &RenderContext::new(&theme()),
                Some(Arc::new(move |next| {
                    event_sink.lock().expect("event lock").push(next);
                    *mount.lock().expect("mount lock") =
                        build(next, Arc::clone(&mount), Arc::clone(&event_sink));
                })),
            );
            node.id = Some("ct-controlled".to_owned());
            node
        }

        let events = Arc::new(Mutex::new(Vec::<bool>::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(false, Arc::clone(&mounted), Arc::clone(&events));
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.label.as_deref(), Some("Collapse"));
            assert_eq!(node.a11y.expanded, Some(true));
            assert_eq!(icon_name(&node), "chevron-left");
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("ct-controlled");
        driver.pointer_activate_id("ct-controlled");
        assert_eq!(*events.lock().expect("event lock"), [true]);
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.label.as_deref(), Some("Expand"));
            assert_eq!(node.a11y.expanded, Some(false));
            assert_eq!(icon_name(&node), "chevron-right");
        }

        driver.wait_for_focus_handle("ct-controlled");
        driver.focus_element("ct-controlled");
        driver.dispatch_key_raw("enter");
        assert_eq!(*events.lock().expect("event lock"), [true, false]);
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.label.as_deref(), Some("Collapse"));
            assert_eq!(node.a11y.expanded, Some(true));
            assert_eq!(icon_name(&node), "chevron-left");
        }

        driver.wait_for_focus_handle("ct-controlled");
        driver.focus_element("ct-controlled");
        driver.dispatch_key_raw("space");
        assert_eq!(*events.lock().expect("event lock"), [true, false, true]);
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.label.as_deref(), Some("Expand"));
            assert_eq!(node.a11y.expanded, Some(false));
            assert_eq!(icon_name(&node), "chevron-right");
        }
    });

    // ── Explicit label survives host rebuild ───────────────────────────
    run_headless(|cx| {
        fn build(
            collapsed: bool,
            mounted: Arc<Mutex<Node>>,
            events: Arc<Mutex<Vec<bool>>>,
        ) -> Node {
            let event_sink = Arc::clone(&events);
            let mount = Arc::clone(&mounted);
            let mut node = poodle_render::collapse_toggle(
                &CollapseToggleSpec::new()
                    .with_collapsed(collapsed)
                    .with_aria_label("Collapse left dock"),
                &RenderContext::new(&theme()),
                Some(Arc::new(move |next| {
                    event_sink.lock().expect("event lock").push(next);
                    *mount.lock().expect("mount lock") =
                        build(next, Arc::clone(&mount), Arc::clone(&event_sink));
                })),
            );
            node.id = Some("ct-named".to_owned());
            node
        }

        let events = Arc::new(Mutex::new(Vec::<bool>::new()));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(false, Arc::clone(&mounted), Arc::clone(&events));
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.label.as_deref(),
            Some("Collapse left dock")
        );
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.expanded,
            Some(true)
        );

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 240.0);
        driver.wait_for_focus_handle("ct-named");
        driver.pointer_activate_id("ct-named");
        assert_eq!(*events.lock().expect("event lock"), [true]);
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.label.as_deref(), Some("Collapse left dock"));
            assert_eq!(node.a11y.expanded, Some(false));
        }

        driver.wait_for_focus_handle("ct-named");
        driver.focus_element("ct-named");
        driver.dispatch_key_raw("enter");
        assert_eq!(*events.lock().expect("event lock"), [true, false]);
        {
            let node = mounted.lock().expect("mount lock");
            assert_eq!(node.a11y.label.as_deref(), Some("Collapse left dock"));
            assert_eq!(node.a11y.expanded, Some(true));
        }
    });
}

/// g16.016. Pagination page destinations, variant summaries, boundary/loading
/// inertia, and the wired page-size Select through real mounted GPUI pointer
/// and keyboard dispatch with host-owned rebuilds.
///
/// Deliberately not claimed: Select's own mounted ledger cell, navigation
/// landmark/current-page accessibility, visual comparison, or Jetstream.
#[test]
fn pagination_navigation_limit_and_loading_through_mounted_pointer_and_keyboard() {
    use poodle_specs::{PaginationSpec, PaginationVariant};

    #[derive(Clone)]
    struct Host {
        page: usize,
        total_pages: usize,
        variant: PaginationVariant,
        page_size: usize,
        limit_open: bool,
        loading: bool,
        pages: Vec<usize>,
        opens: Vec<bool>,
        sizes: Vec<usize>,
    }

    fn stamp_ids(node: &mut Node) {
        if let Some(label) = node.a11y.label.clone() {
            match label.as_str() {
                "Previous page" => node.id = Some("pagination-prev".to_owned()),
                "Next page" => node.id = Some("pagination-next".to_owned()),
                "Items per page" => {
                    node.id = Some("pagination-limit".to_owned());
                    node.runtime_id = Some("pagination-limit".to_owned());
                }
                _ => {}
            }
        }
        if let NodeKind::Button { label } = &node.kind {
            let id = match label.as_str() {
                "Prev" => Some("pagination-simple-prev".to_owned()),
                "Next" => Some("pagination-simple-next".to_owned()),
                "««" => Some("pagination-first".to_owned()),
                "»»" => Some("pagination-last".to_owned()),
                digits if digits.chars().all(|c| c.is_ascii_digit()) && !digits.is_empty() => {
                    Some(format!("pagination-page-{digits}"))
                }
                _ => None,
            };
            if let Some(id) = id {
                node.id = Some(id);
            }
        }
        if node.id.is_none()
            && node.interaction.focusable
            && node
                .find(&|c| matches!(&c.kind, NodeKind::Icon { name, .. } if name == "chevron-down"))
                .is_some()
            && node
                .find(&|c| c.has_text("10") || c.has_text("25") || c.has_text("50"))
                .is_some()
        {
            node.id = Some("pagination-limit".to_owned());
            node.runtime_id = Some("pagination-limit".to_owned());
        }
        for child in &mut node.children {
            stamp_ids(child);
        }
    }

    fn build(host: Arc<Mutex<Host>>, mounted: Arc<Mutex<Node>>) -> Node {
        let state = host.lock().expect("host lock").clone();
        let page_host = Arc::clone(&host);
        let page_mount = Arc::clone(&mounted);
        let open_host = Arc::clone(&host);
        let open_mount = Arc::clone(&mounted);
        let size_host = Arc::clone(&host);
        let size_mount = Arc::clone(&mounted);

        let mut spec = PaginationSpec::new()
            .with_current_page(state.page)
            .with_total_pages(state.total_pages)
            .with_sibling_count(1)
            .with_variant(state.variant)
            .with_page_size(state.page_size)
            .with_total_items(248)
            .with_show_limit_selector(true)
            .with_limit_options(vec![10, 25, 50])
            .with_aria_label("Results pagination")
            .with_loading(state.loading);
        if state.variant == PaginationVariant::Simple {
            // Simple summary uses item range; keep totals for the label.
            spec = spec.with_show_info(false);
        }

        let mut node = poodle_render::pagination_with_handlers(
            &spec,
            &RenderContext::new(&theme()),
            &poodle_render::PaginationHandlers {
                page_change: Some(Arc::new(move |page| {
                    let mut host = page_host.lock().expect("host lock");
                    host.pages.push(page);
                    host.page = page;
                    drop(host);
                    *page_mount.lock().expect("mount lock") =
                        build(Arc::clone(&page_host), Arc::clone(&page_mount));
                })),
                limit_open: state.limit_open,
                limit_open_change: Some(Arc::new(move |open| {
                    let mut host = open_host.lock().expect("host lock");
                    host.opens.push(open);
                    host.limit_open = open;
                    drop(host);
                    *open_mount.lock().expect("mount lock") =
                        build(Arc::clone(&open_host), Arc::clone(&open_mount));
                })),
                page_size_change: Some(Arc::new(move |size| {
                    let mut host = size_host.lock().expect("host lock");
                    host.sizes.push(size);
                    host.page_size = size;
                    host.limit_open = false;
                    drop(host);
                    *size_mount.lock().expect("mount lock") =
                        build(Arc::clone(&size_host), Arc::clone(&size_mount));
                })),
                ..poodle_render::PaginationHandlers::new("pagination-limit")
            },
        );
        stamp_ids(&mut node);
        node
    }

    fn target<'a>(root: &'a Node, id: &str) -> &'a Node {
        root.find(&|n| n.id.as_deref() == Some(id))
            .unwrap_or_else(|| panic!("missing {id}"))
    }

    // ── Numbered destinations, inertia, pointer / Enter / Space ────────
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            page: 5,
            total_pages: 20,
            variant: PaginationVariant::Numbered,
            page_size: 10,
            limit_open: false,
            loading: false,
            pages: Vec::new(),
            opens: Vec::new(),
            sizes: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        {
            let root = mounted.lock().expect("mount lock");
            assert!(root.has_text("..."));
            assert!(target(&root, "pagination-page-5")
                .interaction
                .on_activate
                .is_none());
            assert!(!target(&root, "pagination-prev").interaction.disabled);
            assert!(!target(&root, "pagination-next").interaction.disabled);
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 160.0);
        driver.wait_for_focus_handle("pagination-page-4");
        assert!(
            poodle_gpui_node_backend::bounds_for("pagination-page-4").is_some(),
            "pointer proof needs a real hit target"
        );
        driver.pointer_activate_id("pagination-page-4");
        assert_eq!(host.lock().expect("host lock").pages, [4]);
        assert_eq!(host.lock().expect("host lock").page, 4);
        {
            let root = mounted.lock().expect("mount lock");
            assert!(target(&root, "pagination-page-4")
                .interaction
                .on_activate
                .is_none());
        }

        // After landing on 4, the window is 1 … 3 4 5 … 20 — drive Enter/Space
        // against pages that stay visible.
        driver.wait_for_focus_handle("pagination-page-5");
        driver.focus_element("pagination-page-5");
        driver.dispatch_key_raw("enter");
        assert_eq!(host.lock().expect("host lock").pages, [4, 5]);
        assert_eq!(host.lock().expect("host lock").page, 5);

        driver.wait_for_focus_handle("pagination-page-6");
        driver.focus_element("pagination-page-6");
        driver.dispatch_key_raw("space");
        assert_eq!(host.lock().expect("host lock").pages, [4, 5, 6]);
        assert_eq!(host.lock().expect("host lock").page, 6);

        driver.wait_for_focus_handle("pagination-next");
        driver.pointer_activate_id("pagination-next");
        assert_eq!(host.lock().expect("host lock").pages, [4, 5, 6, 7]);

        driver.wait_for_focus_handle("pagination-prev");
        driver.pointer_activate_id("pagination-prev");
        assert_eq!(host.lock().expect("host lock").pages, [4, 5, 6, 7, 6]);

        // Current page stays inert under pointer.
        let before = host.lock().expect("host lock").pages.len();
        assert!(
            poodle_gpui_node_backend::bounds_for("pagination-page-6").is_some(),
            "current page still has a hit target"
        );
        driver.pointer_activate_id("pagination-page-6");
        assert_eq!(host.lock().expect("host lock").pages.len(), before);
    });

    // ── Boundary disabled first/last pages ─────────────────────────────
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            page: 1,
            total_pages: 5,
            variant: PaginationVariant::Numbered,
            page_size: 10,
            limit_open: false,
            loading: false,
            pages: Vec::new(),
            opens: Vec::new(),
            sizes: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        {
            let root = mounted.lock().expect("mount lock");
            assert!(target(&root, "pagination-prev").interaction.disabled);
            assert!(target(&root, "pagination-prev")
                .interaction
                .on_activate
                .is_none());
        }
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 140.0);
        driver.draw_frame();
        assert!(
            poodle_gpui_node_backend::bounds_for("pagination-prev").is_some(),
            "disabled prev still paints a hit target"
        );
        driver.pointer_activate_id("pagination-prev");
        driver.wait_for_focus_handle("pagination-next");
        driver.focus_element("pagination-next");
        driver.dispatch_key_raw("enter");
        assert_eq!(host.lock().expect("host lock").pages, [2]);

        // Move to last page and prove next is inert.
        host.lock().expect("host lock").page = 5;
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        {
            let root = mounted.lock().expect("mount lock");
            assert!(target(&root, "pagination-next").interaction.disabled);
            assert!(target(&root, "pagination-next")
                .interaction
                .on_activate
                .is_none());
        }
        driver.draw_frame();
        let before = host.lock().expect("host lock").pages.len();
        driver.pointer_activate_id("pagination-next");
        assert_eq!(host.lock().expect("host lock").pages.len(), before);
    });

    // ── Simple and full variants ───────────────────────────────────────
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            page: 3,
            total_pages: 10,
            variant: PaginationVariant::Simple,
            page_size: 25,
            limit_open: false,
            loading: false,
            pages: Vec::new(),
            opens: Vec::new(),
            sizes: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        assert!(mounted.lock().expect("mount lock").has_text("51–75 of 248"));

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 520.0, 140.0);
        driver.wait_for_focus_handle("pagination-simple-next");
        driver.pointer_activate_id("pagination-simple-next");
        assert_eq!(host.lock().expect("host lock").pages, [4]);
        driver.wait_for_focus_handle("pagination-simple-prev");
        driver.pointer_activate_id("pagination-simple-prev");
        assert_eq!(host.lock().expect("host lock").pages, [4, 3]);

        host.lock().expect("host lock").variant = PaginationVariant::Full;
        host.lock().expect("host lock").pages.clear();
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        assert!(mounted.lock().expect("mount lock").has_text("Page 3 of 10"));
        driver.draw_frame();
        driver.wait_for_focus_handle("pagination-first");
        driver.pointer_activate_id("pagination-first");
        assert_eq!(host.lock().expect("host lock").pages, [1]);
        driver.wait_for_focus_handle("pagination-last");
        driver.pointer_activate_id("pagination-last");
        assert_eq!(host.lock().expect("host lock").pages, [1, 10]);
    });

    // ── Wired page-size Select open + choose ────────────────────────────
    run_headless(|cx| {
        // Open path: closed trigger reports the next open state.
        let host = Arc::new(Mutex::new(Host {
            page: 2,
            total_pages: 10,
            variant: PaginationVariant::Numbered,
            page_size: 10,
            limit_open: false,
            loading: false,
            pages: Vec::new(),
            opens: Vec::new(),
            sizes: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 420.0);
        driver.draw_frame();
        assert!(
            poodle_gpui_node_backend::bounds_for("pagination-limit").is_some(),
            "limit Select needs a real hit target"
        );
        driver.pointer_activate_id("pagination-limit");
        assert_eq!(host.lock().expect("host lock").opens, [true]);
        assert!(host.lock().expect("host lock").limit_open);
    });

    run_headless(|cx| {
        // Choose path: pointer on the production deferred option reports a
        // parsed limit and rebuilds closed. No test-only option id or ring.
        let host = Arc::new(Mutex::new(Host {
            page: 2,
            total_pages: 10,
            variant: PaginationVariant::Numbered,
            page_size: 10,
            limit_open: false,
            loading: false,
            pages: Vec::new(),
            opens: Vec::new(),
            sizes: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        let option_id = poodle_render::select_option_id("pagination-limit", "25");

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 420.0);
        driver.pointer_activate_id("pagination-limit");
        assert!(host.lock().expect("host lock").limit_open);
        driver.draw_frame();
        {
            let root = mounted.lock().expect("mount lock");
            let option = root
                .find(&|n| n.runtime_id.as_deref() == Some(option_id.as_str()))
                .expect("production option id");
            assert!(
                option.interaction.on_activate.is_some(),
                "open option must carry the production change handler"
            );
            assert!(
                !option.interaction.focusable,
                "option rows are pointer targets, not a keyboard workaround"
            );
        }
        assert!(
            poodle_gpui_node_backend::bounds_for(&option_id).is_some(),
            "deferred limit option is a real pointer target"
        );
        driver.pointer_activate_id(&option_id);
        assert_eq!(host.lock().expect("host lock").sizes, [25]);
        assert!(!host.lock().expect("host lock").limit_open);
        assert_eq!(host.lock().expect("host lock").page_size, 25);
        {
            let root = mounted.lock().expect("mount lock");
            assert!(root.has_text("25"));
            assert!(
                root.find(&|n| n.a11y.role == Some(NodeRole::ListBoxOption))
                    .is_none(),
                "host rebuild closes the Select after a limit choice"
            );
        }
    });

    // ── Loading suppresses page, open, and page-size events ────────────
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            page: 5,
            total_pages: 20,
            variant: PaginationVariant::Numbered,
            page_size: 10,
            limit_open: false,
            loading: true,
            pages: Vec::new(),
            opens: Vec::new(),
            sizes: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        {
            let root = mounted.lock().expect("mount lock");
            assert!(target(&root, "pagination-page-4").interaction.disabled);
            assert!(target(&root, "pagination-next").interaction.disabled);
            assert!(target(&root, "pagination-limit").interaction.disabled);
            assert!(target(&root, "pagination-limit")
                .interaction
                .on_activate
                .is_none());
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 720.0, 160.0);
        driver.draw_frame();
        for id in [
            "pagination-page-4",
            "pagination-next",
            "pagination-prev",
            "pagination-limit",
        ] {
            assert!(
                poodle_gpui_node_backend::bounds_for(id).is_some(),
                "{id} still paints under loading"
            );
            driver.pointer_activate_id(id);
        }
        // Keyboard on a disabled limit / next must also stay silent.
        if poodle_gpui_node_backend::focus_handle_for("pagination-limit").is_some() {
            driver.focus_element("pagination-limit");
            driver.dispatch_key_raw("enter");
            driver.dispatch_key_raw("space");
        }
        assert!(host.lock().expect("host lock").pages.is_empty());
        assert!(host.lock().expect("host lock").opens.is_empty());
        assert!(host.lock().expect("host lock").sizes.is_empty());
    });
}

#[test]
fn rating_nullable_fractional_and_whole_step_through_mounted_pointer_and_keyboard() {
    #[derive(Clone)]
    struct Host {
        value: Option<f64>,
        step: f64,
        allow_clear: bool,
        disabled: bool,
        payloads: Vec<Option<f64>>,
    }

    fn item_id(scope: &str, value: u8) -> String {
        format!("rating:{scope}:item:{value}")
    }

    fn root_id(scope: &str) -> String {
        format!("rating:{scope}:root")
    }

    fn find_item<'a>(node: &'a Node, scope: &str, value: u8) -> &'a Node {
        let id = item_id(scope, value);
        node.find(&|n| n.runtime_id.as_deref() == Some(id.as_str()))
            .unwrap_or_else(|| panic!("missing {id}"))
    }

    fn build(host: Arc<Mutex<Host>>, mounted: Arc<Mutex<Node>>, scope: &str) -> Node {
        let state = host.lock().expect("host lock").clone();
        let mut spec = RatingSpec::new()
            .with_step(state.step)
            .with_allow_clear(state.allow_clear)
            .with_disabled(state.disabled)
            .with_aria_label("Mounted rating");
        if let Some(value) = state.value {
            spec = spec.with_value(value);
        }
        let rebuild_host = Arc::clone(&host);
        let rebuild_mount = Arc::clone(&mounted);
        let scope_owned = scope.to_owned();
        let mut node = poodle_render::rating(
            &spec,
            &RenderContext::new(&theme()),
            RatingHandlers::new(scope).on_change(Arc::new(move |next| {
                let mut host = rebuild_host.lock().expect("host lock");
                host.payloads.push(next);
                host.value = next;
                drop(host);
                *rebuild_mount.lock().expect("mount lock") = build(
                    Arc::clone(&rebuild_host),
                    Arc::clone(&rebuild_mount),
                    &scope_owned,
                );
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        node
    }

    // Fractional default half-step: pointer, keys, clear, disabled inertia.
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            value: Some(2.0),
            step: 0.5,
            allow_clear: true,
            disabled: false,
            payloads: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(Arc::clone(&host), Arc::clone(&mounted), "half");
        {
            let root = mounted.lock().expect("mount lock");
            assert_eq!(root.a11y.role, Some(NodeRole::Slider));
            assert_eq!(root.a11y.value, Some(2.0));
            assert_eq!(root.a11y.value_text.as_deref(), Some("2 out of 5"));
            assert!(!find_item(&root, "half", 3).interaction.focusable);
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 320.0, 64.0);
        let third = item_id("half", 3);
        driver.wait_for_focus_handle(&root_id("half"));
        assert!(
            poodle_gpui_node_backend::bounds_for(&third).is_some(),
            "fractional star needs a real hit target"
        );
        // Center of star 3 → ratio 0.5 → snap-up half step → 2.5.
        driver.pointer_activate_id(&third);
        assert_eq!(host.lock().expect("host lock").payloads, [Some(2.5)]);
        assert_eq!(host.lock().expect("host lock").value, Some(2.5));
        {
            let root = mounted.lock().expect("mount lock");
            assert_eq!(root.a11y.value, Some(2.5));
            assert_eq!(root.a11y.value_text.as_deref(), Some("2.5 out of 5"));
        }

        driver.focus_element(&root_id("half"));
        driver.dispatch_key_raw("right");
        assert_eq!(
            host.lock().expect("host lock").payloads,
            [Some(2.5), Some(3.0)]
        );
        driver.dispatch_key_raw("home");
        assert_eq!(
            host.lock().expect("host lock").payloads,
            [Some(2.5), Some(3.0), Some(0.0)]
        );
        driver.dispatch_key_raw("end");
        assert_eq!(host.lock().expect("host lock").value, Some(5.0));
        driver.dispatch_key_raw("space");
        assert_eq!(host.lock().expect("host lock").value, None);
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.value_text.as_deref(),
            Some("No rating selected out of 5")
        );
        // Enter clear uses on_submit on the focused slider root.
        driver.pointer_activate_id(&third);
        assert_eq!(host.lock().expect("host lock").value, Some(2.5));
        driver.focus_element(&root_id("half"));
        driver.dispatch_key_raw("enter");
        assert_eq!(host.lock().expect("host lock").value, None);
        assert_eq!(
            mounted.lock().expect("mount lock").a11y.value_text.as_deref(),
            Some("No rating selected out of 5")
        );
    });

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            value: Some(3.0),
            step: 0.5,
            allow_clear: true,
            disabled: true,
            payloads: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(Arc::clone(&host), Arc::clone(&mounted), "disabled");
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 320.0, 64.0);
        driver.draw_frame();
        driver.pointer_activate_id(&item_id("disabled", 4));
        assert!(host.lock().expect("host lock").payloads.is_empty());
    });

    // Whole-step radiogroup: roving focus without selection, then activate.
    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            value: Some(2.0),
            step: 1.0,
            allow_clear: true,
            disabled: false,
            payloads: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") =
            build(Arc::clone(&host), Arc::clone(&mounted), "whole");
        {
            let root = mounted.lock().expect("mount lock");
            assert_eq!(root.a11y.role, Some(NodeRole::RadioGroup));
            assert_eq!(find_item(&root, "whole", 2).a11y.selected, Some(true));
            assert_eq!(find_item(&root, "whole", 2).a11y.tab_index, Some(0));
            assert_eq!(find_item(&root, "whole", 1).a11y.selected, Some(false));
        }

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 320.0, 64.0);
        let two = item_id("whole", 2);
        let three = item_id("whole", 3);
        let one = item_id("whole", 1);
        let five = item_id("whole", 5);
        driver.wait_for_focus_handle(&two);
        driver.focus_element(&two);
        driver.dispatch_key_raw("right");
        assert!(
            host.lock().expect("host lock").payloads.is_empty(),
            "arrows move focus without selecting"
        );
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&three),
            Some(true)
        );

        driver.dispatch_key_raw("home");
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&one), Some(true));
        assert!(host.lock().expect("host lock").payloads.is_empty());

        // Web clamps: Left/Down on the first star and Right/Up on the last are inert.
        driver.dispatch_key_raw("left");
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&one), Some(true));
        driver.dispatch_key_raw("down");
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&one), Some(true));
        assert!(host.lock().expect("host lock").payloads.is_empty());

        driver.dispatch_key_raw("end");
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&five), Some(true));
        assert!(host.lock().expect("host lock").payloads.is_empty());
        driver.dispatch_key_raw("right");
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&five), Some(true));
        driver.dispatch_key_raw("up");
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&five), Some(true));
        assert!(host.lock().expect("host lock").payloads.is_empty());

        driver.dispatch_key_raw("space");
        assert_eq!(host.lock().expect("host lock").payloads, [Some(5.0)]);
        assert_eq!(host.lock().expect("host lock").value, Some(5.0));
        assert_eq!(
            find_item(&mounted.lock().expect("mount lock"), "whole", 5)
                .a11y
                .selected,
            Some(true)
        );

        driver.focus_element(&three);
        driver.dispatch_key_raw("enter");
        assert_eq!(
            host.lock().expect("host lock").payloads,
            [Some(5.0), Some(3.0)]
        );
        assert_eq!(host.lock().expect("host lock").value, Some(3.0));
        assert_eq!(
            find_item(&mounted.lock().expect("mount lock"), "whole", 3)
                .a11y
                .selected,
            Some(true)
        );

        driver.wait_for_focus_handle(&three);
        driver.pointer_activate_id(&three);
        assert_eq!(
            host.lock().expect("host lock").payloads,
            [Some(5.0), Some(3.0), None],
            "clearable whole-step reselect clears"
        );
        assert_eq!(host.lock().expect("host lock").value, None);
    });

    // Arbitrary display fraction is not quantized; separate instances keep focus ids.
    run_headless(|cx| {
        let left = poodle_render::rating(
            &RatingSpec::new().with_value(3.7),
            &RenderContext::new(&theme()),
            RatingHandlers::new("left"),
        );
        assert!((RatingSpec::new().with_value(3.7).fill_ratio(3) - 0.7).abs() < 1e-9);
        assert_eq!(left.a11y.value, Some(3.7));

        let mut tree = Node::container()
            .child(poodle_render::rating(
                &RatingSpec::new().with_value(1.0).with_step(1.0),
                &RenderContext::new(&theme()),
                RatingHandlers::new("left"),
            ))
            .child(poodle_render::rating(
                &RatingSpec::new().with_value(1.0).with_step(1.0),
                &RenderContext::new(&theme()),
                RatingHandlers::new("right"),
            ));
        tree.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::new(Mutex::new(tree)), 480.0, 64.0);
        let left_id = item_id("left", 1);
        let right_id = item_id("right", 1);
        driver.wait_for_focus_handle(&left_id);
        driver.wait_for_focus_handle(&right_id);
        assert_ne!(left_id, right_id);
    });
}

/// g16.019. Two independently scoped Selects open, choose, type, clear, and
/// dismiss through real GPUI pointer and keyboard dispatch with host-owned
/// rebuilds. Pointer proof uses production option identity, not a test-only
/// ring or id stamp.
#[test]
fn select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds() {
    use poodle_render::{
        select, select_option_id, select_search_focus_id, select_trigger_focus_id, SelectHandlers,
    };
    use poodle_specs::{ChoiceOption, SelectSpec};

    #[derive(Clone)]
    struct Instance {
        spec: SelectSpec,
        values: Vec<String>,
        queries: Vec<String>,
        opens: Vec<bool>,
        transitions: usize,
    }

    #[derive(Clone)]
    struct Host {
        left: Instance,
        right: Instance,
    }

    fn fruit() -> Vec<ChoiceOption> {
        vec![
            ChoiceOption::new("apple", "Apple"),
            ChoiceOption::new("banana", "Banana"),
            ChoiceOption::new("cherry", "Cherry"),
            {
                let mut spinach = ChoiceOption::new("spinach", "Spinach");
                spinach.is_disabled = true;
                spinach.group = Some("Vegetables".to_owned());
                spinach
            },
        ]
    }

    fn apply_result(instance: &mut Instance, result: poodle_render::SelectTransitionResult) {
        instance.transitions += 1;
        let previous_query = instance.spec.search_query.clone();
        instance.spec = instance.spec.clone().applying_context(&result.context);
        if let Some((start, end)) = result.search_selection {
            instance.spec.search_selection_start = start;
            instance.spec.search_selection_end = end;
        } else if instance.spec.search_query != previous_query {
            let len = instance
                .spec
                .search_query
                .as_deref()
                .unwrap_or("")
                .chars()
                .count();
            instance.spec.search_selection_start = len;
            instance.spec.search_selection_end = len;
        }
        for effect in result.effects {
            match effect {
                poodle_render::SelectEffect::OpenChanged { open } => instance.opens.push(open),
                poodle_render::SelectEffect::QueryChanged { query } => instance.queries.push(query),
                poodle_render::SelectEffect::ValueChanged { value } => instance.values.push(value),
            }
        }
    }

    fn build(host: Arc<Mutex<Host>>, mounted: Arc<Mutex<Node>>) -> Node {
        let state = host.lock().expect("host lock").clone();
        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 24.0;

        for (scope, instance) in [("left", state.left), ("right", state.right)] {
            let host_i = Arc::clone(&host);
            let mount_i = Arc::clone(&mounted);
            let is_left = scope == "left";
            let handlers = SelectHandlers::new(scope).on_transition(Arc::new(move |result| {
                let mut host = host_i.lock().expect("host lock");
                if is_left {
                    apply_result(&mut host.left, result);
                } else {
                    apply_result(&mut host.right, result);
                }
                let request_search = if is_left {
                    host.left.spec.searchable && host.left.spec.current_open()
                } else {
                    host.right.spec.searchable && host.right.spec.current_open()
                };
                drop(host);
                if request_search {
                    poodle_gpui_node_backend::request_focus(&select_search_focus_id(scope));
                }
                *mount_i.lock().expect("mount lock") =
                    build(Arc::clone(&host_i), Arc::clone(&mount_i));
            }));
            root = root.child(select(
                &instance.spec,
                &RenderContext::new(&theme()),
                &handlers,
            ));
        }
        root
    }

    run_headless(|cx| {
        let left = Instance {
            spec: SelectSpec::new(fruit())
                .with_placeholder("Left fruit")
                .with_clearable(true),
            values: Vec::new(),
            queries: Vec::new(),
            opens: Vec::new(),
            transitions: 0,
        };
        let mut right_spec = SelectSpec::new(fruit())
            .with_placeholder("Right fruit")
            .with_searchable(true)
            .with_freeform(true);
        right_spec.searchable = true;
        right_spec.freeform = true;
        let right = Instance {
            spec: right_spec,
            values: Vec::new(),
            queries: Vec::new(),
            opens: Vec::new(),
            transitions: 0,
        };
        let host = Arc::new(Mutex::new(Host { left, right }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));

        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 360.0, 420.0);
        let left_trigger = select_trigger_focus_id("left");
        let left_banana = select_option_id("left", "banana");
        let left_spinach = select_option_id("left", "spinach");
        let right_trigger = select_trigger_focus_id("right");
        let right_search = select_search_focus_id("right");
        driver.wait_for_focus_handle(&left_trigger);
        driver.wait_for_focus_handle(&right_trigger);

        driver.pointer_activate_id(&left_trigger);
        assert_eq!(host.lock().expect("host lock").left.opens, [true]);
        driver.draw_frame();
        let listbox = "select:left:listbox";
        assert!(
            poodle_gpui_node_backend::bounds_for(listbox).is_some(),
            "open panel records containment bounds"
        );
        let group_header = "select:left:group-Vegetables";
        assert!(
            poodle_gpui_node_backend::bounds_for(group_header).is_some(),
            "group header is a real painted target"
        );
        driver.pointer_activate_id(group_header);
        {
            let host = host.lock().expect("host lock");
            assert!(
                host.left.spec.current_open(),
                "a click on a group header is inside the layer"
            );
            assert!(host.left.values.is_empty());
        }
        assert!(
            poodle_gpui_node_backend::bounds_for(&left_spinach).is_some(),
            "disabled deferred option still paints"
        );
        driver.pointer_activate_id(&left_spinach);
        {
            let host = host.lock().expect("host lock");
            assert!(host.left.values.is_empty(), "disabled option is inert");
            assert!(host.left.spec.current_open());
        }
        assert!(
            poodle_gpui_node_backend::bounds_for(&left_banana).is_some(),
            "left deferred option is a real pointer target"
        );
        driver.pointer_activate_id(&left_banana);
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.left.values, ["banana"]);
            assert!(!host.left.spec.current_open());
            assert!(host.right.values.is_empty());
        }

        driver.pointer_activate_id(&left_trigger);
        assert!(host.lock().expect("host lock").left.spec.current_open());
        driver.pointer_press(point(px(8.0), px(8.0)));
        driver.pointer_release(point(px(8.0), px(8.0)));
        {
            let host = host.lock().expect("host lock");
            assert!(!host.left.spec.current_open(), "outside pointer closes");
            assert_eq!(host.left.values, ["banana"]);
        }

        driver.pointer_activate_id(&right_trigger);
        assert!(host.lock().expect("host lock").right.spec.current_open());
        driver.draw_frame();
        driver.wait_for_focus_handle(&right_search);
        driver.focus_element(&right_search);
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                host.right.spec.highlighted_value.as_deref(),
                Some("apple"),
                "open searchable starts on the first option"
            );
        }
        driver.dispatch_key_raw("end");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                host.right.spec.highlighted_value.as_deref(),
                Some("cherry"),
                "End highlights the last enabled option through a host rebuild"
            );
            assert!(host.right.spec.current_open());
        }
        driver.dispatch_key_raw("home");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                host.right.spec.highlighted_value.as_deref(),
                Some("apple"),
                "Home highlights the first enabled option through a host rebuild"
            );
        }
        driver.dispatch_key_raw("b");
        driver.dispatch_key_raw("a");
        driver.dispatch_key_raw("n");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.right.queries.last().map(String::as_str), Some("ban"));
            assert_eq!(
                (
                    host.right.spec.search_selection_start,
                    host.right.spec.search_selection_end
                ),
                (3, 3)
            );
            assert!(host.right.values.is_empty());
        }
        driver.dispatch_key_raw("shift-left");
        driver.dispatch_key_raw("shift-left");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                (
                    host.right.spec.search_selection_start,
                    host.right.spec.search_selection_end
                ),
                (3, 1),
                "backward selection keeps anchor at the original caret"
            );
        }
        driver.dispatch_key_raw("shift-left");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                (
                    host.right.spec.search_selection_start,
                    host.right.spec.search_selection_end
                ),
                (3, 0),
                "Shift+Arrow after rebuild moves the head, not a swapped anchor"
            );
        }
        driver.dispatch_key_raw("right");
        driver.dispatch_key_raw("left");
        driver.dispatch_key_raw("left");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                (
                    host.right.spec.search_selection_start,
                    host.right.spec.search_selection_end
                ),
                (1, 1)
            );
        }
        driver.dispatch_key_raw("x");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.right.queries.last().map(String::as_str), Some("bxan"));
            assert_eq!(host.right.spec.search_selection_range(), (2, 2));
        }
        driver.dispatch_key_raw("backspace");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.right.queries.last().map(String::as_str), Some("ban"));
        }
        let search_value = format!("{right_search}-value");
        driver.pointer_press(payload_frac(&search_value, 0.2, 0.5));
        driver.pointer_release(payload_frac(&search_value, 0.2, 0.5));
        {
            let host = host.lock().expect("host lock");
            assert_ne!(
                (
                    host.right.spec.search_selection_start,
                    host.right.spec.search_selection_end
                ),
                (3, 3),
                "pointer placement moves the search caret"
            );
        }
        driver.dispatch_key_raw("enter");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.right.values, ["banana"]);
            assert!(!host.right.spec.current_open());
            assert_eq!(host.left.values, ["banana"]);
        }

        let left_clear = "select:left:clear";
        driver.wait_for_focus_handle(&left_trigger);
        assert!(
            poodle_gpui_node_backend::bounds_for(left_clear).is_some()
                || mounted
                    .lock()
                    .expect("mount lock")
                    .find(&|n| n.runtime_id.as_deref() == Some(left_clear))
                    .is_some()
        );
        if poodle_gpui_node_backend::bounds_for(left_clear).is_some() {
            driver.pointer_activate_id(left_clear);
            assert_eq!(
                host.lock()
                    .expect("host lock")
                    .left
                    .values
                    .last()
                    .map(String::as_str),
                Some("")
            );
        }

        driver.keyboard_key(&right_trigger, "escape");
        assert!(!host.lock().expect("host lock").right.spec.current_open());

        driver.pointer_activate_id(&right_trigger);
        driver.wait_for_focus_handle(&right_search);
        driver.focus_element(&right_search);
        for _ in 0..16 {
            driver.dispatch_key_raw("backspace");
        }
        driver.dispatch_key_raw("z");
        driver.dispatch_key_raw("z");
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for(&right_search),
            Some(true),
            "search editor holds focus before control blur"
        );
        let before_blur = host.lock().expect("host lock").right.transitions;
        driver.dispatch_key_raw("tab");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                host.right.transitions,
                before_blur + 1,
                "control blur emits one transition result"
            );
            assert_eq!(host.right.values.last().map(String::as_str), Some("zz"));
            assert!(!host.right.spec.current_open());
            assert_eq!(host.right.spec.search_query.as_deref(), Some("zz"));
        }
    });
}

/// g16.019. A production Select listbox that exceeds `size.menu.maxHeight`
/// clips option rows past the cap. Short menus stay content-sized; this
/// case is the long-menu half of that contract.
#[test]
fn a_long_select_menu_clips_overflowing_option_rows() {
    use poodle_render::{select, select_option_id, select_trigger_focus_id, SelectHandlers};
    use poodle_specs::{ChoiceOption, SelectSpec};

    #[derive(Clone)]
    struct Host {
        spec: SelectSpec,
        values: Vec<String>,
    }

    fn options() -> Vec<ChoiceOption> {
        (0..20)
            .map(|index| ChoiceOption::new(format!("{index}"), format!("Option {index}")))
            .collect()
    }

    fn build(host: Arc<Mutex<Host>>, mounted: Arc<Mutex<Node>>) -> Node {
        let spec = host.lock().expect("host lock").spec.clone();
        let host_i = Arc::clone(&host);
        let mount_i = Arc::clone(&mounted);
        let handlers = SelectHandlers::new("long").on_transition(Arc::new(move |result| {
            let mut host = host_i.lock().expect("host lock");
            host.spec = host.spec.clone().applying_context(&result.context);
            for effect in result.effects {
                if let poodle_render::SelectEffect::ValueChanged { value } = effect {
                    host.values.push(value);
                }
            }
            drop(host);
            *mount_i.lock().expect("mount lock") = build(Arc::clone(&host_i), Arc::clone(&mount_i));
        }));
        select(&spec, &RenderContext::new(&theme()), &handlers)
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(Host {
            spec: SelectSpec::new(options()),
            values: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(Arc::clone(&host), Arc::clone(&mounted));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 280.0, 420.0);
        let trigger = select_trigger_focus_id("long");
        let first = select_option_id("long", "0");
        let last = select_option_id("long", "19");
        let listbox = "select:long:listbox";
        driver.wait_for_focus_handle(&trigger);
        driver.pointer_activate_id(&trigger);
        driver.draw_frame();
        {
            let tree = mounted.lock().expect("mount lock");
            let panel = tree
                .find(&|n| n.runtime_id.as_deref() == Some(listbox))
                .expect("listbox");
            assert_eq!(
                panel.style.descriptor.layout.overflow_y,
                LayoutOverflow::Scroll
            );
        }
        assert!(poodle_gpui_node_backend::bounds_for(&first).is_some());
        assert!(poodle_gpui_node_backend::bounds_for(&last).is_some());
        driver.pointer_activate_id(&last);
        assert!(
            host.lock().expect("host lock").values.is_empty(),
            "a row past max_height does not activate"
        );
        if !host.lock().expect("host lock").spec.current_open() {
            driver.pointer_activate_id(&trigger);
            driver.draw_frame();
        }
        // GPUI scroll offset is `[-max, 0]`. Negative pixel delta moves the
        // viewport down the list.
        driver.scroll_vertical_id(listbox, -800.0);
        driver.pointer_activate_id(&last);
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                host.values.last().map(String::as_str),
                Some("19"),
                "wheel scrolling must bring the last enabled row into hit-test"
            );
            assert!(!host.spec.current_open());
        }
    });
}

/// g16.025. Tree's complete authored native behaviour through real mounted
/// GPUI dispatch: row selection, twisty expand/collapse, a keyboard command,
/// and the drag route it now takes through the shared substrate — hovered
/// intent with the nested band, a committed reorder, and a cancelled one that
/// changes nothing. Every step drives real input and asserts the host spec the
/// component rebuilt from.
#[test]
fn tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec() {
    use poodle_render::TreeHandlers;
    use poodle_specs::{DropPosition as TreeDropPosition, TreeNode, TreeSpec};

    #[derive(Clone)]
    struct TreeHost {
        nodes: Vec<TreeNode>,
        selected: Vec<String>,
        expanded: Vec<String>,
        drag: Option<String>,
        drop_target: Option<String>,
        drop_position: TreeDropPosition,
        reorders: Vec<(String, String, poodle_node::DropEdge)>,
        keys: Vec<String>,
    }

    fn nodes() -> Vec<TreeNode> {
        vec![
            TreeNode::new("alpha", "Alpha"),
            TreeNode::new("bravo", "Bravo").with_children(vec![TreeNode::new("bravo-1", "Bravo 1")]),
            TreeNode::new("charlie", "Charlie"),
        ]
    }

    fn spec_of(host: &TreeHost) -> TreeSpec {
        TreeSpec::new(host.nodes.clone())
            .with_selected_values(host.selected.clone())
            .with_expanded_values(host.expanded.clone())
            .with_reorderable(true)
            .with_drag(
                host.drag.clone(),
                host.drop_target.clone(),
                host.drop_position,
            )
            .with_aria_label("Files")
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(TreeHost {
            nodes: nodes(),
            selected: Vec::new(),
            expanded: Vec::new(),
            drag: None,
            drop_target: None,
            drop_position: TreeDropPosition::After,
            reorders: Vec::new(),
            keys: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));

        fn build(host: &Arc<Mutex<TreeHost>>, mounted: &Arc<Mutex<Node>>) -> Node {
            let rebuild = {
                let host = Arc::clone(host);
                let mounted = Arc::clone(mounted);
                move || {
                    let next = build(&host, &mounted);
                    *mounted.lock().expect("mount lock") = next;
                }
            };
            let spec = spec_of(&host.lock().expect("host lock"));
            poodle_render::tree(
                &spec,
                &RenderContext::new(&theme()),
                TreeHandlers {
                    on_select: Some({
                        let host = Arc::clone(host);
                        let rebuild = rebuild.clone();
                        Arc::new(move |value: &str| {
                            host.lock().expect("host lock").selected = vec![value.to_owned()];
                            rebuild();
                        })
                    }),
                    on_toggle_expand: Some({
                        let host = Arc::clone(host);
                        let rebuild = rebuild.clone();
                        Arc::new(move |value: &str| {
                            {
                                let mut host = host.lock().expect("host lock");
                                if let Some(index) =
                                    host.expanded.iter().position(|open| open == value)
                                {
                                    host.expanded.remove(index);
                                } else {
                                    host.expanded.push(value.to_owned());
                                }
                            }
                            rebuild();
                        })
                    }),
                    on_key: Some({
                        let host = Arc::clone(host);
                        Arc::new(move |value: &str, key, _mods| {
                            host.lock()
                                .expect("host lock")
                                .keys
                                .push(format!("{value}:{key:?}"));
                        })
                    }),
                    on_drag_over: Some({
                        let host = Arc::clone(host);
                        let rebuild = rebuild.clone();
                        Arc::new(move |dragged: &str, over: &str, edge| {
                            {
                                let mut host = host.lock().expect("host lock");
                                host.drag = Some(dragged.to_owned());
                                host.drop_target = Some(over.to_owned());
                                host.drop_position = match edge {
                                    poodle_node::DropEdge::Before => TreeDropPosition::Before,
                                    poodle_node::DropEdge::Inside => TreeDropPosition::Inside,
                                    poodle_node::DropEdge::After => TreeDropPosition::After,
                                };
                            }
                            rebuild();
                        })
                    }),
                    on_reorder: Some({
                        let host = Arc::clone(host);
                        let rebuild = rebuild.clone();
                        Arc::new(move |dragged: &str, over: &str, edge| {
                            {
                                let mut host = host.lock().expect("host lock");
                                host.reorders
                                    .push((dragged.to_owned(), over.to_owned(), edge));
                                let from = host
                                    .nodes
                                    .iter()
                                    .position(|node| node.value == dragged);
                                let to = host.nodes.iter().position(|node| node.value == over);
                                if let (Some(from), Some(to)) = (from, to) {
                                    let moved = host.nodes.remove(from);
                                    let mut index = host
                                        .nodes
                                        .iter()
                                        .position(|node| node.value == over)
                                        .unwrap_or(to);
                                    if edge == poodle_node::DropEdge::After {
                                        index += 1;
                                    }
                                    host.nodes.insert(index, moved);
                                }
                                host.drag = None;
                                host.drop_target = None;
                            }
                            rebuild();
                        })
                    }),
                    ..TreeHandlers::default()
                },
            )
        }

        *mounted.lock().expect("mount lock") = build(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 260.0, 180.0);
        driver.draw_frame();

        // ── Selection through the real row listener ──
        driver.pointer_activate_id("tree:charlie");
        assert_eq!(host.lock().expect("host lock").selected, ["charlie"]);

        // ── Twisty expands, and the revealed child is a real row ──
        driver.pointer_activate_id("tree-twisty:bravo");
        assert_eq!(host.lock().expect("host lock").expanded, ["bravo"]);
        assert!(
            poodle_gpui_node_backend::bounds_for("tree:bravo-1").is_some(),
            "the expanded branch's child paints as its own row"
        );

        // ── A keyboard command reaches the host with the row it landed on ──
        //
        // Tree rows are sequential tab stops without a backend-owned focus
        // handle (they declare no focus ring or focus-change observer), so the
        // route in is GPUI's own traversal — the native counterpart of Tab.
        driver.focus_next_tab_stop();
        driver.dispatch_key_raw("down");
        {
            let keys = host.lock().expect("host lock").keys.clone();
            assert_eq!(keys.len(), 1, "one keystroke, one report: {keys:?}");
            let (value, key) = keys[0].split_once(':').expect("value:key");
            assert!(
                ["alpha", "bravo", "bravo-1", "charlie"].contains(&value),
                "the key is reported for the row that actually holds focus: {keys:?}"
            );
            assert_eq!(
                key, "ArrowDown",
                "a navigation key reaches the host as a key, not as a focus move the                  component invented"
            );
        }

        // ── Cancelled drag: hovered intent appears, then nothing commits ──
        //
        // Bravo is a branch. Its middle band nests. Charlie is a sibling leaf,
        // so the same fraction would land after it rather than inside.
        let alpha = payload_frac("tree:alpha", 0.5, 0.5);
        driver.pointer_press(alpha);
        driver.pointer_drag(point(px(f32::from(alpha.x) + 4.0), alpha.y));
        driver.pointer_drag(payload_frac("tree:bravo", 0.5, 0.5));
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.drag.as_deref(), Some("alpha"));
            assert_eq!(host.drop_target.as_deref(), Some("bravo"));
            assert_eq!(
                host.drop_position,
                TreeDropPosition::Inside,
                "a branch row's middle band is a nested placement, not a sibling one"
            );
        }
        driver.dispatch_key("escape");
        {
            let host = host.lock().expect("host lock");
            assert!(host.reorders.is_empty(), "Escape commits nothing");
            assert_eq!(
                host.nodes.iter().map(|node| node.value.as_str()).collect::<Vec<_>>(),
                ["alpha", "bravo", "charlie"]
            );
            // `TreeHandlers` has no clear or terminal channel, so the host's
            // indicator state stays latched after a cancelled drag. That is
            // Tree's pre-existing native gap, carried to the card that adds
            // the callback — not something this substrate can close without
            // changing Tree's public API.
            assert_eq!(
                host.drop_target.as_deref(),
                Some("bravo"),
                "the latched indicator is the documented Tree gap"
            );
        }

        // ── Committed drag: the bottom band is `after`, and the host reorders ──
        let alpha = payload_frac("tree:alpha", 0.5, 0.5);
        driver.pointer_press(alpha);
        driver.pointer_drag(point(px(f32::from(alpha.x) + 4.0), alpha.y));
        driver.pointer_drag(payload_frac("tree:charlie", 0.5, 0.9));
        driver.pointer_release(payload_frac("tree:charlie", 0.5, 0.9));

        let host = host.lock().expect("host lock");
        assert_eq!(
            host.reorders,
            [(
                "alpha".to_string(),
                "charlie".to_string(),
                poodle_node::DropEdge::After
            )],
            "exactly one reorder, carrying the resolved band"
        );
        assert_eq!(
            host.nodes.iter().map(|node| node.value.as_str()).collect::<Vec<_>>(),
            ["bravo", "charlie", "alpha"]
        );
        assert!(
            host.drag.is_none() && host.drop_target.is_none(),
            "a committed reorder clears the host's own drag state, because \
             `on_reorder` is the channel that tells it the move happened"
        );
    });
}

/// g16.025 review. Release is decided by the release *point*, not by whatever
/// the last move happened to leave.
///
/// A gesture can reach mouse-up with no intervening move — release outside the
/// window, or a coalesced move — and committing the stale hover would drop on
/// a target the pointer is no longer over.
#[test]
fn a_release_away_from_the_hovered_target_commits_nothing() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("custom-zone-a"),
            "the hover leaves a live intent"
        );

        // Straight to mouse-up, far outside, with no drag-move in between.
        driver.pointer_release(point(px(4.0), px(4.0)));

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:"),
            0,
            "a stale hover must not commit when the release lands elsewhere: {events:?}"
        );
        assert!(events.contains(&"cleared:custom-zone-a".to_owned()), "{events:?}");
        assert_eq!(
            count_starting_with(&events, "end:cancelled:"),
            1,
            "{events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.025 review. A release that lands on a *different* target commits that
/// one — the mirror of the case above, so the fix cannot be "never commit".
#[test]
fn a_release_over_another_target_commits_the_one_under_the_pointer() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        driver.pointer_release(payload_frac("custom-zone-b", 0.5, 0.25));

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:custom-zone-a"),
            0,
            "{events:?}"
        );
        assert_eq!(
            count_starting_with(&events, "drop:custom-zone-b:before"),
            1,
            "the release point resolves its own intent: {events:?}"
        );
        assert_eq!(count_starting_with(&events, "end:"), 1, "{events:?}");
    });
}

/// g16.025 review. One sensor owns an open gesture.
///
/// Escape is the deliberate exception — an accessible cancel that works on any
/// session. Everything else must be inert against a mouse-owned drag, or a
/// keystroke moves or commits a gesture the pointer is still holding.
#[test]
fn keys_other_than_escape_cannot_drive_a_mouse_owned_drag() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("custom-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("custom-zone-a", 0.5, 0.75));
        assert_eq!(
            controller.snapshot().input_kind,
            Some(NodeDragInputKind::Mouse)
        );

        for key in ["down", "up", "home", "end"] {
            driver.dispatch_key(key);
            assert_eq!(
                controller.snapshot().target_id.as_deref(),
                Some("custom-zone-a"),
                "`{key}` must not move a mouse-owned intent"
            );
        }
        driver.dispatch_key("enter");
        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:"),
            0,
            "Enter must not commit a mouse-owned drag: {events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // Escape still reaches it: cancellation is the one crossing sensor.
        driver.dispatch_key("escape");
        assert_eq!(
            count_starting_with(&trace_of(&trace), "end:cancelled:Escape"),
            1
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.025 review. Keyboard traversal starts from the source's declared
/// `keyboard_order`, not from the end of the registry.
///
/// A source between two targets must move Next to the one after it and
/// Previous to the one before it. Jumping to index 0 on the first Next was the
/// reverse of the contract for any source that does not sit at the start.
#[test]
fn keyboard_traversal_starts_from_the_sources_declared_origin() {
    fn origin_tree(trace: &Arc<Mutex<Vec<String>>>) -> Node {
        let mut before = drag_box("origin-before", 60.0, 40.0);
        before.interaction.drop_target = Some(traced_target(
            "origin-before",
            "Before",
            trace,
            false,
            1,
            NodeDropCommit::Committed,
        ));

        let mut source = drag_box("origin-source", 60.0, 40.0);
        source.interaction.focusable = true;
        source.a11y.tab_index = Some(0);
        let mut registration = traced_source("origin-source", "Alpha", trace);
        // Sits between the two targets rather than before both of them.
        registration.keyboard_order = Some(5);
        source.interaction.drag_source = Some(registration);

        let mut after = drag_box("origin-after", 60.0, 40.0);
        after.interaction.drop_target = Some(traced_target(
            "origin-after",
            "After",
            trace,
            false,
            9,
            NodeDropCommit::Committed,
        ));

        let mut row = Node::container();
        row.id = Some("origin-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(180.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        row.child(before).child(source).child(after)
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(origin_tree(&trace)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 220.0, 80.0);
        driver.wait_for_focus_handle("origin-source");
        driver.focus_element("origin-source");
        let controller = driver.drag();

        driver.dispatch_key_raw("space");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        driver.dispatch_key_raw("down");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("origin-after"),
            "the first Next lands on the nearest target past the origin"
        );
        driver.dispatch_key_raw("escape");

        driver.dispatch_key_raw("space");
        driver.dispatch_key_raw("up");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("origin-before"),
            "the first Previous lands on the nearest target before the origin"
        );
        driver.dispatch_key_raw("escape");
    });
}

/// g16.025 review. A keyboard pickup must not also activate the row it picked
/// up. GPUI synthesizes a click from Enter/Space on key-up for any focused
/// element with a click listener, so a handled drag key has to prevent that
/// default.
#[test]
fn a_keyboard_pickup_does_not_also_activate_its_own_source() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let activations = Arc::new(Mutex::new(0usize));
        let node = {
            let mut tree = custom_drag_tree(&trace, false);
            let activations = Arc::clone(&activations);
            let source = tree
                .children
                .iter_mut()
                .find(|child| child.id.as_deref() == Some("custom-source"))
                .expect("the source row");
            source.interaction.on_activate = Some(Arc::new(move || {
                *activations.lock().expect("activation lock") += 1;
            }));
            Arc::new(Mutex::new(tree))
        };
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.wait_for_focus_handle("custom-source");
        driver.focus_element("custom-source");
        let controller = driver.drag();

        driver.dispatch_key_raw("space");

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            *activations.lock().expect("activation lock"),
            0,
            "the pickup keystroke must not also fire the row's activation"
        );
        driver.dispatch_key_raw("escape");

        // The suppression is scoped to the handled drag key: an ordinary
        // Enter on the same focused row, with no session to pick up, still
        // activates it.
        driver.focus_element("custom-source");
        driver.dispatch_key_raw("enter");
        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Dragging,
            "Enter over an opted-in source is a pickup"
        );
        driver.dispatch_key_raw("escape");
        assert_eq!(
            *activations.lock().expect("activation lock"),
            0,
            "and it is still only a pickup"
        );
    });
}

/// g16.025 review. Reorder surfaces are scoped by *eligibility*, not only by
/// registration id. Two lists sharing one controller must not resolve each
/// other's targets, or a row from one mutates the other.
#[test]
fn two_reorder_surfaces_under_one_controller_cannot_cross_drop() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));

        fn surface(scope: &str, trace: &Arc<Mutex<Vec<String>>>) -> Node {
            let mut source = drag_box(&format!("{scope}-row"), 60.0, 40.0);
            let mut registration =
                poodle_render::reorder_source(scope, "row", "Row");
            let start = Arc::clone(trace);
            let scope_name = scope.to_string();
            registration.on_drag_start = Some(Arc::new(move |session: &DragSession| {
                push_trace(&start, format!("start:{scope_name}:{}", session.subject.id));
            }));
            source.interaction.drag_source = Some(registration);

            let mut target = poodle_render::reorder_target(scope, "row", "Row");
            let drop_trace = Arc::clone(trace);
            let scope_name = scope.to_string();
            target.on_drop = Some(Arc::new(move |event: &NodeDropCommitEvent| {
                push_trace(
                    &drop_trace,
                    format!("drop:{scope_name}:{}", event.subject.id),
                );
                NodeDropCommit::Committed
            }));
            source.interaction.drop_target = Some(target);
            source
        }

        let mut row = Node::container();
        row.id = Some("cross-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        let row = row
            .child(surface("list-a", &trace))
            .child(surface("list-b", &trace));

        let node = Arc::new(Mutex::new(row));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 200.0, 80.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("list-a-row", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(trace_of(&trace), ["start:list-a:row"]);

        // Over the OTHER list's row. Same value, same shape, different scope.
        // It is *refused*, not invisible: the surface can say so. What it must
        // never be is an accepted intent.
        driver.pointer_drag(payload_frac("list-b-row", 0.5, 0.75));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Rejected),
            "another reorder surface refuses this subject kind"
        );
        assert_eq!(
            snapshot.position, None,
            "a refusal resolves no placement"
        );
        driver.pointer_release(payload_frac("list-b-row", 0.5, 0.75));

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:"),
            0,
            "a cross-surface drop must never commit: {events:?}"
        );

        // Its own row is also refused — a row cannot be dropped onto itself.
        let source = payload_frac("list-a-row", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("list-a-row", 0.5, 0.75));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Rejected),
            "a self-drop is rejected, not silently accepted"
        );
        assert_eq!(
            snapshot.rejected_reason.as_deref(),
            Some("A row cannot be dropped onto itself"),
            "and the surface is told why"
        );
        driver.pointer_release(payload_frac("list-a-row", 0.5, 0.75));
        assert_eq!(count_starting_with(&trace_of(&trace), "drop:"), 0);
    });
}

/// g16.025 review. A rebuild that reuses one `source_id` for a different
/// subject has changed the source. Leaving the old subject dragging would let
/// it commit against a tree it no longer belongs to.
#[test]
fn a_source_that_changes_subject_during_a_rebuild_cancels_once() {
    fn subject_tree(trace: &Arc<Mutex<Vec<String>>>, subject_id: &str) -> Node {
        let mut source = drag_box("subject-source", 60.0, 40.0);
        let mut registration = traced_source("subject-source", "Alpha", trace);
        registration.subject = custom_subject(subject_id);
        source.interaction.drag_source = Some(registration);

        let mut zone = drag_box("subject-zone", 60.0, 40.0);
        zone.interaction.drop_target = Some(traced_target(
            "subject-zone",
            "Zone",
            trace,
            false,
            1,
            NodeDropCommit::Committed,
        ));

        let mut row = Node::container();
        row.id = Some("subject-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        row.child(source).child(zone)
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(subject_tree(&trace, "first")));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 200.0, 80.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("subject-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("subject-zone", 0.5, 0.75));
        assert_eq!(trace_of(&trace).first().map(String::as_str), Some("start:first"));

        // Same source id, different subject: a different row now lives here.
        *node.lock().expect("mount lock") = subject_tree(&trace, "second");
        driver.draw_frame();

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "end:cancelled:SourceLost"),
            1,
            "{events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        driver.pointer_release(payload_frac("subject-zone", 0.5, 0.75));
        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:"),
            0,
            "the superseded subject must not commit: {events:?}"
        );
        assert_eq!(count_starting_with(&events, "end:"), 1, "{events:?}");
    });
}

/// g16.025 review 2. Traversal stops at an absent boundary instead of wrapping.
///
/// A source past every target has nothing after it, so the first Next selects
/// nothing rather than jumping backwards to the end of the registry; a source
/// before every target has nothing before it. This is the web controller's
/// `firstTargetAfterSource` / `firstTargetBeforeSource` rule, which returns and
/// leaves the intent alone.
#[test]
fn keyboard_traversal_selects_nothing_past_an_absent_boundary() {
    fn bounded_tree(trace: &Arc<Mutex<Vec<String>>>, source_order: i32) -> Node {
        let mut target = drag_box("bounded-target", 60.0, 40.0);
        target.interaction.drop_target = Some(traced_target(
            "bounded-target",
            "Only",
            trace,
            false,
            5,
            NodeDropCommit::Committed,
        ));

        let mut source = drag_box("bounded-source", 60.0, 40.0);
        source.interaction.focusable = true;
        source.a11y.tab_index = Some(0);
        let mut registration = traced_source("bounded-source", "Alpha", trace);
        registration.keyboard_order = Some(source_order);
        source.interaction.drag_source = Some(registration);

        let mut row = Node::container();
        row.id = Some("bounded-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        row.child(target).child(source)
    }

    // Source after the only target: Next has nowhere to go.
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(bounded_tree(&trace, 9)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 200.0, 80.0);
        driver.wait_for_focus_handle("bounded-source");
        driver.focus_element("bounded-source");
        let controller = driver.drag();

        driver.dispatch_key_raw("space");
        driver.dispatch_key_raw("down");
        assert_eq!(
            controller.snapshot().target_id,
            None,
            "a source past every target must not wrap backwards on Next"
        );
        // Previous still finds the target that IS before it.
        driver.dispatch_key_raw("up");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("bounded-target")
        );
        driver.dispatch_key_raw("escape");
    });

    // Source before the only target: Previous has nowhere to go.
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(bounded_tree(&trace, 1)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 200.0, 80.0);
        driver.wait_for_focus_handle("bounded-source");
        driver.focus_element("bounded-source");
        let controller = driver.drag();

        driver.dispatch_key_raw("space");
        driver.dispatch_key_raw("up");
        assert_eq!(
            controller.snapshot().target_id,
            None,
            "a source before every target must not wrap forwards on Previous"
        );
        driver.dispatch_key_raw("down");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("bounded-target")
        );
        driver.dispatch_key_raw("escape");
    });
}

/// g16.025 review 2. The pickup key is also the drop key, so with no target
/// chosen it puts the row back down.
///
/// Reporting the key handled and leaving the drag open would strand a keyboard
/// user in a gesture their own pickup key could not close.
#[test]
fn the_pickup_key_cancels_a_session_that_never_chose_a_target() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(custom_drag_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.wait_for_focus_handle("custom-source");
        driver.focus_element("custom-source");
        let controller = driver.drag();

        driver.dispatch_key_raw("space");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            controller.snapshot().target_id, None,
            "a target this window does not have resolves to no intent at all"
        );

        // Straight back down, no traversal.
        driver.dispatch_key_raw("space");

        let events = trace_of(&trace);
        assert_eq!(count_starting_with(&events, "drop:"), 0, "{events:?}");
        assert_eq!(
            count_starting_with(&events, "end:cancelled:Explicit"),
            1,
            "the pickup key puts the row back down, exactly once: {events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
    });
}

/// g16.025 review 2. Activation suppression is tied to the key it suppresses.
///
/// One flag was consumable by *any* key-up: a modifier release, a neighbouring
/// shortcut, or an overlapping Enter would clear it, and the real Space release
/// would then synthesize the focused row's click after all.
#[test]
fn an_unrelated_key_release_cannot_re_enable_the_suppressed_activation() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let activations = Arc::new(Mutex::new(0usize));
        let node = {
            let mut tree = custom_drag_tree(&trace, false);
            let activations = Arc::clone(&activations);
            let source = tree
                .children
                .iter_mut()
                .find(|child| child.id.as_deref() == Some("custom-source"))
                .expect("the source row");
            source.interaction.on_activate = Some(Arc::new(move || {
                *activations.lock().expect("activation lock") += 1;
            }));
            Arc::new(Mutex::new(tree))
        };
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.wait_for_focus_handle("custom-source");
        driver.focus_element("custom-source");
        let controller = driver.drag();

        // Press Space and hold it: the pickup is armed against `space`.
        driver.dispatch_key_press("space");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // Another key arrives and leaves while Space is still held.
        driver.dispatch_key_press("a");
        driver.dispatch_key_release("a");
        // And an overlapping Enter press/release, which arms and consumes its
        // own entry without touching Space's.
        driver.dispatch_key_press("enter");
        driver.dispatch_key_release("enter");

        // Now the real release.
        driver.dispatch_key_release("space");

        assert_eq!(
            *activations.lock().expect("activation lock"),
            0,
            "an unrelated key-up must not re-enable the row's activation"
        );
    });
}

/// g16.025 review 3. The public snapshot distinguishes a refused target from
/// no target, and carries the refusal's reason.
///
/// Spec 069 requires accepted/rejected target posture on `DragDropSnapshot`.
/// Without it a custom surface cannot paint a rejected target at all: the
/// kernel discards refused candidates, so hovering a target that says no looks
/// exactly like hovering empty space. This drives the full cycle —
/// accepted → rejected → empty → terminal — through real mounted input.
#[test]
fn the_snapshot_reports_accepted_rejected_and_empty_target_posture() {
    use poodle_gpui_node_backend::DragDropTargetPosture;

    fn posture_tree(trace: &Arc<Mutex<Vec<String>>>) -> Node {
        let mut source = drag_box("posture-source", 60.0, 60.0);
        source.interaction.drag_source = Some(traced_source("posture-source", "Alpha", trace));

        let mut open = drag_box("posture-open", 60.0, 60.0);
        open.interaction.drop_target = Some(traced_target(
            "posture-open",
            "Open",
            trace,
            false,
            1,
            NodeDropCommit::Committed,
        ));

        let mut locked = drag_box("posture-locked", 60.0, 60.0);
        let mut locked_target = traced_target(
            "posture-locked",
            "Locked",
            trace,
            false,
            2,
            NodeDropCommit::Committed,
        );
        locked_target.can_drop = Some(Arc::new(|_intent, _subject| DropEligibility::Rejected {
            reason: Some("Locked by another editor".to_string()),
        }));
        locked.interaction.drop_target = Some(locked_target);

        let mut row = Node::container();
        row.id = Some("posture-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(180.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(60.0);
        row.child(source).child(open).child(locked)
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(posture_tree(&trace)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 240.0, 100.0);
        driver.draw_frame();
        let controller = driver.drag();

        let source = payload_frac("posture-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));

        // ── Accepted ──
        driver.pointer_drag(payload_frac("posture-open", 0.5, 0.75));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_posture,
            Some(DragDropTargetPosture::Accepted)
        );
        assert_eq!(snapshot.target_id.as_deref(), Some("posture-open"));
        assert_eq!(snapshot.position.as_deref(), Some("after"));
        assert_eq!(
            snapshot.rejected_reason, None,
            "an accepted intent leaves no refusal beside it"
        );

        // ── Rejected: named, with its reason, and no placement ──
        driver.pointer_drag(payload_frac("posture-locked", 0.5, 0.75));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_posture,
            Some(DragDropTargetPosture::Rejected)
        );
        assert_eq!(
            snapshot.target_id.as_deref(),
            Some("posture-locked"),
            "a refused target is still the target the surface is over"
        );
        assert_eq!(
            snapshot.rejected_reason.as_deref(),
            Some("Locked by another editor")
        );
        assert_eq!(
            snapshot.position, None,
            "a refusal has no placement to draw"
        );
        assert!(
            trace_of(&trace)
                .iter()
                .all(|event| !event.starts_with("intent:posture-locked")),
            "a refusal never becomes an intent"
        );

        // ── Empty: over nothing at all ──
        driver.pointer_drag(point(px(4.0), px(4.0)));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_posture, None,
            "no target is not the same as a refusing target"
        );
        assert_eq!(snapshot.target_id, None);
        assert_eq!(snapshot.rejected_reason, None);

        // ── Refuse again, then end on it: the terminal clears the posture ──
        driver.pointer_drag(payload_frac("posture-locked", 0.5, 0.75));
        assert_eq!(
            controller.snapshot().target_posture,
            Some(DragDropTargetPosture::Rejected)
        );
        driver.pointer_release(payload_frac("posture-locked", 0.5, 0.75));

        let snapshot = controller.snapshot();
        assert_eq!(snapshot.phase, DragSessionPhase::Idle);
        assert_eq!(
            snapshot.target_posture, None,
            "a finished session refuses nothing"
        );
        assert_eq!(snapshot.rejected_reason, None);
        assert_eq!(snapshot.target_id, None);

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:"),
            0,
            "releasing over a refusing target commits nothing: {events:?}"
        );
        assert_eq!(
            count_starting_with(&events, "end:cancelled:"),
            1,
            "{events:?}"
        );
    });
}

// ── External files (g16.027) ──────────────────────────────────────────────
//
// Both directions of the external-file boundary, mounted. The stubs record
// every command the controller sends, because the load-bearing claims are
// again about what it did *not* do: arm an unvalidated receipt, delete an
// artifact, ask a settled receipt to cancel, or let an unchecked batch reach
// a consumer's own resolver.

#[derive(Default)]
struct ExportLog {
    prepares: Vec<String>,
    starts: Vec<String>,
    stops: Vec<String>,
    cancels: Vec<(String, poodle_node::DragCancelReason)>,
    aborts: Vec<poodle_node::DragCancelReason>,
}

type PendingExport = (
    poodle_node::CrossWindowAbort,
    poodle_node::DragExportPrepareComplete,
);

#[derive(Default)]
struct ExportStubState {
    log: ExportLog,
    pending: Vec<PendingExport>,
    terminal: Option<poodle_node::DragExportTerminalCallback>,
    /// The temporary files this host wrote. Nothing in Poodle may remove one.
    artifacts: Vec<String>,
}

#[derive(Clone, Default)]
struct ExportStub {
    state: Arc<Mutex<ExportStubState>>,
    multiple_files: bool,
}

/// Drive one export to a chosen terminal and read back what a screen reader
/// would have been told. Each of these reaches the *same* kernel terminal — a
/// cancellation — and none of them means the same thing to the person doing it.
enum ExportEnding {
    Declined,
    Ended,
    Cancelled,
    Failed,
}

impl ExportStub {
    fn log<T>(&self, read: impl FnOnce(&ExportLog) -> T) -> T {
        read(&self.state.lock().expect("export state").log)
    }

    fn artifacts(&self) -> Vec<String> {
        self.state.lock().expect("export state").artifacts.clone()
    }

    /// Answer the n-th outstanding preparation with a receipt of `count` files.
    fn settle(&self, index: usize, receipt: Option<(&str, u32)>) {
        let entry = {
            let mut state = self.state.lock().expect("export state");
            if index >= state.pending.len() {
                return;
            }
            state.pending.remove(index)
        };
        let (abort, complete) = entry;
        if let Some(reason) = abort.reason() {
            self.state
                .lock()
                .expect("export state")
                .log
                .aborts
                .push(reason);
        }
        let prepared = receipt.map(|(id, count)| {
            // The host writes its temporary file here, exactly as a shell
            // would, and keeps it until it decides otherwise.
            self.state
                .lock()
                .expect("export state")
                .artifacts
                .push(id.to_string());
            poodle_node::PreparedFileExport {
                receipt_id: id.to_string(),
                display_name: Some("take-01.wav".to_string()),
                form: poodle_node::DragExportForm::MaterializedFile,
                file_count: Some(count),
                data_types: Vec::new(),
            }
        });
        complete(prepared);
    }

    fn report(&self, terminal: poodle_node::DragExportTerminal) {
        let state = self.state.lock().expect("export state");
        if let Some(callback) = state.terminal.as_ref() {
            callback(terminal);
        }
    }
}

impl poodle_node::DragExportBridge for ExportStub {
    fn capabilities(&self) -> poodle_node::DragExportCapabilities {
        poodle_node::DragExportCapabilities {
            files: true,
            multiple_files: self.multiple_files,
            promised_files: false,
            custom_data_types: Vec::new(),
        }
    }

    fn prepare(
        &self,
        request: poodle_node::DragExportPrepareRequest,
        abort: poodle_node::CrossWindowAbort,
        complete: poodle_node::DragExportPrepareComplete,
    ) {
        let mut state = self.state.lock().expect("export state");
        state.log.prepares.push(request.session_id);
        state.pending.push((abort, complete));
    }

    fn start(
        &self,
        prepared: poodle_node::PreparedFileExport,
        on_terminal: poodle_node::DragExportTerminalCallback,
    ) -> poodle_node::CrossWindowCleanup {
        let receipt = prepared.receipt_id.clone();
        {
            let mut state = self.state.lock().expect("export state");
            state.log.starts.push(receipt.clone());
            state.terminal = Some(on_terminal);
        }
        let stub = self.clone();
        Box::new(move || {
            let mut state = stub.state.lock().expect("export state");
            state.terminal = None;
            state.log.stops.push(receipt);
        })
    }

    fn cancel(
        &self,
        prepared: poodle_node::PreparedFileExport,
        reason: poodle_node::DragCancelReason,
    ) {
        // Not a delete order: the artifact stays, so a test can prove Poodle
        // never asked for its removal.
        self.state
            .lock()
            .expect("export state")
            .log
            .cancels
            .push((prepared.receipt_id, reason));
    }
}

fn attach_export_bridge(
    node: &mut Node,
    source_id: &str,
    bridge: Arc<dyn poodle_node::DragExportBridge>,
) {
    if let Some(source) = node.interaction.drag_source.as_mut() {
        if source.source_id == source_id {
            source.file_export_bridge = Some(bridge);
            return;
        }
    }
    for child in node.children.iter_mut() {
        attach_export_bridge(child, source_id, Arc::clone(&bridge));
    }
}

#[derive(Default)]
struct InboundStubState {
    released: Vec<(String, poodle_node::InboundFileOutcome)>,
    listener: Option<Box<dyn Fn(poodle_node::InboundFileEvent) + Send>>,
}

#[derive(Clone, Default)]
struct InboundStub {
    state: Arc<Mutex<InboundStubState>>,
}

impl InboundStub {
    fn released(&self) -> Vec<(String, poodle_node::InboundFileOutcome)> {
        self.state.lock().expect("inbound state").released.clone()
    }

    fn send(&self, event: poodle_node::InboundFileEvent) {
        let state = self.state.lock().expect("inbound state");
        if let Some(listener) = state.listener.as_ref() {
            listener(event);
        }
    }
}

impl poodle_node::InboundFileHostBridge for InboundStub {
    fn capabilities(&self) -> poodle_node::InboundFileCapabilities {
        poodle_node::InboundFileCapabilities {
            files: true,
            multiple_files: true,
            transport: poodle_node::InboundFileTransport::Host,
            custom_data_types: Vec::new(),
        }
    }

    fn subscribe(
        &self,
        listener: Box<dyn Fn(poodle_node::InboundFileEvent) + Send>,
    ) -> poodle_node::CrossWindowCleanup {
        self.state.lock().expect("inbound state").listener = Some(listener);
        let stub = self.clone();
        Box::new(move || {
            stub.state.lock().expect("inbound state").listener = None;
        })
    }

    fn release(&self, batch_id: &str, outcome: poodle_node::InboundFileOutcome) {
        self.state
            .lock()
            .expect("inbound state")
            .released
            .push((batch_id.to_string(), outcome));
    }
}

fn inbound_batch(files: Vec<poodle_node::InboundFileReceipt>) -> poodle_node::InboundFileBatch {
    poodle_node::InboundFileBatch {
        protocol_version: poodle_node::INBOUND_FILE_PROTOCOL_VERSION,
        batch_id: "batch-1".to_string(),
        transport: poodle_node::InboundFileTransport::Host,
        files,
    }
}

fn inbound_file(
    id: &str,
    name: Option<&str>,
    media_type: &str,
    size: Option<u64>,
) -> poodle_node::InboundFileReceipt {
    poodle_node::InboundFileReceipt {
        receipt_id: id.to_string(),
        name: name.map(|value| value.to_string()),
        media_type: media_type.to_string(),
        size,
    }
}

/// A surface with one file drop zone, built from the shared renderer-neutral
/// construction rather than a hand-written registration.
fn inbound_tree(
    trace: &Arc<Mutex<Vec<String>>>,
    constraints: poodle_node::InboundFileConstraints,
) -> Node {
    let mut zone = drag_box("files-zone", 60.0, 40.0);
    let mut target = poodle_render::inbound_file_target("files-zone", "Library", constraints);
    let drop_trace = Arc::clone(trace);
    target.on_drop = Some(Arc::new(move |event: &NodeDropCommitEvent| {
        let names = event
            .inbound_files
            .as_ref()
            .map(|batch| {
                batch
                    .files
                    .iter()
                    .map(|file| file.name.clone().unwrap_or_default())
                    .collect::<Vec<String>>()
                    .join(",")
            })
            .unwrap_or_default();
        push_trace(&drop_trace, format!("drop:files-zone:{names}"));
        NodeDropCommit::Committed
    }));
    let eligibility_trace = Arc::clone(trace);
    target.can_drop = Some(Arc::new(
        move |intent: &poodle_node::DropIntent, _subject: &DragSubject| {
            // Recorded so a test can prove validation ran *before* this.
            push_trace(&eligibility_trace, "can_drop:files-zone".to_string());
            DropEligibility::Accepted {
                intent: intent.clone(),
            }
        },
    ));
    zone.interaction.drop_target = Some(target);

    let mut row = Node::container();
    row.id = Some("files-row".to_string());
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
    row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
    row.child(zone)
}

/// g16.027. The export half, end to end: preparation runs before activation,
/// nothing starts until the receipt arms, the host's terminal is the only way
/// out, and an ending does not authorize deleting what the host made.
#[test]
fn a_gpui_export_prepares_before_activation_and_ends_without_authorizing_deletion() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = ExportStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("fx", &trace);
        attach_export_bridge(&mut node, "fx-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("fx-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));

        assert_eq!(host.log(|log| log.prepares.len()), 1);
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Preparing);
        assert!(
            host.log(|log| log.starts.is_empty()),
            "an unarmed receipt cannot start a native drag"
        );
        assert_eq!(
            controller.snapshot().file_export.map(|export| export.state),
            Some(poodle_node::DragExportState::Preparing)
        );

        host.settle(0, Some(("export-1", 1)));
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(host.log(|log| log.starts.clone()), vec!["export-1".to_string()]);
        let armed = controller.snapshot().file_export.expect("export");
        assert_eq!(armed.state, poodle_node::DragExportState::Dragging);
        assert_eq!(armed.display_name.as_deref(), Some("take-01.wav"));

        host.report(poodle_node::DragExportTerminal::Ended);
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(
            controller.snapshot().file_export.map(|export| export.state),
            Some(poodle_node::DragExportState::Ended),
            "the ending is visible after the session is gone"
        );
        assert!(
            host.log(|log| log.cancels.is_empty()),
            "a settled receipt is never cancelled again"
        );
        assert_eq!(host.log(|log| log.stops.clone()), vec!["export-1".to_string()]);
        assert_eq!(
            host.artifacts(),
            vec!["export-1".to_string()],
            "the host still owns what it made"
        );

        // A repeat is inert.
        host.report(poodle_node::DragExportTerminal::Ended);
        driver.draw_frame();
        assert_eq!(
            count_starting_with(&trace_of(&trace), "end:"),
            1,
            "one terminal per session"
        );
    });
}

/// g16.027. A receipt beyond the adapter's own advertised capabilities is
/// refused *and handed back*, so the artifact it made for a drag that will
/// never start is not silently abandoned.
#[test]
fn a_gpui_export_receipt_beyond_its_capabilities_is_refused_and_returned() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = ExportStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("fx", &trace);
        attach_export_bridge(&mut node, "fx-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("fx-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));

        // Three files from an adapter that advertised one.
        host.settle(0, Some(("export-1", 3)));
        driver.draw_frame();

        assert!(
            host.log(|log| log.starts.is_empty()),
            "an unvalidated receipt never arms a native drag"
        );
        assert_eq!(
            host.log(|log| log.cancels.clone()),
            vec![(
                "export-1".to_string(),
                poodle_node::DragCancelReason::PreparationFailed
            )],
            "the receipt goes back to the host that made it"
        );
        assert_eq!(
            controller.snapshot().file_export.map(|export| export.state),
            Some(poodle_node::DragExportState::Failed)
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(
            host.artifacts(),
            vec!["export-1".to_string()],
            "refusing is not deleting"
        );
    });
}

/// g16.027. A superseded export is aborted, and its late receipt is returned
/// rather than arming the session that replaced it.
#[test]
fn a_superseded_gpui_export_receipt_cannot_arm_its_successor() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = ExportStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("fx", &trace);
        attach_export_bridge(&mut node, "fx-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("fx-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(host.log(|log| log.prepares.len()), 1);

        driver.dispatch_key_raw("escape");
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(host.log(|log| log.prepares.len()), 2);

        // The first host answers late, for a session that is gone.
        host.settle(0, Some(("stale-export", 1)));
        driver.draw_frame();

        assert!(
            host.log(|log| log.aborts.contains(&poodle_node::DragCancelReason::Escape)),
            "the abandoned preparation was told to stop: {:?}",
            host.log(|log| log.aborts.clone())
        );
        assert_eq!(
            host.log(|log| log.cancels.clone()),
            vec![(
                "stale-export".to_string(),
                poodle_node::DragCancelReason::Superseded
            )]
        );
        assert!(
            host.log(|log| log.starts.is_empty()),
            "a stale receipt cannot arm the live session"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Preparing);
    });
}

/// g16.027. Inbound files reach the ordinary target path, commit through the
/// ordinary handler with their receipts, and release exactly once.
#[test]
fn an_inbound_gpui_batch_commits_through_the_common_target_path_and_releases_once() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints {
                accept: Some("audio/*".to_string()),
                ..Default::default()
            },
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        let (x, y) = (f32::from(zone.x), f32::from(zone.y));

        // Hover: the platform has disclosed a declared type and nothing else.
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![inbound_file("batch-1:0", None, "audio/wav", None)]),
            x,
            y,
        });
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            controller.snapshot().target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Accepted)
        );
        assert_eq!(
            controller
                .snapshot()
                .inbound_files
                .map(|batch| batch.files.len()),
            Some(1)
        );

        // Drop: names and sizes finally exist, and the same target commits.
        host.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![inbound_file(
                "batch-1:0",
                Some("take-01.wav"),
                "audio/wav",
                Some(2_048),
            )]),
            x,
            y,
        });
        driver.draw_frame();

        let events = trace_of(&trace);
        assert!(
            events.contains(&"drop:files-zone:take-01.wav".to_string()),
            "the commit handler received the disclosed receipts: {events:?}"
        );
        assert_eq!(
            host.released(),
            vec![("batch-1".to_string(), poodle_node::InboundFileOutcome::Committed)],
            "one release, with the outcome the session actually reached"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(controller.snapshot().inbound_files.is_none());
    });
}

/// g16.027. Untrusted external metadata is refused by the boundary, before
/// the consumer's own eligibility resolver is asked anything.
#[test]
fn an_inbound_gpui_batch_is_validated_before_the_targets_own_resolver() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints {
                max_files: Some(1),
                ..Default::default()
            },
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![
                inbound_file("batch-1:0", Some("a.wav"), "audio/wav", Some(1)),
                inbound_file("batch-1:1", Some("b.wav"), "audio/wav", Some(1)),
            ]),
            x: f32::from(zone.x),
            y: f32::from(zone.y),
        });
        driver.draw_frame();

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "can_drop:"),
            0,
            "the target's resolver never saw an over-limit batch: {events:?}"
        );
        assert_eq!(
            controller.snapshot().target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Rejected)
        );

        host.send(poodle_node::InboundFileEvent::Cancelled {
            batch_id: "batch-1".to_string(),
        });
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(
            host.released(),
            vec![("batch-1".to_string(), poodle_node::InboundFileOutcome::Cancelled)]
        );
    });
}

/// g16.027. Hover acceptance is provisional: the platform hides sizes until
/// the drop, and a file that is only too large once disclosed is refused
/// there rather than committed on the strength of the hover.
#[test]
fn a_disclosed_gpui_drop_is_validated_again_before_it_can_commit() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints {
                max_size: Some(1_000),
                ..Default::default()
            },
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        let (x, y) = (f32::from(zone.x), f32::from(zone.y));

        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![inbound_file("batch-1:0", None, "audio/wav", None)]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            controller.snapshot().target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Accepted),
            "an undisclosed size cannot refuse yet"
        );

        host.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![inbound_file(
                "batch-1:0",
                Some("take-01.wav"),
                "audio/wav",
                Some(9_999),
            )]),
            x,
            y,
        });
        driver.draw_frame();

        let events = trace_of(&trace);
        assert_eq!(
            count_starting_with(&events, "drop:"),
            0,
            "hover acceptance did not carry a file the target now refuses: {events:?}"
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(
            host.released(),
            vec![("batch-1".to_string(), poodle_node::InboundFileOutcome::Cancelled)]
        );
    });
}

/// g16.027. Every export terminal reaches the same kernel cancellation and
/// none of them means the same thing. The native runtime must say which,
/// exactly as the web controller does — otherwise a screen reader is told
/// "cancelled" for a file the user successfully dragged onto their desktop.
#[test]
fn gpui_export_terminals_are_announced_in_their_own_words() {
    // One app, one window, one terminal per call. Sharing a `TestAppContext`
    // across four drivers means every window redraws on every other window's
    // frame, and the controllers' end-of-frame refresh keeps them all dirty.
    fn announcement_for(ending: ExportEnding) -> String {
        let announcement = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&announcement);
        run_headless(move |cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = ExportStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();

        let mut node = scoped_drag_tree("fx", &trace);
        attach_export_bridge(&mut node, "fx-source", Arc::new(host.clone()));
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("fx-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));

        match ending {
            ExportEnding::Declined => {
                host.settle(0, None);
                driver.draw_frame();
            }
            ExportEnding::Ended => {
                host.settle(0, Some(("export-1", 1)));
                driver.draw_frame();
                host.report(poodle_node::DragExportTerminal::Ended);
                driver.draw_frame();
            }
            ExportEnding::Cancelled => {
                host.settle(0, Some(("export-1", 1)));
                driver.draw_frame();
                host.report(poodle_node::DragExportTerminal::Cancelled {
                    reason: poodle_node::DragCancelReason::WindowLost,
                });
                driver.draw_frame();
            }
            ExportEnding::Failed => {
                host.settle(0, Some(("export-1", 1)));
                driver.draw_frame();
                host.report(poodle_node::DragExportTerminal::Failed {
                    reason: Some("disk full".to_string()),
                });
                driver.draw_frame();
            }
        }

        *captured.lock().expect("announcement") = controller
            .snapshot()
            .announcement
            .expect("an announcement for every terminal");
        });
        let text = announcement.lock().expect("announcement").clone();
        text
    }

    assert_eq!(announcement_for(ExportEnding::Ended), "Finished exporting Alpha.");
    assert_eq!(
        announcement_for(ExportEnding::Cancelled),
        "Cancelled exporting Alpha."
    );
    assert_eq!(
        announcement_for(ExportEnding::Failed),
        "Export failed for Alpha. disk full"
    );
    assert_eq!(
        announcement_for(ExportEnding::Declined),
        "Alpha cannot be exported."
    );
}

/// g16.027. Exactly one release per observed batch, on the native runtime
/// too: a repeat of the live id is one observation, a second batch is
/// answered rather than ignored, and news for a batch this window refused can
/// neither start nor end anything.
#[test]
fn a_gpui_window_owns_one_batch_and_answers_every_other_one_once() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints::default(),
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        let (x, y) = (f32::from(zone.x), f32::from(zone.y));
        let file = || inbound_file("batch-1:0", Some("take-01.wav"), "audio/wav", Some(16));

        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // The same batch again is one observation, not two.
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert!(host.released().is_empty(), "{:?}", host.released());

        // A second, different batch is answered — and the live one continues.
        let second = poodle_node::InboundFileBatch {
            batch_id: "batch-2".to_string(),
            ..inbound_batch(vec![file()])
        };
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: second.clone(),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            host.released(),
            vec![(
                "batch-2".to_string(),
                poodle_node::InboundFileOutcome::Rejected
            )]
        );
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(
            controller
                .snapshot()
                .inbound_files
                .map(|batch| batch.batch_id),
            Some("batch-1".to_string())
        );

        // News for the refused batch can neither commit nor cancel.
        host.send(poodle_node::InboundFileEvent::Dropped {
            batch: second,
            x,
            y,
        });
        host.send(poodle_node::InboundFileEvent::Cancelled {
            batch_id: "batch-2".to_string(),
        });
        driver.draw_frame();
        assert_eq!(host.released().len(), 1);
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        assert_eq!(count_starting_with(&trace_of(&trace), "drop:"), 0);

        host.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            host.released(),
            vec![
                (
                    "batch-2".to_string(),
                    poodle_node::InboundFileOutcome::Rejected
                ),
                (
                    "batch-1".to_string(),
                    poodle_node::InboundFileOutcome::Committed
                )
            ]
        );

        // A late repeat of the finished id cannot resurrect it or release
        // twice — including a replayed `Entered`, which is the shape that
        // would otherwise open a second session over one batch.
        host.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        host.send(poodle_node::InboundFileEvent::Cancelled {
            batch_id: "batch-1".to_string(),
        });
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: poodle_node::InboundFileBatch {
                batch_id: "batch-2".to_string(),
                ..inbound_batch(vec![file()])
            },
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(host.released().len(), 2);
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(controller.snapshot().inbound_files.is_none());
    });
}

/// g16.027. A local gesture owns this controller — and a refusal is still an
/// answer, so the host is told rather than left holding material for a
/// gesture nobody will finish.
#[test]
fn a_gpui_batch_arriving_mid_gesture_is_refused_and_the_host_is_told() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(host.clone()), cx));

        // A surface with both a local source and a file zone.
        let mut node = scoped_drag_tree("fx", &trace);
        let zone_tree = inbound_tree(&trace, poodle_node::InboundFileConstraints::default());
        for child in zone_tree.children.into_iter() {
            node = node.child(child);
        }
        let node = Arc::new(Mutex::new(node));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let source = payload_frac("fx-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        let local_subject = controller.snapshot().subject;

        host.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![inbound_file(
                "batch-1:0",
                Some("take-01.wav"),
                "audio/wav",
                Some(16),
            )]),
            x: f32::from(source.x),
            y: f32::from(source.y),
        });
        driver.draw_frame();

        assert_eq!(
            controller.snapshot().subject,
            local_subject,
            "the user's own gesture keeps the session"
        );
        assert!(controller.snapshot().inbound_files.is_none());
        assert_eq!(
            host.released(),
            vec![(
                "batch-1".to_string(),
                poodle_node::InboundFileOutcome::Rejected
            )],
            "the refused batch is still answered"
        );
    });
}

/// g16.027. Replacing the bridge ends the outgoing batch's session, and A's
/// queued news is answered through A rather than through B — a receipt
/// belongs to the host that observed it.
#[test]
fn replacing_the_gpui_inbound_bridge_ends_the_session_and_answers_the_old_host() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let first = InboundStub::default();
        let second = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(first.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints::default(),
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        let (x, y) = (f32::from(zone.x), f32::from(zone.y));
        let file = |id: &str| inbound_file(id, Some("take-01.wav"), "audio/wav", Some(16));

        first.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file("batch-1:0")]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // A queues more news that will not be drained until after B is in.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: poodle_node::InboundFileBatch {
                batch_id: "batch-stale".to_string(),
                ..inbound_batch(vec![file("batch-stale:0")])
            },
            x,
            y,
        });

        driver.update_app(|cx| {
            controller.set_inbound_file_bridge(Arc::new(second.clone()), cx)
        });

        // Checked *before* the next frame: the replacement ends the session
        // itself. Leaving a live session behind for the end-of-frame sweep to
        // notice would close it as a lost source, one frame later, and only
        // because this runtime happens to have a sweep at all.
        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Idle,
            "replacing the bridge ends the batch's session rather than stranding it"
        );

        driver.drain();
        assert_eq!(
            first.released(),
            vec![
                (
                    "batch-1".to_string(),
                    poodle_node::InboundFileOutcome::Cancelled
                ),
                (
                    "batch-stale".to_string(),
                    poodle_node::InboundFileOutcome::Rejected
                )
            ],
            "A's own batches are answered through A"
        );
        assert!(
            second.released().is_empty(),
            "B never issued anything and is never told about A's material: {:?}",
            second.released()
        );

        // B is live and owns the window from here.
        second.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file("batch-1:0")]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
    });
}

/// g16.027. A batch from a protocol this build does not speak is refused
/// before the consumer's own resolver is asked anything.
#[test]
fn a_gpui_batch_from_another_protocol_version_is_refused_before_eligibility() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints::default(),
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        host.send(poodle_node::InboundFileEvent::Entered {
            batch: poodle_node::InboundFileBatch {
                protocol_version: poodle_node::INBOUND_FILE_PROTOCOL_VERSION + 1,
                ..inbound_batch(vec![inbound_file(
                    "batch-1:0",
                    Some("take-01.wav"),
                    "audio/wav",
                    Some(16),
                )])
            },
            x: f32::from(zone.x),
            y: f32::from(zone.y),
        });
        driver.draw_frame();

        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(count_starting_with(&trace_of(&trace), "can_drop:"), 0);
        assert_eq!(
            host.released(),
            vec![(
                "batch-1".to_string(),
                poodle_node::InboundFileOutcome::Rejected
            )]
        );
    });
}

/// g16.027. A release ends an id, not one observation of it. A host that
/// publishes `Entered` again for a batch that already finished is repeating
/// itself, and taking it again would open a second session over one batch and
/// release it twice — while a *replacement* host may legitimately use the
/// same opaque text, because an id is one host's own name for something.
#[test]
fn a_finished_gpui_batch_id_stays_inert_until_its_bridge_is_replaced() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let first = InboundStub::default();
        let second = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(first.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints::default(),
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        let (x, y) = (f32::from(zone.x), f32::from(zone.y));
        let file = || inbound_file("batch-1:0", Some("take-01.wav"), "audio/wav", Some(16));

        // One batch commits.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        first.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            first.released(),
            vec![(
                "batch-1".to_string(),
                poodle_node::InboundFileOutcome::Committed
            )]
        );
        assert_eq!(count_starting_with(&trace_of(&trace), "drop:"), 1);

        // The host publishes the same id again, from idle, exactly as it
        // would a new drag. It is not one.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(controller.snapshot().inbound_files.is_none());

        first.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(first.released().len(), 1, "one id, one release");
        assert_eq!(count_starting_with(&trace_of(&trace), "drop:"), 1);

        // A refused id is just as final: this one is refused for arriving
        // while another batch owns the controller.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: poodle_node::InboundFileBatch {
                batch_id: "batch-9".to_string(),
                ..inbound_batch(vec![file()])
            },
            x,
            y,
        });
        driver.draw_frame();
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: poodle_node::InboundFileBatch {
                batch_id: "batch-refused".to_string(),
                ..inbound_batch(vec![file()])
            },
            x,
            y,
        });
        first.send(poodle_node::InboundFileEvent::Cancelled {
            batch_id: "batch-9".to_string(),
        });
        driver.draw_frame();
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: poodle_node::InboundFileBatch {
                batch_id: "batch-refused".to_string(),
                ..inbound_batch(vec![file()])
            },
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(
            first
                .released()
                .iter()
                .filter(|(id, _)| id == "batch-refused")
                .count(),
            1,
            "a refused id is answered once: {:?}",
            first.released()
        );

        // A replacement host is a different relationship. The same text may
        // name a batch this window has never seen.
        driver.update_app(|cx| {
            controller.set_inbound_file_bridge(Arc::new(second.clone()), cx)
        });
        driver.drain();

        second.send(poodle_node::InboundFileEvent::Entered {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Dragging,
            "a replacement host may reuse the same opaque text"
        );
        second.send(poodle_node::InboundFileEvent::Dropped {
            batch: inbound_batch(vec![file()]),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            second.released(),
            vec![(
                "batch-1".to_string(),
                poodle_node::InboundFileOutcome::Committed
            )]
        );
        assert_eq!(count_starting_with(&trace_of(&trace), "drop:"), 2);
    });
}

/// g16.027. Exactness has no threshold. A bounded tombstone would answer the
/// replay case correctly for a while and then silently stop — the key evicted
/// to make room is exactly the one a repeating host is most likely to send
/// again — so the first id answered stays inert after thousands of others,
/// while a fresh id and a replacement installation are still ordinary
/// business.
#[test]
fn the_first_answered_gpui_batch_id_stays_inert_after_thousands_of_later_ones() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let first = InboundStub::default();
        let second = InboundStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_inbound_file_bridge(Arc::new(first.clone()), cx));

        let node = Arc::new(Mutex::new(inbound_tree(
            &trace,
            poodle_node::InboundFileConstraints::default(),
        )));

        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();

        let zone = payload_frac("files-zone", 0.5, 0.5);
        let (x, y) = (f32::from(zone.x), f32::from(zone.y));
        let batch = |id: &str| poodle_node::InboundFileBatch {
            batch_id: id.to_string(),
            ..inbound_batch(vec![inbound_file(
                "batch:0",
                Some("take-01.wav"),
                "audio/wav",
                Some(16),
            )])
        };

        // One batch owns the controller, so every id after it takes the
        // busy-refusal path and is answered without a session of its own.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: batch("first"),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);

        // Comfortably past the 4096 a bounded tail used to keep.
        let later = 5_000;
        for index in 0..later {
            first.send(poodle_node::InboundFileEvent::Entered {
                batch: batch(&format!("answered-{index}")),
                x,
                y,
            });
        }
        driver.draw_frame();
        assert_eq!(first.released().len(), later);

        first.send(poodle_node::InboundFileEvent::Cancelled {
            batch_id: "first".to_string(),
        });
        driver.draw_frame();
        assert_eq!(first.released().len(), later + 1);
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);

        // The one a bounded tail would have forgotten first, and the one whose
        // terminal was the last thing remembered.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: batch("answered-0"),
            x,
            y,
        });
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: batch("first"),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(first.released().len(), later + 1);

        // Remembering does not make the window deaf.
        first.send(poodle_node::InboundFileEvent::Entered {
            batch: batch("fresh"),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        first.send(poodle_node::InboundFileEvent::Dropped {
            batch: batch("fresh"),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(
            first.released().last().cloned(),
            Some((
                "fresh".to_string(),
                poodle_node::InboundFileOutcome::Committed
            ))
        );

        // And a replacement installation still starts with no history.
        driver.update_app(|cx| {
            controller.set_inbound_file_bridge(Arc::new(second.clone()), cx)
        });
        driver.drain();
        second.send(poodle_node::InboundFileEvent::Entered {
            batch: batch("answered-0"),
            x,
            y,
        });
        driver.draw_frame();
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
    });
}

// ── g16.028 migrated composites on the native substrate ───────────────────
//
// EditableList, OrderBy, and BlockEditor drew reorder affordances that could
// not produce their contract result. These drive the completed paths through
// real mounted GPUI dispatch and assert the host spec each component rebuilt
// from — never a directly invoked handler.

/// EditableList's native reorder: two mounted lists holding the same item ids
/// cannot cross-drop, an accepted drop emits one complete next order, a
/// keyboard pickup commits through the same path, and a cancelled session
/// commits nothing.
#[test]
fn editable_list_substrate_reorder_rebuilds_the_host_spec() {
    use poodle_render::{editable_list, EditableListHandlers};
    use poodle_specs::{EditableListItem, EditableListSpec};

    struct ListHost {
        a: Vec<EditableListItem>,
        b: Vec<EditableListItem>,
        orders: Vec<(String, Vec<String>)>,
    }

    fn rows() -> Vec<EditableListItem> {
        vec![
            EditableListItem::new("row-1").with_label("One"),
            EditableListItem::new("row-2").with_label("Two"),
            EditableListItem::new("row-3").with_label("Three"),
        ]
    }

    fn ids(items: &[EditableListItem]) -> Vec<String> {
        items.iter().map(|item| item.id.clone()).collect()
    }

    fn build(host: &Arc<Mutex<ListHost>>, mounted: &Arc<Mutex<Node>>) -> Node {
        let rebuild = {
            let host = Arc::clone(host);
            let mounted = Arc::clone(mounted);
            move || {
                let next = build(&host, &mounted);
                *mounted.lock().expect("mount lock") = next;
            }
        };
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let (a, b) = {
            let host = host.lock().expect("host lock");
            (host.a.clone(), host.b.clone())
        };

        let list = |scope: &'static str, items: Vec<EditableListItem>| {
            let mut handlers = EditableListHandlers::new(scope);
            let host = Arc::clone(host);
            let rebuild = rebuild.clone();
            handlers.on_reorder = Some(Arc::new(move |next: &[EditableListItem]| {
                {
                    let mut host = host.lock().expect("host lock");
                    host.orders.push((scope.to_string(), ids(next)));
                    if scope == "list-a" {
                        host.a = next.to_vec();
                    } else {
                        host.b = next.to_vec();
                    }
                }
                rebuild();
            }));
            editable_list(
                &EditableListSpec::new()
                    .with_items(items)
                    .with_aria_label("Rows"),
                &ctx,
                handlers,
            )
        };

        let mut root = Node::container();
        root.style.descriptor.layout.direction = LayoutDirection::Column;
        root.style.descriptor.layout.spacing.gap = 8.0;
        root.child(list("list-a", a)).child(list("list-b", b))
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(ListHost {
            a: rows(),
            b: rows(),
            orders: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 320.0, 560.0);
        driver.draw_frame();
        let controller = driver.drag();

        // ── The same item ids in two lists never cross ──
        let handle = payload_frac("editable-list:list-a:row-1:handle", 0.5, 0.5);
        driver.pointer_press(handle);
        driver.pointer_drag(point(px(f32::from(handle.x) + 4.0), handle.y));
        driver.pointer_drag(payload_frac("editable-list:list-b:row-3:row", 0.5, 0.9));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Rejected),
            "the other list refuses this subject rather than accepting it"
        );
        assert_eq!(snapshot.position, None, "a refusal resolves no placement");
        driver.pointer_release(payload_frac("editable-list:list-b:row-3:row", 0.5, 0.9));
        assert!(
            host.lock().expect("host lock").orders.is_empty(),
            "a cross-list drop commits nothing"
        );

        // ── One accepted drop, one complete order ──
        let handle = payload_frac("editable-list:list-a:row-1:handle", 0.5, 0.5);
        driver.pointer_press(handle);
        driver.pointer_drag(point(px(f32::from(handle.x) + 4.0), handle.y));
        driver.pointer_drag(payload_frac("editable-list:list-a:row-3:row", 0.5, 0.9));
        driver.pointer_release(payload_frac("editable-list:list-a:row-3:row", 0.5, 0.9));
        {
            let host = host.lock().expect("host lock");
            assert_eq!(
                host.orders,
                [(
                    "list-a".to_string(),
                    vec![
                        "row-2".to_string(),
                        "row-3".to_string(),
                        "row-1".to_string()
                    ]
                )],
                "exactly one reorder, carrying the whole next order"
            );
            assert_eq!(ids(&host.b), ["row-1", "row-2", "row-3"], "list B is untouched");
        }

        // ── Keyboard pickup commits through the same path ──
        let drops_before = controller
            .announcements()
            .iter()
            .filter(|line| line.starts_with("Dropped "))
            .count();
        driver.wait_for_focus_handle("editable-list:list-a:row-2:handle");
        driver.focus_element("editable-list:list-a:row-2:handle");
        driver.dispatch_key_raw("space");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Dragging);
        driver.dispatch_key_raw("down");
        assert_eq!(
            controller.snapshot().target_id.as_deref(),
            Some("editable-list:list-a:target:row-3"),
            "traversal follows list order"
        );
        driver.dispatch_key_raw("space");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.orders.len(), 2, "one keystroke pair, one commit");
            assert_eq!(
                host.orders[1].1,
                ["row-3".to_string(), "row-2".to_string(), "row-1".to_string()]
            );
        }
        // The keyboard terminal is announced once, and focus stays on the
        // handle that was carrying the row.
        {
            let spoken = controller.announcements();
            let drops: Vec<&String> = spoken
                .iter()
                .filter(|line| line.starts_with("Dropped "))
                .collect();
            assert_eq!(
                drops.len() - drops_before,
                1,
                "one keyboard drop, one terminal: {spoken:?}"
            );
            assert_eq!(drops[drops.len() - 1], "Dropped Two after Three.");
        }
        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("editable-list:list-a:row-2:handle"),
            Some(true),
            "focus follows the row that was carried, not the row it passed"
        );

        // ── A cancelled keyboard session commits nothing and clears posture ──
        driver.wait_for_focus_handle("editable-list:list-a:row-3:handle");
        driver.focus_element("editable-list:list-a:row-3:handle");
        driver.dispatch_key_raw("space");
        driver.dispatch_key_raw("down");
        driver.dispatch_key_raw("escape");
        let snapshot = controller.snapshot();
        assert_eq!(snapshot.phase, DragSessionPhase::Idle);
        assert_eq!(snapshot.target_id, None);
        assert_eq!(snapshot.target_posture, None);
        assert_eq!(
            host.lock().expect("host lock").orders.len(),
            2,
            "Escape commits nothing"
        );
    });
}

/// OrderBy's native reorder: a pointer drop and the contract's Alt+Arrow both
/// emit one complete next ordering, and a clause's own controls still work.
#[test]
fn order_by_substrate_reorder_and_alt_arrow_rebuild_the_host_spec() {
    use poodle_render::{order_by, OrderByHandlers};
    use poodle_specs::{OrderByField, OrderBySpec, SortDirection, SortField};

    struct SortHost {
        value: Vec<OrderByField>,
        orderings: Vec<Vec<String>>,
        removed: Vec<String>,
    }

    fn keys(value: &[OrderByField]) -> Vec<String> {
        value.iter().map(|item| item.key.clone()).collect()
    }

    fn build(host: &Arc<Mutex<SortHost>>, mounted: &Arc<Mutex<Node>>) -> Node {
        let rebuild = {
            let host = Arc::clone(host);
            let mounted = Arc::clone(mounted);
            move || {
                let next = build(&host, &mounted);
                *mounted.lock().expect("mount lock") = next;
            }
        };
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let value = host.lock().expect("host lock").value.clone();

        let spec = OrderBySpec::new()
            .with_fields(vec![
                SortField::new("name", "Name"),
                SortField::new("date", "Date"),
                SortField::new("size", "Size"),
            ])
            .with_value(value)
            .with_open(true);

        let on_reorder = {
            let host = Arc::clone(host);
            let rebuild = rebuild.clone();
            Arc::new(move |next: &[OrderByField]| {
                {
                    let mut host = host.lock().expect("host lock");
                    host.orderings.push(keys(next));
                    host.value = next.to_vec();
                }
                rebuild();
            }) as Arc<dyn Fn(&[OrderByField]) + Send + Sync>
        };
        let on_remove = {
            let host = Arc::clone(host);
            Arc::new(move |key: &str| {
                host.lock().expect("host lock").removed.push(key.to_string());
            }) as Arc<dyn Fn(&str) + Send + Sync>
        };

        order_by(
            &spec,
            &ctx,
            OrderByHandlers {
                on_reorder: Some(on_reorder),
                on_remove: Some(on_remove),
                ..OrderByHandlers::new("sort")
            },
        )
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(SortHost {
            value: vec![
                OrderByField::new("name", SortDirection::Asc),
                OrderByField::new("date", SortDirection::Desc),
                OrderByField::new("size", SortDirection::Asc),
            ],
            orderings: Vec::new(),
            removed: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 360.0, 400.0);
        driver.draw_frame();
        let controller = driver.drag();

        // ── A drop lands the clause *at* the row it was dropped on ──
        let handle = payload_frac("order-by:sort:name:handle", 0.5, 0.5);
        driver.pointer_press(handle);
        driver.pointer_drag(point(px(f32::from(handle.x) + 4.0), handle.y));
        driver.pointer_drag(payload_frac("order-by:sort:size:row", 0.5, 0.5));
        let snapshot = controller.snapshot();
        assert_eq!(
            snapshot.target_id.as_deref(),
            Some("order-by:sort:target:size"),
            "the hovered row is the target the pointer is over"
        );
        assert_eq!(
            snapshot.position.as_deref(),
            Some("after"),
            "a clause travelling down arrives after its target"
        );
        driver.pointer_release(payload_frac("order-by:sort:size:row", 0.5, 0.5));
        assert_eq!(
            host.lock().expect("host lock").orderings,
            [vec![
                "date".to_string(),
                "size".to_string(),
                "name".to_string()
            ]],
            "one drop, one complete ordering"
        );

        // ── Alt+Arrow reaches the same emitter ──
        driver.keyboard_key("order-by:sort:size:handle", "alt-up");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.orderings.len(), 2, "one keystroke, one commit");
            assert_eq!(
                host.orderings[1],
                ["size".to_string(), "date".to_string(), "name".to_string()]
            );
        }

        // ── A plain Arrow on the handle is not a reorder ──
        driver.keyboard_key("order-by:sort:size:handle", "down");
        assert_eq!(
            host.lock().expect("host lock").orderings.len(),
            2,
            "the reorder chord is Alt+Arrow, not Arrow"
        );

        // ── The terminal is announced once, by the controller ──
        //
        // OrderBy has no live region of its own, so the substrate's is the
        // only voice and its source does not claim otherwise.
        let spoken = controller.announcements();
        let drops: Vec<&String> = spoken
            .iter()
            .filter(|line| line.starts_with("Dropped "))
            .collect();
        assert_eq!(drops.len(), 1, "one drop, one terminal announcement: {spoken:?}");

        // ── Focus is where the keyboard left it ──
        //
        // Alt+Arrow reaches the same emitter as a drop but is not a substrate
        // session: the native controller exposes no keyboard-drop command to a
        // renderer, so there is no session to announce or to return focus
        // from. The handle the person is on keeps focus, which is the part
        // that matters.
        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("order-by:sort:size:handle"),
            Some(true),
            "the reorder chord leaves focus on the handle it was pressed on"
        );
        assert_eq!(
            controller.announcements().len(),
            spoken.len(),
            "and being no session, it adds no announcement: {:?}",
            controller.announcements()
        );
    });
}

/// BlockEditor's native reorder: the grip is the drag source, the move buttons
/// are the keyboard route, and both emit one complete next block order. The
/// content area is not a drag handle.
#[test]
fn block_editor_grip_drag_and_move_controls_rebuild_the_host_spec() {
    use poodle_render::{block_editor, BlockEditorHandlers};
    use poodle_specs::{BlockEditorSpec, BlockTypeDefinition, EditorBlock};

    struct BlockHost {
        blocks: Vec<EditorBlock>,
        orders: Vec<Vec<String>>,
    }

    fn blocks() -> Vec<EditorBlock> {
        vec![
            EditorBlock::new("b1", "paragraph").with_content("one"),
            EditorBlock::new("b2", "paragraph").with_content("two"),
            EditorBlock::new("b3", "paragraph").with_content("three"),
        ]
    }

    fn ids(blocks: &[EditorBlock]) -> Vec<String> {
        blocks.iter().map(|block| block.id.clone()).collect()
    }

    fn build(host: &Arc<Mutex<BlockHost>>, mounted: &Arc<Mutex<Node>>) -> Node {
        let rebuild = {
            let host = Arc::clone(host);
            let mounted = Arc::clone(mounted);
            move || {
                let next = build(&host, &mounted);
                *mounted.lock().expect("mount lock") = next;
            }
        };
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let current = host.lock().expect("host lock").blocks.clone();

        let mut handlers = BlockEditorHandlers::new("editor");
        let sink = Arc::clone(host);
        handlers.on_reorder = Some(Arc::new(move |next: &[EditorBlock]| {
            {
                let mut host = sink.lock().expect("host lock");
                host.orders.push(ids(next));
                host.blocks = next.to_vec();
            }
            rebuild();
        }));

        block_editor(
            &BlockEditorSpec::new()
                .with_blocks(current)
                .with_block_types(vec![BlockTypeDefinition::new("paragraph", "Paragraph", "text")]),
            &ctx,
            handlers,
        )
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(BlockHost {
            blocks: blocks(),
            orders: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 560.0);
        driver.draw_frame();
        let controller = driver.drag();

        // ── The block body is an editing surface, not a grip ──
        let body = payload_frac("block-editor:editor:b1:block", 0.5, 0.85);
        driver.pointer_press(body);
        driver.pointer_drag(point(px(f32::from(body.x) + 24.0), body.y));
        assert_eq!(
            controller.snapshot().phase,
            DragSessionPhase::Idle,
            "a press in the block body never arms a drag"
        );
        driver.pointer_release(point(px(f32::from(body.x) + 24.0), body.y));

        // ── The grip drags, and the drop lands the block at its target ──
        let grip = payload_frac("block-editor:editor:b1:grip", 0.5, 0.5);
        driver.pointer_press(grip);
        driver.pointer_drag(point(px(f32::from(grip.x) + 4.0), grip.y));
        driver.pointer_drag(payload_frac("block-editor:editor:b3:block", 0.5, 0.5));
        driver.pointer_release(payload_frac("block-editor:editor:b3:block", 0.5, 0.5));
        assert_eq!(
            host.lock().expect("host lock").orders,
            [vec!["b2".to_string(), "b3".to_string(), "b1".to_string()]],
            "one drop, one complete block order"
        );

        // ── The terminal is announced once, by the controller ──
        //
        // BlockEditor has no live region of its own, so the substrate's is the
        // only voice.
        let spoken = controller.announcements();
        let drops: Vec<&String> = spoken
            .iter()
            .filter(|line| line.starts_with("Dropped "))
            .collect();
        assert_eq!(drops.len(), 1, "one drop, one terminal announcement: {spoken:?}");

        // ── Move up reaches the same emitter, and keeps its own focus ──
        driver.wait_for_focus_handle("block-editor:editor:b1:up");
        driver.keyboard_activate("block-editor:editor:b1:up");
        {
            let host = host.lock().expect("host lock");
            assert_eq!(host.orders.len(), 2, "one activation, one commit");
            assert_eq!(
                host.orders[1],
                ["b2".to_string(), "b1".to_string(), "b3".to_string()]
            );
        }
        driver.draw_frame();
        assert_eq!(
            poodle_gpui_node_backend::focus_state_for("block-editor:editor:b1:up"),
            Some(true),
            "the move control keeps focus on the block it moved"
        );
        assert_eq!(
            controller.announcements().len(),
            spoken.len(),
            "a move control is not a substrate session, so it adds no announcement: {:?}",
            controller.announcements()
        );
    });
}

/// g16.028 review. A composite with its own live region narrates its own
/// sessions, and the controller says nothing about them.
///
/// The catalogue announces "Moved X to position N of M" through `on_announce`.
/// Without `NodeDragSource::owns_announcements` the controller also composes
/// "Dropped X on Y", so one drop is read out twice in two different sentences.
#[test]
fn model_catalogue_editor_pointer_drop_is_announced_once_by_the_editor() {
    use poodle_headless::model_connection::ModelCatalogueItem;
    use poodle_specs::ModelCatalogueEditorSpec;

    struct CatalogueHost {
        items: Vec<ModelCatalogueItem>,
        orders: Vec<Vec<String>>,
        announcements: Vec<String>,
    }

    fn build(host: &Arc<Mutex<CatalogueHost>>, mounted: &Arc<Mutex<Node>>) -> Node {
        let rebuild = {
            let host = Arc::clone(host);
            let mounted = Arc::clone(mounted);
            move || {
                let next = build(&host, &mounted);
                *mounted.lock().expect("mount lock") = next;
            }
        };
        let theme = theme();
        let items = host.lock().expect("host lock").items.clone();
        let order_sink = Arc::clone(host);
        let announce_sink = Arc::clone(host);
        poodle_render::model_catalogue_editor(
            &ModelCatalogueEditorSpec::new().with_items(items),
            &RenderContext::new(&theme),
            poodle_render::ModelCatalogueEditorHandlers {
                on_order_change: Some(Arc::new(move |order: &[String]| {
                    {
                        let mut host = order_sink.lock().expect("host lock");
                        host.orders.push(order.to_vec());
                        let mut next = Vec::new();
                        for id in order {
                            if let Some(item) =
                                host.items.iter().find(|item| &item.id == id).cloned()
                            {
                                next.push(item);
                            }
                        }
                        host.items = next;
                    }
                    rebuild();
                })),
                on_announce: Some(Arc::new(move |message: &str| {
                    announce_sink
                        .lock()
                        .expect("host lock")
                        .announcements
                        .push(message.to_string())
                })),
                ..poodle_render::ModelCatalogueEditorHandlers::default()
            },
        )
    }

    run_headless(|cx| {
        let host = Arc::new(Mutex::new(CatalogueHost {
            items: vec![
                ModelCatalogueItem::new("model-alpha", "Alpha"),
                ModelCatalogueItem::new("model-beta", "Beta"),
                ModelCatalogueItem::new("model-gamma", "Gamma"),
            ],
            orders: Vec::new(),
            announcements: Vec::new(),
        }));
        let mounted = Arc::new(Mutex::new(Node::container()));
        *mounted.lock().expect("mount lock") = build(&host, &mounted);
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 420.0, 420.0);
        driver.draw_frame();
        let controller = driver.drag();

        let handle = payload_frac("model-catalogue-editor:model-alpha:handle", 0.5, 0.5);
        driver.pointer_press(handle);
        driver.pointer_drag(point(px(f32::from(handle.x) + 4.0), handle.y));
        driver.pointer_drag(payload_frac("model-catalogue-editor:model-gamma:handle", 0.5, 0.5));
        driver.pointer_release(payload_frac(
            "model-catalogue-editor:model-gamma:handle",
            0.5,
            0.5,
        ));

        let host = host.lock().expect("host lock");
        assert_eq!(
            host.orders,
            [vec![
                "model-beta".to_string(),
                "model-gamma".to_string(),
                "model-alpha".to_string()
            ]],
            "one drop, one complete shown order"
        );
        assert_eq!(
            host.announcements,
            ["Moved Alpha to position 3 of 3."],
            "the editor's own region says it once"
        );
        assert!(
            controller.announcements().is_empty(),
            "and the controller says nothing about a session its source narrates: {:?}",
            controller.announcements()
        );
    });
}

/// g16.028 review round 2. The renderer-neutral half of `ownsAnnouncements`:
/// a self-narrating source silences the controller for its whole session, and
/// for no other session.
///
/// The guarantee is a latch taken at session start, not a lookup. `active_source`
/// is cleared during terminal cleanup, and the node tree can drop the
/// registration mid-drag, so a lookup would find nothing exactly when the
/// terminal announcement lands and would narrate it after all.
#[test]
fn a_self_narrating_source_silences_its_whole_session_and_only_that_session() {
    fn tree(trace: &Arc<Mutex<Vec<String>>>, quiet_present: bool) -> Node {
        let mut quiet = drag_box("quiet-source", 60.0, 40.0);
        quiet.interaction.focusable = true;
        quiet.a11y.tab_index = Some(0);
        if quiet_present {
            let mut registration = traced_source("quiet-source", "Quiet", trace);
            registration.owns_announcements = true;
            quiet.interaction.drag_source = Some(registration);
        }

        let mut loud = drag_box("loud-source", 60.0, 40.0);
        loud.interaction.focusable = true;
        loud.interaction.drag_source = Some(traced_source("loud-source", "Loud", trace));

        let mut zone = drag_box("quiet-zone", 60.0, 40.0);
        zone.interaction.drop_target = Some(traced_target(
            "quiet-zone",
            "Zone",
            trace,
            false,
            1,
            NodeDropCommit::Committed,
        ));

        let mut row = Node::container();
        row.id = Some("quiet-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(180.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        row.child(quiet).child(loud).child(zone)
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(tree(&trace, true)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 260.0, 80.0);
        driver.draw_frame();
        let controller = driver.drag();

        // ── Pickup and hover intent are silent ──
        let origin = payload_frac("quiet-source", 0.5, 0.5);
        driver.pointer_press(origin);
        driver.pointer_drag(point(px(f32::from(origin.x) + 4.0), origin.y));
        driver.pointer_drag(payload_frac("quiet-zone", 0.5, 0.5));
        assert_eq!(
            controller.snapshot().target_posture,
            Some(poodle_gpui_node_backend::DragDropTargetPosture::Accepted)
        );
        assert!(
            controller.announcements().is_empty(),
            "pickup and intent are the source's to narrate: {:?}",
            controller.announcements()
        );

        // ── The registration leaves mid-drag; the terminal is still silent ──
        //
        // This is the case a live lookup gets wrong: there is no source to ask
        // by the time the drop is announced.
        driver.mount_node(Arc::new(Mutex::new(tree(&trace, false))));
        driver.draw_frame();
        driver.pointer_release(payload_frac("quiet-zone", 0.5, 0.5));
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(
            controller.announcements().is_empty(),
            "a terminal after the source left is still the source's: {:?}",
            controller.announcements()
        );

        // ── A cancelled self-narrated session is silent too ──
        driver.mount_node(Arc::new(Mutex::new(tree(&trace, true))));
        driver.draw_frame();
        let origin = payload_frac("quiet-source", 0.5, 0.5);
        driver.pointer_press(origin);
        driver.pointer_drag(point(px(f32::from(origin.x) + 4.0), origin.y));
        driver.pointer_drag(payload_frac("quiet-zone", 0.5, 0.5));
        driver.dispatch_key("escape");
        assert_eq!(controller.snapshot().phase, DragSessionPhase::Idle);
        assert!(
            controller.announcements().is_empty(),
            "cancel is a terminal, and this session's terminals are silent: {:?}",
            controller.announcements()
        );

        // ── The latch resets: an ordinary source in the same controller is
        //    narrated again ──
        let origin = payload_frac("loud-source", 0.5, 0.5);
        driver.pointer_press(origin);
        driver.pointer_drag(point(px(f32::from(origin.x) + 4.0), origin.y));
        driver.pointer_drag(payload_frac("quiet-zone", 0.5, 0.5));
        driver.pointer_release(payload_frac("quiet-zone", 0.5, 0.5));
        let spoken = controller.announcements();
        assert!(
            spoken.iter().any(|line| line.contains("Loud")),
            "the next ordinary session is narrated: {spoken:?}"
        );
        assert!(
            !spoken.iter().any(|line| line.contains("Quiet")),
            "and the silenced session never appears late: {spoken:?}"
        );
    });
}

/// g16.028 review round 3. An incoming cross-window projection is never
/// silenced by the local session that ran before it.
///
/// `owns_announcements` belongs to a *source*, and a projection has none: it
/// arrives from another window with no local registration at all, so this
/// controller's live region is the only voice it will ever have. Leaving the
/// previous session's latch in place would hand a remote drag the silence a
/// local composite asked for, and there is nothing on the other side to notice.
#[test]
fn an_incoming_projection_is_narrated_after_a_self_narrating_local_session() {
    fn tree(trace: &Arc<Mutex<Vec<String>>>) -> Node {
        let mut source = drag_box("xw-source", 60.0, 40.0);
        source.interaction.focusable = true;
        source.a11y.tab_index = Some(0);
        let mut registration = traced_source("xw-source", "Local", trace);
        registration.owns_announcements = true;
        source.interaction.drag_source = Some(registration);

        let mut zone = drag_box("xw-zone-a", 60.0, 40.0);
        zone.interaction.drop_target = Some(traced_target(
            "xw-zone-a",
            "Zone A",
            trace,
            false,
            1,
            NodeDropCommit::Committed,
        ));

        let mut row = Node::container();
        row.id = Some("xw-row".to_owned());
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        row.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        row.child(source).child(zone)
    }

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let host = HostStub::default();
        let controller = poodle_gpui_node_backend::DragDropController::new();
        cx.update(|cx| controller.set_cross_window_target_bridge(Arc::new(host.clone()), cx));

        let node = Arc::new(Mutex::new(tree(&trace)));
        let build = {
            let controller = controller.clone();
            let node = Arc::clone(&node);
            Rc::new(move || {
                let tree = node.lock().expect("lock").clone();
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::drag_drop_provider(
                        &controller,
                        || gpui::div().child(poodle_gpui_node_backend::to_gpui(&tree)),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };

        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();
        driver.drain();

        // ── A complete local session from the self-narrating source ──
        let origin = payload_frac("xw-source", 0.5, 0.5);
        driver.pointer_press(origin);
        driver.pointer_drag(point(px(f32::from(origin.x) + 4.0), origin.y));
        driver.pointer_drag(payload_frac("xw-zone-a", 0.5, 0.5));
        driver.pointer_release(payload_frac("xw-zone-a", 0.5, 0.5));
        driver.drain();
        assert!(
            controller.announcements().is_empty(),
            "the local session narrates itself: {:?}",
            controller.announcements()
        );

        // ── An incoming projection right afterwards is the controller's to
        //    narrate: it has no local source to have asked for silence ──
        host.project(projection_for("lease-1", Some("xw-zone-a")));
        driver.drain();
        let spoken = controller.announcements();
        assert!(
            spoken.iter().any(|line| line.contains("Remote")),
            "an incoming projection is announced: {spoken:?}"
        );
        // And announced by its *name*. The projection carries the host's own
        // accessible label precisely because this window has no source to ask;
        // falling back to the subject id would read an opaque identifier to
        // the one person who cannot see the row it names.
        assert!(
            !spoken.iter().any(|line| line.contains("remote-row")),
            "the accessible name is the projection's label, never its subject id: {spoken:?}"
        );

        // ── And so is its terminal ──
        host.cancel_from_host(
            poodle_node::CrossWindowDragReceipt {
                protocol_version: poodle_node::CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
                token: "lease-1".to_string(),
            },
            poodle_node::DragCancelReason::TransportLost,
        );
        driver.drain();
        let after = controller.announcements();
        assert!(
            after.len() > spoken.len(),
            "the projection's terminal is announced too: {after:?}"
        );
        assert!(
            after[spoken.len()..].iter().all(|line| !line.contains("remote-row")),
            "including at the terminal, where the label must not decay: {after:?}"
        );
    });
}

// ── HistoryCenter rejection surface (g16.033) ──────────────────────────────

/// The exact contract table (`docs/contracts/components/history-center.md`
/// §"Rejection handling"), in declaration order. Every native proof below
/// reads this list, so deleting a category or collapsing one onto another
/// fails here.
const HISTORY_CENTER_REJECTION_COPY: [(HistoryCenterRejection, &str); 5] = [
    (
        HistoryCenterRejection::AlreadyAtTarget,
        "Already at the requested target",
    ),
    (HistoryCenterRejection::UnknownEntry, "Entry does not exist"),
    (
        HistoryCenterRejection::StaleHistory,
        "History changed; this entry was not deleted",
    ),
    (
        HistoryCenterRejection::ProtectedEntry,
        "This history entry is protected",
    ),
    (
        HistoryCenterRejection::DeletionUnavailable,
        "History deletion is unavailable",
    ),
];

/// g16.033: every accepted refusal reaches a native operator as its own line.
///
/// This is the mounted counterpart of the resolver proofs in `poodle-core`,
/// `poodle-headless`, `poodle-specs` and `poodle-render`: each of the five
/// codes is mapped by the component, painted into a real GPUI window through
/// the real node backend, and read back off the surface that actually mounted.
/// The papercut this replaced showed a stale revision, a protected entry and
/// an unavailable deletion all as "Entry does not exist" — so the assertion
/// that matters is that the five lines are five, and that no deletion refusal
/// is the unknown-entry line.
#[test]
fn every_history_center_rejection_mounts_its_own_native_copy() {
    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let unknown = HistoryCenterSpec::new()
            .with_rejection(HistoryCenterRejection::UnknownEntry)
            .rejection_message()
            .expect("the unknown-entry code resolves copy");
        let mut mounted: Vec<String> = Vec::new();

        for (code, expected) in HISTORY_CENTER_REJECTION_COPY {
            let spec = HistoryCenterSpec::new()
                .with_can_undo(true)
                .with_open(true)
                .with_rejection(code);
            let message = spec
                .rejection_message()
                .unwrap_or_else(|| panic!("{code:?} resolves component-owned copy"));
            assert_eq!(message, expected, "{code:?} owns exact copy");

            let node = history_center(
                &spec,
                &ctx,
                &HistoryCenterView {
                    is_open: true,
                    rejection: Some(message.to_owned()),
                    ..HistoryCenterView::default()
                },
                &HistoryCenterHandlers::default(),
            );

            let notice = node
                .find(&|candidate| {
                    candidate.id.as_deref() == Some(poodle_render::history_center::HISTORY_CENTER_REJECTION_ID)
                })
                .unwrap_or_else(|| panic!("{code:?} paints a rejection notice"))
                .clone();
            assert_eq!(notice.a11y.role, Some(NodeRole::Status));
            assert_eq!(notice.texts(), vec![expected]);
            if code != HistoryCenterRejection::UnknownEntry {
                assert_ne!(
                    notice.texts(),
                    vec![unknown],
                    "{code:?} must not read as a missing entry",
                );
            }

            // Mount the whole surface in a real window through the real
            // backend. A node tree that only exists in memory proves nothing
            // about what a native operator sees.
            let mounted_node = Arc::new(Mutex::new(node));
            let mut driver =
                HeadlessDriver::new_in_box(cx, Arc::clone(&mounted_node), 640.0, 520.0);
            driver.wait_for_focus_handle(poodle_render::history_center::HISTORY_CENTER_UNDO_ID);
            driver.draw_frame();

            mounted.push(notice.texts().join(""));
        }

        let distinct = mounted.len();
        mounted.sort();
        mounted.dedup();
        assert_eq!(
            mounted.len(),
            distinct,
            "five refusal meanings must mount as five distinct lines",
        );
    });
}

/// g16.035. Production MarkdownEditor preview scroll under a definite host
/// height, through the real GPUI node backend. The fixture stamps runtime ids
/// and synthetic overflow content only — it does not mutate production sizing
/// or overflow. The host wrapper is the sole definite height constraint.
#[test]
fn markdown_editor_bounded_preview_scrolls_under_host_height() {
    use poodle_render::markdown_editor;
    use poodle_specs::MarkdownEditorSpec;

    const HOST_W: f32 = 320.0;
    const HOST_H: f32 = 256.0; // 16rem at 16px
    const ROW_H: f32 = 28.0;
    const ROW_COUNT: usize = 40;

    fn long_markdown() -> String {
        (1..=40)
            .map(|n| {
                format!(
                    "## Heading {n}\n\nParagraph {n} forces the preview past a 16rem host.\n"
                )
            })
            .collect()
    }

    /// Fixture-only runtime ids + synthetic overflow rows. No sizing/overflow
    /// mutations on the production editor/body/preview nodes.
    fn stamp_bounded_preview(editor: &mut Node, activated: &Arc<Mutex<bool>>) {
        editor.runtime_id = Some("md-editor".to_owned());

        let body = editor.children.get_mut(1).expect("toolbar then body");
        let preview = body
            .children
            .iter_mut()
            .find(|child| child.a11y.label.as_deref() == Some("Preview"))
            .expect("preview pane");
        preview.runtime_id = Some("md-preview".to_owned());
        // Replace production source-text children with fixture overflow only —
        // content stamp, not sizing/overflow mutation.
        preview.children.clear();

        // Column runway as a child — does not rewrite preview direction/sizing.
        let mut runway = Node::container();
        runway.runtime_id = Some("md-preview-runway".to_owned());
        runway.style.descriptor.layout.direction = LayoutDirection::Column;
        runway.style.fill_width = true;

        for index in 0..(ROW_COUNT - 1) {
            let mut row = Node::container();
            row.runtime_id = Some(format!("md-preview-row-{index}"));
            {
                let s = &mut row.style;
                s.descriptor.layout.width = LayoutSizing::Grow;
                s.descriptor.layout.height = LayoutSizing::Fixed(ROW_H);
                s.min_height = Some(ROW_H);
                s.fill_width = true;
            }
            row = row.child(Node::text(format!("row {index}")));
            runway = runway.child(row);
        }

        let seen = Arc::clone(activated);
        let mut tail = Node::button("tail").interaction_on_activate(move || {
            *seen.lock().expect("activated lock") = true;
        });
        tail.runtime_id = Some("md-preview-tail".to_owned());
        tail.a11y.label = Some("Preview tail".to_owned());
        tail.interaction.focusable = true;
        {
            let s = &mut tail.style;
            s.descriptor.layout.width = LayoutSizing::Grow;
            s.descriptor.layout.height = LayoutSizing::Fixed(ROW_H);
            s.min_height = Some(ROW_H);
            s.fill_width = true;
        }
        runway = runway.child(tail);
        preview.children.push(runway);
    }

    fn wrap_host(editor: Node) -> Node {
        let mut host = Node::container();
        host.runtime_id = Some("md-host".to_owned());
        {
            let s = &mut host.style;
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.width = LayoutSizing::Fixed(HOST_W);
            s.descriptor.layout.height = LayoutSizing::Fixed(HOST_H);
            s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
            s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        }
        host.child(editor)
    }

    run_headless(|cx| {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let spec = MarkdownEditorSpec::new()
            .with_mode("preview")
            .with_value(long_markdown());
        let activated = Arc::new(Mutex::new(false));
        let mut editor = markdown_editor(&spec, &ctx);
        stamp_bounded_preview(&mut editor, &activated);

        let mounted = Arc::new(Mutex::new(wrap_host(editor)));
        let mut driver =
            HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), HOST_W + 64.0, HOST_H + 64.0);
        driver.draw_frame();

        let host = poodle_gpui_node_backend::bounds_for("md-host").expect("host bounds");
        let editor_bounds =
            poodle_gpui_node_backend::bounds_for("md-editor").expect("editor bounds");
        let preview = poodle_gpui_node_backend::bounds_for("md-preview").expect("preview bounds");
        assert!(
            (f32::from(host.size.height) - HOST_H).abs() < 1.0,
            "host keeps the fixture height {}",
            f32::from(host.size.height)
        );
        assert!(
            f32::from(editor_bounds.size.height) <= f32::from(host.size.height) + 1.0,
            "editor stays within host: editor={} host={}",
            f32::from(editor_bounds.size.height),
            f32::from(host.size.height)
        );
        assert!(
            f32::from(preview.size.height) < f32::from(host.size.height),
            "preview sits under the toolbar inside the host: preview={} host={}",
            f32::from(preview.size.height),
            f32::from(host.size.height)
        );

        let first = poodle_gpui_node_backend::bounds_for("md-preview-row-0").expect("first row");
        let tail = poodle_gpui_node_backend::bounds_for("md-preview-tail").expect("tail bounds");
        assert!(
            f32::from(tail.origin.y) + f32::from(tail.size.height)
                > f32::from(preview.origin.y) + f32::from(preview.size.height) + 8.0,
            "fixture tail must sit past the preview viewport"
        );
        assert!(
            (f32::from(first.size.height) - ROW_H).abs() < 2.0,
            "fixture rows keep their fixed height {}",
            f32::from(first.size.height)
        );

        driver.pointer_activate_id("md-preview-tail");
        assert!(
            !*activated.lock().expect("activated lock"),
            "clipped preview tail must not activate before scroll"
        );

        // GPUI scroll offset is `[-max, 0]`. Negative pixel delta moves down.
        driver.scroll_vertical_id("md-preview", -5000.0);
        let first_after =
            poodle_gpui_node_backend::bounds_for("md-preview-row-0").expect("first row after scroll");
        assert!(
            f32::from(first_after.origin.y) < f32::from(first.origin.y) - 8.0,
            "wheel must move preview content: before={} after={}",
            f32::from(first.origin.y),
            f32::from(first_after.origin.y)
        );

        let editor_after =
            poodle_gpui_node_backend::bounds_for("md-editor").expect("editor after scroll");
        let host_after = poodle_gpui_node_backend::bounds_for("md-host").expect("host after scroll");
        assert!(
            (f32::from(editor_after.origin.y) - f32::from(editor_bounds.origin.y)).abs() < 1.0
                && (f32::from(editor_after.size.height) - f32::from(editor_bounds.size.height))
                    .abs()
                    < 1.0,
            "editor root stays put while preview scrolls"
        );
        assert!(
            (f32::from(host_after.origin.y) - f32::from(host.origin.y)).abs() < 1.0
                && (f32::from(host_after.size.height) - f32::from(host.size.height)).abs() < 1.0,
            "host stays put while preview scrolls"
        );

        driver.pointer_activate_id("md-preview-tail");
        assert!(
            *activated.lock().expect("activated lock"),
            "wheel scrolling must bring the clipped preview tail into hit-test"
        );
    });
}

/// g16.034. Construction-time motion: loops wait for a committed first frame,
/// preloaded toasts paint the endpoint, underline is one paint-only indicator,
/// and unsupported translation is a named GPUI approximation.
#[test]
fn mounted_motion_policy_construction_does_not_invent_clocks() {
    run_headless(|cx| {
        let capture = Arc::new(Mutex::new(Vec::<&'static str>::new()));
        let tree = Arc::new(Mutex::new(skeleton(&SkeletonSpec::new(), &RenderContext::new(&theme()))));
        let build = {
            let capture = Arc::clone(&capture);
            let tree = Arc::clone(&tree);
            Rc::new(move || {
                let node = tree.lock().expect("tree lock").clone();
                poodle_gpui_node_backend::begin_probe_capture();
                use gpui::{IntoElement as _, ParentElement as _};
                let element = gpui::div()
                    .child(poodle_gpui_node_backend::to_gpui(&node))
                    .into_any_element();
                *capture.lock().expect("capture lock") = poodle_gpui_node_backend::take_probe_capture();
                element
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };
        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();
        assert!(
            !capture
                .lock()
                .expect("capture lock")
                .contains(&"surface.animation.scheduled"),
            "default skeleton construction must not schedule a loop"
        );

        *tree.lock().expect("tree lock") = skeleton(
            &SkeletonSpec::new(),
            &RenderContext::new(&theme()).with_first_frame_committed(true),
        );
        driver.draw_frame();
        assert!(
            capture
                .lock()
                .expect("capture lock")
                .contains(&"surface.animation.scheduled"),
            "a committed first frame may attach the skeleton pulse"
        );

        *tree.lock().expect("tree lock") = spinner(
            &SpinnerSpec::default(),
            &RenderContext::new(&theme()),
        );
        driver.draw_frame();
        assert!(
            !capture
                .lock()
                .expect("capture lock")
                .contains(&"surface.animation.scheduled"),
            "default spinner construction must not schedule a loop"
        );

        *tree.lock().expect("tree lock") = toast_stack(
            &ToastStackSpec::new().with_toasts(vec![Toast::new("save", "Saved")]),
            &RenderContext::new(&theme()),
            ToastStackHandlers::default(),
        );
        driver.draw_frame();
        assert!(
            !capture
                .lock()
                .expect("capture lock")
                .contains(&"surface.animation.scheduled"),
            "preloaded toasts must not enter"
        );

        *tree.lock().expect("tree lock") = tabs(
            &TabsSpec::new(vec![
                TabDefinition::new("a", "A"),
                TabDefinition::new("b", "B"),
            ])
            .with_variant(TabVariant::Block)
            .with_active_edge(ActiveEdge::Underline)
            .with_value("a"),
            &RenderContext::new(&theme()),
            None,
            None,
        );
        driver.draw_frame();
        let tabs_tree = tree.lock().expect("tree lock").clone();
        assert!(
            tabs_tree
                .find(&|n| n.id.as_deref() == Some("poodle-tabs-indicator"))
                .is_some(),
            "underline uses one paint-only indicator"
        );
        let selected = tabs_tree
            .find(&|n| n.a11y.selected == Some(true))
            .expect("selected tab");
        assert_eq!(selected.style.border_bottom_width, None);
        assert_eq!(selected.style.border_color_bottom, None);

        let mut translated = Node::container();
        translated.style.animation = Some(NodeAnimation {
            key: "g16-034-translate".into(),
            keyframes: vec![
                AnimKeyframe {
                    at: 0.0,
                    values: vec![(AnimProperty::TranslateY, 8.0)],
                },
                AnimKeyframe {
                    at: 1.0,
                    values: vec![(AnimProperty::TranslateY, 0.0)],
                },
            ],
            duration_secs: 0.18,
            easing: AnimEasing::EaseOut,
            loop_mode: AnimLoop::Once,
        });
        *tree.lock().expect("tree lock") = translated;
        driver.draw_frame();
        let channels = capture.lock().expect("capture lock").clone();
        assert!(
            channels.contains(&"surface.animation.approximation.opacity-stand-in"),
            "unsupported translation must stay a named approximation: {channels:?}"
        );
    });
}

/// g16.047. Native danger projects Alert; success stays ListItem. Drawing the
/// node tree does not claim GPUI assistive-technology parity.
#[test]
fn mounted_toast_danger_uses_alert_role() {
    run_headless(|cx| {
        let node = toast_stack(
            &ToastStackSpec::new().with_toasts(vec![
                Toast::new("ok", "Saved").with_tone(ToastTone::Success),
                Toast::new("fail", "Publishing failed").with_tone(ToastTone::Danger),
            ]),
            &RenderContext::new(&theme()),
            ToastStackHandlers::default(),
        );
        assert_eq!(
            node.find(&|n| n.id.as_deref() == Some("poodle-toast-ok"))
                .expect("success toast")
                .a11y
                .role,
            Some(NodeRole::ListItem)
        );
        assert_eq!(
            node.find(&|n| n.id.as_deref() == Some("poodle-toast-fail"))
                .expect("danger toast")
                .a11y
                .role,
            Some(NodeRole::Alert)
        );

        let tree = Arc::new(Mutex::new(node));
        let build = {
            let tree = Arc::clone(&tree);
            Rc::new(move || {
                use gpui::{IntoElement as _, ParentElement as _};
                gpui::div()
                    .child(poodle_gpui_node_backend::to_gpui(
                        &tree.lock().expect("tree lock").clone(),
                    ))
                    .into_any_element()
            }) as Rc<dyn Fn() -> gpui::AnyElement>
        };
        let mut driver = HeadlessDriver::new_element(cx, build);
        driver.draw_frame();
        let mounted = tree.lock().expect("tree lock").clone();
        assert_eq!(
            mounted
                .find(&|n| n.id.as_deref() == Some("poodle-toast-fail"))
                .expect("mounted danger")
                .a11y
                .role,
            Some(NodeRole::Alert)
        );
    });
}

/// g16.066. Window-owned tooltip lifecycle runtime: 300ms delay, hover/focus
/// trigger, pointer leave, focus departure (blur), Escape, click dismissal,
/// target supersession, removal sweep, disabled inertness, and window isolation.
#[test]
fn gpui_node_tooltip_delay_timing_and_mounted_lifecycle() {
    use poodle_gpui_node_backend::{
        is_tooltip_pending, is_tooltip_visible, painted_tooltip, TOOLTIP_DELAY,
    };
    use std::time::Duration;

    run_headless(|cx| {
        let mut save_btn = Node::button("Save");
        save_btn.id = Some("save-button".into());
        save_btn.interaction.focusable = true;
        save_btn.a11y.tab_index = Some(0);
        save_btn.tooltip = Some("Save document".into());
        save_btn.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        save_btn.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);

        let mounted = Arc::new(Mutex::new(save_btn));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 400.0, 300.0);
        driver.wait_for_focus_handle("save-button");

        let target_bounds = poodle_gpui_node_backend::bounds_for("save-button")
            .expect("save-button painted bounds");
        let target_center = target_bounds.center();
        let outside = point(px(10.0), px(10.0));

        assert!(!is_tooltip_pending("save-button"));
        assert!(!is_tooltip_visible("save-button"));
        assert!(painted_tooltip().is_none());

        driver.pointer_hover(target_center);
        assert!(
            is_tooltip_pending("save-button"),
            "pointer hover must start a pending timer"
        );
        assert!(!is_tooltip_visible("save-button"));

        driver.advance_clock(Duration::from_millis(299));
        driver.draw_frame();
        assert!(
            is_tooltip_pending("save-button"),
            "tooltip must remain pending at 299ms"
        );
        assert!(
            !is_tooltip_visible("save-button"),
            "tooltip must stay absent at 299ms"
        );
        assert!(painted_tooltip().is_none());

        driver.advance_clock(Duration::from_millis(1));
        driver.draw_frame();
        assert!(
            is_tooltip_visible("save-button"),
            "tooltip must become visible at 300ms"
        );
        assert!(!is_tooltip_pending("save-button"));

        let painted = painted_tooltip().expect("painted tooltip at 300ms");
        assert_eq!(painted.target_id, "save-button");
        assert_eq!(painted.text, "Save document");

        driver.pointer_hover(outside);
        assert!(
            !is_tooltip_visible("save-button"),
            "pointer leave must hide visible tooltip immediately"
        );
        assert!(painted_tooltip().is_none());

        driver.pointer_hover(target_center);
        assert!(is_tooltip_pending("save-button"));
        driver.advance_clock(Duration::from_millis(100));
        driver.pointer_hover(outside);
        assert!(
            !is_tooltip_pending("save-button"),
            "early pointer leave must cancel pending timer"
        );
        driver.advance_clock(Duration::from_millis(500));
        driver.draw_frame();
        assert!(
            !is_tooltip_visible("save-button"),
            "cancelled timer must never paint tooltip"
        );
        assert!(painted_tooltip().is_none());

        driver.focus_element("save-button");
        assert!(
            is_tooltip_pending("save-button"),
            "focus enter must start pending timer"
        );
        driver.advance_clock(TOOLTIP_DELAY);
        driver.draw_frame();
        assert!(
            is_tooltip_visible("save-button"),
            "focus timer at 300ms must make tooltip visible"
        );
        assert!(painted_tooltip().is_some());

        driver.blur_element_focus("save-button");
        assert!(
            !is_tooltip_visible("save-button"),
            "focus departure must hide tooltip immediately"
        );
        assert!(painted_tooltip().is_none());

        driver.pointer_hover(target_center);
        driver.advance_clock(TOOLTIP_DELAY);
        driver.draw_frame();
        assert!(is_tooltip_visible("save-button"));

        driver.dispatch_key("escape");
        assert!(
            !is_tooltip_visible("save-button"),
            "Escape must dismiss visible tooltip"
        );
        assert!(painted_tooltip().is_none());

        driver.pointer_hover(outside);
        driver.pointer_hover(target_center);
        driver.advance_clock(TOOLTIP_DELAY);
        driver.draw_frame();
        assert!(is_tooltip_visible("save-button"));

        driver.pointer_press(target_center);
        assert!(
            !is_tooltip_visible("save-button"),
            "pointer press must dismiss visible tooltip"
        );
        driver.pointer_release(target_center);
    });
}

/// g16.066. Target supersession, paint authority, disabled inertness, and empty tooltips.
#[test]
fn gpui_node_tooltip_generation_supersession_paint_authority_and_disabled() {
    use poodle_gpui_node_backend::{
        is_tooltip_pending, is_tooltip_visible, painted_tooltip, TOOLTIP_DELAY,
    };
    use std::time::Duration;

    run_headless(|cx| {
        let mut btn_a = Node::button("A");
        btn_a.id = Some("btn-a".into());
        btn_a.tooltip = Some("Alpha action".into());
        btn_a.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        btn_a.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);

        let mut btn_b = Node::button("B");
        btn_b.id = Some("btn-b".into());
        btn_b.tooltip = Some("Beta action".into());
        btn_b.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        btn_b.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);

        let mut btn_disabled = Node::button("Disabled");
        btn_disabled.id = Some("btn-disabled".into());
        btn_disabled.interaction.disabled = true;
        btn_disabled.tooltip = Some("Cannot activate".into());
        btn_disabled.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        btn_disabled.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);

        let mut btn_empty = Node::button("Empty");
        btn_empty.id = Some("btn-empty".into());
        btn_empty.tooltip = Some(String::new());
        btn_empty.style.descriptor.layout.width = LayoutSizing::Fixed(80.0);
        btn_empty.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);

        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.spacing.gap = 12.0;
        row = row
            .child(btn_a)
            .child(btn_b)
            .child(btn_disabled)
            .child(btn_empty);

        let mounted = Arc::new(Mutex::new(row));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 500.0, 300.0);
        driver.draw_frame();

        let bounds_a = poodle_gpui_node_backend::bounds_for("btn-a").expect("bounds for A");
        let bounds_b = poodle_gpui_node_backend::bounds_for("btn-b").expect("bounds for B");
        let bounds_disabled =
            poodle_gpui_node_backend::bounds_for("btn-disabled").expect("bounds for disabled");
        let bounds_empty =
            poodle_gpui_node_backend::bounds_for("btn-empty").expect("bounds for empty");

        driver.pointer_hover(bounds_a.center());
        assert!(is_tooltip_pending("btn-a"));

        driver.advance_clock(Duration::from_millis(150));
        assert!(is_tooltip_pending("btn-a"));

        driver.pointer_hover(bounds_b.center());
        assert!(
            !is_tooltip_pending("btn-a"),
            "hovering B must cancel A's pending state"
        );
        assert!(is_tooltip_pending("btn-b"));

        driver.advance_clock(Duration::from_millis(200));
        driver.draw_frame();
        assert!(
            !is_tooltip_visible("btn-a"),
            "A must never paint: superseded generation is inert"
        );
        assert!(
            !is_tooltip_visible("btn-b"),
            "B is not visible yet (only 200ms elapsed)"
        );

        driver.advance_clock(Duration::from_millis(100));
        driver.draw_frame();
        assert!(
            is_tooltip_visible("btn-b"),
            "B must become visible after its full 300ms delay"
        );
        let painted = painted_tooltip().expect("painted tooltip for B");
        assert_eq!(painted.target_id, "btn-b");
        assert_eq!(painted.text, "Beta action");

        driver.pointer_hover(point(px(10.0), px(10.0)));

        driver.pointer_hover(bounds_a.center());
        assert!(is_tooltip_pending("btn-a"));

        let mut empty_root = Node::container();
        empty_root.id = Some("empty-root".into());
        *mounted.lock().expect("mounted lock") = empty_root;
        driver.draw_frame();

        driver.advance_clock(Duration::from_millis(500));
        driver.draw_frame();
        assert!(
            !is_tooltip_visible("btn-a"),
            "removed target must never show tooltip"
        );
        assert!(painted_tooltip().is_none());

        driver.pointer_hover(bounds_disabled.center());
        assert!(
            !is_tooltip_pending("btn-disabled"),
            "disabled target must not start tooltip timer"
        );
        driver.advance_clock(TOOLTIP_DELAY);
        driver.draw_frame();
        assert!(!is_tooltip_visible("btn-disabled"));

        driver.pointer_hover(bounds_empty.center());
        assert!(
            !is_tooltip_pending("btn-empty"),
            "empty tooltip text must not start tooltip timer"
        );
        driver.advance_clock(TOOLTIP_DELAY);
        driver.draw_frame();
        assert!(!is_tooltip_visible("btn-empty"));
    });
}

/// g16.066. Overlapping two-window isolation: both windows stay alive.
/// A frame, timer, or dismiss in B must not cancel or paint A's tooltip.
#[test]
fn gpui_node_tooltip_overlapping_two_window_isolation() {
    use poodle_gpui_node_backend::{
        is_tooltip_pending, is_tooltip_visible, painted_tooltip_for, TOOLTIP_DELAY,
    };

    fn tooltip_button(id: &str, label: &str, tooltip: &str) -> Node {
        let mut btn = Node::button(label);
        btn.id = Some(id.into());
        btn.tooltip = Some(tooltip.into());
        btn.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        btn.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        btn
    }

    run_headless(|cx| {
        let mut cx_b = cx.clone();
        let mounted_a = Arc::new(Mutex::new(tooltip_button(
            "win1-btn",
            "Window 1 Button",
            "Window 1 Tooltip",
        )));
        let mounted_b = Arc::new(Mutex::new(tooltip_button(
            "win2-btn",
            "Window 2 Button",
            "Window 2 Tooltip",
        )));
        let mut driver_a = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted_a), 400.0, 300.0);
        let mut driver_b =
            HeadlessDriver::new_in_box(&mut cx_b, Arc::clone(&mounted_b), 400.0, 300.0);

        let handle_a = driver_a.with_window(|w, _cx| w.window_handle());
        let handle_b = driver_b.with_window(|w, _cx| w.window_handle());
        assert!(
            handle_a != handle_b,
            "overlapping windows must have distinct handles"
        );

        driver_a.draw_frame();
        let center_a = poodle_gpui_node_backend::bounds_for("win1-btn")
            .expect("bounds for win1-btn")
            .center();
        driver_a.pointer_hover(center_a);
        assert!(
            is_tooltip_pending("win1-btn"),
            "window A must start its own pending timer"
        );

        driver_b.draw_frame();
        let center_b = poodle_gpui_node_backend::bounds_for("win2-btn")
            .expect("bounds for win2-btn")
            .center();
        driver_b.pointer_hover(center_b);
        assert!(
            is_tooltip_pending("win1-btn"),
            "window B's frame must not sweep window A's pending timer"
        );
        assert!(
            is_tooltip_pending("win2-btn"),
            "window B must start its own pending timer while A is still alive"
        );

        driver_a.advance_clock(TOOLTIP_DELAY);
        driver_a.draw_frame();
        driver_b.draw_frame();

        let painted_a = painted_tooltip_for(handle_a)
            .expect("window A tooltip must survive window B's overlapping paint");
        let painted_b = painted_tooltip_for(handle_b).expect("window B painted tooltip");
        assert_eq!(painted_a.target_id, "win1-btn");
        assert_eq!(painted_a.text, "Window 1 Tooltip");
        assert_eq!(painted_b.target_id, "win2-btn");
        assert_eq!(painted_b.text, "Window 2 Tooltip");

        driver_b.pointer_hover(point(px(10.0), px(10.0)));
        assert!(!is_tooltip_visible("win2-btn"), "dismissing B must hide B");
        driver_a.draw_frame();
        let painted_a_after_b_dismiss = painted_tooltip_for(handle_a)
            .expect("dismissing B must not hide A's visible tooltip");
        assert_eq!(painted_a_after_b_dismiss.text, "Window 1 Tooltip");
        assert!(
            painted_tooltip_for(handle_b).is_none(),
            "B's tooltip must stay gone after A paints"
        );
    });
}

/// g16.066. Production window teardown clears pending and visible tooltip
/// state and blocks late paint. The path is `Window::remove_window`, not
/// `reset_focus_registry`.
#[test]
fn gpui_node_tooltip_window_teardown_clears_pending_visible_and_blocks_late_paint() {
    use poodle_gpui_node_backend::{
        is_tooltip_pending, is_tooltip_visible, painted_tooltip_for, tooltip_runtime_owns_window,
        TOOLTIP_DELAY,
    };

    fn tooltip_button(id: &str, label: &str, tooltip: &str) -> Node {
        let mut btn = Node::button(label);
        btn.id = Some(id.into());
        btn.tooltip = Some(tooltip.into());
        btn.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        btn.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        btn
    }

    run_headless(|cx| {
        let mut cx_live = cx.clone();
        let mut cx_witness = cx.clone();

        let mounted_pending = Arc::new(Mutex::new(tooltip_button(
            "pending-btn",
            "Pending",
            "Pending Tooltip",
        )));
        let mounted_live = Arc::new(Mutex::new(tooltip_button(
            "live-btn",
            "Live",
            "Live Tooltip",
        )));
        let mounted_witness = Arc::new(Mutex::new(tooltip_button(
            "witness-btn",
            "Witness",
            "Witness Tooltip",
        )));

        let mut driver_pending =
            HeadlessDriver::new_in_box(cx, Arc::clone(&mounted_pending), 400.0, 300.0);
        let mut driver_live =
            HeadlessDriver::new_in_box(&mut cx_live, Arc::clone(&mounted_live), 400.0, 300.0);
        let handle_pending = driver_pending.with_window(|w, _cx| w.window_handle());
        let handle_live = driver_live.with_window(|w, _cx| w.window_handle());

        driver_pending.draw_frame();
        let pending_center = poodle_gpui_node_backend::bounds_for("pending-btn")
            .expect("bounds for pending-btn")
            .center();
        driver_pending.pointer_hover(pending_center);
        assert!(is_tooltip_pending("pending-btn"));
        assert!(tooltip_runtime_owns_window(handle_pending));

        driver_pending.close_window();
        assert!(
            !is_tooltip_pending("pending-btn"),
            "pending tooltip must die on window close"
        );
        assert!(
            !tooltip_runtime_owns_window(handle_pending),
            "closed window must not retain tooltip runtime state"
        );
        assert!(painted_tooltip_for(handle_pending).is_none());

        driver_pending.advance_clock(TOOLTIP_DELAY + std::time::Duration::from_millis(200));
        driver_live.draw_frame();
        assert!(
            !is_tooltip_visible("pending-btn"),
            "closed pending tooltip must not paint after its timer would have fired"
        );
        assert!(painted_tooltip_for(handle_pending).is_none());

        driver_live.draw_frame();
        let live_center = poodle_gpui_node_backend::bounds_for("live-btn")
            .expect("bounds for live-btn")
            .center();
        driver_live.pointer_hover(live_center);
        driver_live.advance_clock(TOOLTIP_DELAY);
        driver_live.draw_frame();
        let painted_live = painted_tooltip_for(handle_live).expect("live window visible tooltip");
        assert_eq!(painted_live.text, "Live Tooltip");
        assert!(tooltip_runtime_owns_window(handle_live));

        driver_live.close_window();
        assert!(
            !is_tooltip_visible("live-btn"),
            "visible tooltip must die on window close"
        );
        assert!(!tooltip_runtime_owns_window(handle_live));
        assert!(painted_tooltip_for(handle_live).is_none());

        let mut driver_witness =
            HeadlessDriver::new_in_box(&mut cx_witness, Arc::clone(&mounted_witness), 400.0, 300.0);
        driver_witness.draw_frame();
        driver_witness.advance_clock(TOOLTIP_DELAY);
        driver_witness.draw_frame();
        assert!(
            painted_tooltip_for(handle_live).is_none(),
            "a later window's frames must not resurrect the torn-down visible tooltip"
        );
        assert!(
            painted_tooltip_for(handle_pending).is_none(),
            "a later window's frames must not resurrect the torn-down pending tooltip"
        );
    });
}

/// g16.066. Repeated production create/close must return tooltip runtime and
/// teardown-binding counts to baseline. `reset_focus_registry` is not this path.
#[test]
fn gpui_node_tooltip_teardown_bindings_retire_across_repeated_close() {
    use poodle_gpui_node_backend::{
        begin_probe_capture, is_tooltip_pending, take_probe_capture, tooltip_runtime_owns_window,
        tooltip_runtime_window_count, tooltip_teardown_binding_count,
    };

    fn tooltip_button(id: &str, label: &str, tooltip: &str) -> Node {
        let mut btn = Node::button(label);
        btn.id = Some(id.into());
        btn.tooltip = Some(tooltip.into());
        btn.style.descriptor.layout.width = LayoutSizing::Fixed(120.0);
        btn.style.descriptor.layout.height = LayoutSizing::Fixed(40.0);
        btn
    }

    run_headless(|cx| {
        let baseline_bindings = tooltip_teardown_binding_count();
        let baseline_runtime = tooltip_runtime_window_count();
        assert_eq!(baseline_bindings, 0, "test start must not inherit bindings");
        assert_eq!(baseline_runtime, 0, "test start must not inherit runtime");

        for cycle in 0..3 {
            let mut cx_cycle = cx.clone();
            let id = format!("cycle-{cycle}-btn");
            let mounted = Arc::new(Mutex::new(tooltip_button(&id, "Cycle", "Cycle Tooltip")));
            let mut driver =
                HeadlessDriver::new_in_box(&mut cx_cycle, Arc::clone(&mounted), 400.0, 300.0);
            let handle = driver.with_window(|w, _cx| w.window_handle());

            driver.draw_frame();
            let center = poodle_gpui_node_backend::bounds_for(&id)
                .expect("cycle button bounds")
                .center();
            driver.pointer_hover(center);
            assert!(
                is_tooltip_pending(&id),
                "cycle {cycle} must start a pending tooltip"
            );
            assert_eq!(
                tooltip_teardown_binding_count(),
                baseline_bindings + 1,
                "cycle {cycle} must add exactly one close binding"
            );
            assert_eq!(
                tooltip_runtime_window_count(),
                baseline_runtime + 1,
                "cycle {cycle} must own exactly one tooltip runtime"
            );

            begin_probe_capture();
            driver.close_window();
            let channels = take_probe_capture();
            assert!(
                channels.contains(&"tooltip.lifecycle.teardown"),
                "cycle {cycle} close must emit teardown: {channels:?}"
            );

            assert_eq!(
                tooltip_teardown_binding_count(),
                baseline_bindings,
                "cycle {cycle} close must retire its binding"
            );
            assert_eq!(
                tooltip_runtime_window_count(),
                baseline_runtime,
                "cycle {cycle} close must drop tooltip runtime"
            );
            assert!(
                !tooltip_runtime_owns_window(handle),
                "closed cycle {cycle} must not retain runtime"
            );
        }
    });
}

/// g16.066. Tooltip probe channels receipt: verify lifecycle probe emissions.
#[test]
fn gpui_node_tooltip_probe_channels() {
    use poodle_gpui_node_backend::{begin_probe_capture, take_probe_capture, TOOLTIP_DELAY};

    run_headless(|cx| {
        let mut btn = Node::button("Probe Target");
        btn.id = Some("probe-target".into());
        btn.tooltip = Some("Probe Tooltip".into());
        btn.style.descriptor.layout.width = LayoutSizing::Fixed(100.0);
        btn.style.descriptor.layout.height = LayoutSizing::Fixed(36.0);

        let mounted = Arc::new(Mutex::new(btn));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&mounted), 400.0, 300.0);
        driver.draw_frame();

        begin_probe_capture();

        let center = poodle_gpui_node_backend::bounds_for("probe-target")
            .expect("bounds for probe-target")
            .center();

        driver.pointer_hover(center);
        driver.advance_clock(TOOLTIP_DELAY);
        driver.draw_frame();

        let channels = take_probe_capture();
        assert!(
            channels.contains(&"tooltip.projection.received"),
            "must emit tooltip.projection.received: {channels:?}"
        );
        assert!(
            channels.contains(&"tooltip.lifecycle.pending"),
            "must emit tooltip.lifecycle.pending: {channels:?}"
        );
        assert!(
            channels.contains(&"tooltip.lifecycle.shown"),
            "must emit tooltip.lifecycle.shown: {channels:?}"
        );

        begin_probe_capture();
        driver.pointer_hover(point(px(10.0), px(10.0)));
        let channels = take_probe_capture();
        assert!(
            channels.contains(&"tooltip.lifecycle.hidden"),
            "must emit tooltip.lifecycle.hidden on leave: {channels:?}"
        );
    });
}
