# Rating

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Rating`
- Layer: `foundation`
- Summary: an ordinal judgment control for choosing a bounded score
- In scope: bounded item count, single-value selection, optional clear-on-repeat
- Out of scope: review workflows, written feedback, weighted scoring systems

## 2. Anatomy

```text
[Root]
  └── [Rating Item...]
```

## 3. Props And Inputs

- `value`: `number | null`
- `defaultValue`: `number | null`
- `max`: `number`
- `allowClear`: `boolean`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`

## 4. States

- empty
- selected value
- disabled

## 5. Events

- `onValueChange`

## 6. Accessibility

- role: radio-group style bounded choice control
- required semantics: accessible group label, one current selected value at a
  time, keyboard movement across options
- keyboard: arrow movement across items, home/end to bounds, enter/space select

## 7. Layout

- items pack inline by default
- parent owns surrounding helper copy, labels, and review context

## 8. Token Usage

- interactive text, accent, focus, and optional hover-highlight roles

## 9. Svelte Notes

- glyph shape is implementation-owned; the public contract is ordinal selection,
  not star-specific branding

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::rating`

## 11. Parity Checklist

- [ ] bounded ordinal selection semantics match
- [ ] keyboard and focus movement match
- [ ] clear-on-repeat behavior stays equivalent when enabled

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| glyph style may differ | shape rendering is implementation-specific | allowed | keep ordinal meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: quality scoring, preference capture, review forms

## Next Task

Use `Rating` for bounded ordinal judgment, and keep richer review or feedback
workflows outside the primitive layer.
