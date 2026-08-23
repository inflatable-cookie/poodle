//! g15.052 focused-state evidence, on the g16.005 non-activating window
//! transport.
//!
//! One invocation renders one closed evidence scene through the production
//! path (`poodle_render` → `poodle_gpui_node_backend::to_gpui`) into one real
//! GPUI window opened with `focus: false`, moves REAL focus to the scene's
//! target control through the backend's focus registry (no pointer, no
//! activation), and writes the captured PNG plus a typed receipt. The receipt
//! carries the paint pass's own record of the ring (`painted_ring_for`); an
//! invocation where the ring never painted is a hard failure, never a green
//! skip.
//!
//! Element focus inside the capture window is not application focus: the
//! window is never made key by this process, and the run's own
//! frontmost-application samples ride on the receipt.
//!
//! This is point-in-time operator review evidence for the native focus-ring
//! channel, not a baseline: nothing reads these files back, and no fixture
//! inventory or comparison policy is involved. Scenes:
//!
//! - `button`: two bordered secondary Buttons, the right one focused — the
//!   contracted 2px ring at a 2px offset around the preserved 1px border.
//! - `stepper-trigger`: a borderless Stepper trigger focused — the same ring
//!   on a control with no resting border.
//! - `stepper-summary`: the collapsible summary focused — the contracted
//!   INSET (-2px) ring.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::{bail, Context as _, Result};
use gpui::{
    div, px, AnyElement, App, AppContext as _, Context, IntoElement, ParentElement, Render, Styled,
    Window,
};
use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{ButtonSpec, Orientation, StepStatus, StepperSpec, StepperStep};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::fixture_capture::{inter_fonts, FixtureAssets};
use crate::presentation_axes::ThemePreset;
use crate::publish_pair;
use crate::transport::{self, GPUI_SOURCE, GPUI_VERSION, TRANSPORT};

/// Versioned evidence receipt schema identity. `v2` is the windowed,
/// non-activating transport; `v1` claimed a fork-only offscreen readback.
const EVIDENCE_RECEIPT_SCHEMA: &str = "poodle.gpui-focus-evidence.v2";

/// The closed scene set.
const SCENES: &[&str] = &["button", "stepper-trigger", "stepper-summary"];

/// Parsed and validated focus-evidence command line.
pub struct FocusEvidenceArgs {
    pub scene: String,
    pub out_png: PathBuf,
    pub out_receipt: PathBuf,
}

/// The evidence scenes are a closed contract: exactly `--focus-evidence`,
/// `--out`, and `--receipt`, all required, every other flag rejected.
pub fn parse_args(argv: &[String]) -> Result<FocusEvidenceArgs> {
    const USAGE: &str = "usage: poodle-window-capture --focus-evidence \
<button|stepper-trigger|stepper-summary> --out <png> --receipt <json>";
    let mut scene: Option<String> = None;
    let mut out_png: Option<PathBuf> = None;
    let mut out_receipt: Option<PathBuf> = None;

    let mut i = 0;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .with_context(|| format!("missing value for {flag}\n{USAGE}"))?;
        i += 2;
        match flag {
            "--focus-evidence" => {
                if !SCENES.contains(&value.as_str()) {
                    bail!("unknown focus-evidence scene '{value}': expected one of {SCENES:?}");
                }
                scene = Some(value.to_string());
            }
            "--out" => out_png = Some(PathBuf::from(value)),
            "--receipt" => out_receipt = Some(PathBuf::from(value)),
            other => bail!("argument '{other}' is not accepted in focus-evidence mode\n{USAGE}"),
        }
    }

    Ok(FocusEvidenceArgs {
        scene: scene.with_context(|| format!("--focus-evidence is required\n{USAGE}"))?,
        out_png: out_png.with_context(|| format!("--out is required\n{USAGE}"))?,
        out_receipt: out_receipt.with_context(|| format!("--receipt is required\n{USAGE}"))?,
    })
}

/// What one scene renders and which element receives real focus.
struct EvidenceScene {
    node: Node,
    focus_id: &'static str,
    logical_width: f32,
    logical_height: f32,
}

fn build_scene(scene: &str, ctx: &poodle_render::RenderContext<'_>) -> EvidenceScene {
    match scene {
        "button" => {
            // Two bordered secondary Buttons at rest; the right one takes
            // focus. Distinct stamped ids make the focus target addressable
            // and keep the resting twin observable.
            let mut rest = poodle_render::button(
                &ButtonSpec::new().with_label("Save"),
                ctx,
                None,
            );
            rest.id = Some("evidence:button:rest".to_owned());
            let mut focused = poodle_render::button(
                &ButtonSpec::new().with_label("Save"),
                ctx,
                None,
            );
            focused.id = Some("evidence:button:focus".to_owned());
            let mut row = Node::container();
            row.style.descriptor.layout.direction = LayoutDirection::Row;
            row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            row.style.descriptor.layout.spacing.gap = 24.0;
            let row = row.child(rest).child(focused);
            EvidenceScene {
                node: row,
                focus_id: "evidence:button:focus",
                logical_width: 320.0,
                logical_height: 88.0,
            }
        }
        "stepper-trigger" | "stepper-summary" => {
            let spec = StepperSpec::new(vec![
                StepperStep::new("read", "Read").with_status(StepStatus::Complete),
                StepperStep::new("verify", "Verify").with_status(StepStatus::Complete),
                StepperStep::new("apply", "Apply"),
            ])
            .with_orientation(Orientation::Vertical)
            .with_collapsible(true)
            .with_collapsed(false)
            .with_show_rerun(true)
            .with_value("apply");
            let mut node =
                poodle_render::stepper(&spec, ctx, poodle_render::StepperHandlers::default());
            // Evidence-host presentation: a fixed width so the full-width
            // rows (and the summary's inset ring) read as they do in product.
            node.style.descriptor.layout.width = LayoutSizing::Fixed(320.0);
            EvidenceScene {
                node,
                focus_id: if scene == "stepper-summary" {
                    "poodle-stepper-summary"
                } else {
                    "poodle-stepper:trigger:read"
                },
                logical_width: 360.0,
                logical_height: 300.0,
            }
        }
        other => unreachable!("scene validated at parse time: {other}"),
    }
}

/// The scene root: the canvas-colored padded surface the evidence node sits
/// on. Same placement contract as the fixture scenes.
struct EvidenceRoot {
    node: Node,
    canvas: gpui::Hsla,
}

impl Render for EvidenceRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        poodle_gpui_node_backend::reset_element_ids();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&self.node);
        div()
            .size_full()
            .p(px(16.0))
            .flex()
            .items_start()
            .bg(self.canvas)
            .font_family("Inter")
            .child(element)
    }
}

#[derive(Serialize)]
struct PaintedRingEvidence {
    color: [f32; 4],
    width: f32,
    offset: f32,
    /// Outer edge of the painted ring: x, y, width, height (logical px).
    bounds: [f32; 4],
}

#[derive(Serialize)]
struct FocusEvidenceReceipt {
    schema: &'static str,
    scene: String,
    focused_element: String,
    backend_focused: bool,
    painted_ring: PaintedRingEvidence,
    gpui_source: &'static str,
    gpui_version: &'static str,
    transport: &'static str,
    platform: &'static str,
    theme: &'static str,
    logical_viewport: [f32; 2],
    scale: f32,
    png_sha256: String,
    foreground: transport::ForegroundEvidence,
}

pub fn run(args: &FocusEvidenceArgs) -> ! {
    match prepare(args) {
        Ok(scene) => transport::capture(scene),
        Err(error) => {
            eprintln!("poodle-window-capture: {error:#}");
            std::process::exit(1)
        }
    }
}

/// What the settled frame recorded, carried from the main thread to the
/// capture thread.
struct RingEvidence {
    backend_focused: bool,
    painted: PaintedRingEvidence,
}

fn prepare(args: &FocusEvidenceArgs) -> Result<transport::Scene<EvidenceRoot, FixtureAssets>> {
    let theme = ThemePreset::Eclipse.build_theme();
    let canvas = theme.resolve_color("color.background.canvas");
    let ctx = poodle_render::RenderContext::new(&theme);
    let scene = build_scene(&args.scene, &ctx);
    let focus_id = scene.focus_id;
    let logical_width = scene.logical_width;
    let logical_height = scene.logical_height;
    let node = scene.node;

    let evidence: Arc<Mutex<Option<RingEvidence>>> = Arc::new(Mutex::new(None));
    let frame_evidence = Arc::clone(&evidence);

    let scene_name = args.scene.clone();
    let out_png = args.out_png.clone();
    let out_receipt = args.out_receipt.clone();

    // The focus handle is created in the paint pass and attached from the
    // next build, so the scene waits for it rather than assuming a frame
    // count. `focused` records that focus was already requested.
    let mut focused = false;

    Ok(transport::Scene {
        logical_width,
        logical_height,
        assets: FixtureAssets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        },
        fonts: inter_fonts()?,
        build: Box::new(move |_window, cx: &mut App| {
            cx.new(|_| EvidenceRoot {
                node,
                canvas: poodle_gpui_node_backend::color(canvas),
            })
        }),
        on_frame: Box::new(move |window, _cx, _frame| {
            let Some(handle) = poodle_gpui_node_backend::focus_handle_for(focus_id) else {
                return Ok(transport::Settled::Wait);
            };
            if !focused {
                // Real focus through the backend registry — no pointer, and
                // no window or application activation anywhere.
                handle.focus(window);
                focused = true;
                return Ok(transport::Settled::Wait);
            }

            let backend_focused =
                poodle_gpui_node_backend::focus_state_for(focus_id).unwrap_or(false);
            if !backend_focused {
                return Ok(transport::Settled::Wait);
            }
            let Some(painted) = poodle_gpui_node_backend::painted_ring_for(focus_id) else {
                return Ok(transport::Settled::Wait);
            };

            *frame_evidence.lock().expect("ring slot") = Some(RingEvidence {
                backend_focused,
                painted: PaintedRingEvidence {
                    color: [
                        painted.ring.color.0,
                        painted.ring.color.1,
                        painted.ring.color.2,
                        painted.ring.color.3,
                    ],
                    width: painted.ring.width,
                    offset: painted.ring.offset,
                    bounds: painted.bounds,
                },
            });
            Ok(transport::Settled::Ready)
        }),
        finish: Box::new(move |facts: &transport::CaptureFacts| {
            let recorded = evidence
                .lock()
                .expect("ring slot")
                .take()
                .with_context(|| {
                    format!(
                        "the focus ring for '{focus_id}' never painted — capture would not be \
                         evidence"
                    )
                })?;
            if !recorded.backend_focused {
                bail!("the backend never reported '{focus_id}' focused");
            }
            let png_sha256 = format!("{:x}", Sha256::digest(&facts.png));

            let receipt = FocusEvidenceReceipt {
                schema: EVIDENCE_RECEIPT_SCHEMA,
                scene: scene_name.clone(),
                focused_element: focus_id.to_owned(),
                backend_focused: recorded.backend_focused,
                painted_ring: recorded.painted,
                gpui_source: GPUI_SOURCE,
                gpui_version: GPUI_VERSION,
                transport: TRANSPORT,
                platform: "macos",
                theme: ThemePreset::Eclipse.label(),
                logical_viewport: [logical_width, logical_height],
                scale: facts.scale,
                png_sha256,
                foreground: facts.foreground.clone(),
            };
            let receipt_json = serde_json::to_vec_pretty(&receipt)?;
            publish_pair(&out_png, &facts.png, &out_receipt, &receipt_json)?;

            eprintln!(
                "captured focus-evidence {} ({}x{} logical @ {}x) focused={} ring=({}px @{}px) \
                 sha256={}",
                scene_name,
                logical_width,
                logical_height,
                facts.scale,
                focus_id,
                receipt.painted_ring.width,
                receipt.painted_ring.offset,
                receipt.png_sha256
            );
            Ok(())
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    const VALID: &[&str] = &[
        "--focus-evidence", "button",
        "--out", "ring.png",
        "--receipt", "ring.json",
    ];

    #[test]
    fn a_valid_invocation_parses() {
        let args = parse_args(&argv(VALID)).expect("the canonical invocation parses");
        assert_eq!(args.scene, "button");
    }

    #[test]
    fn an_unknown_scene_is_rejected() {
        let mut v = argv(VALID);
        v[1] = "dialog".to_string();
        assert!(parse_args(&v).is_err(), "only the closed scene set parses");
    }

    #[test]
    fn every_flag_is_required_and_extras_are_rejected() {
        for flag in ["--focus-evidence", "--out", "--receipt"] {
            let mut v = argv(VALID);
            let i = v.iter().position(|a| a == flag).expect("flag present");
            v.drain(i..=i + 1);
            assert!(parse_args(&v).is_err(), "missing {flag} must fail");
        }
        let mut v = argv(VALID);
        v.push("--baseline".to_string());
        v.push("x".to_string());
        assert!(parse_args(&v).is_err(), "no extra flags in this mode");
    }
}
