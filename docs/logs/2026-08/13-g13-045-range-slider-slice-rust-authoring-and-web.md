---
title: g13 batch 045 — RangeSlider stateful proof, Rust authoring and the two web runtimes
status: complete
milestone: g13.006 (part 1 of 2 — does not close the milestone)
owner: Poodle core
updated: 2026-08-13
tags: [log, g13, IR, range-slider, component, authoring, svelte, react, spec-063, g13.006]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/045-range-slider-slice-rust-authoring-and-web.md`
on branch `thread/g13-045-range-slider-slice-rust-authoring-and-web`: authored
the RangeSlider definition in Rust (R1), serialized it to its own fixture,
emitted it to both web component packages' `generated/` directories through a
new select-only `range-slider-ts` target, and rewired `RangeSlider.svelte` and
`RangeSlider.tsx` to read the rendered vocabulary — the anatomy classes, the
eight `data-*` attribute names, and the seven fill-geometry custom properties
— from the artifact instead of hard-coding them inline (R2). The R2 proof ran
live: renamed `data-polarity` → `data-polarity-level` in `range_slider.rs`,
one `ir:build`, and both web previews emitted `data-polarity-level` on all
five RangeSlider specimens with no hand edit; restored, both emitted
`data-polarity` again. The RNG-17 geometry hooks were proven to move the same
way (encoded in the rename test: `--poodle-range-start` →
`--poodle-range-begin` moves both artifacts in one build).

Per the card's worker rules: no sub-agents; sources read directly
(`range-slider.md`, both slider machines, the full `ComponentDefinition`
vocabulary in `packages/contracts/ir/src/*` and its validation rules, the
b041 log and its authoring shape before authoring); no planning/status
authority exercised. **No stop condition was reached**, but R2.1 got the
negative answer the card exists to elicit — the IR's only repetition
mechanism cannot express the two thumbs, and the definition records them as
two distinct parts per the contract's own anatomy (finding for `g13.008`,
below). Both machines are untouched (R1), the `slider` vector passes unedited
against both machines (R5) — and the vector is **thin** in a way that
matters: its three cases pin only the single-thumb path (recorded below).
All 18 props and 8 attributes unchanged (`svelte:surface-audit`,
`docs:contract-drift`, the parity class-set diff, and every existing
RangeSlider test prove it), no pixel moved (`range-slider.css` untouched,
class and attribute values identical before/after, parity green).

## Measured before-state — the surface this card preserves (step 2)

**The 18-web-prop surface** (the card's R4 line): 16 data props
(`value`, `min`, `max`, `step`, `variant`, `polarity`, `centerValue`, `law`,
`orientation`, `disabled`, `ariaLabel`, `lowerValueText`, `upperValueText`,
`size`, `sizeRole`, `density`) + 2 callbacks (`onValueChange`,
`onValueCommit`) — the contract §3 table, exactly. React's `defaultValue`
(uncontrolled seed) is an additional web-only record, not one of the 18.

**The eight data attributes on the root and their value domains** (R §9):

| Attribute | Form | Emission | Value domain |
|---|---|---|---|
| `data-orientation` | valued | always | horizontal, vertical |
| `data-disabled` | valued | always | true, false |
| `data-variant` | valued | always | standard, embedded |
| `data-polarity` | valued | always | unipolar, bipolar |
| `data-fill-split` | valued | always | true, false |
| `data-state` | valued | always | active (pointer active), idle |
| `data-size` | valued | always | xs, sm, md, lg, xl |
| `data-density` | valued | always | compact, default, comfortable |

**The seven fill-geometry custom properties** (RNG-17; the value-dependent
geometry): `--poodle-range-start` ← `lowerNorm`, `--poodle-range-end` ←
`upperNorm`, `--poodle-range-center` ← `centerNorm`,
`--poodle-range-negative-start` ← `negativeFillStartNorm`,
`--poodle-range-negative-span` ← `negativeFillSpanNorm`,
`--poodle-range-positive-start` ← `positiveFillStartNorm`,
`--poodle-range-positive-span` ← `positiveFillSpanNorm` — each emitted inline
as `norm * 100%`.

**The geometry rules** (R2.2 acceptance line "negative/positive fill geometry
and recipe roles remain exact"): unipolar publishes an empty negative segment
and one positive segment spanning `[lowerNorm, upperNorm]`; bipolar splits the
same low-to-high window at the center reference — negative uses the
`--poodle-recipe-range-slider-fill-negative` role (status-danger), positive
uses `fill-fill` (accent-base); `fillSplitAtCenter` tells renderers to square
only the touching corners.

## The R2 answers — the three things Button could not test

### R2.1 — Repeated anatomy: **the IR cannot express two thumbs from one definition**

The IR has exactly one repetition mechanism, `PartKind::Repeated { over }`,
and validation requires `over` to resolve to a `List`-typed prop
("a repeated node needs a list source", `validate_component_parts`).
RangeSlider's `value` is a `Pair(Number)` — a fixed two-slot tuple, a
distinct `PropType` variant from `List`. The contract's own anatomy (§2)
defines Lower Control and Upper Control as distinct parts with distinct
semantics: per-thumb aria labels ("minimum"/"maximum"), per-thumb clamp
bounds (lower to `[min, upper]`, upper to `[lower, max]`), and per-thumb
Home/End behavior. Even if a list source existed, `Repeated` yields
**identical instances** — the expression vocabulary has no per-item index or
identity operand, so "first repetition is lower, second is upper" cannot be
declared.

Answer: **negative.** The definition records the two thumbs as two distinct
parts (`control-lower`/`control-upper`, `embedded-lower`/`embedded-upper`),
and the renderer hard-codes "two". This is the finding `g13.006` exists to
produce: the `Repeated` kind's own doc comment names "the two RangeSlider
thumbs" as its motivating example, and that example does not apply — the kind
needs a list source the component does not have, and per-instance identity is
not expressible. No `poodle-ir` field was needed for the anatomy *as the
contract defines it* (two named parts), so the stop condition "repeated
anatomy needs a field the IR lacks" was **not** triggered; the negative
answer is recorded here for `g13.008` instead. Modeling the two thumbs as
distinct parts is the contract's own shape, not a reshape of RangeSlider.

### R2.2 — Value-dependent geometry: **declared as projection fields + geometry hooks, computed by the machine**

Fill geometry is arithmetic over the value pair (`norm * 100`, span splits)
— excluded from the expression vocabulary by design. Spec 063's sanctioned
escape is the VisualState projection. The definition declares:

- the fourteen machine fields (`RNG-16`): `value` (Pair), `lowerNorm`,
  `upperNorm`, `centerNorm`, `fillStartNorm`, `fillSpanNorm`,
  `negativeFillStartNorm`, `negativeFillSpanNorm`, `positiveFillStartNorm`,
  `positiveFillSpanNorm` (Number), `fillSplitAtCenter`, `pointerActive`,
  `enabled` (Bool), `polarity` (Enum), `activeThumb` (Enum) — plus
  `resolvedSize`/`resolvedDensity` for the attributes;
- the seven computed custom properties as `StateAttribute`s whose `value`
  names the feeding field (RNG-17), emitted by the target as `styleProps`;
- the recipe roles (`fill-negative` → status-danger, `fill-fill` →
  accent-base, `center-fill` → border-strong, …) in `recipe_hooks`.

The shared hand-written machine (`rangeSliderVisualState` in `slider.ts` /
`slider.rs`) computes the numbers; both web runtimes call the same machine;
drawing consumes the projection (IR-06). Which side of the origin the fill
grows from and how negative fill is expressed are declared by the
negative/positive start/span fields and their hooks. No runtime-specific
value path — the stop condition was not triggered.

### R2.3 — Gesture effects: **declared intent, machine-owned semantics**

Begin/move/end semantics and nearer-thumb selection
(`POINTER_BEGIN`/`MOVE`/`END`, `activeThumb` hold, clamp at sibling) are
implemented by the hand-written `rangeSliderControlTransition`. The
definition declares the gesture **intent**: the value-change/value-commit
events with their firing phases (DuringInteraction / OnRelease, pair
payload), the adapter capabilities (pointer capture on the shared root for
the embedded variant, per-thumb focus stops, native scrub-fraction
reporting), the keyboard table (arrows/Home/End per-thumb, Tab focus
cycling), and the conformance reference to the `slider` vector the machines
honor (R1 — the IR declares the machine, it does not absorb it). The
transitions themselves stay in the machine, which is exactly the expected
answer for this component.

## Deliverables (only the card's writable paths)

- `packages/codegen/src/models/range_slider.rs` (new) — the Rust-authored
  RangeSlider definition (`range_slider_definition()` /
  `range_slider_model()`): 7 shared types (`slider-variant`,
  `slider-polarity`, `slider-thumb`, `control-size`, `control-density`,
  `control-size-role`, `orientation`), 16 props + 2 events (the 18-web-prop
  surface) plus the web-only `defaultValue` record, 9 anatomy parts (the
  split fill/center and both variants' controls), 8 `data-*` attributes +
  7 geometry custom properties (RNG-17), the size ladder with the
  contract's fixed rem metrics, the density hit-area adjustments (the §8
  exception, recorded via `DensityAdjustment.applies_to: root`), the
  orientation axis, 9 semantic token refs, the 11 recipe hooks with their
  token chains, accessibility intent, the keyboard table, the 17-field
  VisualState projection, the `slider` conformance reference, and the R §12
  known deltas as extensions (RNG-26/27 + the §12 rows). Module header
  records the R2 answers, the pilot-scoped placement, and the vocabulary
  notes.
- `packages/codegen/src/models/mod.rs` — `pub mod range_slider;`.
- `packages/codegen/src/targets/range_slider.rs` (new) — the
  `range-slider-ts` target (output root `generated/range-slider`, one
  `index.ts` per model). The artifact carries `parts` (id, name, full DOM
  class list — base + modifier for fill/control parts), `attributes` (the
  eight `data-*` names, forms, emission policies, value domains),
  `styleProps` (the seven geometry hook names + the visual field each is
  fed by), and `recipeHooks`. The generic attribute-domain projections are
  reused from `targets/button.rs` (component-agnostic helpers; hoisting to
  a shared module when a third component arrives is a g13.008 question).
- `packages/codegen/src/targets/mod.rs` — `range-slider-ts` registered in
  `selectable()`, **not** in `all()`: a plain `ir:build` over the synthetic
  fixture must never write into a web package.
- `packages/codegen/src/bin/poodle-codegen.rs` —
  `--author-range-slider <OUT> [--check]`, the mirror of `--author-button`.
- `packages/codegen/fixtures/range-slider-model.json` (new, 53 KB) — the
  serialized model, generated by `--author-range-slider` after a validate
  round trip. Other fixtures untouched.
- `packages/{svelte,react}/components/src/generated/range-slider/index.ts`
  (new) — the committed artifact both web components consume.
  Byte-identical in both packages (the parity test proves it).
- `packages/svelte/components/src/RangeSlider.svelte` and
  `packages/react/components/src/RangeSlider.tsx` — the components read the
  artifact: module-level `parts`/`attributes`/`styleProps` maps supply the
  anatomy classes, the eight `data-*` names (via an attribute spread), and
  the seven geometry hook names (into `rangeStyle`). The value derivation
  and the machine calls stay in the components (the runtime's projection,
  CROSS-13/14). No prop, default, class string, emitted value, or geometry
  hook name changed.
- `packages/{svelte,react}/components/test/RangeSlider.generated.test.ts(x)`
  (new) — the definition→DOM tests: the eight attributes emit under the
  artifact's names, the anatomy renders under the artifact's classes
  (standard and embedded variants, two thumbs each), the geometry hooks
  emit under the artifact's names with machine-computed values, and the
  bipolar specimen reports `data-fill-split="true"`.
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` run
  `--author-range-slider` first (write / byte-compare), then the fixture
  through `--target range-slider-ts` into each web component package.
- `packages/codegen/tests/range_slider.rs` (new, 8 tests) — see Tests.
- This log.

Nothing else in the repo changed. No `poodle-ir` change (R2.1's answer is a
finding, not a schema gap — the stop condition was checked against the
contract's own two-part anatomy); no machine touched (`slider.ts`,
`slider.rs` — R1); no `poodle-render`, adapter, or native preview touched
(R6); `machines.json` untouched (R5); `synthetic-model.json`,
`shell-model.json`, `button-model.json` and the button artifacts untouched;
no visual baseline refreshed (`range-slider.css` untouched, R4).

## Design

- **The authoring form.** `range_slider.rs` is ordinary Rust types and
  constructor helpers (spec 063 "Authoring Form"), no macros. Prop order is
  the contract's §3 table order; shared types carry `canonical_ref` to
  `range-slider.md` (component-specific) or `004-shared-control-types.md`
  (cross-component `control-size`/`control-density`/`control-size-role`/
  `orientation`).
- **The R2 reading — the artifact carries the rendered vocabulary, now
  including geometry.** As with b041, `parts`/`attributes`/`recipeHooks`
  carry what the markup renders. The stateful addition: the seven RNG-17
  geometry hooks are declared as `StateAttribute`s named `--poodle-*` and
  emitted in a separate `styleProps` array (name + feeding visual field),
  so renaming a geometry hook in the definition moves the inline style in
  both previews with no hand edit — the value-dependent geometry is part of
  the declared vocabulary, not a hand-written list in the components.
- **The conditional variant anatomy.** The four control parts are
  `ConditionalExpr` on `variant == standard` / `variant == embedded` — the
  expression form of a part render condition, type-checked against the
  `slider-variant` shared type.
- **The declarative vector reference.** `ComponentDefinition.conformance`
  requires the named vector to resolve in the model, so the model carries a
  declarative `ConformanceVector` (id `slider`, applies to both web
  runtimes) with step intents for the machine semantics it relies on —
  INPUT/COMMIT normalization, pair ordering, the crossing invariant,
  begin-selects-nearer-thumb, end-commits, the change/commit split. It
  cites `machines.json` as evidence; it does not duplicate the executable
  data (R1/R5).
- **`defaultValue` and `law` defaults.** React's `value`/`defaultValue` pair
  is controlled-wins (`value !== undefined` wins), not do-not-mix, so it is
  recorded through the props (`defaultValue` web-only) rather than
  `controlled_state` — the same shape as Button's toggle pair (b041
  vocabulary note). `law` defaults to `{ type: "linear" }`, an opaque
  object no `Value` variant can carry, so the prop is `Opaque` with
  `default: None` and the default named in the description — the b041
  nullable-shared-prop pattern extended to opaque payloads.
- **The artifact placement.** The card's writable paths put the artifact
  inside the component packages (`components/src/generated/range-slider/`),
  not the preview packages — the b041 cross-package import papercut (the
  packed tarball could not resolve `../../preview/src/generated/button`)
  does not reproduce; `test:web-pack-install` passes (3/3).

## The R2 proof (step 6, live)

Renamed the `polarity` attribute in `packages/codegen/src/models/range_slider.rs`
(`data-polarity` → `data-polarity-level`), ran `effigy ir:build`, and drove
both previews with a browser (`#components/range-slider`):

| Step | Svelte preview (:4174) | React preview (:4181) |
|---|---|---|
| after rename + rebuild | 5 RangeSliders; **5 carry `data-polarity-level`** (`unipolar`/`bipolar`), 0 RangeSliders carry `data-polarity`; geometry hooks present (`--poodle-range-start: 20%; --poodle-range-end: 80%`) | 5 RangeSliders; **5 carry `data-polarity-level`** (`unipolar`/`bipolar`), 0 RangeSliders carry `data-polarity`; geometry hooks present |
| after restore + rebuild | 5 RangeSliders; **5 carry `data-polarity`**, 0 carry `data-polarity-level` | 5 RangeSliders; **5 carry `data-polarity`**, 0 carry `data-polarity-level` |

One definition change moved both web previews' DOM in one `ir:build` with no
hand edit. The only file touched during the proof was `range_slider.rs`
(renamed, then restored); the restored artifacts contain zero occurrences of
`data-polarity-level` (grep-verified, `ir:check` green). Note for the record:
one `data-polarity` on the Svelte specimen page during the renamed phase
belonged to the out-of-scope single-thumb `poodle-slider` component, which
keeps its own hard-coded attribute.

The geometry half of the proof is encoded in
`one_definition_change_moves_both_web_artifacts`: renaming
`--poodle-range-start` → `--poodle-range-begin` moves both committed
artifacts in one build.

## R5 — the vector passes unedited, and it is thin

- `packages/contracts/headless/vectors/machines.json`: zero diff.
- TS machine: `packages/core/test/conformance.test.ts` runs every non-
  description vector case — 41/41 pass, including the `slider` cases.
- Rust machine: `packages/contracts/headless/tests/conformance.rs`
  `slider_conformance` iterates the same `slider` cases — 10/10 pass.

**Thinness finding (the card's "say that too" clause).** The `slider` vector
has exactly three cases — INPUT snap+clamp, COMMIT clamp-to-max, degenerate-
range widen — and every one drives the **single-thumb** `sliderTransition`.
Nothing in the vector pins the two-thumb path: `rangeSliderTransition`
(thumb-crossing clamp, pair ordering on SET_VALUE, per-thumb COMMIT), the
control machine (`POINTER_BEGIN` nearer-thumb selection, gesture hold,
`POINTER_END` commit), or `rangeSliderVisualState` (bipolar negative/positive
geometry). Those behaviors are exactly the stateful surface R2 stresses, and
the shared vector the roadmap relies on ("shared conformance vectors where
runtime machines remain hand-written") would **not** catch a divergence
between `slider.ts` and `slider.rs` on any of them. The machines agree today
— `wave1.test.ts` pins the two-thumb behavior at unit level in the TS core,
and the Rust machine mirrors it — but the agreement is unit-test-local, not
vector-pinned. For `g13.008`: the roadmap's vector mechanism needs
two-thumb cases before it can be called a safety net for stateful controls.

## The R7 hand-written exception inventory (per runtime)

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* For a stateful control this inventory is the
main output. What came from the definition, what stayed hand-written, and
why.

### Svelte (`RangeSlider.svelte`)

**From the definition (via the artifact):** the eight `data-*` attribute
names; the seven geometry hook names; the root and part class names
(including the `--negative`/`--positive`/`--lower`/`--upper` modifiers);
the attribute value domains (carried in the artifact).

**Hand-written:**

| Exception | Reason |
|---|---|
| the two `<input type="range">` elements and their native attributes (`min`/`max`/`step`/`disabled`, the per-thumb clamped bounds) | DOM element and native-attribute projection are adapter-owned (`NEG-02`, `IR-05`); the definition declares the mappings (`accessibility.native`) |
| the embedded focus stops (`role="slider"`, `tabindex`, per-thumb `aria-valuemin/max/now`, `aria-orientation`) | adapter-owned a11y projection of the declared intent (R3; `RNG-15/20`) |
| event wiring (`oninput`/`onchange` → `send`, the pointer handlers, `embeddedKey` running INPUT then COMMIT) | events are declared intent (`CROSS-05`); delivery is framework lifecycle (`IR-05`) |
| the machine calls (`rangeSliderTransition`, `rangeSliderControlTransition`, `rangeSliderVisualState`, `normalizeRangeValue`, `safeSliderMax`) | the machine is hand-written by design (R1); the definition declares its semantics and pins them by vector |
| `data-state` value selection (`pointerActive ? "active" : "idle"`) and the `data-*` value derivation | the emission-policy/value logic (`CROSS-13`) in the runtime; the names and domains come from the definition. The web targets do not yet evaluate IR expressions — a g13.008 question |
| the geometry values (`lowerNorm * 100`, the `%` projection into `rangeStyle`) | arithmetic is excluded from the expression vocabulary by design (spec 063); the names and their source fields come from the definition's `styleProps` |
| per-thumb aria-label construction (`${ariaLabel} minimum` / `"Minimum value"`) | string interpolation is excluded (spec 063); the rule is declared in `accessibility.aria` |
| the `$derived` resolution of `resolvedSize`/`resolvedDensity` | the VisualState projection's runtime computation (`CROSS-14`, RNG-16) |

### React (`RangeSlider.tsx`)

Mirrors Svelte exactly, plus:

| Exception | Reason |
|---|---|
| `useState`/`useRef` for `uncontrolledValue` and the control machine | the controlled/uncontrolled pair (`CROSS-04`) in React's lifecycle idiom (`IR-05`); `defaultValue` is declared web-only in the definition |
| `isControlled` derivation (`value !== undefined`) | controlled-wins selection, recorded in the definition's `defaultValue` note |
| `CSSProperties` cast on the style object | inline-style mechanics are web-owned |

### Both runtimes, shared

| Exception | Reason |
|---|---|
| recipe hooks are carried in the artifact but consumed by `range-slider.css`, not by component markup | the CSS is the styling seam (R4 — untouched); the definition is now the single record of the hooks and their chains |
| the embedded pointer gesture (pointer capture on the shared root, `pointNorm` fraction, `activePointer` bookkeeping) | pointer capture is an adapter capability (R3; `RNG-15`); the definition declares it, the runtime implements it |
| `safeMax` (`safeSliderMax`) feeding the native `max` | machine-owned degenerate-range guard (`CROSS-19`); declared in the vector's step intents |

## Vocabulary notes recorded for g13.008

- **`PartKind::Repeated` does not cover its own motivating example.** The
  kind requires a `List`-typed prop and yields identical instances; the
  two-thumb anatomy is a fixed `Pair` with per-position semantics, so the
  thumbs are recorded as distinct parts and the renderer hard-codes "two".
  If "a part that occurs N times" is wanted as a first-class shape, the IR
  needs either a fixed-cardinality repetition source or a per-instance
  identity operand — schema work, explicitly out of this card's scope.
- **The value pair is controlled-wins, not do-not-mix.** `defaultValue` is
  declared web-only; the pair is recorded through the props and the
  machine's normalize path, the b041 Button-toggle precedent.
- **`law`'s default is an opaque object literal.** No `Value` variant
  carries `{ type: "linear" }`; the prop is `Opaque` with `default: None`
  and the default named in the description.
- **Geometry hooks are style properties, not `data-*` attributes.** The
  seven `--poodle-range-*` names are declared through the same
  `StateAttribute` mechanism (RNG-17) and emitted as `styleProps`; the
  components read them into `rangeStyle`. The card's "18 props, 8
  attributes" surface counts the DOM attributes only.
- **The `slider` vector is thin on exactly the stateful surface.** Three
  single-thumb cases; no two-thumb case exists. See the R5 finding.
- **The artifact lands inside the component packages.** The b041
  cross-package import papercut does not reproduce; `test:web-pack-install`
  passes.

## Tests (69 in `poodle-codegen` + 1070 web component tests, all passing)

`tests/range_slider.rs` (8 new):

- `range_slider_model_validates_and_round_trips_as_json` — in-memory
  validate clean; serialization round-trips; the committed fixture equals
  the authored model.
- `range_slider_definition_authors_the_full_contract_surface` — 17 props
  (16 + web-only `defaultValue`), 2 events, 15 attributes (8 `data-*` +
  7 geometry hooks), 9 parts, 11 recipe hooks, 7 shared types; key defaults
  (`value` pair `[0, 100]`, `variant` standard, `polarity` unipolar,
  `orientation` horizontal, `centerValue` null); the md rung rem metrics
  (1.5 / 0.375 / 1.0); the orientation axis; the density adjustments
  applied to the root part. Also asserts **no part is `Repeated`** — the
  R2.1 finding is structural, not just logged.
- `slider_vector_declares_the_hand_written_machine_semantics` — the
  component's `conformance` names `slider`, the vector resolves with both
  web runtimes, and `machines.json` still carries the `slider` key
  unedited (R1/R5).
- `both_web_components_carry_the_same_range_slider_derived_artifact` — the
  card's required parity test: the expectation is the target's render of
  the authored definition (derived, never hand-listed), and **both**
  committed web artifacts must equal it byte-exact.
- `artifact_renders_parts_attributes_style_props_and_recipe_hooks` — the
  R2 vocabulary: every part id and class (including the base+modifier
  pairs), every attribute name with its form and value domain, the
  `styleProps` with their source fields, and every recipe hook with its
  chain kinds.
- `artifact_header_names_the_source_definition_and_generator_version` — the
  Generated Artifact Contract.
- `one_definition_change_moves_both_web_artifacts` — the R2 proof encoded:
  renaming `data-polarity` → `data-polarity-level` **and**
  `--poodle-range-start` → `--poodle-range-begin` in a cloned model moves
  **both** committed artifacts in one build.
- `range_slider_artifacts_fail_check_on_drift_and_check_never_writes` — the
  CLI `--target range-slider-ts --check` fails on planted drift + stale
  orphan and leaves the tree byte-identical; `--author-range-slider` gates
  the fixture the same way.

Plus the target's unit tests (part-class projection, 1) and the two
definition→DOM component tests per runtime (Svelte 5, React 5). The
card's required drift proof ran live: planted one line into
`packages/svelte/components/src/generated/range-slider/index.ts` →
`effigy ir:check` exits 1 naming the artifact → restored → exits 0.

## Validation (all step-9 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored shell + button + range-slider models, all targets (range-slider-ts ×2) |
| `effigy ir:check` | 0 — all current (range-slider fixture + both artifacts gated) |
| `effigy ci:rust` | 0 |
| `effigy test:core` | 0 — 41/41 conformance vector cases (slider included), wave1 range tests included |
| `effigy test:components` | 0 — 77 files, 1070 tests (new RangeSlider tests included) |
| `effigy test:parity` | 0 — 165 tests; RangeSlider class-set diff green |
| `effigy check:svelte` | 0 — install-smoke + 707 component files, 0 errors |
| `effigy docs:lint` | 0 |
| `effigy docs:contract-drift` | 0 — the 18-prop surface is unchanged (R4) |
| `effigy docs:callback-drift` | 0 |
| `effigy docs:focus-ring-drift` | 0 |
| `effigy drift:recipes` | 0 |
| `effigy svelte:surface-audit` | 0 |
| `effigy ci:web` | 0 — includes `test:web-pack-install` (3/3, the b041 papercut does not reproduce) |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 69 passed |
| `cargo test --manifest-path packages/contracts/headless/Cargo.toml --test conformance` | 0 — 10 passed, `slider_conformance` included |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |

**R4 proof — the surface is byte-identical.** The components' prop
interfaces were not touched (18 props, same names/types/defaults);
`docs:contract-drift` and `svelte:surface-audit` pass; the parity test's
class-set diff passes; every existing RangeSlider test (core machine tests,
a11y sweep) passes unchanged; the browser samples before and after the
proof show the same attribute set and values (`data-orientation`,
`data-disabled`, `data-variant`, `data-polarity`, `data-fill-split`,
`data-state="idle"`, `data-size`, `data-density` on the same specimen
sliders, and the same geometry hooks).

## Acceptance criteria

- [x] R2's three questions each answered explicitly in the log — R2.1 is
      the negative answer the card exists to elicit (two thumbs are not one
      `Repeated` part; the renderer hard-codes two).
- [x] The machines are untouched and the `slider` vector passes unedited
      (TS 41/41, Rust `slider_conformance`); the vector's thinness on the
      two-thumb surface is recorded.
- [x] 18 props, 8 attributes, no pixel moved (`range-slider.css` untouched,
      class/attribute values identical, parity green).
- [x] A definition change moves the DOM in both web previews — shown live
      (`data-polarity` → `data-polarity-level`, both previews) and encoded
      as a test (attribute + geometry hook renames move both artifacts).
- [x] The hand-written exception inventory exists, per runtime, with
      reasons.
- [x] All step-9 commands exit 0; no baseline refreshed.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
`poodle-render`, adapter, or native preview work (046), no other component,
no IR schema change (R2.1 is a recorded finding, not a schema edit), no
visual-baseline refresh, no hand edit of generated files. The contract
(`docs/contracts/components/range-slider.md`) was not edited: the
definition's two items beyond the contract's portable surface —
`defaultValue` (React's uncontrolled seed) and the rendered split-fill
anatomy — are web-side rendering records (§8 already documents the fill
segments), and the card's "say so" is this log entry.
