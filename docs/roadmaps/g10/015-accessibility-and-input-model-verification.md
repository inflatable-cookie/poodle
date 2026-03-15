# g10.015 — Accessibility and Input Model Verification

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.014
Primary repos: `pug`

## Goals

- [ ] verify Jetstream-specific input handling works correctly for all
  interactive components
- [ ] document accessibility differences from Svelte and GPUI

## Execution Checklist

- [ ] verify keyboard navigation:
  - [ ] Tab key cycles focus between interactive elements
  - [ ] Arrow keys navigate within component groups (radio, menu, tabs)
  - [ ] Enter activates buttons and confirms selections
  - [ ] Escape closes overlays and cancels operations
- [ ] verify gamepad navigation (Jetstream-specific):
  - [ ] D-pad/stick moves focus between elements
  - [ ] A button confirms/activates (maps to Enter)
  - [ ] B button cancels/closes (maps to Escape)
  - [ ] Focus ring is clearly visible on all interactive elements
- [ ] verify `FocusState` management:
  - [ ] focus transfers correctly when opening overlays
  - [ ] focus returns to trigger when closing overlays
  - [ ] modal overlays trap focus within the overlay
  - [ ] tab order is logical within each screen
- [ ] verify focus visualization:
  - [ ] focus ring uses accent color from theme
  - [ ] focus ring is visible against all surface tones
  - [ ] focus ring size is consistent across component types
- [ ] document accessibility differences:
  - [ ] no screen reader support (Jetstream limitation)
  - [ ] no ARIA attributes (not applicable to game engine)
  - [ ] gamepad-only additions (not present in Svelte/GPUI)
- [ ] verify all interactive specimens respond to both keyboard and gamepad

## Acceptance Criteria

- [ ] every interactive component is keyboard-navigable
- [ ] gamepad navigation works for all interactive components
- [ ] focus management works correctly for overlays
- [ ] focus ring is visible and correctly themed
- [ ] accessibility differences are documented

## Next Task

Open `g10.016` and close the generation.
