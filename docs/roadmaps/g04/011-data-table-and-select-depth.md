# g04.011 DataTable And Select Depth

Status: planned
Owner: Flint Core
Updated: 2026-03-14
Depends on: g04.002 through g04.010
Primary repos: `flint`

## Goals

- [ ] extend DataTable with CSV export, column visibility toggles, and enhanced
  sorting controls
- [ ] extend Select with grouped options using section headings

## Execution Checklist

- [ ] amend DataTable contract: add `exportCsv` action, `columnVisibility`
  toggle config, column reordering
- [ ] implement DataTable CSV export with filename and delimiter options
- [ ] implement DataTable column visibility toggles via a popover menu
- [ ] implement DataTable enhanced sorting with multi-column sort indicators
- [ ] amend Select contract: add `group` property to options, `groupLabel`
  rendering
- [ ] implement Select grouped options with visual separators and group headings
- [ ] update DataTable specimen with export and column visibility examples
- [ ] update Select specimen with grouped options example

## Acceptance Criteria

- [ ] DataTable CSV export generates a downloadable CSV file from visible data
- [ ] DataTable column visibility menu allows hiding/showing individual columns
- [ ] DataTable multi-column sort shows numbered sort indicators
- [ ] Select renders option groups with non-selectable heading labels
- [ ] Select group headings are ARIA-compliant (role=group with aria-label)
- [ ] all enhancements pass build and render in the preview catalogue

## Next Task

Open `g04.012` and implement operational display and monitoring patterns.
