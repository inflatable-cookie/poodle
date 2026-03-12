# Meter

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Meter`
- Layer: `foundation`
- Summary: a bounded measurement display for current level within a known range
- In scope: value, range, low/high/optimum hints
- Out of scope: progress-task completion semantics and animated loading

## 2. Anatomy

```text
[Root]
  ├── [Track]
  └── [Value Fill]
```

## 3. Props And Inputs

- `value`: `number`
- `min`: `number`
- `max`: `number`
- `low`: `number | null`
- `high`: `number | null`
- `optimum`: `number | null`
- `ariaLabel`: `string | null`

## 4. States

- in-range
- low
- high
- optimum

## 5. Events

- none

## 6. Accessibility

- role: meter semantics or native equivalent
- required semantics: current value and range
- keyboard: none

## 7. Layout

- width is parent-owned
- remains compact and inline-friendly

## 8. Token Usage

- background, status, and bounded-value display roles

## 9. Svelte Notes

- native `<meter>` semantics are preferred when practical

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::meter`

## 11. Parity Checklist

- [ ] bounded-value semantics match
- [ ] range and optimum hints match
- [ ] progress-vs-meter meaning stays distinct

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| native meter styling may vary | platform visuals are not the contract | allowed | keep measurement semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: health bars, storage usage, bounded scoring

## Next Task

Keep `Meter` distinct from `Progress` so measurement displays do not inherit
task-completion semantics.
