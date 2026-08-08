---
title: g04.006 gpui overlay disclosure navigation and menu baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, primitives, overlays]
---

## Summary

Completed `g04.006` by widening `pug-gpui-primitives` into the first GPUI
overlay, disclosure, navigation, and menu primitive baseline.

## What changed

- added the normative baseline `docs/specs/053-gpui-overlay-disclosure-navigation-and-menu-primitives-baseline.md`
- completed `docs/roadmaps/g04/006-gpui-overlay-disclosure-navigation-and-menu-primitives.md`
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
- added shared GPUI primitive types for disclosure items, overlay placement,
  dialog kind, drawer edge, menu entries, and tab or navigation selection
  models
- pinned GPUI open-state, dismissal, overlay-layer, and top-level navigation
  posture inside the Rust crate so later composites inherit the same contract
  semantics as Svelte
- added crate tests for disclosure state, dialog or drawer dismissal posture,
  overlay placement, actionable menu counts, and navigation selection
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the new GPUI
  overlay or disclosure or navigation or menu baseline artifact is
  machine-checked
- rolled the package and roadmap surfaces forward to `g04.007`

## Validation

- `cargo fmt --manifest-path packages/gpui/primitives/Cargo.toml`
- `cargo check --manifest-path packages/gpui/primitives/Cargo.toml`
- `cargo test --manifest-path packages/gpui/primitives/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.006` is now explicit. Pug has a broader GPUI primitive baseline for
overlay, disclosure, navigation, and menu semantics, which removes another
large “Svelte-only by default” gap before composite and workstation parity.

## Next

Open `g04.007` and implement the GPUI form, validation, and remediation
composite parity tranche on top of the now-explicit primitive surface.
