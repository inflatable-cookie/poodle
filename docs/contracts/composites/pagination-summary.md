# PaginationSummary

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `PaginationSummary`
- Layer: `composites`
- Summary: a compact pagination and range-summary surface paired with browse
  or table results
- In scope: current range summary, page count, previous/next controls
- Out of scope: arbitrary page-jump input, server pagination policy,
  virtualization internals

## 2. Anatomy

```text
[Root]
  ├── [Range Summary]
  └── [Paging Actions]
        ├── [Previous]
        ├── [Page Count]
        └── [Next]
```

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `currentPage` | `number` | `1` | yes | current page index |
| `totalPages` | `number` | `1` | yes | total available pages |
| `totalItems` | `number` | `0` | yes | total filtered items |
| `pageSize` | `number` | implementation choice | yes | current page size |

## 4. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| `onPageChange` | previous/next requested | `{ page }` | host owns page state |

## 5. Accessibility

- Role: labeled group
- Required behavior: visible textual range summary and disabled previous/next
  semantics at boundaries
- keyboard rule: previous and next are ordinary focusable buttons
- GPUI-native accessibility mapping notes: GPUI must preserve page position,
  disabled boundary state, and actionable previous/next controls explicitly

## 6. Composition

- parent expectations: `DataTable`, `ListShell`, `GridShell`
- host ownership: page size, fetch policy, and page reset rules stay host-owned

## 7. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Summary | body typography and subdued text roles | range copy |
| Actions | control, border, and focus roles | pagination controls |

## 8. Next Task

Pair `PaginationSummary` with browse and table composites instead of teaching
row or card items about pagination state directly.
