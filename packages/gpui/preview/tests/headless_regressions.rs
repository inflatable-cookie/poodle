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
