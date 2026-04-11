# g08.006 Input And Selection Component Fixes (Batch 2)

Status: complete
Owner: Poodle Core
Depends on: g08.004

## Contract Check

Before starting each component, read its contract end to end. If the contract
has changed, update the implementation to match before proceeding.

## Components

### text_area

- [ ] Verify contract: `docs/contracts/components/text-area.md`
- [ ] Resolve padding, radius, row height from spec tokens

### number_entry

- [ ] Verify contract: `docs/contracts/components/number-entry.md`
- [ ] Resolve height, radius from spec tokens
- [ ] Wire stepper button click handlers

### radio_group

- [ ] Verify contract: `docs/contracts/components/radio-group.md`
- [ ] Resolve circle size, inner dot size, gap from spec tokens
- [ ] Wire per-option on-change handlers

### slider

- [ ] Verify contract: `docs/contracts/components/slider.md`
- [ ] Resolve track height, thumb size from spec tokens
- [ ] Position thumb on track at normalized value offset
- [ ] Render filled portion of track

### segmented_control

- [ ] Verify contract: `docs/contracts/components/segmented-control.md`
- [ ] Resolve outer/inner radius from spec tokens
- [ ] Replace hardcoded hover color with `color_mix`

### pin_input

- [ ] Verify contract: `docs/contracts/components/pin-input.md`
- [ ] Resolve cell dimensions, radius, gap from spec tokens

## Acceptance Criteria

- [ ] All 6 components verified against current contracts
- [ ] All visual values resolve from tokens
- [ ] Slider thumb positioned correctly on track
- [ ] Number entry steppers functional
- [ ] Components compile and render correctly
