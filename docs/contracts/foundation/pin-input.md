# PinInput

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `PinInput`
- Layer: `foundation`
- Summary: a fixed-length code-entry control split across multiple visible
  cells
- In scope: fixed length, digit-by-digit entry, completion callback, optional
  masking
- Out of scope: arbitrary text entry and secret-management workflows

## 2. Anatomy

```text
[Root]
  └── [Cell Input...]
```

## 3. Props And Inputs

- `value`: `string | null`
- `defaultValue`: `string`
- `length`: `number`
- `isDisabled`: `boolean`
- `ariaLabel`: `string | null`
- `mask`: `boolean`

## 4. States

- empty
- partially complete
- complete
- focus
- disabled

## 5. Events

- `onValueChange`
- `onComplete`

## 6. Accessibility

- role: grouped text-entry control
- required semantics: accessible group label plus per-cell naming
- keyboard: digit entry, backspace, and directional movement between cells

## 7. Layout

- fixed-size cells with stable spacing
- completion should not shift layout

## 8. Token Usage

- text-input border, focus, spacing, and code-typography roles

## 9. Svelte Notes

- may use multiple coordinated inputs rather than a single hidden input

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::pin_input`

## 11. Parity Checklist

- [ ] fixed-length semantics match
- [ ] directional movement and backspace behavior match
- [ ] completion callback meaning matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| masking visuals may differ | platform text-entry visuals differ | allowed | keep fixed-length behavior strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: verification flows, compact code-entry surfaces

## Next Task

Keep `PinInput` focused on fixed-length token entry rather than general secure
text-input behavior.
