---
title: g13 batch 048 — TextInput environment boundary, Rust authoring and the two web runtimes
status: complete
milestone: g13.007 (part 1 of 2 — **does not close the milestone**)
owner: Poodle core
updated: 2026-08-13
tags: [log, g13, IR, text-input, component, authoring, svelte, react, spec-063, g13.007, environment-boundary]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/048-text-input-slice-rust-authoring-and-web.md`
on branch `thread/g13-048-text-input-slice-rust-authoring-and-web`: authored
the TextInput definition in Rust (R1), serialized it to its own fixture,
emitted it to both web component packages' `generated/` directories through a
new select-only `text-input-ts` target, and rewired `TextInput.svelte` and
`TextInput.tsx` to read the rendered vocabulary — the anatomy classes, the
four `data-*` attribute names, and the five TXT-16 padding custom properties
— from the artifact instead of hard-coding them inline (R2). The R2 proof ran
live: renamed `data-validation-state` → `data-validation-level` in
`text_input.rs`, one `ir:build`, and both web previews emitted
`data-validation-level` on their TextInput with no hand edit; restored, both
emitted `data-validation-state` again. The typed capability boundary (R2) —
the card's deliverable — is declared for all six environment capabilities
(focus, selection, composition/IME, clipboard, measurement, native text
editing) plus the component-owned timers, with the per-capability ownership
split named in prose; **the two negative answers are recorded**: selection is
not a first-class capability name (it rides on `TextEditing` +
`Measurement`), and per-runtime ownership has no typed field in the IR —
it is prose in each capability's `purpose`. The editing model is untouched
(R5), no TS machine was created (R5), and `machines.json` passes unedited
with **no `text` key** — the text machine is unit-test-pinned only, a vector
gap recorded for `g13.008`.

Per the card's worker rules: no sub-agents; sources read directly (b045's and
b046's logs first — this is the third data point in their series — plus the
contract, both Rust sources, the full `ComponentDefinition` vocabulary in
`packages/contracts/ir/src/*` and its validation rules, and both web
components); no planning/status authority exercised. **One stop condition was
reached and resolved without a stop**: the card's required IME test — "a
composition sequence must not fire intermediate `onValueChange`" — failed
against the current components, because the web runtimes *did* fire
intermediate changes during composition (the browser fires `input` events
with `isComposing: true` for each buffer update, and `handleInput` had no
guard). The fix is a composition gate in both runtimes — buffer during
composition, commit exactly once on `compositionend` — which preserves the
"handled not at all" shape the measured table records (no
`compositionstart`/`update` interception of the composition's text editing;
the browser still produces the buffer). The composition text editing itself
remains 100% browser-native; the gate only filters when the value is
reported. No prop, attribute, or pixel moved.

## Measured before-state — the surface this card preserves (card steps 1–3)

**The 49-web-prop surface** (the card's counting authority: entries of the
Svelte `Props` interface) = 41 non-callback props + 8 callbacks. The 8
callbacks are declared as events per the Button/RangeSlider pattern:
`onValueChange`, `onSubmit`, `onCancel`, `onClear`, `onFocus`, `onBlur`, and
the Rust-only `onSelectionChange` are typed as events;
`onValidationChange` and `onKeyDown` have **no `EventKind`** and are
recorded as findings (see Vocabulary notes). Plus the 3 Rust-only props
(`selectionStart`/`selectionEnd`/`isFocused`, contract §3 "Rust targets
only") — the IR has no rust-only flag, so the definition records them as
portable props: 44 props + 7 typed events + 2 untyped callbacks = the full
T §3 surface.

**The 3 documented data attributes** (contract §9): `data-size`,
`data-density`, `data-validation-state`. The DOM also emits a fourth —
`data-type` — documented only in the corpus row TXT-18, not in the contract
prose; the definition carries all four emitted names, and the card's
3-documented count is preserved (the contract stays untouched — see Not
done).

**Validation/debounce semantics preserved** (TXT-11/12/28): `debounce`
delays `onValueChange` (flush on blur, immediate for clear and slug source
regeneration); `validationDebounce` (300) + `validateOnBlur` (true) gate
async validation; the `validationStatusToState` mapping with the
idle→caller/validating→pending/valid→valid/invalid→invalid table; slug
normalisation and built-in validation. None of it changed.

## The R2 capability boundary — typed, with the two negative answers

The six environment capabilities the milestone names, each a typed
[`Capability`] with a `purpose` naming what it means and who owns it
(`IR-05`/`IR-08`: declared, never implemented; no runtime reads machine
state from drawing code):

| Capability | IR type | Web owner | Rust owner |
|---|---|---|---|
| focus | `Capability::Focus` | the DOM (native input; `autofocus`, imperative `focus()`) | the backend (`on_focus_change`, caret drawing); `isFocused` is host-driven and does not drive the caret (TXT-21) |
| selection | `Capability::TextEditing` + `Capability::Measurement` | the DOM, entirely — the web components contain zero selection code (measured: 0 occurrences of selection terms in both components) | the host owns the caret position (`selectionStart`/`selectionEnd`, `onSelectionChange`), the backend owns drawing it (glyph measurement), the shared edit model owns the semantics (`edit_transition`, `word_range_at`, `selected_text`) |
| composition/IME | `Capability::Ime` | the DOM, natively — no composition-event interception; the value path is gated so intermediate buffer events never fire `onValueChange` (see the IME finding) | the platform text input handler (`Window::handle_input`), UTF-16 boundary, backend-owned marked range (TXT-24) |
| clipboard | `Capability::Clipboard` | the DOM, natively | the backend owns the platform clipboard; the shared model owns paste landing (`insert_transition`) and copy/cut source (`selected_text`) (TXT-23) |
| measurement | `Capability::Measurement` | the browser | the backend (`shape_line`/`x_for_index`, `closest_index_for_x`) (TXT-21/22) |
| native text editing | `Capability::TextEditing` | **the browser is the machine** — there is no TS text machine (R5) | the shared headless edit model, driven from key events and insertions |

Plus `Capability::Timers` — debounce/validation timing is web-component-owned
(`setTimeout` with destroy cleanup); the Rust targets have no timer surface.

**Negative answer 1 — selection is not a first-class capability name.** The
`Capability` inventory (`CROSS-17`) has no `Selection` variant; the
milestone's six names include selection, and the definition types it through
`TextEditing` (semantics) + `Measurement` (drawing). That is typed, but the
card's boundary question — "can the definition express that a capability
exists, what it means, and which runtime owns it" — has a weaker answer for
selection than for the other five: the *existence* is typed, the *ownership
split* is prose. Recorded as a finding for `g13.008`; not routed around.

**Negative answer 2 — per-runtime ownership has no typed field.** A
`CapabilityRequirement` is `capability` + `purpose`; there is no
per-runtime ownership map. The web/Rust split for every capability is
expressed in `purpose` prose. A typed ownership field would need a
`poodle-ir` schema change — which is explicitly out of this card's scope and
is `g13.007`'s finding (card Stop Condition 1 was checked against this: the
capability *names* all exist, so no stop; the ownership typing gap is
recorded here for `g13.008`).

**No stop condition reached on the boundary itself.** The boundary is
declared without generating lifecycle code (Stop Condition 2 not triggered);
honouring it changed no composition/selection/focus behaviour beyond the
required IME gate (Stop Condition 3 not triggered — the gate is the
*restoration* of the no-intermediate-fire semantics the card requires); no
visual baseline moved (Stop Condition 4 not triggered).

## R3 — the three questions, answered

### Q1 — Can one declaration serve both, with each runtime honouring the half it owns?

**Yes — one declaration serves both, because the declaration names the
boundary, not the implementation.** The six capabilities are the same typed
vocabulary on both sides; what differs is which runtime implements which
half, and that difference is exactly what "adapter capability" means
(`IR-05`). The web runtime honours the text-editing half by *delegating to
the browser* (no TS machine — R5); the Rust runtime honours it by driving
the shared headless model. The single declaration records both halves in the
capability `purpose`s, and the conformance vector pins the shared semantics
(the Rust targets' machine; the browser happens to provide the same
semantics natively). The declaration does not need to be per-runtime —
the asymmetry is in the ownership prose, not in the vocabulary. The stronger
reading of the question — can one declaration *execute* both halves — is
answered no by design: the web half has nothing to execute, which is the
measured asymmetry the milestone exists to capture.

### Q2 — For each of the six capabilities, name the owner per runtime and say whether the definition can express that ownership or merely note it in prose.

| Capability | Web owner | Rust owner | Expressible? |
|---|---|---|---|
| focus | DOM native input | backend (`on_focus_change`) + host `isFocused` prop | typed (the `Focus` capability); ownership in prose |
| selection | DOM (nothing to write) | host caret props + backend drawing + shared semantics | typed via `TextEditing` + `Measurement`; **ownership in prose** (no `Selection` capability, no ownership field) |
| composition/IME | DOM natively, value path gated | platform input handler, UTF-16 boundary | typed (`Ime`); ownership in prose |
| clipboard | DOM natively | backend clipboard + shared model landing | typed (`Clipboard`); ownership in prose |
| measurement | browser | backend glyph measurement | typed (`Measurement`); ownership in prose |
| native text editing | the browser (no machine) | shared headless model | typed (`TextEditing`); the *semantics* are additionally pinned by the `text-input` conformance vector — the only capability whose content is more than prose |

The honest answer: **every capability is typed (the enum name), and every
ownership split is prose.** There is no typed "owner per runtime" field in
the IR. Whether prose ownership counts as "expressible" is precisely the
question `g13.008` decides; this card records that the prose is the only
mechanism.

### Q3 — With 49 props mapping to 3 attributes, how much of TextInput's surface does the definition actually reach?

The number `g13.008` turns on, stated plainly:

- **Props: 49/49 reached as declared entries** — 41 non-callback props
  (names, types, defaults — the exact Svelte interface) + 7 callbacks typed
  as events + `onKeyDown` recorded (native passthrough, no `EventKind`).
  `onValidationChange` is the one callback the vocabulary cannot type at
  all (`PayloadKind::ValidationStatus` exists, no `EventKind` uses it).
- **Attributes: 4/4 emitted attributes declared** (3 contract-documented +
  `data-type` corpus-documented), with names, forms, emission policies, and
  value domains.
- **Anatomy: 10/10 parts declared**, with render conditions as bounded
  expressions (`canClear`, the validation-indicator gate, the affix
  presence) — the expression vocabulary reaches TextInput's conditional
  structure.
- **The behaviour the definition does NOT reach**: the validation
  orchestration (async timing, context merging, slug rules), the debounce
  timer machinery, the slug normalisation (`slugify`/`isValidSlugFormat`),
  the controlled/uncontrolled merge, the TXT-16 padding arithmetic, the
  value-dependent modifier classes (`--multiline`, `--over`,
  `--pending/--valid/--invalid`), and — by design — the editing model, IME,
  selection, and clipboard (the boundary). That remainder is the R7
  inventory below.

The ratio that matters: **49 props and 4 emitted attributes are *declared*
(vocabulary), and roughly half the component's runtime code is *behaviour*
the vocabulary deliberately does not reach.** The card's "small honest
number": of the 49 prop entries, the definition carries every name and
default, but the number of props whose *semantics* the definition executes
is **zero** — TextInput's value semantics are machine/browser-owned, and the
definition declares them. This is a larger hand-written remainder than
Button's and RangeSlider's (see the R7 comparison), and it is the
environment-boundary result the milestone exists to produce.

## Deliverables (only the card's writable paths)

- `packages/codegen/src/models/text_input.rs` (new) — the Rust-authored
  TextInput definition (`text_input_definition()` / `text_input_model()`):
  9 shared types (`text-input-type`, `validation-state`, `input-mode`,
  `resize-direction`, `enter-key-hint`, `autocorrect-mode` +
  `control-size`/`control-density`/`control-size-role`), 44 props (41 web
  non-callback + 3 Rust-only) + 7 typed events + 2 recorded untyped
  callbacks, the `value`/`defaultValue` DoNotMix controlled pair (T §3),
  10 anatomy parts (T §2) with expression render conditions, 4 `data-*`
  attributes + 5 TXT-16 style props, the size ladder with the contract's
  §8 table (token-relative `calc()` metrics recorded as `Text` values) and
  the density adjustments (compact/comfortable, inline AND block on the
  root — the documented §8 orthogonality exception), 13 semantic token
  refs, the 6 recipe hooks (the focus-fill chain runs hook → component
  variable → recipe hook → token, the Button-style chain), accessibility
  intent (native input role, aria-label/describedby/invalid/busy, 16
  native-attribute projections), the 6-capability boundary + timers (R2),
  the 17-command keyboard table (TXT-20/23/25), the 12-field VisualState
  projection (TXT-19), the `text-input` conformance reference (R5), and
  the two extensions (`jetstream-clear-only` TXT-31, `react-omits-autocorrect`
  OBS-03). Module header records the R2 answers, the placement, and the
  vocabulary notes.
- `packages/codegen/src/models/mod.rs` — `pub mod text_input;`.
- `packages/codegen/src/targets/text_input.rs` (new) — the `text-input-ts`
  target (output root `generated/text-input`, one `index.ts` per model).
  The artifact carries `parts` (id, name, full DOM class list — base +
  modifier for the affix/affordance parts), `attributes` (the four `data-*`
  names, forms, emission policies, value domains), `styleProps` (the five
  TXT-16 hook names + the visual field each is fed by), and `recipeHooks`.
  The generic attribute-domain projections are reused from
  `targets/button.rs` — the same sharing `range-slider-ts` uses; the
  `button-*`/`range-slider-*`/`shell-*` outputs did not move (R1, proven
  by `ir:check` byte-compare).
- `packages/codegen/src/targets/mod.rs` — `text-input-ts` registered in
  `selectable()`, **not** in `all()`: a plain `ir:build` over the synthetic
  fixture must never write into a web package.
- `packages/codegen/src/bin/poodle-codegen.rs` —
  `--author-text-input <OUT> [--check]`, the mirror of `--author-button` /
  `--author-range-slider`.
- `packages/codegen/fixtures/text-input-model.json` (new, 78 KB) — the
  serialized model, generated by `--author-text-input` after a validate
  round trip. Other fixtures untouched.
- `packages/{svelte,react}/components/src/generated/text-input/index.ts`
  (new) — the committed artifact both web components consume.
  Byte-identical in both packages (the parity test proves it).
- `packages/svelte/components/src/TextInput.svelte` and
  `packages/react/components/src/TextInput.tsx` — the components read the
  artifact: `parts`/`attributes`/`styleProps` records supply the anatomy
  classes, the four `data-*` names (via a computed-key spread), and the
  TXT-16 padding var names (Svelte emits all five; React emits the three
  shared ones — the pre-existing asymmetry, recorded in R7). The value
  derivations, the validation/debounce machinery, and the machine calls
  stay in the components (the runtime's projection, CROSS-13/14). No prop,
  default, class string, emitted value, or style var name changed (R4).
  Plus the IME composition gate in both (buffer during composition, commit
  once on `compositionend`) — see the IME finding.
- `packages/{svelte,react}/components/test/TextInput.generated.test.ts(x)`
  (new) — the definition→DOM tests: the four attributes emit under the
  artifact's names, the anatomy renders under the artifact's classes, the
  conditional parts (clear button, validation indicator, char count) gate
  on their expression conditions, the TXT-16 style props emit under the
  artifact's names, **IME composition does not fire intermediate
  `onValueChange`** (the acceptance line, both runtimes), and selection is
  untouched by re-renders (the DOM still owns it).
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` run
  `--author-text-input` first (write / byte-compare), then the fixture
  through `--target text-input-ts` into each web component package.
- `packages/codegen/tests/text_input.rs` (new, 9 tests) — see Tests.
- This log.

Nothing else in the repo changed. No `poodle-ir` change (the two negative
answers are findings, not schema gaps); no machine touched (`text_input.rs`
in `contracts/headless` and `render` — R5); no `poodle-render`, adapter, or
native preview touched (R6); no TS text machine created (R5);
`machines.json` untouched (R5); `synthetic-model.json`, `shell-model.json`,
`button-model.json`, `range-slider-model.json` and the button/range-slider/
shell artifacts untouched; no visual baseline refreshed (R4); the contract
`text-input.md` untouched (see Not done); no `poodle-ir`/`poodle-codegen`
dependency in either web package (asserted by test + `test:web-pack-install`).

## Design

- **The authoring form.** `text_input.rs` is ordinary Rust types and
  constructor helpers (spec 063 "Authoring Form"), no macros — the
  established 041/045 shape. Prop order is the contract's §3 table order;
  shared types carry `canonical_ref` to `text-input.md` (component-specific)
  or `004-shared-control-types.md` (cross-component).
- **The controlled pair is DoNotMix — the RangeSlider contrast.** The
  contract says "do not mix controlled and uncontrolled modes
  simultaneously" (T §3), so `controlled_state` carries the pair with
  `DoNotMix` — the shape b045 deliberately avoided for RangeSlider because
  React's pair is controlled-wins. TextInput's contract rule is exactly the
  IR's only rule; the definition records the contract instead of the
  implementation.
- **The four emitted attributes, three documented.** `data-type` is the
  fourth emitted attribute; the contract §9 documents three and the corpus
  row TXT-18 documents the fourth. The definition declares all four; the
  card's counting authority (the contract) is preserved untouched.
- **The five TXT-16 style props, Svelte 5 / React 3.** The definition
  declares the full TXT-16 vocabulary; Svelte emits all five, React emits
  the three shared padding reservations (the clear/trailing overlay insets
  are Svelte-only emissions whose CSS fallbacks — `0.5rem` — cover React).
  The asymmetry is pre-existing and unchanged; recorded in R7.
- **The capability boundary as prose ownership.** The `Capability` enum
  types the boundary; the ownership split is `purpose` prose. The card's
  acceptance — "typed, or its failure to be typed is recorded per
  capability" — is satisfied per capability: five of six are typed by a
  direct enum name; selection is typed via `TextEditing` + `Measurement`
  with the gap recorded; per-runtime ownership is uniformly prose.
- **The conformance vector points at the natives.** The `text-input`
  vector's `applies_to` is `[Gpui, Jetstream]` — the runtimes that
  implement the shared edit model. The web runtimes are deliberately absent:
  there is no TS text machine (R5), and the browser is the machine. The
  vector's step intents pin the shared semantics (`insert-at-caret`,
  selection-replaces, deletion semantics, caret-moves-do-not-edit,
  shift-extends, select-all-replaces, paste-lands-at-caret, word-boundaries,
  undo-coalescing, submit/cancel/tab fall-through) with the headless edit
  model's own unit tests as evidence — `machines.json` carries **no `text`
  key** (GAP-01), a fixed target (R5).
- **The expression vocabulary reaches TextInput's conditions.** The clear
  button gates on `isSearch && showClearButton && !disabled && !readOnly &&
  currentValue ≠ ""` as a bounded expression (type equality against the
  `text-input-type` member, boolean props, negation, emptiness); the
  validation indicator gates on `showValidationStatus &&
  effectiveValidationState ≠ none`; the affixes gate on
  `is_present && !is_empty`; the multiline `rows > 1` case is expressible
  (`is_present(rows) && gt(rows, 1)`-shaped ordering — the type checker's
  `Number` ordering). The value-dependent modifier classes (`--multiline`,
  `--over`, `--pending/--valid/--invalid`) have no sanctioned slot — the IR
  has no state-derived class mechanism — and stay hand-written (R7).

## The R2 proof (card step 6, live)

Renamed the `validation-state` attribute in
`packages/codegen/src/models/text_input.rs` (`data-validation-state` →
`data-validation-level`), ran `effigy ir:build`, and drove both previews
with a browser. One definition change, one build, both web DOMs moved with
no hand edit.

| Step | Svelte preview (:4175) | React preview (:4180) |
|---|---|---|
| after rename + rebuild | 1 TextInput; **1 carries `data-validation-level`**, 0 carry `data-validation-state`; `data-size`/`data-density`/`data-type` and the five TXT-16 style vars present | 1 TextInput; **1 carries `data-validation-level`**, 0 carry `data-validation-state`; same full attribute set |
| after restore + rebuild | 1 TextInput; **1 carries `data-validation-state`** (value `"none"`), 0 carry `data-validation-level` | 1 TextInput; **1 carries `data-validation-state`**, 0 carry `data-validation-level` |

Restored → one `ir:build` → `ir:check` 0, and the restored artifacts and
fixture contain zero occurrences of `data-validation-level` (grep-verified).
The only file touched during the proof was `text_input.rs` (renamed, then
restored). The artifact-level proof is encoded in
`one_definition_change_moves_both_web_artifacts` (renames the attribute
**and** a TXT-16 style prop; both artifacts move in one build).

**Environment note — the preview port.** The main checkout's preview owns
:4173; this card's Svelte preview bound :4175 (vite port-shifted on a
collision, the `strictPort`-less default). Verified live against :4175 —
the log records the port so the next card does not observe the main
checkout's stale DOM by mistake.

## The IME finding — the required test exposed a real gap, and the fix

The card's acceptance line — "a composition sequence must not fire
intermediate `onValueChange`" — **failed against the current components**,
and the failure is the card's most useful measurement: the measured table
says the web runtimes handle composition "not at all", and that was
literally true — `handleInput` committed on *every* native `input` event,
including the `isComposing: true` events the browser fires for each
composition buffer update. The components fired intermediate `onValueChange`
during IME composition.

The fix, in both runtimes: `compositionstart` sets a flag; `input` events
while composing record the buffer and return; `compositionend` commits the
buffer exactly once. The composition's *text editing* is never intercepted —
the browser still produces the buffer; the gate only decides when the value
is reported. Two implementation notes measured along the way:

- **An event-only `isComposing` filter breaks React.** React's `onChange`
  is value-diff-based (the input value tracker); the final committed input
  event carries the same value as the last intermediate buffer, so React
  never fires `onChange` for it — the composed text would never reach
  `onValueChange`. The `compositionend` commit point is required, not
  optional.
- **The commit is exactly-once under the spec order.** The UI Events spec
  fires the final committed `input` (isComposing false) *before*
  `compositionend`; buffering during composition + committing on
  `compositionend` yields exactly one commit in every browser that follows
  the spec order.

The IME test in both component suites simulates the full sequence —
`compositionstart`, two intermediate buffer updates (isComposing true), the
final committed input, `compositionend` — and asserts zero intermediate
calls and exactly one final call with the composed text. This is the
acceptance line proven, in both runtimes. No prop, attribute, class, or
pixel changed; the gate is the only behavioural delta, and it is the
restoration of the semantics the card requires.

## R5 — the editing model stays hand-written, and it is not vector-pinned

- `packages/contracts/headless/src/text_input.rs`: zero diff (untouched).
- `packages/render/src/text_input.rs`: zero diff (untouched).
- No `packages/core/src/text-input.ts` was created — the absence is by
  design (card Current State; b047 baselined `rs:text_input` as *correctly
  different* in `docs/machine-shape-drift`).
- `machines.json`: zero diff — and it carries **no `text` key** (GAP-01).
  The b045/b046 slider-vector thinness finding generalizes to a stronger
  statement: *no text vector exists at all*. The shared edit model is pinned
  by its own unit tests in both runtimes, never by the vector mechanism —
  the same "agreement is unit-test-local, not vector-pinned" shape b046
  recorded for the two-thumb path. For `g13.008`: text-editing cases belong
  in the vector before it can be called a safety net for TextInput.

## The R7 hand-written exception inventory (per runtime, with the comparison)

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* This is the card's headline output; the
three-component comparison is at the end.

### Svelte (`TextInput.svelte`)

**From the definition (via the artifact):** the four `data-*` attribute
names; the five TXT-16 style-prop names; the root, field, affix, affordance,
indicator, and char-count part classes (including the `--prefix`/`--suffix`/
`--leading`/`--trailing` modifiers); the attribute value domains (carried in
the artifact).

**Hand-written:**

| Exception | Reason |
|---|---|
| the `<input>`/`<textarea>` elements, their native attributes (id, name, type, inputmode, list, required, pattern, spellcheck, autocapitalize, autocorrect, enterkeyhint, maxlength, autofocus, disabled, readonly), and the value binding | DOM element and native-attribute projection are adapter-owned (`NEG-02`, `IR-05`); the definition declares the mappings (`accessibility.native`) |
| the `input-control` and `clear-button` **base classes as Svelte literals** | the focus-coverage gate (`focus-ring-drift.ts`, g13.037/038) resolves focusable-element classes by literal source scan, and text-input.css draws a stacked `:focus-within` wrapper ring (T §4) whose focusables must show outline coverage; the base classes stay literal in Svelte only — the definition and the artifact carry them, React reads them from the artifact, and every non-focusable class and all modifiers are artifact-sourced. The gate is not a writable path |
| the value-dependent modifier classes (`--multiline` on root/control, `--over` on char-count, `--pending/--valid/--invalid` on the indicator) | value-dependent classes have no sanctioned slot in the IR (no state-derived class mechanism); the *base* names come from the definition (CROSS-13 pattern) |
| the validation orchestration (triggerValidation/runValidation, context serialization and merging, `validationStatusToState` mapping, slug built-in rules, empty-value idle skip) | validation timing and async orchestration are component-owned (TXT-12/28); the definition declares the intent (events, indicator gating, aria-invalid/busy) and the VisualState shape |
| the debounce machinery (`emitValueChange`, `flushDebouncedValue`, timer lifecycle) | timing is adapter-owned (`Timers` capability, TXT-11/28); the event timing is declared (debounce, flush-on-blur, immediate phases) |
| the slug machinery (`slugify`/`isValidSlugFormat` calls, source auto-generation, user-edited tracking) | machine-owned (`slugify` is a core helper; TXT-09); declared in the vector-free machine gap note |
| the controlled/uncontrolled merge (`isControlled`, `commitValue`) | the DoNotMix pair is declared in `controlled_state`; the merge is runtime state logic (CROSS-04) |
| the IME composition gate (composing flag, buffer, commit on `compositionend`) | the card's required no-intermediate-fire semantics; composition itself stays browser-native (TXT-24's web half — see the IME finding) |
| the `handleInput` value read + `normalizeInputValue` (slug) call | input-event delivery is framework lifecycle (IR-05); the value path is the runtime's projection |
| the TXT-16 padding `calc()` strings (`controlPaddingStart/End`, the insets) | arithmetic/string building is excluded from the expression vocabulary by design (spec 063); the names and their source fields come from the definition's `styleProps` |
| the `$derived` resolutions (`effectiveValidationState`, `resolvedSize`, `resolvedDensity`, `charCountText`, `isOverLimit`, the aria derivations) | the VisualState projection's runtime computation (CROSS-14, TXT-19) |
| the char-count text (`{n}/{max}`) and validation-message paragraph | string interpolation is excluded (spec 063); formatting is a projection concern |
| submit/cancel key handling, focus passthroughs, the clear handler | events are declared intent (`CROSS-05`); delivery is framework lifecycle (`IR-05`) |

### React (`TextInput.tsx`)

Mirrors Svelte exactly, plus:

| Exception | Reason |
|---|---|
| `useState`/`useRef` for the uncontrolled value, the validation refs, and the composition refs | React's lifecycle idiom (`IR-05`); the composition refs mirror the Svelte gate |
| `useImperativeHandle` for `focus()` | the imperative handle precedent (MenuSurface); the Svelte export is the mirror |
| `rootStyle` emits three of the five TXT-16 style props | the pre-existing asymmetry: the clear/trailing overlay insets are Svelte-only emissions; the stylesheet fallbacks (`0.5rem`) cover React (TXT-16, recorded, unchanged) |
| the `autocorrect` omission | OBS-03 — React's surface has no autocorrect prop; recorded as an extension |

### Both runtimes, shared

| Exception | Reason |
|---|---|
| the recipe hooks are carried in the artifact but consumed by `text-input.css`, not by component markup | the CSS is the styling seam (R4 — untouched); the definition is now the single record of the hooks and their chains |
| `data-type`'s value derivation (`type` prop passthrough) | the value is the prop itself; the name and domain come from the definition |
| the value-dependent attribute *values* (`effectiveValidationState`, `resolvedSize`, `resolvedDensity`) | the emission-value logic (CROSS-13) in the runtime; the names and domains come from the definition. The web targets do not yet evaluate IR expressions — a g13.008 question |
| the focus/IME/selection/clipboard/measurement implementation | adapter-owned by design (R2) — the boundary is the deliverable |

### The three-component comparison (the trend `g13.008` needs, in one place)

| | Button (b041/b042) | RangeSlider (b045/b046) | **TextInput (this card)** |
|---|---|---|---|
| props → documented data attributes | 34 → 11 (~3:1) | 18 → 8 (~2:1, +7 geometry hooks) | **49 → 3 (~16:1), 4 emitted** |
| hand-written remainder | DOM element + event wiring + every derived value + per-attribute derivation | the machine (gestures, ordering, geometry numbers) + size resolution + token resolution + specimen chrome | **the validation/debounce orchestration, the slug machinery, the IME gate, the controlled merge, the TXT-16 arithmetic, all six environment capabilities implemented by the adapters/browser** |
| machines | toggle machine (small) | slider machine (shared vector, thin) | **the edit model — hand-written, and NOT vector-pinned (no `text` key)** |
| capabilities | focus/pointer-capture | pointer-capture/focus/scrub-fraction | **six typed capabilities + timers; ownership prose-only; selection untyped by name** |

The trend is the milestone's answer: **the environment-boundary slice did not
shrink the hand-written remainder — it added the largest one yet.** Button's
remainder was adapter plumbing; RangeSlider's added the machine and the
geometry projection; TextInput's remainder is the component's *entire
behavioural core* (validation, debounce, slug, IME gating, controlled state)
on top of a browser-owned editing model the definition can declare but never
reach into. The definition reaches the vocabulary (49 props, 4 attributes,
10 parts, 6 capabilities, 17 commands, 12 projection fields) and stops at
the boundary — which is exactly the result `g13.007` exists to produce.

## Vocabulary notes recorded for g13.008

- **The controlled pair is DoNotMix, unlike RangeSlider** — the contract's
  rule is the IR's only rule (T §3 vs the b045 controlled-wins note).
- **`onValidationChange` has no `EventKind`.** `PayloadKind::ValidationStatus`
  exists (`{status, valid, message}`) but no `EventKind` uses it, so the
  callback cannot be typed as an event. `onKeyDown` is a native passthrough
  with no kind either. Both are recorded; the definition types the other
  six callbacks (`onFocus`/`onBlur` share `FocusChange`).
- **Selection is not a first-class capability name** — typed via
  `TextEditing` + `Measurement`; the ownership split is prose (R2 negative
  answer 1).
- **Per-runtime capability ownership has no typed field** — prose
  `purpose` only (R2 negative answer 2; a typed ownership map would be
  `poodle-ir` schema work, out of scope).
- **The text machine is not vector-pinned** — `machines.json` has no
  `text` key; unit tests only (R5, GAP-01).
- **The web runtimes fire intermediate `onValueChange` during composition
  without the gate** — measured; the gate is this card's one behavioural
  addition (see the IME finding).
- **`color.text.muted` does not resolve** — text-input.css references
  `--poodle-color-text-muted`, which is absent from the semantic token
  registry; the definition records `color.text.tertiary` as the intended
  family with the gap noted (a pre-existing token gap, unchanged).
- **React emits three of the five TXT-16 style props** — the Svelte/React
  DOM asymmetry is pre-existing; the definition declares all five.
- **The web emits four data-* attributes; the contract documents three** —
  `data-type` is corpus-documented (TXT-18); the card's count stands.

## Tests (86 in `poodle-codegen` + 738 web component tests, all passing)

`tests/text_input.rs` (9 new):

- `text_input_model_validates_and_round_trips_as_json` — in-memory
  validate clean; serialization round-trips; the committed fixture equals
  the authored model.
- `text_input_definition_authors_the_full_contract_surface` — 44 props
  (41 web non-callback + 3 Rust-only), 7 typed events, and the accounting
  `41 + 7 + 1 (onKeyDown) = 49` (the card's web-prop surface); the 13
  web-only props enumerated; the 4 emitted data-* names + 5 style props;
  10 parts with **no `Repeated`**; 6 recipe hooks; 9 shared types; the
  six-capability boundary in order (Focus, TextEditing, Ime, Clipboard,
  Measurement, Timers); key defaults (`value` null, `defaultValue` "",
  `type` text, `validationState` none, `validationDebounce` 300,
  `validateOnBlur` true, `showClearButton` true, `resize` vertical); the
  DoNotMix pair; the 5-rung size ladder; the density adjustments (2, on
  the root, inline + block); no orientation axis; 2 extensions.
- `text_input_vector_declares_the_hand_written_machine_semantics` — the
  `text-input` conformance resolves; `applies_to` is Gpui + Jetstream and
  **not** Svelte/React (the no-TS-machine fact encoded); `machines.json`
  carries no `text` key (the R5 gap asserted, not just logged).
- `both_web_components_carry_the_same_text_input_derived_artifact` — the
  card's required parity test: the expectation is the target's render of
  the authored definition (derived, never hand-listed), and **both**
  committed web artifacts must equal it byte-exact.
- `artifact_renders_parts_attributes_style_props_and_recipe_hooks` — every
  part id and class (including the base+modifier pairs), every attribute
  name with its form and value domain (`none/invalid/valid/pending`,
  `text/multiline/search/slug`, the size/density domains), the styleProps
  with their source fields, and every recipe hook with its chain kinds
  (including the focus-fill component-variable chain).
- `artifact_header_names_the_source_definition_and_generator_version` — the
  Generated Artifact Contract.
- `one_definition_change_moves_both_web_artifacts` — the R2 proof encoded:
  renaming `data-validation-state` → `data-validation-level` **and**
  `--poodle-text-input-control-padding-start` →
  `--poodle-text-input-padding-start-v2` moves **both** committed artifacts
  in one build.
- `text_input_artifacts_fail_check_on_drift_and_check_never_writes` — the
  CLI `--target text-input-ts --check` fails on planted drift + stale
  orphan and leaves the tree byte-identical; `--author-text-input` gates
  the fixture the same way.
- `web_component_manifests_carry_no_poodle_ir_or_codegen_dependency` —
  reads both component `package.json` files and asserts neither crate is
  named (R1 asserted, not just avoided — the card's required test).

Plus the target's unit test (the ten-part class projection, 1). The
definition→DOM component tests per runtime (Svelte 6, React 6): the four
attributes under the artifact's names, the anatomy under the artifact's
classes, the conditional parts, the TXT-16 style props (5 Svelte / 3
React), the IME no-intermediate-fire acceptance, and the selection-is-DOM's
re-render proof. The card's required drift proof ran live: planted one line
into `packages/svelte/components/src/generated/text-input/index.ts` →
`effigy ir:check` exits 1 naming the artifact → regenerated → exits 0.

## Validation (all step-10 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored shell + button + range-slider + text-input models, all targets (text-input-ts ×2) |
| `effigy ir:check` | 0 — all current (text-input fixture + both text-input artifacts gated) |
| `effigy ci:rust` | 0 |
| `effigy test:core` | 0 |
| `effigy test:components` | 0 — 74 files, 738 tests (new TextInput generated tests included) |
| `effigy test:parity` | 0 — TextInput class-set diff green |
| `effigy check:svelte` | 0 — install-smoke + 710 component files, 0 errors |
| `effigy docs:lint` | 0 — including the focus-coverage gate (see the R7 literal-class entry) |
| `effigy docs:contract-drift` | 0 — the 49-web-prop surface is unchanged (R4) |
| `effigy docs:callback-drift` | 0 |
| `effigy docs:machine-shape-drift` | 0 — `rs:text_input` still baselined as correctly different |
| `effigy drift:recipes` | 0 |
| `effigy svelte:surface-audit` | 0 |
| `effigy ci:web` | 0 — includes `test:web-pack-install` (the b041 papercut does not reproduce) |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 86 passed (9 new in `tests/text_input.rs`) |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |
| `npx tsc --noEmit -p packages/react/preview/tsconfig.json` | 0 — TextInput.tsx type-checks |

**R4 proof — the surface is byte-identical.** The components' prop
interfaces were not touched (49 entries, same names/types/defaults);
`docs:contract-drift` and `svelte:surface-audit` pass; the parity test's
class-set diff passes; every existing TextInput test (autofocus/focus
parity) passes unchanged; the browser samples before and after the proof
show the same attribute set and values (`data-validation-state`,
`data-size`, `data-density`, `data-type` on the same specimen, and the same
TXT-16 style vars).

## Acceptance criteria

- [x] R2's capability boundary is typed — all six capabilities have a typed
      `Capability` name; the two gaps (selection has no name of its own;
      per-runtime ownership is prose, not a field) are recorded per
      capability, not routed around.
- [x] R3's three questions each answered explicitly in the log — Q1 one
      declaration serves both (the boundary, not the implementation); Q2
      per-capability owners with the typed/prose split named; Q3 the honest
      number: every prop and attribute declared, zero props executed by the
      definition, the largest hand-written remainder of the three slices.
- [x] The Rust editing model is untouched; no TS machine was created
      (R5 — zero diff on both Rust files, no `core/src/text-input.ts`).
- [x] 49 props, 3 contract-documented attributes (+ the 4th emitted),
      no pixel moved (surface gates + parity + unedited existing tests +
      untouched CSS).
- [x] A definition change moves the DOM in both web previews — shown live
      (`data-validation-state` → `data-validation-level`, both previews)
      and encoded as a test (attribute + style-prop renames move both
      artifacts).
- [x] IME composition proven intact in both runtimes — the required test
      exposed the missing gate; the fix (buffer + commit-on-compositionend)
      is tested in both runtimes with a full composition sequence.
- [x] The exception inventory exists per runtime, with the Button /
      RangeSlider / TextInput comparison (the R7 trend table).
- [x] All step-10 commands exit 0; no baseline refreshed.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
`poodle-render`, adapter, or native preview work (049), no `poodle-ir`
schema change (the two R2 negative answers are recorded findings — a typed
capability-ownership field would be a schema gap, which the card stops at
and records), no machine or vector edit (R5), no other component, no visual
baseline refresh (R4), no hand edit of generated files. The contract
(`docs/contracts/components/text-input.md`) was **not** edited: the
definition's records beyond the contract's documented surface — the fourth
emitted attribute `data-type` (corpus row TXT-18), the five TXT-16 style
props (the contract names the two padding reservations; the three
additional insets are implementation records), and the IME composition gate
— are web-side rendering records, and the contract is the milestone's
counting authority (3 documented attributes); editing it would move the
card's measured baseline. The card's "say so" is this log entry — the b045
precedent. `docs/roadmaps/g13/007-*.md` status untouched (the card does not
close `g13.007`). No PAPERCUTS entry was needed (no environment repair this
card had to make; the port-collision note is recorded in the R2 proof
section instead).
