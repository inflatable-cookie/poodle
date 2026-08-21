# GPUI Offscreen Capture Feasibility

Status: promoted decision — **go**
Created: 2026-08-21
Card: `../roadmaps/g15/044-gpui-offscreen-capture-feasibility.md`
Parent: `../roadmaps/g15/012-visual-conformance-lane.md`
Log: `../logs/2026-08/20260821-g15-044-gpui-offscreen-capture-feasibility.md`
Reproduction: `../logs/2026-08/assets/g15-044/reproduce.sh` (complete recipe)
Receipt: `../logs/2026-08/assets/g15-044/receipt.txt` (verbatim run output)

## Question

Can Poodle read genuine GPUI pixels without a visible window, desktop capture,
focus theft, or a Screen Recording permission — and what would adopting that
route cost?

This is a raster-readback question, not a headlessness question. GPUI 0.2.2
already gives in-memory construction, layout, geometry, and interaction
evidence through `TestAppContext`. That proves nothing about pixels. The two
claims are separate and were kept separate throughout.

## Verdict

**Go.** A real Poodle Button renders to a deterministic RGBA PNG entirely
offscreen at an exact upstream revision, and the production migration is
bounded at **17 mechanical compile errors across 9 files, plus one added
dependency**, with the retained headless regression suite passing 56/56 after
the migration.

Everything below is reproduced end-to-end by
`../logs/2026-08/assets/g15-044/reproduce.sh`, which contains the complete
harness source, both manifests, the deterministic migration patch, and every
command. The verbatim output of a full clean run is retained as
`../logs/2026-08/assets/g15-044/receipt.txt`.

The script is also the **verifier** for this note. Its `EXPECT` block holds
every durable structural claim below, and it asserts all of them — 25 checks
covering the three per-surface error counts and their total, both lock counts,
the regression pass and fail counts, the equal-input capture count, the
distinct-hash count, the canonical SHA-256, the canonical and viewport PNG
dimensions read from the IHDR chunk, the committed evidence PNG's hash and size,
and that the viewport captures differ from the canonical one and from each
other. It also asserts that every literal migration replacement matched before
applying it, so a silently-skipped patch fails rather than passing. It reports
*all* drift in one run rather than stopping at the first, and exits non-zero.

Timings are recorded but deliberately **not** asserted: they are machine- and
load-dependent, and a verifier that fails on them would be noise.

An earlier revision of this note claimed the script asserted its claims when it
checked only two dependency strings and one image hash — it reported success
while the regression count had drifted. That gap is closed, and the gate was
negative-tested: seeding two wrong `EXPECT` values produced exit 1 with both
mismatches named.

The production pin is unchanged by this card. `g15.045` remains the only
adoption authority and needs operator review of this verdict first.

## Current Pin: What 0.2.2 Cannot Do

Pin: `gpui = "0.2.2"` from crates.io (checksum `979b45cfa6ec723b6f42330915a1b3769b930d02b2d505f9697f8ca602bee707`)
in `packages/gpui/node-backend/Cargo.toml` and `packages/gpui/preview/Cargo.toml`.
0.2.2 is the newest published version; the crates.io index lists
`0.1.0`, `0.2.0`, `0.2.1`, and `0.2.2` and nothing above it. So no readback
route exists on the registry at all — any capability here is git-only.

Two source facts settle the limitation:

- `PlatformWindow` in `gpui-0.2.2/src/platform.rs` has no readback method of
  any kind. `draw(&self, scene: &Scene)` is one-way; nothing returns an image.
- `gpui-0.2.2/src/platform/test/window.rs:269` is
  `fn draw(&self, _scene: &crate::Scene) {}`. The test window discards the
  scene. There is no texture, no atlas upload, no pixels to read.

`App::headless()` (`gpui-0.2.2/src/app.rs:143`) prevents opening windows. It
does not add rendering; on Linux its own client answers
`"Headless mode does not support screen capture."`

### The current windowed gate

`packages/gpui/preview/src/main.rs:2434-2470` opens a real window, shells out
to `swift -e` to find its own window id through
`CGWindowListCopyWindowInfo`, then runs `screencapture -x -l <wid>`.
`packages/gpui/preview/scripts/capture-window.swift` is the same shape and
additionally drives `osascript` System Events to set the process frontmost
before `screencapture -R`.

That path needs a visible `NSWindow`, takes focus, and needs Screen Recording
permission. It is evidence to replace, not a fallback.

## Candidate: Exact Immutable Upstream Revision

**`https://github.com/zed-industries/zed` @ `1ea16c1ab9dd6d36649e002dc60995634da04daf`**
(committed 2026-08-21, licence Apache-2.0 on `crates/gpui`).

This is a commit SHA, not a branch. It was resolved through the GitHub API and
independently re-fetched by Cargo into `~/.cargo/git`; the proof did not read
any pre-existing local checkout. No upstream code was copied into Poodle.

The revision's `rust-toolchain.toml` pins channel `1.97.1`. The local
toolchain is already `rustc 1.97.1 (8bab26f4f 2026-07-14)` — exact match, no
toolchain change needed.

### What it adds

| Symbol | Location | Purpose |
| --- | --- | --- |
| `PlatformWindow::render_to_image(&Scene) -> Result<RgbaImage>` | `crates/gpui/src/platform.rs:986` | window-level readback, `#[cfg(test-support)]` |
| `trait PlatformHeadlessRenderer` | `crates/gpui/src/platform.rs:992` | `render_scene_to_image` / `render_scene` / `sprite_atlas` |
| `HeadlessAppContext` | `crates/gpui/src/app/headless_app_context.rs` | `with_platform(text_system, assets, renderer_factory)`, `open_window`, `capture_screenshot` |
| `MetalHeadlessRenderer` | `crates/gpui_apple/src/metal_renderer.rs:1601` | macOS implementation |
| `gpui_platform::current_headless_renderer()` | `crates/gpui_platform/src/gpui_platform.rs:85` | platform-neutral factory |

At this revision the framework is also split into `gpui`, `gpui_apple`,
`gpui_macos`, `gpui_linux`, `gpui_windows`, `gpui_web`, `gpui_wgpu`,
`gpui_platform`, and support crates. `crates/gpui` still declares version
`0.2.2` while depending on unpublished workspace siblings, which is why the
capability cannot arrive through a registry bump.

## The Proof

### Scene

One real Poodle Button through the real path, no plain GPUI rectangle and no
hand-built element tree:

```text
ButtonSpec + GpuiThemeProvider
  -> poodle_render::button(...)            (poodle-render, the single Rust impl)
  -> poodle_node::Node
  -> poodle_gpui_node_backend::to_gpui(..) (the production GPUI backend)
  -> gpui::AnyElement
  -> HeadlessAppContext::capture_screenshot -> RgbaImage -> PNG
```

Harness root — the scene under test. This is an excerpt for reading; the
complete compiling source, both manifests, and the setup commands are in
`reproduce.sh`:

```rust
struct ProofRoot;

impl Render for ProofRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = GpuiThemeProvider::new();
        let spec = ButtonSpec::new()
            .with_label("Save")
            .with_variant(ButtonVariant::Primary);
        let node = poodle_render::button(&spec, &theme, None);
        poodle_gpui_node_backend::reset_element_ids();
        let element: AnyElement = poodle_gpui_node_backend::to_gpui(&node);
        div().size_full().p(px(16.0)).bg(gpui::rgb(0xffffff)).child(element)
    }
}

let platform = gpui_platform::current_platform(true);          // headless
let text_system = platform.text_system();
let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(()), || {
    gpui_platform::current_headless_renderer()
});
let window = cx.open_window(size(px(240.0), px(80.0)), |_, cx| cx.new(|_| ProofRoot))?;
cx.run_until_parked();
let image = cx.capture_screenshot(window.into())?;             // RgbaImage
image.save(out)?;
```

Dependencies that make the above compile, from the harness manifest:

```toml
gpui = { git = "https://github.com/zed-industries/zed",
         rev = "1ea16c1ab9dd6d36649e002dc60995634da04daf",
         features = ["test-support"] }
gpui_platform = { git = "https://github.com/zed-industries/zed",
                  rev = "1ea16c1ab9dd6d36649e002dc60995634da04daf",
                  features = ["test-support", "font-kit"] }
```

### Result

`docs/logs/2026-08/assets/g15-044/button-offscreen.png` — 480×160 RGBA,
SHA-256 `be94eaceb6c310c4e067c012b579c53d2c6d4147fc63160673316538c9997c6d`.
A primary Button with the resolved fill token, corner radii, real Core Text
glyph shaping, and antialiased edges. Rendered pixels, not a geometry dump.

### No window, no capture, no permission — by construction

This was proved from source, not by opening a window and looking:

- `HeadlessAppContext` is backed by `TestPlatform`, whose `open_window`
  (`crates/gpui/src/platform/test/platform.rs:399`) constructs a `TestWindow`.
  A `TestWindow` is an in-memory struct. No `NSWindow` is ever created, so
  `show: false` / `focus: false` in `HeadlessAppContext::open_window` are
  belt-and-braces rather than the mechanism.
- `TestWindow::render_to_image` (`platform/test/window.rs:410`) forwards to
  the headless renderer and never touches a window server.
- `MetalRenderer::new_headless` (`gpui_apple/src/metal_renderer.rs:184`) is
  constructed with `layer: None` — there is no `CAMetalLayer`.
- `render_scene_to_image` (`metal_renderer.rs:569`) allocates a private
  `MTLTexture` (`BGRA8Unorm`, `RenderTarget | ShaderRead`, `Managed`) as the
  render target, commits, waits, and reads the texture back.
- The harness runs no subprocess. No `screencapture`, no `osascript`, no
  `CGWindowListCopyWindowInfo`, no AXUIElement, no `scap`. The
  `screen-capture` cargo feature is off.
- `gpui_platform::current_platform(true)` is called only to obtain
  `MacTextSystem`. `MacPlatform::new` (`gpui_macos/src/platform.rs:197`)
  builds a dispatcher, text system, executors, and pasteboard handles. It does
  not create an `NSApplication`, register with the window server, or activate.
  `run()` is never called.

### Repeatability

**Ten** captures of identical input (240×80 logical, default arguments), all
byte-identical:

| Set | Captures | Files |
| --- | --- | --- |
| canonical single capture | 1 | `button.png` |
| successive captures inside one process | 5 | `rep.0.png` … `rep.4.png` |
| separate process invocations | 3 | `proc1.png` … `proc3.png` |
| after `cargo clean` and a full rebuild | 1 | `after-clean.png` |
| **total** | **10** | — |

All ten: `be94eaceb6c310c4e067c012b579c53d2c6d4147fc63160673316538c9997c6d`.
**Distinct hashes: 1.**

The set is enumerated explicitly in `reproduce.sh`, never by a glob, so the
deliberately-different viewport captures below cannot leak into it. The script
fails if the distinct-hash count is not exactly 1. Per-file hash output is in
`receipt.txt`.

Independent confirmation: a full clean run of `reproduce.sh` in a fresh
`mktemp -d` workspace produced that same hash, matching the PNG committed to
`docs/logs/2026-08/assets/g15-044/button-offscreen.png`.

Scope of that claim: one machine, one OS, one GPU, one font set. It proves the
pipeline is not frame-timing- or GC-sensitive. It does **not** prove
cross-machine reproducibility — see the caveats below.

### Viewport and scale

Viewport is fully controllable: the logical size passed to `open_window`
determines the output. 240×80 → 480×160, 320×120 → 640×240, 160×48 → 320×96.
These are different inputs and hash differently, as they should
(`a4ffa571…` and `56f775b1…`); they are excluded from the equal-input set
above by construction.

Scale is **not** controllable at this revision.
`TestWindow::scale_factor` (`platform/test/window.rs:227`) is a hardcoded
`2.0`. Device pixels are always logical × 2. A 1× capture needs an upstream
change or a local platform shim. This is a bounded, measured constraint, not
an unknown; it does not block a 2×-only comparison lane.

Window appearance is likewise fixed — `TestPlatform::window_appearance` returns
`WindowAppearance::Light`. That is harmless here: no Poodle package reads
`WindowAppearance`. Theme selection is entirely `GpuiThemeProvider`-driven, so
dark and custom themes are controllable through the existing provider.

### Cost

Measured on an Apple M5 Max (40-core GPU), macOS Darwin 25.5.0, SDK 26.5,
debug profile:

| Measure | Value |
| --- | --- |
| headless context construction | 36–63 ms |
| first capture (shader/pipeline warm-up) | 26–52 ms |
| subsequent captures in-process | ~15.5 ms |
| whole process, cached build, one capture | ~117 ms |
| clean rebuild of the proof binary | 26–28 s (3 measurements) |

One earlier ad-hoc measurement of the clean rebuild read 74.6 s. It did not
reproduce; three consecutive measurements under `reproduce.sh` give 26–28 s,
and that is the figure to trust. Both are recorded rather than the inconvenient
one being dropped.

A long-running capture process amortises well: after the first frame the
marginal cost is ~15 ms. That answers the sidecar-lifecycle question in the
Longhorn-lab triage note in favour of one long-running process over one
process per fixture, though the lab still owns that decision.

## Production Migration Cost

Measured by repointing disposable copies of the real crates at the pinned
revision and compiling. Nothing was written into the worker checkout, and the
production manifests and lockfile are untouched.

Each surface is measured **before its own patch is applied** and patched before
the next is measured, because they depend on each other. `reproduce.sh` step 6
performs exactly that sequence, prints every error line, and asserts each count
against the table below; `receipt.txt` holds the verbatim output. The patch
itself is in the same script, as deterministic literal string replacements with
an occurrence assertion on each, so the 17-error figure and the 56/56 run can
both be recreated without guessing what was changed — and a replacement that
silently stops matching fails the run instead of quietly reducing the cost.

| Surface | Errors | Files |
| --- | --- | --- |
| `poodle-gpui` (adapter) | **0** | — (no `gpui` dependency at all) |
| `poodle-gpui-node-backend` | 8 | 4 |
| `poodle-gpui-preview` (bin) | 6 | 4 |
| `poodle-gpui-preview` headless tests | 3 | 1 |
| **Total** | **17** | **9** |

Every error is a mechanical signature change. There is no renderer redesign,
no node-backend redesign, and no architectural work:

| Upstream change | Sites | Fix |
| --- | --- | --- |
| `FocusHandle::focus(window)` → `focus(window, cx)` | 4 | thread `cx` |
| `Styled::flex_grow()` → takes an `f32` | 5 | `flex_grow_1()` |
| `BoxShadow` gains required `inset: bool` | 4 | set the flag |
| `ScrollHandle::max_offset()` returns `Point` not `Size` | 1 | `.height` → `.y` |
| `Line::paint` gains `align: TextAlign, align_width: Option<Pixels>` | 1 | pass `TextAlign::Left, None` |
| `KeyDownEvent` gains `prefer_character_input: bool` | 1 | set `false` |
| `Application::new()` removed | 1 | `gpui_platform::application()` |

The `BoxShadow` change is a capability *gain*. `node-backend/src/style.rs:300`
currently carries `APPROXIMATION: gpui 0.2.2 BoxShadow has no inset flag, so
inset (highlight) layers are dropped`. Upstream now supports inset shadows, so
adoption can delete that approximation rather than port it.

### Behavioural evidence

The suite counts **56** at this branch's base, and the count is base-dependent:

| Base | Count | What moved it |
| --- | --- | --- |
| `eb4bc165` (card start) | 53 | — |
| `dd00ab26` | 54 | `ae838e67` — g15.041 Popover interactive triggers |
| `5e72e2d5` (current) | 56 | `408577ab` — g15.042 Stepper native interaction parity |

This is a sibling-lane coupling, not instability in the migration: each rebase
that picks up a new native regression moves the number. `reproduce.sh` asserts
the exact count, so the drift fails the run and forces this table to be updated
rather than letting a stale figure sit in the docs. Both earlier readings were
correct at their base; only the current one is claimed here.

`cargo test -p poodle-gpui-preview --test headless_regressions` on the
migrated copy: **56 passed, 0 failed** in 0.54 s. The retained native
regression suite — focus handles, mounted-window flows, roving focus, real
input — survives the migration unchanged.

### Dependency and lock delta

- Lock package count: **704 → 702**. Essentially flat.
- 10 published `gpui*` crates.io packages are replaced by **23 packages from
  the zed git source**: `collections`, `derive_refineable`, `gpui`,
  `gpui_apple`, `gpui_linux`, `gpui_macos`, `gpui_macros`, `gpui_platform`,
  `gpui_shared_string`, `gpui_util`, `gpui_web`, `gpui_wgpu`, `gpui_windows`,
  `http_client`, `media`, `perf`, `refineable`, `scheduler`, `sum_tree`,
  `util_macros`, `zlog`, `ztracing`, `ztracing_macro`.
- One new direct dependency: `gpui_platform` (needed for both
  `current_headless_renderer()` and the `Application::new()` replacement).
- `render_to_image` is `#[cfg(any(test, feature = "test-support"))]`, so the
  capture target needs `gpui/test-support`. That feature also pulls
  `leak-detection`, `proptest`, and `collections/test-support`. Acceptable for
  a capture binary or test target; it should not enter the shipping preview
  binary.

### Adoption friction to plan for

- The zed git checkout is **474 MB** in `~/.cargo/git/checkouts`. Any CI lane
  adopting this clones the whole Zed repository for one framework crate.
- `MetalHeadlessRenderer` is **macOS-only**. `current_headless_renderer()`
  returns `None` everywhere else, and `render_to_image` then bails. Poodle's
  GPUI target is already macOS, so this constrains where captures can run, not
  what they cover.
- A **real Metal device is required**. `MetalRenderer::create_device`
  (`metal_renderer.rs:189`) calls `std::process::exit(1)` when no compatible
  device is available. Whether the project's macOS runners expose Metal is
  unverified and belongs to `g15.045`.
- `MacTextSystem` is `pub(crate)` in `gpui_macos`. The only supported public
  route to it is `gpui_platform::current_platform(true).text_system()`, which
  constructs a `MacPlatform` purely to borrow its text system. It works and
  opens nothing, but it is an awkward seam worth watching upstream.
- `HeadlessAppContext` sets `GpuiMode::test()` and uses `TestDispatcher`.
  Timers are simulated, so animation-dependent visual states need
  `advance_clock` rather than sleeping. Good for determinism; it must be
  designed into the fixture lane.

## Environmental Caveats

Stable rendering and reproducible-anywhere rendering are different claims, and
only the first was proved.

Glyphs come from the host's Core Text stack through `MacTextSystem`, and the
scene resolves system fonts. The same Poodle scene can therefore differ across
macOS versions, font availability, and font-smoothing settings. GPU rasterisation
is also device-dependent in principle; the readback path even branches on
unified memory (`metal_renderer.rs:596`), and only the unified-memory branch was
exercised here.

Consequence for `g15.047`: native baselines need renderer-aware tolerance and a
recorded capture environment. They are not safe as strict byte comparisons across
machines. That was already the lane's stated posture; this proof does not soften
it.

## Longhorn Boundary — Unchanged

Longhorn contract 022 was inspected read-only at
`/Users/tom/Dev/projects/longhorn/docs/contracts/022-agent-app-control.md`.
The Longhorn checkout was not modified.

What it gives: stateless MCP control of an unfocused Tauri app; semantic
snapshot/click/type/press/scroll/drag per opted-in webview; window-composed
screenshots that work occluded, unfocused, and minimized, with no screen
recording permission and no private API; explicit window sizing.

What it explicitly does not give, quoted from the contract:

> a genuinely native (non-webview) surface does not appear in the image — the
> core crate exposes a provider seam so such a surface can later register its
> own snapshot and action handlers, and no provider ships under this contract.

So the assessment in the runway-recompile log holds unchanged. Longhorn is a
**control plane** for Svelte and React webviews. It is not a native pixel
source, not component or behaviour authority, and not a Poodle package
dependency. This card supplies the missing half: a GPUI sidecar can now return
real pixels over local IPC, which is exactly the provider-seam shape contract
022 anticipates.

## What This Does Not Decide

- It does not change the production pin. `g15.045` owns adoption.
- It does not create named fixtures, a comparator, tolerances, or a scene
  language. `g15.046` and `g15.047` own those.
- It does not build the Tauri conformance lab, and does not make it a v0.2.0
  prerequisite.
- It does not revive the rejected g14 executable corpus in any form.
