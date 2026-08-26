# g16.008 — Native Text Event Routing Cleanup

Date: 2026-08-26
Status: complete — PR #82, pending operator merge authority
Branch: `t3code/native-text-event-routing-cleanup`
Card: `docs/roadmaps/g16/008-native-text-event-routing-cleanup.md`
Source triage: `docs/triage/20260826-224901-text-input-native-event-cleanup.md`

## Outcome

Two generic GPUI text-event defects recorded but not repaired by `g16.007` are
closed in the node/backend seam. Enter is submission and Tab is traversal;
transient text state is keyed by the node that actually paints a field's value.
TextInput, CodeInput, DurationInput, and EditableLabel keep their contracts,
proven through mounted key dispatch and gpui's own tab-stop order.

No ledger cell moves. The generated parity evidence ledger is byte-for-byte
unchanged at 37 mounted / 137 missing, and `git status` confirms it was never
written.

## Defect: Tab was routed through the submit channel

`Interaction::on_submit` documented itself as "Enter or Tab", and
`interaction.rs` matched `"enter" | "tab"` before forwarding edit keys. Every
field carrying a submit handler swallowed Tab — TextInput's contract §Keyboard
gives Tab to focus traversal, DurationInput assigns it to segment traversal,
and CodeInput has no submit contract at all. EditableLabel *did* commit on Tab,
but for the wrong reason: its contract says Tab moves focus and blur commits.

- `packages/contracts/node/src/lib.rs` — `SubmitHandler`,
  `Interaction::on_submit`, and the `NodeKey` note now say Enter, and say why
  Tab is absent for a different reason from Escape: it is traversal, which the
  backend owns outright, so no node ever sees it.
- `packages/gpui/node-backend/src/interaction.rs` — Tab returns from the field
  listener before submit, cancel, or the edit-key transition. Returning ends the
  listener, not propagation, so the keystroke still bubbles.

## Repair: Tab had to reach real traversal, and there were no tab stops

Removing the intercept exposed two things the card's envelope assumed already
worked. Both are inside the same seam, and neither is a new focus architecture:
gpui 0.2.2 already owns sequential traversal and the tab-stop map.

- gpui **binds no key** to `Window::focus_next`/`focus_prev`; an application
  does. `attach_overlay_host` — the window-level root the production preview and
  the headless driver both use, already carrying Escape — now maps Tab and
  Shift+Tab onto them. That is where a browser puts a document-level default
  action, and it keeps the gesture out of every component.
- `Interaction::focusable` documents itself as participating in focus
  traversal, but the backend gave a focusable node's handle `tab_stop(false)`
  unless `a11y.tab_index` was declared. Nothing in Poodle was keyboard-reachable
  by Tab. A focusable node with no declared index is now a tab stop at index 0 —
  the DOM default for `<input>` and `<button>` — and `-1` still means
  programmatically focusable and skipped. Measured before and after: with the
  old flags `window.focus_next()` could not leave a mounted TextInput at all.

## Defect: transient text state was keyed by the wrong node

Keys and focus land on a field's focusable root. The *value* is painted
somewhere else, and where depends on the shape: a composite `TextInput` paints
a derived `<field-id>-value` text child among affixes and counters, while a
childless input — native `EditableLabel`'s editing field — paints itself. The
backend had one hardcoded derivation, `history_key(id) = "{id}-value"`, so:

- undo and redo on a childless input addressed a node that never existed, and
  found no history to restore;
- blur called `input_text::forget(&id)` with the root id, so the composite
  field's measured line, scroll offset, blink epoch, marked range and composing
  text all survived the reset that exists to clear them.

`packages/gpui/node-backend/src/input_text.rs` now derives the key from the
node's own shape in one place: `painted_key` finds the descendant the backend
would turn into an `InputText` and returns the element id it paints under,
falling back to the root. It resolves ids the way the rest of the backend does,
so a runtime-stamped value node is addressed by what it actually painted under
rather than by the `-value` convention. Blur clears both the root and the
painted key. Undo history is not transient and is not cleared — it reaches back
across a focus excursion for the mounted lifetime of the field.

## Defect: the first frame reported a blur

`FOCUS_STATES.insert(id, now) != Some(now)` treats the first observation of an
unfocused node as a change, so every node reported `on_focus_change(false)` on
its first painted frame. Harmless for a host that only stores the flag; fatal
for a field that commits on blur, which committed and left edit mode before it
was ever focused. A node that never held focus did not lose it, so the first
observation now reports only a gain.

## Component contracts preserved

`packages/render/src/editable_label.rs` — the editing input observes blur
through `on_focus_change` and commits the current draft. Enter still commits
through `on_submit`, Escape still cancels, and the public
`EditableLabelHandlers` surface is unchanged. The host owns `is_editing`, so a
commit that ends the edit unmounts the field before a second can arrive; the
shared machine's guard against commit-after-cancel is restated by the fixture
host.

## Focused tests

`packages/gpui/node-backend/src/tests.rs` (39 tests, was 36):

- `the_painted_text_key_follows_the_node_that_draws_the_value` — composite,
  childless, and paints-nothing shapes;
- `the_painted_text_key_uses_the_element_id_the_backend_paints_under` — a
  runtime-stamped value node, and a multiline value that measures nothing;
- `blur_clears_transient_text_state_and_keeps_undo_history` — the five
  transient entries go, history stays, and the retained snapshot restores.

`packages/contracts/node` (8) and `packages/render` (431) stay green.

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs` — 79 → 83 tests. Every
claim drives real key dispatch through gpui's dispatch tree and real focus
handles; nothing invokes a component handler, a transition, or a focus helper
as a shortcut.

- `text_input_submits_on_enter_and_traverses_on_tab` — Enter submits exactly
  once and never reaches the edit transition; Tab moves focus and the host
  hears nothing else; Shift+Tab walks the same order back; Enter and Escape
  still belong to whichever field holds focus; two stops wrap.
- `blur_clears_the_painted_field_state_and_keeps_its_undo_history` — a
  composite field whose root owns no painted state and whose value child owns
  all of it, an input-method mark cleared by blur, and undo reaching back after
  refocus; then a childless input proving the same repair on its own id, with
  `<id>-value` holding nothing, and the neighbouring field sharing neither.
- `code_and_duration_inputs_traverse_on_tab_without_mutating` — CodeInput is
  one stop and neither Tab completes a code one key from full, while a digit at
  that stop still types and completes; DurationInput is crossed by five Tabs
  without a segment reporting, and Shift+Tab plus an arrow key proves the
  seconds/minutes/hours order by what each stop acts on.
- `editable_label_commits_on_enter_and_once_through_the_blur_tab_causes` —
  Enter commits once; Escape cancels and leaves nothing behind for a blur;
  Tab produces `label/commit` *before* the next field's focus gain, focus
  really advances, and further frames produce no second commit.

Retained green: the existing `g16.007` TextInput mounted evidence, the
LicenceActivation / key-validation / machine-name / ModelCatalogueEditor
text-entry regressions, and `effigy probe:gpui-specimens` (8).

## Ledger invariant

`effigy test:parity-evidence-ledger` (5) and `effigy check:parity-evidence-ledger`
(175 rows) pass with no generated change. GPUI mounted behaviour stays
37 mounted / 137 missing. CodeInput, DurationInput, and EditableLabel keep
their current evidence levels: this card proves routing, not parity.

## Validation

`effigy regressions:native` (83), `effigy probe:gpui-specimens` (8),
`effigy ci:native`, `effigy test:parity-evidence-ledger`,
`effigy check:parity-evidence-ledger`, `effigy docs:check`, `effigy qa`,
`cargo test -p poodle-node` (8), `cargo test -p poodle-render` (431),
`cargo test -p poodle-gpui-node-backend` (39), and
`git diff --check origin/main...HEAD`.

## Recorded, not repaired

- `DurationInput` and `TextInput` mark their component root `focusable`. That
  was inert while nothing was a tab stop; it now means a root that draws
  nothing of its own takes a Tab before its segments do. Recorded in
  `PAPERCUTS.md` — it is a component declaration, not this card's seam.
- `CodeInput` and `DurationInput` carry no id prop, so their focusable nodes
  have no element id and no retrievable focus handle. The mounted tests observe
  those stops by what a key does at them, which is real evidence but not the
  same as reading focus back.

## Remaining gaps

- No new ledger cell closes. CodeInput, DurationInput, and EditableLabel are
  not claimed as mounted parity.
- `NumberInput`'s value model stays open in
  `docs/triage/20260826-213343-number-input-native-value-model.md`.
- Multiline, slug, validation timing, broad IME, native accessibility, visual
  comparison, and Jetstream admission are all unchanged and unclaimed.
- `effigy doctor` was already red on the planning base (generated-in-src,
  god-files, stale-suppressions). That baseline is unchanged and was not
  absorbed.
- The next evidence lane is an orchestrator checkpoint against the unchanged
  37 mounted / 137 missing ledger.
