# g12.002 — GPUI Workstation Implementation Batch 1: Windows, Regions, Strips

Status: planned
Owner: Pug Core
Updated: 2026-03-17
Depends on: g12.001
Primary repos: `pug`

## Goals

- [ ] implement the first half of the new workstation substrate in GPUI
- [ ] keep GPUI aligned to the g11 contracts rather than growing a native-only
  shell path

## Execution Checklist

- [ ] implement workspace window host primitives in GPUI
- [ ] implement expanded region snapshot support
- [ ] implement strip rail components and variants
- [ ] implement resize handles, split dividers, and collapse affordances
- [ ] add preview/specimen coverage for the new surface

## Acceptance Criteria

- [ ] the new window, region, strip, and resize surface exists in GPUI
- [ ] GPUI semantics match the documented contract rather than a renderer-local
  reinterpretation

## Next Task

Open `g12.003` and complete the GPUI workstation implementation.
