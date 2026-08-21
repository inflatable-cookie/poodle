//! g15.045 — deterministic offscreen capture smoke target.
//!
//! Renders one real Poodle Button through the production path
//! (`ButtonSpec` → `poodle_render::button` → `poodle_gpui_node_backend::to_gpui`)
//! into GPUI's `HeadlessAppContext` and reads genuine rasterized pixels back
//! through the Metal headless renderer. No `NSWindow` is created, no desktop
//! capture runs, no focus is taken, no subprocess is spawned.
//!
//! This is an internal smoke command for the visual-conformance lane. It is
//! not a public component API, a fixture namespace, a baseline, or a portable
//! scene representation.
//!
//! One-shot contract: every invocation captures once and writes a PNG plus a
//! typed JSON receipt. All inputs are explicit and validated before any
//! renderer is constructed; unsupported scale, unknown theme or control size,
//! an unsupported OS, and a missing headless renderer are hard failures,
//! never green skips.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{bail, Context as _, Result};
use gpui::{
    AnyElement, App, Context, HeadlessAppContext, IntoElement, ParentElement, Render, Styled,
    Window, div, px, size,
};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, ButtonVariant};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[path = "../presentation_axes.rs"]
mod presentation_axes;

use presentation_axes::{ControlSize, ThemePreset};

/// The immutable upstream revision this seam is adopted at. Keep identical to
/// the `gpui` / `gpui_platform` manifest pins.
const GPUI_REVISION: &str = "1ea16c1ab9dd6d36649e002dc60995634da04daf";

/// The adopted revision's `TestWindow::scale_factor` is hardcoded `2.0`. This
/// lane is 2×-only by measured upstream constraint; any other requested scale
/// is rejected rather than silently approximated.
const ACCEPTED_SCALE: f32 = 2.0;

/// Versioned receipt schema identity.
const RECEIPT_SCHEMA: &str = "poodle.gpui-offscreen-capture.v1";

struct CaptureRoot {
    theme: GpuiThemeProvider,
}

impl Render for CaptureRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let spec = ButtonSpec::new()
            .with_label("Save")
            .with_variant(ButtonVariant::Primary);
        let node = poodle_render::button(&spec, &self.theme, None);
        poodle_gpui_node_backend::reset_element_ids();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&node);
        div()
            .size_full()
            .p(px(16.0))
            .bg(gpui::rgb(0xffffff))
            .child(element)
    }
}

/// Parsed and fully validated command line. Construction is the validation
/// gate: a `CaptureArgs` value is proof that every input was explicit and in
/// domain.
struct CaptureArgs {
    out_png: PathBuf,
    out_receipt: PathBuf,
    logical_width: f32,
    logical_height: f32,
    theme: ThemePreset,
    control_size: ControlSize,
}

const USAGE: &str = "usage: poodle-offscreen-capture \
--out <png> --receipt <json> --width <logical> --height <logical> \
--theme <name> --control-size <xs|sm|md|lg|xl> --scale 2.0";

fn parse_args(argv: &[String]) -> Result<CaptureArgs> {
    let mut out_png: Option<PathBuf> = None;
    let mut out_receipt: Option<PathBuf> = None;
    let mut width: Option<f32> = None;
    let mut height: Option<f32> = None;
    let mut theme: Option<ThemePreset> = None;
    let mut control_size: Option<ControlSize> = None;
    let mut scale_seen = false;

    let mut i = 0;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .with_context(|| format!("missing value for {flag}\n{USAGE}"))?;
        i += 2;
        match flag {
            "--out" => out_png = Some(PathBuf::from(value)),
            "--receipt" => out_receipt = Some(PathBuf::from(value)),
            "--width" => {
                let v: f32 = value
                    .parse()
                    .with_context(|| format!("--width must be a number, got '{value}'"))?;
                if !v.is_finite() || v <= 0.0 {
                    bail!("--width must be a positive finite logical size, got '{value}'");
                }
                width = Some(v);
            }
            "--height" => {
                let v: f32 = value
                    .parse()
                    .with_context(|| format!("--height must be a number, got '{value}'"))?;
                if !v.is_finite() || v <= 0.0 {
                    bail!("--height must be a positive finite logical size, got '{value}'");
                }
                height = Some(v);
            }
            "--theme" => {
                theme = Some(
                    ThemePreset::parse(value)
                        .with_context(|| format!("unknown theme '{value}'"))?,
                );
            }
            "--control-size" => {
                control_size = Some(
                    ControlSize::parse(value)
                        .with_context(|| format!("unknown control size '{value}'"))?,
                );
            }
            "--scale" => {
                let v: f32 = value
                    .parse()
                    .with_context(|| format!("--scale must be a number, got '{value}'"))?;
                if v != ACCEPTED_SCALE {
                    bail!(
                        "unsupported scale '{value}': the adopted GPUI revision's test window \
                         is hardcoded to {ACCEPTED_SCALE}; no local shim exists"
                    );
                }
                scale_seen = true;
            }
            other => bail!("unknown argument '{other}'\n{USAGE}"),
        }
    }

    // Scale has exactly one accepted value, but it must still be stated.
    // Requiring it keeps every invocation self-describing when a future
    // revision supports more than 2.0.
    if !scale_seen {
        bail!("--scale is required\n{USAGE}");
    }

    Ok(CaptureArgs {
        out_png: out_png.with_context(|| format!("--out is required\n{USAGE}"))?,
        out_receipt: out_receipt.with_context(|| format!("--receipt is required\n{USAGE}"))?,
        logical_width: width.with_context(|| format!("--width is required\n{USAGE}"))?,
        logical_height: height.with_context(|| format!("--height is required\n{USAGE}"))?,
        theme: theme.with_context(|| format!("--theme is required\n{USAGE}"))?,
        control_size: control_size.with_context(|| format!("--control-size is required\n{USAGE}"))?,
    })
}

/// Typed capture receipt. Serialized as JSON beside the PNG. The stable
/// identity carries no timestamps and no machine-specific paths.
#[derive(Serialize)]
struct CaptureReceipt {
    schema: &'static str,
    component: ComponentSmoke,
    gpui_revision: &'static str,
    renderer: &'static str,
    platform: &'static str,
    theme: &'static str,
    control_size: &'static str,
    logical_viewport: Viewport,
    scale: f32,
    device_dimensions: DeviceDimensions,
    png_sha256: String,
}

#[derive(Serialize)]
struct ComponentSmoke {
    name: &'static str,
    variant: &'static str,
    label: &'static str,
}

#[derive(Serialize)]
struct Viewport {
    width: f32,
    height: f32,
}

#[derive(Serialize)]
struct DeviceDimensions {
    width: u32,
    height: u32,
}

/// Write `bytes` to `path` through a sibling temporary file so an interrupted
/// run never leaves a partial file at the real path.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension(format!(
        "tmp-{}",
        std::process::id()
    ));
    std::fs::write(&tmp, bytes).with_context(|| format!("write {}", tmp.display()))?;
    std::fs::rename(&tmp, path).with_context(|| format!("rename onto {}", path.display()))?;
    Ok(())
}

fn run(args: &CaptureArgs) -> Result<()> {
    if !cfg!(target_os = "macos") {
        bail!("offscreen capture requires macOS: the Metal headless renderer exists nowhere else");
    }
    // A missing headless renderer (no Metal device) is an explicit failure,
    // checked before any capture is attempted — never a green skip.
    if gpui_platform::current_headless_renderer().is_none() {
        bail!("no GPUI headless renderer available: this machine exposes no compatible Metal device");
    }

    let mut theme = args.theme.build_theme();
    theme = theme.with_control_size(args.control_size.token_definition());

    let platform = gpui_platform::current_platform(true);
    let text_system = platform.text_system();
    let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(()), || {
        gpui_platform::current_headless_renderer()
    });

    let window = cx.open_window(
        size(px(args.logical_width), px(args.logical_height)),
        |_, cx: &mut App| <App as gpui::AppContext>::new(cx, |_| CaptureRoot { theme }),
    )?;
    cx.run_until_parked();

    // Real offscreen readback. A blank image, a synthetic PNG, or a node-tree
    // serialization would not be evidence; this is the rasterized frame.
    let image = cx.capture_screenshot(window.into())?;

    let expected_w = (args.logical_width * ACCEPTED_SCALE).round() as u32;
    let expected_h = (args.logical_height * ACCEPTED_SCALE).round() as u32;
    if image.width() != expected_w || image.height() != expected_h {
        bail!(
            "device dimensions {}x{} do not equal logical {}x{} × {ACCEPTED_SCALE}",
            image.width(),
            image.height(),
            args.logical_width,
            args.logical_height
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

    // PNG first, receipt second: an interrupted run can leave a PNG without a
    // receipt (a failure by contract) but never a matching-looking pair that
    // did not both come from this capture.
    write_atomic(&args.out_png, &png_bytes)?;

    let receipt = CaptureReceipt {
        schema: RECEIPT_SCHEMA,
        component: ComponentSmoke {
            name: "Button",
            variant: "primary",
            label: "Save",
        },
        gpui_revision: GPUI_REVISION,
        renderer: "metal-headless",
        platform: "macos",
        theme: args.theme.label(),
        control_size: args.control_size.label(),
        logical_viewport: Viewport {
            width: args.logical_width,
            height: args.logical_height,
        },
        scale: ACCEPTED_SCALE,
        device_dimensions: DeviceDimensions {
            width: image.width(),
            height: image.height(),
        },
        png_sha256,
    };
    let receipt_json = serde_json::to_vec_pretty(&receipt)?;
    write_atomic(&args.out_receipt, &receipt_json)?;

    eprintln!(
        "captured {}x{} (logical {}x{} @ {ACCEPTED_SCALE}) theme={} size={} sha256={}",
        image.width(),
        image.height(),
        args.logical_width,
        args.logical_height,
        args.theme.label(),
        args.control_size.label(),
        receipt.png_sha256
    );
    Ok(())
}

fn main() -> Result<()> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = parse_args(&argv)?;
    run(&args)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    const VALID: &[&str] = &[
        "--out", "button.png",
        "--receipt", "button.json",
        "--width", "240",
        "--height", "80",
        "--theme", "default",
        "--control-size", "md",
        "--scale", "2.0",
    ];

    #[test]
    fn valid_invocation_parses() {
        let args = parse_args(&argv(VALID)).expect("the canonical invocation parses");
        assert_eq!(args.logical_width, 240.0);
        assert_eq!(args.logical_height, 80.0);
        assert_eq!(args.theme, ThemePreset::Default);
        assert_eq!(args.control_size, ControlSize::Md);
    }

    fn replace(flag: &str, value: &str) -> Vec<String> {
        let mut v = argv(VALID);
        let i = v.iter().position(|a| a == flag).expect("flag present");
        v[i + 1] = value.to_string();
        v
    }

    #[test]
    fn non_double_scale_is_rejected() {
        for scale in ["1.0", "1.5", "3.0", "2.000001"] {
            assert!(
                parse_args(&replace("--scale", scale)).is_err(),
                "scale {scale} must be rejected"
            );
        }
    }

    #[test]
    fn unknown_theme_is_rejected() {
        assert!(parse_args(&replace("--theme", "dracula")).is_err());
    }

    #[test]
    fn unknown_control_size_is_rejected() {
        assert!(parse_args(&replace("--control-size", "xxl")).is_err());
    }

    #[test]
    fn every_flag_is_required() {
        for flag in [
            "--out", "--receipt", "--width", "--height", "--theme", "--control-size", "--scale",
        ] {
            let mut v = argv(VALID);
            let i = v.iter().position(|a| a == flag).expect("flag present");
            v.drain(i..=i + 1);
            assert!(parse_args(&v).is_err(), "missing {flag} must fail");
        }
    }

    #[test]
    fn nonpositive_viewport_is_rejected() {
        assert!(parse_args(&replace("--width", "0")).is_err());
        assert!(parse_args(&replace("--height", "-10")).is_err());
        assert!(parse_args(&replace("--width", "nan")).is_err());
    }

    #[test]
    fn unknown_argument_is_rejected() {
        let mut v = argv(VALID);
        v.push("--baseline".to_string());
        v.push("x".to_string());
        assert!(parse_args(&v).is_err());
    }

    #[test]
    fn revision_constant_matches_manifest_pin() {
        // The receipt identity and the dependency pin are the same claim;
        // drift between them invalidates every receipt.
        let manifest = include_str!("../../Cargo.toml");
        assert!(
            manifest.contains(&format!("rev = \"{GPUI_REVISION}\"")),
            "GPUI_REVISION drifted from the manifest pin"
        );
    }
}
