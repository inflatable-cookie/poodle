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
- [x] Wave 3b: TextInput (full pipeline: debounced change, async validator
  with key-guarded races, validation-context re-runs, slug mode with source
  auto-generation, multiline, char count, clear, affordances), NumberInput
  (draft/commit editing, arrow stepping with snap+clamp, steppers, async
  validation), DurationInput (segment machinery), TokenInput (split/merge/
  backspace token machinery). Verified: typing, slug autogen from source,
  invalid->valid validation message lifecycle, arrow step + max clamp,
  duration segment arrows, comma-commit + backspace-remove — 8/8.
- [x] Wave 3c: Popover (popover machine + registerDismissLayer + initial
  focus via getFocusableElements; parts records adapted through a
  reactifyPart helper for tabindex/class), Field (control render prop,
  described-by wiring, info popover, nested UiPresentationProvider),
  CodeInput (code machinery: sanitize/insert-replacement/slot-selection
  with beforeinput interception). Verified: popover toggle + initial focus
  + Escape and outside dismiss through the shared dismiss-layer stack,
  field error message, 6-digit entry with onComplete + filled slots.
- [x] Wave 4a: Select (custom + searchable + native modes, grouped options,
  lazy loadOptions with request-id races, dismiss layer, menu placement via
  selectMenuPlacement, trigger/option/empty render props), FieldSet,
  PasswordRequirements. Caught in verification: dropped Svelte's isGrouped
  guard around filterSelectGroups — crashed on flat searchable options;
  restored. Verified 7/7: click select, keyboard open/Escape, disabled
  options filtered from flat lists (Svelte parity), search filter + Enter,
  native <select> mode, password rule transitions, fieldset grid.
- [ ] Wave 4b: OrderBy (needs Select — now available), FormActions (needs
  Menu — overlay batch)

## Notes

- React-specific trap documented: never position overlays from an inline
  ref callback — React re-invokes it every render and the setState loops
  (caught live by "Maximum update depth exceeded" on IconButton; fixed with
  useLayoutEffect + change-guarded setState).
