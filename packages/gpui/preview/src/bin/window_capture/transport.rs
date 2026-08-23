//! g16.005 — the non-activating window capture transport.
//!
//! Stock crates.io GPUI 0.2.2 has no scene readback and no headless renderer:
//! `PlatformWindow::draw` is one-way and the test window discards the scene.
//! Genuine GPUI pixels therefore require a real platform window. This module
//! is that window, and nothing more than that:
//!
//! - one GPUI window is opened with `focus: false`, `show: true`;
//! - `App::activate`, `Window::activate_window`, `makeKeyAndOrderFront`, and
//!   System Events activation are never called from anywhere in this binary
//!   (`crate::forbidden` pins that as a test over these sources);
//! - the window is found by this process's own pid and captured with
//!   `screencapture -x -o -l <window-id>` — one window id, never the desktop,
//!   never a region;
//! - the frontmost application is sampled for the whole run, and a run during
//!   which it changed fails rather than publishing evidence.
//!
//! It is windowed, not offscreen and not headless. It needs a macOS window
//! server and Screen Recording permission, so it is an explicit operator
//! diagnostic and stays out of `qa`, CI, and every release gate.

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{bail, Context as _, Result};
use gpui::{
    px, size, App, Application, AssetSource, Bounds, Entity, Point, Render, Window, WindowBounds,
    WindowOptions,
};
use serde::Serialize;

/// The only scale this lane accepts. The fixture inventory is 2×-only, so a
/// display that backs the capture window at any other factor is a hard
/// failure naming the observed factor — never a silent resample.
pub const ACCEPTED_SCALE: f32 = 2.0;

/// How the receipt names this transport. It is deliberately not the word
/// `offscreen` and not the word `headless`: both would be false here.
pub const TRANSPORT: &str = "macos-window-server-nonactivating";

/// The published GPUI identity these pixels come from. A consumer reading a
/// receipt must be able to tell that Poodle used the registry crate.
pub const GPUI_SOURCE: &str = "crates.io";
pub const GPUI_VERSION: &str = "0.2.2";

/// How many drawn frames a scene with nothing to read back waits for. One is
/// not enough: the first frame can land before layout has settled.
pub const FRAMES_BEFORE_CAPTURE: u32 = 3;

/// Ceiling on the settle chain for scenes that wait on something (a focus
/// handle the paint pass has not created yet, for example). A scene that
/// never reports itself settled fails; it does not capture a half-built
/// frame.
const MAX_SETTLE_FRAMES: u32 = 64;

/// Floor on how soon a capture may be taken regardless of frames drawn. The
/// preview's own screenshot path learned this the hard way: three frames land
/// in about 50ms, and a capture that early comes back at half the device size
/// because the window has painted but is not yet on the Retina backing store.
const MIN_SETTLE: Duration = Duration::from_millis(900);

/// Hard ceiling on waiting for a settled frame.
const SETTLE_DEADLINE: Duration = Duration::from_secs(20);

/// How often the frontmost application is sampled.
const FOREGROUND_SAMPLE_INTERVAL: Duration = Duration::from_millis(50);

static FRAMES_DRAWN: AtomicBool = AtomicBool::new(false);

/// The main thread's settled read, published for the capture thread. `None`
/// means the read has not run yet; `Some(Err(_))` fails the whole invocation.
static SETTLED: Mutex<Option<Result<()>>> = Mutex::new(None);

/// What a scene's frame hook reports about its own readiness.
pub enum Settled {
    /// Not yet: paint another frame and ask again.
    Wait,
    /// This frame is the one to capture.
    Ready,
}

/// Called on the main thread once per painted frame with the frame's
/// 1-based index. A scene uses it to wait for what it needs (a focus handle,
/// a settled layout) and to read back anything the receipt will carry —
/// landmark bounds, focus state, the painted ring. Returning `Err` aborts the
/// run without writing anything.
pub type FrameHook = Box<dyn FnMut(&mut Window, &mut App, u32) -> Result<Settled>>;

/// The default hook: wait a fixed number of frames, read nothing back.
pub fn settle_after(frames: u32) -> FrameHook {
    Box::new(move |_window, _cx, frame| {
        Ok(if frame >= frames {
            Settled::Ready
        } else {
            Settled::Wait
        })
    })
}

/// What the run observed about the frontmost application, recorded on every
/// receipt. `changed` is the claim the capture contract actually makes.
#[derive(Serialize, Clone)]
pub struct ForegroundEvidence {
    pub baseline: Option<String>,
    pub observed: Vec<String>,
    pub samples: u64,
    pub changed: bool,
}

#[derive(Default)]
struct ForegroundState {
    baseline: Option<String>,
    observed: BTreeSet<String>,
    samples: u64,
}

/// Samples the frontmost macOS application for the life of the run.
///
/// This is the capture contract's own evidence: opening the window must not
/// change the frontmost application, and a run that cannot prove that must
/// not publish a PNG.
pub struct ForegroundMonitor {
    state: Arc<Mutex<ForegroundState>>,
    stop: Arc<AtomicBool>,
}

#[cfg(target_os = "macos")]
fn frontmost_application() -> Option<String> {
    use objc2_app_kit::NSWorkspace;
    let workspace = NSWorkspace::sharedWorkspace();
    let app = workspace.frontmostApplication()?;
    app.bundleIdentifier()
        .map(|id| id.to_string())
        .or_else(|| app.localizedName().map(|name| name.to_string()))
}

#[cfg(not(target_os = "macos"))]
fn frontmost_application() -> Option<String> {
    None
}

impl ForegroundMonitor {
    /// Take the baseline BEFORE any window exists, then sample in the
    /// background for the rest of the run.
    pub fn start() -> Self {
        let baseline = frontmost_application();
        let state = Arc::new(Mutex::new(ForegroundState {
            observed: baseline.iter().cloned().collect(),
            samples: 1,
            baseline,
        }));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_state = Arc::clone(&state);
        let thread_stop = Arc::clone(&stop);
        std::thread::spawn(move || {
            while !thread_stop.load(Ordering::Acquire) {
                if let Some(app) = frontmost_application() {
                    let mut state = thread_state.lock().expect("foreground state");
                    state.observed.insert(app);
                    state.samples += 1;
                }
                std::thread::sleep(FOREGROUND_SAMPLE_INTERVAL);
            }
        });
        Self { state, stop }
    }

    /// Stop sampling and report what was seen.
    pub fn finish(&self) -> ForegroundEvidence {
        self.stop.store(true, Ordering::Release);
        let state = self.state.lock().expect("foreground state");
        let observed: Vec<String> = state.observed.iter().cloned().collect();
        ForegroundEvidence {
            changed: foreground_changed(state.baseline.as_deref(), &observed),
            baseline: state.baseline.clone(),
            observed,
            samples: state.samples,
        }
    }
}

/// Did the frontmost application change during the run?
///
/// A pure function so the claim the receipt makes is testable without a
/// window server. Note the `None` baseline case: no frontmost application at
/// all (a locked screen, a login window) means the run cannot prove it kept
/// its hands off the foreground, so an observation is treated as a change
/// rather than as an absence of evidence.
pub fn foreground_changed(baseline: Option<&str>, observed: &[String]) -> bool {
    match baseline {
        Some(baseline) => observed.iter().any(|app| app != baseline),
        None => !observed.is_empty(),
    }
}

/// Everything one capture produces, handed to the mode's finisher.
pub struct CaptureFacts {
    /// The window-server PNG exactly as `screencapture` wrote it. It is not
    /// re-encoded: what is published is what was captured.
    pub png: Vec<u8>,
    pub device_width: u32,
    pub device_height: u32,
    pub scale: f32,
    pub foreground: ForegroundEvidence,
}

/// One capture scene: what to render, what to read once it has painted, and
/// what to write once it has been captured.
pub struct Scene<V: Render, A: AssetSource> {
    pub logical_width: f32,
    pub logical_height: f32,
    pub assets: A,
    pub fonts: Vec<Cow<'static, [u8]>>,
    /// Builds the root view. Runs on the main thread.
    pub build: Box<dyn FnOnce(&mut Window, &mut App) -> Entity<V>>,
    /// Drives the scene to its settled frame and reads back whatever the
    /// receipt will carry. Runs on the main thread, once per painted frame.
    pub on_frame: FrameHook,
    /// Writes the PNG and its receipt. Runs on the capture thread.
    pub finish: Box<dyn FnOnce(&CaptureFacts) -> Result<()> + Send>,
}

/// Render `scene` in one non-activating window, capture that window, and exit.
///
/// This never returns: `Application::run` owns the process from here, so the
/// capture thread terminates it with the run's exit status.
pub fn capture<V: Render, A: AssetSource>(scene: Scene<V, A>) -> ! {
    if !cfg!(target_os = "macos") {
        fail(anyhow::anyhow!(
            "window capture requires macOS: the window-server capture path exists nowhere else"
        ));
    }

    let monitor = Arc::new(ForegroundMonitor::start());

    let Scene {
        logical_width,
        logical_height,
        assets,
        fonts,
        build,
        on_frame,
        finish,
    } = scene;

    Application::new().with_assets(assets).run(move |cx: &mut App| {
        if let Err(error) = open(cx, logical_width, logical_height, fonts, build, on_frame) {
            fail(error);
        }

        // The window is open and never activated. Everything from here runs
        // off the main thread so the run loop keeps drawing and the
        // foreground monitor keeps sampling.
        let monitor = Arc::clone(&monitor);
        std::thread::spawn(move || {
            let outcome = run_capture(logical_width, logical_height, &monitor, finish);
            match outcome {
                Ok(()) => std::process::exit(0),
                Err(error) => fail(error),
            }
        });
    });

    // `Application::run` does not return on macOS; if it ever does, the run
    // produced no capture, which is a failure rather than a silent success.
    fail(anyhow::anyhow!(
        "the GPUI application exited before the capture completed"
    ));
}

fn open<V: Render>(
    cx: &mut App,
    logical_width: f32,
    logical_height: f32,
    fonts: Vec<Cow<'static, [u8]>>,
    build: Box<dyn FnOnce(&mut Window, &mut App) -> Entity<V>>,
    on_frame: FrameHook,
) -> Result<()> {
    if !fonts.is_empty() {
        cx.text_system()
            .add_fonts(fonts)
            .with_context(|| "load the capture scene fonts")?;
    }

    let bounds = Bounds {
        origin: Point {
            x: px(0.0),
            y: px(0.0),
        },
        size: size(px(logical_width), px(logical_height)),
    };

    // `titlebar: None` is the frame contract, not decoration: GPUI maps it to
    // a titled, full-size-content window with a transparent, title-less bar
    // and no traffic lights, so the window FRAME equals the requested logical
    // content rect. The capture can then assert device == logical × scale
    // instead of guessing where the content sits inside a frame.
    //
    // `focus: false` with no activation call anywhere is the focus contract.
    let window = cx
        .open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: None,
                focus: false,
                show: true,
                is_movable: false,
                is_resizable: false,
                is_minimizable: false,
                ..Default::default()
            },
            build,
        )
        .with_context(|| "open the capture window")?;

    window
        .update(cx, |_, window, _cx| {
            window.refresh();
            schedule_settle(window, 1, on_frame);
        })
        .with_context(|| "schedule the settle chain")?;

    Ok(())
}

/// Run the scene's frame hook on the main thread once per painted frame until
/// it reports itself settled, then release the capture thread. The hook must
/// see the same painted frame the capture will.
fn schedule_settle(window: &mut Window, frame: u32, mut on_frame: FrameHook) {
    window.on_next_frame(move |window, cx| {
        let outcome = on_frame(window, cx, frame);
        let settled = match outcome {
            Ok(Settled::Ready) => Some(Ok(())),
            Ok(Settled::Wait) if frame < MAX_SETTLE_FRAMES => None,
            Ok(Settled::Wait) => Some(Err(anyhow::anyhow!(
                "the scene never reported itself settled within {MAX_SETTLE_FRAMES} frames"
            ))),
            Err(error) => Some(Err(error)),
        };
        match settled {
            Some(result) => {
                *SETTLED.lock().expect("settle slot") = Some(result);
                FRAMES_DRAWN.store(true, Ordering::Release);
            }
            None => {
                window.refresh();
                schedule_settle(window, frame + 1, on_frame);
            }
        }
    });
}

fn run_capture(
    logical_width: f32,
    logical_height: f32,
    monitor: &ForegroundMonitor,
    finish: Box<dyn FnOnce(&CaptureFacts) -> Result<()> + Send>,
) -> Result<()> {
    wait_for_settled_frame()?;
    match SETTLED.lock().expect("settle slot").take() {
        Some(Ok(())) => {}
        Some(Err(error)) => return Err(error),
        None => bail!("the settle chain never ran"),
    }

    let png = capture_own_window()?;
    let (device_width, device_height) = png_dimensions(&png)?;
    verify_device_size(logical_width, logical_height, device_width, device_height)?;

    // The focus claim is checked before anything is published: a run that
    // changed the frontmost application is not evidence, it is a defect.
    let foreground = monitor.finish();
    if foreground.changed {
        bail!(
            "the capture changed the frontmost application (baseline {:?}, observed {:?}) — \
             the non-activating contract was violated and no capture was published",
            foreground.baseline,
            foreground.observed
        );
    }

    let facts = CaptureFacts {
        png,
        device_width,
        device_height,
        scale: ACCEPTED_SCALE,
        foreground,
    };
    finish(&facts)
}

fn wait_for_settled_frame() -> Result<()> {
    let started = Instant::now();
    loop {
        if FRAMES_DRAWN.load(Ordering::Acquire) && started.elapsed() >= MIN_SETTLE {
            return Ok(());
        }
        if started.elapsed() > SETTLE_DEADLINE {
            bail!(
                "the capture window never reported a settled frame within {}s",
                SETTLE_DEADLINE.as_secs()
            );
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

/// The exact device size a logical scene must capture at. A pure function so
/// the policy is testable without a window server.
pub fn expected_device_size(logical_width: f32, logical_height: f32) -> (u32, u32) {
    (
        (logical_width * ACCEPTED_SCALE).round() as u32,
        (logical_height * ACCEPTED_SCALE).round() as u32,
    )
}

pub fn verify_device_size(
    logical_width: f32,
    logical_height: f32,
    device_width: u32,
    device_height: u32,
) -> Result<()> {
    let (expected_w, expected_h) = expected_device_size(logical_width, logical_height);
    if device_width != expected_w || device_height != expected_h {
        bail!(
            "captured {device_width}x{device_height}, expected {expected_w}x{expected_h} \
             (logical {logical_width}x{logical_height} × {ACCEPTED_SCALE}). A display that does \
             not back this window at {ACCEPTED_SCALE}×, or a window frame larger than its \
             content, would produce this. The capture is rejected rather than resampled or \
             cropped."
        );
    }
    Ok(())
}

/// Read width and height out of a PNG's IHDR chunk.
///
/// The captured bytes are published verbatim, so the only thing this needs to
/// do is read the header — decoding and re-encoding would replace the window
/// server's own pixels with this process's idea of them.
pub fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32)> {
    const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    if bytes.len() < 33 || bytes[..8] != SIGNATURE {
        bail!("the capture output is not a PNG");
    }
    if &bytes[12..16] != b"IHDR" {
        bail!("the capture output has no leading IHDR chunk");
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    if width == 0 || height == 0 {
        bail!("the capture output declares a zero dimension");
    }
    Ok((width, height))
}

/// Find this process's own window through the window server and capture it by
/// id. Nothing here can reach another application's window, the desktop, or a
/// screen region: `screencapture -l` takes exactly one window id, and the id
/// is filtered to this pid.
fn capture_own_window() -> Result<Vec<u8>> {
    let pid = std::process::id();
    let window_id = own_window_id(pid)?;

    let directory = std::env::temp_dir().join(format!("poodle-window-capture-{pid}"));
    std::fs::create_dir_all(&directory)
        .with_context(|| format!("create the capture staging directory {}", directory.display()))?;
    let staged = directory.join("capture.png");
    let _ = std::fs::remove_file(&staged);

    // `-x` no sound, `-o` exclude the window's drop shadow, `-l` this window
    // id only. There is no `-R` (region) and no desktop mode anywhere.
    let status = std::process::Command::new("screencapture")
        .args([
            "-x",
            "-o",
            "-l",
            &window_id.to_string(),
            staged.to_str().with_context(|| "capture path is not UTF-8")?,
        ])
        .status()
        .with_context(|| "run screencapture")?;
    if !status.success() {
        bail!(
            "screencapture failed with status {status}. Window capture needs a macOS window \
             server and Screen Recording permission for this binary's parent process; there is \
             no desktop or region fallback."
        );
    }

    let bytes = std::fs::read(&staged).with_context(|| {
        format!(
            "screencapture reported success but wrote no file at {}",
            staged.display()
        )
    })?;
    let _ = std::fs::remove_file(&staged);
    let _ = std::fs::remove_dir(&directory);
    if bytes.is_empty() {
        bail!("screencapture wrote an empty file");
    }
    Ok(bytes)
}

/// The largest on-screen window owned by this pid.
fn own_window_id(pid: u32) -> Result<u64> {
    let script = format!(
        concat!(
            "import CoreGraphics\n",
            "let list = CGWindowListCopyWindowInfo(.optionAll, kCGNullWindowID) as! [[String: Any]]\n",
            "var best = 0; var bestArea = 0\n",
            "for entry in list {{\n",
            "  guard let owner = entry[\"kCGWindowOwnerPID\"] as? Int, owner == {pid} else {{ continue }}\n",
            "  let bounds = entry[\"kCGWindowBounds\"] as? [String: Any] ?? [:]\n",
            "  let height = bounds[\"Height\"] as? Int ?? 0\n",
            "  let width = bounds[\"Width\"] as? Int ?? 0\n",
            "  if height * width > bestArea {{\n",
            "    bestArea = height * width\n",
            "    best = entry[\"kCGWindowNumber\"] as? Int ?? 0\n",
            "  }}\n",
            "}}\n",
            "print(best)",
        ),
        pid = pid
    );
    let output = std::process::Command::new("swift")
        .arg("-e")
        .arg(script)
        .output()
        .with_context(|| "look this process's own window up in the window server")?;
    if !output.status.success() {
        bail!(
            "window lookup failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let id: u64 = text
        .trim()
        .parse()
        .with_context(|| format!("window lookup returned {text:?}"))?;
    if id == 0 {
        bail!(
            "no on-screen window is owned by this process (pid {pid}). A macOS window server \
             is required: crates.io GPUI {GPUI_VERSION} exposes no scene readback, so there is \
             no windowless capture path to fall back to."
        );
    }
    Ok(id)
}

fn fail(error: anyhow::Error) -> ! {
    eprintln!("poodle-window-capture: {error:#}");
    std::process::exit(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ihdr(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
        bytes.extend_from_slice(&13u32.to_be_bytes());
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&width.to_be_bytes());
        bytes.extend_from_slice(&height.to_be_bytes());
        bytes.extend_from_slice(&[8, 6, 0, 0, 0, 0, 0, 0, 0]);
        bytes
    }

    #[test]
    fn png_dimensions_read_the_ihdr_chunk() {
        assert_eq!(png_dimensions(&ihdr(480, 160)).unwrap(), (480, 160));
    }

    #[test]
    fn a_non_png_capture_is_rejected() {
        assert!(png_dimensions(b"not a png at all, not even close").is_err());
        let mut truncated = ihdr(480, 160);
        truncated.truncate(20);
        assert!(png_dimensions(&truncated).is_err());
    }

    #[test]
    fn a_zero_dimension_capture_is_rejected() {
        assert!(png_dimensions(&ihdr(0, 160)).is_err());
        assert!(png_dimensions(&ihdr(480, 0)).is_err());
    }

    #[test]
    fn the_expected_device_size_is_logical_times_the_accepted_scale() {
        assert_eq!(expected_device_size(240.0, 80.0), (480, 160));
        assert_eq!(ACCEPTED_SCALE, 2.0);
    }

    fn apps(names: &[&str]) -> Vec<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn a_run_that_only_ever_saw_the_baseline_did_not_take_focus() {
        assert!(!foreground_changed(
            Some("com.example.editor"),
            &apps(&["com.example.editor"])
        ));
    }

    #[test]
    fn any_other_frontmost_application_counts_as_a_change() {
        assert!(foreground_changed(
            Some("com.example.editor"),
            &apps(&["com.example.editor", "com.inflatablecookie.poodle-window-capture"])
        ));
    }

    /// Without a baseline the run has no evidence, and no evidence is not the
    /// same as proof. It must not pass.
    #[test]
    fn an_absent_baseline_with_any_observation_is_not_proof() {
        assert!(foreground_changed(None, &apps(&["com.example.editor"])));
        assert!(!foreground_changed(None, &[]));
    }

    /// A 1× display, or a window frame bigger than its content, must fail
    /// loudly. Silently resampling or cropping would turn a broken capture
    /// into evidence.
    #[test]
    fn a_device_size_that_is_not_logical_times_scale_is_rejected() {
        assert!(verify_device_size(240.0, 80.0, 480, 160).is_ok());
        assert!(verify_device_size(240.0, 80.0, 240, 80).is_err());
        assert!(verify_device_size(240.0, 80.0, 480, 216).is_err());
        assert!(verify_device_size(240.0, 80.0, 616, 262).is_err());
    }
}
