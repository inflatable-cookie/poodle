# g01.012 Workstation-Shell Composites And Panel System Baseline

Status: completed
Owner: Pug Core
Updated: 2026-03-11
Depends on: g01.007, g01.008, g01.009, g01.010
Primary repos: `pug`

## Context

Pug is expected to support desktop/workstation applications beyond simple web
product UIs. That requires an explicit shell layer.

## Goals

- [x] define app-header and project-header patterns
- [x] define panel surface, panel header, panel tabs, and surface tabs
- [x] define dock, split-view, and workspace-region expectations
- [x] define command-palette and utility-shell posture at a contract level

## Non-Goals

- [x] no DAW-specific transport, timeline, or mixer widgets

## Execution Checklist

- [x] define the workstation-shell component families in scope
- [x] define panel, tab, and dock interactions at the contract level
- [x] define layout persistence and shell-state expectations at a high level
- [x] document where app-specific workstation widgets begin and Pug ends

## Acceptance Criteria

- [x] workstation-shell composite layer is explicit
- [x] panel-system ownership is explicit
- [x] extension boundary for app-specific widgets is preserved

## Deliverables

- [x] workstation-shell composite catalogue
- [x] panel-system baseline notes

## Next Task

Open `g01.013` and define the Underlay bridge against the now-explicit token,
primitive, product-composite, and workstation-shell layers.
