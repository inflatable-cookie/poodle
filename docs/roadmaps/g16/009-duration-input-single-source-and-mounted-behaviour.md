# g16.009 — DurationInput Single Source And Mounted Behaviour

Status: ready
Opened: 2026-08-27
Depends on: merged `g16.008` / PR #82
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/duration-input.md`,
`parity-evidence-ledger.md`,
`../../triage/20260827-094214-post-g16-008-native-lane-decision.md`

## Goal

Make the three duration segments the single Rust value authority, align the
portable spec with the Svelte-reference contract, and prove the resulting
editing behaviour through real mounted GPUI input and focus dispatch.

This is an operator-approved pre-1.0 breaking migration. Remove the duplicate
formatted-value and caller-supplied-validation paths cleanly. Do not keep
aliases, synchronization fallbacks, or deprecated builders.

The generated ledger may move exactly one cell: DurationInput GPUI mounted
behaviour from `missing` to `mounted`, taking the totals from 37 to 38 mounted
and 137 to 136 missing.

## Current Evidence

- The component contract exposes `hours`, `minutes`, and `seconds` as the
  controlled value. It does not expose a formatted `value` prop or a
  caller-owned validation-state prop.
- `DurationInputSpec` currently carries both segment fields and
  `value: Option<String>`. The renderer displays and edits the formatted value,
  so a host must keep both representations synchronized manually.
- Rust defaults `show_seconds` to `false`; the contract and both web runtimes
  default `showSeconds` to `true`.
- Rust stores `min_total_seconds` and `max_total_seconds`, but the renderer
  ignores them when choosing the invalid border. A separate
  `validation_state` field controls that presentation.
- The shared headless duration machine already owns carry, borrow, digit shift,
  clamping, and swallowed carry at `max_hours`, but its total and the renderer
  callback use `u32` while the spec's total uses `u64`.
- `g16.008` established real segment tab stops and proves
  `H → M → S → out`. Its routing test deliberately duplicates the formatted
  value and segment fields and does not claim full DurationInput mounted
  behaviour.
- Svelte and React focused tests already agree on carry, borrow, digit entry,
  bounds, `showSeconds`, and disabled behaviour.

## Fixed Behaviour Envelope

### One value authority

- `DurationInputSpec.hours`, `.minutes`, and `.seconds` are the only controlled
  duration value in Rust.
- Remove `DurationInputSpec.value`, `with_value`, `validation_state`, and
  `with_validation_state`. Do not replace them with aliases or silent parsing.
- Format each displayed segment from the numeric fields with the shared
  padding rule. `show_seconds` controls only whether Seconds is rendered; it
  does not erase the stored seconds value or change callback payload shape.
- Default `show_seconds` to `true`, matching the contract and web reference.
- Keep segment values within the existing contract domains: Hours is bounded by
  `max_hours`; Minutes and Seconds use `0..=59` after an edit transition.

### Derived totals and validation

- Compute total seconds from the segment fields using a non-overflowing `u64`
  path. Align the shared headless helper and native callback payload with that
  type.
- Invalid presentation is derived from the current total:
  `total < min_total_seconds` or, when a maximum exists,
  `total > max_total_seconds`.
- Bounds are presentation/validation constraints, not edit clamps. Segment
  transitions keep their existing carry, borrow, digit, and `max_hours` rules;
  the host still receives the actual edited total.
- Zero remains valid at the default minimum. Inclusive endpoints are valid.

### Mounted native behaviour

- Drive real GPUI focus and key dispatch through a host that rebuilds from only
  the three segment fields.
- Prove Hours is the entry stop; Tab and Shift+Tab traverse the visible
  segments in order and then leave the component.
- Prove ArrowUp carry, ArrowDown borrow, digit-shift entry, `max_hours`
  swallowing/clamping, and exact `{hours, minutes, seconds, total}` callback
  values after host rebuilds.
- Prove `show_seconds=false` produces only Hours and Minutes tab stops while
  retaining Seconds in stored state and callback totals.
- Prove disabled DurationInput exposes no editable segment stops and emits no
  change.
- Keep `g16.008`'s generic Tab-versus-submit and adjacent text-routing
  regressions green.

## Explicit Non-Claims

- This card does not add native IME, free-form text parsing, selection ranges,
  a new editor abstraction, or a backend-specific duration widget.
- It does not change Svelte or React public behaviour, specimen design, visual
  comparison, broad accessibility status, or GPUI assistive-technology claims.
- It does not include NumberInput, TimeInput, EditableLabel, IconButton,
  DatePicker, or other date/time composites.
- It does not admit Jetstream. In-repo Jetstream callers must compile against
  the corrected shared spec, but no Jetstream preview, QA, parity, or ledger
  claim is promoted.
- It does not move any ledger row except DurationInput's GPUI mounted-behaviour
  cell.

## Delivery

### 1. Align contract and Rust spec

- Amend the DurationInput contract's Rust/GPUI notes where they still describe
  the stale formatted-value shape. Do not change the public web contract.
- Remove the two non-contract value/validation fields and builders from
  `DurationInputSpec`; set the correct default; add focused spec tests for the
  value, total, bounds, and default rules.
- Change shared duration totals and the native change callback to `u64` without
  changing segment types or transition semantics.

### 2. Render from segments

- Make `poodle-render` format, edit, and validate from one `DurationValue`
  built from the spec's segment fields.
- Retain the existing shared headless carry/digit functions rather than
  reimplementing them in the renderer or fixture.
- Add focused renderer tests for display source, default Seconds, derived
  invalid state, endpoints, and callback totals.

### 3. Migrate in-repo callers

- Replace every DurationInput `with_value(...)` and explicit validation-state
  construction with `with_segments(...)` plus bounds where required.
- Update GPUI preview/specimen state so interaction writes the three segment
  values directly. Keep its human-facing specimen structure unchanged.
- Update deferred Jetstream in-repo fixtures only as required to compile from
  the shared spec. Do not widen into Jetstream behaviour work.
- Delete stale comments and tests that describe the dual-source model.

### 4. Prove and close

- Replace the routing-only DurationInput fixture with, or add beside it, one
  named mounted regression covering the fixed behaviour envelope through
  production focus/key dispatch.
- Regenerate the evidence ledger and verify only DurationInput changes:
  38 mounted / 136 missing.
- Mark the source triage note resolved, add one August execution log, close this
  card, and return g16 to an orchestrator evidence checkpoint. Do not compile or
  start `g16.010` in the worker thread.

## Acceptance

- [ ] Rust has one duration value authority: segment fields only; the formatted
      value and caller-validation fields/builders are gone.
- [ ] `show_seconds` defaults to `true` across contract, Rust, and both web
      references.
- [ ] Display strings, edit transitions, callback payloads, totals, and bounds
      validation derive from the same segment value.
- [ ] Total calculation and the native callback use `u64`; large valid Hours
      values cannot overflow the total path.
- [ ] Inclusive min/max endpoints are valid; totals outside either bound render
      invalid without clamping the edited value.
- [ ] Every in-repo Rust caller uses the corrected spec; no compatibility shim
      or duplicate synchronization remains.
- [ ] One named mounted GPUI regression proves carry, borrow, digit entry,
      max-hours behavior, visible-segment traversal, callback totals, and
      disabled inertia through production dispatch and host rebuilds.
- [ ] Existing Svelte and React DurationInput tests remain green and unchanged
      unless a test-only correction is required to state existing behaviour.
- [ ] The ledger changes only DurationInput from missing to mounted: 38 mounted
      / 136 missing.
- [ ] One August log records the migration, evidence, validation, exact
      non-claims, and the next orchestrator checkpoint.

## Writable Scope

- `docs/contracts/components/duration-input.md`
- `packages/contracts/components/src/duration_input.rs`
- `packages/contracts/headless/src/duration.rs` and its focused conformance
  vectors/tests only where the total type changes
- `packages/render/src/duration_input.rs`
- DurationInput-only in-repo callers under `packages/gpui/` and
  `packages/jetstream/`
- DurationInput focused Svelte/React tests only if needed to preserve existing
  reference evidence; do not change web implementation behavior
- the smallest mounted regression changes in
  `packages/gpui/preview/tests/headless_regressions.rs`
- generated parity ledger/check surfaces only as required for the one-cell move
- this card, its source triage note, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit other component contracts or implementations, specimens outside
DurationInput, theme/token files, visual fixtures, accessibility reports,
versions, release metadata, workflows, downstream repositories, or sibling
runtime repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-specs`, `poodle-headless`, and `poodle-render` tests covering
  DurationInput and duration transitions;
- focused Svelte and React DurationInput tests;
- the named mounted DurationInput regression plus retained `g16.008` routing
  tests;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
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

- The Svelte and React references disagree on the segment value, default,
  bounds, or callback rules named here.
- A non-DurationInput consumer relies semantically on the removed formatted
  value or caller-validation field rather than merely constructing the stale
  spec.
- The migration needs a compatibility alias, silent fallback, broad component
  API change, new native editor, GPUI patch/fork, or application-owned state in
  Poodle.
- Honest mounted proof requires direct handler invocation, fixture-only state,
  or bypassing production focus/key dispatch.
- The ledger generator changes another row or promotes accessibility/visual
  evidence not proved by this card.
- Validation exposes a NumberInput, TimeInput, EditableLabel, date-picker,
  Jetstream, release, or downstream decision outside this runway.
