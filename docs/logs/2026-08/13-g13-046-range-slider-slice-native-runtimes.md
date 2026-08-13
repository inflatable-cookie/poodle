---
title: g13 batch 046 — RangeSlider stateful proof, the two native runtimes
status: complete
milestone: g13.006 (part 2 of 2 — **closes the milestone**)
owner: Poodle core
updated: 2026-08-13
tags: [log, g13, IR, range-slider, component, gpui, jetstream, render, spec-063, g13.006]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/046-range-slider-slice-native-runtimes.md` on
branch `thread/g13-046-range-slider-slice-native-runtimes`: added the
`range-slider-rust` emitter target (R2 — a sibling of `range-slider-ts`,
which is byte-frozen by b045's tests), emitted a self-contained Rust
artifact into `packages/render/src/generated/` (R1/R1a — plain data, no
`use` of any Poodle crate, pulled in via `#[path]`, in the package that
ships it), and rewired `poodle-render::range_slider` to take its vocabulary
— the variant treatments, the two-thumb anatomy, the state-attribute names,
and the fill-geometry hooks — from the generated definition instead of its
own literals (R3). The card's second job is the headline: **b045's two
findings were re-tested natively, and both hold — the Repeated-anatomy
limitation is structural, not web-specific, and the slider vector's
thinness is shared.** The four-runtime proof ran live: a `data-variant`
rename moved both web previews' DOM and dropped the two thumbs in both
natives (GPUI and Jetstream pixel-verified); a `--poodle-range-positive-span`
rename moved both web previews' inline geometry styles in one `ir:build`.
Both natives were screenshotted in the states Button could not have (R5),
the exception inventory for both natives was written (R6), and `g13.006`
closed.

Per the card's worker rules: no sub-agents; sources read directly (b045's
log first — its findings are the point of the card — plus b036/b042 for the
self-contained Rust route, the `shell-rust`/`button-rust` targets, the
authored `range_slider.rs` model, and both native specimens); no planning/
status authority exercised beyond the card's own writable status line. No
stop condition was reached. `machines.json` passed unedited (R3), the
`slider` vector was **not** widened (card 047's job — recorded below), no
`poodle-ir`/`poodle-codegen` dependency entered `packages/render/Cargo.toml`
(asserted by test), no visual baseline was refreshed (R4), and no native
preview source was touched.

## Environment note — the Jetstream symlink, repointed for the proof

As b036/b042 recorded, `jetstream-poodle` is a sibling-repo path dep that
only resolves with one spelling of the tree. The `poodle-wt/poodle` symlink
pointed at the main checkout, so a Jetstream build through it would compile
the main repo's `poodle-render` — not this card's. The symlink was
temporarily repointed at `/Users/tom/Dev/projects/poodle-wt/b046` for the
Jetstream captures (the exact b042 pattern), then restored to
`/Users/tom/Dev/projects/poodle`. All Jetstream invocations used
`--manifest-path /Users/tom/Dev/projects/poodle-wt/poodle/packages/jetstream/preview/Cargo.toml`
(one spelling, no lockfile collision). Environment repair, not a repo
change; nothing of it is staged.

## The R2 answers — does each b045 finding generalise?

Both findings were re-tested natively against `poodle-render`'s node tree.
**Both generalise.** R2.1 is the stronger result the card's worker rules
hoped for: the limitation is structural, not a web-renderer gap.

### R2.1 — Repeated anatomy: **structural. The node path hard-codes "two" too.**

b045 found the web renderer hard-codes "two". `poodle-render`'s
`range_slider.rs` does the same, and the node tree gives it no better
mechanism to declare with:

- `make_thumb()` is called exactly twice (`thumb_lo`, `thumb_hi`), two
  thumb layers are attached (`low_thumb_layer`, `high_thumb_layer`), and
  the machine's thumb identity is a fixed two-value enum
  (`RangeThumb::Lower | RangeThumb::Upper`). The count "two" is a literal
  in the render.
- The `poodle-node` vocabulary has no repetition construct at all — `Node`
  is a plain tree of `children: Vec<Node>`; there is no `Repeated`-kind
  analog to be missing.
- The definition records the two thumbs as two **distinct** parts
  (`control-lower`/`control-upper`, `embedded-lower`/`embedded-upper`) —
  the contract's own anatomy (§2), exactly as b045 recorded. Card 046's
  wiring (R3) anchors the hard-coded count to that declared pair: the
  two-thumb block renders only while the definition declares both control
  parts, but the render must still build exactly two nodes.

So the answer is: **yes, `range_slider.rs` also hard-codes "two"** — and
that makes the finding structural rather than web-specific. The IR's only
repetition mechanism (`PartKind::Repeated` over a `List`-typed prop,
yielding identical instances) cannot express the fixed two-slot pair in
either rendering model; the DOM renderer and the node-tree renderer both
fall back to a hard-coded pair anchored to two named parts. This is the
stronger result for `g13.008`: the `Repeated` kind's doc-comment example —
"the two RangeSlider thumbs" — does not apply at the IR level, independent
of who renders.

Encoded as tests: `the_standard_render_builds_exactly_two_thumbs` (the
render crate pins the count at the node level — two shadowed thumbs for a
standard spec, zero for embedded) and the b045 suite's
`no part is Repeated` assertion, which this card extends to the native
consumption path (`part_declared("control-lower")` + `part_declared("control-upper")`
gate the thumb block).

### R2.2 — Vector thinness: **shared. The Rust machine's two-thumb path is exercised by nothing in the vector.**

The natives consume the same `slider` vector:
`packages/contracts/headless/tests/conformance.rs` `slider_conformance`
iterates the three `machines.json` `slider` cases — and every one drives
the **single-thumb** `slider_transition` with a `SliderContext` (a single
`value: f64`). Nothing in the vector touches the Rust two-thumb surface:
`range_slider_transition` (pair ordering on SET_VALUE, per-thumb COMMIT),
`range_slider_control_transition` (POINTER_BEGIN nearer-thumb selection,
gesture hold, POINTER_END commit), or `range_slider_visual_state` (bipolar
negative/positive geometry) — the same functions `poodle-render`'s
`range_slider` calls for every specimen.

What exercises the Rust two-thumb path today is unit tests only:
`packages/contracts/headless/src/slider.rs` `control_tests`
(`bipolar_fill_grows_from_center`, `bipolar_range_splits_negative_and_positive_fill`,
`bipolar_range_only_squares_a_join_when_both_segments_meet`,
`range_gesture_keeps_the_chosen_thumb`) and `poodle-render`'s RangeSlider
scrub tests. The agreement between `slider.ts` and `slider.rs` on the
two-thumb surface is unit-test-local in both runtimes, vector-pinned in
neither — **the thinness is a shared finding, not web-only**, and the
roadmap's vector mechanism cannot catch a divergence on any two-thumb
behavior on either side. For `g13.008`: two-thumb cases belong in the
vector before it can be called a safety net for stateful controls.

## Deliverables (only the card's writable paths)

- `packages/codegen/src/targets/range_slider_rust.rs` (new) — the
  `range-slider-rust` sibling target (R2): the same definition as a
  self-contained Rust artifact — `RANGE_SLIDER_DEFINITION`, a `pub static`
  of plain data structs (`RangeSliderDefinition`/`RangeSliderPart`/
  `RangeSliderAttribute`/`RangeSliderStyleProp`/`RangeSliderRecipeHook`),
  zero `use` of any Poodle crate (R1), `#![allow(dead_code)]` by design.
  The vocabulary projections are shared with the existing targets via the
  widened `pub(crate)` helpers from b042/b045 (`attribute_values`,
  `emission_name`, `form_name`, `link_kind_name`, `shared_member_names`,
  `value_visual_field`, the `range_slider` part-class projection,
  `shell_rust`'s `rust_string_literal`/`static_name`) — one home for each
  projection; the `range-slider-ts` bytes did not move (`ir:check` + the
  b045 parity tests prove it). Over-length literals (the
  `embedded-control` class pairs) emit as single backslash-continuation
  literals so the artifact is rustfmt-clean (the `shell-rust` precedent;
  adjacent literals do not parse in field position — verified).
- `packages/codegen/src/targets/mod.rs` — `range-slider-rust` registered
  in `selectable()`, not `all()` (same scoping rule: a plain `ir:build`
  over the synthetic fixture never writes into a consumer package).
- `packages/render/src/generated/range-slider/index.rs` (new, committed) —
  the artifact, in its **own nested root** under `packages/render/src/generated/`.
  The `generated/` top level belongs to `button-rust`; `write.rs`'s orphan
  sweep deletes top-level files a target did not produce, so two targets
  sharing the top level delete each other's artifacts. The disjoint-roots
  layout (card 041's mechanism, documented in `write.rs`) resolves it —
  the same shape as `range-slider-ts` (`generated/range-slider/index.ts`).
  Carries: the `slider-variant`/`slider-polarity`/`control-size`/
  `control-density` member lists, the nine parts with the DOM class
  projection, the eight `data-*` attributes with names/forms/emission/
  value domains, the seven RNG-17 geometry hooks as `style_props` with
  their source fields, and the 11 recipe-hook chains. `ir:check` gates
  drift in it.
- `packages/render/src/lib.rs` — `#[path = "range-slider/index.rs"] pub mod
  range_slider;` inside `generated`, the generated-module declaration only.
- `packages/render/src/range_slider.rs` — consumes the vocabulary (R3), see
  Design.
- `tasks/effigy.tasks.toml` — `ir:build` / `ir:check` gain the
  `--target range-slider-rust` step into `packages/render/src`.
- `packages/codegen/tests/range_slider_rust.rs` (new, 6 tests) — see Tests.
- `docs/roadmaps/g13/006-range-slider-stateful-control-proof.md` — status
  line only: `planned` → `complete` (046 closes the milestone).
- `PAPERCUTS.md` — one entry (the Jetstream snap overwrite).
- This log.

Nothing else in the repo changed: no `poodle-ir`/`poodle-codegen` dep in
`packages/render/Cargo.toml` (asserted by test, not just avoided), no
`button-rust`/`button-ts`/`shell-rust`/`shell-scene` output change (the
`range-slider-ts` artifacts byte-identical — b045's parity tests + the
proof restores), `machines.json` untouched (R3), no native preview source
touched, no visual baseline refreshed, no web component source touched.

## Design

- **The artifact is the definition, not a slice.** `range-slider-rust`
  renders the full definition in Rust shape — the shared-type member
  lists, the parts, the eight attributes, the seven geometry hooks, and
  the recipe chains — mirroring the `range-slider-ts` artifact's
  information content. Pulled in by `#[path]`, so **no manifest change was
  needed** — which is what makes R1 hold without touching `Cargo.toml`.
- **R3 — the render takes its vocabulary from the definition.** The render
  now discriminates through `RANGE_SLIDER_DEFINITION`:
  - the **size ladder** (`thumb_diameter_rem`, `track_thickness_rem`) is
    gated on the definition's `control-size` members — each rung applies
    while declared, falling back to the default rung (Button's density-arm
    shape);
  - the **variant treatments** are gated on the `slider-variant` members,
    the `data-variant` attribute name, and the declared parts: the
    standard two-thumb block renders only while
    `control-lower`/`control-upper` are both declared, the embedded
    center-marker block only while `center` is declared (the marker is
    the `center` part's node — "hidden in unipolar and in the standard
    variant");
  - the **fill geometry roles** are gated on the declared RNG-17 hooks:
    the negative segment's danger role while
    `--poodle-range-negative-span` is declared, the positive segment's
    accent role while `--poodle-range-positive-span` is declared (the
    spans stay machine-computed — R1; the hooks declare which VisualState
    fields feed them, CROSS-14);
  - the **corner-squaring treatment** is gated on the `data-fill-split`
    attribute name (its value derives from `fillSplitAtCenter`).
  The node tree carries no attribute channel (the `Node` vocabulary is
  frozen), so the names gate the treatments rather than flowing into a
  DOM — a rename drops the treatment until restored, which is the drift
  direction the proof demonstrates (the b042 shape).
- **The proof value.** b042 renamed `data-has-leading`; this card renames
  `data-variant` — the one attribute whose treatment (the two thumbs) is
  visible in all four previews on every default specimen. The geometry
  channel is proven separately with `--poodle-range-positive-span`
  (web-live; the native fill gates share the same mechanism).
- **R2.1's native shape.** The two-thumb block's parts gate is the
  render-side encoding of the R2.1 answer: the render hard-codes the count
  two, but only renders while the definition declares the two distinct
  parts. The node path expresses no more than the DOM path — the finding
  is structural.

## The R2 proof (step 6, live)

### Rename 1: `data-variant` → `data-variant-level` (the four-runtime proof)

One definition edit, one `ir:build`, all three artifacts moved (render
artifact + both web artifacts, each carrying exactly one occurrence of the
new name, zero of the old — grep-verified).

| Runtime | Observation |
|---|---|
| Svelte | 5 RangeSliders; **5 emit `data-variant-level`**, 0 emit `data-variant`; full attribute set otherwise unchanged (`data-orientation`, `data-disabled`, `data-polarity`, `data-fill-split`, `data-state`, `data-size`, `data-density`, geometry hooks) |
| React | 5 RangeSliders; **5 emit `data-variant-level`**, 0 `data-variant` |
| GPUI | pixel diff renamed vs restored: **9355 changed samples**, bbox (526, 932)–(1049, 1808) at 2696×2396 — entirely within the specimen content region; accent-fill count in the specimen region 330px higher in the renamed capture (2468 → 2798): the two thumbs' coverage of every fill is gone |
| Jetstream | pixel diff renamed vs restored: **9580 changed samples**, bbox (16, 41)–(331, 639) at 900×640 — every in-frame slider row; the Default row (y48) shows the two thumb clusters at x≈78–89 and x≈258–269 in the restored capture and **zero thumb pixels** in the renamed one (fill uncovered: 148 → 178 accent px) |

Restored → one `ir:build` → all three artifacts back (`data-variant`, zero
`data-variant-level`), both web previews back to 5/5 `data-variant`
(browser re-check after reload), GPUI and Jetstream captures restored,
`ir:check` 0. The only file touched during the proof was
`packages/codegen/src/models/range_slider.rs` (renamed, then restored); the
restored artifacts contain zero occurrences of the temporary name.

### Rename 2: `--poodle-range-positive-span` → `--poodle-range-positive-width` (the geometry channel)

One `ir:build` moved all three artifacts. Both web previews: **5/5
RangeSliders emit `--poodle-range-positive-width`** with the
machine-computed value (`60.00000000000001%` on the default specimen), 0
emit the old name. Restored → 0 occurrences of the temporary name,
`ir:check` 0. The native fill gates are the same
`style_prop_declared` mechanism the variant proof exercised; the artifact
level is encoded in
`one_definition_change_moves_all_three_range_slider_artifacts` (renames
both values in one test).

## R5 — both natives, the hard states (screenshots)

Captures (current state, restored): GPUI
`/tmp/g13-046-gpui-restored.png` (2696×2396), Jetstream
`/tmp/g13-046-jetstream-restored.png` (900×640), plus the renamed-state
pairs for the diffs above. No vision model was available, so content was
verified from pixel evidence (the same approach b042 recorded).

- **Two thumbs at rest.** GPUI: every standard slider's fill carries the
  two thumb notches (renamed capture's fill is 330px wider across the
  specimen region — the thumbs are the difference). Jetstream Default row
  (y48): two distinct thumb clusters at x≈78–89 (20%) and x≈258–269 (80%)
  over the 20%–80% accent fill; the fill's 148px run equals the full
  180px window minus the thumbs' 32px coverage.
- **Bipolar fill with a negative value.** Both natives' Embedded bipolar
  specimen (`[-0.6, 0.35]` on `[-1, 1]`): the negative fill renders in the
  status-danger red to the left of the center reference alongside the
  positive accent fill (Jetstream bipolar row y≈408–420: danger-dominant
  pixels x64–123 plus accent pixels; GPUI saturated-color scan shows the
  same two families).
- **Crossed or clamped.** **Neither preview specimen passes crossed values**
  (all specs keep `low ≤ high`), so there is no crossed pixel to capture —
  and the preview sources are not this card's writable paths, so a crossed
  specimen cannot be added here. The clamped state is proven at the render
  level instead: `a_crossed_spec_renders_the_ordered_clamped_pair` — a
  `(80, 20)` spec renders the machine's ordered pair (thumb anchors at
  0.2/0.8, identical to a `(20, 80)` spec), i.e. the thumbs can never
  visually cross (RNG-12 `normalize_range_value`). Recorded honestly as a
  render-level proof, not a screenshot.
- **The Jetstream frame check (the b042 warning).** The 900×640 snap's
  visible rows, measured from the fill bands: Default (y44), With step
  (y96), Positions ×4 (y148–208), Custom (y256), Disabled (y308), Embedded
  unipolar (y360), **Embedded bipolar (y≈412 — inside the frame)**, then
  Sizes xs–lg (12 rows, y464–624). Sizes xl and Densities sit below the
  fold. All three R5 states land inside the frame, so the snap's evidence
  is valid.

**R4 — no baseline moved.** The restored-state renders are the pre-proof
code path with all gates passing: the only render-side changes are
whitespace-safe gates whose conditions all hold today (the render crate's
182 tests pass unedited, including every existing RangeSlider test), and
the restored captures show the two thumbs and both fill roles present.
No baseline was refreshed; no stop condition triggered.

## R3 — the machine stays hand-written, the vector stays fixed

`packages/contracts/headless/vectors/machines.json`: zero diff (git-clean
at the end of the batch). The Rust machine is untouched; the definition's
declarative `slider` conformance vector resolves with both web runtimes as
b045 shipped. The vector was **not** widened — its thinness is the R2.2
finding above, and card `047` owns vector coverage. Fixing it here would
blur the evidence `g13.008` needs.

## The R6 hand-written exception inventory (GPUI and Jetstream)

Spec 063's acceptance: *"hand-written exceptions are zero or explicitly
justified in the pilot log."* b045 covered Svelte and React; this is the
GPUI and Jetstream extension. Both natives consume the same
`poodle_render::range_slider` node, so the render-side story is one row;
the host-side rows differ per preview.

### From the definition (via the artifact → `poodle-render`)

The size ladder and its rem metrics (rungs gated on `control-size`); the
variant treatments and their value domains (standard two-thumb anatomy
gated on the declared `control-lower`/`control-upper` parts, embedded
center marker gated on the declared `center` part); the eight `data-*`
attribute names (names gate the native treatments; the value domains are
carried in the artifact for the web channel); the seven RNG-17 geometry
hooks (names gate the fill roles; the values stay machine-computed); the
recipe-hook chains (carried; consumed by the web styling seam).

### GPUI (`packages/gpui/preview/src/specimens/range_slider.rs` — host side)

| Exception | Reason |
|---|---|
| the specimen layout (Eyebrow headings, rows, sections) | specimen chrome — the preview's arrangement of rendered nodes, not RangeSlider's definition (the web specimen pages are the same category) |
| the `RangeSliderSpec` constructions (values, bounds, step, polarity, size/density per specimen) | specimens are authored content — the pilot's specimen layer stays hand-authored (spec 063) |
| change handlers and `NodeSpecimenEvent` state (the lo/hi readouts) | interaction wiring is host-owned (IR-05); the live value readouts are preview state, not rendering |
| `poodle_gpui_node_backend::to_gpui` node interpretation | the adapter's drawing (IR-06): the render produces the node, the backend paints it |
| `RangeSliderSpec` defaults and `resolve_semantic_size` | the frozen poodle-specs surface (R4): RangeSliderSpec keeps its fields; the render projects the enums onto the definition's names (CROSS-14) |

### Jetstream (`packages/jetstream/preview/src/specimens/range_slider.rs`)

Mirrors GPUI (same `poodle_render::range_slider` via `compat::js_range_slider`), plus:

| Exception | Reason |
|---|---|
| the `nel`/`compat` El-building (group/label/row wrappers) | the preview's element language for specimen chrome — host-owned, like GPUI's Eyebrow rows |
| the `snap`/`a11y` bins' viewport and projection | tooling, not RangeSlider rendering |

### Both natives, shared

| Exception | Reason |
|---|---|
| the fill geometry numbers (`lowerNorm * 100` projection, span splits) | arithmetic is excluded from the expression vocabulary by design (spec 063); the RNG-17 hook names and their source fields come from the definition's `styleProps` |
| the value pair ordering (crossed specs clamp to the ordered pair) | machine-owned (`normalize_range_value`, RNG-12); declared in the vector's step intents |
| the gesture machine (`range_slider_control_transition`: nearer-thumb pick, gesture hold, pair commit) and the scrub overlay | the machine is hand-written by design (R1); the definition declares its semantics and pins them by vector; pointer capture/scrub-fraction are adapter capabilities (RNG-13/15) |
| the token names (`color.status.danger`, `color.accent.base`, …) resolved in the render | the definition records the tone→token mapping in its recipe hooks (RNG-21), but the render resolves the family colors directly; the recipe chains are consumed by the web styling seam — the same inventory line as b042's recipe-hooks entry, still a g13.008 question |
| the boolean value domains of `data-disabled`/`data-fill-split` carried, not consumed | the native node tree has no value channel (no DOM): the domains gate the web DOM's values; the names gate the native treatments |
| the enum→name projection (`SliderVariant::Standard` → `"standard"`) | the frozen poodle-specs enums (R4) projected onto the definition's words — the native counterpart of the web components' value derivation (CROSS-14) |

**The trend the verdict needs, stated plainly.** Button's native remainder
(b042) was large — DOM element, event wiring, every derived value,
per-attribute derivation — and RangeSlider's is comparably large: the
machine (gestures, ordering, geometry numbers), the size resolution, the
token resolution, the specimen chrome. Two data points are still not a
trend, but the stateful control did not shrink the remainder; it added the
machine and the geometry projection to it. That is the number `g13.008`
turns on.

## Tests (82 in `poodle-codegen` + 182 in `poodle-render`, all green)

`tests/range_slider_rust.rs` (6 new):

- `render_artifact_matches_the_target_render` — the card's parity test: the
  expected artifact is the `range-slider-rust` target's render of the
  authored definition (derived, never hand-listed), and the committed
  `packages/render/src/generated/range-slider/index.rs` must equal it
  byte-exact. Also asserts the artifact imports no Poodle crate (R1).
- `render_artifact_carries_the_rendered_vocabulary` — variants/polarities/
  sizes/densities member lists; every part with its DOM class (including
  the base+modifier pairs — the emitter's backslash-continuation folded
  back); every attribute name/form/emission/value domain (orientation,
  variant, polarity, boolean, size, density domains; `data-state` carries
  no domain); every styleProp with its source field; every recipe hook
  with its chain kinds.
- `artifact_header_names_the_source_definition_and_generator_version` —
  the Generated Artifact Contract.
- `one_definition_change_moves_all_three_range_slider_artifacts` — the R2
  proof encoded: renaming `data-variant` **and**
  `--poodle-range-positive-span` moves the render artifact and both web
  artifacts in one build.
- `render_manifest_carries_no_poodle_ir_or_codegen_dependency` — reads
  `packages/render/Cargo.toml` and asserts neither crate is named (R1
  asserted, not just avoided).
- `range_slider_rust_artifacts_fail_check_on_drift_and_check_never_writes` —
  planted drift + stale orphan in a render artifact fails `--target
  range-slider-rust --check` and check mode leaves the tree byte-identical.

Render crate (`range_slider.rs` tests, 3 new; all 182 pass unedited R4):

- `the_standard_render_builds_exactly_two_thumbs` — the R2.1 native answer
  encoded: the standard render builds exactly two shadowed thumbs; the
  embedded render builds none.
- `a_crossed_spec_renders_the_ordered_clamped_pair` — the R5 clamped state
  at the render level: `(80, 20)` displays as the ordered pair
  (thumb anchors 0.2/0.8).
- plus the existing suite untouched (grab-overlay scrub, nearer-thumb
  press, gesture hold, release-click no-op, size metrics).

## Validation (all step-9 commands exit 0)

| Command | Exit state |
|---|---|
| `effigy ir:build` | 0 — authored models, all targets incl. `range-slider-rust` |
| `effigy ir:check` | 0 — all current (range-slider fixture + both `range-slider-ts` artifacts + the render artifact gated) |
| `effigy ci:rust` | 0 — `slider_conformance` 10/10 included |
| `effigy ci:web` | 0 — includes `test:web-pack-install` (3/3) |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |
| `cargo test --manifest-path packages/codegen/Cargo.toml` | 0 — 82 passed |
| `cargo clippy --manifest-path packages/codegen/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/codegen/Cargo.toml -- --check` | 0 |
| `cargo test --manifest-path packages/render/Cargo.toml` | 0 — 182 passed |
| `cargo clippy --manifest-path packages/render/Cargo.toml --all-targets -- -D warnings` | 0 |
| `cargo build -p poodle-gpui-preview` | 0 (worktree manifest) |
| Jetstream preview build + `snap -- specimens --slug=range-slider` through the symlink path | 0 |

**Rust drift proof (required test, live).** Planted drift in a scratch
render artifact → `--target range-slider-rust --check` exits 1 naming the
artifact → restored → exits 0. (The test suite covers the same property;
the live proof is the `ir:check` run after each rename/restore cycle.)

(The render crate's `cargo fmt --check` reports pre-existing drift in files
this card did not touch — `tabs.rs`, `theme_select.rs`, `app_header.rs`,
etc., present on the branch before this card; this card's files and the
generated artifact are rustfmt-clean. Render-crate fmt is not part of any
gate this card runs — the same note b042 recorded.)

## Acceptance criteria

- [x] One definition change visible in all four previews — `data-variant`
  rename, live: both web DOMs' attribute name, both natives' thumbs
  (GPUI 9355-sample diff; Jetstream 9580-sample diff with the Default-row
  thumb clusters as the crisp proof).
- [x] R2's two questions answered — **both b045 findings generalise**:
  the two-thumb limitation is structural (the node path hard-codes "two"
  too), and the vector thinness is shared (the Rust machine's two-thumb
  path is exercised by nothing in the vector).
- [x] `poodle-render` depends on neither `poodle-ir` nor `poodle-codegen`
  (asserted by test; artifact pulled in by `#[path]`).
- [x] The `slider` vector passes unedited and was **not** widened
  (`machines.json` zero diff; thinness recorded, coverage is 047's).
- [x] Both natives screenshotted including two-thumb and bipolar states
  (crossed/clamped proven at render level — no specimen crosses, preview
  sources not writable).
- [x] Exception inventory covers GPUI and Jetstream (R6).
- [x] `g13.006` marked complete.
- [x] All step-9 commands exit 0; no baseline refreshed.

## Not done

Per batch card and worker rules: no merge (branch pushed only), no
`poodle-ir` schema change (R2.1 is a recorded finding — a schema gap would
be a stop condition, and the anatomy fits the contract's two-part shape
without one), no machine or vector edit (R3), no vector widening (047's
job), no native preview source change (the crossed specimen cannot be
added here), no visual baseline refresh (R4), no hand edit of generated
files. The web components' geometry-value derivation and the natives'
token resolution remain the recorded g13.008 questions from b045/b042.
