# Pug GPUI Primitives

Contract-backed GPUI primitive baseline for Pug.

## Public Surface

- crate: `pug-gpui-primitives`
- current structural tranche:
  - `BoxSpec`
  - `StackSpec`
  - `GridSpec`
  - `SurfaceSpec`
  - `SeparatorSpec`
  - `ScrollShellSpec`
- current action, field, and text-entry tranche:
  - `ButtonSpec`
  - `IconButtonSpec`
  - `FieldSpec`
  - `FieldRelationships`
  - `TextInputSpec`
  - `TextAreaSpec`
  - `SearchFieldSpec`
  - `FormActionsSpec`
- current selection, feedback, and date-time tranche:
  - `CheckboxSpec`
  - `RadioGroupSpec`
  - `SwitchSpec`
  - `SelectSpec`
  - `SegmentedControlSpec`
  - `SliderSpec`
  - `ProgressSpec`
  - `BadgeSpec`
  - `StatusIndicatorSpec`
  - `CalendarSpec`
  - `RangeCalendarSpec`
  - `DatePickerSpec`
  - `DateRangePickerSpec`
  - `TimeFieldSpec`
  - `DateTimePickerSpec`
  - `DateTimeRangePickerSpec`
- current overlay, disclosure, navigation, and menu tranche:
  - `AccordionSpec`
  - `CollapsibleSpec`
  - `DialogSpec`
  - `DrawerSpec`
  - `PopoverSpec`
  - `TooltipSpec`
  - `MenuSpec`
  - `ContextMenuSpec`
  - `TabsSpec`
  - `NavigationMenuSpec`
  - `MenubarSpec`
  - `TabStripSpec`
- shared support types:
  - `AccordionItemSpec`
  - `AccordionSelectionValue`
  - `Alignment`
  - `BadgeVariant`
  - `ButtonVariant`
  - `CalendarWeekStart`
  - `CheckState`
  - `ChoiceOption`
  - `ControlSize`
  - `DateRangeValue`
  - `DateTimeRangeValue`
  - `DateTimeValue`
  - `DialogKind`
  - `Dimension`
  - `Direction`
  - `DrawerEdge`
  - `FormActionAlign`
  - `Inset`
  - `MenubarEntry`
  - `MenuEntry`
  - `MenuItemKind`
  - `NavigationMenuEntry`
  - `Orientation`
  - `Overflow`
  - `OverlayPlacement`
  - `PaddingScale`
  - `PopoverInitialFocus`
  - `RuleTone`
  - `SeparatorOrientation`
  - `StatusTone`
  - `SurfaceBorder`
  - `SurfaceRole`
  - `SurfaceTone`
  - `TabActivationMode`
  - `TabDefinition`
  - `TabStripItem`
  - `ValidationState`

## Current Posture

- this crate now carries the `g04.003` structural baseline, the `g04.004`
  action or field or text-entry baseline, the `g04.005` selection or feedback
  or date-time baseline, and the `g04.006` overlay or disclosure or
  navigation or menu baseline
- `g04.010` now makes the native accessibility, focus, keyboard, and
  assistive-technology posture for this primitive surface explicit in
  `packages/gpui/native-accessibility-proof.json`
- it intentionally exposes contract-backed GPUI primitive specs and token
  resolution helpers before the repo contains a full mounted GPUI widget
  runtime for every primitive
- later GPUI tranches should build real native nodes and interaction behavior
  on top of these structural semantics instead of inventing one-off meanings

## Token Dependency

- `pug-gpui-primitives` resolves structural spacing, surface, border,
  elevation, overlay, focus, control-size, and validation tokens from
  `pug-gpui-tokens`
- the crate should stay aligned to emitted token artifacts rather than local
  Rust constants

## Non-Goals

- this crate does not yet prove mounted GPUI rendering parity
- this crate does not yet prove mounted GPUI overlay layering, focus-scope,
  dismissal, submenu, or keyboard traversal behavior
- this crate does not yet expose GPUI composite or workstation-shell families
- this crate does not treat token-aware structural specs as proof that later
  native accessibility mapping is already complete

## Next Task

Use this widened primitive baseline and the explicit native accessibility proof
posture while executing `g04.011`, hardening the cross-runtime parity report,
intentional delta register, and acceptance-harness expansion.
