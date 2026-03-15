# g04.004 Button And Card Pattern Extensions

Status: planned
Owner: Pug Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `pug`

## Goals

- [ ] implement SplitButton as a primitive combining a primary action with a
  dropdown menu of secondary actions
- [ ] implement CardRadioGroup as a composite providing radio-selection across
  rich card options
- [ ] extend Card with specialized layout variants matching Underlay card
  patterns

## Execution Checklist

- [ ] write contract for SplitButton: primary action, dropdown items, disabled
  state, loading state, size variants
- [ ] implement SplitButton primitive in `@pug/svelte-primitives`
- [ ] write contract for CardRadioGroup: card items with title/description/icon,
  single selection, disabled items, controlled value
- [ ] implement CardRadioGroup composite in `@pug/svelte-composites`
- [ ] extend Button contract with split variant support
- [ ] extend Card contract with radio-selection mode and visual-selection state
- [ ] create specimens for SplitButton and CardRadioGroup
- [ ] update existing Button and Card specimens with new variants
- [ ] register new components in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] SplitButton renders primary action and chevron-triggered dropdown
- [ ] SplitButton keyboard navigation works across both zones
- [ ] CardRadioGroup renders selectable cards with visual selection indicator
- [ ] CardRadioGroup supports keyboard navigation and ARIA radiogroup semantics
- [ ] Card supports a `selected` visual state for use in radio contexts
- [ ] all new components pass build and render in the preview catalogue

## Next Task

Open `g04.005` and implement input depth and specialized entry patterns.
