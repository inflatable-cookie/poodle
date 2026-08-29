# g16.030 — NumberInput Value, Draft, And Mounted Parity

Status: ready — decision approved; `g16.029` merged; worker handoff published
Opened: 2026-08-28
Depends on: merged `g16.021`, then completed `g16.029`; all three touch paired
core/headless exports and the shared domain-vector corpus
Governing refs: `../../contracts/components/number-input.md`,
`../../architecture/006-headless-core-and-machine-model.md`,
`../../contracts/001-working-rules.md`,
`../../triage/20260826-213343-number-input-native-value-model.md`

## Goal

Replace NumberInput's misleading number-or-string API and static native value
label with one paired numeric editing model. Give every active runtime the same
committed `number | null` value, optional raw-draft channel, validity rules,
step/precision behavior, commit boundary, and mounted editing result.

Make the clean pre-1.0 public break in the same tranche: delete string value and
constraint inputs, replace `onSubmit` with `onCommit`, remove
`onIncrement`/`onDecrement`, and add no aliases or fallbacks.

## Locked Public Surface

- Web committed inputs are `value?: number | null` and
  `defaultValue?: number | null`.
- Web raw editing is optionally controlled through
  `draftValue?: string | null` plus
  `onDraftValueChange?: (draft: string | null) => void`. Undefined means the
  adapter owns the draft; null means no override.
- Rust carries optional committed/default numbers plus
  `draft_value: Option<String>`; the GPUI host wrapper retains draft, selection,
  and focus between rebuilds.
- `min`, `max`, and `step` are optional numbers only. Omitted step is the
  documented default `1`; an authored invalid step is not silently replaced.
- `precision` is an optional non-negative integer that limits accepted
  fractional digits and fixes resolved display scale. It does not silently
  round an over-precision draft.
- Keep `prefix`, `suffix`, validation, standard control, size, density, and
  Recipe surfaces.
- Keep `onValueChange(number | null)`, `onValidationChange`, and web-only
  `onFocus`/`onBlur` observation.
- Add `onCommit(number | null)` for valid Enter/blur and successful step
  boundaries.
- Remove `onSubmit`, `onIncrement`, `onDecrement`, value-mode inference, and
  string coercion. Do not retain deprecated types, builders, reexports, or
  compatibility wrappers.

Exact Rust field/builder and pure-machine type names may follow local
conventions. They must preserve these distinctions and remain searchable in
the component contract and generated drift surfaces.

## Locked Semantics

- Portable direct syntax is finite signed base-10 decimal text with at most one
  period. No exponent, radix, grouping, locale separator, whitespace,
  `NaN`, or infinity syntax.
- Raw drafts such as `-`, `.`, `-.`, `1.`, leading-zero forms, and empty text
  remain exact on the draft channel.
- A complete draft emits a distinct committed value only when it satisfies
  syntax, precision, inclusive bounds, and step alignment anchored at `min` or
  zero.
- Empty reports raw `""` and committed `null`.
- Incomplete, malformed, over-precision, out-of-range, and off-step drafts
  stay visible and invalid while editing; they emit no committed value.
- Blur or Escape reverts an unresolved draft without a value or commit
  callback. Enter on an unresolved draft stays invalid and emits nothing.
- Valid Enter/blur fires `onCommit`; a successful Arrow/stepper action reports
  normalized draft, changed value, and commit. Home/End selects a finite valid
  bound and is otherwise inert.
- Direct editing never silently clamps, snaps, rounds, defaults to zero, or
  recovers an invalid authored configuration. Step controls stop before an
  invalid result.
- External controlled replacement discards an uncontrolled draft. A host echo
  of the value just emitted by the active edit does not erase the active draft.
- Disabled and read-only states are inert through text, pointer, key, clear,
  focus-mutation, and step routes.
- Async validation runs only on committed non-null canonical decimal strings;
  empty returns to idle.

## Shared Semantic Model And Vectors

Add idiomatic paired TypeScript/Rust forms of:

- numeric input configuration and validation;
- decimal draft classification and canonical parsing/formatting;
- committed value plus raw draft state;
- value, draft, commit, and invalid/no-op effects; and
- raw edit, clear, Enter, blur, Escape, step, Home/End, and external replacement
  events.

Extend `packages/contracts/headless/vectors/domain.json` with a bounded
`numberInput` corpus consumed by the existing TypeScript/Rust domain runners.
Cover at least:

- complete, incomplete, malformed, exponent, radix, whitespace, non-finite,
  leading-zero, trailing-period, and negative drafts;
- empty clear versus partial clear;
- omitted and authored invalid configuration;
- inclusive bounds and `min > max` rejection;
- whole and fractional steps anchored at zero and min;
- precision acceptance, rejection, and fixed display;
- valid live change, duplicate silence, commit, invalid blur/Escape, and stale
  controlled-value replacement;
- step/Arrow/Home/End behavior at and between bounds; and
- repeated clear/blur/Escape plus disabled/read-only event inertia.

Use decimal-safe shared normalization. Do not create an IR, generator, runtime
registry, specimen matrix, or second evidence ledger.

## Execution Plan

- [ ] **Batch 1 — paired semantics and vectors.** Land the pure TypeScript/Rust
      model and shared vector corpus before changing adapters.
- [ ] **Batch 2 — clean web migration.** Replace Svelte/React value unions,
      duplicated helpers, callback surface, and draft lifecycle; migrate
      Poodle-owned ColorPicker/FilterBuilder calls and focused tests.
- [ ] **Batch 3 — clean Rust migration.** Replace concrete/infinite-sentinel
      spec state and source-specific handlers; migrate poodle-render and every
      in-repository caller without an alias.
- [ ] **Batch 4 — mounted GPUI editor.** Route real replacement text,
      selection, focus, Enter/Escape, arrows, Home/End, and steppers through the
      Node/GPUI backend and specimen-owned rebuild state.
- [ ] **Batch 5 — specimens, migration evidence, and closeout.** Curate useful
      numeric, empty, precision, bounded, stepper, invalid-draft, and disabled
      examples; record downstream migration shapes and exact evidence in one
      August log.

## Acceptance Criteria

- [ ] TypeScript and Rust return identical classification, parsing, formatting,
      validity, stepping, draft, value, and commit effects for the shared
      corpus.
- [ ] Svelte and React expose only committed `number | null`, preserve exact
      controlled/uncontrolled drafts, and match callback timing through direct,
      clear, commit, revert, step, and replacement routes.
- [ ] No old string-value/constraint union, value-mode inference,
      string-coercion branch, `onSubmit`, `onIncrement`, or `onDecrement`
      remains on active public or internal NumberInput surfaces.
- [ ] `NumberInputSpec` represents empty committed state and raw draft without
      concrete-zero or infinite-bound sentinels.
- [ ] poodle-render declares one genuinely editable spin-button node; GPUI
      routes mounted text/key/focus/pointer dispatch and host rebuilds through
      the shared transition results.
- [ ] Named mounted tests prove valid direct editing, partial/invalid no-emit,
      empty clear, blur/Escape reversion, Enter commit, fractional step,
      precision, bounds, Home/End, controlled replacement, two-instance
      identity, and disabled/read-only inertia.
- [ ] Accessibility tests prove name, current value, optional bounds,
      unresolved invalid state, validation busy state, stepper labels/bounds,
      and one component focus treatment.
- [ ] ColorPicker and FilterBuilder remain green as regression consumers and do
      not carry local NumberInput fallback semantics.
- [ ] Human-facing Svelte, React, and GPUI specimens explain the useful modes
      without becoming exhaustive conformance tables.
- [ ] The migration evidence lists in-repository removals plus inspected
      sibling patterns for Acowtancy, Jetstream, and Underlay; sibling repos are
      not edited.
- [ ] Only NumberInput's GPUI mounted-behavior ledger cell moves from missing
      to mounted. If `g16.029` closes at the expected 48 / 126 totals, this card
      moves them to 49 / 125; accessibility and visual-comparison cells do not
      move.
- [ ] Known-delta totals stay at the contract-promotion baseline of 116 present
      / 59 not-applicable; this implementation card does not claim a second
      delta movement.
- [ ] Jetstream receives mechanical compile maintenance only and remains
      deferred and unclaimed.

## Writable Scope

- one focused NumberInput semantic module under `packages/core/src/`, its root
  export and focused tests;
- one focused Rust headless NumberInput module/export and focused tests;
- the `numberInput` section of the existing domain vectors and both existing
  domain-conformance runners;
- Svelte/React NumberInput implementations, public types, focused tests,
  styles only where required for semantic states, and curated specimens/docs;
- Poodle-owned Svelte/React ColorPicker and FilterBuilder NumberInput call sites
  and focused regression tests only;
- `packages/contracts/components/src/number_input.rs`, its crate export/tests,
  `packages/render/src/number_input.rs`, its crate export/tests, and exact
  in-repository NumberInput callers;
- Node vocabulary and GPUI backend/compatibility/specimen/regression files only
  for reusable text, selection, focus, role, bounds, and commit projection;
- mechanical Jetstream adapter/preview references required by the clean spec
  migration, with no behavior or evidence claim;
- NumberInput contract, drift/census surfaces, the one ledger cell/checker,
  this card, one August log, g16/front-door closeout, and `PAPERCUTS.md` only
  for new execution friction.

Do not edit TimeInput, EditableLabel, drag-and-drop semantics/adapters, other
component behavior, tokens except a proven existing semantic-token gap,
workflows, releases, versions, sibling repositories, or downstream consumers.

## Validation

Use Effigy selectors discovered after worker startup. At minimum:

- focused paired NumberInput model/domain-vector tests;
- focused Svelte and React NumberInput tests plus ColorPicker/FilterBuilder
  regression tests;
- focused poodle-specs, poodle-render, Node, and GPUI backend tests;
- named mounted NumberInput regressions and `effigy probe:gpui-specimens`;
- `effigy test:core`, `effigy test:components`, and
  `effigy test:contracts`;
- contract/callback/value-domain/capability drift checks and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, and
  `effigy docs:check`;
- one final headless `effigy qa`; and
- `git diff --check origin/main...HEAD` plus repository searches proving the
  removed public names and polymorphic branches are absent.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The approved numeric committed value, optional raw draft, invalid no-emit,
  revert, callback removal, or clean-break decision must change.
- Correctness requires locale-aware numbers, exponent syntax, a formatting
  callback, a raw string committed value, silent normalization, or a
  compatibility surface.
- Controlled web editing cannot distinguish a host echo from a genuinely
  external replacement without widening the approved public contract.
- GPUI cannot expose editable SpinButton text/selection/focus behavior through
  the active crates.io backend without a wider input or accessibility program.
- Work expands into TimeInput, EditableLabel, drag/drop, broad validation/IME,
  Jetstream admission, release, or sibling-repository migration.
- More than the exact NumberInput mounted ledger cell would move.

## Continuation

Return the paired API/model, vector results, clean-removal searches,
web/native mounted behavior, composite regressions, migration table, ledger
delta, validation, and August log to the orchestrator. Do not start
EditableLabel, another component, a drag card, or downstream migration. After
operator-authorized merge, the orchestrator chooses the next lane from
`g16.022` and the component-continuation runway.
