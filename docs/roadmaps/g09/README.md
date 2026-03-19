# g09 Jetstream Production Quality

Status: planned
Updated: 2026-03-19

## Context

Prior generations built a Jetstream adapter with a solid theme bridge and layout
mapper, but only 8 real component implementations. ~100 adapter render functions
exist as stubs that return type strings with no component-specific rendering.
The 8 real components all hardcode their dimensions.

This generation mirrors g08's program for the Jetstream target: audit against
current contracts, fix token resolution, build out missing components where
feasible within Jetstream's rendering constraints, and achieve visual parity
with Svelte for all supported components.

Jetstream has inherent constraints (no SVG, no gradients, no transforms, limited
text rendering, no ARIA) that mean some components will require native adaptation
or intentional exclusion. These must be documented honestly in the delta register.

## Starting State

- 8 real Jetstream components: button, accordion, checkbox, switch, badge,
  progress, separator, status_indicator
- ~100 adapter stubs returning type strings with `_spec`/`_theme` unused
- Theme bridge (`theme.rs`) and layout mapper (`style_map.rs`) are solid
- 8 specimen pages in preview app
- 20 real component tests, ~150 stub assertion tests
- Demo/Tokens sections are "coming soon" placeholders
- All 8 real components hardcode dimensions as `f32` constants
- Svelte-side refactoring may still be in progress — contracts are a
  moving target

## Exit State

- Every feasible Jetstream component has a real implementation with token
  resolution
- Unfeasible components documented with rationale in delta register
- Adapter stubs replaced with real implementations or explicitly excluded
- Specimen pages match contract definitions for all implemented components
- Visual parity with Svelte verified for supported components
- Cross-cutting issues (hardcoded geometry, disabled opacity) fixed

## Non-Goals

- No new contract work
- No renegotiating Jetstream rendering constraints — adapt within them
- No downstream app adoption proof

## Milestone Status

| ID  | Milestone | Depends On | Class | Status |
|-----|-----------|------------|-------|--------|
| 001 | Sync with contracts and feasibility assessment | g08.009 | Foundation | Planned |
| 002 | Fix existing 8 components: token resolution | 001 | Implementation | Planned |
| 003 | Implement feasible missing components (batch 1) | 002 | Implementation | Planned |
| 004 | Implement feasible missing components (batch 2) | 002 | Implementation | Planned |
| 005 | Remove or document unfeasible adapter stubs | 003, 004 | Implementation | Planned |
| 006 | Specimen pages for all implemented components | 005 | Implementation | Planned |
| 007 | Visual parity verification and delta register | 006 | Hardening | Planned |
| 008 | Generation closeout | 007 | Closure | Planned |

## Dependency Shape

```text
g08.009 GPUI Complete
  -> 001 Sync + Feasibility
      -> 002 Fix Existing 8
          -> 003 New Batch 1  ─┐
          -> 004 New Batch 2  ─┴─> 005 Clean Up Stubs
                                       -> 006 Specimens
                                            -> 007 Parity
                                                 -> 008 Closeout
```

## Contract Verification Rule

Same rule as g08: **every milestone must begin by checking the current state of
relevant contracts.** Component names, props, and token targets may have changed
since the previous milestone completed. Do not assume stability — verify.

## Jetstream Rendering Constraints

For reference, Jetstream cannot support:
- SVG rendering (icons must be texture-based or omitted)
- Gradients (adapter uses solid approximations)
- Transforms (no rotation, scale, skew)
- Rich text or IME
- ARIA / screen readers (not applicable in game engine context)
- Stacked or inset shadows
- Touch / multi-touch input

Components requiring these capabilities should be classified as unfeasible
with clear rationale in the delta register.
