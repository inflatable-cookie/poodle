# g16.021 — Drag-And-Drop Semantic Kernel

Status: complete — pushed for orchestrator review; merge is operator-authorised
Date: 2026-08-28
Card: `docs/roadmaps/g16/021-drag-drop-semantic-kernel.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/architecture/006-headless-core-and-machine-model.md`

## Outcome

One renderer-neutral drag session now exists once per language pair, proved
against one shared transition corpus that both conformance runners execute.
Lifecycle, session identity, semantic intent, cancellation, nested-target
arbitration, and exactly-once terminal effects are settled before any adapter
or component migration begins.

No component, contract, specimen, Svelte, React, GPUI, Jetstream, token,
generated artifact, package version, workflow, or sibling-repository file was
changed. The parity evidence ledger is byte-identical to `origin/main` at
47 mounted / 127 missing.

## Paired public API

`@inflatable-cookie/poodle-core` — `packages/core/src/drag-drop.ts`, re-exported
from the package root:

| TypeScript | Rust (`poodle_headless::drag_drop`) |
| --- | --- |
| `DragOperation` | `DragOperation` |
| `DropPosition`, `StandardDropPosition` | `DropPosition`, `DROP_POSITION_BEFORE`/`_INSIDE`/`_AFTER` |
| `DragSubject` | `DragSubject` |
| `DropIntent` | `DropIntent` |
| `DropEligibility` | `DropEligibility` |
| `DragSessionPhase` | `DragSessionPhase` |
| `DragCancelReason` | `DragCancelReason` |
| `DragTerminalOutcome` | `DragTerminalOutcome` |
| `DragAnnouncementKind` | `DragAnnouncementKind` |
| `DragSession` | `DragSession` |
| `DragSessionContext` | `DragSessionContext` |
| `DragSessionEvent` | `DragSessionEvent` (plus `session_id()`) |
| `DragSessionEffect` | `DragSessionEffect` |
| `DragSessionResult` | the `(phase, context, effects)` return tuple |
| `DropTargetCandidate` | `DropTargetCandidate` |
| `dragSessionTransition` | `drag_session_transition` |
| `resolveDropTarget` | `resolve_drop_target` |

Names are idiomatic per language; the distinctions are identical. Nothing in
either module names a DOM event, GPUI type, coordinate, rectangle, file, path,
host window id, or application record.

Two shape choices are worth recording because a reviewer will ask:

- The phase type is named `DragSessionPhase` rather than `*State` because spec
  069 names it that. It is still the machine's state: the TypeScript transition
  returns the canonical `TransitionResult<DragSessionPhase, …>`.
- `DragSessionResult` has no Rust counterpart type. The Rust mirrors return
  tuples throughout (`modal`, `tabs`, `select`); this one returns
  `(DragSessionPhase, DragSessionContext, Vec<DragSessionEffect>)` because the
  session carries both a phase and a context.

## Locked transition rules as implemented

Exactly-once is a property of the phase, not of a flag. `emitDragStart` has
exactly one transition that emits it (`armed -> dragging`), `requestDrop` one
(`dragging -> dropping`), and the terminal quartet — result, announcement,
focus-return, cleanup — one (the single transition into `ended` or
`cancelled`). Every repeat therefore arrives in a phase that no longer accepts
it and is inert, with no bookkeeping to keep in step.

- `PREPARE` allocates one caller-supplied identity and emits one
  `prepareSession`. It is inert when the requested operation is not in the
  source's allowed operations, and inert from `dragging`, `dropping`, `ended`,
  and `cancelled`.
- Supersession is `PREPARE` naming a different session while `preparing` or
  `armed`: the previous session's cancelled result, announcement, and cleanup
  are emitted, then the new `prepareSession`. Re-preparing the same session is
  inert.
- Only completion naming the current session may arm it. Every other event
  carries the session id it was created for and is dropped when it names a
  session that is no longer current, so a late `PREPARED` for a superseded
  session is inert.
- `ended` means an authoritative drop result arrived — committed, rejected, or
  failed. `cancelled` means the session aborted without one. Both are
  observable; `RESET` returns to `idle` and is inert from anywhere else.
- Intent and operation change only while `dragging`. An intent is refused when
  its operation is outside the source's allowed operations, and accepting one
  sets the session's current operation. `dropping` refuses `TARGET_INTENT`,
  `TARGET_CLEARED`, and `OPERATION_CHANGED`, which is how "a released pointer
  or native OS drop cannot return from dropping to dragging" is enforced.
- `TARGET_LOST` cancels only when it names the current intent's target; a
  target the session is not resolved against is inert.
- `returnFocus` is emitted on a terminal transition out of `dragging` or
  `dropping` only. A session that never reached pickup never took focus, so it
  does not ask for it back.
- Semantic state and effects are immutable values. The transition never
  mutates the caller's context, performs no callback, timer, measurement,
  mutation, focus, announcement, or I/O, and returns the caller's own context
  object unchanged when the event is inert.

`announce` carries only the announcement kind. The adapter already holds the
session (target, position, operation) and, for a terminal announcement, the
`emitDropResult` immediately preceding it, so duplicating that state into the
effect would create a second place for it to drift.

## Nested-target arbitration

`resolveDropTarget` / `resolve_drop_target` accepts already-measured
candidates and returns at most one intent. It discards non-containing and
ineligible candidates, then prefers the deepest, then explicit priority among
equal depth, then stable registration order. Geometry measurement stays
adapter-owned: the adapter decides `containsPoint` and `depth`, the kernel
decides which candidate wins. The result does not depend on the order the
adapter collected its candidates in — proved in both languages.

## Shared vector coverage

`packages/contracts/headless/vectors/machines.json` gains one hand-authored
`dragDrop` section: **24 session cases across 132 ordered steps** and
**7 arbitration cases**. No code generation, generated schema, IR, runtime
registry, or second evidence ledger was added.

The drag session is the only machine here whose claims are about ordering
across a whole lifecycle rather than one transition, so its cases are step
sequences: each case starts at `idle` with no session, and each step pins the
resulting phase, the effects that step emitted in order, and — where the case
states it — a subset of the resulting session. That is the existing
hand-authored fixture pattern extended with a step list, not a new format.

Covered: the full prepare/arm/activate/hover/drop/commit/cleanup/reset path;
preparation decline, failure, and supersession; late completion after
supersession; operation change and unsupported-operation inertia; target
enter, change, repeat, and leave; drop-time rejection and drop failure; source,
target, transport, and window loss; loss before pickup; Escape and explicit
cancellation; repeated start, drop request, terminal result, and cleanup;
activation outside `armed`; a drop request without an accepted intent;
`dropping` refusing to return to `dragging`; stale events naming another
session; and deepest-target, priority, stable-order, discarded-candidate, and
no-eligible-target arbitration.

## Changed surfaces

- `packages/core/src/drag-drop.ts` — new; the TypeScript kernel.
- `packages/core/src/index.ts` — root export block for the new module.
- `packages/core/test/drag-drop.test.ts` — new; 7 focused TypeScript tests.
- `packages/core/test/conformance.test.ts` — runs the `dragDrop` sessions and
  arbitration cases.
- `packages/contracts/headless/src/drag_drop.rs` — new; the Rust kernel plus 5
  focused unit tests.
- `packages/contracts/headless/src/lib.rs` — module registration.
- `packages/contracts/headless/tests/conformance.rs` — `drag_drop_conformance`
  runs the same cases.
- `packages/contracts/headless/vectors/machines.json` — the `dragDrop` section.
- `packages/svelte/preview/scripts/machine-shape-drift.ts` — one `PINNED`
  registry entry; see the note below.

## One edit outside the card's writable list

`machine-shape-drift` requires that a machine present in both runtimes be
covered by a shared vector both harnesses execute. Landing the pair without
registering it would have created a new finding in that gate, so the entry was
added. It is six lines of registry data in a drift script — no Svelte
component, contract, or implementation was touched.

That gate was already failing on `origin/main` with 16 findings, and it still
reports exactly those 16. `dragDrop` appears in neither its pinning nor its
convention findings: the new module satisfies the machine-shape convention
(`Context` + `Event` + `Effect` + `Result` + `TransitionResult`), and the gate
now checks 22 pinned machines instead of 21. The 16 pre-existing findings are
untouched and remain someone else's card.

## Validation

Run from the worker worktree after `bun install` restored the locked
dependencies:

- focused TypeScript drag tests — pass, 7 tests;
- focused Rust drag tests — pass, 5 of 155 `poodle-headless` unit tests;
- shared TypeScript vector runner — pass, 103 cases in
  `packages/core/test/conformance.test.ts` (31 of them `dragDrop`);
- shared Rust vector runner — pass, `drag_drop_conformance` among 12
  conformance tests;
- `effigy test:core` — pass, 858 tests across 51 files (820 before this card);
- `effigy test:contracts` — pass;
- `effigy check:parity-evidence-ledger` — pass, 175 component evidence rows,
  47 mounted / 127 missing unchanged;
- `effigy ci:web` — pass, exit 0; 3137 component tests across 359 files;
- `effigy ci:rust` — pass, exit 0;
- `effigy docs:check` — pass, exit 0;
- `effigy qa` — pass, exit 0;
- `git diff --check origin/main...HEAD` — clean.

Everything stayed headless. No `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selector was run.

Execution friction: the launcher worktree had no `node_modules`, so
`test:core` first failed on a missing `marked` package unrelated to this card.
`bun install` fixed it and did not modify `bun.lock`. Not recorded as a
papercut — it is ordinary first-use bootstrap in a fresh worktree, already
covered by `effigy bootstrap:deps`.

## Non-claims

- No adapter exists. Nothing here is wired to a pointer, touch, keyboard, DOM,
  GPUI, or transport surface, and no component uses it yet.
- The existing `dock-external-drag` and `tabs-reorder` helpers are untouched.
  Migrating or removing the old public drag exports is explicitly a later
  card's decision and was not made here.
- No geometry, auto-scroll, preview, capture, cross-window transport, inbound
  file, or drag-out behaviour is implemented — only the semantics those
  adapters will report into.
- No component evidence cell moved, and no ledger row changed.
- Jetstream remains program-deferred; nothing here claims otherwise.

## Next

`g16.022` — the drag-and-drop web custom-surface substrate — is the next card
in the programme. It stays planned until this kernel is reviewed and merged,
and until the public migration boundary in
`docs/triage/20260828-221415-drag-drop-public-migration-boundary.md` is
resolved by the operator.
