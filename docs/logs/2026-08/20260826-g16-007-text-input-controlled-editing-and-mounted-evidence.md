# g16.007 — TextInput Controlled Editing And Mounted Evidence

Date: 2026-08-26
Status: complete — PR pending operator review
Branch: `t3code/text-input-mounted-evidence`
Card: `docs/roadmaps/g16/007-text-input-controlled-editing-and-mounted-evidence.md`

## Outcome

`TextInput`'s core controlled-editing contract now has one named mounted GPUI
regression. Focus is the backend's, the edit rules are shared Rust's, and the
value and caret are the host's — restated as props on every frame. No mounted
assertion invokes a handler, a transition, or a renderer function directly.

The generated ledger moves only TextInput's GPUI mounted-behaviour cell from
`missing` to `mounted`. Summary: 36 → 37 mounted; 138 → 137 missing. GPUI
accessibility stays `manual`. GPUI visual stays `missing`. Jetstream stays
deferred. No other component's cell moves.

## Defect: maxLength had no owner in the Rust path

`TextInputSpec::max_length` was declared, rendered into the character counter,
and read by nothing that could stop a value growing. A native field took
unlimited input while Svelte and React relied on the browser's `maxlength`
attribute.

Enforcing it in a backend was never an option: a backend can only clamp the
*result*, which deletes the tail of a full field when the caret is at its
start. So the limit went where the rest of the text rules already live.

- `packages/contracts/headless/src/text_input.rs` — `edit_transition` and
  `insert_transition` take the field's limit and spend it against the range the
  insertion replaces. A printable key into a full field is still *ours* (so it
  cannot fall through to another handler) but reports no value and leaves the
  caret alone; an over-long insertion truncates to fit, as a browser paste
  does; a deletion is never blocked, so a host that hands down an over-long
  value can still edit it down.
- `packages/render/src/text_input.rs` — passes `spec.max_length` at all three
  edit doors (root keys, root insertion, value-node insertion).
- `packages/render/src/editable_label.rs` — passes its own `max_length` into
  the shared transitions; its private post-truncation is gone from those two
  paths. The whole-value `on_text_change` channel keeps its clamp, because that
  one is a replacement rather than an edit.

## Defect: unchanged outcomes were reported anyway

Found in orchestrator review of PR #81. The component reported every consumed
edit outcome: a keystroke rejected by `maxLength` still sent its *unmoved*
caret through `on_selection_change`, and a paste with no room left still sent
the value the host already held through `on_change`. A controlled `on_change`
means "the value is now this", so a host that counts edits, marks a form dirty,
or debounces a save saw an edit that never happened.

`packages/render/src/text_input.rs` now has one `report_edit` boundary that all
three edit doors go through: no value callback when the next value equals the
current one, no selection callback when the outcome selection equals the input
selection. The keys stay consumed — swallowing them is what stops them falling
through to another handler — there is simply nothing to report. Genuine caret
movement and selection replacement are unchanged.
`packages/render/src/editable_label.rs` already guarded its value callback and
reports no selection, so it needed nothing.

Both the focused and the mounted proof now assert the **complete** callback log
is empty for a rejected edit rather than filtering it: the earlier assertion
passed while the host was still receiving `name/select:6-6`.

## Defect: search fields shared one clear-button identity

Every search field rendered its clear button under the constant element id
`text-input-clear`. The GPUI backend keys focus handles and paint bounds by
element id, so two mounted search fields shared one focusable control. It is
now derived from the field id — `poodle-input-{id}-clear` — the same way the
value node already was.

## Portable web and shared machine

- `packages/svelte/components/test/TextInput.test.ts`,
  `packages/react/components/test/TextInput.test.tsx` — Enter submits the
  current value and Escape cancels, neither reporting a value change; disabled
  and read-only render their native state and no clear control; a read-only
  field still submits; `maxLength` reaches the native limit. The existing
  autofocus, imperative-focus, and value-change-before-clear cases are
  unchanged. Public props are unchanged.
- `packages/contracts/headless/src/text_input.rs` — four focused cases for the
  limit: full-field rejection, selection budget, deletion below the limit, and
  truncated insertion.
- `packages/render/src/text_input.rs` —
  `a_rejected_full_field_edit_reports_nothing_at_all` logs every channel in one
  ordered list and asserts it is empty for a full-field key, a full-field
  paste, and Backspace at index 0; the same test asserts that a genuine edit
  still reports value then caret in order, that an over-long paste truncates
  and reports the truncated result, and that a caret move with no edit reports
  the caret alone. Plus a focused case for per-field clear identity.

## Mounted test

`packages/gpui/preview/tests/headless_regressions.rs#text_input_controlled_editing_and_identity_rebuild_the_host_spec`

Three mounted hosts, each rebuilding the public `TextInputSpec` from stored
value, selection, and focus after every reported callback:

- one editable field — pointer focus reaching the real focus handle, focus gain
  reported once, `end`/`home` caret movement, printable insertion, `maxLength`
  rejection and an inert Delete each leaving the complete callback log empty,
  `shift-right` extension, typing over the
  selection, Backspace, Delete, Enter submit and Escape cancel with the value
  untouched, placeholder text separated from the value by the caret channel's
  `showing_placeholder` flag, and focus loss reported exactly once with value
  and caret intact;
- a search / disabled / read-only trio — clear reports the empty value *before*
  the clear command and then disappears; a disabled field has no focus handle
  at all and reports nothing; a read-only field takes real backend focus,
  reports selection, still submits, and never mutates;
- two fields holding equal values — independent focus state, independent
  values, independent carets, and independent backend undo history.

Retained green: `effigy regressions:native` (79 tests, including the existing
LicenceActivation, key-validation, machine-name, and ModelCatalogueEditor
text-entry regressions), the node backend's caret/selection/clipboard/undo/IME
tests (36), and `effigy probe:gpui-specimens` (8).

The headless driver needed no new primitive: mount, real pointer press and
release at painted bounds, real key dispatch, real focus and blur through the
backend registry were already there.

## Findings recorded, not repaired

- `packages/gpui/node-backend/src/interaction.rs` maps `tab` to submit
  alongside `enter`. Contract §Keyboard gives Tab to focus traversal.
  CodeInput and DurationInput may depend on the current mapping, so this is an
  orchestrator decision, not a repair inside this card's envelope.
- `apply_listeners` calls `input_text::forget(&id)` on blur with the *field
  root* id, while `MEASURED`, `SCROLL`, `BLINK_EPOCH` and `MARKED` are keyed by
  the value node id (`{id}-value`). The blur-time reset the contract describes
  ("Cleared on blur, so a field re-read from the start") therefore never runs.
  There is no public channel that observes it, so it was not repaired blind.

## Remaining gaps

- Multiline rows, wrapping, resize, vertical scrolling, and Cmd/Ctrl+Enter
  submission are not claimed.
- Slug normalization, source-following state, and reserved-route validation are
  not claimed.
- Debounce and async-validation timing stay web adapter effects.
- OS input methods are not claimed from a headless test. The backend's IME
  channel tests are retained mechanism proof, nothing more.
- `NumberInput`'s value model stays open in
  `docs/triage/20260826-213343-number-input-native-value-model.md`.
- GPUI accessibility remains `manual`; GPUI visual comparison remains
  Button-only.
- `effigy doctor` was already red on the planning base (generated-in-src,
  god-files, stale-suppressions). That baseline is unchanged and was not
  absorbed.
- The next evidence decision belongs to the orchestrator after review.
