---
title: g13 batch 049 — TextInput environment boundary, the two native runtimes
status: complete
milestone: g13.007 (part 2 of 2 — **closes the milestone**)
owner: Poodle core
updated: 2026-08-13
tags: [log, g13, IR, text-input, component, gpui, jetstream, render, spec-063, g13.007, environment-boundary]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/049-text-input-slice-native-runtimes.md` on
branch `thread/g13-049-text-input-slice-native-runtimes`: added the
`text-input-rust` emitter target (R1 — a sibling of `text-input-ts`, which
is byte-frozen by b048's tests), emitted a self-contained Rust artifact
into `packages/render/src/generated/text-input/` (R1/R1a — plain data, no
`use` of any Poodle crate, pulled in via `#[path]`, in the package that
ships it), and rewired `poodle-render::text_input` to take its vocabulary
— the ten-part anatomy, the validation-state treatment, the size/density
ladders, and the typed capability boundary — from the generated definition
instead of its own literals (R3). The card's second job is the headline:
**the three-way split is real and it is the milestone's answer.** The
four-runtime proof ran live: a `data-validation-state` rename moved both
web previews' DOM (Svelte browser-verified; React jsdom-verified) and
dropped the validation treatment in both natives (GPUI and Jetstream
pixel-verified), and restored clean. R2's three questions are answered
per capability against GPUI's real behaviour, and R3's answer is the
single most useful sentence this card can produce for `g13.008`:
**the IR cannot express that a runtime lacks a declared capability.** The
R7 exception inventory covers both natives, the three-slice comparison
table is written, and `g13.007` closed.

Per the card's worker rules: no sub-agents; sources read directly (b048's
log first — its capability table is the thing this card tests — plus
b042/b046 for the generated-Rust route, the `text-input-ts`/`button-rust`/
`range-slider-rust` targets, the authored `text_input.rs` model, both
native specimens, the GPUI node-backend `interaction.rs`/`input_text.rs`/
`ime.rs`, and the Jetstream repo's poodle integration); no planning/status
authority exercised beyond the card's own writable status line. No stop
condition was reached. `machines.json` passed unedited (R5 — and it still
carries no `text` key, re-recorded below), no `poodle-ir`/
`poodle-codegen` dependency entered `packages/render/Cargo.toml` (asserted
by test), no visual baseline was refreshed (R4), and no native preview
source was touched.

## Measured before-state — the card's Current State, verified (steps 1–2)

- **Svelte/React delegate entirely.** Both components read the four
  `data-*` attribute names from the `text-input-ts` artifact
  (`attributeName("validation-state")` in `TextInput.svelte`/`TextInput.tsx`
  resolve `attributes["validation-state"]`); zero selection code in either
  component (b048 measured; unchanged).
- **GPUI implements it.** `packages/gpui/node-backend/src/ime.rs` (218
  lines) is a direct gpui `InputHandler` implementation with explicit
  UTF-16 conversions; `input_text.rs` (574 lines) paints the caret and
  selection at measured glyph positions, owns undo history and the marked
  range; `interaction.rs` consumes `on_edit_key`/`on_edit_insert`/
  `on_select_range`/`on_focus_change` (lines 20–64, 121, 196–213);
  `lib.rs` feeds `on_edit_insert`/`on_select_range` into the platform
  input handler (lines 145–146). All present.
- **Jetstream does not have it at all.** Measured across
  `/Users/tom/Dev/projects/jetstream`: **zero** occurrences of
  `on_edit_key`, `on_select_range`, or `on_edit_insert` in any `.rs` file;
  no `ime.rs` anywhere; `jetstream-poodle` maps the TextInput node role to
  `accesskit::Role::TextInput` and renders a static
  `ui_element::text_input(value, placeholder)` element (lib.rs:196–198) —
  display-only, no caret, no selection, no key path. The specimen is
  display-only (`with_value("Hello world")`).
- **Sizes.** `render/src/text_input.rs` 625 lines; the new artifact 317
  lines; corpus `TXT-NN` rows 32, of which 13 (TXT-20–TXT-32) name
  Rust/backend/native ownership — verified against
  `docs/roadmaps/g13/pilot-expressiveness-corpus.md` (the ids are the
  corpus's; `docs/contracts/components/text-input.md` carries none).
- **The two precision notes hold.** `shape_line`/`x_for_index`/
  `closest_index_for_x` do not exist as poodle functions — they name a
  backend glyph-measurement obligation (TXT-21; gpui's `ShapedLine`
  provides the actual methods). Every symbol the corpus names for
  TXT-21–24 exists: `TextInputHandlers`, `selection_range`, `on_edit_key`,
  `on_select_range`, `on_focus_change`, `on_edit_insert`,
  `SelectGranularity`.

## The four-runtime proof (step 7, live)

Renamed `data-validation-state` → `data-validation-level` in
`packages/codegen/src/models/text_input.rs`, one `ir:build`, all three
artifacts moved (each carrying exactly one occurrence of the new name,
zero of the old — grep-verified). Restored → one `ir:build` → all three
artifacts back (zero `data-validation-level`), `ir:check` 0, tree clean.
The only file touched during the proof was `text_input.rs` (the model),
renamed then restored.

| Runtime | Renamed observation | Restored observation |
|---|---|---|
| Svelte preview (:4174) | **8/8 TextInputs emit `data-validation-level`**, 0 emit `data-validation-state` (the five `data-validation-state` elements on the page are `.poodle-field` roots — a different component, not TextInput); `data-size`/`data-density`/`data-type` unchanged | 8/8 TextInputs emit `data-validation-state`, 0 `data-validation-level` |
| React | jsdom component test fails exactly on the renamed attribute — `attributeName("validation-state")` resolves to `data-validation-level`, and the test asserting `"data-validation-state"` fails while the DOM carries the new name (5/6 pass; the other five tests are value/anatomy, unaffected) | the same test suite is 6/6 green |
| GPUI | 6722 changed samples vs restored, bbox (542, 1211)–(1013, 2104) at 2696×2396 — exactly three fields (the With-validation/Async-validation/Valid rows, diff runs y1211–1266, 1477–1532, 2049–2104); the restored capture's status borders (danger/success/accent families) and the shared error-message row (identical in both — the diff is the TextInput treatment, not the Field chrome) | status borders + indicators present; identical to the pre-proof render (R4) |
| Jetstream | 32148 changed samples vs restored, bbox (24, 210)–(323, 409) at 900×640 — exactly three rows (diff runs y210–245, 292–327, 374–409 = the Invalid/Valid/Pending fields, all inside the 640px frame — the b042 frame warning checked); the renamed capture's borders are the default border family, the restored capture's are the status families (danger `(224,106,95)`-ish, success `(127,210,77)`-ish, accent `(240,178,77)`-ish at the border edge) | status borders + indicator pixels present |

**Environment notes (both pre-recorded hazards hit, plus one new):**

- **The :4173 squatter (b048's port trap, re-hit).** The main checkout's
  preview owns :4173 and serves the main repo's **stale** artifacts. My
  Svelte preview port-shifted to :4174; the first DOM check ran against
  the squatter and showed the old attribute name — exactly the trap b048
  recorded. Verified against the correct port before recording anything.
- **The Jetstream snap overwrite (b042's papercut).** Each snap output was
  copied to `/tmp/g13-049-jetstream-{renamed,restored}.png` immediately
  after rendering, before the next render.
- **The React preview is dead on this branch — pre-existing, not this
  card.** `packages/react/preview/src/gallery/specimen-map.ts` throws at
  module load ("Missing React specimens: update-center, update-status")
  because the Svelte canonical registry lists those two as standalone
  specimens (added by the UpdateCenter PR `b433498d`, an ancestor of
  HEAD) while the React gallery has no specimens for them. The React
  preview therefore cannot mount at all on this branch (root div empty,
  `pageerror` captured). It predates this card (both files untouched by
  it) and is out of its writable scope; recorded in `PAPERCUTS.md`. The
  React half of the four-runtime proof is served by the jsdom component
  test (the DOM the component actually renders) plus the artifact-parity
  test that byte-moves both web artifacts in one build.
- **The GPUI click/key driver does not land in this session.** Six
  attempts (`--click`, `--click`+`--type`, `--click X,Y,2` double-click,
  with/without `osascript` activation, `--hold 1500`) produced no focus,
  caret, selection, or typed text in the capture — the only per-capture
  delta is the pending spinner's rotation. The calibration probes are
  observed (no "calibration probes were not observed" error), so events
  are posted and dispatched, but the effects never reach the capture;
  main.rs's own comment says a `--screenshot` with the display link
  stopped shows the last drawn frame, and a visible window captures
  correctly — this agent session's window is not interactively live. This
  blocks the *interactive* caret/selection captures below; it is
  session-specific (the driver is designed for visible windows), so it is
  recorded here rather than in `PAPERCUTS.md`.

## R2 — the three questions, answered per capability against GPUI's real behaviour

The card's per-capability questions: (1) does the GPUI owner named in
b048's table match what GPUI actually does; (2) what does the declaration
mean for Jetstream, which does not implement the capability at all; (3)
can one declaration serve delegate / implement / absent.

| Capability | b048's Rust owner | GPUI actually does (measured) | Match? | What the declaration means for Jetstream |
|---|---|---|---|---|
| focus | backend (`on_focus_change`, caret drawing); `isFocused` host-driven, does not drive the caret (TXT-21) | `interaction.rs:20–64` tracks real gpui focus via an owned focus handle, fires `on_focus_change` on both gain and loss, records `FOCUSED_FIELD`; the caret paints for the focused field only (`input_text.rs`) | **yes** | nothing — Jetstream never observes focus for poodle fields (no `on_focus_change` route); the field is not focusable in the adapter's accesskit tree in any meaningful poodle sense |
| selection | host owns caret position; backend owns drawing (glyph measurement); shared model owns semantics (TXT-21/22) | `input_text.rs` paints caret + selection wash at measured positions; click→character resolution via the measured `ShapedLine` (`char_index_for_position`); the render's `on_select_range` resolves word/line through `poodle_headless::text_input::word_range_at`; drag anchors scoped per field (unit-tested) | **yes** | nothing — zero selection code in the Jetstream repo; the adapter's `text_input` element has no caret channel, so even a spec with a selection renders none (TXT-31's "no route" recorded in the corpus) |
| composition/IME | platform text input handler (`Window::handle_input` with `InputHandler`), UTF-16 boundary, backend-owned marked range (TXT-24) | `ime.rs` is a direct `InputHandler` implementation (no entity binding), all three encodings converted explicitly (chars ↔ UTF-16 ↔ bytes), `MARKED` range per field in `input_text.rs`, candidate-window bounds via `bounds_for_chars` | **yes** | nothing — no `ime.rs` anywhere in the Jetstream repo; no input handler is ever registered for a poodle field |
| clipboard | backend owns platform clipboard; shared model owns paste landing and copy/cut source (TXT-23) | `lib.rs:145–146` feeds `on_edit_insert` + `on_select_range` into the platform `NodeInputHandler`; paste/cut land through `poodle_headless::text_input::insert_transition` / `selected_text` (render-side wiring) | **yes** | nothing — no clipboard path exists for poodle fields |
| measurement | backend glyph measurement (`shape_line`/`x_for_index`, `closest_index_for_x`) (TXT-21/22) | `input_text.rs` shapes the value line, measures it, and answers clicks with `x_for_index`/`closest_index_for_x` (gpui `ShapedLine` methods — the corpus vocabulary names the obligation, not poodle functions, per the card's precision note) | **yes** | nothing — no shaped-line measurement, no caret placement, no click-to-index |
| native text editing | shared headless model driven from key events and insertions (TXT-20/23/25) | `interaction.rs:196–213` wires `on_edit_key`/`on_edit_insert` from the node tree into key/IME dispatch; the render drives `poodle_headless::text_input::edit_transition`/`insert_transition` (the same machine the web targets share) | **yes** | nothing — zero occurrences of the three edit channels in the Jetstream repo; the shared edit model exists in `poodle-headless` but Jetstream never drives it for poodle components |
| timers | web-component-owned (debounce/validation timing); Rust targets have no timer surface | n/a — neither native has a timer surface | n/a | the declaration matches (natives have no timers) |

**Answer 1: every GPUI owner matches.** b048's capability table is
accurate against the implementation — all six Rust-owned halves exist in
the GPUI node-backend exactly where the table says, and the shared edit
model drives the semantics. No mismatch found.

**Answer 2: the declaration asserts existence, and Jetstream's absence is
invisible to it.** For every capability the declaration says "this
component requires X, and (in prose) the Rust runtime owns half of it".
Jetstream is a Rust runtime named nowhere — the `text-input` conformance
vector even lists it in `applies_to` (with Gpui), claiming it honors the
shared edit-model step intents, which nothing in the Jetstream repo
exercises. The declaration, the artifact, and the vector all assert the
capability; none can assert "Jetstream does not implement this".

**Answer 3: one declaration serves delegate + implement, but not
absent.** b048 answered the two-runtime question: one declaration serves
both web (delegate) and GPUI (implement) because the declaration names
the boundary, not the implementation, and the asymmetry lives in prose
ownership. This card adds the third case: **the moment a runtime does not
implement the capability at all, prose ownership stops being merely
untidy and becomes load-bearing.** With delegate/implement, the prose
split was a description; with absent, there is no prose slot for "this
runtime has none of it" — the declaration silently over-claims. The
card's artifact makes the mechanism concrete: the render wires the edit
handlers whenever the definition names `text-editing`/`measurement`/
`focus`, component-wide, for every runtime that consumes the node; there
is no per-runtime expression anywhere in the pipeline. b048's negative
answer 2 (per-runtime ownership has no typed field) is therefore not a
cosmetic gap — it is the load-bearing hole `g13.008` must decide about.

## R3 — Jetstream's gap: visible and typed, or silent? **Silent.**

**The honest answer is the sentence the card hoped for: the IR cannot
express that a runtime lacks a declared capability.**

- The capability boundary is typed as an *existence* claim
  (`CapabilityRequirement { capability, purpose }`); the ownership split
  is prose inside `purpose`; there is no per-runtime presence/absence
  mechanism.
- The `text-input` conformance vector declares `applies_to: [Gpui,
  Jetstream]` — the model claims Jetstream honors the shared edit-model
  step intents (insert-at-caret, selection-replaces, deletion-semantics,
  …), and nothing verifies that a listed runtime actually runs the steps.
  The vector's `applies_to` is a claim, not a measurement: Jetstream is
  listed and exercises none of them.
- The artifact this card adds carries the capability list for the
  component — the same six names every runtime sees — and the render
  honors it component-wide: `a_capability_gap_is_visible_in_the_artifact_and_moves_the_render`
  proves the only available expression of a gap is a *definition-wide*
  drop that would strip the editing wiring from **both** natives (GPUI
  included). There is no way to type "Jetstream only".
- The Jetstream specimen renders a text field that cannot be typed into,
  cannot be focused for editing, cannot show a caret or a selection — and
  the model says the capability exists. Nothing in the IR, the artifact,
  or the vector says otherwise. That is the silent gap the milestone's
  acceptance line ("capability gaps are visible and typed; none are
  silently ignored") was written to catch, and this is the first pilot
  component that can demonstrate it.

For `g13.008`: typing a per-runtime gap needs either a per-runtime
capability list on the requirement (an `applies_to` on
`CapabilityRequirement`), a presence/absence flag per runtime, or a
re-think of the vector's `applies_to` semantics (a listed runtime should
mean "conformance is verified", not "the runtime exists"). Each is a
`poodle-ir` schema question — out of this card's scope by rule (Out of
scope, Stop Condition 1 examined and not reached: the card's work did not
need the field).

## R5 — the editing model and the vectors stay fixed

- `packages/contracts/headless/src/text_input.rs`: zero diff (untouched).
- `packages/render/src/text_input.rs` edits are vocabulary-sourcing only;
  every gate holds today, so the render is byte-behaviour-identical (R4 —
  the 182 pre-existing render tests pass unedited).
- `machines.json`: zero diff — and it still carries **no `text` key**
  (GAP-01, re-recorded as the card requires: the text machine is
  unit-test-pinned only, and closing that vector gap is a follow-up card,
  not this one). The `slider` vector passes unedited; the `text-input`
  vector resolves with `applies_to: [Gpui, Jetstream]` — see the R3 note:
  that listing is precisely the over-claim the card measures.

## R6 — both natives, the states only natives have (screenshots)

Captures: GPUI `/tmp/g13-049-gpui-restored.png` (2696×2396) and the
renamed/double-click/typed variants; Jetstream
`/tmp/g13-049-jetstream-restored.png` (900×640) and the renamed twin.
No vision model was available, so content was verified from pixel
evidence (the same approach b042/b046 recorded).

- **Validation state — captured in both.** The restored GPUI capture
  shows the three validation fields (With validation/Async validation/
  Valid) with status borders (danger/success/accent families at the
  border edge) and the shared danger error message; the Jetstream
  restored capture shows the Invalid/Valid/Pending rows (y210–245,
  292–327, 374–409 — inside the 640px frame, the b042 warning checked)
  with status borders. The renamed captures show the default border
  family on those same rows and nothing else differing — the
  before/after pair is the proof.
- **Focused with a caret — GPUI capture blocked; render + backend
  evidence substituted.** The interactive driver does not land in this
  session (recorded above). The caret path is otherwise proven: the
  render's caret channel carries the selection and colours
  (`the_value_is_one_text_node_carrying_the_caret_and_its_colors`), the
  backend paints it at measured positions and resolves clicks
  (`char_and_byte_offsets_round_trip_through_multibyte_text`,
  `a_caret_after_an_emoji_lands_on_a_boundary`,
  `drag_anchors_are_scoped_to_the_field_that_started_the_drag`,
  `ime.rs`'s UTF-16 tests), and the edit-handler wiring is unchanged (R4).
- **Selection range — same blocking, same substituted evidence.** The
  double-click word-select path (`--click X,Y,2`) is driver-blocked; the
  render-level word/line resolution is tested
  (`pointer_selection_is_reported_from_the_value_node_only`), and the
  backend's drag-anchor scoping is unit-tested.
- **Jetstream: the absence is the finding.** A caret or selection state
  cannot be captured because the adapter's `text_input` element has no
  caret channel at all — the snap literally cannot show one. That is the
  R3 evidence made visual: the Jetstream field looks editable and is not,
  and nothing in the model says so.

## The R7 hand-written exception inventory (GPUI and Jetstream, with the three-slice comparison)

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* b048 covered Svelte and React; this is the
GPUI and Jetstream extension. Both natives consume the same
`poodle_render::text_input` node, so the render-side story is one row;
the host-side rows differ per preview.

### From the definition (via the artifact → `poodle-render`)

The ten-part anatomy (part ids gate the affix/affordance/char-count/
indicator nodes); the `data-validation-state` attribute name (gates the
validation border and indicator); the `validation-state` member domain
(gates each border arm); the `control-size`/`control-density` member
lists (gate the metric rungs); the declared capability boundary
(`text-editing` gates `on_edit_key`/`on_edit_insert` and the root
selection channel, `measurement` gates the caret and pointer selection,
`focus` gates `on_focus_change`); the five TXT-16 padding hooks and the
six recipe-hook chains carried (the web styling seam's vocabulary).

### GPUI (`packages/gpui/node-backend/src/{interaction,input_text,ime}.rs`, `lib.rs` — host side)

| Exception | Reason |
|---|---|
| the caret/selection/IME painting and hit-testing (`ShapedLine` measurement, `x_for_index`/`closest_index_for_x`, blink, scroll-into-view, marked range, undo history) | the measurement and IME halves of the boundary are adapter-owned by design (TXT-21/22/24/25, IR-05); the definition declares the capabilities and the caret channel |
| the platform `InputHandler` registration and UTF-16 conversion | the IME capability's implementation (TXT-24; `ime.rs`) — the vocabulary speaks chars, the text system bytes, macOS UTF-16; adapter-owned |
| focus observation (owned focus handle, `FOCUSED_FIELD`, gain/loss reporting) | the focus capability's implementation (TXT-21); the backend is the only layer that can see blur |
| the specimen layout and `live_text_input` value/caret host state | specimen chrome + host state, the same category as b046's (spec 063; the host owns the caret position per TXT-21) |
| `poodle_gpui_node_backend::to_gpui` node interpretation | the adapter's drawing (IR-06) |
| the platform clipboard integration | the clipboard capability's implementation (TXT-23) — paste/cut flow through `on_edit_insert`/`selected_text` |

### Jetstream (`packages/jetstream/preview/src/specimens/text_input.rs`, `jetstream-poodle`, `compat::js_text_input`)

| Exception | Reason |
|---|---|
| the entire text-editing surface — editing, IME, selection, clipboard, caret, focus observation | **absent by R3's design**: the card forbids closing the gap (R3); the absence is the finding. The host owns the editor per TXT-31's corpus record and feeds value back through the spec |
| the `nel`/`compat` El-building and the `snap` viewport/projection | specimen chrome and tooling (same category as b046) |
| the display-only `ui_element::text_input(value, placeholder)` lowering | the adapter's element mapping — the only poodle text-input surface Jetstream has |

### Both natives, shared

| Exception | Reason |
|---|---|
| the body-size/density values behind the gated rungs (0.75–1.0rem, ±0.125rem) | the metrics are the §8 table's numbers; the definition's ladder declares the rungs and the render resolves the tokens — the b046 shape (arithmetic/values stay runtime-side, spec 063) |
| the token resolution (`color.status.danger`, `color.accent.base`, …) | the definition records the tone→token intent in its recipe hooks; the render resolves the families directly — the standing g13.008 question from b042/b046, still open |
| the boolean/value domains of the attributes carried, not consumed | the native node tree has no value channel (no DOM): the domains gate the web DOM's values; the names gate the native treatments |
| the enum→name projection (`ValidationState::Invalid` → `"invalid"`) | the frozen poodle-specs enums (R4) projected onto the definition's words (CROSS-14) |
| the capability list carried but honoured component-wide | the R3 finding: the render cannot drop a capability per runtime — the list is the same for both natives |

### The three-component comparison (the trend `g13.008` needs, in one place)

| | Button (b041/b042) | RangeSlider (b045/b046) | **TextInput (b048 + this card)** |
|---|---|---|---|
| props → documented data attributes | 34 → 11 (~3:1) | 18 → 8 (~2:1, +7 geometry hooks) | **49 → 3 (~16:1), 4 emitted** |
| hand-written remainder | DOM element + event wiring + every derived value + per-attribute derivation | the machine (gestures, ordering, geometry numbers) + size resolution + token resolution + specimen chrome | **the validation/debounce orchestration, the slug machinery, the IME gate, the controlled merge, the TXT-16 arithmetic, and — new in the natives — an entire absent runtime: Jetstream has none of the six capabilities the definition declares** |
| machines | toggle machine (small) | slider machine (shared vector, thin) | **the edit model — hand-written, and NOT vector-pinned (no `text` key)** |
| capabilities | focus/pointer-capture | pointer-capture/focus/scrub-fraction | **six typed capabilities + timers; ownership prose-only; selection untyped by name; per-runtime absence inexpressible** |

**The trend, stated plainly as the card demands.** The environment-boundary
slice did not shrink the hand-written remainder — it added the largest one
yet, and the natives confirm it: GPUI's remainder is the entire text-system
implementation the boundary declares (measurement, IME, clipboard, focus,
undo), and Jetstream's remainder is the whole capability set, silently.
Button's remainder was adapter plumbing; RangeSlider's added the machine
and the geometry projection; TextInput's remainder is the component's
behavioural core plus, on the native side, the text system itself — and the
IR cannot even say that one of its own Rust targets implements none of it.
**A clear negative trend is the successful pilot outcome, and this is the
clearest data point yet: the IR carries vocabulary, not behaviour, and the
gap between the two grows with each slice.**

## Tests (95 in `poodle-codegen` + 185 in `poodle-render`, all green)

`tests/text_input_rust.rs` (7 new):

- `render_artifact_matches_the_target_render` — the card's parity test: the
  expected artifact is the `text-input-rust` target's render of the authored
  definition (derived, never hand-listed), and the committed
  `packages/render/src/generated/text-input/index.rs` equals it byte-exact;
  the artifact imports no Poodle crate (R1).
- `render_artifact_carries_the_rendered_vocabulary` — types/validation-states/
  sizes/densities member lists; the capability boundary in declaration order
  (focus, text-editing, ime, clipboard, measurement, timers); every part with
  its DOM class (including the affix/affordance base+modifier pairs); every
  attribute name/form/emission/value domain; every styleProp with its source
  field; every recipe hook with its chain kinds (including the
  component-variable chain).
- `artifact_header_names_the_source_definition_and_generator_version` — the
  Generated Artifact Contract.
- `one_definition_change_moves_all_three_text_input_artifacts` — the card's
  step-7 proof encoded: renaming `data-validation-state` **and**
  `--poodle-text-input-control-padding-start` moves the render artifact and
  both web artifacts in one build.
- `a_capability_gap_is_visible_in_the_artifact_and_moves_the_render` — the
  R3 answer encoded: dropping `text-editing` from the definition shrinks the
  artifact's capability list and moves the render artifact — the only
  mechanism for a gap, and it is definition-wide, never per-runtime.
- `render_manifest_carries_no_poodle_ir_or_codegen_dependency` — reads
  `packages/render/Cargo.toml` and asserts neither crate is named (R1
  asserted, not just avoided).
- `text_input_rust_artifacts_fail_check_on_drift_and_check_never_writes` —
  planted drift + stale orphan in a render artifact fails
  `--target text-input-rust --check` and check mode leaves the tree
  byte-identical.

Render crate (`text_input.rs` tests, 3 new; all 185 pass — the 182
pre-existing tests unedited, R4):

- `the_definition_declares_every_gate_the_render_consumes` — every part,
  the validation attribute + domain, the size/density rungs, and the
  capabilities the render gates on are declared by the committed artifact
  (a definition edit that drops vocabulary fails this test, making the
  drift direction visible rather than silent).
- `the_edit_handlers_wire_while_the_definition_declares_the_capabilities` —
  the card's required "GPUI's edit handlers still work" at the render
  level: `on_edit_key`/`on_edit_insert`/`on_select_range` (value + root)/
  `on_focus_change`/both carets wire while declared, and an insertion and a
  key both run through the shared edit model with correct selection
  reports.
- `anatomy_nodes_render_while_their_parts_are_declared` — the full anatomy
  (prefix, leading affordance, value, char count, validation indicator,
  trailing affordance, suffix) renders in row order while every part is
  declared.

Plus the emitter's unit tests (the `TEXT_INPUT_DEFINITION` static name and
the capability serde names matching the IR renames, 2).

**Rust drift proof (required test, live).** Planted drift in a scratch
render artifact → `--target text-input-rust --check` exits 1 naming the
artifact → restored → exits 0. (The test suite covers the same property;
the live proof is the `ir:check` run after each rename/restore cycle.)

## Validation (all step-10 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored models, all targets incl. `text-input-rust` |
| `effigy ir:check` | 0 — all current (text-input fixture + both `text-input-ts` artifacts + the render artifact gated) |
| `effigy ci:rust` | 0 |
| `effigy ci:web` | 0 — includes `test:web-pack-install` |
| `effigy test:core` | 0 |
| `effigy test:components` | 0 — includes the TextInput generated suites (see the React note) |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 |
| `effigy docs:lint` | 0 |
| `effigy docs:machine-shape-drift` | 0 |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 95 passed (7 new in `tests/text_input_rust.rs` + the target's unit tests) |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |
| `cargo test --manifest-path packages/render/Cargo.toml` | 0 — 185 passed (3 new; 182 pre-existing unedited) |
| GPUI preview build + capture | 0 (worktree manifest) |
| Jetstream preview build + `snap specimens --slug=text-input` through the symlink path | 0 |

**R4 proof — the surface is byte-identical.** The render's public
signatures (`text_input`, `text_input_with_change`,
`text_input_with_handlers`, `TextInputHandlers`) are untouched; every gate
holds today, so the restored renders are the pre-proof code path with all
gates passing; the restored captures match the pre-proof state (the only
capture deltas in the whole batch are the intentional proof renames,
restored); `TextInputSpec` keeps its fields; the web components and their
artifacts are byte-identical to the b048 state (`ir:check` + the parity
tests prove it); `machines.json` untouched.

## Acceptance criteria

- [x] R2's three questions answered per capability against GPUI's real
      behaviour — every GPUI owner matches (no mismatch), Jetstream's
      absence is invisible to every declaration, and the one-declaration
      answer is "delegate + implement yes, absent no — prose ownership
      becomes load-bearing the moment a third runtime disagrees".
- [x] R3 answered — **the IR cannot express that a runtime lacks a
      declared capability**; Jetstream silently renders a field that
      cannot be typed into, and the model (declaration + artifact +
      vector `applies_to`) over-claims conformance.
- [x] Jetstream gained no text-editing implementation (zero Jetstream
      repo changes; the display-only specimen untouched).
- [x] `poodle-render` depends on neither `poodle-ir` nor
      `poodle-codegen` (asserted by test; artifact pulled in by
      `#[path]`).
- [x] One definition change visible in all four runtimes —
      `data-validation-state` rename: Svelte 8/8 DOM, React jsdom
      attribute-name failure/restore, GPUI 6722-sample diff, Jetstream
      32148-sample diff, all three artifacts moved, restored clean.
- [x] Both natives screenshotted including caret and selection states —
      validation captured in both; GPUI caret/selection capture blocked
      by the session's dead interactive driver (evidence substituted:
      render + backend unit tests, R4 unchanged wiring); Jetstream's
      inability to show a caret is the R3 finding itself.
- [x] The three-slice comparison table exists (R7), and the negative
      trend is stated plainly.
- [x] `g13.007` marked complete.
- [x] All step-10 commands exit 0; no baseline refreshed.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
`poodle-ir` schema change (the R3 finding is recorded — a per-runtime
capability expression would be a schema gap, which the card stops at and
records), no Jetstream text implementation (R3 — the gap is the
deliverable), no machine or vector edit (R5; `machines.json` zero diff
and still carries no `text` key), no visual baseline refresh (R4), no
native preview source touched, no web component source touched (the
React gallery defect that blocks the React preview is recorded in
`PAPERCUTS.md`, out of this card's writable scope), no hand edit of
generated files. The GPUI caret/selection interactive captures remain a
recorded environment gap (the click driver needs a visibly live window);
the render and backend unit evidence covers the behaviour this card was
asked to prove unchanged.
