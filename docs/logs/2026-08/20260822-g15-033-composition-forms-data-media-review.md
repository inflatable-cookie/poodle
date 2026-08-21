# g15.033 — Screen-clear review: composition, forms, data, and media

Date: 2026-08-22
Card: `docs/roadmaps/g15/033-review-composition-forms-data-media.md`
Handoff: `docs/handoffs/20260821-232939-g15-033-review-composition-forms-data-media.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: https://github.com/inflatable-cookie/poodle/pull/63

## Outcome

Final serial screen-clear review child. All seven owned pages received the
human teaching review against the carried rubric — live Svelte and React
routes at ordinary width and 768px, GPUI specimen source, and the
`g15.026` headless construction/axis evidence. **Five pages keep unchanged;
FieldSet and ValidationSummary received bounded Svelte/React specimen repairs.**
No component implementation, public API, contract, shared CSS, generated
catalogue, or GPUI source changed.

The seven human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; the mechanical totals remain
unchanged at A/A/A and `keep` for all seven.

## Verdict inventory

| Page | Verdict |
| --- | --- |
| `FieldSet` | keep — legend/description, two-column span, legendless gap, and multiple form sections teach grouping choices; Sv/Rc paired; Gp mirrors the grouping axes with renderer-owned gap evidence |
| `ValidationSummary` | keep — blocking and mixed-pending summaries point to real invalid/pending controls; link clicks stay inside the preview route and focus the target; Sv/Rc paired; Gp covers blocking, pending-only, mixed, and empty states |
| `CardRadioGroup` | keep — plan and instance choices teach live selection, disabled state, and size/density axes; pointer, disabled, and ArrowLeft gestures verified in Sv/Rc; Gp wires selection and mirrors the disabled/axis evidence |
| `ListContainer` | keep — ready slot composition, pagination, filters/batch affordances, and Loading/Empty/Error states are distinct; page and state changes verified in Sv/Rc; Gp mirrors the construction/state surfaces |
| `OrderBy` | keep — multi-field builder, direction/remove controls, icon trigger, disabled state, and axes are clear; Sv/Rc paired with live direction/remove/Escape paths; Gp mirrors open, disabled, and axis construction |
| `SelectionSummary` | keep — selected chips, removal, clear, single-item, and truncation states are distinct; live removal/clear verified in Sv/Rc; Gp adds empty, size, and clear evidence |
| `MediaThumbnail` | keep — image/video metadata, compact document/audio, loading, and contained-fit stories are distinct; Sv/Rc paired; Gp extends with kind fallback, aspect, states, and fit evidence |

## Repairs

- FieldSet’s first web group now uses the public `description` prop, matching
  the native description evidence and making group-level guidance visible.
- ValidationSummary’s blocking example now includes real invalid and pending
  `Field` controls with matching IDs and states.
- The ValidationSummary specimen intercepts those in-page links locally so the
  catalogue’s component hash route is preserved; the target scrolls into view
  and receives focus in both web runtimes.
- Focused Svelte and React tests cover the description and link-target/focus
  teaching paths.

## Live evidence

- Svelte: `http://localhost:4173`
- React: `http://localhost:4180` (the checked-in preview config owns port
  4180)
- At 768px, all seven Svelte and React specimen sections stayed within their
  content panes with no horizontal overflow.
- CardRadioGroup: plan selection, disabled option, and ArrowLeft roving
  selection verified in both runtimes.
- ListContainer: pagination and Ready/Loading/Empty/Error state changes
  verified in both runtimes.
- OrderBy: open, direction toggle, remove, and Escape/reopen paths verified in
  both runtimes. The native add-field select and its change path are present;
  the preview automation did not commit the OS-native option popup, so that
  gesture is not claimed here.
- SelectionSummary: chip removal and Clear verified in both runtimes.
- ValidationSummary: clicking Project name preserved the component route and
  focused the `project-name` input in both runtimes.
- GPUI source was read for all seven pages, and the carried `g15.026`
  headless evidence remains construction/axis evidence only. No GPUI specimen
  source changed.

## Changed routes for operator review

Changed Svelte routes: `field-set`, `validation-summary`
Changed React routes: `field-set`, `validation-summary`
Changed GPUI routes: none

The operator reviewed and approved all four changed Svelte and React routes on
2026-08-22 before PR #63 merged as `7e2cdb15`.

## Changed files

- `packages/svelte/preview/src/specimens/FieldSetSpecimen.svelte`
- `packages/svelte/preview/src/specimens/ValidationSummarySpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/FieldSetSpecimen.tsx`
- `packages/react/preview/src/gallery/specimens/ValidationSummarySpecimen.tsx`
- `packages/svelte/preview/test/g15-033-composition-forms-data-media.test.ts`
- `packages/react/preview/test/g15-033-composition-forms-data-media.test.tsx`
- `docs/roadmaps/g15/specimen-catalogue-audit.md`
- `docs/logs/2026-08/20260822-g15-033-composition-forms-data-media-review.md`

## Validation

- `bunx vitest run packages/svelte/preview/test/g15-033-composition-forms-data-media.test.ts` — passed (2)
- `bunx vitest run packages/react/preview/test/g15-033-composition-forms-data-media.test.tsx` — passed (2)
- `effigy catalogue:check` — passed
- `effigy check:svelte` — passed (0 errors; existing warnings only)
- `effigy react:build` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — passed

No `*-windowed`, `test:native-visual`, Jetstream, release, or workflow
selector ran. No screenshot gate is claimed.

## Operator checkpoint

Accepted. The operator approved the four changed Svelte/React routes and
directed merge. PR #63 merged as `7e2cdb15`.
