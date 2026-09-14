# g18.036 — Slider-role drag boundary

Owner: Poodle web drag-and-drop substrate
Created: 2026-09-14
Governing refs: `../../architecture/011-drag-and-drop-substrate.md`,
`../../specs/069-dependable-drag-and-drop-substrate.md`,
`../../../packages/core/src/dom/drag-drop-controller.ts`
Depends on: none
UI classification: refinement — restore the existing interactive-descendant
gesture boundary; no visual or workflow decision

## Outcome

A pointer gesture beginning on an element with `role="slider"`, or one of its
descendants, never starts a Poodle drag session from an ancestor registered
source. Slider, Fader, Knob, and consumer-defined ARIA sliders can keep their
continuous gesture without a consumer-side `data-poodle-no-drag` workaround.

## Ready-State Rubric

- [x] The defect is reproduced by the shared `INTERACTIVE_SELECTOR`, which
  covers native controls and `role="button"` but omits `role="slider"`.
- [x] Spec 069 already requires whole-row drag sources to skip interactive
  descendants and architecture 011 keeps slider/fader gestures outside the
  payload drag machine.
- [x] Loophole has one retained workaround and no active implementation lane;
  this task edits Poodle only.
- [x] The Poodle Queue is empty and the post-0.4.0 consumer sweep is the
  canonical next frontier.
- [x] Scope, evidence, stop conditions, and the no-release boundary are fixed.
- [x] UI classification is a refinement with no presentation decision.

## Decisions

- Add only `[role='slider']` to the shared interactive-descendant selector.
  Do not broaden this task into an ARIA widget-role inventory.
- Match descendants through the existing `closest(...)` boundary so gestures
  on slider internals are protected as well as gestures on the role host.
- Keep `[data-poodle-no-drag]` as the explicit escape hatch for non-semantic
  consumer surfaces. Do not remove or rewrite Loophole's current workaround.
- Preserve public drag APIs, source handles, thresholds, pointer capture, and
  all keyboard drag behavior.

## UI Design Brief

Classification: refinement. The visible UI does not change. Pointerdown on a
slider remains owned by that continuous control; pointerdown on the remaining
registered source chrome retains the existing drag behavior. Keyboard focus,
labels, styling, and motion are unchanged.

## Dispatch manifest

- **State:** ready; sole active Poodle product task; independent of Loophole's
  retained workaround lane.
- **Completion:** exact role-slider counterexamples pass, the existing
  interactive-descendant matrix remains green, one bounded web board passes,
  and an independently reviewed PR merges through Queue closeout.
- **Owned mutable paths:**
  `packages/core/src/dom/drag-drop-controller.ts`,
  `test/headless-dom/drag-drop-controller.test.ts`, and
  `docs/specs/069-dependable-drag-and-drop-substrate.md`.
- **Reserved closeout surfaces:** this task and submitted handoff, g18 README,
  roadmap root/index/dispatch, lifecycle state/projections, `PAPERCUTS.md`, and
  any execution log.
- **Worker:** automatic general pool; this is a small shared DOM behavior
  correction with an exact regression seam.
- **Excluded:** Loophole or other consumer edits, removal of consumer
  workarounds, other ARIA roles, public API additions, Svelte/React component
  changes, native/GPUI behavior, release preparation or publication, workflow
  edits, and broad drag-substrate redesign.
- **Escalation:** return to Chatterbox if protecting `role="slider"` requires
  changing source registration semantics or exposes a broader role-policy
  decision.

## Work

1. Add a failing focused DOM regression with a registered ancestor source and
   nested `role="slider"` host. Drive pointerdown and threshold-crossing move
   from both the role host and a child inside it; assert no selection
   suppression, capture, or drag session begins.
2. Keep a control proving pointer movement from ordinary source chrome still
   activates the source under the existing threshold rules.
3. Add `[role='slider']` to the shared selector and update spec 069's exact
   interactive-descendant inventory.
4. Run the focused DOM test, then one `effigy ci:web` final board and
   `git diff --check`. Do not stack `test:components`, `docs:check`, or `qa`
   around the green board.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Slider gestures stay local | pointerdown on the `role="slider"` host crosses the drag threshold and starts the ancestor source | focused DOM regression remains idle and leaves root selection styles untouched |
| Slider internals stay local | pointerdown on a thumb/label child inside the role host bypasses an exact-target check | nested-child regression proves the existing `closest(...)` boundary |
| Source chrome still drags | adding the role selector disables the entire registered member | same fixture starts the source from a non-slider sibling after the normal threshold |
| Existing interactive controls remain protected | the selector change regresses native controls, `role="button"`, contenteditable, or `data-poodle-no-drag` | existing drag-drop controller matrix stays green |
| Framework behavior remains shared | one wrapper receives a component-specific workaround | diff is confined to the core controller, its contract, and focused shared tests; `ci:web` passes |
| Consumer ownership stays intact | the task edits Loophole or deletes its workaround before a released Poodle version is adopted | exact diff contains no consumer path and no cross-repository mutation |

## Stop conditions

- Stop if the fix needs a public option, changes pointer/keyboard source
  semantics, or requires deciding a general ARIA widget-role policy.
- Stop if an active Poodle or Loophole worker/PR starts owning the same behavior.
- Stop after one failed final web board with the named failing leaf; repair the
  candidate and rerun only when that candidate changes.
- Do not prepare or publish an npm patch from this task.

## Evidence

- `INTERACTIVE_SELECTOR` currently contains native controls,
  `[role='button']`, contenteditable hosts, and `[data-poodle-no-drag]`, but not
  `[role='slider']`.
- Poodle's Fader and Knob contracts expose `role=slider` at the root.
- Loophole `PAPERCUTS.md` records that its strip source needs an explicit handle
  plus `data-poodle-no-drag` on the body because the shared selector omits that
  role. Its existing workaround remains consumer-owned and is not a competing
  implementation lane.
- Queue inspection on 2026-09-14 found no unfinished Poodle or Loophole task.

## Next task

Return to Chatterbox after merge. This repair may join a later compatible npm
patch batch, but it authorizes no release mutation or automatic successor.
