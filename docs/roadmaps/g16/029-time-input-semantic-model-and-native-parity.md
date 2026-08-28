# g16.029 — TimeInput Semantic Model And Native Parity

Status: planned — decision approved; promote after g16.021 merges
Opened: 2026-08-28
Depends on: approved `docs/contracts/components/time-input.md`; merged
`g16.021` before dispatch because both lanes edit shared core/headless exports
Governing refs: `../../contracts/components/time-input.md`,
`../../architecture/006-headless-core-and-machine-model.md`,
`../../contracts/001-working-rules.md`,
`../../triage/20260828-224148-time-input-native-editing-decision.md`

## Goal

Replace TimeInput's unconstrained native text substitute with one paired pure
TypeScript/Rust time-entry model, align the Svelte and React native-input
adapters to the same commit boundary, and mount a real segmented GPUI editor.
Preserve the public canonical string value while making partial drafts,
constraints, stepping, clearing, controlled replacement, focus, and
accessibility predictable across the active cohort.

Cleanly rename the legacy Rust `TimeFieldSpec` / `time_field` surface to
`TimeInputSpec` / `time_input` in the same pre-1.0 tranche. No compatibility
alias or dual path remains.

## Locked Semantic Surface

Implement paired idiomatic TypeScript and Rust forms of:

- `TimeParts`: hour, minute, and second;
- `TimeInputConfig`: optional minimum/maximum plus positive whole-second step;
- canonical parse/format and seconds-visibility functions;
- syntax, bound-membership, and step-alignment validation;
- forward/backward configured-step movement over linear and overnight ranges;
- draft state carrying visible segment text separately from the last committed
  canonical value; and
- pure events/transitions for segment digits, segment clearing, whole-control
  clearing, step up/down, blur, Escape, and external committed-value
  replacement.

Exact names may follow local conventions, but both languages must expose the
same distinctions and run the same shared cases. The machine emits a resulting
canonical value or `null` only when the contract says `onValueChange` fires.
It performs no focus, callback, drawing, timer, locale, or platform I/O.

## Locked Behavior

- Canonical values remain zero-padded 24-hour `HH:MM` or `HH:MM:SS` strings.
- Seconds are visible when step is below 60 seconds or any authored
  value/default/bound includes seconds.
- Step is a positive whole number of seconds, anchored at `min` or midnight.
- Unbounded stepping wraps midnight. Linear bounds stop at their endpoints.
  `min > max` is an overnight range and stepping crosses midnight without
  entering the excluded gap.
- Complete direct edits commit live only when syntax, bounds, and step align.
- Partial, out-of-range, and off-step drafts remain adapter-owned and emit no
  value. They expose invalid state while focused and revert on blur or Escape.
- Clearing every segment emits `null`; clearing one segment creates a local
  incomplete draft.
- External controlled replacement discards the local draft.
- Disabled state is inert through every pointer, key, text, and focus route.
- Svelte/React retain native `input[type=time]`; GPUI uses one visual group with
  labelled hour/minute/conditional-second `SpinButton` segments.
- Custom native Tab/Shift+Tab traverse visible segments, then leave. Digit
  entry edits the focused segment. Arrow Up/Down move the time by configured
  step. Escape restores without emitting.

## Shared Vector Corpus

Add a bounded `timeInput` section to
`packages/contracts/headless/vectors/domain.json` and run it through the
existing TypeScript/Rust domain-conformance runners. Cover at least:

- canonical parsing and rejection of malformed, non-padded, impossible, and
  fractional-second values;
- format with and without seconds;
- seconds visibility from step, value, default, min, and max;
- unbounded wrap, linear clamp, overnight wrap, and excluded-gap rejection;
- step anchoring at midnight and at min, including off-grid direct entry;
- partial digit drafts, complete valid commits, invalid drafts, blur and Escape
  reversion;
- one-segment clear versus whole-control clear;
- controlled replacement during a draft; and
- repeated blur/Escape/clear and disabled-event inertia.

This extends the existing hand-authored domain fixture. Do not create an IR,
generator, runtime registry, specimen matrix, or second evidence ledger.

## Execution Plan

- [ ] **Batch 1 — paired model and vectors.** Land the TypeScript/Rust pure
      semantics and shared domain cases without touching adapters.
- [ ] **Batch 2 — web commit boundary.** Route Svelte and React native input,
      validity, clearing, blur, and controlled replacement through the shared
      semantics; keep browser picker presentation native.
- [ ] **Batch 3 — clean Rust rename.** Rename the declaration/module/renderer
      and every in-repository caller, compatibility surface, census, and
      generated inventory from `TimeField` to `TimeInput`. Add no alias.
- [ ] **Batch 4 — mounted GPUI editor.** Render and dispatch the segmented
      control through Node and the GPUI backend, including focus traversal,
      digit/arrow/clear/Escape routes, invalid presentation, and accessibility
      projection.
- [ ] **Batch 5 — specimens and closeout.** Curate default, bounded, seconds,
      overnight, invalid-draft, and disabled examples; record exact evidence
      and honest non-claims in one August log.

## Acceptance Criteria

- [ ] TypeScript and Rust return identical parse, format, validation, stepping,
      draft, and effect results for the shared corpus.
- [ ] Svelte and React emit only canonical constraint-valid values or `null`;
      invalid/off-step native values do not leak through `onValueChange`.
- [ ] GPUI presents separately focusable labelled segments inside one labelled
      visual control and routes real mounted key/text/focus dispatch through the
      shared machine.
- [ ] Named mounted tests prove valid live commit, partial/invalid no-emit,
      blur/Escape reversion, clear, step, linear and overnight bounds,
      conditional seconds, controlled replacement, Tab traversal, and disabled
      inertia.
- [ ] `TimeInputSpec` / `time_input` are the only public Rust names; every
      in-repository composite and compatibility caller is migrated and no
      `TimeFieldSpec`, `time_field` module, alias, or fallback remains.
- [ ] Human-facing specimens demonstrate the useful states without becoming an
      exhaustive conformance listing.
- [ ] Only TimeInput's GPUI mounted-behaviour ledger cell moves from missing to
      mounted, taking the live total from 47 / 127 to 48 / 126 if the named
      mounted proof passes. Accessibility and GPUI visual-comparison cells do
      not move.
- [ ] Jetstream receives mechanical compile maintenance required by the clean
      renderer-neutral rename only; it remains deferred and unclaimed.

## Writable Scope

- one focused TypeScript module under `packages/core/src/`, root export,
  focused tests, and the existing domain-conformance runner;
- one focused Rust headless module/export, focused tests, and the existing
  Rust domain-conformance runner;
- `packages/contracts/headless/vectors/domain.json` for `timeInput` cases;
- TimeInput Svelte/React implementations, tests, types, styles, and curated
  specimens;
- `packages/contracts/components/src/time_field.rs` renamed to
  `time_input.rs`, its crate exports/tests, `packages/render/src/time_field.rs`
  renamed to `time_input.rs`, and all exact in-repository Rust callers;
- Node vocabulary and GPUI backend/compatibility/specimen/regression files only
  for the segmented editor's reusable focus, text, role, and invalid-state
  projection;
- mechanical Jetstream adapter/preview references needed by the clean rename,
  with no new behavior or evidence claim;
- TimeInput contract, parity/census references, ledger/checker for the one
  mounted-cell move, this card, one August log, g16/front-door closeout, and
  `PAPERCUTS.md` only for new execution friction.

Do not edit drag-and-drop semantics/adapters, NumberInput, EditableLabel,
DurationInput behavior, DateTimePicker family behavior beyond mechanical
TimeInput renames, tokens except a proven existing semantic token gap,
workflows, package versions, releases, sibling repositories, or downstream
consumers.

## Validation

Use Effigy selectors discovered after worker startup. At minimum:

- focused paired time-model/domain-vector tests;
- focused Svelte and React TimeInput tests;
- focused poodle-specs, poodle-render, Node, and GPUI backend tests;
- named mounted TimeInput regressions and `effigy probe:gpui-specimens`;
- `effigy test:core`, `effigy test:components`, and
  `effigy test:contracts`;
- contract/callback/value-domain/capability drift checks and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`, and
  `effigy docs:check`;
- one final headless `effigy qa`; and
- `git diff --check origin/main...HEAD` plus a repository search proving the
  removed legacy Rust names are absent from active surfaces.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The approved public string, draft/commit, revert, step, overnight, seconds,
  or clean-rename decision must change.
- The native browser control cannot preserve the portable callback boundary
  without replacing it with a custom web editor.
- GPUI cannot expose the segment focus/text/SpinButton semantics through the
  active crates.io backend without a wider input or accessibility programme.
- Correctness needs locale/timezone/date ownership, a picker overlay, raw draft
  callbacks, a compatibility alias, or silent normalization.
- More than the exact TimeInput mounted ledger cell would move, or work expands
  into drag/drop, another component's behavior, Jetstream admission, release,
  or a sibling repository.

## Continuation

Return the exact paired API, vectors, web/native behavior evidence, clean-rename
search, named mounted regressions, ledger delta, validation, and August log to
the orchestrator. Do not start another component or drag card. After
operator-authorized merge, the orchestrator chooses the next serial lane from
`g16.022` and the component-continuation runway.
