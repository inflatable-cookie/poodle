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
use gpui::{point, px, Pixels, Point, TestAppContext};
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{
    ColorValue, LayoutDirection, LayoutSizing, Node, NodeDropEvent, NodeKind, NodeRole,
};
use poodle_render::{
    ui_presentation_provider, RadioGroupHandlers, RenderContext, SliderHandlers, TabsHandlers,
    ToggleGroupHandlers,
};
use poodle_specs::{
    AgentTranscriptSpec, ControlDensity, ControlSize, Orientation, PopoverSpec, RangeSliderSpec,
    SliderSpec, TabActivationMode, TabDefinition, TabsSpec, UiPresentationProviderSpec,
};

#[path = "../src/headless_driver.rs"]
mod headless_driver;

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

fn payload_lifecycle_tree(trace: &Arc<Mutex<Vec<String>>>, disabled_source: bool) -> Node {
    let push = |label: &'static str| {
        let trace = Arc::clone(trace);
        Arc::new(move |payload: &str| {
            trace
                .lock()
                .expect("trace lock")
                .push(format!("{label}:{payload}"));
        }) as Arc<dyn Fn(&str) + Send + Sync>
    };
    let push_drop = |label: &'static str| {
        let trace = Arc::clone(trace);
        Arc::new(move |event: &NodeDropEvent| {
            trace
                .lock()
                .expect("trace lock")
                .push(format!("{label}:{}:{:?}", event.payload, event.edge));
        }) as Arc<dyn Fn(&NodeDropEvent) + Send + Sync>
    };
    let push_leave = |label: &'static str| {
        let trace = Arc::clone(trace);
        Arc::new(move || {
            trace.lock().expect("trace lock").push(label.to_owned());
        }) as Arc<dyn Fn() + Send + Sync>
    };

    let mut source = payload_paint_box("payload-source");
    source.interaction.disabled = disabled_source;
    source.interaction.drag_payload = Some("alpha".to_owned());
    source.interaction.on_drag_start = Some(push("start"));
    source.interaction.on_drag_end = Some(push("end"));

    let mut zone_a = payload_paint_box("payload-zone-a");
    zone_a.interaction.drop_zone = true;
    zone_a.interaction.on_drop_hover = Some(push_drop("hover-a"));
    zone_a.interaction.on_drop_leave = Some(push_leave("leave-a"));
    zone_a.interaction.on_drop = Some(push_drop("drop-a"));

    let mut zone_b = payload_paint_box("payload-zone-b");
    zone_b.interaction.drop_zone = true;
    zone_b.interaction.on_drop_hover = Some(push_drop("hover-b"));
    zone_b.interaction.on_drop_leave = Some(push_leave("leave-b"));
    zone_b.interaction.on_drop = Some(push_drop("drop-b"));

    let mut row = Node::container();
    row.id = Some("payload-row".to_owned());
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.width = LayoutSizing::Fixed(240.0);
    row.style.descriptor.layout.height = LayoutSizing::Fixed(80.0);
    row.child(source).child(zone_a).child(zone_b)
}

/// g16.006. The reusable payload/drop seam reports one start after GPUI's
/// drag threshold, hit-tested hover/leave, a drop with the retained edge,
/// and exactly one end — including outside release and Escape — on stock
/// GPUI. Direct handler invocation is not evidence.
#[test]
fn payload_lifecycle_hit_tests_retains_edge_and_ends_once() {
    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(payload_lifecycle_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("payload-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        assert_eq!(
            trace.lock().expect("trace lock").as_slice(),
            ["start:alpha"],
            "start fires once after the runtime drag threshold"
        );

        driver.pointer_drag(payload_frac("payload-zone-a", 0.5, 0.1));
        driver.pointer_drag(payload_frac("payload-zone-b", 0.5, 0.5));
        let events = trace.lock().expect("trace lock").clone();
        assert!(
            events
                .iter()
                .any(|event| event.starts_with("hover-a:alpha:Before")),
            "the hit zone receives the before-band edge: {events:?}"
        );
        assert!(
            events.contains(&"leave-a".to_owned()),
            "leaving zone A fires leave: {events:?}"
        );
        assert!(
            events
                .iter()
                .any(|event| event.starts_with("hover-b:alpha:")),
            "the new hit zone receives hover: {events:?}"
        );
        assert!(
            !events
                .iter()
                .any(|event| event.starts_with("hover-b:") && event.contains("hover-a")),
            "capture-wide moves must not deliver hover to a miss: {events:?}"
        );
        let hover_b_count = events
            .iter()
            .filter(|event| event.starts_with("hover-b:"))
            .count();
        let hover_a_after_leave = events
            .iter()
            .skip_while(|event| *event != "leave-a")
            .filter(|event| event.starts_with("hover-a:"))
            .count();
        assert!(hover_b_count >= 1);
        assert_eq!(
            hover_a_after_leave, 0,
            "zone A must not keep receiving hover after leave"
        );
    });

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(payload_lifecycle_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("payload-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("payload-zone-a", 0.5, 0.1));
        driver.pointer_release(payload_frac("payload-zone-a", 0.5, 0.1));

        let events = trace.lock().expect("trace lock").clone();
        let start = events.iter().position(|event| event == "start:alpha");
        let drop = events
            .iter()
            .position(|event| event == "drop-a:alpha:Before");
        let end = events.iter().position(|event| event == "end:alpha");
        assert!(start.is_some(), "start: {events:?}");
        assert_eq!(
            drop.map(|index| events[index].as_str()),
            Some("drop-a:alpha:Before"),
            "drop reuses the last hover edge, not Inside: {events:?}"
        );
        assert!(end.is_some(), "end: {events:?}");
        assert!(
            start < drop && drop < end,
            "successful drop ordering is start → drop → end: {events:?}"
        );
        assert_eq!(
            events.iter().filter(|event| *event == "end:alpha").count(),
            1,
            "end fires exactly once on drop: {events:?}"
        );
        driver.pointer_release(payload_frac("payload-zone-a", 0.5, 0.5));
        assert_eq!(
            trace
                .lock()
                .expect("trace lock")
                .iter()
                .filter(|event| *event == "end:alpha")
                .count(),
            1,
            "a later mouse-up must not emit a second end"
        );
    });

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(payload_lifecycle_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("payload-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("payload-zone-a", 0.5, 0.5));
        driver.pointer_drag(point(px(8.0), px(8.0)));
        driver.pointer_release(point(px(8.0), px(8.0)));

        let events = trace.lock().expect("trace lock").clone();
        assert!(events.contains(&"start:alpha".to_owned()), "{events:?}");
        assert!(events.contains(&"leave-a".to_owned()), "{events:?}");
        assert!(
            !events.iter().any(|event| event.starts_with("drop-")),
            "outside release is cancellation, not drop: {events:?}"
        );
        assert_eq!(
            events.iter().filter(|event| *event == "end:alpha").count(),
            1,
            "outside release ends once: {events:?}"
        );
    });

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(payload_lifecycle_tree(&trace, false)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("payload-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("payload-zone-b", 0.5, 0.9));
        driver.dispatch_key("escape");

        let events = trace.lock().expect("trace lock").clone();
        assert!(events.contains(&"start:alpha".to_owned()), "{events:?}");
        assert!(events.contains(&"leave-b".to_owned()), "{events:?}");
        assert!(
            !events.iter().any(|event| event.starts_with("drop-")),
            "Escape is cancellation: {events:?}"
        );
        assert_eq!(
            events.iter().filter(|event| *event == "end:alpha").count(),
            1,
            "Escape ends once: {events:?}"
        );
        driver.dispatch_key("escape");
        assert_eq!(
            trace
                .lock()
                .expect("trace lock")
                .iter()
                .filter(|event| *event == "end:alpha")
                .count(),
            1,
            "a second Escape must not emit another end"
        );
    });

    run_headless(|cx| {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let node = Arc::new(Mutex::new(payload_lifecycle_tree(&trace, true)));
        let mut driver = HeadlessDriver::new_in_box(cx, Arc::clone(&node), 280.0, 100.0);
        driver.draw_frame();

        let source = payload_frac("payload-source", 0.5, 0.5);
        driver.pointer_press(source);
        driver.pointer_drag(point(px(f32::from(source.x) + 4.0), source.y));
        driver.pointer_drag(payload_frac("payload-zone-a", 0.5, 0.5));
        driver.pointer_release(payload_frac("payload-zone-a", 0.5, 0.5));
        assert!(
            trace.lock().expect("trace lock").is_empty(),
            "a disabled source is inert"
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
        driver.pointer_drag(payload_frac("tabs:drag:tab:one", 0.5, 0.5));
        assert_eq!(live.lock().unwrap().drop.as_deref(), Some("one"));
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
            let mut node = poodle_render::licence_activation(
                &LicenceActivationSpec::new()
                    .with_mode(LicenceActivationMode::Account)
                    .with_machine_label(Some(label.to_string()))
                    .with_machine_label_editing(editing),
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
            None,
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
    editing: Mutex<bool>,
    log: EventLog,
}

fn label_routing_tree(host: &Arc<LabelRouting>, mounted: &Arc<Mutex<Node>>) -> Node {
    let provider = theme();
    let ctx = RenderContext::new(&provider);
    let editing = *host.editing.lock().expect("editing");
    let shown = if editing {
        host.draft.lock().expect("draft").clone()
    } else {
        host.value.lock().expect("value").clone()
    };
    let spec = poodle_specs::EditableLabelSpec::new()
        .with_value(&shown)
        .with_editing(editing)
        .with_aria_label("track name");
    let id = host.id.clone();

    let change_host = Arc::clone(host);
    let change_mount = Arc::clone(mounted);
    let commit_host = Arc::clone(host);
    let commit_mount = Arc::clone(mounted);
    let cancel_host = Arc::clone(host);
    let cancel_mount = Arc::clone(mounted);
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
            on_commit: Some(Arc::new(move |next: &str| {
                // The shared machine's guard, restated by the host: a commit
                // only lands while the field is editing, so a blur that
                // follows Escape or a completed Enter cannot emit a second.
                if !*commit_host.editing.lock().expect("editing") {
                    return;
                }
                *commit_host.editing.lock().expect("editing") = false;
                *commit_host.value.lock().expect("value") = next.to_owned();
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
        editing: Mutex::new(true),
        log: event_log(),
    })
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

/// g16.008. EditableLabel still commits when Tab leaves it — but for the
/// reason its contract gives: Tab moves focus, and the blur commits the draft
/// once. Enter is the direct commit, Escape cancels, and neither leaves a
/// second commit behind for the blur to fire.
///
/// Deliberately not claimed: activation modes, select-on-focus, the display
/// mode's own affordances, or focus restoration after a commit.
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
            vec!["label/commit:Kicks"],
            "Enter commits directly, exactly once"
        );
        assert_eq!(*host.value.lock().expect("value"), "Kicks");
        assert!(!*host.editing.lock().expect("editing"));
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
        assert_eq!(take_events(&host.log), vec!["label/cancel"]);
        assert_eq!(
            *host.value.lock().expect("value"),
            "Kick",
            "a cancelled edit never reaches the committed value"
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

        // Further frames cannot produce a second commit: the edit is over.
        driver.draw_frame();
        driver.draw_frame();
        assert_eq!(take_events(&host.log), Vec::<String>::new());
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
        CollapsibleHandlers, COLLAPSIBLE_CONTENT_SEMANTIC_ID, COLLAPSIBLE_TRIGGER_SEMANTIC_ID,
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

        let closed_trigger =
            collapsible_trigger_focus_id(Some("closed"));
        let aria_trigger = collapsible_trigger_focus_id(Some("aria"));
        let disabled_trigger = collapsible_trigger_focus_id(Some("disabled"));

        {
            let trigger = target(&root, &closed_trigger);
            assert_eq!(trigger.a11y.role, Some(NodeRole::Button));
            assert_eq!(trigger.a11y.label.as_deref(), Some("Project settings"));
            assert_eq!(trigger.a11y.expanded, Some(false));
            assert_eq!(
                trigger.a11y.controls.as_deref(),
                Some(collapsible_content_focus_id(Some("closed")).as_str())
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
        let trigger = collapsible_trigger_focus_id(Some("controlled"));
        {
            let node = mounted.lock().expect("mount lock");
            let trigger_node = node
                .find(&|n| n.runtime_id.as_deref() == Some(trigger.as_str()))
                .expect("trigger");
            assert_eq!(trigger_node.a11y.expanded, Some(false));
            assert!(node
                .find(&|n| n.id.as_deref() == Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID))
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
            .find(&|n| n.id.as_deref() == Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID))
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
        let trigger = collapsible_trigger_focus_id(Some("seeded"));
        {
            let node = mounted.lock().expect("mount lock");
            let trigger_node = node
                .find(&|n| n.runtime_id.as_deref() == Some(trigger.as_str()))
                .expect("trigger");
            assert_eq!(trigger_node.a11y.expanded, Some(true));
            assert!(node
                .find(&|n| n.id.as_deref() == Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID))
                .is_some());
            let region = node
                .find(&|n| n.id.as_deref() == Some(COLLAPSIBLE_CONTENT_SEMANTIC_ID))
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
        let left = collapsible_trigger_focus_id(Some("left"));
        let right = collapsible_trigger_focus_id(Some("right"));
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
