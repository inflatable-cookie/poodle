# Table

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Table`
- Layer: `foundation`
- Summary: a low-level structured data surface that preserves native table
  semantics without pulling sorting, selection, or row actions into the same
  contract
- In scope: column headers, row and cell relationships, row-header posture,
  empty state, caption
- Out of scope: sorting, row selection, row actions, bulk actions,
  virtualization, pagination policy

## 2. Anatomy

```text
[Root]
  └── [Table]
        ├── [Caption] (optional)
        ├── [Header Row]
        │     └── [Column Header...]
        └── [Body]
              └── [Row...]
                    └── [Cell...]
```

## 3. Props And Inputs

- `columns`: `Array<{ id: string; label: string; align?: "start" | "end"; isRowHeader?: boolean }>`
- `rows`: `Array<{ id: string; cells: Record<string, string>; summary?: string | null }>`
- `caption`: `string | null`
- `emptyMessage`: `string`
- `ariaLabel`: `string | null`

## 4. States

- ready
- empty

## 5. Events

- none in this baseline

## 6. Accessibility

- role: native table semantics
- required semantics: stable header-to-cell relationships, row header support,
  caption or accessible name when needed
- keyboard: standard document and assistive navigation through native table
  semantics

## 7. Layout

- table may overflow horizontally within its shell
- parent owns surrounding filter, sort, selection, and pagination composition

## 8. Token Usage

- surface, border, text, subdued header, and spacing roles

## 9. Svelte Notes

- should prefer real `<table>` markup
- richer interactivity should compose above this primitive rather than being
  implied by the foundation table itself

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::table`

## 11. Parity Checklist

- [ ] header, row, and cell semantics match
- [ ] row-header posture matches
- [ ] empty-table posture remains explicit

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact overflow handling may differ | layout internals differ by runtime | allowed | keep structured table meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: simple structured reports, low-interaction admin tables,
  read-only data surfaces

## Next Task

Use `Table` for low-level structured data meaning, and layer sorting,
selection, actions, and browse posture in composites like `DataTable`.
