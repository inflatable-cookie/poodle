# Time Field

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `TimeField`
- Layer: `foundation`
- Summary: a time-only value control for local wall-clock entry
- In scope: time value entry, min/max constraints, step sizing, disabled state
- Out of scope: timezone conversion, date ownership, recurrence, schedule
  workflows

## 2. Anatomy

```text
[Input]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `min`: `string | null`
- `max`: `string | null`
- `step`: `number`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`
- `describedBy`: `string | null`

## 4. States

- empty
- populated
- disabled

## 5. Events

- `onValueChange`

## 6. Accessibility

- role: time entry field using native input semantics
- required semantics: accessible name, disabled state, descriptive relation when
  supplied
- keyboard: standard text and native time-input editing

## 7. Layout

- follows shared control sizing and field chrome
- parent owns any surrounding label, helper, or validation composition

## 8. Token Usage

- control background, border, text, and focus roles

## 9. Svelte Notes

- public value uses local time strings in `HH:MM` form
- implementation may use browser-native `input[type="time"]` behavior as long
  as the public value contract stays Pug-owned

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::time_field`

## 11. Parity Checklist

- [ ] time value semantics match
- [ ] disabled and descriptive semantics match
- [ ] min/max/step posture stays equivalent

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native editing affordances may differ | platform time-entry controls differ | allowed | keep public value meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: settings rows, booking fields, datetime pickers

## Next Task

Use `TimeField` as the standalone time-value primitive, and let composed
datetime controls build on it rather than redefining time entry semantics.
