# 018 — RichTextEditor controlled-echo selection preservation

Status: ready behind g18.013 — operator-confirmed release blocker
Owner: Poodle web editors
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/rich-text-editor.md`,
`../../../packages/svelte/components/src/RichTextEditor.svelte`,
`../../../packages/react/components/src/RichTextEditor.tsx`,
`../../../packages/svelte/components/src/rich-text-engine.ts`,
`../../../packages/react/components/src/rich-text-engine.ts`
Depends on: merged g18.013

## Outcome

Make an accepted controlled echo of a `RichTextEditor` user transaction a true
no-op in both web wrappers. Typing, replacing a selection, or committing IME
text in the middle of a document must preserve the live caret or selection
instead of moving it to the document end.

Preserve real host authority: a genuinely different host document still
replaces or rejects the local edit without a callback echo.

## Ready-State Rubric

- [x] The operator reproduced the caret jumping to the document end after each
  typed character and approved the repair.
- [x] Both public specimens immediately echo `onChange` JSON through `value`.
- [x] Both wrappers use object identity to classify every emitted JSON object as
  a new host push.
- [x] Both engines route that accepted echo through TipTap `setContent`, which
  replaces the document and selection.
- [x] Existing host-echo tests assert content and callback count but never bind
  caret, non-collapsed selection, history or composition continuity.
- [x] g18.013 is active on the same RichTextEditor shells, so this task is
  dependency-queued behind it rather than creating a conflicting workspace.

## Decisions

- Controlled equality is semantic document equality, not JavaScript object
  identity. An echo matching the editor's current emitted ProseMirror JSON must
  advance host-acceptance bookkeeping without calling `setContent` or remounting.
- Keep ProseMirror JSON as the public controlled value. Do not expose TipTap
  editors, transactions, steps, history or selections to consumers.
- Preserve the existing immediate-edit model: the editor applies a local user
  transaction, emits the complete document once, and the host may accept it,
  replace it, or restore the prior accepted document.
- A host value different from the live editor document remains authoritative
  and may use `setContent` without emitting `onChange`. A restored prior value
  must still reject the local edit.
- Bind both the ordinary same-object specimen echo and a structurally equal
  cloned echo. Neither may alter focus, caret, selection, scroll position or
  undo history.
- Bind repeated middle-of-document typing, non-collapsed selection replacement,
  paste and IME composition. The final callback payload, visible document and
  selection must agree after each accepted echo.
- Cover delayed or stale host values explicitly. Do not mistake an older emitted
  document for acceptance of a newer local transaction; the actual incoming
  host value remains authoritative.
- Keep the Svelte and React engine behavior identical apart from their existing
  framework header. Prefer one shared state law over wrapper-specific timing
  patches when the existing package boundary permits it.

## Dispatch manifest

- **State:** dependency-queued behind g18.013; may run in parallel with g18.014
  and g18.017 after that dependency closes; explicit prerequisite of g18.011
  and serial before retained g18.006; g18.009 waits on g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** paired RichTextEditor shells and private engines;
  focused Svelte/React component and browser tests; rich-text contract only for
  the semantic-equality clarification; paired rich-text specimens only where
  required to bind their existing controlled journey; one g18.018 execution log
- **Reserved closeout surfaces:** toolbar presentation owned by g18.013; image
  specimen behavior owned by g18.014; new public props or engine exports; other
  components; g18 README/index/dispatch; g18.006/g18.009/g18.011 task state;
  versions/lockfile; workflows; release/tag/publication; Desktop; native/GPUI/
  Jetstream
- **Worker:** web-editor worker comfortable with TipTap/ProseMirror transaction,
  selection, history and IME semantics across controlled Svelte and React shells
- **Excluded:** toolbar redesign; image-policy changes; new rich-text schema or
  commands; uncontrolled mode; selection callbacks; release work; workflow
  changes; Desktop
- **Escalation:** Chatterbox for any required public API change, inability to
  distinguish accepted echo from host rejection, loss of host authority, or
  release/workflow mutation

## Work

1. Plant paired mounted failures: place the caret in the middle of a multi-block
   document, type at least three characters through immediate controlled echoes,
   and prove focus and the exact advancing caret after every character.
2. Add a non-collapsed selection replacement, paste and IME composition journey.
   Assert one callback per committed transaction and retained selection/history.
3. Bind same-object and structurally equal cloned echoes as no-ops. Spy on or
   otherwise prove the document replacement path is not invoked.
4. Bind the counter-cases: a different host document replaces current content;
   restoring the prior accepted document rejects a local edit; a delayed stale
   host value cannot be treated as acceptance of newer editor state.
5. Repair the paired wrapper/engine bookkeeping. Preserve validation,
   feature-reconfiguration, table normalization, image insertion and no-echo
   semantics.
6. Exercise the existing controlled specimen journeys in both real previews so
   the human-visible reproduction is closed, not only the engine unit path.
7. Run focused paired component/browser tests, both package and preview builds,
   relevant accessibility checks, Effigy docs QA and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Accepted echo is a no-op | every fresh JSON object calls `setContent` | replacement-path spy plus stable editor document/selection identity |
| Repeated typing stays local | first character moves caret to document end | middle-document multi-character journey in both wrappers |
| Selection replacement survives | insertion works only for a collapsed caret | non-collapsed replacement with exact resulting range/caret |
| IME stays coherent | composition partials reset or commit at the end | composition journey with one committed callback and exact caret |
| History survives | accepted echo clears undo | edit/echo sequence followed by undo/redo proof |
| Focus and viewport survive | content stays right but focus or scroll jumps | active-element, selection and scroll assertions |
| Structural clone is harmless | only reference-identical echoes avoid reset | deep-cloned emitted JSON produces no replacement |
| Host authority survives | all equal-looking updates are ignored forever | different document and prior-value rejection counter-tests |
| Stale values are not accepted | delayed older echo blesses newer local state | ordered delayed-host journey with incoming value winning |
| Frameworks match | Svelte fixes timing while React still replaces | identical paired mounted journeys |
| Public boundary stays closed | repair exports transactions or selections | unchanged ProseMirror JSON API and private engine types |
| Release remains gated | candidate resumes before repair and sweep | merged g18.018 before g18.011 release and operator acceptance |

## Stop conditions

- Stop if the repair requires exposing engine or selection state publicly.
- Stop if accepted-echo preservation cannot coexist with genuine host
  replacement and rejection semantics.
- Stop before toolbar/image redesign, package dependency, workflow, release or
  Desktop changes.

## Evidence

Tom reproduced the caret moving to the end of the document after every typed
character on 2026-09-11. Source inspection traced the path from fresh
`onChange` JSON object identity in both wrappers to `engine.update({ value })`
and TipTap `setContent(..., { emitUpdate: false })`. The current host-echo test
asserts only content retention and callback count, leaving selection unbound.

## Next task

After this task and g18.013/g18.014/g18.017/g18.019 close, Queue dependencies
dispatch g18.011 for the complete four-surface editor acceptance sweep. Keep
g18.006 paused; g18.009 waits on it.
