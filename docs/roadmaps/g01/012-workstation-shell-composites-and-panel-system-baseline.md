# g01.012 Workstation-Shell Composites And Panel System Baseline

Status: planned
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.007, g01.008, g01.009, g01.010
Primary repos: `pug`

## Context

Pug is expected to support desktop/workstation applications beyond simple web
product UIs. That requires an explicit shell layer.

## Goals

- [ ] define app-header and project-header patterns
- [ ] define panel surface, panel header, panel tabs, and surface tabs
- [ ] define dock, split-view, and workspace-region expectations
- [ ] define command-palette and utility-shell posture at a contract level

## Non-Goals

- [ ] no DAW-specific transport, timeline, or mixer widgets

## Execution Checklist

- [ ] define the workstation-shell component families in scope
- [ ] define panel, tab, and dock interactions at the contract level
- [ ] define layout persistence and shell-state expectations at a high level
- [ ] document where app-specific workstation widgets begin and Pug ends

## Acceptance Criteria

- [ ] workstation-shell composite layer is explicit
- [ ] panel-system ownership is explicit
- [ ] extension boundary for app-specific widgets is preserved

## Deliverables

- [ ] workstation-shell composite catalogue
- [ ] panel-system baseline notes

## Next Task

Open `g01.013` and define the Underlay bridge against the now-explicit token
and component layers.
