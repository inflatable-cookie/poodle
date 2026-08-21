# g15.045 GPUI Offscreen Capture Adoption

Date: 2026-08-21
Card: `../../roadmaps/g15/045-gpui-offscreen-capture-adoption.md`
Parent: `../../roadmaps/g15/012-visual-conformance-lane.md`
Proof: `../../research/gpui-offscreen-capture-feasibility.md` (g15.044, verdict `go`, PR #61)
Handoff: `../../handoffs/20260821-215028-g15-045-gpui-offscreen-capture-adoption.md`
Worker branch: `t3code/gpui-offscreen-capture-adoption`

## Outcome

The proved offscreen GPUI raster seam is adopted. `gpui = "0.2.2"` is replaced
by the immutable revision `zed-industries/zed@1ea16c1ab9dd6d36649e002dc60995634da04daf`
in `poodle-gpui-node-backend` and `poodle-gpui-preview`, with `gpui_platform`
(`font-kit` enabled) as the preview's one new normal dependency. The measured
8 + 6 + 3 mechanical migration is applied unchanged, with one deliberate
correction: the proof patch's placeholder `inset: false` became a truthful
projection of `shadow_layers[*].inset`, and the gpui 0.2.2 "inset layers are
dropped" approximation is deleted.

A new internal smoke command, `poodle-offscreen-capture`, renders a real Poodle
Button through `ButtonSpec` → `poodle_render::button` →
`poodle_gpui_node_backend::to_gpui` → GPUI `HeadlessAppContext` Metal readback
and writes a PNG plus a typed JSON receipt. It runs under the Effigy selector
`smoke:gpui-offscreen-capture`, which captures identical input repeatedly in a
temporary directory, asserts one hash, verifies each receipt against its PNG,
and exercises every negative case. No window, focus, desktop permission,
baseline, or fixture namespace is involved.

## Dependency and Package Changes (spec 022)

- **Changed packages:** `poodle-gpui-node-backend` (public-intent, preview
  channel) and `poodle-gpui-preview` (internal tooling). No version bumps;
  pre-1.0 baseline unchanged.
- **Public-entry-point impact:** none. No public API was added, removed, or
  changed. The capture binary is an internal smoke target behind a
  non-default feature, not a library surface.
- **Class:** behavioral — same component semantics, but the rendered pixels of
  GPUI output are now produced by the adopted upstream revision, and inset
  shadow layers now render where they were previously dropped.
- **Dependencies:** 10 published crates.io `gpui*` packages are replaced by 23
  packages from the pinned zed git source (lock 704 → 700 packages).
  `gpui_platform` (with `font-kit`) is the one new normal dependency;
  `image` and `sha2` are optional dependencies enabled only by the `capture`
  feature and were already in the graph transitively. The zed git checkout
  (~474 MB in `~/.cargo/git`) is a new CI/machine cost recorded by g15.044.
- **MSRV:** unchanged. Public `rust-version = "1.95"` metadata stands; the
  focused build, headless regressions, and capture selector pass under
  `RUSTUP_TOOLCHAIN=1.95.0` (see below). Upstream's own 1.97.1 toolchain pin
  was not copied into Poodle metadata.
- **Downstream re-check:** GPUI consumers rebuild against the pinned revision;
  nothing else. Web packages, contracts, and tokens are untouched.
- **`test-support` isolation:** verified, not inferred — `cargo tree
  -e normal` on the preview manifest contains zero test-support/proptest
  crates without the `capture` feature, and the ordinary
  `cargo build --bin poodle-preview` builds without it. `test-support` is
  enabled only by dev tests and by the required feature of the capture
  binary.

## Capture Command Contract

`poodle-offscreen-capture` requires explicit `--out`, `--receipt`, `--width`,
`--height`, `--theme`, `--control-size`, and `--scale`. Scale accepts exactly
`2.0` (the adopted revision's `TestWindow::scale_factor` is hardcoded 2.0; no
local shim exists). Theme and control size validate against the preview's
single domain authority — `src/presentation_axes.rs`, extracted from
`app_state.rs` so the interactive preview and the capture target share one
enumeration; `app_state` re-exports it. Unknown values are rejected, never
silently defaulted. All validation happens before renderer construction, and
unsupported OS or a missing Metal device is an explicit failure, never a
green skip.

The receipt (`poodle.gpui-offscreen-capture.v1`) records the component smoke
identity, the immutable GPUI revision, renderer/platform, theme, control size,
logical viewport, scale, device dimensions, and the PNG's SHA-256 — no
timestamps, no machine-specific paths. PNG and receipt are fully staged in
distinct sibling temporary files. Any prior final receipt is invalidated
before the PNG is published, and the new receipt is published last. A failed
or interrupted publish can therefore leave a PNG without a receipt (a failure
by contract), but never stale matching-looking success evidence. Colliding
PNG/receipt destinations are rejected before renderer construction.

## Validation

All on the worker worktree, macOS with a real Metal device:

| Check | Result |
| --- | --- |
| `cargo build --bin poodle-preview` (ordinary, no `capture`) | builds; `test-support` absent from the normal feature graph |
| `cargo build --bin poodle-offscreen-capture --features capture` | builds |
| `poodle-gpui-node-backend` tests | 24/24, including the two new inset-shadow projection tests |
| capture bin unit tests | 10/10 (argument validation, output collision, stale-receipt invalidation, revision-constant drift check) |
| `effigy smoke:gpui-offscreen-capture` | all checks pass — 3 repeated captures at one hash, receipts verified, 7 negative cases fail loudly |
| `effigy regressions:native` | 56/56 |
| `effigy probe:gpui-specimens` | 8/8 |
| `effigy check:gpui` | pass |
| `effigy ci:native` | pass |
| `effigy docs:check` | pass |
| `RUSTUP_TOOLCHAIN=1.95.0` focused build + regressions + capture smoke | pass |
| `git diff --check origin/main...HEAD` | clean |

### Smoke evidence

Three captures of identical input (240×80 logical, theme `default`, control
size `md`, scale `2.0`), all byte-identical:

```
be94eaceb6c310c4e067c012b579c53d2c6d4147fc63160673316538c9997c6d
```

This equals the g15.044 proof's canonical hash for the same scene — the
production seam reproduces the measured result exactly on this machine. Device
dimensions are 480×160 (logical × 2). The selector was run twice from the
worktree with identical results. Negative cases exercised through the selector:
scale `1.0` rejected, unknown theme rejected, unknown control size rejected,
missing `--receipt` rejected (and no PNG written), colliding PNG/receipt paths
rejected before writes, a forced PNG publish failure invalidated a seeded stale
receipt, and a tampered PNG was detected by the receipt check. Generated
captures lived in a temporary directory and are not in Git.

Cross-machine byte identity is not claimed: glyphs come from the host Core
Text stack and rasterisation is device-dependent. Renderer-aware tolerance is
`g15.047`'s, as the lane already states.

## Scope Notes

- The windowed capture scripts and `test:native-visual` remain local-only
  historical tooling; this card neither ran nor touched them.
- No named fixtures, baselines, tolerances, image comparison, or
  cross-runtime claims — those belong to `g15.046`/`g15.047`.
- No workflow, release, or version changes. Card/roadmap status is the
  orchestrator's to update.
