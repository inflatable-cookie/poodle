# Solid Tone Visual Tuning

Date: 2026-08-19
Scope: direct post-merge correction to `g15.035` and `g15.036`

## Finding

Live review exposed a palette inversion in the first solid-tone recipe. Mixing
status tones toward `color.text.primary` made dark themes pale and washed out,
while light themes moved toward near-black fills. Passing contrast checks did
not make that visual direction sound.

## Correction

- Non-neutral backgrounds now promote the tint-border colour into the fill:
  34% tone with `color.border.default`, or 24% for pending/custom accent.
- Neutral backgrounds mix `color.text.secondary` and
  `color.background.surface` equally.
- Content, icons, pending spinners, and Pill remove affordances use
  `color.text.primary`.
- Fill and border use the same resolved colour as one continuous surface.
- Callout, RemediationBanner, and Pill use the same recipe in shared CSS and
  renderer-neutral Rust.

Dark themes keep moderately dark coloured surfaces; light themes keep
moderately light ones. No stronger outline separates border from fill.

## Evidence

The renderer test covers all twelve themes plus neutral, info, success,
warning, danger, and accent bases. It evaluates translucent tint-border fills
over the panel token. The lowest measured normal-text contrast is 4.528:1,
above the 4.5:1 floor.

React preview inspection covered Callout, RemediationBanner, and Pill in
Eclipse and Iceberg. Eclipse retained white primary text on darkened tone
surfaces; Iceberg retained dark primary text on pale tone surfaces. Live theme
switching resolved the recipe without component-local theme branches.

## Validation

| Command | Outcome |
|---------|---------|
| `effigy test:core` | 765 tests pass |
| `effigy test:components` | 347 files, 2828 tests pass |
| `effigy test:parity` | 6 files, 365 tests pass |
| `effigy ir:build` / `effigy ir:check` | generated specimens current |
| `effigy check:svelte` | 0 errors; baseline warnings only |
| `effigy react:build` | pass |
| `effigy check:gpui` | render 335 and node backend 19 tests pass |
| `effigy docs:check` | pass |
