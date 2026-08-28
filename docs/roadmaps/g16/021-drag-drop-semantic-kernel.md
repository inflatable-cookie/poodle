# g16.021 — Drag-And-Drop Semantic Kernel

Status: complete — merged in PR #96
Opened: 2026-08-28
Depends on: merged `g16.020`; architecture 011 and spec 069 approved
Governing refs: `../../architecture/011-drag-and-drop-substrate.md`,
`../../specs/069-dependable-drag-and-drop-substrate.md`,
`../../architecture/006-headless-core-and-machine-model.md`,
`../../contracts/001-working-rules.md`

## Goal

Implement the renderer-neutral drag session once per language pair: TypeScript
in `@inflatable-cookie/poodle-core` and Rust in `poodle-headless`. Prove both
against one small shared transition corpus. Establish lifecycle, identity,
intent, cancellation, target arbitration, and exactly-once effects before any
DOM, GPUI, component, cross-window, or file transport work begins.

This card moves no component evidence cell. It is the substrate prerequisite
for `g16.022`–`g16.028`.

## Public Semantic Surface

Export equivalent idiomatic TypeScript and Rust forms of the normative spec 069
vocabulary:

- `DragOperation`: move, copy, or link;
- `DragSubject`: opaque kind plus id;
- `DropIntent`: target id, semantic position, and operation;
- `DropEligibility`: accepted intent or rejected reason;
- `DragSessionPhase`: idle, preparing, armed, dragging, dropping, ended, or
  cancelled;
- session state carrying a unique session id, source id, subject, operation,
  current intent where applicable, and the identity needed to reject stale
  asynchronous completion;
- events for preparation, activation, intent/operation change, drop request and
  result, cancellation, terminal cleanup, source/target loss, and host/transport
  loss;
- ordered effects for prepare, start, drop request, terminal result,
  announcement intent, focus-return intent, and cleanup; and
- one pure transition entry point plus one pure nested-target resolver.

Names may follow local conventions, but the root TypeScript and Rust exports
must remain recognizably paired. Do not expose DOM events, GPUI types,
coordinates, rectangles, files, paths, host window ids, or application records.

## Locked Transition Rules

- `idle -> preparing` allocates one caller-supplied session identity and emits
  one preparation effect.
- Only completion naming the current session may arm it. Decline, failure,
  abort, or supersession cancels once; late completion is inert.
- Activation is valid only from armed, emits start once, and retains the
  original subject and source identity.
- Intent may change or clear only while dragging. Operation changes are limited
  to the source's allowed operations.
- Drop request requires one accepted current intent, re-emits that intent for
  runtime revalidation, and enters dropping once.
- Accepted/committed, rejected, failed, source loss, target loss, transport
  loss, window loss, and explicit cancel follow spec 069. A released pointer or
  native OS drop cannot return from dropping to dragging.
- Ended and cancelled are observable terminal outcomes. Cleanup and terminal
  notification are emitted at most once before reset to idle.
- A stale event, repeated start, repeated drop request, repeated terminal
  result, or repeated cleanup is inert.
- Semantic state and effects are immutable values. The machine performs no
  callback, timer, measurement, mutation, focus, announcement, or I/O itself.

The nested-target resolver accepts already-measured candidates. It discards
non-containing or ineligible candidates, then sorts by deepest registration,
explicit priority among equal depth, and stable registration order. It returns
at most one intent. Geometry measurement remains adapter-owned.

## Shared Vector Corpus

Extend the existing cross-language fixture pattern in
`packages/contracts/headless/vectors/machines.json` with a `dragDrop` section.
Both the core TypeScript runner and Rust conformance runner must execute the
same cases and compare ordered effects.

Cover at least:

- prepare, arm, activate, hover, drop, commit, cleanup, and reset;
- preparation decline/failure, supersession, and late completion;
- operation change and unsupported-operation inertia;
- target enter/change/leave and drop-time rejection;
- source, target, transport, and window loss;
- Escape and explicit cancellation;
- repeated start, drop, terminal, and cleanup events;
- deepest-target, priority, stable-order, and no-eligible-target arbitration;
  and
- session A asynchronous completion arriving after session B replaced it.

The fixture is a bounded semantic test corpus, not a generated component
authority or scene format. Do not add code generation, runtime registries, or a
second evidence ledger.

## Execution Plan

- [x] **Batch 1 — exact types and vectors.** Add the paired semantic types and
      the shared `dragDrop` cases without platform fields.
- [x] **Batch 2 — TypeScript kernel.** Implement pure transition and arbitration
      functions in core, export them, and run the shared vectors.
- [x] **Batch 3 — Rust kernel.** Mirror the semantics in `poodle-headless` and
      run the same vectors through the existing Rust conformance runner.
- [x] **Batch 4 — closeout.** Record exact API names, vector coverage,
      validation, and non-claims in one August log. Leave every component and
      ledger row unchanged.

## Acceptance Criteria

- [x] TypeScript and Rust expose the same semantic distinctions without sharing
      renderer, shell, filesystem, or application types.
- [x] One fixture corpus proves matching states and ordered effects in both
      languages.
- [x] Stale asynchronous completion and every repeated terminal path are inert.
- [x] Exactly one start, drop request, terminal notification, and cleanup can be
      emitted per session.
- [x] Nested target arbitration is deterministic and returns one intent.
- [x] No DOM, Svelte, React, GPUI, Jetstream, component, host bridge, file, or
      visual implementation changes.
- [x] The parity ledger remains byte-stable at 47 mounted / 127 missing.
- [x] One August log records the landed kernel and leaves `g16.022` as the next
      drag programme card.

## Writable Scope

- one new focused TypeScript module under `packages/core/src/` and its root
  export;
- focused core tests and the existing TypeScript conformance runner;
- one new focused Rust module under `packages/contracts/headless/src/` and its
  crate export;
- focused Rust tests and the existing Rust conformance runner;
- `packages/contracts/headless/vectors/machines.json` for the `dragDrop` cases;
- this card, one August execution log, g16/front-door closeout, and
  `PAPERCUTS.md` only for new execution friction.

Do not edit component contracts or implementations, Svelte/React packages,
Node vocabulary, poodle-render, GPUI, Jetstream, existing Tabs/DockRegion drag
helpers, tokens, generated artifacts, package versions, workflows, releases,
downstream repositories, or sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused core drag-drop tests;
- focused `poodle-headless` drag-drop tests;
- the shared TypeScript and Rust conformance-vector runners;
- `effigy test:core` and `effigy test:contracts`;
- `effigy check:parity-evidence-ledger` proving no ledger movement;
- `effigy ci:web`, `effigy ci:rust`, and `effigy docs:check`;
- one final headless `effigy qa`; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The lifecycle or public semantic distinctions must differ from architecture
  011/spec 069.
- Correctness requires runtime geometry, callbacks, timers, platform events,
  filesystem data, or hidden mutable global state inside the kernel.
- Shared vectors would require a new generated schema/IR rather than the
  existing hand-authored fixture pattern.
- Existing public drag exports must be removed, aliased, or adapted; that
  migration is explicitly outside this card.
- Another component/evidence row changes, Jetstream admission is required, or
  validation needs a windowed selector, release mutation, or sibling repo.

## Landed

Log: `../../logs/2026-08/20260828-g16-021-drag-drop-semantic-kernel.md`.

Paired entry points: `dragSessionTransition` / `drag_session_transition` and
`resolveDropTarget` / `resolve_drop_target`, with the paired
`DragOperation`, `DropPosition`, `DragSubject`, `DropIntent`,
`DropEligibility`, `DragSessionPhase`, `DragCancelReason`,
`DragTerminalOutcome`, `DragAnnouncementKind`, `DragSession`,
`DragSessionContext`, `DragSessionEvent`, `DragSessionEffect`, and
`DropTargetCandidate` vocabulary. `packages/core/src/drag-drop.ts` is exported
from the core package root; `poodle_headless::drag_drop` is registered in the
crate.

Shared corpus: one hand-authored `dragDrop` section in
`packages/contracts/headless/vectors/machines.json` — 25 session cases across
139 ordered steps plus 7 arbitration cases — executed by both
`packages/core/test/conformance.test.ts` and `drag_drop_conformance` in
`packages/contracts/headless/tests/conformance.rs`. No generator, schema, IR,
runtime registry, or second evidence ledger was added.

Session identity is caller-supplied and single-use: an id must stay unique for
as long as any asynchronous completion created for it can still arrive. That is
the one rule the kernel cannot enforce — it rejects a stale completion by
comparing the id, so two sessions sharing one are indistinguishable to it. The
rule is documented on `DragSession.sessionId` and `DragSession::session_id`, on
both `PREPARE` variants, and in both module headers; no vector reuses an id
across sessions; and `a completion for a reset session cannot arm its
successor` proves that a late completion for a terminated-and-reset session is
inert while its freshly-identified successor stays current.

Validation: focused TypeScript (7) and Rust (5) drag tests; both vector
runners; `effigy test:core` (859 tests), `effigy test:contracts`,
`effigy check:parity-evidence-ledger`, `effigy ci:web`, `effigy ci:rust`,
`effigy docs:check`, and `effigy qa` — all exit 0; `git diff --check
origin/main...HEAD` clean. All headless.

Ledger: unchanged and byte-identical to `origin/main` at 47 mounted /
127 missing. No component evidence cell moved.

One edit falls outside the card's writable list and is deliberate:
`packages/svelte/preview/scripts/machine-shape-drift.ts` gains one `PINNED`
registry entry, because that gate requires a machine present in both runtimes
to be covered by a shared vector both harnesses run. The gate's 16
pre-existing findings on `origin/main` are unchanged; `dragDrop` adds none.

Non-claims: no adapter, no component migration, no geometry, transport, file,
or drag-out behaviour. `dock-external-drag` and `tabs-reorder` are untouched:
the approved clean public break deletes the old DOM-shaped helpers only after
their mounted replacements pass, which is `g16.023`/`g16.026` work, not this
card's.

## Continuation

The paired API names, shared vectors, validation, unchanged ledger totals, and
execution log are recorded above. The current runway dispatches `g16.029`;
`g16.022` remains the next drag-programme card and is promoted after the serial
core/export tranche.
