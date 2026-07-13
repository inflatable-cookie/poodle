# g12.004 React Batch: Controls And Forms

Status: in progress (2026-07-13)
Owner: Poodle core
Depends on: `g12.003`

## Progress

- [x] Wave 1: IconButton (hover-machine tooltip + overlay positioning),
  Switch, Radio, RadioGroup (switch/single-select machines). Infra:
  `overlay-position.ts` (window-bound wrapper over the core anchor
  resolver). Verified: switch toggle + read-only revert, radio-group select
  + disabled-option inertness, tooltip open/close through `hoverTransition`
  (300ms delay), styling probes.
- [x] Wave 2: SegmentedControl, ToggleGroup (multi + allowDeactivation),
  TriStateSwitch, Slider, RangeSlider, Rating (fractional slider mode +
  whole-star radiogroup mode). Verified: selection machines, keyboard
  stepping on both sliders, rating keyboard step + whole-star roving
  focus/click — 8/8 probes.
- [x] Wave 3a: EditableLabel (edit-label machine: commit/cancel/focus
  effects), TimeInput. Verified: dblclick edit + Enter commit + Escape
  cancel through editLabelTransition; time value change.
- [ ] Wave 3b: TextInput, NumberInput, DurationInput, CodeInput, TokenInput
- [ ] Wave 4: Select, OrderBy, Field, FieldSet, FormActions,
  PasswordRequirements

## Notes

- React-specific trap documented: never position overlays from an inline
  ref callback — React re-invokes it every render and the setState loops
  (caught live by "Maximum update depth exceeded" on IconButton; fixed with
  useLayoutEffect + change-guarded setState).
