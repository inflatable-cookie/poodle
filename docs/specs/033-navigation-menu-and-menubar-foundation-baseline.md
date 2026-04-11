# 033 Navigation Menu And Menubar Foundation Baseline

Status: active
Updated: 2026-03-12
Depends on: `032-hover-card-rating-and-navigation-menu-ownership.md`, `004-overlay-focus-dismissal-and-layering-rules.md`, `028-primitive-baseline-and-bits-aligned-surface.md`

## Purpose

The navigation-family ownership question is now explicit enough to resolve.
This spec promotes the smallest generalized navigation family into foundation
without collapsing routing, shell architecture, and command ownership into one
ambiguous primitive.

## Foundation Navigation Family

The current navigation-family primitives are:

- `NavigationMenu`
- `Menubar`

They cover persistent top-level navigation disclosure and persistent top-level
command menus without claiming ownership of routing, app-shell layout, or
native window integrations.

## Ownership Split Rule

`NavigationMenu` owns:

- persistent top-level navigation triggers
- active open nav state
- associated viewport or disclosure content

`Menubar` owns:

- persistent top-level command menu triggers
- submenu overlays
- command activation and focus restoration

The following remain outside this foundation batch:

- breadcrumb trails
- sidebar tree navigation
- router integration
- native application menu bridging
- workstation panel or dock navigation

## Accessibility Rule

Navigation-family primitives must preserve:

- reachable persistent top-level triggers
- keyboard movement across top-level items
- clear differentiation between navigation disclosure and command menus
- predictable dismissal and focus restoration

## Composition Rule

Higher layers may wrap these primitives for real product information
architecture, but they should not redefine the underlying top-level nav and
menu semantics when these baseline primitives already fit.

## Current Risk

This batch intentionally stops at one submenu layer and slot-driven viewport
content. Richer routing, nested nav hierarchies, and native OS menu bridges
still need higher-layer or future specialized contracts.

## Evidence

- `docs/contracts/components/navigation-menu.md`
- `docs/contracts/components/menubar.md`
- `packages/svelte/primitives/README.md`
- `docs/specs/029-advanced-primitive-promotion-and-substrate-mapping.md`

## Next Task

Keep the navigation-family baseline stable while deciding whether deeper
routing-aware navigation or native menu integration belongs in composites,
workstation, or a future specialized tranche.
