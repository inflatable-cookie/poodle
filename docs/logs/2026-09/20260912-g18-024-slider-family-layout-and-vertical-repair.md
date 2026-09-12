# g18.024 — Slider-family layout and vertical repair

Status: in review — round 2 at the native-vertical head
Date: 2026-09-12
Branch: `ns-0d7f161b-615a-4570-be16-0bc1cab8c2ef`
Card: `docs/roadmaps/g18/024-slider-family-layout-and-vertical-repair.md`
Handoff: `docs/handoffs/20260912-g18-024-slider-family-layout-and-vertical-repair.md`
Governing refs: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`docs/architecture/012-feedback-motion-and-state-change.md`
Base: `origin/main` at `4255c62ee16b0f35447189509142fc5b7e6e50ae`

## Review rounds

- Round 1 at `ec9e97821…` (PR comment `5645560034`): **changes required** —
  the web half verified green (hit probe 248/0, inline probe 192/0 across
  Chromium + WebKit; ladder, density, hit-out-of-layout, and the step-aware
  serializer correct), but the native vertical half was not repaired and two
  native regressions were measured on a temporary mounted GPUI probe:
  `fraction_anchor_vertical` applied the centring offset to `top` (handle off
  the rail axis and shifted 10px up), and `block_grab_with_axis` kept fill
  sizing alongside the negative cross insets, so filled sizing won and the
  scrub overlay stayed capsule-sized, leaving the 44×44 overflow band dead at
  xs/sm/md. Two pre-existing in-scope defects were also measured: native
  vertical block fills grew sideways (width percentage, value-independent on
  the block axis), and vertical anchor direction was inverted against the
  bottom-referenced scrub axis (range lower thumb above upper). The review
  also flagged the missing mounted vertical coverage and noted the exported
  core helper signature change as non-blocking.
- Round 2 at `00d90ba64…`: all four findings fixed in
  `packages/render` (`slider_block.rs`, `slider.rs`, `range_slider.rs`) with
  three new mounted regressions in `headless_regressions.rs` (vertical single
  Slider geometry, vertical RangeSlider geometry, horizontal xs band
  coverage/dispatch); `effigy regressions:native` re-run at this head
  (241/0) and the Nucleus/GPUI census evidence repinned per precedent.

## Outcome

The block Slider family now consumes the shared control-size axis and renders
as a complete, aligned control at every size and orientation:

- The block capsule consumes the shared control-height ladder (24/28/36/44/52px
  for `xs`–`xl`) in CSS and in shared Rust composition
  (`slider_block::capsule_height_rem`); the private 28/30/32/36/40px ladder is
  gone from both.
- The root's measured layout box equals the visible capsule. The ≥44×44 hit
  rectangle stays measurable, interactive, and out of layout: it lives in
  absolute overflow layers (web) and absolute anchor layers with centred
  overflow (native), and the native scrub overlay extends across the hit
  overflow so the envelope remains draggable.
- Density no longer participates in block layout: the density padding rules
  are scoped to the embedded variant in both stylesheets, and density stays
  independent of size everywhere.
- One shared default display serializer binds both families: the snapped value
  rounds to the decimal precision implied by `min` and a finite positive
  `step` (`sliderDisplayPrecision` / `slider_display_precision`), trims
  insignificant zeroes, normalizes negative zero, and never shows binary
  tails. Without a finite positive step the value keeps its shortest exact
  form. The web `formatVisibleValue` callback and native
  `visible_*_text`/`resolved_visible_text` explicit strings remain the
  authoritative overrides. Value math, callbacks, accessible values, and the
  formatter props are unchanged.
- Vertical block is complete native-axis geometry: the rail cross-size is the
  shared capsule size (no more `max(size, 44px)` width), text keeps a fixed
  value-independent block inset, and the optional RangeSlider label centers on
  the exact rail middle as an absolute overlay (web and native), so the upper
  and lower endpoints stay pinned to the physical top and bottom and the three
  upright rows never clip, shift, or collapse each other. This also retires
  the g18.022 caveat that RangeSlider label centering was exact only for equal
  endpoint advances.
- Specimens put each family beside same-size rows and show representative
  fractional vertical values; the paired probes assert the new law.

## Execution

- Measured the failures first by rewriting `test/block-slider-hit-probe` to
  the new law (five shared heights beside same-size Button/TextInput in both
  frameworks, density independence, capsule-equals-root, 44×44 hit overflow
  and dispatch) and extending `test/block-slider-inline-probe` with
  fractional-serializer and vertical-range-rail oracles; both failed against
  merged main exactly as the card's evidence predicted.
- Fixed the shared law first (`packages/core/src/slider.ts`,
  `packages/contracts/headless/src/slider.rs`), then mirrored layout changes
  across CSS, Svelte/React wrappers, shared Rust composition
  (`packages/render/src/slider_block.rs`, `slider.rs`, `range_slider.rs`),
  and the GPUI host (`block_slider_host.rs`).
- g18.023 was not touched; no release, version, changelog, workflow, Desktop,
  or unrelated-component files changed.

## Validation

- `bun test packages/core` — core suite green, including new serializer
  vectors (fractional steps, fractional minima, integer steps, trailing-zero
  trimming, negative zero, no-step passthrough, custom-formatter precedence).
- `cargo test -p poodle-headless` — 224+ green including the mirrored Rust
  serializer vectors; `cargo test -p poodle-specs` — 330 green;
  `cargo test -p poodle-jetstream` — 163 green.
- `cargo test -p poodle-render` — 645 passed with only the two pre-existing
  unrelated failures (context a11y-wrapper, segmented-control icon-only)
  verified identical on the base commit.
- `effigy regressions:native` — `headless_regressions` 238/0 plus the other
  three gpui-preview test targets (catalogue 7, icon geometry 6, visual
  fixture inventory 15) and `poodle-gpui-node-backend` 52/0.
- Native: `effigy regressions:native` — `headless_regressions` 241/0 at the
  native-vertical head (238 pre-existing plus the three new vertical/horizontal
  block geometry regressions), and the other
  three gpui-preview test targets (catalogue 7, icon geometry 6, visual
  fixture inventory 15) and `poodle-gpui-node-backend` 52/0.
- Block-slider hit probe 124/0 and inline probe 96/0 per engine on headless
  Chromium and WebKit at the round-1 head; the web half is unchanged in round
  2 and was independently verified at 248/0 and 192/0 across both engines.
- Full vitest board — 415 files / 4134 tests green (svelte-components,
  react-components, previews, parity, a11y, headless-dom).
- `svelte-check` components — 0 errors; preview back at its pre-existing
  232-error baseline with zero new errors; react `tsc` shows no slider errors
  (pre-existing unrelated errors unchanged, one fewer than base).
- Package builds green: core, svelte components, react components, svelte
  preview, react preview.
- Visual smoke board: slider pairs match; the only failure (`pill` at
  0.665%) reproduces identically on the base commit and is environmental.
- Census/evidence hygiene: source changes move the receipts' SOURCE_PATHS, so
  the Nucleus/GPUI census evidence was repinned — first to the round-1 source
  head, then to the native-vertical head (`00d90ba64`) following the g18.022
  precedent (receipts differ only in `source_commit`/run id;
  `effigy regressions:native` re-run at each head; no observation or capture
  change; admitted capabilities identical: 73 admitted rows, 65 mounted
  receipts).
- `git diff --check` — clean.

## Explicitly not done

- g18.023, release state, versions, changelog, workflows, Desktop, and
  unrelated components untouched.
- TextInput draws its own 1px border outside the shared ladder (its total box
  is 2px taller at every size); the probe asserts alignment within that
  TextInput-owned border. Fixing TextInput's box model is outside this task.
- The `pill` visual-smoke delta is pre-existing/environmental (reproduces on
  the base commit) and stays out of scope.
- The exported core helper signature change
  (`resolveSliderVisibleValue`/`resolveRangeVisibleValue`/
  `defaultVisibleValueText` and the Rust `resolved_visible_text` now carry
  `min`/`step`) is deliberate: the precision law needs them, the web
  `formatVisibleValue` prop and native explicit-string channels are
  unchanged, and pre-v1 rules forbid compatibility shims. The reviewer
  classified it non-blocking.

## Continuation

After both repairs merge, the retained g18.006 release-candidate task resumes
with operator acceptance; g18.009 stays serial behind it.
