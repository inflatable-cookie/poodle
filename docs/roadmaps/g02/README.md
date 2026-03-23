# g02 Advanced Composites, Quality Cleanup, And Release Baseline

Status: completed
Updated: 2026-03-11

## Context

`g02` begins once Poodle has a credible token system, contract model, primitive
suite, workstation-shell baseline, and first Underlay bridge posture. The next
bottleneck is no longer whether Poodle exists conceptually; it is whether Poodle can
carry a richer catalogue, a usable docs/review surface, and a cleaner package
shape before downstream repos start depending on it.

## Starting State

- `g01` has frozen token and contract foundations
- first primitives exist or are explicitly bounded
- first workstation-shell layer exists as a documented surface
- Underlay integration posture is real enough to bound later adoption work

## Exit State

- advanced composites are explicit and sequenced
- product-app and workstation-app component families both have credible depth
- docs and examples are usable enough to support sustained internal review
- downstream adoption is deliberately deferred until the surface is less rough
- packaging and release baseline exist before `g03` hardening

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Forms and validation system depth | g01.014 | Depth | Completed |
| 002 | Data table and bulk-action suite | g01.014 | Depth | Completed |
| 003 | Lists, grids, filters, pagination, and search depth | g01.014 | Depth | Completed |
| 004 | Detail display, cards, headers, and navigation suite | g01.014 | Depth | Completed |
| 005 | Picker, relation, and selection workflow suite | 001, 003, 004 | Depth | Completed |
| 006 | Media preview, embed, and asset-surface suite | 004 | Depth | Completed |
| 007 | Loading, empty, error, notification, and remediation depth | 001-006 | Hardening | Completed |
| 008 | Command palette and action-discovery depth | g01.012, 004, 007 | Workstation | Completed |
| 009 | App-shell and workspace-shell depth | g01.012, 004 | Workstation | Completed |
| 010 | Dock, split-view, tabs, and persistence orchestration | 009 | Workstation | Completed |
| 011 | Accessibility, focus, keyboard, and state semantics hardening | 001-010 | Hardening | Completed |
| 012 | Docs site, examples, and component discoverability baseline | 001-011 | Adoption | Completed |
| 013 | Preview, docs, and example-harness usability hardening | 001-012 | Hardening | Completed |
| 014 | Component API cleanup, package ergonomics, and parity debt | 008-012 | Hardening | Completed |
| 015 | Packaging, release, and versioning baseline | 012-014 | Foundation | Completed |
| 016 | Generation closeout and `g03` cutover plan | 015 | Closure | Completed |

## Next Task

`g02` is complete. Open `g03.001` and freeze token evolution, migration, and
compatibility policy before the first real downstream adoption tranche begins.
