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
use poodle_specs::{AgentTranscriptSpec, PopoverSpec, RangeSliderSpec};

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
            let content = poodle_render::agent_transcript(
                &spec,
                &build_theme,
                poodle_render::AgentTranscriptHandlers::default(),
            );
            let mut jump = poodle_render::agent_transcript::agent_transcript_jump(
                &spec,
                &build_theme,
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
            &theme(),
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
                &theme(),
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
        let mut checked = poodle_render::code_input_with_handlers(
            &CodeInputSpec::new()
                .with_length(6)
                .with_value("123456")
                .with_completion_result(CodeInputCompletion::Passed("123456".to_string())),
            &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
                &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
                &theme(),
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
                &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
                &theme(),
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
            &theme(),
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
                &theme(),
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
            &theme(),
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
                &theme(),
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
            &theme(),
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
            &theme(),
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
                &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
            &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
                &theme(),
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
    let default = empty_state(&EmptyStateSpec::new("No projects yet"), &theme);
    let compact = empty_state(
        &EmptyStateSpec::new("No projects yet").with_size(EmptyStateSize::Compact),
        &theme,
    );

    let default_title = title_text_size(&default).expect("default title");
    let compact_title = title_text_size(&compact).expect("compact title");
    assert!(compact_title < default_title);

    let default_icon = icon_container_side(&default).expect("default icon box");
    let compact_icon = icon_container_side(&compact).expect("compact icon box");
    assert!(compact_icon < default_icon);
}
