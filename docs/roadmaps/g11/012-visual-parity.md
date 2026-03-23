# g11.012 Visual Parity Verification

Status: planned
Owner: Flint Core
Depends on: g11.003–011

## Actions

- [ ] Run GPUI preview app — screenshot each component specimen
- [ ] Compare against Svelte preview screenshots
- [ ] Document any remaining visual deltas with rationale (GPUI-specific
      limitations like no CSS gradients, no CSS animations)
- [ ] Fix any deltas that are bugs (wrong colors, wrong spacing, missing parts)
- [ ] Create a delta register documenting intentional differences

## Delta Categories

### Expected deltas (GPUI limitations)
- No CSS `box-shadow` inset highlights (button primary has dual shadow in Svelte)
- No CSS `letter-spacing` (typography detail)
- No CSS `transition` (state changes are instant in GPUI)
- Spinner uses SVG rotation vs CSS border-based spinner

### Must-fix deltas (bugs)
- Any wrong color (resolved from wrong token)
- Any wrong dimension (hardcoded px instead of token)
- Any missing anatomy part (contract says it exists, GPUI doesn't render it)
- Any broken state (disabled doesn't reduce opacity, hover doesn't change color)
