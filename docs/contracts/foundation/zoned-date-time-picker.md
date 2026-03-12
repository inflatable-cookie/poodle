# Zoned Date Time Picker

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `ZonedDateTimePicker`
- Layer: `foundation`
- Summary: a value control that combines date selection, local time entry, and
  timezone selection
- In scope: selected date, local time, timezone, open state, calendar plus time
  and timezone composition
- Out of scope: recurrence, timezone conversion workflows, transport schedules,
  booking rules

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  │     ├── [Selected Value or Placeholder]
  │     └── [Disclosure Indicator]
  └── [Popup Surface]
        ├── [Calendar]
        ├── [Time Field]
        └── [Time Zone Select]
```

## 3. Props And Inputs

- `value`: `{ date: string | null; time: string | null; timeZone: string | null } | null`
- `defaultValue`: `{ date: string | null; time: string | null; timeZone: string | null }`
- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `placeholder`: `string`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `timeZoneOptions`: timezone options
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- placeholder
- partial zoned value
- complete zoned date-time value
- open
- disabled

## 5. Events

- `onValueChange`
- `onOpenChange`

## 6. Accessibility

- role: trigger button with dialog-like popup ownership
- required semantics: expanded state, reachable calendar, time, and timezone
  controls, selected-value exposure, predictable dismissal
- keyboard: enter or space toggles, escape dismisses, all composed controls
  remain reachable while open

## 7. Layout

- trigger follows shared control sizing
- popup owns the calendar, time field, and timezone select stack

## 8. Token Usage

- control background, border, text, focus, accent, and overlay surface roles

## 9. Svelte Notes

- public value uses contract-owned local date, local time, and timezone string
  fields rather than timestamps
- hosts may still provide curated timezone options when product policy requires
  them

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::zoned_date_time_picker`

## 11. Parity Checklist

- [ ] trigger and open-state semantics match
- [ ] date, time, and timezone value semantics match
- [ ] popup composition and dismissal behavior match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact timezone option ordering may differ | runtime registries and host option policies differ | allowed | keep committed timezone value semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: publishing flows, appointments, scheduler setup,
  localized reminders

## Next Task

Use `ZonedDateTimePicker` for explicit local date-time plus timezone values,
and keep timezone conversion or domain scheduling workflows in higher layers.
