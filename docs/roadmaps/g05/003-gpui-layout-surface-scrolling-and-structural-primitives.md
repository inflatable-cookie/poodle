# g05.003 GPUI Layout, Surface, Scrolling, And Structural Primitives

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g05.001, g05.002
Primary repos: `pug`

## Goals

- [x] implement the structural GPUI primitives that the rest of the shared
  surface depends on
- [x] prove token and layout parity at the foundation layer before broader
  component depth begins

## Execution Checklist

- [x] implement GPUI equivalents for layout, surface, separator, and scrolling
  primitives where the contracts already exist
- [x] keep token usage, spacing, elevation, and density aligned to the shared
  contracts
- [x] document any native layout or scrolling deltas explicitly instead of
  burying them in code
- [x] verify the new primitives are usable by later GPUI component tranches

## Acceptance Criteria

- [x] GPUI structural primitive baseline is explicit
- [x] structural parity and delta posture is explicit

## Completed Work

- added the normative baseline `docs/specs/050-gpui-structural-primitives-baseline.md`
- added the machine-readable artifact `packages/gpui/structural-primitives-baseline.json`
- added the first real GPUI runtime crate `packages/gpui/primitives` with:
  - `BoxSpec`
  - `StackSpec`
  - `GridSpec`
  - `SurfaceSpec`
  - `SeparatorSpec`
  - `ScrollShellSpec`
- bound the new crate to `pug-gpui-tokens` so spacing, surface, border, elevation, and focus treatment resolve from emitted tokens rather than local constants
- added crate-level tests proving the initial structural token mappings and defaults
- promoted `pug-gpui-primitives` into the repo release metadata and package-stability docs as a preview-channel public-intent Rust crate
- updated the GPUI token README so the GPUI package story no longer reads as token-only

## Next Task

Open `g05.004` and implement the GPUI action, text-entry, and field primitive
tranche on top of the new structural baseline.
