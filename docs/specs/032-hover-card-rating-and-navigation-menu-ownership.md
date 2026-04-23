# 032 Hover Card Rating And Navigation Menu Ownership

Status: active
Updated: 2026-03-12
Depends on: `029-advanced-primitive-promotion-and-substrate-mapping.md`, `004-overlay-focus-dismissal-and-layering-rules.md`

## Purpose

The wider primitive catalogue still had an unresolved utility bucket around
preview overlays, judgment controls, and navigation menu families. This spec
closes that ambiguity by promoting the generalized pieces and explicitly
deferring the navigation-owned ones.

## Promote To Foundation

The current utility tranche promotes:

- `HoverCard`
- `Rating`

These are generalized enough to serve broad app needs without pulling in
application navigation ownership.

## Value And Behavior Rule

`HoverCard` owns:

- delayed hover or focus open posture
- non-modal preview content
- anchored overlay behavior distinct from tooltip and popover

`Rating` owns:

- bounded ordinal selection
- optional clear-on-repeat behavior
- radio-group style keyboard semantics

## Follow-On Resolution

`NavigationMenu` and `Menubar` were deferred initially to force an explicit
ownership decision. That follow-on decision is now captured in
`033-navigation-menu-and-menubar-foundation-baseline.md`, which promotes both
surfaces into foundation with a narrower, explicit ownership boundary.

## Ownership Boundary Rule

Foundation should own compact utility overlays and judgment controls when:

- the control meaning is generalized across apps
- value semantics are small and explicit
- the primitive does not silently claim product navigation structure

Foundation should defer navigation-menu families until there is an explicit
contract for:

- site or app navigation hierarchy ownership
- expanded or collapsed nav state semantics
- command vs navigation distinctions
- parity expectations with workstation and composite shells

## Current Risk

`HoverCard` and `Rating` are explicit foundation surfaces, and the former
navigation ambiguity has now been resolved by the dedicated navigation-family
baseline. Richer routing and shell integration still remain intentionally
outside this utility tranche.

## Evidence

- `docs/contracts/components/hover-card.md`
- `docs/contracts/components/rating.md`
- `packages/svelte/components/README.md`
- `docs/specs/029-advanced-primitive-promotion-and-substrate-mapping.md`

## Next Task

Use this utility tranche alongside the dedicated navigation-family baseline,
and keep widening only where the ownership line remains explicit.
