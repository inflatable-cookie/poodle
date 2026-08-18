# g15.011 — Human-Centred Specimen Catalogue Audit

Status: **partial** — screening baseline and three pilots delivered; completion
requires the native probe (`g15.026`) and the screen-clear human review
children (`g15.028`–`g15.033`)
Depends on: `g15.001` (measured roster), `g15.010` (final GPUI specimen set);
carries forward `g14.026` with its
rubric and bounded shared specimen-plan boundary intact
Governing refs: `release-baseline-roster.md`,
`../../roadmaps/g14/026-human-centred-specimen-catalogue-audit.md`,
`../../contracts/001-working-rules.md`

## Outcome

Execute the carried human-centred specimen catalogue audit on the frozen
175-component roster: inventory every catalogue entry, grade each page for
teaching value and runtime drift, and compile bounded curation tranches.
Pilot the rubric on Button, RangeSlider, and Tabs. The audit is kept separate
from exhaustive conformance evidence; specimens teach humans and never
become test reports.

## Scope

- full Svelte, React, and GPUI catalogue pages for all 175 components
- pilot rework of Button, RangeSlider, and Tabs
- one renderer-neutral specimen plan (outline-level only)

## Audit Record

Write the complete inventory to
`docs/roadmaps/g15/specimen-catalogue-audit.md`. Use one row per component
with separate Svelte, React, and GPUI grades, named cross-runtime drift, and
one disposition. A missing or unusable runtime page is graded, not skipped.

Use this fixed grade vocabulary:

- **A — ready:** concise teaching page, meaningful interaction, and no named
  curation defect.
- **B — usable:** teaches the component but has a small named presentation,
  copy, interaction, or runtime-alignment defect.
- **C — curate:** overloaded, unclear, misleading, substantially drifted, or
  missing important teaching value.
- **D — missing/broken:** no real specimen, dead primary interaction, or page
  cannot be used as documentation.

Use one disposition: `keep`, `pilot-fix`, `curation-tranche`, or
`contract/runtime-blocker`. Record evidence briefly; do not paste screenshots
or source dumps into the table.

Write the shared outline to
`docs/roadmaps/g15/specimen-plan-outline.md`. It may describe ordered tabs,
sections, captions, example IDs, reusable fixture references, and axis
eligibility. It is a planning artifact only: this card adds no schema,
codegen, generated adapter, or runtime consumer.

## Goals

- [ ] Every generated catalogue entry has an explicit grade and disposition;
      no page silently skipped because a runtime specimen is missing.
- [ ] Three pilot `Examples` views concise and free of exhaustive size/density
      expansion.
- [ ] Svelte and React structure/copy agree; GPUI teaches the same component
      where the active runtime supports it.
- [ ] Confirm the rejected corpus projection is absent from the catalogue.

## Acceptance

- [ ] Page-by-page inventory with curation grades and named defects.
- [ ] Operator-approved pilot pages for Button, RangeSlider, and Tabs.
- [ ] Evidence that no `Conformance` projection tab exists.
- [ ] Remaining work split into reviewable curation and screen-clear review
      tranches (new bounded cards), not one catalogue-wide rewrite.
- [ ] The operator reviews the three live pilot pages before the worker calls
      them approved. Unreviewed pages remain an explicit PR item.

## Continuation

The delivered A–D table is a mechanical screening baseline, not the completed
human-centred verdict. `g15.026` replaces the provisional GPUI column with
live headless evidence. `g15.027` is a non-dispatchable parent whose six exact
children (`g15.028`–`g15.033`) apply the teaching rubric to the 56 pages that
screened clear and therefore do not appear in a defect-led curation tranche.
This card completes only after both lanes land.

## Stop Conditions

- The shared specimen plan grows into a universal scene/component language.
- Jetstream parity is smuggled into this audit before backend admission.
- The audit turns specimens into an exhaustive case matrix.

## Writable Scope

- specimen-plan outline, curation grades, pilot pages, new curation cards
- one August batch log with audit totals, pilot changes, source-cost evidence,
  and the operator-review state
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy catalogue:check`
- `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`
- `effigy docs:check`
- `git diff --check`

Use live Svelte and React previews for operator review. Never run a
`*-windowed` conformance selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
