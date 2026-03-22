# g11.005 Buttons Batch

Status: planned
Owner: Pug Core
Depends on: contract audit

## Components

button, icon_button, split_button

## Structural Issues

None — all three have contracts, Rust specs, Svelte, and GPUI implementations.

## Known Issues (from g08 audit)

- [ ] Button: hover/active uses color-mix blending (contract §8) — verify
      GPUI matches exactly
- [ ] Button: danger×secondary fill/border use color-mix formulas — verify
- [ ] Button: primary border darkened with color-mix(accent 84%, black) — verify
- [ ] Button: chevron opacity 0.5 and margin-left -2px — verify
- [ ] Button: spinner uses SVG with animation — verify GPUI equivalent
- [ ] Button: typography uses FontWeight::MEDIUM (500) — verify
- [ ] IconButton: verify icon sizing matches contract per ControlSize
- [ ] SplitButton: verify separator, dropdown trigger anatomy

## Per-Component Compliance

- [ ] button — audit against `docs/contracts/foundation/button.md`
- [ ] icon_button — audit against `docs/contracts/foundation/icon-button.md`
- [ ] split_button — audit against `docs/contracts/foundation/split-button.md`
