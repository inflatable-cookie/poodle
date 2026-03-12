# Pagination

Status: seed contract
Updated: 2026-03-12

## 1. Purpose

- Component name: `Pagination`
- Layer: `foundation`
- Summary: a low-level page navigation control for moving between discrete
  result pages
- In scope: previous/next controls, page buttons, current-page state,
  truncated page window
- Out of scope: range summaries, page-size controls, server fetch policy,
  progressive loading

## 2. Anatomy

```text
[Root]
  ├── [Previous]
  ├── [Page Button...]
  └── [Next]
```

## 3. Props And Inputs

- `currentPage`: `number`
- `totalPages`: `number`
- `siblingCount`: `number`
- `ariaLabel`: `string | null`

## 4. States

- first page
- middle page
- last page
- truncated page window

## 5. Events

- `onPageChange`

## 6. Accessibility

- role: labeled navigation group
- required semantics: current page exposure, disabled boundary controls,
  reachable page buttons
- keyboard: standard button navigation in DOM order

## 7. Layout

- page controls may wrap when space is constrained
- parent owns surrounding range summary, total counts, and browse-shell context

## 8. Token Usage

- control, focus, border, accent, and subdued text roles

## 9. Svelte Notes

- public contract owns page navigation semantics only
- range summaries and browse-shell copy stay outside this primitive

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::primitives::pagination`

## 11. Parity Checklist

- [ ] current-page and boundary semantics match
- [ ] page-change requests match
- [ ] truncated page window remains equivalent enough for review

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| exact truncation window may differ slightly | pagination windowing is implementation-owned | allowed | keep current page and boundary meaning strict |

## 13. Approval And Adoption Notes

- contract status: `seed contract`
- downstream adopters: simple browse surfaces, reports, low-interaction data
  views

## Next Task

Use `Pagination` for page navigation only, and keep browse summaries or table
selection interplay in composites.
