# g15.044 — GPUI Offscreen Capture Feasibility

Status: **in flight** — PR #61; evidence complete, verdict `go`, awaiting
orchestrator/operator review
Parent: `012-visual-conformance-lane.md`
Depends on: `g15.001` (frozen active cohort)
Unblocks: `g15.045`
Governing refs: `../../roadmaps/g14/022-generation-closeout.md`,
`../../roadmaps/g14/conformance-estate.md`,
`../../contracts/001-working-rules.md`, `012-visual-conformance-lane.md`

## Problem

The current GPUI visual gate launches a real window and captures it through
macOS `screencapture`. It can take focus and makes the operator's machine
unusable. GPUI 0.2.2's in-memory `TestAppContext` proves construction,
geometry, and interaction, but its test platform does not expose raster
readback.

Newer upstream GPUI source contains offscreen scene rendering and image
readback. Longhorn's agent-control layer can drive and screenshot unfocused
Tauri webviews, but contract 022 explicitly cannot capture genuinely native
content. The release path therefore depends on proving a GPUI-native offscreen
pixel route, not on wrapping the existing windowed gate in Tauri.

## Goal

Reach a measured go/no-go decision for headless GPUI pixels without changing
Poodle's production dependency graph. Prove the smallest real Poodle primitive
possible, identify an exact upstream version/revision if an upgrade is needed,
and quantify the migration before authorising adoption.

## Scope

- Confirm the exact limitation of the current GPUI 0.2.2 test platform and the
  existing windowed capture path from source.
- Probe an exact candidate upstream GPUI revision in an isolated throwaway
  harness. Render at least one real Poodle Button scene to RGBA/PNG without an
  `NSWindow`, screen capture, focus, pointer movement, or Screen Recording
  permission.
- Record toolchain, Metal/device requirements, font/theme determinism,
  viewport/scale control, startup cost, and repeated-output stability.
- Measure the production migration surface: manifest/lock changes and compile
  failures across the GPUI adapter, node backend, preview, and headless tests.
- Inspect Longhorn contract 022 read-only and record its useful boundary:
  Svelte/React child webviews are controllable and composable; native GPUI is
  not visible until a native provider or GPUI-produced image exists.
- Produce one promoted capture-platform decision and update the parent. Do not
  build the full conformance lab or change the Poodle GPUI pin in this card.

## Acceptance

- [x] The report distinguishes layout/interaction headlessness from actual
      raster readback.
- [x] A successful result names an immutable GPUI revision, produces a real
      Poodle primitive PNG with no desktop focus/window, and gives a bounded
      adoption cost.
- [x] A failed result names the exact missing capability and leaves the native
      conformance lane blocked; it does not fall back to the windowed harness.
      *(Not reached — the result is a `go`. The windowed harness was neither
      run nor wrapped.)*
- [x] The Longhorn/Tauri lab is assessed as a control plane, not component
      authority or a Poodle package dependency.
- [x] No production package, public API, committed baseline, workflow, or
      sibling repository changes.

## Verdict — `go`

Evidence: [`../../research/gpui-offscreen-capture-feasibility.md`](../../research/gpui-offscreen-capture-feasibility.md),
log [`../../logs/2026-08/20260821-g15-044-gpui-offscreen-capture-feasibility.md`](../../logs/2026-08/20260821-g15-044-gpui-offscreen-capture-feasibility.md).
Reproduce with
[`reproduce.sh`](../../logs/2026-08/assets/g15-044/reproduce.sh) — the complete
recipe, asserting every claim; verbatim output retained as
[`receipt.txt`](../../logs/2026-08/assets/g15-044/receipt.txt).

**Current pin.** GPUI 0.2.2 has no raster readback by construction:
`PlatformWindow` declares no readback method, and
`platform/test/window.rs:269` is `fn draw(&self, _scene: &Scene) {}` — the test
window discards the scene. 0.2.2 is also the newest published version, so no
registry bump can supply this. The existing gate opens a real window, resolves
its own window id through `CGWindowListCopyWindowInfo`, forces frontmost via
`osascript`, and shells `screencapture`.

**Immutable candidate.**
`https://github.com/zed-industries/zed` @
`1ea16c1ab9dd6d36649e002dc60995634da04daf` (2026-08-21, Apache-2.0). Adds
`PlatformHeadlessRenderer`, `PlatformWindow::render_to_image`,
`HeadlessAppContext::capture_screenshot`, and `MetalHeadlessRenderer`. Its
`rust-toolchain.toml` pins 1.97.1, which the local toolchain already matches.

**Real-Poodle result.** `ButtonSpec` → `poodle_render::button` →
`poodle_gpui_node_backend::to_gpui` → `capture_screenshot` produces a 480×160
RGBA PNG (`../../logs/2026-08/assets/g15-044/button-offscreen.png`, SHA-256
`be94eace…`). No `NSWindow` exists at all — `TestPlatform::open_window` builds
an in-memory `TestWindow`, and `MetalRenderer::new_headless` is constructed
with `layer: None`, rendering into a private `MTLTexture`. No subprocess, no
permission, no focus.

**Repeatability.** 10 captures of identical input — 1 canonical, 5 in-process,
3 cross-process, 1 after a clean rebuild — are byte-identical: one SHA-256,
distinct hashes 1. Proved on one machine only; cross-machine font/GPU
reproducibility is explicitly not claimed.

**Migration cost.** 17 mechanical compile errors across 9 files, plus
`gpui_platform` as one new direct dependency. `poodle-gpui` needs zero changes
(it has no `gpui` dependency). Lock delta 704 → 702. The migrated disposable
copy passes `headless_regressions` 56/56. Upstream's new `BoxShadow.inset`
retires the standing `APPROXIMATION` in `node-backend/src/style.rs:300`.

**Measured constraints for `g15.045`.** `TestWindow::scale_factor` is hardcoded
`2.0`, so 1× captures need an upstream change or a local shim. The renderer is
macOS-only and needs a real Metal device (`create_device` exits on failure);
runner support is unverified. The zed git checkout is 474 MB. `render_to_image`
is behind `test-support`, so the capture target — not the shipping preview
binary — carries that feature.

## Continuation

The pin is unchanged in this branch, and `g15.045` is neither started nor
marked. It becomes eligible for planning only after the orchestrator verifies
this evidence and the operator accepts the `go`.

`g15.045` inherits three open items this card measured but does not decide:
whether a 2×-only lane is acceptable or the scale shim is required, whether the
project's macOS runners expose Metal, and whether captures run as a
long-running process (~15 ms marginal) or one process per fixture (~117 ms).

## Stop Conditions

- The probe needs a visible `NSWindow`, `screencapture`, or accessibility /
  screen-recording permission.
- The only working route requires a private platform API.
- The candidate GPUI migration cannot be bounded without first redesigning the
  renderer or node backend.
- The proof starts growing shared component fixtures or comparison semantics;
  those belong to later children.

## Writable Scope

- a bounded research note under `docs/research/`
- this parent/child runway and one August batch log
- optional retained proof fixture only if it is dependency-isolated and cannot
  enter package or release graphs
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- the exact isolated proof command recorded in the log
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Never run `*-windowed`, `test:native-visual`, Jetstream, or release selectors.
