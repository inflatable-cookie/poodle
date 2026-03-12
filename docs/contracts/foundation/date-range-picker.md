# Date Range Picker

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `DateRangePicker`
- Layer: `foundation`
- Summary: a range value control that combines a trigger with calendar-based
  bounded range selection
- In scope: range display, open state, selected start and end dates,
  range-calendar overlay
- Out of scope: recurring windows, time ranges, report presets, availability
  logic

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  │     ├── [Selected Range or Placeholder]
  │     └── [Disclosure Indicator]
  └── [Range Calendar Overlay]
```

## 3. Props And Inputs

- `value`: `{ start: string | null; end: string | null } | null`
- `defaultValue`: `{ start: string | null; end: string | null }`
- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `placeholder`: `string`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- placeholder
- start selected
- complete range selected
- open
- disabled

## 5. Events

- `onValueChange`
- `onOpenChange`

## 6. Accessibility

- role: trigger button with dialog-like popup ownership
- required semantics: expanded state, selected-range exposure, trigger-to-popup
  relationship, range-calendar keyboard support
- keyboard: enter or space toggles, escape dismisses, range-calendar keyboard
  rules stay active while open

## 7. Layout

- trigger follows shared control sizing
- overlay anchors to the trigger unless later placement rules expand

## 8. Token Usage

- control background, border, focus, text, range-highlight, and overlay surface
  roles

## 9. Svelte Notes

- public value uses `{ start, end }` ISO-date objects
- implementation may keep the popup open after start selection until the end of
  the range is committed

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::date_range_picker`

## 11. Parity Checklist

- [ ] trigger and open-state semantics match
- [ ] range commit semantics match
- [ ] popup range-calendar behavior matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| interim range-preview phrasing may differ | helper copy is implementation-owned | allowed | keep committed start/end semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: reporting filters, review windows, bounded search forms

## Next Task

Use `DateRangePicker` for bounded date values, and keep preset-driven or
domain-specific date-range workflows in composites.
