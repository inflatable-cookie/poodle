# g15.044 — GPUI Offscreen Capture Feasibility

Status: **ready** — independent research/proof lane; may run beside `g15.041`
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

- [ ] The report distinguishes layout/interaction headlessness from actual
      raster readback.
- [ ] A successful result names an immutable GPUI revision, produces a real
      Poodle primitive PNG with no desktop focus/window, and gives a bounded
      adoption cost.
- [ ] A failed result names the exact missing capability and leaves the native
      conformance lane blocked; it does not fall back to the windowed harness.
- [ ] The Longhorn/Tauri lab is assessed as a control plane, not component
      authority or a Poodle package dependency.
- [ ] No production package, public API, committed baseline, workflow, or
      sibling repository changes.

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
