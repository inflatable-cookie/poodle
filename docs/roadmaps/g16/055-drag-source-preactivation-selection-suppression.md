# g16.055 — Drag Source Pointer-gesture Browser Suppression

Status: complete — merged in PR #151
Opened: 2026-09-02
Depends on: g16.021–g16.028 drag substrate and the merged Tree interaction
repair from PR #125; independent of the post-triage product runway
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/069-drag-and-drop.md`, `../../contracts/components/tree.md`
Reported failures: dragging a reorderable Tree row can paint a browser text
selection across labels traversed before the pointer activation threshold is
crossed; after an activated move, the browser delivers a compatibility click
to the source row and Tree emits a redundant selection request
Merge: `2245ea0a27a06e16d584cdf62895b7f12f3d6f09`

## Goal

Prevent native browser text selection from starting during an accepted pointer
drag candidate. Suppression begins when the shared DOM controller accepts a
draggable source on pointerdown, remains through an activated drag, and restores
the exact authored root style on every pre-threshold and terminal exit.

Consume the browser compatibility click produced by an activated pointer drag
before it reaches the draggable row. A tap or any gesture abandoned before
activation remains an ordinary click.

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
  inline `user-select` / `-webkit-user-select` value and priority.
- An activated pointer drag consumes only its own compatibility click at the
  shared controller capture boundary: stop immediate propagation and prevent
  the cancelable click's default. The rule is independent of whether the
  eventual drop commits, rejects, fails, or cancels after activation.
- Compatibility-click suppression is bounded to that completed activated
  gesture and source path. If no compatibility click arrives, it expires
  without swallowing a later unrelated click.
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
3. Add one bounded compatibility-click guard at the shared controller boundary.
   Arm it only for a completed activated pointer gesture, consume that
   gesture's source-row click in capture, and prove stale guards expire.
4. Add focused controller regressions for accepted pointerdown, pre-threshold
   exits, exact authored-style restoration, interactive exclusions, source
   loss, destroy/disconnect, compatibility-click delivery, and consecutive
   gestures.
5. Add paired Svelte and React Tree coverage for tap selection, activated-drag
   click suppression, and rename/input exclusion. Add a mounted browser oracle
   that inspects the real Selection range and visible selection state while the
   pointer crosses several labels, and observes no trailing selection callback.
6. Reconcile spec 069, the Tree contract only if it states the affected
   interaction law, this card, and one September execution log. Open one PR.

## Acceptance

- A pointer drag through multiple reorderable Tree labels produces no non-empty
  browser Selection range in Chromium or WebKit, including motion before the
  distance threshold is crossed.
- An ordinary row click still selects that row exactly once in both Svelte and
  React.
- An activated pointer drag delivers no compatibility click to Tree row
  selection in Svelte or React. This remains true when the drop result is
  committed, rejected, failed, or cancelled after activation.
- A tap and a gesture abandoned before the activation threshold still deliver
  their ordinary click and select the row. Keyboard reorder is unchanged.
- Pointerup or cancellation before activation restores the connected root's
  exact prior authored inline `user-select` / `-webkit-user-select` value and
  priority. Empty, `text`, one non-default authored value, and `!important`
  are covered.
- An activated pointer drag's compatibility click is cancelled at capture,
  including the default action of a registered link or button source. A tap
  still follows that default.
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
| Authored `!important` is restored | root begins with inline `user-select: text !important` and `-webkit-user-select: all !important` | focused controller proof observes both values and `getPropertyPriority()` `important` after pre-threshold release |
| Click semantics survive | press and release one reorderable row without crossing threshold | paired mounted Svelte/React proof records one row-selection callback and no drag commit |
| Activated drag has no trailing row click | complete one pointer move, then let the browser dispatch its compatibility click before an async commit settles | paired shells record one reorder request and zero selection requests; controller capture proves the click never reaches the source row |
| Compatibility click default is cancelled | register an `<a href>` as the source, activate, then let the browser dispatch the compatibility click | controller proof records `defaultPrevented`; Chromium/WebKit probe hash stays off the href; a tap still follows href |
| Drop outcome does not reopen click | resolve activated gestures as committed, rejected, failed, and no-intent cancelled | each consumes its gesture-bound compatibility click; no outcome-dependent Tree selection path exists |
| Suppression cannot eat later input | finish an activated drag whose browser emits no click, then click the row later | stale guard expires; the later click selects exactly once |
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
- the mounted drag-drop browser probe in Chromium and WebKit, including real
  compatibility-click delivery;
- relevant drag inventory and contract drift checks;
- `effigy ci:web` and `effigy docs:check`;
- `git diff --check origin/main...HEAD` and exact diff-scope checks.

Do not run `*-windowed`, native visual, release, tag, publication, workflow
mutation, or sibling-repository commands.

## Stop Conditions

- Correct behavior requires suppressing selection before source eligibility is
  known, calling `preventDefault()` in a way that breaks click/focus, or changing
  the existing interactive-host exclusion contract.
- Correct behavior requires Tree to recognize stale revisions, suppress its own
  click, or make selection idempotent around a drag.
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

## Implementation

Shared DOM controller suppresses `user-select` and `-webkit-user-select` on the
connected root at accepted primary pointerdown, restores the exact prior inline
value and priority on every exit, and consumes one source-path compatibility
click after an activated pointerup/cancel: stop immediate propagation and
preventDefault when cancelable. The guard expires on timeout/rAF, the next
press, consume, disconnect, or destroy. Keyboard pickup is unchanged. No Tree
CSS, public API, or native parity change.

Focused controller tests cover pre-threshold suppression, authored-style
restore including `!important`, exclusions, source-loss/disconnect/destroy,
activated click consumption and default cancellation for commit/reject/fail/
cancel, a registered link source, async dropping, and stale-guard expiry.
Paired Svelte/React Tree tests cover tap selection, rename/input exclusion,
and no trailing selection after those drop outcomes. Chromium and WebKit
probes prove empty Selection through a reorderable label drag, no trailing
selected value, a tap that still selects, a non-reorderable counterexample
that produces a real range, and a registered link source whose drag does not
follow href while a tap still does.
