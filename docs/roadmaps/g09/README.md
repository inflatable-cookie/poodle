# g09 GPUI Continuation

Status: active
Updated: 2026-03-30

## Context

`g09` began as the architecture-unification generation.
That original tranche is complete, and the generation has been extended
with the semantic sizing/density rollout and cross-runtime parity work.

## Milestones

### Completed (g09.001–g09.009)

| Milestone | Status | Summary |
|-----------|--------|---------|
| g09.001 | ✅ complete | Unify token crates |
| g09.002 | ✅ complete | Merge primitives |
| g09.003 | ✅ complete | Merge composites |
| g09.004 | ✅ complete | Delete GPUI duplicates |
| g09.005 | ✅ complete | Simplify component API |
| g09.006 | ✅ complete | Delete workstation crates |
| g09.007 | ✅ complete | Verify both targets |
| g09.008 | ✅ complete | Generation closeout (original) |
| g09.009 | ✅ complete | Semantic sizing and density rollout |

### g09.009 Scope (2026-03-30)

The largest milestone in g09. Delivered:
- Global size-and-density contract + treatment tokens contract
- `SemanticControlSizeRole`, `ControlDensity` Rust types
- 74 spec structs updated, 5 new specs created
- Density/size wired into GPUI + Jetstream adapters
- GPUI: 78 components wired, 5 new implementations
- Jetstream: 128 components (86 upgraded + 42 new), 243 tests passing
- Svelte: 75 components with size/density CSS, 3 accessibility fixes
- All 35 seed contracts → detailed (zero seeds remaining)
- Deep contract audit of 70 components, 16 systemic fixes
- OrderBy + Pagination contracts rewritten from scratch

## Current State

All three runtimes (Svelte, GPUI, Jetstream) have full contract coverage
with semantic sizing and density support. The contract surface is fully
detailed and audited against the Svelte reference implementation.

## Remaining for g09 Closeout

- [ ] Jetstream preview specimens for 42 new components
- [ ] Cross-runtime visual parity spot-check
- [ ] GPUI preview specimens for 5 new components
- [ ] Final `effigy health` + Rust test pass across all crates

## Working Rule

- treat `g09.001` through `g09.009` as completed work
- remaining items above are verification/polish, not new feature work
- `g10` can start once the verification pass confirms parity

## g10 Scope (Next Generation)

`g10` is Jetstream-focused: renderer integration, preview app polish,
and production readiness for the Jetstream target. The heavy lifting
of component implementation is now done in g09.009 — g10 focuses on
runtime integration, visual verification, and production hardening.
