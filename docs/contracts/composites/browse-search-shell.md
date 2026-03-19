# BrowseSearchShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `BrowseSearchShell`
- Layer: `composites`
- Summary: a reusable composition pattern that pairs a `FilterToolbar` with a
  browse result shell such as `ListShell`, `GridShell`, or `DataTable`
- In scope: query/filter control grouping, result summary posture, shell state
  transitions, pagination or progressive-loading pairing
- Out of scope: query execution, domain-specific filter logic, row/card
  rendering, network policy

## 2. Anatomy

```text
[Root]
  ├── [FilterToolbar]
  ├── [State / Summary Region] (optional)
  └── [Browse Result Shell]
        ├── [ListShell]
        ├── [GridShell]
        └── [DataTable]
```

## 3. Core Rule

`BrowseSearchShell` adds composition rules, not new low-level control meaning.

It exists to make three relationships explicit:

- search and filter controls are grouped and labeled
- result-state posture is visible and distinct
- the chosen browse shell owns result presentation without redefining query
  semantics

## 4. Accessibility

- search and filter controls must remain in logical tab order before the result
  shell
- summary text supplements, but does not replace, browse-region labels
- no-results state must remain distinct from empty-collection state
- GPUI-native accessibility mapping notes: GPUI must preserve the same query
  group, summary, and result-region relationships without relying on HTML
  landmarks alone

## 5. Composition Guidance

- use `FilterToolbar` for grouped controls and visible result summary
- use `ListShell` when row cadence and progressive append are primary
- use `GridShell` when card/tile browse is primary
- use `DataTable` when structured columns, sorting, and row selection are primary
- pair with `PaginationSummary` when stable page ranges matter
- prefer progressive loading when preserving context matters more than precise
  page position

## 6. Specimen Definitions

No Svelte specimen file exists for `BrowseSearchShell`. A specimen should be created at `packages/svelte/preview/src/specimens/BrowseSearchShellSpecimen.svelte` when the component is implemented, demonstrating the composition of `FilterToolbar` with at least one browse result shell variant (`ListShell`, `GridShell`, or `DataTable`) across ready, loading, empty, and no-results states.

## 7. Next Task

Use `BrowseSearchShell` as the composition baseline for `g02.003` and later
browse-heavy tranches instead of reinventing search-and-results structure per
screen.
