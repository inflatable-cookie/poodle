# g16.055 — Drag Source Pre-activation Selection Suppression

Status: ready
Opened: 2026-09-02
Depends on: g16.021–g16.028 drag substrate and the merged Tree interaction
repair from PR #125; independent of the post-triage product runway
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/069-drag-and-drop.md`, `../../contracts/components/tree.md`
Reported failure: dragging a reorderable Tree row can paint a browser text
selection across labels traversed before the pointer activation threshold is
crossed

## Goal

Prevent native browser text selection from starting during an accepted pointer
drag candidate. Suppression begins when the shared DOM controller accepts a
draggable source on pointerdown, remains through an activated drag, and restores
the exact authored root style on every pre-threshold and terminal exit.

This is a shared web-substrate repair, not Tree styling. Do not make Tree text
globally unselectable, add a public prop, or create component-specific gesture
logic.

## Fixed Behavior Envelope

- A primary pointerdown suppresses selection only after `sourceFromEvent`
  accepts a live, enabled registered source inside the connected root.
- Suppression covers the armed pre-threshold interval and the active pointer
  drag. It starts early enough that movement through adjacent labels cannot
  create or paint a browser `Selection` range.
- Pointerup, pointercancel, lost capture, Escape, pre-threshold abandonment,
  touch tolerance cancellation, source loss, disconnect, destroy, native drag
  handoff, and any other hardware-release path restore the root's exact prior
  inline `user-select` declaration.
- A click or abandoned candidate still reaches the component's ordinary click
  behavior. Tree row selection is unchanged.
- Interactive descendants and no-drag descendants remain excluded before
  suppression. Rename inputs, text inputs, textareas, contenteditable hosts,
  links, buttons, and other existing exclusions keep their native selection and
  interaction behavior.
- Non-reorderable Tree text and content outside an accepted drag source remain
  normally selectable.
- Svelte and React inherit the behavior from the same controller. Rust/GPUI has
  no browser text-selection lifecycle and receives no synthetic parity change.

## Ordered Work

1. Reproduce the paint/range leak in a mounted reorderable Tree in Chromium and
   WebKit, including movement before and after the activation threshold.
2. Move root selection suppression to the smallest accepted-source pointerdown
   boundary. Reuse one idempotent restore path; do not fork lifecycle cleanup.
3. Add focused controller regressions for accepted pointerdown, pre-threshold
   exits, exact authored-style restoration, interactive exclusions, source
   loss, destroy/disconnect, and consecutive gestures.
4. Add paired Svelte and React Tree coverage for click selection and rename/input
   exclusion. Add a mounted browser oracle that inspects the real Selection
   range and visible selection state while the pointer crosses several labels.
5. Reconcile spec 069, the Tree contract only if it states the affected
   interaction law, this card, and one September execution log. Open one PR.

## Acceptance

- A pointer drag through multiple reorderable Tree labels produces no non-empty
  browser Selection range in Chromium or WebKit, including motion before the
  distance threshold is crossed.
- An ordinary row click still selects that row exactly once in both Svelte and
  React.
- Pointerup or cancellation before activation restores the connected root's
  exact prior authored inline `user-select` value. Empty, `text`, and one
  non-default authored value are covered.
- Rename/input text remains selectable and receives no root suppression because
  its pointerdown is rejected by the existing interactive-host boundary.
- A non-reorderable Tree does not register drag sources and its labels retain
  ordinary browser selection behavior.
- No Tree CSS `user-select: none`, public API, React/Svelte fork, or native
  behavior change enters the diff.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Required proof |
| --- | --- | --- |
| Suppression precedes browser selection | press a label, move less than the activation distance across text, then cross several sibling labels | Chromium and WebKit mounted Tree probe has an empty Selection throughout; planting suppression back in `activate()` creates a non-empty range or visible paint |
| Abandoned gestures restore authored style | root begins with inline `user-select: text`; press and release below threshold | focused controller proof observes `none` during the candidate and exactly `text` after release; empty and a second authored value also restore |
| Click semantics survive | press and release one reorderable row without crossing threshold | paired mounted Svelte/React proof records one row-selection callback and no drag commit |
| Interactive text remains selectable | begin a selection inside the Tree rename/input descendant | controller never suppresses the root; real Selection/input selection changes normally in both shells |
| Unregistered text stays ordinary | render the same Tree with `reorderable=false` and drag across its label text | browser Selection becomes non-empty, proving the test is not incapable of observing selection |
| Cleanup is total | abandon by pointercancel, source unregister, disconnect, destroy, then begin a later gesture | prior inline style returns after each path; no stale suppression survives or clobbers the later session |

Commit the real proof before planting the current activation-time behavior.
Confirm the mounted browser oracle and focused lifecycle tests fail for the
intended reason, restore from the committed proof, and rerun green.

## Writable Scope

- `packages/core/src/dom/drag-drop-controller.ts` and its focused tests;
- paired Svelte and React Tree tests only where component exposure must be
  proved;
- the smallest existing drag-drop browser fixture/probe extension needed for a
  real Selection oracle;
- `docs/specs/069-drag-and-drop.md`, `docs/contracts/components/tree.md` only if
  required for the settled behavior, this card, one September execution log,
  and `PAPERCUTS.md` only for new execution friction.

The orchestrator owns `docs/roadmaps/g16/README.md` and
`docs/roadmaps/generation-index.md` integration after merge. Do not edit Tree
geometry, external reorder authority, component CSS, Rust/GPUI, versions,
release files, workflows, downstream repositories, or the post-triage runway.

## Validation

Use Effigy selector discovery in the worker worktree. At minimum:

- focused shared-controller tests;
- focused Svelte and React Tree tests;
- the mounted drag-drop browser probe in Chromium and WebKit;
- relevant drag inventory and contract drift checks;
- `effigy ci:web` and `effigy docs:check`;
- `git diff --check origin/main...HEAD` and exact diff-scope checks.

Do not run `*-windowed`, native visual, release, tag, publication, workflow
mutation, or sibling-repository commands.

## Stop Conditions

- Correct behavior requires suppressing selection before source eligibility is
  known, calling `preventDefault()` in a way that breaks click/focus, or changing
  the existing interactive-host exclusion contract.
- A browser-specific workaround or Tree CSS rule is required instead of one
  controller lifecycle.
- The fix changes the activation threshold, pointer capture, touch scrolling,
  row-selection semantics, rename behavior, or public component API.
- The mounted browser proof cannot distinguish real Selection behavior from a
  declaration-only assertion.

## Continuation

After accepted merge, this repair rides the next unreleased `0.3.0` candidate.
Do not create a `0.2.x` patch publication lane. Figmatic may verify against the
merged source head immediately, but package adoption waits for the separately
authorized compiled-distribution `0.3.0` release lane.
