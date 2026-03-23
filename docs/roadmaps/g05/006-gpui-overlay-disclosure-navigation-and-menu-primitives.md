# g05.006 GPUI Overlay, Disclosure, Navigation, And Menu Primitives

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g05.001, g05.002, g05.003, g05.004, g05.005
Primary repos: `flint`

## Goals

- [x] implement the GPUI overlay, disclosure, navigation, and menu primitive families
- [x] define which browser-specific patterns should become explicit native deltas

## Execution Checklist

- [x] implement dialog, popover, menu, tabs, tooltip, drawer, disclosure, and
  related navigation primitives in GPUI where the contracts already exist
- [x] define native focus-scope, dismissal, and layering behavior explicitly
- [x] record any platform-specific deltas for hover, submenu, or disclosure behavior
- [x] avoid baking browser-only assumptions into the native runtime surface

## Acceptance Criteria

- [x] GPUI overlay and navigation primitive posture is explicit
- [x] native focus-scope and dismissal deltas are explicit

## Completed Work

- added the normative baseline `docs/specs/053-gpui-overlay-disclosure-navigation-and-menu-primitives-baseline.md`
- added the machine-readable artifact `packages/gpui/overlay-navigation-menu-baseline.json`
- expanded `packages/gpui/primitives` with:
  - `AccordionSpec`
  - `CollapsibleSpec`
  - `DialogSpec`
  - `DrawerSpec`
  - `PopoverSpec`
  - `TooltipSpec`
  - `MenuSpec`
  - `ContextMenuSpec`
  - `TabsSpec`
  - `NavigationMenuSpec`
  - `MenubarSpec`
  - `TabStripSpec`
- added shared GPUI primitive types for disclosure items, overlay placement, dialog kind, drawer edge, menu entries, and tab or navigation selection models
- froze GPUI open-state, dismissal, overlay-layer, and top-level navigation posture inside the Rust crate so later composites inherit the same contract-owned semantics as Svelte
- added crate tests for disclosure state, dialog or drawer dismissal posture, overlay placement, actionable menu counts, and navigation-selection ownership
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI overlay or disclosure or navigation or menu baseline artifact is machine-checked
- updated package and roadmap surfaces so the repo now points at `g05.007`

## Next Task

Open `g05.007` and implement the GPUI form, validation, and remediation
composite parity tranche.
