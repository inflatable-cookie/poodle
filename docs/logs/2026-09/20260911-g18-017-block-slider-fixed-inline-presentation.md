# g18.017 — Block Slider fixed inline presentation

Status: merged
Merge: `f91be412b60739e96c29a45e9c17969c05b85f49` (PR #246) on 2026-09-11
Date: 2026-09-11
Branch: `ns-be813655-c1eb-4b57-af4b-974d2f8c84de`
Card: `docs/roadmaps/g18/017-block-slider-fixed-inline-presentation.md`
Handoff: `docs/handoffs/20260911-g18-017-block-slider-fixed-inline-presentation.md`
Governing refs: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`packages/core/src/slider.ts`, `packages/core/src/styles/slider.css`,
`packages/core/src/styles/range-slider.css`,
`packages/contracts/headless/src/slider.rs`,
`packages/render/src/slider.rs`, `packages/render/src/range_slider.rs`
Base: `origin/main` at `cc67412d89cf4cbff66ca13d8365c0c8650fd1a0`

## Outcome

Block Slider and block RangeSlider capsules resolve the rounded-square
`radius.control` corner token instead of the pill in web CSS, shared Rust
composition, and GPUI; visible thumbs stay circular. A single block Slider no
longer assigns its label/value to value-sized regions: the label is pinned to
the logical inline start and the value to the logical inline end at every
value, one stable text row paints through two clipped foreground layers
(selected/remainder roles), and the shared all-or-nothing segment fit law with
its external fallback line is replaced by a deterministic whole-track
collision rule that suppresses the optional label first and never moves,
truncates, or ejects the exact numeric value. RangeSlider keeps its
three-region inline placement, per-region fit law, and fallback line — radius
identity only. No public prop, callback, keyboard, pointer, or ARIA change.

## What changed

- Contracts first. `docs/contracts/components/slider.md` §2/§4/§8/§9/§10 now
  describe the fixed two-layer inline presentation, the whole-track
  `available >= ceil(label) + ceil(value)` coexistence law (equality fits),
  label-first suppression with the value never suppressed, the
  `radius.control` capsule corner, the circular thumb, and the
  forced-colour mapping of the two text layers (`HighlightText` /
  `CanvasText`). `docs/contracts/components/range-slider.md` records the
  radius-only family change and restates that its inline placement and fit
  law are unchanged.
- `packages/core/src/styles/slider.css`: block capsule
  `border-radius: var(--poodle-radius-control)`; the fill/remainder segments
  are paint-only (they no longer host text); new `.poodle-slider__inline`
  layers (full-capsule, `clip-path: inset(...)` bound to
  `--poodle-slider-percent`, mirrored under `[data-direction="rtl"]`) over a
  `.poodle-slider__inline-row` that pins label and value to the logical edges
  with `space-between` and non-shrinking slots; forced-colour rules moved to
  the layers. `range-slider.css` changes the capsule radius only.
- `packages/core/src/slider.ts`: `layoutSliderBlock` is now the whole-track
  collision law returning `{ labelInline, valueInline }` — it takes no
  selected span, so the decision cannot depend on the value.
  **Operator-visible surface removal:** the core root export
  `sliderFallbackText` (`@inflatable-cookie/poodle-core`, re-exported from
  `packages/core/src/index.ts`) is deleted by this migration, not aliased —
  pre-v1, no compatibility shim. It existed only to render the single-Slider
  external fallback line this card retires; no in-repo consumer remained
  after the law replacement, and RangeSlider's fallback helper
  (`rangeSliderFallbackText`) is untouched. Downstream code still importing
  `sliderFallbackText` must drop the single-Slider fallback per this card or
  render its own line.
- Svelte (`Slider.svelte`) and React (`Slider.tsx`) render the identical two
  clipped layers; the external fallback element is gone from the block
  branch. Slot elements stay in the DOM (empty) when an item is absent or
  suppressed, so the surviving string never changes position.
- `packages/contracts/headless/src/slider.rs`: `SliderBlockLayout` /
  `layout_slider_block` mirror the TS law signature-for-signature;
  `RangeSliderBlockLayout` untouched.
- `packages/render/src/slider.rs`: `paint_slider_block` resolves
  `radius.control` for the capsule and builds the split paint from the shared
  node substrate — two absolutely positioned per-region clip containers
  (`LayoutOverflow::Hidden`) each holding one full-capsule-width
  `MainAxisAlignment::SpaceBetween` text row, physically mirrored for RTL,
  stamped with the selection/canvas forced-colour roles, and identified
  (`block-slider-clip-selected/-remainder`, per-layer label/value slot ids)
  for mounted observation. The external fallback line is not rendered.
  `packages/render/src/range_slider.rs` resolves `radius.control` for the
  block capsule; nothing else moves.
- Substrate evidence (recorded per the card's fallback clause): the existing
  node vocabulary already carries per-axis overflow clipping and pixel-fixed
  layout inside the block host's measured width, so the preferred
  split-colour path is expressible cross-runtime without a vocabulary
  addition. The operator-approved larger-side fallback was therefore not
  invoked, and no runtime renders an external row.
- New paired browser fixture `test/block-slider-inline-probe/` (Svelte +
  React harnesses; Chromium + WebKit) with effigy selectors
  `test:block-slider-inline{-chromium,-webkit}`.

## Evidence

- `effigy test:block-slider-inline` (both engines, both frameworks): capsule
  computed radius `6px` while the thumb stays `999px`/square; label and value
  glyph boxes identical at low/mid/high while the fill edge tracks
  10/50/90%; at low the boundary crosses the label glyphs and at high it
  crosses the value glyphs with the two layers showing different foreground
  colours and identical coordinates; narrow-width collision suppresses the
  label in both layers while the exact value stays in the capsule; no
  `.poodle-slider__fallback` exists anywhere; RTL keeps the label at the
  logical start and value at the logical end with split foreground; the
  block RangeSlider shows the rounded-square capsule with its inline
  placement intact; a pointer click at the thumb still dispatches with the
  text layers mounted.
- GPUI mounted (`packages/gpui/preview/tests/headless_regressions.rs`):
  `block_slider_text_layers_stay_fixed_while_the_boundary_moves` proves
  identical label/value bounds at 10/50/90 with clip boxes that track the
  selected span and sum to the capsule width;
  `block_capsule_is_rounded_square_while_the_thumb_stays_circular` proves
  `radius.control` identity for both block capsules and the circular thumb
  radius from shared composition;
  `block_slider_fit_uses_parent_width_and_shaped_advance` now proves the
  shaped-advance fit through label-slot suppression (no fallback exists);
  `block_slider_production_host_height_matches_surface_and_range_keeps_fallback`
  proves the single-Slider host reserves exactly the surface height at any
  width while the RangeSlider still grows for its own fallback line.
- Component suites (`Slider.test.ts` / `Slider.test.tsx`): two clipped layers
  carrying identical strings, paint-only fills, collision journey with the
  value exact and no fallback, RTL layer rendering, and CSS assertions for
  the capsule radius, circular thumb, clip rules and their RTL mirrors, and
  the `HighlightText`/`CanvasText` mapping.
- Core law tests (`packages/core/test/wave1.test.ts`,
  `poodle-headless` unit tests): equality fits, one-pixel collision
  suppresses only the label, absent channels keep their slots, and the law
  takes no selected-span input.

## Review round (2026-09-11, head e3ce0d8ba)

Independent review required two changes, both applied:

1. `packages/svelte/preview/artifacts/recipe-inventory.json` was stale after
   the slider.css rewrite. Regenerated with the architecture-007 generator
   (`bun packages/svelte/preview/scripts/build-recipe-inventory.ts`):
   `--poodle-recipe-slider-block-fallback-text` drops out of the slider
   section and `summary.recipeHooks` returns to 1175. No other component or
   count moved.
2. The contract no longer implies any runtime consumes the Slider-named
   fallback hook. `--poodle-recipe-slider-block-fallback-text` is removed
   from the Slider recipe-hook list with an explicit note: no runtime
   consumes it since g18.017; block RangeSlider keeps its separately named
   `--poodle-recipe-range-slider-block-fallback-text` for its retained
   fallback line.

Review also flagged, as non-blocking, the missing mounted native RTL
text-position assertion. Added:
`block_slider_rtl_mirrors_the_clip_geometry_and_keeps_logical_anchors`
mounts an RTL block Slider and proves the selected clip anchors to the
physical right (72px at value 30, 240px span), the remainder to the physical
left, the label keeps the logical start (physical right, inset 8px) and the
value the logical end (physical left, inset 8px).

## Explicitly not done

- No RangeSlider text redesign: its assigned regions, per-region fit law,
  and fallback line are byte-for-byte the previous behaviour apart from the
  capsule radius.
- No new tokens, props, or public API; the removed `sliderFallbackText`
  export was single-Slider-only and dead after the migration (pre-v1, no
  shim).
- No vertical block admission, no motion, no tooltips/marks.
- g18.006/g18.009 remain held; g18.011 untouched; no release, workflow, or
  Desktop mutations.

## Validation

- `bun test test/wave1.test.ts` (packages/core) — 28 pass.
- `cargo test --manifest-path packages/contracts/headless/Cargo.toml` — 223
  pass.
- `cargo test --manifest-path packages/render/Cargo.toml` — 645 pass; 2
  pre-existing failures (`context::tests::the_provider_adds_no_wrapper...`,
  `segmented_control::tests::icon_only_without_an_icon...`) reproduce
  identically on the clean base commit `cc67412d8` and are unrelated to this
  task.
- `bunx vitest run` — 402 files, 3976 pass (includes the Svelte/React
  component, a11y, and parity projects).
- `effigy test:block-slider-inline` — all Chromium and WebKit checks pass for
  both frameworks.
- `effigy test:block-slider-hit` — the g16.046 hit-target probe still passes
  on both engines (interaction untouched).
- `cargo test --test headless_regressions` (packages/gpui/preview) — 237
  pass, including the four block-Slider mounted proofs above.
- `cargo test` (packages/jetstream/adapter) — 163 pass (compile-compatible
  consumer).
- `bunx svelte-check --workspace packages/svelte/components --threshold
  error` — 0 errors; `bunx tsc -p packages/react/components/tsconfig.json
  --noEmit` — no new errors versus the base commit (remaining Tree/
  BlockEditorBlock errors pre-exist and are unrelated files).
- `effigy docs:lint` — pass after `core:build` + `svelte:build` +
  `react:build` (paired preview dist builds).
- Recipe inventory regenerated against the review head
  (`bun packages/svelte/preview/scripts/build-recipe-inventory.ts`): 147
  components, 1175 recipe hooks, 7 candidates, 410 metric variables — the
  slider section no longer lists the unconsumed
  `--poodle-recipe-slider-block-fallback-text`.
- `effigy test:visual-smoke` — slider compares clean; the single failing pair
  (`pill` 0.665%) reproduces identically on the clean base commit and is
  unrelated.
- `git diff --check` — clean.

## Closeout

- Merge performed by the plugin as
  `f91be412b60739e96c29a45e9c17969c05b85f49` on 2026-09-11 (PR #246),
  with parents `9cc62661d7f9cacdec287d82e356f45331fb81b8` (main) and
  `666ca0fbfdd14189bcfa6ae5fca9fd6c338ca323` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` re-review of
  head `666ca0fbfdd14189bcfa6ae5fca9fd6c338ca323` by betterthanclay
  ([comment #5633380283](https://github.com/inflatable-cookie/poodle/pull/246#issuecomment-5633380283)).
  No findings; the g18.017 implementation is untouched since the earlier
  approved `69ed890ca`, and the delta to the reviewed head holds no
  implementation file.
- Reviewed-head validation (reviewer ran at the exact head, tree left
  clean): `effigy check:parity-evidence-ledger` exit 0 (176 component
  evidence rows); `effigy check:gpui-census` exit 0;
  `bun test scripts/nucleus-parity-receipts.test.ts` 17 pass;
  `effigy audit:tokens` and `effigy drift:recipes` pass;
  `effigy test:block-slider-inline` exit 0, 144 checks pass on Chromium and
  WebKit for Svelte and React; `cargo test --test headless_regressions
  block` 9 pass including the four g18.017 mounted proofs and the RTL
  clip/anchor test; `bun test packages/core/test/wave1.test.ts` 28 pass;
  Svelte/React Slider suites 45 pass; `git diff --check` clean;
  `git status --porcelain` empty. Round-1 carryover still applies:
  `poodle-headless` 223 pass, `poodle-render slider` 35 pass. CI `rust`
  and `web` pass at the merge.
- Worker validation at the branch head is recorded above under Validation;
  the merged head adds only main-side docs commits (g18.019 closeout,
  triage note) over the reviewed head, no source.
- Deferred: the two `poodle-render` failures
  (`context::tests::the_provider_adds_no_wrapper...`,
  `segmented_control::tests::icon_only_without_an_icon...`) and the single
  `pill` visual-smoke pair reproduce identically on the clean base commit
  and stay unrelated to this task. No release, tag, publish, Desktop,
  native, or retained-task work starts from this task. `g18.011` stays
  queued behind g18.014 and g18.018; `g18.006` stays paused and `g18.009`
  stays queued behind it.

## Continuation

Merge before the queued g18.011 web-editor acceptance sweep. Do not resume
g18.006 or g18.009 from this task.
