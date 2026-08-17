# g15.011 — Human-Centred Specimen Catalogue Audit

Status: **blocked** — orchestration hold; `g15.007` is next
Depends on: `g15.001` (measured roster); carries forward `g14.026` with its
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
- [ ] Remaining work split into reviewable curation tranches (new bounded
      cards), not one catalogue-wide rewrite.

## Stop Conditions

- The shared specimen plan grows into a universal scene/component language.
- Jetstream parity is smuggled into this audit before backend admission.
- The audit turns specimens into an exhaustive case matrix.

## Writable Scope

- specimen-plan outline, curation grades, pilot pages, new curation cards
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy catalogue:check`
- `effigy check:svelte`, `effigy react:build`, `effigy check:gpui`
- `effigy docs:check`
- `git diff --check`

Use live Svelte and React previews for operator review. Never run a
`*-windowed` conformance selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
