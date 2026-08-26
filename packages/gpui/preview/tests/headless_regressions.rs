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
use gpui::TestAppContext;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::{ui_presentation_provider, RenderContext};
use poodle_specs::{
    AgentTranscriptSpec, ControlDensity, ControlSize, PopoverSpec, RangeSliderSpec,
    UiPresentationProviderSpec,
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

fn radio_option_id(scope: &str, value: &str) -> String {
    format!("radio:{scope}:option:{value}")
}

fn radio_option_selected(node: &Node, scope: &str, value: &str) -> bool {
    let id = radio_option_id(scope, value);
    node.find(&|n| n.id.as_deref() == Some(id.as_str()))
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

/// RadioGroup exclusive selection, wrap, disabled-option skip, and disabled
/// group inertia through mounted pointer and directional keys. The host
/// rebuilds from the emitted value.
#[test]
fn radio_group_exclusive_focus_and_disabled_paths_through_mounted_input() {
    use poodle_specs::{Orientation, RadioGroupSpec};

    run_headless(|cx| {
        fn build(
            value: &str,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<String>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let mut node = poodle_render::radio_group(
                &RadioGroupSpec::new(selection_radio_options())
                    .with_name("plan")
                    .with_orientation(Orientation::Horizontal)
                    .with_value(value),
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
        *mounted.lock().unwrap() = build("free", Arc::clone(&mounted), Arc::clone(&payloads));
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));

        let free = radio_option_id("plan", "free");
        let pro = radio_option_id("plan", "pro");
        let enterprise = radio_option_id("plan", "enterprise");
        driver.wait_for_focus_handle(&free);
        driver.pointer_activate_id(&enterprise);
        assert_eq!(payloads.lock().unwrap().as_slice(), ["enterprise"]);
        assert!(radio_option_selected(
            &mounted.lock().unwrap(),
            "plan",
            "enterprise"
        ));
        assert!(!radio_option_selected(
            &mounted.lock().unwrap(),
            "plan",
            "free"
        ));

        driver.pointer_activate_id(&enterprise);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["enterprise"],
            "same-value selection is inert"
        );

        driver.pointer_activate_id(&pro);
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["enterprise"],
            "a disabled option emits nothing"
        );

        driver.wait_for_focus_handle(&enterprise);
        driver.focus_element(&enterprise);
        driver.dispatch_key_raw("right");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["enterprise", "free"],
            "directional movement wraps and skips the disabled option"
        );
        assert_eq!(poodle_gpui_node_backend::focus_state_for(&free), Some(true));
        driver.dispatch_key_raw("left");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            ["enterprise", "free", "enterprise"]
        );
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = RadioGroupSpec {
            is_disabled: true,
            ..RadioGroupSpec::new(selection_radio_options())
                .with_name("disabled-plan")
                .with_orientation(Orientation::Horizontal)
                .with_value("free")
        };
        let mut node = poodle_render::radio_group(
            &spec,
            &RenderContext::new(&theme()),
            Some(Arc::new(move |next: &str| {
                sink.lock().unwrap().push(next.to_string())
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.pointer_activate_id(&radio_option_id("disabled-plan", "enterprise"));
        assert!(payloads.lock().unwrap().is_empty());
        assert!(poodle_gpui_node_backend::focus_handle_for(&radio_option_id(
            "disabled-plan",
            "free"
        ))
        .is_none());
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

fn toggle_item_on(node: &Node, value: &str) -> bool {
    let id = format!("toggle:{value}");
    node.find(&|n| n.id.as_deref() == Some(id.as_str()))
        .and_then(|n| n.a11y.toggled)
        .map(|toggled| toggled == poodle_node::NodeToggled::True)
        .unwrap_or(false)
}

fn selection_toggle_options() -> Vec<poodle_specs::ToggleGroupOption> {
    vec![
        poodle_specs::ToggleGroupOption::new("grid", "Grid"),
        poodle_specs::ToggleGroupOption::new("list", "List"),
        poodle_specs::ToggleGroupOption::new("board", "Board"),
    ]
}

/// ToggleGroup single, deactivating-single, multiple, and disabled payload
/// semantics through mounted input. Native emit is the activated option; the
/// host applies `next_value_on_toggle` and rebuilds the spec.
#[test]
fn toggle_group_single_multiple_and_disabled_payloads_through_mounted_input() {
    use poodle_specs::{ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

    run_headless(|cx| {
        fn build(
            spec: ToggleGroupSpec,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<Vec<String>>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let current = spec.clone();
            let mut node = poodle_render::toggle_group(
                &spec,
                &RenderContext::new(&theme()),
                Some(Arc::new(move |value: &str| {
                    let next = current.next_value_on_toggle(value);
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() = build(
                        current.clone().with_value(next),
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
            ToggleGroupSpec::new(selection_toggle_options()).with_value(vec!["grid".into()]),
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("toggle:list");
        driver.pointer_activate_id("toggle:list");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [vec!["list".to_string()]]
        );
        assert!(toggle_item_on(&mounted.lock().unwrap(), "list"));
        assert!(!toggle_item_on(&mounted.lock().unwrap(), "grid"));

        driver.pointer_activate_id("toggle:list");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [vec!["list".to_string()], vec!["list".to_string()]],
            "re-selection emits the unchanged value"
        );
    });

    run_headless(|cx| {
        fn build(
            spec: ToggleGroupSpec,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<Option<String>>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let current = spec.clone();
            let mut node = poodle_render::toggle_group(
                &spec,
                &RenderContext::new(&theme()),
                Some(Arc::new(move |value: &str| {
                    let next = current.next_value_on_toggle(value);
                    sink.lock().unwrap().push(next.first().cloned());
                    *mount.lock().unwrap() = build(
                        current.clone().with_value(next),
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
            ToggleGroupSpec::new(selection_toggle_options())
                .with_value(vec!["grid".into()])
                .with_allow_deactivation(true),
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("toggle:grid");
        driver.pointer_activate_id("toggle:grid");
        assert_eq!(payloads.lock().unwrap().as_slice(), [None]);
        assert!(!toggle_item_on(&mounted.lock().unwrap(), "grid"));
    });

    run_headless(|cx| {
        fn build(
            spec: ToggleGroupSpec,
            mounted: Arc<Mutex<Node>>,
            payloads: Arc<Mutex<Vec<Vec<String>>>>,
        ) -> Node {
            let mount = Arc::clone(&mounted);
            let sink = Arc::clone(&payloads);
            let current = spec.clone();
            let mut node = poodle_render::toggle_group(
                &spec,
                &RenderContext::new(&theme()),
                Some(Arc::new(move |value: &str| {
                    let next = current.next_value_on_toggle(value);
                    sink.lock().unwrap().push(next.clone());
                    *mount.lock().unwrap() = build(
                        current.clone().with_value(next),
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
            ToggleGroupSpec::new(selection_toggle_options())
                .with_selection_mode(ToggleGroupSelectionMode::Multiple)
                .with_value(vec!["grid".into()]),
            Arc::clone(&mounted),
            Arc::clone(&payloads),
        );
        let mut driver = HeadlessDriver::new(cx, Arc::clone(&mounted));
        driver.wait_for_focus_handle("toggle:board");
        driver.pointer_activate_id("toggle:board");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [vec!["grid".to_string(), "board".to_string()]]
        );
        driver.pointer_activate_id("toggle:grid");
        assert_eq!(
            payloads.lock().unwrap().as_slice(),
            [
                vec!["grid".to_string(), "board".to_string()],
                vec!["board".to_string()]
            ]
        );
        assert!(toggle_item_on(&mounted.lock().unwrap(), "board"));
        assert!(!toggle_item_on(&mounted.lock().unwrap(), "grid"));
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let mut node = poodle_render::toggle_group(
            &ToggleGroupSpec::new(vec![
                ToggleGroupOption::new("grid", "Grid"),
                ToggleGroupOption::new("blocked", "Blocked").with_disabled(true),
            ])
            .with_value(vec!["grid".into()]),
            &RenderContext::new(&theme()),
            Some(Arc::new(move |value: &str| {
                sink.lock().unwrap().push(value.to_string())
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.pointer_activate_id("toggle:blocked");
        assert!(payloads.lock().unwrap().is_empty());
    });

    run_headless(|cx| {
        let payloads = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&payloads);
        let spec = ToggleGroupSpec::new(vec![
            poodle_specs::ToggleGroupOption::new("locked-grid", "Grid"),
            poodle_specs::ToggleGroupOption::new("locked-list", "List"),
        ])
        .with_disabled(true);
        let mut node = poodle_render::toggle_group(
            &spec,
            &RenderContext::new(&theme()),
            Some(Arc::new(move |value: &str| {
                sink.lock().unwrap().push(value.to_string())
            })),
        );
        node.id = Some(FIXTURE_ID.to_owned());
        let mut driver = HeadlessDriver::new(cx, Arc::new(Mutex::new(node)));
        driver.draw_frame();
        driver.pointer_activate_id("toggle:locked-list");
        assert!(payloads.lock().unwrap().is_empty());
    });
}
