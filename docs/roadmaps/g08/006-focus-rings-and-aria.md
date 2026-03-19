# g08.006 Focus Rings And ARIA Attributes

Status: planned
Owner: Pug Core
Depends on: g08.003, g08.004, g08.005

## Contract Check

Before starting, verify the current focus ring token (`semantic.color.accent.focusRing`
or equivalent) and the ARIA requirements for each interactive component in the
contracts.

## Goals

All interactive GPUI components currently lack focus rings and most lack ARIA
attributes. This milestone adds both across all interactive components in a
single pass, ensuring consistency.

## Execution Checklist

### Focus Rings

- [ ] Establish a shared focus ring pattern (e.g., `outline` or `box_shadow`
      using the accent focus ring token)
- [ ] Apply to all interactive components: button, icon_button, checkbox,
      switch, select, tabs, text_input, text_area, number_entry, radio_group,
      slider, segmented_control, pin_input, toggle, split_button, etc.
- [ ] Verify focus ring only appears on keyboard focus (`:focus-visible`
      equivalent), not mouse click

### ARIA Attributes

- [ ] For each interactive component, read its contract's Accessibility section
- [ ] Apply required `role` attributes
- [ ] Apply `aria-label`, `aria-expanded`, `aria-checked`, `aria-selected`,
      `aria-disabled`, `aria-busy` as specified
- [ ] Apply keyboard behavior where contract requires it

## Acceptance Criteria

- [ ] Every interactive component shows a focus ring on keyboard focus
- [ ] Focus ring color resolves from token
- [ ] ARIA attributes match contract requirements for all interactive components
- [ ] All changes compile and render correctly
