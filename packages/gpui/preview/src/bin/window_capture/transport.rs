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
//! - the frontmost process is sampled for the whole run, and a run during
//!   which THIS process ever became frontmost fails rather than publishing
//!   evidence. Unrelated foreground transitions — the operator switching
//!   applications while the batch runs — are admissible, stay recorded, and
//!   never fail the run.
//!
//! It is windowed, not offscreen and not headless. It needs a macOS window
//! server and Screen Recording permission, so it is an explicit operator
//! diagnostic and stays out of `qa`, CI, and every release gate.
//!
//! A whole batch runs in ONE process. `capture_batch` opens, settles,
//! captures, and closes each scene's window in turn on a single async driver,
//! so an 18-fixture run with its repeat pass is one application and 36
//! sequential windows — not 36 application launches. One foreground monitor
//! spans the whole batch, so its evidence covers every capture in it.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{bail, Context as _, Result};
use gpui::{
    px, size, App, AppContext as _, Application, AssetSource, AsyncApp, Bounds, Entity, Point,
    Render, VisualContext as _, Window, WindowBounds, WindowOptions,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
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

/// How often the frontmost process is sampled.
const FOREGROUND_SAMPLE_INTERVAL: Duration = Duration::from_millis(50);

/// How often the settle chain is polled while the run loop paints.
const SETTLE_POLL: Duration = Duration::from_millis(10);

/// Fewest successful frontmost-process readings a run must have before its
/// evidence supports the claim that the capture process never became
/// frontmost. The monitor samples every `FOREGROUND_SAMPLE_INTERVAL`, and
/// every capture waits at least `MIN_SETTLE`, so a healthy run records
/// several times this many. A run that somehow recorded fewer has not
/// watched the foreground long enough to say anything about it.
pub const MIN_FOREGROUND_SAMPLES: u64 = 8;

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

/// One successful reading of the frontmost process.
///
/// Both halves ride on the receipt: the identity is the auditable "which
/// application", and the pid is what the self test compares. The capture
/// process's own pid rides as `ForegroundEvidence::capturer_pid`, so a
/// reader can tell exactly which process the proof is about and re-derive
/// the verdict from these fields instead of taking the writer's word.
#[derive(Serialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ForegroundSample {
    /// The bundle identifier when the frontmost process has one, else its
    /// localized name — whatever AppKit itself reports, never a guessed
    /// bundle for a bare process.
    pub identity: String,
    /// The frontmost process id. Equality against the capture process's own
    /// pid is the self test; no bundle naming can hide a self-activation.
    pub pid: u32,
}

/// Whether a run's frontmost-application evidence supports the capture
/// contract's claim: that this capture process never activated itself.
///
/// Three states, not a boolean, because "never frontmost" and "could not
/// tell" are different answers and only one of them is proof. Unrelated
/// foreground transitions — the operator switching applications while the
/// batch runs — are admissible evidence and never fail the run.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ForegroundVerdict {
    /// A baseline was read, no required read failed, enough samples were
    /// taken, and the capture process was never frontmost — neither as the
    /// baseline nor in any later sample. This is the only publishable
    /// verdict.
    Proved,
    /// The capture process itself was frontmost at least once — as the
    /// baseline or in a later sample.
    SelfFrontmost,
    /// No baseline, a failed required read, or too few successful samples.
    /// No evidence is not the same as evidence of no activation.
    Unprovable,
}

/// What the run observed about the frontmost process, recorded on every
/// receipt.
///
/// `capturer_pid` names the process under test, and `baseline`/`observed`
/// carry identity AND pid per reading, so the verdict is re-derivable from
/// the receipt rather than trusted on the writer's say-so.
#[derive(Serialize, Clone)]
pub struct ForegroundEvidence {
    /// The pid of the capture process itself — the process this proof is
    /// about. Only this pid can fail the run; every other foreground process
    /// is the operator's own business.
    pub capturer_pid: u32,
    /// The frontmost process BEFORE any window existed. `None` means that
    /// reading failed (a locked screen or a login window), which is not
    /// proof of anything.
    pub baseline: Option<ForegroundSample>,
    /// Every distinct (identity, pid) frontmost reading of the whole run,
    /// the baseline included. Unrelated operator transitions legitimately
    /// appear here and are retained as evidence.
    pub observed: Vec<ForegroundSample>,
    /// Successful readings. The baseline counts when it was readable; a tick
    /// that could not be read never increments this.
    pub samples: u64,
    /// Ticks on which the frontmost process could not be read at all. Any
    /// such failure makes the run unprovable — a silent skip would let a run
    /// claim more watching than it did.
    pub failed_reads: u64,
    pub verdict: ForegroundVerdict,
}

#[derive(Default)]
struct ForegroundState {
    baseline: Option<ForegroundSample>,
    observed: BTreeSet<ForegroundSample>,
    samples: u64,
    failed_reads: u64,
}

/// Samples the frontmost macOS process for the life of the run.
///
/// This is the capture contract's own evidence: opening the window must not
/// make THE CAPTURE PROCESS frontmost, and a run that cannot prove that must
/// not publish a PNG. Unrelated operator transitions do not fail the run;
/// they are recorded and stay on the receipt.
pub struct ForegroundMonitor {
    state: Arc<Mutex<ForegroundState>>,
    stop: Arc<AtomicBool>,
    capturer_pid: u32,
}

#[cfg(target_os = "macos")]
fn frontmost_application() -> Option<ForegroundSample> {
    use objc2_app_kit::NSWorkspace;
    let workspace = NSWorkspace::sharedWorkspace();
    let app = workspace.frontmostApplication()?;
    let identity = app
        .bundleIdentifier()
        .map(|id| id.to_string())
        .or_else(|| app.localizedName().map(|name| name.to_string()))?;
    let pid = app.processIdentifier();
    if pid <= 0 {
        // AppKit returned no usable pid; a reading without one cannot be
        // compared against the capture process, so it fails closed.
        return None;
    }
    Some(ForegroundSample {
        identity,
        pid: pid as u32,
    })
}

#[cfg(not(target_os = "macos"))]
fn frontmost_application() -> Option<ForegroundSample> {
    None
}

impl ForegroundMonitor {
    /// Take the baseline BEFORE any window exists, then sample in the
    /// background for the rest of the run.
    pub fn start() -> Self {
        let capturer_pid = std::process::id();
        let baseline = frontmost_application();
        let state = Arc::new(Mutex::new(ForegroundState {
            observed: baseline.iter().cloned().collect(),
            // Only a successful reading counts. An unreadable baseline leaves
            // samples at zero and records the failed read, which keeps the
            // verdict `Unprovable` rather than letting an empty run look like
            // a watched one.
            samples: u64::from(baseline.is_some()),
            failed_reads: u64::from(baseline.is_none()),
            baseline,
        }));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_state = Arc::clone(&state);
        let thread_stop = Arc::clone(&stop);
        std::thread::spawn(move || {
            while !thread_stop.load(Ordering::Acquire) {
                match frontmost_application() {
                    Some(sample) => {
                        let mut state = thread_state.lock().expect("foreground state");
                        state.observed.insert(sample);
                        state.samples += 1;
                    }
                    None => {
                        // A tick that could not be read is a required reading
                        // that failed. The run fails closed on it rather than
                        // quietly watching less than it claims.
                        thread_state.lock().expect("foreground state").failed_reads += 1;
                    }
                }
                std::thread::sleep(FOREGROUND_SAMPLE_INTERVAL);
            }
        });
        Self {
            state,
            stop,
            capturer_pid,
        }
    }

    /// Snapshot what has been seen so far, without stopping. A batch calls
    /// this once per capture; the samples accumulate across the whole run.
    pub fn evidence(&self) -> ForegroundEvidence {
        let state = self.state.lock().expect("foreground state");
        let observed: Vec<ForegroundSample> = state.observed.iter().cloned().collect();
        ForegroundEvidence {
            capturer_pid: self.capturer_pid,
            verdict: evaluate_foreground(
                self.capturer_pid,
                state.baseline.as_ref(),
                &observed,
                state.samples,
                state.failed_reads,
            ),
            baseline: state.baseline.clone(),
            observed,
            samples: state.samples,
            failed_reads: state.failed_reads,
        }
    }

    /// Stop sampling. Called once the batch is finished.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);
    }
}

/// Grade a run's foreground evidence.
///
/// A pure function, so the claim every receipt makes is testable without a
/// window server. The verdict is about the capture process only: every
/// observed pid is compared against `capturer_pid`, and any other process
/// coming and going is the operator's own business, retained in `observed`
/// as evidence. It fails closed in every direction that matters: an
/// unreadable baseline, a failed required read, and a too-short watch are
/// `Unprovable`, never `Proved`, while a self-frontmost reading — baseline
/// or later — is the typed failure `SelfFrontmost` no matter how short the
/// watch was.
pub fn evaluate_foreground(
    capturer_pid: u32,
    baseline: Option<&ForegroundSample>,
    observed: &[ForegroundSample],
    samples: u64,
    failed_reads: u64,
) -> ForegroundVerdict {
    // Self-activation outranks every other finding: if the capture process
    // WAS frontmost — as the baseline or in any later sample — that is the
    // answer, even when some other reading failed or the watch was short.
    if observed.iter().any(|sample| sample.pid == capturer_pid) {
        return ForegroundVerdict::SelfFrontmost;
    }
    if baseline.is_none() {
        // No frontmost process could be read at all — a locked screen or a
        // login window. The run cannot say what it did or did not disturb.
        return ForegroundVerdict::Unprovable;
    }
    if failed_reads > 0 {
        // A required reading failed somewhere in the run. Failing closed is
        // the only honest answer: the monitor cannot claim to have watched
        // every moment it says it watched.
        return ForegroundVerdict::Unprovable;
    }
    if observed.is_empty() || samples < MIN_FOREGROUND_SAMPLES {
        return ForegroundVerdict::Unprovable;
    }
    ForegroundVerdict::Proved
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
pub struct Shot<V: Render> {
    /// A short name for progress output. Never enters a receipt.
    pub label: String,
    pub logical_width: f32,
    pub logical_height: f32,
    /// Builds the root view. Runs on the main thread.
    pub build: Box<dyn FnOnce(&mut Window, &mut App) -> Entity<V>>,
    /// Drives the scene to its settled frame and reads back whatever the
    /// receipt will carry. Runs on the main thread, once per painted frame.
    pub on_frame: FrameHook,
    /// Writes the PNG and its receipt. Runs off the main thread.
    pub finish: Box<dyn FnOnce(&CaptureFacts) -> Result<()> + Send>,
}

/// Render one scene in one non-activating window, capture it, and exit.
pub fn capture<V: Render, A: AssetSource>(
    assets: A,
    fonts: Vec<Cow<'static, [u8]>>,
    shot: Shot<V>,
) -> ! {
    capture_batch(assets, fonts, vec![shot])
}

/// Render every scene in turn — ONE process, one window at a time — then exit.
///
/// The card asks for one bounded capture process for a fixture batch rather
/// than a focus-capable application per fixture. This is that: the
/// application starts once, and each shot opens its window, settles, is
/// captured, and has its window removed before the next begins.
pub fn capture_batch<V: Render, A: AssetSource>(
    assets: A,
    fonts: Vec<Cow<'static, [u8]>>,
    shots: Vec<Shot<V>>,
) -> ! {
    if !cfg!(target_os = "macos") {
        fail(anyhow::anyhow!(
            "window capture requires macOS: the window-server capture path exists nowhere else"
        ));
    }
    if shots.is_empty() {
        fail(anyhow::anyhow!("the capture batch is empty"));
    }

    // Baseline BEFORE the application exists, let alone a window.
    let monitor = Arc::new(ForegroundMonitor::start());

    Application::new().with_assets(assets).run(move |cx: &mut App| {
        if !fonts.is_empty() {
            if let Err(error) = cx
                .text_system()
                .add_fonts(fonts)
                .with_context(|| "load the capture scene fonts")
            {
                fail(error);
            }
        }

        let monitor = Arc::clone(&monitor);
        cx.spawn(async move |cx: &mut AsyncApp| {
            let total = shots.len();
            for (index, shot) in shots.into_iter().enumerate() {
                if let Err(error) = capture_one(cx, shot, &monitor, index + 1, total).await {
                    monitor.stop();
                    fail(error);
                }
            }
            monitor.stop();
            std::process::exit(0);
        })
        .detach();
    });

    // `Application::run` does not return on macOS; if it ever does, the run
    // produced no capture, which is a failure rather than a silent success.
    fail(anyhow::anyhow!(
        "the GPUI application exited before the capture completed"
    ));
}

async fn capture_one<V: Render>(
    cx: &mut AsyncApp,
    shot: Shot<V>,
    monitor: &ForegroundMonitor,
    index: usize,
    total: usize,
) -> Result<()> {
    let Shot {
        label,
        logical_width,
        logical_height,
        build,
        on_frame,
        finish,
    } = shot;

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
        .update(|cx: &mut App| {
            cx.open_window(
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
        })
        .with_context(|| format!("open the capture window for {label}"))?
        .with_context(|| format!("open the capture window for {label}"))?;

    let opened = Instant::now();
    let settled: Rc<RefCell<Option<Result<()>>>> = Rc::new(RefCell::new(None));
    let chain = Rc::clone(&settled);
    cx.update_window(window.into(), move |_, window, _cx| {
        window.refresh();
        schedule_settle(window, 1, on_frame, chain);
    })
    .with_context(|| format!("schedule the settle chain for {label}"))?;

    // Wait for the scene to report itself settled. The run loop keeps
    // painting on this thread between polls.
    loop {
        if settled.borrow().is_some() {
            break;
        }
        if opened.elapsed() > SETTLE_DEADLINE {
            bail!(
                "{label}: the capture window never reported a settled frame within {}s",
                SETTLE_DEADLINE.as_secs()
            );
        }
        cx.background_executor().timer(SETTLE_POLL).await;
    }
    settled
        .borrow_mut()
        .take()
        .expect("checked above")
        .with_context(|| format!("settle {label}"))?;

    // Frames drawn is not readiness. The preview's own screenshot path
    // learned this: a capture taken too early comes back at half the device
    // size because the window has painted but is not yet on the Retina
    // backing store.
    while opened.elapsed() < MIN_SETTLE {
        cx.background_executor().timer(SETTLE_POLL).await;
    }

    // Resolve the exact Window Server id from this GPUI window. Looking up
    // "the largest window owned by this pid" is racy in a batch: AppKit can
    // retain the just-closed previous window briefly, and a larger previous
    // scene then wins the lookup. The raw handle is GPUI's current NSView, so
    // its NSWindow number cannot name a sibling scene.
    let window_id = cx
        .update_window(window.into(), |_, window, _cx| window_server_id(window))
        .with_context(|| format!("read the AppKit window id for {label}"))??;

    // `screencapture` blocks; run it off the main thread so the run loop
    // keeps drawing and the foreground monitor keeps sampling.
    let png = cx
        .background_executor()
        .spawn(async move { capture_window(window_id) })
        .await
        .with_context(|| format!("capture {label}"))?;

    let (device_width, device_height) = png_dimensions(&png)?;
    verify_device_size(logical_width, logical_height, device_width, device_height)
        .with_context(|| format!("capture {label}"))?;

    // The non-activation claim is checked before anything is published: a
    // run in which the capture process itself became frontmost, or that
    // cannot show it did not, is not evidence. Unrelated operator
    // transitions are admissible and never fail the run — they are recorded
    // on the receipt as the run's own evidence.
    let foreground = monitor.evidence();
    match foreground.verdict {
        ForegroundVerdict::Proved => {}
        ForegroundVerdict::SelfFrontmost => bail!(
            "{label}: the capture process (pid {}) became the frontmost application (baseline \
             {:?}, observed {:?}) — the non-activating contract was violated and nothing was \
             published",
            foreground.capturer_pid, foreground.baseline, foreground.observed
        ),
        ForegroundVerdict::Unprovable => bail!(
            "{label}: the run cannot prove the capture process never became frontmost (baseline \
             {:?}, observed {:?}, {} successful samples, {MIN_FOREGROUND_SAMPLES} required, {} \
             failed readings). No evidence is not the same as evidence of no activation, so \
             nothing was published.",
            foreground.baseline,
            foreground.observed,
            foreground.samples,
            foreground.failed_reads
        ),
    }

    let facts = CaptureFacts {
        png,
        device_width,
        device_height,
        scale: ACCEPTED_SCALE,
        foreground,
    };
    finish(&facts).with_context(|| format!("publish {label}"))?;
    eprintln!("[{index}/{total}] {label}");

    // Close this window before the next one opens: one window at a time, for
    // the whole batch.
    cx.update_window(window.into(), |_, window, _cx| window.remove_window())
        .with_context(|| format!("close the capture window for {label}"))?;
    Ok(())
}

/// Run the scene's frame hook on the main thread once per painted frame until
/// it reports itself settled, then publish the outcome for the driver. The
/// hook must see the same painted frame the capture will.
fn schedule_settle(
    window: &mut Window,
    frame: u32,
    mut on_frame: FrameHook,
    settled: Rc<RefCell<Option<Result<()>>>>,
) {
    window.on_next_frame(move |window, cx| {
        let outcome = on_frame(window, cx, frame);
        let done = match outcome {
            Ok(Settled::Ready) => Some(Ok(())),
            Ok(Settled::Wait) if frame < MAX_SETTLE_FRAMES => None,
            Ok(Settled::Wait) => Some(Err(anyhow::anyhow!(
                "the scene never reported itself settled within {MAX_SETTLE_FRAMES} frames"
            ))),
            Err(error) => Some(Err(error)),
        };
        match done {
            Some(result) => *settled.borrow_mut() = Some(result),
            None => {
                window.refresh();
                schedule_settle(window, frame + 1, on_frame, settled);
            }
        }
    });
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

/// Read the Window Server id from the AppKit window backing this exact GPUI
/// window. The raw handle is an NSView pointer on macOS; following its
/// documented `window` relationship keeps capture bound to the current scene
/// even while an earlier batch window is still disappearing from the Window
/// Server.
fn window_server_id(window: &Window) -> Result<u64> {
    let handle = HasWindowHandle::window_handle(window)
        .map_err(|error| anyhow::anyhow!("read GPUI's raw window handle: {error:?}"))?;
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else {
        bail!("window capture expected an AppKit window handle on macOS");
    };
    let view_ptr = appkit.ns_view.as_ptr().cast::<objc2_app_kit::NSView>();
    // SAFETY: raw-window-handle's AppKit contract says `ns_view` points to
    // the live NSView backing this window. The borrowed GPUI Window keeps it
    // alive for the duration of this lookup.
    let view = unsafe { &*view_ptr };
    let ns_window = view
        .window()
        .with_context(|| "GPUI's NSView is not attached to an NSWindow")?;
    let id = ns_window.windowNumber();
    if id <= 0 {
        bail!("AppKit returned an invalid Window Server id {id}");
    }
    Ok(id as u64)
}

/// Capture one already-resolved Window Server id. Nothing here can fall back
/// to the desktop or a screen region: `screencapture -l` takes exactly one
/// window id.
fn capture_window(window_id: u64) -> Result<Vec<u8>> {
    let pid = std::process::id();

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

    fn samples(pairs: &[(&str, u32)]) -> Vec<ForegroundSample> {
        pairs
            .iter()
            .map(|(identity, pid)| ForegroundSample {
                identity: (*identity).to_string(),
                pid: *pid,
            })
            .collect()
    }

    /// A pid that is never the capture process's own in these tests.
    const CAPTURER_PID: u32 = 4242;
    /// Pids for the operator's own applications. None may equal
    /// `CAPTURER_PID`.
    const EDITOR_PID: u32 = 100;
    const BROWSER_PID: u32 = 200;
    const ENOUGH: u64 = MIN_FOREGROUND_SAMPLES;

    fn editor() -> Vec<ForegroundSample> {
        samples(&[("com.example.editor", EDITOR_PID)])
    }

    #[test]
    fn a_run_that_only_ever_saw_the_baseline_is_proof() {
        let editor = editor();
        assert_eq!(
            evaluate_foreground(
                CAPTURER_PID,
                editor.first(),
                &editor,
                ENOUGH,
                0
            ),
            ForegroundVerdict::Proved
        );
    }

    /// g17.003 — the operator stays free. Editor → browser → editor is
    /// ordinary work, never an activation by the capture process: the run is
    /// publishable and every transition stays recorded in `observed`.
    #[test]
    fn unrelated_frontmost_transitions_are_admissible_and_retained() {
        let observed = samples(&[
            ("com.example.editor", EDITOR_PID),
            ("com.example.browser", BROWSER_PID),
            ("com.example.editor", EDITOR_PID),
        ]);
        assert_eq!(
            evaluate_foreground(
                CAPTURER_PID,
                observed.first(),
                &observed,
                ENOUGH,
                0
            ),
            ForegroundVerdict::Proved
        );
        assert_eq!(observed.len(), 3);
        assert!(
            observed.iter().all(|sample| sample.pid != CAPTURER_PID),
            "no observed sample may be the capture process itself"
        );
    }

    /// The one transition that is never the operator's: the capture
    /// process's own pid appearing frontmost. One such sample is a typed
    /// failure no matter how clean the rest of the run was.
    #[test]
    fn a_later_self_frontmost_sample_is_a_typed_failure() {
        let mut observed = editor();
        observed.push(ForegroundSample {
            identity: "poodle-window-capture".to_string(),
            pid: CAPTURER_PID,
        });
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, editor().first(), &observed, ENOUGH, 0),
            ForegroundVerdict::SelfFrontmost
        );
    }

    /// A capture-process baseline — the capture pid frontmost BEFORE its
    /// first window — is a typed failure, never proof.
    #[test]
    fn a_capture_process_baseline_is_a_typed_failure() {
        let self_baseline = samples(&[("poodle-window-capture", CAPTURER_PID)]);
        assert_eq!(
            evaluate_foreground(
                CAPTURER_PID,
                self_baseline.first(),
                &self_baseline,
                ENOUGH,
                0
            ),
            ForegroundVerdict::SelfFrontmost
        );
    }

    /// The blocker g16.005 closed, unchanged: without a baseline the run
    /// watched nothing, and "nothing observed" must not read as "nothing
    /// happened". Both the empty and the non-empty case are unprovable, and
    /// NEITHER is `Proved`.
    #[test]
    fn an_absent_baseline_is_never_proof() {
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, None, &[], ENOUGH, 0),
            ForegroundVerdict::Unprovable
        );
        let editor = editor();
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, None, &editor, ENOUGH, 0),
            ForegroundVerdict::Unprovable
        );
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, None, &[], 0, 0),
            ForegroundVerdict::Unprovable
        );
    }

    /// A watch too short to mean anything is also not proof. A capture takes
    /// at least `MIN_SETTLE`, so a healthy run records many times this.
    #[test]
    fn too_few_samples_is_never_proof() {
        for samples in 0..MIN_FOREGROUND_SAMPLES {
            let editor = editor();
            assert_eq!(
                evaluate_foreground(CAPTURER_PID, editor.first(), &editor, samples, 0),
                ForegroundVerdict::Unprovable,
                "{samples} samples must not prove anything"
            );
        }
        let editor = editor();
        assert_eq!(
            evaluate_foreground(
                CAPTURER_PID,
                editor.first(),
                &editor,
                MIN_FOREGROUND_SAMPLES,
                0
            ),
            ForegroundVerdict::Proved
        );
    }

    /// A baseline that was read but never observed again: the monitor thread
    /// never got a reading, so there is nothing to stand on.
    #[test]
    fn a_baseline_with_no_observations_is_not_proof() {
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, editor().first(), &[], ENOUGH, 0),
            ForegroundVerdict::Unprovable
        );
    }

    /// g17.003 fail-closed rule: a tick the monitor could not read is a
    /// required reading that failed. Even with a clean baseline, plenty of
    /// samples, and no self-activation, one failed read makes the run
    /// unprovable — the monitor cannot claim to have watched every moment.
    #[test]
    fn a_failed_required_read_is_never_proof() {
        let editor = editor();
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, editor.first(), &editor, ENOUGH, 1),
            ForegroundVerdict::Unprovable
        );
    }

    /// A self-frontmost reading outranks a short watch: if the capture
    /// process WAS frontmost, that is the finding, not "we could not tell".
    /// The same precedence holds against a failed read and an absent
    /// baseline: self-activation is the answer even when the rest of the
    /// evidence is damaged.
    #[test]
    fn a_self_frontmost_sample_outranks_other_unprovable_causes() {
        let mut observed = editor();
        observed.push(ForegroundSample {
            identity: "poodle-window-capture".to_string(),
            pid: CAPTURER_PID,
        });
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, editor().first(), &observed, 1, 0),
            ForegroundVerdict::SelfFrontmost
        );
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, editor().first(), &observed, ENOUGH, 2),
            ForegroundVerdict::SelfFrontmost
        );
        assert_eq!(
            evaluate_foreground(CAPTURER_PID, None, &observed, ENOUGH, 1),
            ForegroundVerdict::SelfFrontmost
        );
    }

    /// The verdict is the receipt's field, so its wire form is part of the
    /// contract the TypeScript verifier reads.
    #[test]
    fn the_verdict_serialises_as_a_closed_lowercase_string() {
        let json = |v: ForegroundVerdict| serde_json::to_string(&v).expect("verdict serialises");
        assert_eq!(json(ForegroundVerdict::Proved), "\"proved\"");
        assert_eq!(json(ForegroundVerdict::SelfFrontmost), "\"selffrontmost\"");
        assert_eq!(json(ForegroundVerdict::Unprovable), "\"unprovable\"");
    }

    /// The receipt evidence is a closed shape: capturer pid, samples with
    /// identity AND pid, a failed-read count, and the verdict. Every field
    /// that makes the non-activation claim re-derivable must serialize under
    /// its contracted name.
    #[test]
    fn the_evidence_serialises_under_the_contracted_names() {
        let editor = editor();
        let evidence = ForegroundEvidence {
            capturer_pid: CAPTURER_PID,
            baseline: editor.first().cloned(),
            observed: editor,
            samples: ENOUGH,
            failed_reads: 0,
            verdict: ForegroundVerdict::Proved,
        };
        let json = serde_json::to_value(&evidence).expect("evidence serialises");
        let object = json.as_object().expect("evidence serialises as an object");
        for key in [
            "capturer_pid",
            "baseline",
            "observed",
            "samples",
            "failed_reads",
            "verdict",
        ] {
            assert!(object.contains_key(key), "missing receipt field '{key}'");
        }
        let baseline = object["baseline"].as_object().expect("baseline is an object");
        for key in ["identity", "pid"] {
            assert!(
                baseline.contains_key(key),
                "missing baseline field '{key}'"
            );
        }
        assert_eq!(object["capturer_pid"], CAPTURER_PID);
        assert_eq!(object["failed_reads"], 0);
        assert_eq!(object["verdict"], "proved");
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
