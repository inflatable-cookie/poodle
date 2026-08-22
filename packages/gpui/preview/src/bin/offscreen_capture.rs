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
//! g15.047 adds fixture mode: `--fixture <exact-name> --out <png> --receipt
//! <json>` renders any of the 18 accepted Button inventory fixtures and writes
//! a typed `poodle.button-visual-capture.v1` receipt (see
//! `offscreen_capture/fixture_capture.rs`). Without `--fixture` the legacy
//! smoke contract below applies unchanged.
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

// g15.046/g15.047 inventory parser, shared with the `visual_fixture_inventory`
// test target by path — one Rust parser, two consumers. The capture binary
// needs only the roster and the typed loader; the validator's test-surface
// items stay reachable through the shared module.
#[allow(dead_code)]
#[path = "offscreen_capture/inventory.rs"]
mod inventory;

#[path = "offscreen_capture/fixture_capture.rs"]
mod fixture_capture;

// g15.052 focused-state evidence: real-focus captures of the ring channel
// for operator review. Point-in-time evidence, never a baseline.
#[path = "offscreen_capture/focus_evidence.rs"]
mod focus_evidence;

use presentation_axes::{ControlSize, ThemePreset};

/// The immutable upstream revision this seam is adopted at. Keep identical to
/// the `gpui` / `gpui_platform` manifest pins.
const GPUI_REVISION: &str = "87d9afbe71ef06ea0634499dc35d104bb29dc020";

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

const FIXTURE_USAGE: &str =
    "usage: poodle-offscreen-capture --fixture <exact-name> --out <png> --receipt <json>";

/// The capture modes. `--fixture` selects fixture mode; `--focus-evidence`
/// selects the g15.052 focused-state evidence mode; without either the
/// legacy smoke contract applies unchanged.
enum CaptureMode {
    Smoke(CaptureArgs),
    Fixture(fixture_capture::FixtureArgs),
    FocusEvidence(focus_evidence::FocusEvidenceArgs),
}

fn parse_cli(argv: &[String]) -> Result<CaptureMode> {
    if argv.iter().any(|arg| arg == "--fixture") {
        parse_fixture_args(argv).map(CaptureMode::Fixture)
    } else if argv.iter().any(|arg| arg == "--focus-evidence") {
        focus_evidence::parse_args(argv).map(CaptureMode::FocusEvidence)
    } else {
        parse_args(argv).map(CaptureMode::Smoke)
    }
}

/// Fixture mode is a closed contract: exactly `--fixture`, `--out`, and
/// `--receipt`, all required, every other flag rejected. The fixture name
/// must be one of the 18 accepted g15.046 identities; anything else is a hard
/// error naming the offender.
fn parse_fixture_args(argv: &[String]) -> Result<fixture_capture::FixtureArgs> {
    let mut fixture: Option<String> = None;
    let mut out_png: Option<PathBuf> = None;
    let mut out_receipt: Option<PathBuf> = None;

    let mut i = 0;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = argv
            .get(i + 1)
            .with_context(|| format!("missing value for {flag}\n{FIXTURE_USAGE}"))?;
        i += 2;
        match flag {
            "--fixture" => {
                if !inventory::BUTTON_FIXTURE_NAMES.contains(&value.as_str()) {
                    bail!(
                        "unknown fixture '{value}': not one of the {} g15.046 identities",
                        inventory::BUTTON_FIXTURE_NAMES.len()
                    );
                }
                fixture = Some(value.to_string());
            }
            "--out" => out_png = Some(PathBuf::from(value)),
            "--receipt" => out_receipt = Some(PathBuf::from(value)),
            other => bail!("argument '{other}' is not accepted in fixture mode\n{FIXTURE_USAGE}"),
        }
    }

    let out_png = normalize_output_path(
        &out_png.with_context(|| format!("--out is required\n{FIXTURE_USAGE}"))?,
        "--out",
    )?;
    let out_receipt = normalize_output_path(
        &out_receipt.with_context(|| format!("--receipt is required\n{FIXTURE_USAGE}"))?,
        "--receipt",
    )?;
    if out_png == out_receipt {
        bail!("--out and --receipt must name different files");
    }

    Ok(fixture_capture::FixtureArgs {
        fixture: fixture.with_context(|| format!("--fixture is required\n{FIXTURE_USAGE}"))?,
        out_png,
        out_receipt,
    })
}

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

    let out_png = normalize_output_path(
        &out_png.with_context(|| format!("--out is required\n{USAGE}"))?,
        "--out",
    )?;
    let out_receipt = normalize_output_path(
        &out_receipt.with_context(|| format!("--receipt is required\n{USAGE}"))?,
        "--receipt",
    )?;
    if out_png == out_receipt {
        bail!("--out and --receipt must name different files");
    }

    Ok(CaptureArgs {
        out_png,
        out_receipt,
        logical_width: width.with_context(|| format!("--width is required\n{USAGE}"))?,
        logical_height: height.with_context(|| format!("--height is required\n{USAGE}"))?,
        theme: theme.with_context(|| format!("--theme is required\n{USAGE}"))?,
        control_size: control_size.with_context(|| format!("--control-size is required\n{USAGE}"))?,
    })
}

/// Resolve the output's existing parent before renderer construction. Besides
/// failing early for unusable destinations, this makes lexical aliases such as
/// `./capture.png` and `capture.png` compare as the same path.
fn normalize_output_path(path: &Path, flag: &str) -> Result<PathBuf> {
    let file_name = path
        .file_name()
        .with_context(|| format!("{flag} must name a file"))?;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent = std::fs::canonicalize(parent)
        .with_context(|| format!("resolve parent directory for {flag}: {}", parent.display()))?;
    Ok(parent.join(file_name))
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

fn staged_path(path: &Path, kind: &str) -> Result<PathBuf> {
    let file_name = path
        .file_name()
        .with_context(|| format!("output path must name a file: {}", path.display()))?;
    let mut staged_name = file_name.to_os_string();
    staged_name.push(format!(".tmp-{}-{kind}", std::process::id()));
    Ok(path.with_file_name(staged_name))
}

fn remove_if_present(path: &Path) -> Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("remove {}", path.display())),
    }
}

/// Stage both files completely, invalidate any prior receipt, then publish the
/// PNG and receipt in that order. Once the prior receipt is removed, every
/// failure path leaves no final receipt, so stale evidence cannot validate a
/// newly published deterministic PNG.
fn publish_pair(
    png_path: &Path,
    png_bytes: &[u8],
    receipt_path: &Path,
    receipt_bytes: &[u8],
) -> Result<()> {
    if png_path == receipt_path {
        bail!("PNG and receipt outputs must name different files");
    }

    let staged_png = staged_path(png_path, "png")?;
    let staged_receipt = staged_path(receipt_path, "receipt")?;
    std::fs::write(&staged_png, png_bytes)
        .with_context(|| format!("stage PNG at {}", staged_png.display()))?;
    if let Err(error) = std::fs::write(&staged_receipt, receipt_bytes)
        .with_context(|| format!("stage receipt at {}", staged_receipt.display()))
    {
        let _ = std::fs::remove_file(&staged_png);
        return Err(error);
    }

    if let Err(error) = remove_if_present(receipt_path) {
        let _ = std::fs::remove_file(&staged_png);
        let _ = std::fs::remove_file(&staged_receipt);
        return Err(error);
    }
    if let Err(error) = std::fs::rename(&staged_png, png_path)
        .with_context(|| format!("publish PNG at {}", png_path.display()))
    {
        let _ = std::fs::remove_file(&staged_png);
        let _ = std::fs::remove_file(&staged_receipt);
        return Err(error);
    }
    // The receipt was absent immediately before publishing the PNG. If it now
    // exists, the two requested names alias on this filesystem (for example,
    // case-only variants on a case-insensitive volume) or another writer raced
    // us. Never let the receipt rename overwrite the published PNG or an
    // unexpected file.
    if receipt_path.exists() {
        let _ = std::fs::remove_file(png_path);
        let _ = std::fs::remove_file(&staged_receipt);
        bail!(
            "receipt output aliases or reappeared during PNG publication: {}",
            receipt_path.display()
        );
    }
    if let Err(error) = std::fs::rename(&staged_receipt, receipt_path)
        .with_context(|| format!("publish receipt at {}", receipt_path.display()))
    {
        let _ = std::fs::remove_file(&staged_receipt);
        return Err(error);
    }
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
    publish_pair(&args.out_png, &png_bytes, &args.out_receipt, &receipt_json)?;

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
    match parse_cli(&argv)? {
        CaptureMode::Smoke(args) => run(&args),
        CaptureMode::Fixture(args) => fixture_capture::run(&args),
        CaptureMode::FocusEvidence(args) => focus_evidence::run(&args),
    }
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
    fn colliding_output_paths_are_rejected() {
        assert!(parse_args(&replace("--receipt", "./button.png")).is_err());
    }

    #[test]
    fn publish_failure_invalidates_a_prior_receipt() {
        let root = std::env::temp_dir().join(format!(
            "poodle-offscreen-publish-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir(&root).expect("create test root");

        let blocked_png = root.join("blocked.png");
        std::fs::create_dir(&blocked_png).expect("make PNG destination unpublishable");
        let receipt = root.join("capture.json");
        std::fs::write(&receipt, b"stale receipt").expect("seed prior receipt");

        let result = publish_pair(&blocked_png, b"png", &receipt, b"new receipt");
        assert!(result.is_err(), "publishing over a directory must fail");
        assert!(
            !receipt.exists(),
            "a failed publish must not retain the prior receipt"
        );

        std::fs::remove_dir_all(&root).expect("remove test root");
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

    // ── g15.047 fixture mode ────────────────────────────────────────────

    const FIXTURE_VALID: &[&str] = &[
        "--fixture", "button/rest-secondary",
        "--out", "fixture.png",
        "--receipt", "fixture.json",
    ];

    #[test]
    fn fixture_invocation_parses() {
        let mode = parse_cli(&argv(FIXTURE_VALID)).expect("the canonical fixture invocation parses");
        let CaptureMode::Fixture(args) = mode else {
            panic!("--fixture must select fixture mode");
        };
        assert_eq!(args.fixture, "button/rest-secondary");
    }

    #[test]
    fn unknown_fixture_is_rejected_by_name() {
        let args = argv(&[
            "--fixture", "button/bogus",
            "--out", "fixture.png",
            "--receipt", "fixture.json",
        ]);
        let result = parse_cli(&args);
        assert!(result.is_err(), "an unknown fixture must not parse");
        let error = result.err().expect("checked above");
        assert!(
            format!("{error:#}").contains("unknown fixture 'button/bogus'"),
            "the error must name the offender, got {error:#}"
        );
    }

    #[test]
    fn legacy_flags_are_rejected_in_fixture_mode() {
        for (flag, value) in [
            ("--width", "240"),
            ("--height", "80"),
            ("--theme", "eclipse"),
            ("--control-size", "md"),
            ("--scale", "2.0"),
        ] {
            let mut v = argv(FIXTURE_VALID);
            v.push(flag.to_string());
            v.push(value.to_string());
            assert!(
                parse_cli(&v).is_err(),
                "{flag} must be rejected in fixture mode"
            );
        }
    }

    #[test]
    fn fixture_mode_requires_all_three_flags() {
        for flag in ["--fixture", "--out", "--receipt"] {
            let mut v = argv(FIXTURE_VALID);
            let i = v.iter().position(|a| a == flag).expect("flag present");
            v.drain(i..=i + 1);
            assert!(parse_cli(&v).is_err(), "missing {flag} must fail");
        }
    }

    #[test]
    fn fixture_flag_requires_a_value() {
        let args = argv(&["--fixture"]);
        assert!(parse_cli(&args).is_err(), "a bare --fixture must fail");
    }

    #[test]
    fn fixture_mode_rejects_colliding_output_paths() {
        let args = argv(&[
            "--fixture", "button/rest-secondary",
            "--out", "same.png",
            "--receipt", "./same.png",
        ]);
        assert!(parse_cli(&args).is_err());
    }

    #[test]
    fn without_fixture_flag_the_legacy_contract_applies() {
        let mode = parse_cli(&argv(VALID)).expect("the legacy invocation parses");
        assert!(
            matches!(mode, CaptureMode::Smoke(_)),
            "no --fixture flag must keep the legacy smoke mode"
        );
    }

    /// The node-backend observation change behind the fixture landmarks: an
    /// id-stamped icon (an svg leaf) must go through the wrapper path so its
    /// paint bounds are recorded. A plain in-memory test-platform window
    /// proves it — no Metal, no screenshot, no OS window.
    #[test]
    fn id_stamped_icon_records_paint_bounds() {
        struct IconRoot {
            node: poodle_node::Node,
        }
        impl Render for IconRoot {
            fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
                poodle_gpui_node_backend::reset_element_ids();
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(poodle_gpui_node_backend::to_gpui(&self.node))
            }
        }

        let mut cx = gpui::TestAppContext::single();
        let mut node = poodle_node::Node::icon("check", 12.0);
        node.id = Some("fixture-icon".to_owned());
        let (root, vc) = cx.add_window_view(|window, _cx| {
            window.refresh();
            IconRoot { node }
        });
        root.update(vc, |_root, cx| cx.notify());
        vc.update(|window, cx| {
            window.refresh();
            let _ = window.draw(cx);
        });
        vc.run_until_parked();

        // The recorded box is the wrapper's own geometry: an svg leaf's id
        // forces the wrapper path, and the wrapper shrink-wraps the leaf
        // inside a centered flex row — the same arrangement a Button's icon
        // slot uses.
        let bounds = poodle_gpui_node_backend::bounds_for("fixture-icon")
            .expect("an id-stamped icon must record its paint bounds");
        assert_eq!(f32::from(bounds.size.width), 12.0);
        assert_eq!(f32::from(bounds.size.height), 12.0);

        // The same teardown the headless regressions mirror from the
        // `#[gpui::test]` macro (which crashes on current rustc).
        cx.dispatcher.run_until_parked();
        cx.background_executor.forbid_parking();
        cx.quit();
        cx.dispatcher.run_until_parked();
    }
}
