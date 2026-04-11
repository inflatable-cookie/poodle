# g08.005 High-Visibility Component Fixes (Batch 1)

Status: complete
Owner: Poodle Core
Depends on: g08.004

## Contract Check

Before starting each component, read its contract in `docs/contracts/` end to
end. Verify component name, props, anatomy, token targets, and states match
what the contract currently says. If the contract has changed since the last
session, update the implementation to match before proceeding.

## Components

### icon_button

- [ ] Verify contract: `docs/contracts/components/icon-button.md`
- [ ] Replace raw text icon rendering with `PoodleIcon` (SVG icons)
- [ ] Replace `.opacity()` hover/active with `color_mix` pattern
- [ ] Resolve radius from spec token
- [ ] Support all variants: default, ghost, danger

### checkbox

- [ ] Verify contract: `docs/contracts/components/checkbox.md`
- [ ] Resolve indicator size from spec token instead of `px(18.0)`
- [ ] Resolve gap from spec token instead of `px(8.0)`
- [ ] Resolve corner radius from spec token
- [ ] Replace hardcoded hover color with token-derived value

### switch

- [ ] Verify contract: `docs/contracts/components/switch.md`
- [ ] Resolve track dimensions from spec tokens
- [ ] Resolve knob size and offset positions from tokens
- [ ] Resolve gap from spec token

### text_input

- [ ] Verify contract: `docs/contracts/components/text-input.md`
- [ ] Resolve height, padding, radius, gap from spec tokens
- [ ] Support leading/trailing icon slots via `PoodleIcon`

### select

- [ ] Verify contract: `docs/contracts/components/select.md`
- [ ] Resolve height, padding, radius from spec tokens
- [ ] Replace hardcoded hover color with `color_mix`
- [ ] Verify chevron icons render via `PoodleIcon`

### tabs

- [ ] Verify contract: `docs/contracts/components/tabs.md`
- [ ] Resolve all padding, font size, gap from spec tokens
- [ ] Replace hardcoded hover color with `color_mix`
- [ ] Implement Card variant if contract requires it

## Acceptance Criteria

- [ ] All 6 components verified against current contracts
- [ ] All visual values resolve from tokens — zero hardcoded px
- [ ] All icon slots use `PoodleIcon`
- [ ] Hover/active states use `color_mix`
- [ ] Components compile and render correctly
