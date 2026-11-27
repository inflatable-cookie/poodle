# 198: hover-only fill, before and after

Before and after captures of the Svelte specimens for poodle#198. Each image is
the `before` panel on the left and the `after` panel on the right, cropped to the
top 520 px of the specimen. `eclipse` is the dark theme and `iceberg` is the
light theme.

- **Before:** `main`'s `packages/core/src/styles`, built into `packages/core/dist`.
- **After:** this branch's styles, built the same way.
- **Method:** the visual harness's specimen capture (`test/visual/capture.ts`),
  pinned clock, frozen transitions, default compact/md axis. Hover and open
  states are driven with Playwright. `metrics-*.json` holds computed
  backgrounds and specimen boxes. `diff-report.json` holds pixel diffs at
  threshold 0.

## Classification

| Specimen | Change | Light / dark specimen pixels changed |
|---|---|---|
| breadcrumbs (hover) | links rest transparent, hover fills | 11,453 / 11,246 |
| sidebar-nav (hover) | items rest transparent, hover fills | 96,341 / 95,840 |
| menu (open, hover) | items rest transparent, hover fills; trigger unchanged (rest 0) | 24,005 / 23,909 |
| select (open) | options rest transparent; trigger unchanged (rest 0) | 24,091 / 23,845 |
| tree (hover) | rows rest transparent, hover fills | 833,692 / 831,557 |
| tabs (hover) | non-card tab buttons rest transparent, hover fills | 74,322 / 73,504 |
| data-table (toolbar hover) | toolbar buttons rest transparent, hover fills | 5,416 / 5,372 |
| text-input (unchanged control) | none | 0 / 0 |
| button (unchanged control) | none | 0 / 0 |

Every changed pixel sits inside the changed control family. The unchanged
controls are exact.

Every capture also differs in a 195 × 24 px strip at the top-left, at y 24–47.
That strip is the preview chrome's section tab row (`Examples`, `Sizes`,
`Densities`). It is a tab family that rests transparent after this change.
This is the intended change, and it is outside each specimen.

## Layout

The specimen box is the same width and height before and after in all 18
pairs (`diff-report.json`, `*-layout` entries). No layout shift.

## Checks

- `effigy visual:report` compares Svelte with React, not before with after.
  It is recorded separately and does not measure this change.
- Remove this folder before merge if the planner prefers evidence to live only
  on the PR. The rendering-style README points here.
