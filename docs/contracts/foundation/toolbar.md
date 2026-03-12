# Toolbar

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Toolbar`
- Layer: `foundation`
- Summary: a semantic grouping container for compact action controls with
  toolbar semantics
- In scope: orientation, grouped labeling, keyboard movement between controls
- Out of scope: workstation-specific panel headers or menu bars

## 2. Anatomy

```text
[Root Toolbar]
  └── [Focusable Controls...]
```

## 3. Props And Inputs

- `orientation`: `"horizontal" | "vertical"`
- `ariaLabel`: `string | null`

## 4. States

- default
- focus-within

## 5. Events

- none owned by the toolbar itself

## 6. Accessibility

- role: `toolbar`
- required semantics: accessible label when no visible title exists
- keyboard: directional movement between focusable descendants

## 7. Layout

- compact grouped spacing
- supports horizontal or vertical arrangement

## 8. Token Usage

- surface, border, spacing, and focus-context roles

## 9. Svelte Notes

- may use a neutral container with descendant focus movement rather than a
  compound substrate API

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::toolbar`

## 11. Parity Checklist

- [ ] toolbar semantics match
- [ ] directional focus movement matches
- [ ] orientation meaning matches

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| descendant focus implementation may differ | runtime focus engines differ | allowed | keep toolbar semantics strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: formatting bars, shell utility rows, compact tool groups

## Next Task

Use `Toolbar` for grouped utility controls and keep product-specific shell
headers in composite or workstation contracts.
