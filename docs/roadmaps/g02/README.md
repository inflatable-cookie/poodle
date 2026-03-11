# g02 Advanced Composites, Adoption, And Release Baseline

Status: planned
Updated: 2026-03-11

## Context

`g02` begins once Pug has a credible token system, contract model, primitive
suite, workstation-shell baseline, and first Underlay bridge posture. The next
bottleneck is no longer whether Pug exists conceptually; it is whether Pug can
carry the richer composite catalogue and downstream adoption depth needed to be
worth standardizing around.

## Starting State

- `g01` has frozen token and contract foundations
- first primitives exist or are explicitly bounded
- first workstation-shell layer exists as a documented surface
- Underlay integration posture is real enough to begin adoption planning

## Exit State

- advanced composites are explicit and sequenced
- product-app and workstation-app component families both have credible depth
- docs and examples are navigable enough to support adoption
- Underlay and GPUI downstream adoption both have real milestone tranches
- packaging and release baseline exist before `g03` hardening

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Forms and validation system depth | g01.014 | Depth | Planned |
| 002 | Data table and bulk-action suite | g01.014 | Depth | Planned |
| 003 | Lists, grids, filters, pagination, and search depth | g01.014 | Depth | Planned |
| 004 | Detail display, cards, headers, and navigation suite | g01.014 | Depth | Planned |
| 005 | Picker, relation, and selection workflow suite | 001, 003, 004 | Depth | Planned |
| 006 | Media preview, embed, and asset-surface suite | 004 | Depth | Planned |
| 007 | Loading, empty, error, notification, and remediation depth | 001-006 | Hardening | Planned |
| 008 | Command palette and action-discovery depth | g01.012, 004, 007 | Workstation | Planned |
| 009 | App-shell and workspace-shell depth | g01.012, 004 | Workstation | Planned |
| 010 | Dock, split-view, tabs, and persistence orchestration | 009 | Workstation | Planned |
| 011 | Accessibility, focus, keyboard, and state semantics hardening | 001-010 | Hardening | Planned |
| 012 | Docs site, examples, and component discoverability baseline | 001-011 | Adoption | Planned |
| 013 | Underlay adoption tranche and wrapper preservation | 001-012 | Adoption | Planned |
| 014 | GPUI app adoption tranche and desktop validation | 008-012 | Adoption | Planned |
| 015 | Packaging, release, and versioning baseline | 012-014 | Foundation | Planned |
| 016 | Generation closeout and `g03` cutover plan | 015 | Closure | Planned |

## Next Task

Keep `g02` focused on broadening the reusable surface and proving adoption, not
on introducing app-specific widgets that should remain downstream.
