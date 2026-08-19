# Solid Tone Visual Tuning

Date: 2026-08-19
Scope: direct post-merge correction to `g15.035` and `g15.036`

## Finding

Live review exposed a palette inversion in the first solid-tone recipe. Mixing
status tones toward `color.text.primary` made dark themes pale and washed out,
while light themes moved toward near-black fills. Passing contrast checks did
not make that visual direction sound.

## Correction

- Non-neutral backgrounds now mix 40% tone with 60%
  `color.background.surface` in sRGB.
- Neutral backgrounds mix `color.text.secondary` and
  `color.background.surface` equally.
- Content, icons, pending spinners, and Pill remove affordances use
  `color.text.primary`.
- Tone and neutral border rules stay unchanged.
- Callout, RemediationBanner, and Pill use the same recipe in shared CSS and
  renderer-neutral Rust.

The recipe follows the theme instead of inverting it: dark themes keep
moderately dark, saturated surfaces; light themes keep moderately light,
muted surfaces.

## Evidence

The renderer test covers all twelve themes plus neutral, info, success,
warning, danger, and accent bases. The lowest measured normal-text contrast is
4.783:1 for a toned surface and 4.860:1 for a neutral surface, both above the
4.5:1 floor.

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

