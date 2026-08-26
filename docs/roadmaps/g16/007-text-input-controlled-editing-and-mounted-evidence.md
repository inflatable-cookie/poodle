# g16.007 — TextInput Controlled Editing And Mounted Evidence

Status: complete
Opened: 2026-08-26
Closed: 2026-08-26
Depends on: merged `g16.006`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/text-input.md`, `parity-evidence-ledger.md`

## Goal

Prove the core controlled-editing contract of `TextInput` through the real
GPUI node/backend/input path and host rebuild. Keep Svelte, React, shared Rust,
and GPUI aligned on the named portable behaviour. Register one honest mounted
evidence row without pretending that one regression certifies every mode,
pixel, or accessibility mechanism.

`TextInput` is the next high-leverage primitive after Tabs. Shared Rust
composites use it for command/search surfaces, settings, model connections,
embed entry, token entry, editable lists, relation pickers, and other forms.
The ledger currently records focused construction but no named mounted GPUI
behaviour.

This is a bounded core-editing lane. It does not solve the separate
`NumberInput` value-model decision, certify multiline layout, or implement the
slug source/autogeneration lifecycle.

## Current Evidence

- The contract defines host-controlled native value, caret/selection, and
  focus state; shared Rust already routes editing through
  `poodle_headless::text_input`.
- The GPUI backend already carries measured caret/selection paint, pointer
  selection, clipboard shortcuts, undo/redo state, and IME composition through
  renderer-neutral channels.
- Existing mounted composite tests type through `TextInput` descendants, but
  no named test owns the primitive's complete core-editing claim and the ledger
  therefore stays `missing`.
- Svelte and React focused `TextInput` tests currently prove autofocus,
  imperative focus, and search-clear ordering. They do not yet make the
  portable submit/cancel and disabled/read-only rules explicit beside the
  native proof.
- The native renderer derives a fallback field id when `id` is absent. The
  public contract requires `id`; this lane uses explicit ids for interactive
  fields and does not invent another identity fallback.

## Fixed Behaviour Envelope

### Controlled value, selection, and focus

- Start with an explicit field id, value, collapsed caret, and unfocused state.
- Pointer or programmatic focus reaches the real GPUI focus handle. The host
  receives focus gain, rebuilds with `is_focused=true`, and the mounted field
  exposes the caret/focus treatment.
- Printable input, replacement insertion, Backspace, Delete, Arrow Left/Right,
  Home/End, Shift+Arrow, and select-all run through the existing shared Rust
  edit transition. The host receives value and selection results, stores them,
  and rebuilds; tests do not call transition or handlers directly.
- Moving focus elsewhere reports loss once. The host rebuild removes the
  caret/focus state without changing the value or selection.
- Two mounted fields with explicit ids and equal values keep independent focus,
  selection, backend history, and composition identity.

### Commands and search clear

- Enter invokes submit with the current controlled value; Escape invokes
  cancel. Neither mutates the value by itself.
- A search field with a value renders one clear control. Activation reports the
  empty value before the clear command, the host rebuilds empty, and the clear
  control disappears.
- Disabled suppresses focus, editing, submit, cancel, and clear. Read-only
  remains focusable/selectable but suppresses value mutation and clear.

### Placeholder, limits, and semantic projection

- Placeholder copy is never treated as the value, selection, clipboard
  content, or undo history.
- `maxLength` constrains accepted inserted content before the host sees a new
  value. If the current shared Rust path does not enforce it, repair it in the
  shared pure transition/component boundary and add a focused regression.
- The mounted node retains one text-input role and projects label,
  disabled/read-only/required/invalid state, value, and selection channels as
  far as the current renderer-neutral vocabulary supports. These assertions
  support mounted behaviour only; GPUI accessibility remains `manual`.

## Explicit Non-Claims

- Multiline rows, wrapping, resize, vertical scrolling, and
  Cmd/Ctrl+Enter submission are not certified by this card. Do not alter their
  contract or silently describe the single-line node as a multiline pass.
- Slug normalization, source-following state, reserved-route validation, and
  source-driven callbacks are not certified. Their adapter/host state needs a
  separate semantic/API lane if current Rust cannot express it cleanly.
- Debounce/async validation timing and DOM-only attributes/events remain owned
  by their documented runtimes. Validation-state projection may be asserted;
  orchestration is not moved into the backend.
- Clipboard, undo/redo, and IME backend tests may be retained or strengthened,
  but this card does not claim OS input-method coverage from a headless test.
- GPUI visual comparison, broad native accessibility, Jetstream admission, and
  full TextInput parity remain separate evidence classes.

## Delivery

### 1. Lock portable web and shared-machine behaviour

- Add focused Svelte and React cases for submit, cancel, disabled/read-only
  suppression, and the existing ordered search clear result. Keep web-native
  autofocus, DOM events, debounce, validation orchestration, and imperative
  focus as adapter evidence rather than porting them into Rust.
- Extend `poodle_headless::text_input` focused cases only for a missing named
  core rule. Keep one Rust transition authority for character editing,
  selection movement/replacement, deletion, and insertion.
- If `maxLength` is currently absent from the Rust path, enforce it once before
  `on_change`; do not reproduce truncation in the GPUI backend or test host.

### 2. Drive controlled shared Rust TextInput

- Build the mounted fixture with `TextInputSpec` and
  `text_input_with_handlers`. Give every interactive field an explicit id.
- Store value, selection, and focus in the test host and rebuild the public spec
  after callbacks. Use the existing production node renderer; no test-only
  component or direct callback invocation satisfies the card.
- Repair measured renderer defects only where the current contract and shared
  edit machine decide the result. Keep owned results at the host boundary and
  avoid per-event compatibility wrappers.

### 3. Exercise the real GPUI backend

- Extend the headless driver only for a generic pointer/text input primitive
  already used by production GPUI. Any driver helper must dispatch the real
  backend event path rather than calling node handlers.
- Prove focus gain/loss, printable insertion, caret movement, selection
  replacement, deletion, submit/cancel, ordered search clear, disabled and
  read-only inertia, placeholder separation, and two-field identity.
- Retain focused backend tests for measured caret paint, pointer selection,
  clipboard/undo, and IME channels when implementation touches them. Do not
  create a fake native editor or windowed capture path.

### 4. Update evidence honestly

- Register the exact mounted regression name in
  `MOUNTED_BEHAVIOUR_TESTS`, regenerate the ledger through its generator, and
  move only `TextInput` from `missing` to `mounted` (36 → 37 mounted; 138 → 137
  missing).
- Keep GPUI accessibility `manual`, GPUI visual `missing`, and Jetstream
  `deferred`. Do not promote multiline, slug, validation timing, or OS IME as
  closed evidence.
- Add one August execution log naming the covered behaviour, defects repaired,
  exact test path, validation, and remaining TextInput/NumberInput gaps. Leave
  g16 at an orchestrator checkpoint.

## Acceptance

- [x] Focused Svelte and React tests agree on submit/cancel,
      disabled/read-only suppression, and value-change-before-clear ordering.
- [x] Shared Rust owns one tested edit transition for the named value and
      selection rules; max-length enforcement is component/shared-machine
      owned, never backend-owned.
- [x] One readable named GPUI regression mounts real `TextInput` nodes, drives
      real pointer/keyboard input, stores callback results, and rebuilds the
      host-controlled spec.
- [x] The mounted proof covers focus gain/loss, typing, movement, selection
      replacement, Backspace/Delete, submit/cancel, search clear,
      disabled/read-only inertia, placeholder separation, and independent
      equal-valued fields.
- [x] No mounted assertion passes by directly invoking a handler, transition,
      renderer function after mount, or spec inspection alone.
- [x] Existing composite text-entry regressions and focused backend
      caret/selection/clipboard/undo/IME tests remain green.
- [x] The generated ledger changes exactly TextInput's mounted-behaviour cell
      and derived totals. Accessibility, visual, and Jetstream statuses do not
      move.
- [x] One August log records the bounded claim and explicitly leaves
      multiline, slug lifecycle, async validation timing, OS IME, and
      NumberInput value-model closure open.

## Writable Scope

- focused `TextInput` tests under `packages/svelte/components/test/` and
  `packages/react/components/test/`; source changes only for a contract-backed
  defect exposed by those tests
- `packages/contracts/headless/src/text_input.rs` and focused tests
- `packages/contracts/components/src/text_input.rs` only for a bounded
  additive semantic/state correction required by the existing contract; no
  alias or constructor migration
- `packages/render/src/text_input.rs` and focused tests
- `packages/contracts/node/` and `packages/gpui/node-backend/` only for a
  directly measured generic text-input defect in an already documented channel
- the smallest headless-driver support and one coherent mounted regression in
  `packages/gpui/preview/`
- existing composite text-entry regression call sites only for mechanical
  adaptation; no composite redesign or ledger movement
- `scripts/parity-evidence-ledger.ts`, its focused test, generated ledger, this
  card, g16/front-door status, one August log, and `PAPERCUTS.md` for new
  execution friction only

Do not change multiline/slug public semantics, redesign `NumberInput`, add a
native editor architecture, patch/fork GPUI, edit specimens or CSS for visual
review, add visual fixtures, promote accessibility, admit Jetstream, edit
workflows, change versions, publish releases, or touch downstream repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused Svelte and React `TextInput` tests;
- focused `poodle-headless`, `poodle-node`, `poodle-render`, and GPUI
  node-backend text-input tests for every changed layer;
- the named mounted `TextInput` regression plus retained composite text-entry
  regressions;
- `effigy regressions:native`;
- `effigy probe:gpui-specimens`;
- `effigy test:parity-evidence-ledger`;
- `effigy check:parity-evidence-ledger`;
- `effigy ci:native`;
- `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch;
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, or publication selectors.

## Stop Conditions

- The named core-editing rule is missing from or contradicted by the active
  contract, Svelte, and React.
- Correct core editing requires choosing the unresolved multiline or slug
  ownership model, changing NumberInput, or adding a general native editor.
- Stable mounted identity cannot be achieved with the contract-required
  explicit field id and existing backend channels.
- The GPUI path needs raw runtime types in component contracts, a GPUI fork,
  window activation, direct handler invocation, or test-only production
  behaviour.
- Repair widens into a composite redesign, broad accessibility mapping, visual
  comparison, Jetstream, workflows, releases, or downstream repositories.
- The ledger changes any component except TextInput or promotes an evidence
  class beyond the exact named tests.

## Outcome

`maxLength` had no owner in the Rust path: `TextInputSpec::max_length` was
declared and read by nobody, so a native field took unlimited input while the
web pair relied on the native attribute. It is now enforced inside
`poodle_headless::text_input`'s two transitions — a keystroke into a full field
is consumed and reports nothing, an over-long insertion truncates to fit — and
EditableLabel's private post-truncation is gone from the transition paths.

An unchanged outcome used to be reported anyway: a rejected keystroke still
sent its unmoved caret through `on_selection_change`, and a paste with no room
left still sent the value the host already held through `on_change`. One
`report_edit` boundary in the component now decides what the host hears — no
value callback when the value did not change, no selection callback when the
caret did not move — so a rejected edit is distinguishable from an accepted
one. The keys are still consumed, so they cannot fall through to another
handler; there is simply nothing to report. Genuine movement and selection
replacement report exactly as before.

Every search field also rendered its clear button under the constant element id
`text-input-clear`, so two search fields shared one focus handle and one
paint-bounds entry. It is derived from the field id now, like the value node.

`text_input_controlled_editing_and_identity_rebuild_the_host_spec` mounts real
`TextInput` nodes and drives real GPUI pointer and keyboard input across three
hosts: one editable field, one search/disabled/read-only trio, and two fields
holding equal values. Ledger: 36 → 37 mounted; 138 → 137 missing.

Two findings are recorded but not repaired, because neither is decided by this
card's envelope:

- `packages/gpui/node-backend/src/interaction.rs` maps `tab` to submit
  alongside `enter`. Contract §Keyboard gives Tab to focus traversal. CodeInput
  and DurationInput may depend on the current mapping.
- `apply_listeners` calls `input_text::forget(&id)` on blur with the *field
  root* id, while `MEASURED`, `SCROLL`, `BLINK_EPOCH` and `MARKED` are keyed by
  the value node id — so the blur-time reset the contract describes never runs.
  Not observable through a public channel, so it was not repaired blind.

## Continuation

Return the mounted TextInput claim, exact defects/repairs, ledger change, and
remaining mode gaps to the orchestrator. Do not compile or implement
NumberInput from the open triage note. Leave the next task as an orchestrator
checkpoint.
