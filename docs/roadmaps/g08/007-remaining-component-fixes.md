# g08.007 Remaining Component Fixes (Batch 3)

Status: complete
Owner: Poodle Core
Depends on: g08.004

## Contract Check

Before starting each component, read its contract end to end. If the contract
has changed, update the implementation to match before proceeding.

## Components

### time_field

- [ ] Verify contract: `docs/contracts/components/time-field.md`
- [ ] Resolve height, padding, radius from spec tokens
- [ ] Replace clock emoji with `PoodleIcon`

### duration_input

- [ ] Verify contract: `docs/contracts/components/duration-input.md`
- [ ] Resolve height, radius from spec tokens

### tri_state_switch

- [x] Verify contract: `docs/contracts/components/tri-state-switch.md`
- [x] Resolve track/thumb dimensions from spec tokens
- [x] Correctly center thumb in mixed state

### rating

- [ ] Verify contract: `docs/contracts/components/rating.md`
- [ ] Replace `"*"` text with `PoodleIcon` star icons
- [ ] Resolve star size from spec token

### tooltip

- [ ] Verify contract: `docs/contracts/components/tooltip.md`
- [ ] Resolve padding, radius from spec tokens

### drawer

- [ ] Verify contract: `docs/contracts/components/drawer.md`
- [ ] Remove hardcoded width/height, use content-driven sizing

### color_picker (broken)

- [ ] Verify contract: `docs/contracts/components/color-picker.md`
- [ ] Fix swatch rendering — apply color as background (currently discarded)
- [ ] Parse hex strings into GPUI colors
- [ ] Update preview to reflect selected color

### range_slider (broken)

- [ ] Verify contract: `docs/contracts/components/range-slider.md`
- [ ] Render filled range segment between thumbs
- [ ] Position both thumbs on track at normalized offsets
- [ ] Use computed `norm_low`/`norm_high` values (currently discarded)
- [ ] Resolve track/thumb dimensions from spec tokens

## Acceptance Criteria

- [ ] All 8 components verified against current contracts
- [ ] All visual values resolve from tokens
- [ ] `color_picker` renders actual colored swatches
- [ ] `range_slider` renders positioned thumbs and filled range
- [ ] All emoji/text placeholders replaced with `PoodleIcon`
- [ ] Components compile and render correctly
