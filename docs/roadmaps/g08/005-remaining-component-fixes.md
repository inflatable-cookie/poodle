# g08.005 Remaining Component Fixes (Batch 3)

Status: planned
Owner: Pug Core
Depends on: g08.002

## Contract Check

Before starting each component, read its contract end to end. If the contract
has changed, update the implementation to match before proceeding.

## Components

### time_field

- [ ] Verify contract: `docs/contracts/foundation/time-field.md`
- [ ] Resolve height, padding, radius from spec tokens
- [ ] Replace clock emoji with `PugIcon`

### duration_input

- [ ] Verify contract: `docs/contracts/foundation/duration-input.md`
- [ ] Resolve height, radius from spec tokens

### tri_state_switch

- [ ] Verify contract: `docs/contracts/foundation/tri-state-switch.md`
- [ ] Resolve track/thumb dimensions from spec tokens
- [ ] Correctly center thumb in mixed state

### rating

- [ ] Verify contract: `docs/contracts/foundation/rating.md`
- [ ] Replace `"*"` text with `PugIcon` star icons
- [ ] Resolve star size from spec token

### tooltip

- [ ] Verify contract: `docs/contracts/foundation/tooltip.md`
- [ ] Resolve padding, radius from spec tokens

### drawer

- [ ] Verify contract: `docs/contracts/foundation/drawer.md`
- [ ] Remove hardcoded width/height, use content-driven sizing

### color_picker (broken)

- [ ] Verify contract: `docs/contracts/foundation/color-picker.md`
- [ ] Fix swatch rendering — apply color as background (currently discarded)
- [ ] Parse hex strings into GPUI colors
- [ ] Update preview to reflect selected color

### range_slider (broken)

- [ ] Verify contract: `docs/contracts/foundation/range-slider.md`
- [ ] Render filled range segment between thumbs
- [ ] Position both thumbs on track at normalized offsets
- [ ] Use computed `norm_low`/`norm_high` values (currently discarded)
- [ ] Resolve track/thumb dimensions from spec tokens

## Acceptance Criteria

- [ ] All 8 components verified against current contracts
- [ ] All visual values resolve from tokens
- [ ] `color_picker` renders actual colored swatches
- [ ] `range_slider` renders positioned thumbs and filled range
- [ ] All emoji/text placeholders replaced with `PugIcon`
- [ ] Components compile and render correctly
