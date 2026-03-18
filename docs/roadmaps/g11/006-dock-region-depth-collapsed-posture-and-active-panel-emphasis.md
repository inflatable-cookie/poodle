# g11.006 — Dock-Region Depth, Collapsed Posture, And Active-Panel Emphasis

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.005
Primary repos: `pug`

## Goals

- [x] deepen `DockRegion` beyond simple panel grouping
- [x] support stronger active-panel emphasis and collapsed-tab behavior

## Execution Checklist

- [x] define tab placement options for dock regions
- [x] define collapsed-tab posture and re-open affordances
- [x] define active versus inactive panel emphasis
- [x] define quieter dock-local status presentation
- [x] keep dock logic generalized and free of product workflow assumptions
- [x] update the existing dock-region contract in `docs/contracts/workstation/`
  following the 12-section template

## Deliverables

- `docs/contracts/workstation/dock-region.md` — updated with `emphasis` prop
  (`standard`/`quiet`/`strong`), `collapsedPosture` prop (`hidden`/`icon-strip`),
  and expanded visual states

## Acceptance Criteria

- [x] dock semantics cover tab placement, collapse, and active-panel emphasis
- [x] dock behavior is strong enough for real workstation use without becoming
  app-specific

## Next Task

Open `g11.007` and make surface and panel tabs window-aware.
