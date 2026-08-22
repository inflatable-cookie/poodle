//! g15.052 — focused-state evidence mode for the offscreen capture target.
//!
//! One invocation renders one closed evidence scene through the production
//! path (`poodle_render` → `poodle_gpui_node_backend::to_gpui`) into GPUI's
//! `HeadlessAppContext`, moves REAL focus to the scene's target control
//! through the backend's focus registry (no pointer, no OS window), and
//! writes the rasterized PNG plus a typed receipt. The receipt carries the
//! paint pass's own record of the ring (`painted_ring_for`); an invocation
//! where the ring never painted is a hard failure, never a green skip.
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
use std::sync::Arc;

use anyhow::{bail, Context as _, Result};
use gpui::{
    div, px, size, AnyElement, App, Context, HeadlessAppContext, IntoElement, ParentElement,
    Render, Styled, Window,
};
use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, LayoutSizing, Node};
use poodle_specs::{ButtonSpec, Orientation, StepStatus, StepperSpec, StepperStep};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::fixture_capture::{inter_fonts, FixtureAssets};
use crate::presentation_axes::ThemePreset;
use crate::{ACCEPTED_SCALE, GPUI_REVISION, publish_pair};

/// Versioned evidence receipt schema identity.
const EVIDENCE_RECEIPT_SCHEMA: &str = "poodle.gpui-focus-evidence.v1";

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
    const USAGE: &str = "usage: poodle-offscreen-capture --focus-evidence \
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

fn build_scene(scene: &str, theme: &poodle_gpui::GpuiThemeProvider) -> EvidenceScene {
    match scene {
        "button" => {
            // Two bordered secondary Buttons at rest; the right one takes
            // focus. Distinct stamped ids make the focus target addressable
            // and keep the resting twin observable.
            let mut rest = poodle_render::button(
                &ButtonSpec::new().with_label("Save"),
                theme,
                None,
            );
            rest.id = Some("evidence:button:rest".to_owned());
            let mut focused = poodle_render::button(
                &ButtonSpec::new().with_label("Save"),
                theme,
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
                poodle_render::stepper(&spec, theme, poodle_render::StepperHandlers::default());
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
    gpui_revision: &'static str,
    renderer: &'static str,
    platform: &'static str,
    theme: &'static str,
    logical_viewport: [f32; 2],
    scale: f32,
    png_sha256: String,
}

pub fn run(args: &FocusEvidenceArgs) -> Result<()> {
    if !cfg!(target_os = "macos") {
        bail!("offscreen capture requires macOS: the Metal headless renderer exists nowhere else");
    }
    if gpui_platform::current_headless_renderer().is_none() {
        bail!("no GPUI headless renderer available: this machine exposes no compatible Metal device");
    }

    let theme = ThemePreset::Eclipse.build_theme();
    let canvas = theme.resolve_color("color.background.canvas");
    let scene = build_scene(&args.scene, &theme);
    let focus_id = scene.focus_id;
    let logical_width = scene.logical_width;
    let logical_height = scene.logical_height;

    let platform = gpui_platform::current_platform(true);
    let text_system = platform.text_system();
    let assets = FixtureAssets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };
    let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(assets), || {
        gpui_platform::current_headless_renderer()
    });
    cx.text_system()
        .add_fonts(inter_fonts()?)
        .with_context(|| "load the fixture Inter fonts")?;
    cx.update(|cx: &mut App| cx.set_reduce_motion(true));

    let window = cx.open_window(
        size(px(logical_width), px(logical_height)),
        |_, cx: &mut App| {
            <App as gpui::AppContext>::new(cx, |_| EvidenceRoot {
                node: scene.node,
                canvas: poodle_gpui_node_backend::color(canvas),
            })
        },
    )?;
    cx.run_until_parked();

    // Pump frames until the backend owns a focus handle for the target: the
    // handle is created in the paint pass and attached from the next build.
    let mut handle_ready = false;
    for _ in 0..16 {
        cx.update_window(window.into(), |_, window, cx| {
            window.refresh();
            let _ = window.draw(cx);
        })?;
        cx.run_until_parked();
        if poodle_gpui_node_backend::focus_handle_for(focus_id).is_some() {
            handle_ready = true;
            break;
        }
    }
    if !handle_ready {
        bail!("focus handle for '{focus_id}' never appeared — the ring declaration did not register one");
    }

    // Real focus through the backend registry — no pointer anywhere — then
    // one more frame so the ring canvas paints with focus held.
    cx.update_window(window.into(), |_, window, cx| {
        if let Some(handle) = poodle_gpui_node_backend::focus_handle_for(focus_id) {
            handle.focus(window, cx);
        }
        window.refresh();
        let _ = window.draw(cx);
    })?;
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.refresh();
        let _ = window.draw(cx);
    })?;
    cx.run_until_parked();

    let backend_focused =
        poodle_gpui_node_backend::focus_state_for(focus_id).unwrap_or(false);
    let painted = poodle_gpui_node_backend::painted_ring_for(focus_id).with_context(|| {
        format!("the focus ring for '{focus_id}' never painted — capture would not be evidence")
    })?;
    if !backend_focused {
        bail!("the backend never reported '{focus_id}' focused");
    }

    // Real offscreen readback: the rasterized focused frame.
    let image = cx.capture_screenshot(window.into())?;
    let expected_w = (logical_width * ACCEPTED_SCALE).round() as u32;
    let expected_h = (logical_height * ACCEPTED_SCALE).round() as u32;
    if image.width() != expected_w || image.height() != expected_h {
        bail!(
            "device dimensions {}x{} do not equal logical {}x{} × {ACCEPTED_SCALE}",
            image.width(),
            image.height(),
            logical_width,
            logical_height
        );
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )?;
    if png_bytes.is_empty() {
        bail!("capture produced an empty PNG encoding");
    }
    let png_sha256 = format!("{:x}", Sha256::digest(&png_bytes));

    let receipt = FocusEvidenceReceipt {
        schema: EVIDENCE_RECEIPT_SCHEMA,
        scene: args.scene.clone(),
        focused_element: focus_id.to_owned(),
        backend_focused,
        painted_ring: PaintedRingEvidence {
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
        gpui_revision: GPUI_REVISION,
        renderer: "metal-headless",
        platform: "macos",
        theme: ThemePreset::Eclipse.label(),
        logical_viewport: [logical_width, logical_height],
        scale: ACCEPTED_SCALE,
        png_sha256,
    };
    let receipt_json = serde_json::to_vec_pretty(&receipt)?;
    publish_pair(&args.out_png, &png_bytes, &args.out_receipt, &receipt_json)?;

    eprintln!(
        "captured focus-evidence {} ({}x{} logical @ {}x) focused={} ring=({}px @{}px) sha256={}",
        args.scene,
        logical_width,
        logical_height,
        ACCEPTED_SCALE,
        focus_id,
        painted.ring.width,
        painted.ring.offset,
        receipt.png_sha256
    );
    Ok(())
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
