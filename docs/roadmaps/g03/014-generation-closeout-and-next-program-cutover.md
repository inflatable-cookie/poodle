# g03.014 Generation Closeout And Next-Program Cutover

Status: completed
Owner: Pug Core
Updated: 2026-03-12
Depends on: g03.012, g03.013
Primary repos: `pug`

## Goals

- [x] summarize what is now mature and stable
- [x] record remaining ecosystem gaps and deferred work
- [x] define the next planning posture after the initial three-generation
  program

## Execution Checklist

- [x] summarize the mature and stable surface
- [x] record remaining ecosystem gaps and deferred work explicitly
- [x] define the next planning posture after the initial three-generation
  program

## Acceptance Criteria

- [x] generation closeout is explicit
- [x] deferred work and next-program posture are explicit

## Completed Work

- added the normative closeout baseline `docs/specs/047-generation-closeout-and-next-program-posture.md`
- added the machine-readable closeout artifact `packages/g03-closeout.json`
- captured the completed milestone set, stable surfaces, carry-forward gaps, and next-program posture explicitly instead of leaving them to summary prose
- extended `packages/svelte/preview/scripts/lint-docs.ts` so closeout surfaces and carry-forward gaps stay machine-checked
- rolled the roadmap and top-level docs surfaces forward so `g03` is visibly complete and no next generation is implied open yet

## Stable `g03` Surface

- [x] token evolution, compatibility, and deprecation posture are explicit
- [x] parity, accessibility, docs linting, and publish-candidate evidence are explicit and machine-checked
- [x] Svelte package surfaces and contract inventories are explicit and linted
- [x] Underlay and Loophole adoption boundaries are explicit and proof-backed
- [x] release operations, ecosystem acceptance, and onboarding/reference guidance are explicit

## Explicit Carry-Forward Gaps

- [x] GPUI component parity remains largely token-level and matrix-level rather than shipped runtime packages
- [x] the preview is still an internal docs and review surface, not a published public docs platform
- [x] reference shapes and onboarding lanes exist, but richer runnable starter or reference apps are still future work
- [x] deeper automation such as screenshot regression and native-runtime acceptance proof remains future work

## Next Task

`g03` is complete. If a new generation is needed, open it only after the
closeout surfaces and carry-forward gaps from `g03` are used as the explicit
planning frame.
