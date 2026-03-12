# Pug Svelte Primitives

Contract-backed Svelte primitive surface for the first generalized Pug baseline.

## Public Surface

- `Accordion`
- `Banner`
- `Badge`
- `Box`
- `Button`
- `Callout`
- `Calendar`
- `Checkbox`
- `Combobox`
- `Collapsible`
- `ContextMenu`
- `DatePicker`
- `DateRangePicker`
- `DateTimePicker`
- `DateTimeRangePicker`
- `Dialog`
- `Drawer`
- `EditableLabel`
- `Field`
- `TextInput`
- `TextArea`
- `TimeField`
- `TimeZoneSelect`
- `SearchField`
- `FormActions`
- `Grid`
- `HoverCard`
- `IconButton`
- `Inline`
- `Meter`
- `Menubar`
- `Menu`
- `NavigationMenu`
- `NumberEntry`
- `Pagination`
- `Pill`
- `PinInput`
- `Popover`
- `Progress`
- `RadioGroup`
- `Rating`
- `RangeCalendar`
- `RangeSlider`
- `ScrollShell`
- `SegmentedControl`
- `Select`
- `Separator`
- `Skeleton`
- `Slider`
- `Spacer`
- `Stack`
- `StatusIndicator`
- `Surface`
- `Switch`
- `Table`
- `Tabs`
- `TabStrip`
- `Toggle`
- `ToggleGroup`
- `Toolbar`
- `Tooltip`
- `TriStateSwitch`
- `ZonedDateTimePicker`
- root import: `@pug/svelte-primitives`
- type-only import: `@pug/svelte-primitives/types`

## Purpose

- ship the standard action, text-entry, selection, and feedback primitives most
  apps need before pushing more depth into compound components
- ship the structural layout and surface primitives that composites should be
  built on rather than leaving composition to ad hoc container CSS
- ship the documented overlay, menu, and navigation primitives so the
  foundation layer is complete enough to stop hiding missing primitives behind
  composites or workstation shells
- add disclosure primitives that are especially useful in more web-oriented
  docs, settings, and marketing-adjacent surfaces without pushing them into
  higher layers by accident
- start widening the primitive catalogue beyond the original foundation baseline
  where the broader substrate maps cleanly to generalized app needs
- add a coherent date-selection baseline rather than leaving date ownership
  implicit or pushing all date controls into composites prematurely
- extend that baseline just far enough to cover general local time entry and
  combined date-plus-time values without silently promoting full scheduling
  ownership
- extend the same local-value contract to bounded date-plus-time ranges without
  promoting timezone or scheduling workflow semantics into foundation
- add the smallest timezone-aware layer only where explicit timezone selection
  and local-plus-zone entry are generalized enough to stand on their own
- widen the utility layer where the ownership line is still clearly foundation
  safe, while leaving app or shell navigation semantics out of the primitive
  package
- promote the smallest generalized navigation family only after splitting
  persistent navigation disclosure from persistent command-menu ownership
- promote only the low-level data surfaces that are truly foundation-safe while
  keeping command discovery and richer table behavior above foundation
- keep the public API contract-owned by Pug even where Bits-aligned headless
  primitives remain the expected Svelte-side substrate
- give the preview app and downstream adopters a reusable primitive package
  instead of relying on inline demo controls or composite-only entry points

## Stability Notes

- public entry points are the package root and `./types`
- preview-specific helpers and demo glue do not belong in this package
- additions should follow documented contract coverage and the explicit
  primitive baseline, not demo convenience
- the documented foundation contract set now has a corresponding Svelte package
  surface, and wider utility, date, and time-aware tranches are now explicit
- deeper command, data-table, scheduling, and other broader substrate families
  still need explicit ownership decisions before they should be added
- parity maturity is still incomplete outside the shipped Svelte surface and
  GPUI still trails this package materially

## Next Task

Use this package surface while following the direct-consumer onboarding lane,
keeping reference-app guidance contract-backed and resisting deep-import or
template-style usage that bypasses the documented foundation surface.
