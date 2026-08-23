# g16.005 — GPUI crates.io recovery

Status: **ready**
Depends on: `g16.001`, completed crates.io non-activating capture prototype
Blocks: `g16.006` and every remaining `g16` adoption lane
Governing refs: `../../research/gpui-cratesio-nonactivating-capture.md`,
`../../research/gpui-offscreen-capture-feasibility.md`,
`../../contracts/001-working-rules.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../../AGENTS.md`

## Outcome

Restore crates.io GPUI source compatibility for consumers. Replace the
fork-only offscreen capture transport with an honestly named, non-activating
window capture tool, without changing component behaviour or admitting visible
capture to default QA.

This card proves the corrected source and capture boundary at the current
version. `g16.006` owns versioning and candidate preparation; `g16.007` is the
operator release gate.

## Failure Boundary

Published tag `v0.2.1` makes `poodle-gpui-node-backend` expose the
`inflatable-cookie/zed` GPUI crate identity. A consumer that also depends on
crates.io `gpui = "0.2.2"` receives two incompatible GPUI types. Longhorn's
GPUI prototypes proved the failure during `g16.002` adoption. The source
choice was made for an internal capture tool and must not remain part of the
public package boundary.

## Scope

- Restore normal and test GPUI dependencies in the node backend and preview to
  exact crates.io `gpui = "0.2.2"`.
- Remove the Poodle-owned Zed fork, `gpui_platform`, and every fork revision
  from active manifests and lockfiles.
- Reverse only the mechanical API migration introduced by `g15.045`; preserve
  all component, presentation, focus-ring, specimen, and interaction behaviour
  landed since then.
- Retain the in-memory GPUI construction, specimen, and interaction suites as
  the default headless native evidence.
- Replace `poodle-offscreen-capture` and its selector/schema language rather
  than leaving a compatibility alias. The new capture transport must use a
  real GPUI window with `focus: false`, never activate the application, and
  capture only its own window id.
- Preserve the accepted Button fixture inventory, typed capture receipts,
  repeated-capture integrity checks, and comparison policy. Rename any command
  that now requires a window so its execution cost is explicit.
- Keep window capture out of `qa`, release gates, CI, and ordinary worker
  validation. It remains an operator-approved local diagnostic.
- Add a deterministic source-policy check that rejects Git-sourced `gpui` or
  `gpui_platform` in Poodle's active package graph.
- Record one execution log that names the source-identity failure, mechanical
  API reversal, retained headless evidence, and window-capture limitations.

## Capture Contract

- Opening the capture window must not change the frontmost application or key
  window.
- No path may call `App::activate`, `makeKeyAndOrderFront`, System Events
  activation, or another focus-taking fallback.
- The implementation should reuse one bounded capture process/window for a
  fixture batch where practical; do not launch a focus-capable application per
  fixture.
- A missing window server or Screen Recording permission fails with a clear
  diagnostic. It never falls back to desktop or region capture.
- The receipt identifies the transport as windowed/non-activating and records
  crates.io GPUI 0.2.2. It must not claim offscreen or headless pixels.

## Acceptance

- [ ] Active Poodle manifests and locks contain one crates.io GPUI 0.2.2
      identity and no Git-sourced GPUI platform crates.
- [ ] A clean downstream proof can depend directly on crates.io GPUI 0.2.2 and
      Poodle's GPUI node backend without type-identity conflicts.
- [ ] Existing headless GPUI construction, specimen, interaction, focus-ring,
      and shared Rust tests remain green.
- [ ] The retained Button visual runner still produces its exact 18 GPUI
      fixtures and typed receipts when the operator explicitly runs the
      windowed diagnostic.
- [ ] A foreground-application monitor proves that diagnostic never takes
      focus. This one visual check is orchestrator/operator-owned; workers do
      not run a `*-windowed` selector without renewed approval.
- [ ] Default `effigy qa` and release gates open no window and require no
      Screen Recording permission.
- [ ] Full headless QA and a clean downstream dual-dependency compile proof
      pass without version or release mutation.

## Stop Conditions

- Restoring crates.io GPUI requires a public component or renderer contract
  change rather than mechanical API reversal.
- Any active dependency still resolves GPUI from a Git source or introduces a
  second GPUI crate identity.
- Pixel capture requires application activation, a modified GPUI source, a
  desktop-wide capture, or a silent fallback.
- A default QA, CI, or release path opens a window.
- Lockfile regeneration introduces unrelated dependency upgrades.
- Version mutation, release notes, candidate preparation, tag creation,
  publication, or workflow editing becomes necessary. Stop and return to
  `g16.006`.

## Validation

Use Effigy selectors discovered from the branch. At minimum:

- dependency/source policy and downstream dual-dependency compile proof;
- focused node-backend and GPUI preview tests;
- `effigy ci:native`;
- `effigy qa`;
- `effigy docs:check`;
- `git diff --check`.

Do not run any windowed selector in the worker. Record the exact command the
orchestrator should run after code review and before accepting the recovery.
