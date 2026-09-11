# 022 — block-first Slider family

Status: ready to dispatch — operator approved 2026-09-11
Owner: Poodle cross-runtime components
Created: 2026-09-11
Governing refs: `../../contracts/components/slider.md`,
`../../contracts/components/range-slider.md`,
`../../architecture/012-feedback-motion-and-state-change.md`
Depends on: g18.017 complete and merged

## Outcome

Make the rounded-square block presentation the default public `Slider` and
`RangeSlider`. Keep the dense track-and-thumb treatment only as
`variant="embedded"`. Remove the old `appearance`, `standard`, and `track`
public choices without aliases.

Finish the family rather than only restyling it: block supports horizontal and
vertical orientation, Slider supports unipolar and bipolar geometry, and
RangeSlider uses fixed endpoint values with a centered label. Text never moves,
hides, or leaves the capsule when a thumb or selected fill crosses it.

This task may run beside g18.011. Both must close before retained g18.006
release-candidate work resumes.

## Ready-State Rubric

- [x] g18.017 established rounded-square block corners and fixed split-colour
  text for horizontal Slider.
- [x] Current RangeSlider still uses value-dependent three-region placement
  and an external fallback.
- [x] Current block inputs reject vertical orientation in every active runtime.
- [x] The operator chose one breaking pre-v1 API: `variant="block"` or
  `variant="embedded"`, default `block`, for both controls.
- [x] The operator rejected compatibility aliases for `appearance`, `track`,
  and `standard`.
- [x] Horizontal RangeSlider placement is fixed: lower value at left, label
  centered, upper value at right.
- [x] Vertical placement is fixed and upright: Slider value at top and label
  centered; RangeSlider upper value at top, label centered, lower value at
  bottom.
- [x] Block Slider must prove unipolar and bipolar geometry.
- [x] The work is path-independent from the active editor-only g18.011 sweep.

## Decisions

- `variant` is the only public presentation switch. Its values are `block` and
  `embedded`; omission resolves to `block`.
- Delete `appearance`, `SliderAppearance`, `RangeSliderAppearance`,
  `variant="standard"`, `appearance="track"`, and their dead branches. Do not
  deprecate, alias, coerce, or silently fall back.
- Delete RangeSlider's combined `formatVisibleRange`/resolved-range surface;
  the fixed block layout always presents independently formatted lower and
  upper endpoint values.
- `block` is the ordinary standalone control. It uses rounded-square capsule
  geometry, small circular handles, explicit visible content, custom pointer
  and keyboard interaction, and per-thumb 44×44 effective targets.
- `embedded` retains the compact track-and-thumb visual and behavior used in
  dense composites. It is the only remaining form of the former track
  presentation.
- Orientation and polarity are orthogonal to variant. Both variants accept
  horizontal and vertical orientation; Slider accepts unipolar and bipolar;
  RangeSlider preserves its unipolar/bipolar selected-window law.
- Horizontal Slider keeps the g18.017 fixed row: label at logical start and
  value at logical end. Vertical Slider keeps upright text with value at the
  physical top and label centered.
- Horizontal RangeSlider keeps three fixed whole-capsule anchors: lower value
  at logical start, label centered, upper value at logical end. Vertical
  RangeSlider keeps upper value at physical top, label centered, and lower
  value at physical bottom. Numeric scale direction remains minimum at logical
  start or bottom and maximum at logical end or top.
- Paint identical fixed text through remainder and selected clipping layers.
  Fill or thumb crossover changes foreground role only. Handles paint above
  text. Focus, drag, equality, overlap, RTL, value updates, and selected-window
  width cannot change text position or visibility.
- Fit is whole-capsule and deterministic. Endpoint/current values are required.
  If label and required values cannot coexist, suppress only the optional
  label. Never render the old external fallback or move text outside the
  capsule.
- Horizontal RangeSlider endpoint placement mirrors with the existing RTL
  scale and pointer normalization: lower stays at logical start and upper at
  logical end.
- Keep visible strings separate from accessible names and value text. Web
  formatters remain web-only; native specs carry resolved strings.
- This is a breaking public change for the pre-v1 `0.4.0` release and must be
  named in migration/release evidence.

## UI Design Brief

- **Mode:** Operate. Compact, legible controls for settings, inspectors, and
  creative-tool workbenches.
- **Visual target:** rounded-square surface capsule by default; selected range
  uses accent, remainder uses surface, circular handles remain small and
  tactile. Embedded stays the quiet dense alternative.
- **Hierarchy:** values are primary operational feedback; labels are secondary
  context. Range endpoints occupy the scale ends and the label owns the visual
  center.
- **State law:** text is spatially invariant. Crossing a fill boundary inverts
  brightness via semantic selected/remainder text roles; it never reflows,
  jumps, disappears, or moves outside the control.
- **Orientation:** vertical text stays upright. Vertical scale reads maximum at
  top and minimum at bottom.
- **States:** prove default, hover, active, keyboard focus, disabled, extrema,
  equal RangeSlider thumbs, narrow fit, LTR/RTL, every size, both polarities,
  Eclipse/Iceberg, and forced colours.
- **Motion:** none. Architecture 012 static feedback remains authoritative.

## Dispatch manifest

- **State:** operator-approved for dispatch; safe to run in parallel with
  retained g18.011; serial before retained g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** Slider/RangeSlider Svelte and React components,
  styles, shared TypeScript core, contracts/specs/headless/render Rust,
  GPUI adapter/preview, focused tests, paired specimen pages and generated
  component docs, public API reports and one g18.022 execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch/task status;
  g18.011 task/thread/workspace and editor paths; g18.006/g18.009 task state;
  versions, changelog, release notes, workflows, release/tag/publication,
  Desktop, Jetstream admission, unrelated controls
- **Worker:** high-reasoning cross-runtime component worker comfortable with
  interaction geometry, CSS clipping, Rust layout/render parity, GPUI headless
  proof, accessibility, and breaking TypeScript API migrations
- **Excluded:** compatibility aliases; general slider redesign; tooltip/tick
  APIs; motion; public fit metrics; windowed selectors; release mutations
- **Escalation:** Chatterbox if vertical native axis proof requires new shared
  node vocabulary, stable text cannot be clipped without moving content, or
  removal of the old API exposes an unclassified downstream break

## Work

1. Plant contract/API failures proving both controls expose only
   `variant="block" | "embedded"`, default block, and reject the removed
   `appearance`, `track`, `standard`, and combined visible-range vocabulary at
   compile/parse time.
2. Consolidate public types and normalized specs in TypeScript and Rust. Delete
   obsolete standard/track/fallback branches and update public API reports.
3. Complete block vertical geometry in Svelte, React, shared composition, and
   GPUI. Keep text upright, pointer math axis-correct, focus truthful, and hit
   targets measurable.
4. Replace RangeSlider's three-region/fallback layout with fixed whole-capsule
   anchors and clipped selected/remainder copies. Prove the horizontal and
   vertical placement laws at extrema, overlap, equality, RTL, and narrow fit.
5. Prove Slider block unipolar and bipolar fill from resolved visual state in
   both orientations. Do not rederive polarity in renderers.
6. Retain the old dense track-and-thumb rendering only under `embedded` and
   prove its horizontal/vertical behavior did not regress.
7. Update paired Svelte/React specimens to make default block, vertical block,
   bipolar block, RangeSlider fixed endpoints, narrow fit, and embedded
   comparison visible. Regenerate checked-in docs artifacts.
8. Run focused core/component/Rust/GPUI checks, paired real-browser specimens,
   accessibility and forced-colour checks, package builds/audits, docs QA, and
   `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Block is the real default | omission still renders the old track | paired DOM/spec/render assertions for omitted `variant` |
| Old API is gone | an alias or hidden branch still accepts `appearance="track"` | type/API-report/parser failures plus dead-code search |
| Embedded preserves density | removing standard also deletes the compact composite control | horizontal/vertical embedded interaction and visual regressions |
| Range text is fixed | label or endpoint follows a thumb, vanishes, or drops below the capsule | paired browser and GPUI layout assertions across values and narrow fit |
| Crossover is paint-only | a glyph changes position at selected-window boundaries | stable bounds with selected/remainder computed-colour changes |
| Vertical block is native | web rotates the whole capsule/text or GPUI rejects it | upright text bounds and axis-correct pointer/keyboard proof in every runtime |
| Endpoint order is meaningful | vertical lower appears at top or RTL text disagrees with the mirrored scale | top=upper/bottom=lower and logical-start=lower/logical-end=upper assertions |
| Bipolar block is real | block ignores center or paints one undifferentiated fill | positive/negative segment visual-state and renderer tests in both axes |
| Accessibility survives | visible label substitutes for thumb names or orientation is omitted | paired a11y assertions for role, names, bounds, values, orientation, focus order |
| Release stays closed | candidate resumes while this breaking source is moving | retained g18.006 remains paused until merged g18.022 and accepted g18.011 |

## Stop conditions

- Stop if removing the old API requires a compatibility shim or silent
  coercion. Return the breakage inventory instead.
- Stop if vertical block support cannot preserve upright text, two independent
  RangeSlider focus stops, or 44×44 effective targets.
- Stop before editing editor sweep state, release state, versions, changelog,
  workflows, Desktop, or unrelated controls.
- Stop before every windowed selector unless the operator separately approves
  it.

## Evidence

Planning inspection on 2026-09-11 found:

- g18.017 merged horizontal Slider split-colour text and rounded-square family
  corners but explicitly left RangeSlider's per-region fallback and all
  vertical block work out of scope;
- both contracts still expose orthogonal `variant="standard" | "embedded"`
  and `appearance="track" | "block"` props with old modes as defaults;
- both active web wrappers and shared Rust/GPUI paths reject vertical block;
- RangeSlider still renders separate selected/lower/upper text regions and an
  external fallback; and
- the operator selected the breaking two-variant migration, stable RangeSlider
  endpoint/center label geometry, scale-aligned upright vertical text, and a
  bipolar block Slider requirement.

## Next task

After g18.022 merges and g18.011 finishes with operator acceptance, resume the
same retained g18.006 release-candidate task. Recompute the full `0.4.0` source
identity and classify this breaking pre-v1 migration in its release evidence.
