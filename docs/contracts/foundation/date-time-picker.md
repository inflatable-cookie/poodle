# Date Time Picker

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `DateTimePicker`
- Layer: `foundation`
- Summary: a value control that combines date selection and time entry in one
  popup-owned surface
- In scope: selected date, selected time, open state, calendar plus time-field
  composition
- Out of scope: timezone selection, recurring schedules, booking availability,
  range workflows

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  │     ├── [Selected Value or Placeholder]
  │     └── [Disclosure Indicator]
  └── [Popup Surface]
        ├── [Calendar]
        └── [Time Field]
```

## 3. Props And Inputs

- `value`: `{ date: string | null; time: string | null } | null`
- `defaultValue`: `{ date: string | null; time: string | null }`
- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `placeholder`: `string`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- placeholder
- partial value
- complete date-and-time value
- open
- disabled

## 5. Events

- `onValueChange`
- `onOpenChange`

## 6. Accessibility

- role: trigger button with dialog-like popup ownership
- required semantics: expanded state, trigger-to-popup relationship, selected
  date and time exposure, calendar and time-field accessibility
- keyboard: enter or space toggles, escape dismisses, calendar keyboard rules
  and time-field editing both remain reachable while open

## 7. Layout

- trigger follows shared control sizing
- popup owns the calendar and time-entry stack in this baseline

## 8. Token Usage

- control background, border, text, focus, accent, and overlay surface roles

## 9. Svelte Notes

- public value uses `{ date, time }` rather than `Date` instances
- partial values are allowed during editing without forcing a hidden timezone
  or combined timestamp contract

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::date_time_picker`

## 11. Parity Checklist

- [ ] trigger and open-state semantics match
- [ ] date and time value semantics match
- [ ] popup composition and focus behavior match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact native time-entry affordances may differ | platform time controls differ | allowed | keep public value and popup semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: publishing forms, reminders, appointments, scheduler
  setup flows

## Next Task

Use `DateTimePicker` for combined date-and-time values, and keep timezone-aware
or domain scheduling workflows in higher layers until they are explicit.
