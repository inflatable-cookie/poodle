# Range Calendar

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `RangeCalendar`
- Layer: `foundation`
- Summary: a visible month grid for selecting a bounded date range
- In scope: range start and end selection, in-range display, month navigation,
  week start policy
- Out of scope: time ranges, recurring windows, booking-specific availability
  logic

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

- `value`: `{ start: string | null; end: string | null } | null`
- `defaultValue`: `{ start: string | null; end: string | null }`
- `visibleMonth`: `string | null`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- no range selected
- start selected
- complete range selected
- in-range date
- disabled

## 5. Events

- `onValueChange`
- `onMonthChange`

## 6. Accessibility

- role: grid with row and gridcell semantics plus interactive day buttons
- required semantics: visible start and end state, in-range treatment, month
  label, predictable focus movement
- keyboard: arrow navigation, home/end within week, page-up/page-down across
  months, enter/space select

## 7. Layout

- the range calendar owns one month view in this baseline
- parents decide whether multiple months, compare views, or helper copy belong
  in a higher-order shell

## 8. Token Usage

- surface, text, accent, range-highlight, border, and focus roles

## 9. Svelte Notes

- public value uses `{ start, end }` ISO-date objects rather than `Date`
  instances
- implementation may normalize start and end ordering internally

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::range_calendar`

## 11. Parity Checklist

- [ ] start/end and in-range semantics match
- [ ] keyboard movement and month navigation match
- [ ] value normalization remains equivalent

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact range-preview visuals may differ | render polish is implementation-specific | allowed | keep start, end, and in-range semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: report filters, booking windows, review spans

## Next Task

Use `RangeCalendar` for bounded date-range selection, and keep richer booking
or availability logic outside the foundation layer.
