# g16.031 — Continuous Audio Machine And Web Lifecycle

Status: ready to launch — audit complete; `g16.030` merged in PR #98
Opened: 2026-08-28
Depends on: completed `g16.030`; this lane edits the same paired core/headless
exports and shared machine-vector corpus
Governing refs: `../../architecture/008-audio-control-family.md`,
`../../contracts/components/knob.md`, `../../contracts/components/fader.md`,
`../../contracts/components/xy-pad.md`,
`../../contracts/001-working-rules.md`

## Goal

Make Knob, Fader, and XYPad use one exact continuous-gesture model in the
TypeScript and Rust cores, then harden the Svelte and React adapters around
that model. Close the observed lifecycle, fine-drag, detent, atomic-pair, and
type-in drift before mounting the controls in GPUI.

This is not payload drag-and-drop. It does not use, extend, or depend on the
`g16.021`–`g16.028` drag session kernel.

## Audit Result

The 2026-08-28 audit found real implementation drift rather than missing
evidence alone:

- TypeScript `DRAG_BEGIN` accepts a second begin while a gesture is active;
  all six web adapters can replace the active pointer without ending it.
- None of the web adapters handles lost pointer capture, so a gesture can stay
  open without its matching end.
- Svelte Knob and Fader commit again on blur after Enter and commit the draft
  after Escape; React already suppresses that blur path.
- Rust has real scalar and XY machines, contrary to the old register wording,
  but its fine movement is current-value interpolation rather than anchored
  movement/rebase. It omits Fader detents, Knob drag-mode distinctions, and
  XYPad's press-position behavior.
- Existing focused runtime tests do not exercise pointer lifecycle or Knob /
  Fader type-in through the component adapters.

## Locked Semantic Model

Keep architecture 008 and the three component contracts authoritative:

- one begin and one end effect around each accepted pointer gesture;
- repeated begin, stale move, stale release, and repeated termination are
  inert;
- cancellation closes the gesture exactly once without reopening it;
- coarse/fine switching rebases at the current value and current pointer, so
  holding or releasing Shift never jumps;
- Knob vertical movement uses anchored pointer delta and `dragSensitivity`;
  circular movement uses the standard 270-degree normalized sweep;
- Fader uses axis-normalized position, declared law, and normalized detent
  snap radius;
- XYPad updates its pair atomically, changes coarse value at the accepted
  press position, and applies anchored fine travel to both axes;
- keys, bounds, reset, wheel, and valid type-in are atomic value-change plus
  value-commit operations, not pointer gestures; and
- disabled controls are inert on every event route.

Exact type and event names may follow local conventions. TypeScript and Rust
must expose the same distinctions and ordered effects.

## Shared Vector Corpus

Add a bounded `audioControls` section to
`packages/contracts/headless/vectors/machines.json` and run it through both
existing machine-conformance runners. Cover at least:

- accepted and repeated begin; coarse moves; release and cancellation;
- stale move/release and repeated end;
- coarse-to-fine and fine-to-coarse rebase without a jump;
- vertical and circular Knob movement;
- horizontal and vertical Fader movement plus nearest-detent tie behavior;
- XYPad coarse press, fine movement, independent laws, and atomic effects;
- reset, wheel, keys, Page Up/Down, Home/End, valid and invalid type-in; and
- disabled-event inertia.

The corpus is hand-authored semantic evidence. Do not create a component IR,
runtime registry, generated adapter, specimen matrix, or second ledger.

## Execution Plan

- [ ] **Batch 1 — shared cases and paired cores.** Bring the TypeScript and
      Rust contexts, events, transitions, and ordered effects onto the locked
      model and prove them against one corpus.
- [ ] **Batch 2 — web pointer lifecycle.** Accept only one primary pointer,
      preserve capture ownership, close once on release/cancel/lost capture or
      teardown, and ignore stale pointer ids in Svelte and React.
- [ ] **Batch 3 — web entry lifecycle.** Give Svelte Knob/Fader the same
      Enter/Escape/blur suppression as React and test both adapters through
      actual entry focus transitions.
- [ ] **Batch 4 — focused component proof.** Add mounted Svelte and React
      pointer/callback tests for all three controls and retain human-centred
      specimens unchanged unless a small stateful example repair is required.
- [ ] **Batch 5 — closeout.** Record exact paired APIs, vectors, callback
      traces, validation, non-claims, and the `g16.032` dependency in one log.

## Acceptance Criteria

- [ ] TypeScript and Rust return identical contexts and ordered effects for
      the shared Knob, Fader, and XYPad cases.
- [ ] Every accepted pointer gesture emits exactly one begin and one end;
      repeated, stale, lost-capture, cancel, and teardown paths cannot strand
      or duplicate a gesture.
- [ ] Fine-mode changes preserve continuity in both languages.
- [ ] Fader detents, Knob modes, and XYPad press/atomic-pair behavior match the
      three contracts in both cores.
- [ ] Svelte and React produce the same pointer and callback traces.
- [ ] Svelte Knob/Fader Enter commits once, Escape commits nothing, and the
      following blur cannot duplicate or reverse either result.
- [ ] Existing public web props and callback names remain intact; no alias,
      fallback, DOM-event public type, or drag-and-drop dependency is added.
- [ ] No ledger cell moves. This card proves paired semantics and web adapter
      behavior; native mounted proof belongs to `g16.032`.

## Writable Scope

- `packages/core/src/audio/value-controls.ts`,
  `packages/core/src/audio/xy-pad.ts`, their focused tests, and exact exports;
- `packages/contracts/headless/src/audio.rs`, its focused tests, and exact
  exports;
- the new bounded audio-control cases in
  `packages/contracts/headless/vectors/machines.json` and both existing
  machine-vector runners;
- Knob, Fader, and XYPad Svelte/React implementations and focused tests;
- their specimens only for a proven stateful-example defect, not curation or
  exhaustive matrices;
- the three component contracts and architecture 008 only for exact wording
  reconciliation after implementation;
- this card, one August log, g16/front-door closeout, and `PAPERCUTS.md` only
  for new execution friction.

Do not edit Node vocabulary, poodle-render audio builders, GPUI/Jetstream
adapters, payload drag-and-drop files, other audio components, recipes/tokens,
the parity ledger, workflows, versions, releases, sibling repositories, or
downstream consumers.

## Validation

Use Effigy selectors discovered after worker startup. At minimum:

- focused TypeScript/Rust audio machine-vector tests;
- focused core audio-value and XYPad tests;
- focused Svelte and React AudioControls tests;
- `effigy test:core`, `effigy test:components`, and
  `effigy test:contracts`;
- contract/callback/value-domain drift checks;
- `effigy ci:web`, `effigy ci:rust`, and `effigy docs:check`;
- one final headless `effigy qa`; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- Correct behavior requires changing the public callback/value surface or one
  of the locked contracts.
- Browser teardown cannot close an accepted gesture exactly once without a
  new public event or global singleton.
- Paired semantics require DOM, GPUI, renderer, focus, or callback execution
  inside the pure machine.
- Work expands into DragNumberField, payload drag-and-drop, Node/GPUI native
  mounting, broad accessibility, visual comparison, Jetstream admission, or a
  sibling repository.

## Continuation

Return the paired contexts/events/effects, shared cases, web pointer and entry
traces, validation, and closeout log to the orchestrator. Do not start native
mounting. After operator-authorized merge, `g16.032` may implement the bounded
Node/GPUI continuous-value seam and mounted proof.
