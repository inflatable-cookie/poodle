# Calendar

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Calendar`
- Layer: `foundation`
- Summary: a visible month grid for selecting one date value
- In scope: month navigation, day-grid semantics, single-date selection, week
  start policy
- Out of scope: time selection, recurrence, timezone handling, scheduling
  workflows

## 2. Anatomy

```text
[Root]
  ├── [Header]
  │     ├── [Previous Month]
  │     ├── [Month Label]
  │     └── [Next Month]
  ├── [Weekday Row]
  └── [Date Grid]
        └── [Day Button...]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `visibleMonth`: `string | null`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- idle
- selected day
- today
- outside visible month
- disabled

## 5. Events

- `onValueChange`
- `onMonthChange`

## 6. Accessibility

- role: grid with row and gridcell semantics plus interactive day buttons
- required semantics: selected day, visible month label, button labels for each
  date, predictable focus movement
- keyboard: arrow navigation, home/end within week, page-up/page-down across
  months, enter/space select

## 7. Layout

- calendar owns its own month header, weekday row, and day grid
- parent owns placement, sizing, and whether the calendar lives inline or in
  an overlay shell

## 8. Token Usage

- surface, border, text, accent, focus, and overlay-adjacent spacing roles

## 9. Svelte Notes

- public value uses ISO `YYYY-MM-DD` strings rather than browser `Date`
  instances
- month-grid generation may stay implementation-owned as long as keyboard and
  selected-date semantics stay stable

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::calendar`

## 11. Parity Checklist

- [ ] selected-day semantics match
- [ ] month navigation and keyboard movement match
- [ ] day-grid accessibility semantics match

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| locale formatting details may differ slightly | platform date-format internals differ | allowed | keep value and navigation semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: inline schedulers, filter calendars, date pickers

## Next Task

Use `Calendar` for visible single-date selection, and reserve higher-level
workflow or scheduling semantics for composites until they are explicit.
