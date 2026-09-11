# 010 — CodeEditor editing focus treatment

Status: ready — operator-confirmed UX correction
Owner: Poodle web components
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/code-editor.md`,
`../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../../packages/core/src/dom/input-modality.ts`
Depends on: `g18.002`, `g18.008`

## Outcome

Keep CodeEditor’s outer focus treatment for keyboard navigation entry, then
remove it when editing begins. Typing inside an already-focused editor must not
paint or retain a large outer focus ring. Deliver the same behavior in Svelte
and React before the 0.4.0 release candidate is accepted.

Do not weaken focus visibility for keyboard navigation, falsify document-wide
input modality, or change CodeMirror’s caret and selection behavior.

## Ready-State Rubric

- [x] The operator observed the persistent ring in the merged Svelte specimen
  and confirmed it is wrong once typing begins.
- [x] The current mechanism is identified: every ordinary `keydown` sets the
  document modality to `keyboard`, while CSS keys the editor ring from that
  live global flag plus `:focus-within`.
- [x] The contract now distinguishes keyboard navigation entry from ongoing
  keyboard editing.
- [x] Svelte and React share the same public contract and correction boundary.
- [x] g18.009 publication is held; g18.006 must resume only after this repair
  merges so its candidate includes the accepted UX.

## Decisions

- Focus indication is an entry/navigation affordance, not a persistent active
  editor border.
- Track the editor’s focus-entry presentation locally. Do not write `pointer`
  into the document modality to hide the ring; that would corrupt unrelated
  controls’ focus origin.
- Tab or Shift+Tab entry shows the outer treatment. Pointer entry does not.
  Programmatic focus follows the existing documented modality policy and must
  not invent a new public option.
- The first actual editing intent dismisses the outer treatment before it can
  persist through typing. Cover text insertion, deletion, composition, paste,
  cut, undo, and redo. Navigation-only keys do not masquerade as edits.
- Leaving the component resets local state. Later keyboard navigation entry
  shows the treatment again. Caret, selection, diagnostics, and focused
  internal controls remain visibly distinguishable.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch; g18.006 is operator-paused and
  resumes on the same task/workspace only after this task merges; g18.009 stays
  held
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** CodeEditor contract; shared CodeEditor styles and a
  bounded framework-free local focus-entry helper if justified; Svelte and
  React CodeEditor wrappers/engines; focused component and browser interaction
  tests; the paired CodeEditor preview specimens only for observable regression
  proof; one g18.010 execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.006 and g18.009 tasks/workspaces/PRs, package versions,
  release notes, workflows, Desktop repository
- **Worker:** web interaction/accessibility worker comfortable with browser
  focus heuristics, IME/input events, CodeMirror, Svelte, and React
- **Excluded:** global modality redesign; focus changes to other components;
  CodeEditor public props; editor feature work; package versioning; release,
  tag, publication, Desktop edits, native editor work, windowed selectors
- **Escalation:** Chatterbox for any need to change the global input-modality
  contract, add a public focus prop, weaken keyboard navigation visibility, or
  touch the paused release lanes

## Work

1. Bind the current failure in both wrappers: pointer-focus then typing and
   keyboard-entry then typing leave the editor with an outer ring because the
   document modality changes on ordinary keydown.
2. Add the smallest component-local focus-entry state that distinguishes
   keyboard navigation arrival from active editing without changing global
   modality.
3. Dismiss the outer treatment on real editing intent across direct typing,
   deletion, composition, clipboard mutation, undo, and redo. Preserve it for
   keyboard entry and navigation-only keys until editing begins.
4. Reset on focus leaving the component and prove a later Tab/Shift+Tab entry
   shows the treatment again. Preserve focused internal-control indication.
5. Exercise the real Svelte and React CodeEditor surfaces in browser tests and
   the paired specimens. Run focused component tests, preview builds, relevant
   accessibility checks, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Typing does not paint a ring | pointer enters, first printable key flips global modality, outer ring appears | browser style/attribute assertion before and after real input in Svelte and React |
| Keyboard entry remains visible | Tab enters with no outer treatment | Tab and Shift+Tab entry assertions in both wrappers |
| Editing dismisses entry treatment | Tab entry ring remains throughout typing or IME | first insert/delete/composition mutation removes only the outer treatment |
| Mutation routes agree | paste, cut, undo, or redo retain/recreate the ring | representative `beforeinput`/transaction routes bound in focused tests |
| Navigation is not editing | Arrow, Home, End, F8, or search traversal dismisses entry indication as if content changed | planted navigation-only key cases preserve entry state until edit or exit |
| Global state stays truthful | component writes document modality to `pointer` to suppress itself | document attribute remains keyboard while local ring state dismisses |
| Re-entry resets | once dismissed, keyboard focus can never show again | blur beyond component then Tab/Shift+Tab re-entry restores treatment |
| Other focus cues survive | caret, selection, diagnostic, or internal-control focus becomes invisible | visual/DOM assertions retain those local cues |
| Both frameworks match | only the Svelte specimen is repaired | paired mounted browser cases and preview builds |

## Stop conditions

- Stop if the fix requires changing document-wide modality semantics or adding
  a public focus-treatment prop.
- Stop if IME or controlled-update behavior regresses; do not trade editing
  correctness for presentation.
- Stop before modifying, merging, cancelling, or replacing g18.006/g18.009;
  their retained state is resumed through Queue after this repair closes.

## Evidence

Inspection on 2026-09-11 found the ring selector in
`packages/core/src/styles/code-editor.css` keyed to
`:root[data-poodle-input-modality="keyboard"] .poodle-code-editor:focus-within`.
The shared modality installer writes `keyboard` for every non-modifier
`keydown`, including ordinary typing. The contract previously said only
“Keyboard focus,” which failed to distinguish navigation entry from editing.

## Next task

After merge and closeout, keep g18.011 held until parallel g18.013 and its
serial g18.014 also merge; then perform the operator-required acceptance sweep
across all three new editor surfaces. Keep g18.006 blocked and g18.009 held
until the sweep and all release-blocking repairs are accepted.
