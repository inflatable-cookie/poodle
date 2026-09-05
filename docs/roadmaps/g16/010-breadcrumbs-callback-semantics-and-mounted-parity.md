# g16.010 — Breadcrumbs Callback Semantics And Mounted Parity

Status: complete
Opened: 2026-08-27
Closed: 2026-08-27
Depends on: merged `g16.009` / PR #83
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/breadcrumbs.md`,
`parity-evidence-ledger.md`

## Goal

Make shared Rust Breadcrumbs obey the existing callback contract and prove
linkless crumb navigation through real mounted GPUI pointer and keyboard
dispatch.

The generated ledger moves Breadcrumbs GPUI mounted behavior from `missing`
to `mounted` (38 → 39 mounted, 136 → 135 missing). Recording the native
`href` inertness as a contract Known Delta also moves Breadcrumbs' known-delta
cell from `not-applicable` to `present` (114 → 115 present, 61 → 60
not-applicable). No other component's row changes.

## Current Evidence

- Svelte and React agree: non-current items with `href` are anchors;
  non-current linkless items are buttons whose activation calls `onNavigate`
  with the item's `value`; current items and the ellipsis are inert.
- `BreadcrumbsSpec` already carries separate `value` and `href` fields and has
  shared current-item and truncation helpers. No public model change is needed.
- `poodle-render::breadcrumbs` currently attaches its handler only to an
  `href` item and sends the URL. A linkless item is inert. This reverses the
  web contract.
- The renderer's focused test blesses that reversed behavior by constructing
  an icon-only item with `href` and expecting activation.
- The GPUI preview `Breadcrumbs` wrapper stores an `on_navigate` handler but
  exposes no builder for it. The specimen has no interactive readout.
- The component contract's strict checklist names the correct `value`
  callback, but its stale Rust-runtime note describes the wrong URL-driven
  behavior.
- The generated evidence ledger records focused web behavior and a GPUI
  specimen route, but no named mounted GPUI regression.

## Fixed Behavior Envelope

### Callback routing

- A crumb is callback-interactive only when all are true:
  - it is not current under `force_last_item_current`;
  - its value is not the synthetic ellipsis sentinel;
  - it has no `href`;
  - the host supplied `on_navigate`.
- Activating that crumb calls `on_navigate` exactly once with
  `BreadcrumbItem.value`, never its label or URL.
- An `href` crumb does not invoke `on_navigate`. Native URL routing remains a
  documented runtime delta until the shared node/backend boundary owns a real
  link channel.
- Current and ellipsis crumbs stay inert even when a handler exists.

### Native control shape

- Each callback-interactive crumb is one native target whether it is text
  only, icon plus label, or visually icon-only.
- It declares button semantics, a sequential focus stop, pointer cursor,
  accessible label from `BreadcrumbItem.label`, and the standard contracted
  focus ring.
- Icon-only crumbs keep the icon decorative and preserve the hidden semantic
  label on the target. Do not create separate icon and label hit targets.
- Non-interactive current, ellipsis, and `href` crumbs do not become button
  focus stops as a side effect of this card.

### Specimen and mounted proof

- Expose the existing GPUI compatibility wrapper's `on_navigate` builder.
- Make the Basic or Icons specimen demonstrate callback navigation with a
  compact visible readout. Keep the human-centered specimen structure; do not
  turn the page into a conformance matrix.
- Add one named mounted regression through the production renderer, node
  backend, focus chain, and event dispatch. It must prove:
  - pointer activation of a linkless text crumb emits its value once;
  - Enter or Space activation of a focused icon-only linkless crumb emits its
    value once and retains the authored accessible name;
  - current, ellipsis, and `href` items expose no callback activation;
  - the enabled callback targets declare the focus ring and the inert items do
    not become sequential stops.

## Explicit Non-Claims

- This card does not add native URL routing, a generic Link node, router
  integration, visited-link behavior, or browser-style anchor semantics.
- It does not change the public Svelte or React API or behavior.
- It does not redesign truncation, icons, size/density, wrapping, separators,
  or current-item rules.
- It does not promote GPUI assistive-technology coverage or visual comparison.
  The mounted test proves the node/backend interaction surface only.
- It does not admit Jetstream. Shared Rust compilation may be repaired if the
  corrected callback shape exposes an in-repo compile failure, but no
  Jetstream behavior or ledger claim moves.
- It does not include IconButton, EditableLabel, NumberInput, TimeInput, Pill,
  or any composite that consumes Breadcrumbs.
- It does not move any ledger row except Breadcrumbs. That row's mounted
  cell and known-delta cell both move.

## Delivery

### 1. Reconcile authority and focused tests

- Correct the stale runtime note in the Breadcrumbs contract and record the
  native `href` routing delta explicitly.
- Replace focused renderer tests that bless URL-driven callbacks with tests for
  linkless-value activation, current/ellipsis/`href` inertia, text/icon target
  unity, button semantics, focusability, label, and focus ring.
- The corrected callback and runtime semantics supersede the historical
  parity audit (`docs/archive/parity/breadcrumbs.md`); that audit stays
  untouched as evidence. Do not revive Jetstream as an active target.

### 2. Repair the shared renderer

- Route `on_navigate` by the fixed behavior envelope without changing
  `BreadcrumbsSpec`'s public fields.
- Apply semantics, focusability, cursor, accessible label, and focus ring to
  the same crumb node that owns activation.
- Reuse shared token resolution and the backend's existing focus-ring channel.
  Do not add a component-specific backend path.

### 3. Wire the GPUI specimen and mounted host

- Add the missing preview wrapper builder and a host-owned navigation readout
  to one existing example group.
- Keep specimen state outside the render-only contract and avoid global or
  cross-instance state.
- Add the named headless regression using real hit testing and keyboard
  dispatch. Assign fixture-local identities after rendering if the driver
  needs handles; do not add a public instance-id prop solely for the test.

### 4. Prove and close

- Regenerate the evidence ledger and verify only Breadcrumbs changes: 39
  mounted / 135 missing, and known-delta `not-applicable` → `present`.
- Mark the source triage note resolved, add one August execution log, close
  this card, and return g16 to an orchestrator evidence checkpoint. Do not
  compile or start `g16.011` in the worker thread.

## Acceptance

- [x] Linkless, non-current, non-ellipsis Rust crumbs invoke `on_navigate`
      exactly once with `BreadcrumbItem.value`.
- [x] `href`, current, and ellipsis crumbs never invoke the callback.
- [x] Text, icon-plus-label, and icon-only callback crumbs are single targets
      with button semantics, accessible label, sequential focus, and the
      contracted focus ring.
- [x] The GPUI specimen visibly demonstrates callback navigation without
      becoming an exhaustive fixture page.
- [x] One named mounted regression proves pointer and keyboard activation
      through production dispatch and proves inert crumbs remain inert.
- [x] Svelte and React Breadcrumbs focused tests stay green without behavior
      changes.
- [x] The contract and parity note no longer describe URL-driven Rust
      callbacks as parity.
- [x] The ledger changes only Breadcrumbs: mounted 38 → 39 / 136 → 135
      missing, and known-delta `not-applicable` → `present` (114 → 115
      present, 61 → 60 not-applicable).
- [x] One August log records the defect, repair, evidence, validation, exact
      non-claims, and next orchestrator checkpoint.

## Outcome

Complete. The full record is
`../../logs/2026-08/20260827-g16-010-breadcrumbs-callback-semantics-and-mounted-parity.md`.

## Writable Scope

- `docs/contracts/components/breadcrumbs.md`
- `packages/render/src/breadcrumbs.rs`
- Breadcrumbs-only compatibility/specimen state under `packages/gpui/preview/`
- the smallest Breadcrumbs mounted regression changes in
  `packages/gpui/preview/tests/headless_regressions.rs`
- focused Svelte/React Breadcrumbs tests only if a test-only correction is
  required; do not change web implementation behavior
- generated parity ledger/check surfaces only as required for the Breadcrumbs
  mounted and known-delta cells
- this card, its source triage note, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit other component contracts or implementations, shared node/backend
APIs, theme/token definitions, visual fixtures, accessibility reports,
versions, release metadata, workflows, downstream repositories, or sibling
runtime repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-render` Breadcrumbs tests;
- focused Svelte and React Breadcrumbs tests;
- the named mounted Breadcrumbs regression;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy drift:handlers` and `effigy drift:events`;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- The Svelte and React references disagree on callback payload or linkless,
  current, ellipsis, or `href` behavior.
- Correct callback navigation requires a public breaking change to
  `BreadcrumbsSpec`, a generic Link node, native router/URL API, or shared
  backend change.
- The focus ring cannot use the existing shared node channel without changing
  visual layout or other components.
- Honest mounted proof requires direct handler invocation, fixture-only
  behavior, or bypassing production focus/key dispatch.
- The ledger generator changes another row or promotes accessibility/visual
  evidence not proved by this card.
- Validation exposes an IconButton, EditableLabel, NumberInput, TimeInput,
  Jetstream, release, or downstream decision outside this runway.
