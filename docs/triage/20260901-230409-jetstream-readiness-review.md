# Jetstream Admission Readiness Review

Status: hold — readiness review complete; admission planning is not authorized
Captured: 2026-09-01
Source handoff: `docs/handoffs/20260901-230409-jetstream-readiness-review.md`
Promotion route: orchestrator review; no merge in this delegate lane

## Decision

Hold Jetstream admission planning.

The shared Rust/Node contract is mature enough to define a bounded pilot, and
the sibling runtime has a live AccessKit platform path. The admission boundary
is not ready, however. The live Node-to-Jetstream bridge drops most of the
interaction and accessibility fields that the Poodle contract exposes; the
runtime event vocabulary is narrower than the Poodle semantic event contract;
and no current same-case evidence closes those gaps. Those are the hold
reasons. A sanctioned paired checkout compiling tomorrow would not clear them.

The absent sibling checkout is recorded separately below as a validation and
environment limit. It is not a product defect or an admission-readiness defect.

This is a readiness hold, not a rejection of Jetstream as a future backend. It
preserves the existing program-level deferral and does not authorize a backend,
specimen, evidence, or ledger change.

Reviewed heads:

- Poodle reviewed base: `371591cce595dd843a93d45de204b5a4499fb0ae`
  (`origin/main` after rebase), pushed and clean before this packet revision.
- Jetstream sibling, read-only: `/Users/tom/Dev/projects/jetstream`,
  `ab6d2e6c82b54732c6bea4a61569c14a2a9a2991`, clean. The AccessKit adoption
  commit `7e997892b3d2ae5f90149d28957b3863f3a2d49c` is an ancestor.
- The expected paired checkout at
  `/Users/tom/.paseo/worktrees/1ugbsx1t/jetstream` is absent.

## Cross-repository authority split

Poodle owns the Node semantics, the renderer-neutral field matrix, adapter
expectations, admission criteria, and Poodle-owned preview and evidence
contracts. Poodle can inspect the sibling runtime read-only and state the
acceptance boundary for a future Jetstream admission programme.

Jetstream owns changes to its element vocabulary, `GameUi` input and event
delivery, AccessKit platform integration, and sibling releases. Those changes
must land through Jetstream-owned work and release controls. This Poodle packet
cannot authorize sibling mutations, and a Poodle worker must not implicitly
edit both repositories.

A future admission programme therefore needs a separately accepted Jetstream
work/handoff for sibling changes, alongside the Poodle-owned admission plan.
The two repositories can be tested together only after both scopes, exact
heads, and validation routes are accepted.

## Current contract and evidence

The working rules keep Svelte, React, shared renderer-neutral Rust, and GPUI
on one semantic contract. Jetstream remains a program-level deferred backend:
shared Rust composition and the in-repo adapter are maintained, but converter,
input, accessibility, preview, and visual evidence are required before backend
admission. The current Jetstream parity report says the same and forbids
promoting old g10 counts or treating shared Rust composition as backend
admission.

The current g16 ledger snapshot (updated 2026-08-26) records:

- 176 public Svelte surfaces and 175 portable native surfaces, with
  MeterSurface outside the denominator.
- Shared Rust present for 175 portable surfaces and GPUI construction focused
  for 175; these are shared/native construction measures, not Jetstream
  admission evidence.
- GPUI mounted evidence is a named bounded regression set: 56 mounted and 119
  missing, explicitly not a 175-component behavior pass.
- GPUI accessibility remains manual, and GPUI visual comparison remains a
  Button-only 18-fixture lane. Those are broader Poodle evidence limits, not a
  reason to manufacture Jetstream claims.
- Jetstream is one program-level deferred row. No Jetstream backend evidence
  or ledger movement is current.

### Poodle Node and render surface

`poodle-node` is renderer-neutral and declares the relevant interaction state
and intents: activation, text editing, submit/cancel, edit keys, selection,
focus changes, dragging, scrubbing, continuous values, wheel input, focus
requests, modified/context activation, dismissal/layer dismissal, key input,
and drag/drop source and target roles. The backend owns dispatch, hit testing,
focus, and capture; it must map these intents or declare a contract-owned
reason for an unsupported capability.

`NodeA11y` declares 17 fields: role, label, expanded, selected, tab index,
controls, labelled-by, orientation, toggled, level, value, minimum, maximum,
value text, invalid, busy, and described-by. This is a wider contract than the
currently projected Jetstream element model.

The shared renderer actively exercises these fields. Current examples include:

- Button: tab index, focusability, disabled state, and activation handler.
- Slider: role, label, value/range/value text, orientation, tab index, keyboard
  input, and scrub handling.
- TextInput: controlled change, selection, focus, submit/cancel, edit-key, and
  edit-insert handlers.
- Popover: trigger and surface roles, expanded state, controls relationship,
  focusability, activation, dismissal, and layer containment.

The direct Poodle Jetstream adapter is a separate legacy path. It reports 60
primitives, 48 composites, and 108 supported components, and
`effigy test:jetstream-adapter` passes 162 tests across 20 crates. Its manifest
and source explicitly do not claim whole-runtime parity. The 108 count and an
empty unsupported list therefore cannot stand in for the larger
renderer-neutral Node bridge or the 175-surface native construction denominator.

## Bridge audit

| Area | Poodle contract | Current Jetstream path | Readiness |
| --- | --- | --- | --- |
| Node kind, layout, style | Renderer-neutral Node data, layout, style, text, and animation inputs | `jetstream-poodle::to_js_el` performs a broad conversion and uses iterative post-order conversion; the shape is plausible but not a parity result | Partial; test field-by-field |
| Interaction | Activation, editing, focus, keyboard, scrub, wheel, dismissal, capture, and drag/drop intents | The bridge directly maps focusable, disabled, activation to click, and drag; sibling callbacks store click, pointer enter/leave, drag, and scroll. Editing, key, focus, scrub, continuous, dismissal, capture, modified/context, and drag/drop intent fields are not represented end to end | Blocker |
| Accessibility | 17 explicit NodeA11y fields plus native actions | The bridge maps role, label, expanded, selected, level, and toggled. It does not map tab index, relationships, orientation, value fields/value text, invalid, busy, or described-by. The sibling element model has some additional fields, but they are not supplied by this bridge | Blocker |
| Events | 12 `SemanticEvent` variants and nine component profiles, with an `EventSink` boundary | Sibling `UiEvent` has Activated, Cancelled, SliderChanged, TextChanged, FocusChanged, Hovered, and HoverLost. There is no current generic connection from Node interaction closures to the Poodle semantic event sink | Blocker |
| Keyboard and pointer | Backend dispatch must preserve focus, capture, edit semantics, and observable event order | `GameUi` handles navigation, text editing, mouse click, slider movement, and callback dispatch directly. Pointer click invokes the callback and emits an event; keyboard Confirm emits an event without the same callback path | Blocker |
| AccessKit platform | Tree/state projection and action round-trip must preserve the Node contract | The sibling runtime has a headless TreeUpdate path, `accesskit_winit` platform integration, and queued action requests. Current actions cover Click and Focus/Blur handling; exact Poodle field mapping and application event delivery remain unproved | Capability present; integration missing |
| Visual | Same named fixture and state must produce comparable output | The sibling snapshot tool is a headless wgpu triage path with quads/background/borders/focus and no glyph pass. No paired Poodle-to-Jetstream visual comparison exists | Missing |

The primary bridge defect is semantic loss, not merely a missing test. A
Poodle component can declare an intent that the current Jetstream element
cannot store, dispatch, or return to application state. Any pilot must make the
field matrix explicit and fail on an unhandled field.

## Runtime, dependency, and preview status

The current Poodle preview is not an independent Poodle-only runtime. Its live
route is:

`nel::El` / compatibility helpers → `jetstream_poodle::to_js_el` → sibling
`GameUi` / `UiElement` → sibling platform and renderer.

### Validation environment limit

The preview manifest resolves `jetstream-poodle`, `jetstream-ui`,
`jetstream-renderer`, `jetstream-platform`, `jetstream-input`, and
`jetstream-text` from `../../../../jetstream/crates/...`. In this worktree that
resolves under `/Users/tom/.paseo/worktrees/1ugbsx1t/jetstream`, which does not
exist. `cargo metadata --no-deps` can describe the manifest, but an actual
`cargo check --manifest-path packages/jetstream/preview/Cargo.toml` fails while
loading the missing `jetstream-input` manifest, before compilation.

This missing checkout is strictly a validation/environment limit. It prevents
this lane from running the paired preview checks; it is not a product defect,
not an admission-readiness defect, and not part of the hold rationale above. A
sanctioned paired checkout can remove this limit without changing the hold.

### Runtime and dependency posture

The relevant package configuration is Rust 1.95, edition 2024, wgpu 30, glam
0.29, and the current sibling workspace versions. The path dependencies are
not pinned to a sibling commit, so a paired build also needs an exact
workspace/head and lockfile record. Effigy’s dependency inventory is clean for
its managed dependencies (Bun only, zero missing or drifted entries); that does
not establish Cargo or sibling-runtime availability.

The generated preview catalogue contains 175 slug entries. The local registry
excludes 26 entries from its `has_specimen` metadata, but that is catalogue and
construction metadata only. It is not a parity or behavior count. The preview
compatibility layer retains old `js_*` call shapes, converts once at the shell,
and frequently passes `Handlers::default()` for specimens. The preview’s
application loop also routes selected events through manual token IDs. These
choices are useful for migration scaffolding but do not prove generic Poodle
behavior.

The current preview event loop does the following:

- keeps one persistent `InputSystem`;
- handles character, backspace/delete, mouse, wheel, focus, tab/tree drag, and
  slider paths in application/runtime code;
- handles only Activated, TextChanged, and SliderChanged from the returned
  `UiEvent` list, ignoring the other current variants; and
- calls the accessibility action handler but discards the `ui_events` it
  returns, so AT-generated component events do not reach application state via
  that list.

The sibling runtime’s AccessKit capability is real. The current native
accessibility contract records live `jetstream-ui::accessibility` projection,
`accesskit_winit` action routing, and a prior measured preview with 471 own UI
elements and 467 named through AXUIElement. That establishes platform
capability and historical runtime evidence; it does not establish a current,
paired Poodle preview or full Node field/action parity.

No `ci:jetstream`, preview, a11y, or visual admission result is claimed in this
packet. The adapter test is the only current Jetstream-specific execution
result: 162 tests passed.

## Existing drift exceptions and claim discipline

- `packages/jetstream/cross-runtime-parity-report.json` is the current
  Jetstream authority: status `deferred`, shared Rust `maintained`, adapter
  `maintained`, and backend evidence missing for converter/input,
  accessibility, preview, and visual comparison.
- `docs/contracts/003-native-accessibility.md` is the current native
  accessibility authority and records Jetstream AccessKit as live. The summary
  in `docs/contracts/README.md` still says neither native runtime exposes an
  accessibility API; that summary is stale and must not override contract 003.
- `docs/parity/jetstream-runtime-requirements.md` is historical. Its old
  runtime paths and no-accessibility conclusion predate the current
  Node/sibling bridge and AccessKit work; it is not current admission evidence.
- Preview a11y and snapshot comments still contain old 135/134/138 counts. The
  old g10 117/117 claim is explicitly forbidden by the current report. Neither
  count family is used here.
- The direct adapter’s 108 supported-component inventory is a legacy seam, not
  the live full preview route and not a backend admission denominator.
- Effigy doctor has pre-existing baseline findings for generated-in-source,
  god-file, stale-suppression, stale-graph, and comment-ratio scans. The
  existing `PAPERCUTS.md` also records the deferred-sibling role-census issue.
  These exceptions are not attributed to this packet and are not repaired in
  this packet-only delegate lane.

Claim rule: current reports, contracts, and named source behavior outrank old
counts and comments. A compiled catalogue, a direct adapter test, a shared Rust
construction count, or the existence of an AccessKit platform adapter cannot be
promoted to Jetstream parity without same-case interaction, accessibility, and
preview evidence.

## Candidate admission tranche

If the hold is cleared, use a serial pilot. This is a planning recommendation
only; it authorizes no implementation, specimen, backend, or evidence work in
this packet.

### Gate 0 — bridge acceptance harness

Establish a fixture-independent field matrix and a fail-closed conversion
harness. It must compare the Poodle Node declaration with the Jetstream element
and report every interaction and accessibility field as mapped, intentionally
unsupported with a contract-owned reason, or failed. This gate precedes any
component admission.

### Gate 1 — Button

Exercise name, role, disabled state, focusability, pointer activation, keyboard
activation, and an AccessKit click action. It is the smallest useful activation
and focus round-trip.

### Gate 2 — TextInput

Exercise controlled editing, caret and selection, focus gain/loss, submit,
cancel, paste/IME boundary, and text-change delivery. This exposes the largest
current interaction mismatch in a bounded surface.

### Gate 3 — Slider

Exercise pointer scrub and capture, keyboard movement including Home/End,
continuous versus committed changes, bounds, orientation, and accessibility
value fields.

### Gate 4 — Popover

Exercise trigger activation, open/close state, expanded and controls
relationships, outside/Escape dismissal, layer containment, and focus return.
Popover is preferred for the pilot because it exercises the generic dismissal
and layer fields directly.

Keep Tree/drag-drop, Select and other multi-selection composites, RangeSlider,
audio controls, broad composite sweeps, and animation/visual sweeps outside the
pilot until the four gates pass. Do not infer a program-wide admission from the
pilot; the working rules retain Jetstream as a program-level gate.

## Serial prerequisites

1. For future evidence execution, establish a sanctioned paired workspace that
   resolves the exact sibling paths, or provide the harness-managed equivalent.
   This is a validation prerequisite, not a reason for the present hold.
   Record the Poodle head, sibling head, Cargo.lock state, package versions,
   Rust toolchain, and wgpu / glam versions. Do not use an unrecorded symlink or
   an arbitrary sibling checkout.
2. Freeze a Node-to-Jetstream field matrix for every pilot fixture. Every
   declared interaction and `NodeA11y` field must map explicitly or carry a
   contract-owned unsupported reason; there must be no silent fallback or drop.
3. Jetstream-owned work must extend the sibling element vocabulary and bridge
   for the exercised intents: text edit/insert/select, submit/cancel, focus
   gain/loss, keyboard, scrub and continuous release/cancel, request-focus,
   overlay dismissal/layer ownership, and modified/context activation. Add
   drag/drop and wheel when those families enter the pilot. This packet does not
   authorize that sibling work; it requires a separately accepted Jetstream
   handoff.
4. Define the event boundary. Translate runtime input and callbacks into the
   Poodle semantic event/profile vocabulary or document the owning host sink.
   `UiEvent` names alone do not satisfy the `EventSink` contract.
5. The paired Poodle/Jetstream programme must prove stable runtime identity,
   focus traversal and tab index, focus requests, pointer capture, overlay
   containment, and focus restoration. Include event ordering in the trace, not
   only final values; assign each side's implementation to its owning repository.
6. Complete AccessKit field projection and action round-trip for role, names,
   state, relationships, orientation, values, invalid/busy status, and focus /
   activation actions. Verify that returned events reach application state.
7. Poodle-owned admission evidence must include a headless same-case harness
   with direct Node-to-JsEl assertions, GameUi input traces, and AccessKit
   TreeUpdate/action traces. Jetstream-owned runtime work must provide its own
   accepted validation handoff for the sibling portions. A preview route name,
   a generated slug, or a compile-only compatibility call is not a behavior
   proof.
8. Add visual evidence only after semantic, input, and accessibility gates:
   deterministic headless snapshots with a named fixture/state inventory and a
   comparison receipt. Any windowed native/accessibility diagnostic remains a
   separate operator-approved complement and is not run locally by this packet.
9. Replay the readiness review against the exact paired heads after the bridge
   changes. Recompute current catalogue/registry counts and replace stale
   comments before any admission recommendation is promoted.

## Evidence cost and sequence

| Cost | Evidence | Use |
| --- | --- | --- |
| Low | Source/manifest scans; Poodle Node/events/render/adapter tests; bridge field-matrix unit tests; direct role/name/value conversion assertions | Establish contract coverage and fail-closed conversion cheaply |
| Medium | Paired headless GameUi input traces; focus/capture/event-order traces; AccessKit TreeUpdate and action assertions; preview build/check; a11y role checks | Prove pilot behavior and application delivery |
| High | Full `ci:jetstream` / `qa:jetstream`; all-route census; offscreen wgpu visual sweep; platform AX smoke; exact cross-worktree provisioning and pinning | Establish program-scale confidence after the pilot, not before it |

Run the evidence in this order:

`semantic bridge → input/focus/capture → AccessKit projection/actions → preview
and a11y → visual`

Compilation and snapshots can support a gate, but neither can substitute for
behavior, event delivery, or native semantics.

## Risks

1. **Silent contract loss.** The current bridge maps only a small activation /
   focus / drag subset of the Node interaction surface and six of 17 NodeA11y
   fields. This is an architecture gap, not a coverage percentage.
2. **Callback and event asymmetry.** Pointer click invokes a sibling callback and
   emits `UiEvent`; keyboard Confirm emits an event through a different path.
   The preview then manually consumes only selected events, so identical user
   intent can produce different application delivery.
3. **Text and focus semantics.** TextInput editing, selection, IME/paste,
   focus-change handlers, and submit/cancel exist in shared render but are not
   carried by the current sibling callback model.
4. **Accessibility overclaim.** AccessKit platform support is present, but
   partial bridge fields and discarded AT event output can make a tree look
   healthy while actions and application state are not equivalent.
5. **Validation environment dependence.** The preview path is absent here, and
   sibling path dependencies are not commit-pinned. That limits local
   reproduction of paired evidence; it is an environment/validation limit, not
   a product or admission-readiness defect. A metadata pass or adapter test can
   remain green while this checkout cannot run the actual preview.
6. **Legacy-count confusion.** The 108 direct adapter inventory, 175 generated
   catalogue entries, nominal registry exclusions, and stale 135/134/138
   comments describe different layers. Mixing them would recreate the retired
   g10 claim pattern.
7. **Deferred drift.** Shared Node vocabulary can move while Jetstream is
   deferred. Exact-head replay and a maintained field matrix are needed to keep
   the boundary honest.
8. **Broader Poodle evidence limits.** GPUI mounted behavior, native
   accessibility, and visual comparison remain bounded in the current ledger.
   They should not be silently converted into Jetstream evidence or used to
   justify a Jetstream backend claim.

## Recommendation and promotion criteria

Recommendation: **hold**.

Do not reject the backend: the renderer-neutral Node contract, shared Rust
composition, direct adapter seam, and sibling AccessKit platform path provide a
credible foundation. Do not go: the paired preview does not build in this
checkout, the bridge and event contracts are materially incomplete, and no
current same-case Jetstream evidence exists.

Promotion to admission planning requires, at minimum:

- a reproducible paired workspace with exact heads and dependency records;
- a passing, fail-closed bridge matrix for Gate 0 and the four pilot surfaces;
- same-case keyboard, pointer, focus, capture, edit, dismissal, and event-order
  traces delivered to application state;
- AccessKit tree and action evidence covering the exercised semantics;
- a runnable preview/a11y path and then a named visual receipt; and
- an updated report that retains the program-level gate and does not revive old
  counts.

Until those conditions are met, keep the current Jetstream report at
`deferred`, preserve existing compatibility/compilation maintenance, and make
no specimen, backend, evidence, or ledger move.

## Settled decisions preserved

The handoff decisions are carried forward without reopening them:

- This work is a readiness review rather than continued silent deferral or
  immediate admission planning.
- Existing deferred Jetstream compatibility and compilation are preserved.
- No Jetstream specimen, backend work, evidence claim, or ledger movement is
  authorized by this packet.

There are no operator questions in this delegate lane. The exact sibling pin,
field mapping, event sink, and fixture identities are future gate requirements,
not reasons to re-ask the settled readiness-review choice.

## Non-goals

- No Jetstream backend or sibling-runtime implementation.
- No mutation of the Jetstream repository; sibling changes require a separately
  accepted Jetstream work/handoff.
- No Poodle specimen or preview rewrite.
- No new evidence claim, parity count, or ledger edit.
- No compatibility shim, alias, silent fallback, or breaking migration.
- No windowed conformance or native accessibility diagnostic.
- No merge or admission decision.

## Evidence index

Poodle authority and current posture:

- `docs/contracts/001-working-rules.md:64-70,103-115,154-162`
- `docs/contracts/003-native-accessibility.md:8-18,28-45,59-88`
- `docs/roadmaps/g16/parity-evidence-ledger.md:15-23,38-62,245-280`
- `packages/jetstream/cross-runtime-parity-report.json:3-34`
- `packages/gpui/cross-runtime-parity-report.json`
- `packages/gpui/native-accessibility-proof.json`

Poodle contracts and shared implementation:

- `packages/contracts/node/src/lib.rs:623-760,891-927`
- `packages/contracts/events/src/lib.rs:9-169`
- `packages/contracts/adapter/src/lib.rs:42-52,103-117`
- `packages/render/src/button.rs:286-293`
- `packages/render/src/slider.rs:99-124,201-302`
- `packages/render/src/text_input.rs:77-118,261-322`
- `packages/render/src/popover.rs:45-56,141-237`
- `packages/jetstream/adapter/src/lib.rs:126-197,199-277,313-320`

Poodle Jetstream preview and runtime wiring:

- `packages/jetstream/preview/Cargo.toml:15-44`
- `packages/jetstream/preview/src/specimens/mod.rs:342-360`
- `packages/jetstream/preview/src/main.rs:616-718`
- `packages/jetstream/preview/src/compat.rs:1-30`
- `packages/jetstream/preview/src/bin/a11y.rs:1-20,74-201`
- `packages/jetstream/preview/src/bin/snap.rs:1-9,393-425`
- `docs/parity/jetstream-runtime-requirements.md` (historical; not current
  authority)
- `docs/contracts/README.md:39-45` (stale summary; overridden by contract 003)

Jetstream sibling, inspected read-only at `ab6d2e6c82b54732c6bea4a61569c14a2a9a2991`:

- `/Users/tom/Dev/projects/jetstream/crates/jetstream-poodle/src/lib.rs:528-650`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-ui-element/src/callbacks.rs:1-22`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-ui-element/src/a11y.rs:9-62`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-ui-element/src/ui_element.rs:1151-1282`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-ui/src/input.rs:7-103`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-ui/src/game_ui_input.rs:3-235,253-343`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-ui/src/accessibility.rs:156-302`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-platform/src/accessibility.rs:1-120`
- `/Users/tom/Dev/projects/jetstream/crates/jetstream-platform/src/lib.rs:247-251,661-667,775-796`

Recent execution and decision logs:

- `docs/logs/2026-08/20260825-g16-023-jetstream-v022-adoption.md`
- `docs/logs/2026-08/20260825-g16-001-parity-evidence-ledger.md`
- `docs/logs/2026-08/20260826-g16-005-slider-axis-keyboard-and-mounted-parity.md`
- `docs/logs/2026-08/20260826-g16-006-tabs-drag-keyboard-and-mounted-parity.md`
- `docs/logs/2026-08/20260826-g16-007-text-input-controlled-editing-and-mounted-evidence.md`
- `docs/logs/2026-08/20260828-g16-021-drag-drop-semantic-kernel.md`
- `docs/logs/2026-09/20260901-g16-034-shared-motion-policy.md`
