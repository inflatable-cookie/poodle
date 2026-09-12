# g18.026 — Slider foundation and RangeSlider parity

Status: implementation complete — PR open for exact-head independent review
Date: 2026-09-12
Branch: `ns-6de406c0-1396-4a02-9e18-3c18e71ada27`
Card: `docs/roadmaps/g18/026-slider-foundation-and-range-parity.md`
Handoff: `docs/handoffs/20260912-g18-026-slider-foundation-and-range-parity.md`
Governing refs: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`docs/architecture/012-feedback-motion-and-state-change.md`
Base: `origin/main` at `1782067cd182c61ff8254e5a753e481841c39d69`

## Outcome

`Slider` and `RangeSlider` now compose one private Slider-family rendering
foundation while keeping separate public components, value types, callbacks,
focus models and ARIA.

- `packages/core/src/styles/slider-family.css` is the one canonical block
  foundation: shared control/block size ladders, capsule, track, fill
  appearance, center marker, inline text scaffolding, layout-neutral 44×44
  targets, the bounded inset line handle and the shared forced-colour/focus
  handle table. Both public sheets `@import` it and keep only genuine
  single/range modifiers.
- `packages/core/src/slider-family.ts` is the framework-free web foundation:
  capsule-span measurement, pointer-to-value mapping, the horizontal collision
  docking law and the marker geometry. Both Svelte and React shells call it; the
  four copies of `pointNorm`/ResizeObserver/docking arithmetic are gone.
- RangeSlider receives every accepted post-g18.024 Slider repair: the inset
  line handle composed twice, bounded handle positions, endpoint text painted
  above the handle, and the shared forced-colour/focus handle table. Its window
  fill remains anchored at the pair (only the handles are clamped) so a pair at
  equality or overlap can never cross its sibling.
- The dead `.poodle-*-__control` embedded-input rules (removed from the
  components in the g18.022 block-first redesign, still carried as CSS) are gone
  from the range sheet; the tests already assert those nodes are absent.

Public API is unchanged: separate scalar and pair components, no union,
inheritance, alias or compatibility wrapper.

## Execution

- Planted the paired laws first: core foundation vectors
  (`slider-family.test.ts`), a source-bound foundation test
  (`slider-family-source.test.ts`) that fails if either component sheet
  re-implements the shared capsule/handle/hit/ladder, and paired Svelte/React
  RangeSlider assertions for the shared handle, clamp, text-above-handle and
  ladder. Rewrote the stale inline-probe expectations that predated the
  accepted docking (g18.026) and fixed-width decimals (g18.024) and added
  RangeSlider handle/window/extrema/equality/vertical geometry to the paired
  browser probe.
- Extracted the foundation, then moved the shared rules out of both component
  sheets. Density stays embedded-only; the block capsule consumes the shared
  control-height ladder through `--poodle-slider-family-block-height`.
- Ported the handle and text paint order to RangeSlider across CSS, Svelte and
  React, and confirmed the same foundation drives both families in both
  frameworks.

## Validation

- `bun test packages/core` — 1394 green (was 1378; +16 foundation/source
  vectors).
- Full vitest board — 417 files / 4146 tests green plus 8 skipped (was 415 /
  4134 at base): svelte-components, react-components, previews, parity, a11y
  and headless-dom, including the new paired RangeSlider foundation describes.
- Paired Chromium + WebKit block probes: `test:block-slider-hit` 124/0 and
  `test:block-slider-inline` 120/0 per engine, including the new RangeSlider
  handle/window/text-above/extrema/equality/vertical geometry.
- Native verification: `effigy regressions:native` — `headless_regressions`
  242/0 at this head. `effigy check:gpui-census` and
  `effigy check:parity-evidence-ledger` resolve clean because no file under the
  receipts' `SOURCE_PATHS` changed.
- `effigy test:core-build` — 66/0 including the updated CSS inventory and
  packed-manifest checks.
- `effigy svelte:package` and `effigy react:package` — green (core, Svelte and
  React dist rebuilt before the docs audits).
- Docs QA: `docs:lint`, `docs:contract-drift`, `docs:react-prop-drift`,
  `docs:callback-drift`, `docs:container-query-drift`, `docs:react-specimen-drift`,
  `docs:capability-drift`, `docs:spec-drift`, `docs:focus-ring-drift`,
  `docs:value-domain-drift` — green.
- `svelte-check` components — 0 errors (2 files, 4 pre-existing warnings).
  React `tsc` — only pre-existing unrelated errors, none in Slider/RangeSlider.
- `git diff --check` — clean.

## Explicitly not done

- No public API change. No union component, inheritance, alias or wrapper.
- No release, version, lockfile, changelog, workflow, Desktop, Jetstream,
  g18.027 or retained g18.006/g18.009 state was touched.
- No windowed selector was run.
- The GPUI block-handle realization still paints the shared circular thumb for
  both families rather than the web bounded inset marker line. The native
  composition is already shared (`slider_block::visible_thumb` / `block_hit` /
  `fraction_anchor`) and the native board is green, so this is recorded as a
  documented pre-v1 divergence in both contracts' Known Deltas tables rather
  than forced churn on the accepted g18.024 receipts. `bipolar` native block
  fill is likewise covered by the shared composition and remains part of the
  web/native visual-parity closure.
- The contracts' older embedded-variant prose (native `<input>` anatomy and
  `::-webkit-slider-thumb` sections) predates the g18.022 block-first redesign
  and is left untouched beyond the block handle law; it is not this task's
  budget and g18.027's freeze audit owns the surface inventory.

## Review and merge

Not yet reviewed. One non-draft PR from the queue branch against `main`;
queue-owned review and merge/closeout follow.

## Continuation

After merge and operator acceptance, run g18.027 against the resulting main
head. Retained g18.006 stays blocked until that audit closes with every public
break classified.
