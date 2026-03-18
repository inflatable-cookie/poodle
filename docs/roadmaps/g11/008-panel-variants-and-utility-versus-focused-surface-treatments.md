# g11.008 — Panel Variants And Utility-Versus-Focused Surface Treatments

Status: complete
Owner: Pug Core
Updated: 2026-03-17
Depends on: g11.007
Primary repos: `pug`

## Goals

- [x] add generalized panel variants that reduce downstream custom styling
- [x] support quiet utility panels and stronger focused panels in one shared
  system

## Execution Checklist

- [x] define at least three panel treatment families:
  - [x] utility/support
  - [x] standard
  - [x] focused/detail
- [x] specify header, body, border, and background deltas for each family
- [x] verify compatibility with existing `PanelSurface` semantics
- [x] ensure token usage remains shared across renderers
- [x] update the panel-surface contract in `docs/contracts/workstation/`
  following the 12-section template

## Deliverables

- `docs/contracts/workstation/panel-surface.md` — updated with `variant` prop
  (`utility`/`standard`/`focused`) defining three treatment families with
  distinct header, body, border, and background deltas

### Variant Treatments

| Variant | Header | Border | Background | Use Cases |
|---------|--------|--------|------------|-----------|
| `utility` | quieter, smaller text | subtle or none | slightly recessed | file browsers, logs, output panels |
| `standard` | normal | standard border | default surface | general-purpose panels |
| `focused` | stronger, bolder | stronger border or accent | slightly elevated | editors, inspectors, detail views |

## Acceptance Criteria

- [x] panel variant model is explicit and documented
- [x] downstream apps can express calmer utility panels and stronger focus
  panels without local one-off restyling

## Next Task

Open `g11.009` and define hosted external-surface semantics.
