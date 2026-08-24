//! g16.005 — deterministic non-activating window capture target.
//!
//! Renders real Poodle components through the production path
//! (`ButtonSpec` → `poodle_render::button` → `poodle_gpui_node_backend::to_gpui`)
//! into ONE real GPUI window opened with `focus: false`, then captures that
//! window by its own window id.
//!
//! This is a WINDOWED transport. crates.io GPUI 0.2.2 exposes no scene
//! readback and no headless renderer, so genuine GPUI pixels cannot be
//! obtained without a platform window. Nothing here is offscreen and nothing
//! here is headless; the receipts say so. It needs a macOS window server and
//! Screen Recording permission, and it therefore stays out of `qa`, CI, and
//! every release gate — it is an explicit operator diagnostic.
//!
//! What it will never do: activate the application, raise the window over the
//! operator's work, capture the desktop or a screen region, or fall back
//! silently when the window server is unavailable. `forbidden.rs` pins those
//! as a test over these sources, and every receipt carries the run's own
//! frontmost-application evidence.
//!
//! Modes:
//!
//! - `--fixture <exact-name> --out <png> --receipt <json>` renders one of the
//!   18 accepted g15.046 Button identities (`window_capture/fixture_capture.rs`);
//! - `--batch <manifest.json>` renders MANY of them in one process — one
//!   application, one window at a time, rather than one launch per fixture;
//! - `--focus-evidence <scene> --out <png> --receipt <json>` renders one
//!   focused-state scene (`window_capture/focus_evidence.rs`);
//! - `--inset-evidence <scene|all> --out-dir <dir>` renders the inset-shadow
//!   scene set (`window_capture/inset_evidence.rs`);
//! - without either, the single-Button smoke contract below applies.
//!
//! One-shot contract: every invocation captures once and writes a PNG plus a
//! typed JSON receipt. All inputs are validated before any window is opened;
//! unsupported scale, unknown theme or control size, an unsupported OS, a
//! missing window server, and a run that changed the frontmost application
//! are hard failures, never green skips.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context as _, Result};
use gpui::{
    AnyElement, App, AppContext as _, Context, IntoElement, ParentElement, Render, Styled, Window,
    div, px,
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
#[path = "window_capture/inventory.rs"]
mod inventory;

#[path = "window_capture/transport.rs"]
mod transport;

// The activation-boundary check reads these sources back and asserts that no
// focus-taking API appears in any code line. It is the one check that can
// prove the focus contract without running the windowed path, so it is a test
// surface rather than a runtime one.
#[cfg(test)]
#[path = "window_capture/forbidden.rs"]
mod forbidden;

#[path = "window_capture/fixture_capture.rs"]
mod fixture_capture;

// g15.052 focused-state evidence: real-focus captures of the ring channel
// for operator review. Point-in-time evidence, never a baseline.
#[path = "window_capture/focus_evidence.rs"]
mod focus_evidence;

// g16.005 inset-shadow evidence: the scene set that actually exercises the
// backend's own inset painter, through real components. The Button visual
// comparison cannot — its inventory is Button-only, and Button emits no
// shadow layers at all.
#[path = "window_capture/inset_evidence.rs"]
mod inset_evidence;

use presentation_axes::{ControlSize, ThemePreset};

use transport::{ACCEPTED_SCALE, GPUI_SOURCE, GPUI_VERSION, TRANSPORT};

/// Versioned receipt schema identity. `v1` claimed offscreen Metal readback
/// through a fork-only API; the transport is now a real non-activating
/// window, so this is a new schema rather than the old name over new facts.
const RECEIPT_SCHEMA: &str = "poodle.gpui-window-capture.v1";

/// The smoke scene: one real primary Button on a white canvas.
struct CaptureRoot {
    theme: GpuiThemeProvider,
}

impl Render for CaptureRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let spec = ButtonSpec::new()
            .with_label("Save")
            .with_variant(ButtonVariant::Primary);
        let ctx = poodle_render::RenderContext::new(&self.theme);
        let node = poodle_render::button(&spec, &ctx, None);
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

const USAGE: &str = "usage: poodle-window-capture \
--out <png> --receipt <json> --width <logical> --height <logical> \
--theme <name> --control-size <xs|sm|md|lg|xl> --scale 2.0";

const FIXTURE_USAGE: &str =
    "usage: poodle-window-capture --fixture <exact-name> --out <png> --receipt <json>";

const BATCH_USAGE: &str = "usage: poodle-window-capture --batch <manifest.json>\n\
manifest: {\"captures\":[{\"fixture\":\"<exact-name>\",\"out\":\"<png>\",\"receipt\":\"<json>\"}]}";

/// The capture modes. `--fixture` selects fixture mode; `--focus-evidence`
/// selects the g15.052 focused-state evidence mode; without either the
/// legacy smoke contract applies unchanged.
enum CaptureMode {
    Smoke(CaptureArgs),
    Fixture(fixture_capture::FixtureArgs),
    InsetEvidence(inset_evidence::InsetEvidenceArgs),
    /// Many fixtures, ONE process. The whole point of this mode is that a
    /// batch is one application launch and one window at a time, not a
    /// focus-capable application per fixture.
    Batch(Vec<fixture_capture::FixtureArgs>),
    FocusEvidence(focus_evidence::FocusEvidenceArgs),
}

fn parse_cli(argv: &[String]) -> Result<CaptureMode> {
    if argv.iter().any(|arg| arg == "--batch") {
        parse_batch_args(argv).map(CaptureMode::Batch)
    } else if argv.iter().any(|arg| arg == "--inset-evidence") {
        inset_evidence::parse_args(argv).map(CaptureMode::InsetEvidence)
    } else if argv.iter().any(|arg| arg == "--fixture") {
        parse_fixture_args(argv).map(CaptureMode::Fixture)
    } else if argv.iter().any(|arg| arg == "--focus-evidence") {
        focus_evidence::parse_args(argv).map(CaptureMode::FocusEvidence)
    } else {
        parse_args(argv).map(CaptureMode::Smoke)
    }
}

/// Batch mode is a closed contract too: exactly `--batch <manifest>`, nothing
/// else. The manifest is a closed JSON shape, every entry is validated to the
/// same standard as a single `--fixture` invocation, and duplicate output
/// paths across entries are rejected — a batch that quietly overwrote its own
/// earlier capture would produce evidence for a fixture it never kept.
fn parse_batch_args(argv: &[String]) -> Result<Vec<fixture_capture::FixtureArgs>> {
    if argv.len() != 2 || argv[0] != "--batch" {
        bail!("batch mode accepts exactly --batch <manifest.json>\n{BATCH_USAGE}");
    }
    let manifest_path = Path::new(&argv[1]);
    let source = std::fs::read_to_string(manifest_path)
        .with_context(|| format!("read the batch manifest {}", manifest_path.display()))?;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Manifest {
        captures: Vec<Entry>,
    }
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Entry {
        fixture: String,
        out: String,
        receipt: String,
    }

    let manifest: Manifest = serde_json::from_str(&source)
        .with_context(|| format!("parse the batch manifest {}", manifest_path.display()))?;
    if manifest.captures.is_empty() {
        bail!("the batch manifest declares no captures");
    }

    let mut parsed = Vec::with_capacity(manifest.captures.len());
    let mut seen: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    for (index, entry) in manifest.captures.iter().enumerate() {
        let args = parse_fixture_args(&[
            "--fixture".to_owned(),
            entry.fixture.clone(),
            "--out".to_owned(),
            entry.out.clone(),
            "--receipt".to_owned(),
            entry.receipt.clone(),
        ])
        .with_context(|| format!("batch entry {index} ({})", entry.fixture))?;

        for path in [&args.out_png, &args.out_receipt] {
            if !seen.insert(path.clone()) {
                bail!(
                    "batch entry {index} ({}) reuses the output path {} — a batch must not \
                     overwrite its own earlier capture",
                    entry.fixture,
                    path.display()
                );
            }
        }
        parsed.push(args);
    }
    Ok(parsed)
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
                        "unsupported scale '{value}': this lane captures at {ACCEPTED_SCALE}× \
                         only, which is the backing scale the fixture inventory is defined at. \
                         A capture at any other factor is rejected, never resampled."
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
    /// The published GPUI identity these pixels came from. Not a Git
    /// revision: a consumer reading this must see the registry crate.
    gpui_source: &'static str,
    gpui_version: &'static str,
    /// Named for what it is. Never `offscreen`, never `headless`.
    transport: &'static str,
    platform: &'static str,
    theme: &'static str,
    control_size: &'static str,
    logical_viewport: Viewport,
    scale: f32,
    device_dimensions: DeviceDimensions,
    png_sha256: String,
    /// The run's own proof that it did not take focus.
    foreground: transport::ForegroundEvidence,
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

fn run(args: &CaptureArgs) -> ! {
    let mut theme = args.theme.build_theme();
    theme = theme.with_control_size(args.control_size.token_definition());

    let logical_width = args.logical_width;
    let logical_height = args.logical_height;
    let theme_label = args.theme.label();
    let control_size_label = args.control_size.label();
    let out_png = args.out_png.clone();
    let out_receipt = args.out_receipt.clone();

    transport::capture(
        fixture_capture::FixtureAssets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        },
        Vec::new(),
        transport::Shot {
        label: "smoke".to_owned(),
        logical_width,
        logical_height,
        build: Box::new(move |_window, cx: &mut App| {
            cx.new(|_| CaptureRoot { theme })
        }),
        // The smoke scene has nothing to read back: its evidence is the
        // rasterized frame and its hash.
        on_frame: transport::settle_after(transport::FRAMES_BEFORE_CAPTURE),
        finish: Box::new(move |facts: &transport::CaptureFacts| {
            let png_sha256 = format!("{:x}", Sha256::digest(&facts.png));
            let receipt = CaptureReceipt {
                schema: RECEIPT_SCHEMA,
                component: ComponentSmoke {
                    name: "Button",
                    variant: "primary",
                    label: "Save",
                },
                gpui_source: GPUI_SOURCE,
                gpui_version: GPUI_VERSION,
                transport: TRANSPORT,
                platform: "macos",
                theme: theme_label,
                control_size: control_size_label,
                logical_viewport: Viewport {
                    width: logical_width,
                    height: logical_height,
                },
                scale: facts.scale,
                device_dimensions: DeviceDimensions {
                    width: facts.device_width,
                    height: facts.device_height,
                },
                png_sha256,
                foreground: facts.foreground.clone(),
            };
            let receipt_json = serde_json::to_vec_pretty(&receipt)?;
            publish_pair(&out_png, &facts.png, &out_receipt, &receipt_json)?;

            eprintln!(
                "captured {}x{} (logical {}x{} @ {}) theme={} size={} sha256={}",
                facts.device_width,
                facts.device_height,
                logical_width,
                logical_height,
                facts.scale,
                theme_label,
                control_size_label,
                receipt.png_sha256
            );
            Ok(())
        }),
    },
    )
}

/// Every mode ends in a `transport` capture, which owns the process from the
/// point the GPUI application starts. Argument parsing is the last thing that
/// can fail without a window ever existing, and it runs first.
fn main() -> ! {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mode = match parse_cli(&argv) {
        Ok(mode) => mode,
        Err(error) => {
            eprintln!("poodle-window-capture: {error:#}");
            std::process::exit(2);
        }
    };
    match mode {
        CaptureMode::Smoke(args) => run(&args),
        CaptureMode::Fixture(args) => fixture_capture::run(&args),
        CaptureMode::Batch(batch) => fixture_capture::run_batch(&batch),
        CaptureMode::InsetEvidence(args) => inset_evidence::run(&args),
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
            "poodle-window-capture-publish-test-{}",
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

    /// The receipt identity and the dependency pin are the same claim; drift
    /// between them invalidates every receipt. This also fails if the
    /// manifest ever goes back to a Git source, because a `git = ` pin
    /// carries no `gpui = "<version>"` line.
    #[test]
    fn the_gpui_identity_matches_the_manifest_pin() {
        let manifest = include_str!("../../Cargo.toml");
        assert!(
            manifest.contains(&format!("gpui = \"{GPUI_VERSION}\"")),
            "GPUI_VERSION drifted from the manifest pin"
        );
        assert!(
            !manifest.contains("git ="),
            "the preview manifest must resolve GPUI from {GPUI_SOURCE}, not a Git source"
        );
        assert_eq!(GPUI_SOURCE, "crates.io");
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

    // ── g16.005 batch mode ──────────────────────────────────────────────

    fn write_manifest(body: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let dir = std::env::temp_dir().join(format!(
            "poodle-window-capture-batch-test-{}-{}",
            std::process::id(),
            body.len()
        ));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("batch.json");
        std::fs::write(&path, body).expect("write manifest");
        (dir, path)
    }

    fn parse_manifest(body: &str) -> Result<Vec<fixture_capture::FixtureArgs>> {
        let (dir, path) = write_manifest(body);
        let result = parse_batch_args(&argv(&["--batch", path.to_str().expect("utf-8")]));
        let _ = std::fs::remove_dir_all(&dir);
        result
    }

    #[test]
    fn a_batch_manifest_parses_every_entry() {
        let batch = parse_manifest(
            r#"{"captures":[
                {"fixture":"button/rest-secondary","out":"a.png","receipt":"a.json"},
                {"fixture":"button/variant-primary","out":"b.png","receipt":"b.json"}
            ]}"#,
        )
        .expect("the canonical batch manifest parses");
        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0].fixture, "button/rest-secondary");
        assert_eq!(batch[1].fixture, "button/variant-primary");
    }

    /// One bad entry fails the whole batch during parsing, before any window
    /// opens — the same standard a single `--fixture` invocation is held to.
    #[test]
    fn one_unknown_fixture_rejects_the_whole_batch() {
        let error = parse_manifest(
            r#"{"captures":[
                {"fixture":"button/rest-secondary","out":"a.png","receipt":"a.json"},
                {"fixture":"button/nope","out":"b.png","receipt":"b.json"}
            ]}"#,
        )
        .expect_err("an unknown fixture must reject the batch");
        let text = format!("{error:#}");
        assert!(text.contains("batch entry 1"), "names the entry: {text}");
        assert!(text.contains("button/nope"), "names the offender: {text}");
    }

    /// A batch that reused an output path would overwrite its own earlier
    /// capture and publish evidence for a fixture it never kept.
    #[test]
    fn a_batch_that_reuses_an_output_path_is_rejected() {
        for body in [
            r#"{"captures":[
                {"fixture":"button/rest-secondary","out":"same.png","receipt":"a.json"},
                {"fixture":"button/variant-primary","out":"same.png","receipt":"b.json"}
            ]}"#,
            r#"{"captures":[
                {"fixture":"button/rest-secondary","out":"a.png","receipt":"same.json"},
                {"fixture":"button/variant-primary","out":"b.png","receipt":"same.json"}
            ]}"#,
            // A PNG that collides with another entry's receipt.
            r#"{"captures":[
                {"fixture":"button/rest-secondary","out":"a.png","receipt":"b.png"},
                {"fixture":"button/variant-primary","out":"b.png","receipt":"c.json"}
            ]}"#,
        ] {
            let error = parse_manifest(body).expect_err("a reused output path must be rejected");
            assert!(
                format!("{error:#}").contains("reuses the output path"),
                "got {error:#}"
            );
        }
    }

    #[test]
    fn an_empty_or_malformed_batch_manifest_is_rejected() {
        assert!(parse_manifest(r#"{"captures":[]}"#).is_err(), "empty batch");
        assert!(parse_manifest("not json").is_err(), "malformed JSON");
        assert!(
            parse_manifest(r#"{"captures":[{"fixture":"button/rest-secondary"}]}"#).is_err(),
            "an entry missing out/receipt"
        );
        assert!(
            parse_manifest(
                r#"{"captures":[{"fixture":"button/rest-secondary","out":"a.png","receipt":"a.json","scale":2}]}"#
            )
            .is_err(),
            "unknown keys are rejected, not ignored"
        );
        assert!(
            parse_manifest(r#"{"shots":[]}"#).is_err(),
            "an unknown top-level key is rejected"
        );
    }

    #[test]
    fn batch_mode_accepts_no_other_flag() {
        let (dir, path) = write_manifest(r#"{"captures":[]}"#);
        let manifest = path.to_str().expect("utf-8").to_owned();
        for extra in [
            vec!["--batch", &manifest, "--fixture", "button/rest-secondary"],
            vec!["--batch", &manifest, "--scale", "2.0"],
            vec!["--batch"],
        ] {
            assert!(
                parse_batch_args(&argv(&extra)).is_err(),
                "batch mode is a closed contract: {extra:?}"
            );
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_batch_flag_selects_batch_mode() {
        let (dir, path) = write_manifest(
            r#"{"captures":[{"fixture":"button/rest-secondary","out":"a.png","receipt":"a.json"}]}"#,
        );
        let mode = parse_cli(&argv(&["--batch", path.to_str().expect("utf-8")]))
            .expect("batch mode parses");
        assert!(matches!(mode, CaptureMode::Batch(ref b) if b.len() == 1));
        let _ = std::fs::remove_dir_all(&dir);
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
