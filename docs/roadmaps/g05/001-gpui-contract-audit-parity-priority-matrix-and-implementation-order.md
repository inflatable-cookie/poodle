# g05.001 GPUI Contract Audit, Parity Priority Matrix, And Implementation Order

Status: completed
Owner: Pug Core
Updated: 2026-03-13
Depends on: g03.014
Primary repos: `pug`

## Goals

- [x] define the GPUI implementation order against the current contract surface
- [x] identify where GPUI should imitate Svelte closely versus where native
  deltas are expected
- [x] define which surfaces must stay visually and structurally close enough to
  support side-by-side Svelte and GPUI comparison

## Execution Checklist

- [x] inventory the current contract-backed Svelte surface by layer and family
- [x] classify GPUI targets as direct parity, native adaptation, or deferred
- [x] define the first delivery order for GPUI primitives, composites, and
  workstation surfaces
- [x] identify which Svelte preview sections should have matching GPUI review
  surfaces rather than only abstract contract coverage
- [x] record initial non-goals so GPUI work does not reopen app-specific
  ownership boundaries

## Acceptance Criteria

- [x] GPUI implementation order is explicit
- [x] initial parity and native-delta posture is explicit
- [x] side-by-side review target surfaces are explicit

## Completed Work

- added the normative baseline `docs/specs/048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`
- added the machine-readable priority artifact `packages/gpui/parity-priority-matrix.json`
- classified every current preview/docs section as `direct-parity`, `native-adaptation`, or `deferred` with explicit GPUI layer ownership
- defined the initial implementation waves from theme or preview foundation through primitives, composites, and workstation shell
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the GPUI priority matrix remains machine-checked
- updated `packages/gpui/tokens/README.md` so the token crate no longer reads as isolated from the new implementation-order baseline

## Next Task

Open `g05.002` and define the GPUI theme runtime, token application, and
native preview app baseline against the now-explicit implementation order.
