# Date Picker

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `DatePicker`
- Layer: `foundation`
- Summary: a date value control that combines a trigger with calendar-based
  single-date selection
- In scope: trigger semantics, open state, selected date display, calendar
  overlay
- Out of scope: time input, free-form locale parsing, recurrence or schedule
  workflows

## 2. Anatomy

```text
[Root]
  ├── [Trigger]
  │     ├── [Selected Value or Placeholder]
  │     └── [Disclosure Indicator]
  └── [Calendar Overlay]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `open`: `boolean | null`
- `defaultOpen`: `boolean`
- `placeholder`: `string`
- `weekStartsOn`: `"sunday" | "monday"`
- `locale`: `string`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- placeholder
- selected date
- open
- disabled

## 5. Events

- `onValueChange`
- `onOpenChange`

## 6. Accessibility

- role: trigger button with dialog-like popup ownership
- required semantics: expanded state, trigger-to-popup relationship, selected
  value exposure, calendar keyboard support
- keyboard: enter or space toggles, escape dismisses, calendar keyboard rules
  stay active while open

## 7. Layout

- trigger follows shared control sizing
- overlay anchors to the trigger unless later placement rules expand

## 8. Token Usage

- control background, border, focus, text, and overlay surface roles

## 9. Svelte Notes

- public selected value uses an ISO date string
- browser-native date inputs may be used internally later, but public value and
  calendar behavior stay Pug-owned

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::date_picker`

## 11. Parity Checklist

- [ ] trigger and open-state semantics match
- [ ] selected-date commit behavior matches
- [ ] popup calendar behavior matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| popup positioning details may differ | overlay runtime details differ by platform | allowed | keep value, open, and focus semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: forms, filter bars, inspector controls

## Next Task

Treat `DatePicker` as the low-level date value control, and keep scheduling or
workflow-specific date orchestration in higher layers.
