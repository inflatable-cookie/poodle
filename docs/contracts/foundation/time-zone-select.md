# Time Zone Select

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `TimeZoneSelect`
- Layer: `foundation`
- Summary: a timezone-value control for choosing a named time zone
- In scope: timezone selection, placeholder behavior, disabled state, optional
  host-provided option set
- Out of scope: offset math, locale-specific timezone display policy,
  scheduling workflows

## 2. Anatomy

```text
[Select Trigger]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string | null`
- `placeholder`: `string | null`
- `options`: timezone options
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`
- `describedBy`: `string | null`

## 4. States

- placeholder
- selected timezone
- disabled

## 5. Events

- `onValueChange`

## 6. Accessibility

- role: select-style value control
- required semantics: accessible name, selected value exposure, disabled state,
  description relation when supplied
- keyboard: standard select-field behavior

## 7. Layout

- follows shared control sizing and field chrome
- parent owns labels, helper text, and wider date-time composition

## 8. Token Usage

- control background, border, text, and focus roles

## 9. Svelte Notes

- the public contract owns timezone identifiers as strings
- implementation may source a default timezone list internally, but hosts may
  still provide a curated option set when product needs require it

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::time_zone_select`

## 11. Parity Checklist

- [ ] timezone value semantics match
- [ ] placeholder and disabled semantics match
- [ ] host-provided option sets remain equivalent

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| default timezone list ordering may differ | platform timezone registries vary | allowed | keep public timezone value meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: scheduler setup, publishing settings, zoned date-time
  pickers

## Next Task

Use `TimeZoneSelect` as the standalone timezone-value primitive, and keep
timezone arithmetic or conversion workflows in higher layers.
