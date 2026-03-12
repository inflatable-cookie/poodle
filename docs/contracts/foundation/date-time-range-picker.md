# Date Time Range Picker

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `DateTimeRangePicker`
- Layer: `foundation`
- Summary: a bounded range value control that combines date-range selection and
  start/end local time entry
- In scope: selected start and end date values, selected start and end local
  time values, open state, range-calendar plus time-field composition
- Out of scope: timezone selection, recurrence, booking availability, transport
  or schedule workflows

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  │     ├── [Selected Range or Placeholder]
  │     └── [Disclosure Indicator]
  └── [Popup Surface]
        ├── [Range Calendar]
        ├── [Start Time Field]
        └── [End Time Field]
```

## 3. Props And Inputs

- `value`: `{ start: { date: string | null; time: string | null }; end: { date: string | null; time: string | null } } | null`
- `defaultValue`: `{ start: { date: string | null; time: string | null }; end: { date: string | null; time: string | null } }`
- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `placeholder`: `string`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- placeholder
- partial range
- complete date-and-time range
- open
- disabled

## 5. Events

- `onValueChange`
- `onOpenChange`

## 6. Accessibility

- role: trigger button with dialog-like popup ownership
- required semantics: expanded state, selected range exposure, reachable
  range-calendar and time-entry controls, predictable dismissal
- keyboard: enter or space toggles, escape dismisses, range-calendar keyboard
  rules and both time fields remain reachable while open

## 7. Layout

- trigger follows shared control sizing
- popup owns the range calendar plus paired start and end time fields

## 8. Token Usage

- control background, border, text, focus, accent, range-highlight, and
  overlay surface roles

## 9. Svelte Notes

- public value uses nested local-value objects rather than `Date` instances
- partial values are allowed during editing without forcing timezone or
  timestamp normalization into the public contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::date_time_range_picker`

## 11. Parity Checklist

- [ ] trigger and open-state semantics match
- [ ] start/end date and time value semantics match
- [ ] popup composition and dismissal behavior match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact close posture after completion may differ | completion timing is implementation-owned | allowed | keep committed value semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: report filters, booking windows, publishing ranges,
  scheduled review windows

## Next Task

Use `DateTimeRangePicker` for bounded local date-and-time values, and keep
timezone-aware or domain scheduling workflows in higher layers until they are
explicit.
