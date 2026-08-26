# g16.008 — Native Text Event Routing Cleanup

Status: complete
Opened: 2026-08-26
Closed: 2026-08-26
Depends on: merged `g16.007`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/text-input.md`,
`../../contracts/components/code-input.md`,
`../../contracts/components/duration-input.md`,
`../../contracts/components/editable-label.md`

## Goal

Repair two generic GPUI text-event defects exposed by `g16.007` before another
editable control is admitted to mounted evidence. Keep Tab as sequential focus
traversal, preserve component-specific blur semantics, and make ephemeral text
state use the identity of the node that actually paints the value.

This is substrate hygiene, not another parity claim. It must leave the
generated evidence ledger unchanged at 37 mounted / 137 missing.

## Current Evidence

- `packages/gpui/node-backend/src/interaction.rs` maps both Enter and Tab to
  `Interaction::on_submit`. The `TextInput` contract assigns Enter to submit
  and Tab to focus traversal.
- `DurationInput` explicitly assigns Tab and Shift+Tab to segment traversal.
  `CodeInput` uses a focusable slot row and has no submit contract.
- `EditableLabel` commits on Enter and on blur. Its current native path relies
  on the backend's incorrect Tab-to-submit mapping instead of observing blur.
- The backend focuses a field root, while composite `TextInput` paints and
  caches its measured value under the derived `<field-id>-value` child id.
  Blur currently forgets only the root id, so measured text, scroll, blink,
  marked range, and composition state survive the reset intended to clear
  them.
- Childless input nodes such as native `EditableLabel` paint under their root
  id. Any state-key repair must preserve that direct-input case rather than
  assuming every input has a derived value child.

## Fixed Behaviour Envelope

### Key routing

- `Interaction::on_submit` means Enter submission. Update renderer-neutral
  comments/types that still describe it as Enter-or-Tab.
- Enter invokes submit once and does not enter the edit-key transition.
- Tab and Shift+Tab never invoke generic submit. They remain available to
  GPUI's sequential focus traversal and move to the next or previous tab stop.
- `TextInput` therefore submits on Enter and traverses on Tab.
- `CodeInput` traverses away on Tab without completing or mutating its code.
- `DurationInput` traverses between segment tab stops, then out of the
  component, without changing a segment.
- `EditableLabel` still commits on Tab, but for the contractually correct
  reason: Tab moves focus, blur commits the current draft once. Escape remains
  cancel and Enter remains explicit commit.

### Text-state identity and cleanup

- Derive the painted text-state key from the node shape in one backend helper:
  a childless input uses its root id; a composite field with a separately
  painted value uses the derived value-node id.
- Keystroke-side undo/redo and paint-side history must address the same key for
  both shapes. If the existing direct-input path is demonstrably mismatched,
  repair it inside this helper and retain focused undo/redo regressions.
- Blur clears transient measured-line, scroll, blink, marked-range, and
  composing-text entries for the actual painted key. It may defensively clear
  both root and derived keys where that keeps the helper simple and exact.
- Do not clear undo history merely because focus moved. History remains for the
  mounted lifetime of the field.
- Two fields with distinct explicit ids retain independent caches and history.

## Explicit Non-Claims

- No new mounted ledger cell closes. Existing `TextInput` evidence stays
  mounted; `CodeInput`, `DurationInput`, and `EditableLabel` stay at their
  current evidence levels.
- This card does not choose `NumberInput`'s raw-draft/value model, certify
  multiline or slug behavior, widen IME claims, promote accessibility, compare
  pixels, or admit Jetstream.
- This card does not introduce a new focus manager, native editor, key-command
  abstraction, compatibility alias, or GPUI patch/fork.

## Delivery

### 1. Correct the renderer-neutral event meaning

- Update the node interaction vocabulary so submit is documented as Enter,
  not Enter-or-Tab.
- Route only Enter to `on_submit` in the GPUI backend. Let Tab continue through
  the runtime's real focus machinery; do not simulate focus by calling
  component handlers or test helpers.
- Add focused backend coverage proving submit and traversal are distinct.

### 2. Preserve component contracts

- Add the smallest native `EditableLabel` blur observation needed to commit
  its current draft exactly once when Tab moves focus.
- Exercise TextInput, CodeInput, DurationInput, and EditableLabel through
  mounted GPUI key dispatch. Use real focus handles and host rebuilds where the
  component owns state.
- Keep Escape, Enter, Shift+Tab, disabled state, and multi-field ordering
  explicit enough to catch another generic remap.

### 3. Unify painted text-state identity

- Centralize root-versus-value key selection inside the node backend. Do not
  teach individual components about backend caches.
- Prove blur removes the transient state attached to the painted value. Retain
  the state that should survive refocus, especially undo history.
- Cover both a composite TextInput and a childless editable input so a fix for
  one shape cannot regress the other.

### 4. Close the bounded follow-up

- Mark the source triage note promoted/resolved by this card and add one August
  execution log with the exact tests and behavior repaired.
- Record the ledger invariant: no cell or derived total changes.
- Leave g16 at an orchestrator checkpoint for the next measured component lane.

## Acceptance

- [x] Node submit vocabulary and GPUI dispatch both mean Enter only.
- [x] Mounted TextInput Enter submits once; Tab and Shift+Tab traverse without
      submit or value mutation.
- [x] Mounted CodeInput and DurationInput traverse on Tab without completion or
      value mutation; DurationInput moves through its segment order.
- [x] Mounted EditableLabel Enter commits directly, Escape cancels, and Tab
      commits once through blur before focus advances.
- [x] One backend identity helper addresses the actual painted text node for
      composite and childless inputs.
- [x] Blur clears measured, scroll, blink, marked, and composing state for the
      painted key without discarding mounted-lifetime undo history.
- [x] Existing TextInput mounted evidence and retained composite text-entry
      regressions stay green.
- [x] The parity evidence ledger remains byte-for-byte unchanged at 37 mounted
      / 137 missing.
- [x] One August log records the repair and leaves NumberInput, multiline,
      slug, accessibility, visual comparison, and Jetstream open.

## Outcome

Complete. The full record is
`../../logs/2026-08/20260826-g16-008-native-text-event-routing-cleanup.md`.

Two further generic repairs were required to make the fixed envelope real, both
inside the same node/backend seam and neither a new focus architecture:

- gpui 0.2.2 binds no key to its own sequential traversal, so Tab now reaches
  `Window::focus_next`/`focus_prev` from the window host that already carries
  Escape — the same place a browser puts a document-level default action.
- a node declaring `Interaction::focusable` was not a tab stop, so no field,
  slot row or segment was reachable by keyboard once Tab stopped being
  intercepted. `focusable` with no declared `a11y.tab_index` now means a tab
  stop at index 0, which is what the vocabulary already documented and what the
  DOM does; `-1` still means programmatically focusable and skipped.

One consequence is recorded in `PAPERCUTS.md` rather than repaired here:
`DurationInput` and `TextInput` mark their component root `focusable`, so a
root that draws nothing of its own is now a tab stop ahead of its segments.

## Writable Scope

- `packages/contracts/node/` for the corrected submit-channel documentation
- `packages/gpui/node-backend/` and focused tests
- `packages/render/src/editable_label.rs` and focused tests only for
  contract-required blur commit
- the smallest real mounted regressions in `packages/gpui/preview/`
- existing TextInput, CodeInput, DurationInput, EditableLabel, and composite
  text-entry tests only where required to retain behavior
- this card, g16/front-door status, the source triage note, one August log, and
  `PAPERCUTS.md` only for new execution friction

Do not change public component props, redesign another editable component,
change ledger evidence, edit specimens or theme CSS, add visual fixtures,
change versions, edit workflows, publish, or touch downstream repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-node`, `poodle-render`, and GPUI node-backend tests;
- mounted TextInput, CodeInput, DurationInput, and EditableLabel routing tests;
- retained mounted composite text-entry regressions;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger` without generated changes;
- `effigy ci:native`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- GPUI does not provide real sequential focus traversal once Tab is no longer
  intercepted, and resolving that requires a new focus architecture.
- EditableLabel blur commit cannot be separated from a public API or ownership
  decision.
- Painted text identity cannot be derived from the current node shape without
  exposing backend state keys to components.
- The repair requires deciding NumberInput, multiline, slug, broad IME,
  accessibility, visual, Jetstream, release, or downstream concerns.
- Any proposed test passes by invoking handlers directly instead of driving
  mounted production dispatch and focus.

