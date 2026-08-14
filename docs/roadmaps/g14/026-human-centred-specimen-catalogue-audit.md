# g14.026 — Human-Centred Specimen Catalogue Audit

Status: blocked pending `g14.008`
Depends on: `g14.008`, `g14.025`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/009-cross-runtime-component-conformance.md`,
`../../specs/066-executable-component-conformance.md`

## Outcome

Freeze a human-centred specimen standard, inspect every catalogue page, and
compile bounded curation tranches. Pilot the standard on Button, RangeSlider,
and Tabs. Decide whether the adopted conformance corpus earns a separate
`Conformance` tab without turning the catalogue into another test report.

This card audits and proves the shape. It does not attempt a one-PR rewrite of
the entire catalogue.

## Goals

- [ ] Inventory every generated catalogue entry and its Svelte, React, GPUI,
      and deferred Jetstream specimen state.
- [ ] Grade every page for teaching value, duplication, dead interactions,
      runtime drift, narrow-layout behaviour, and axis placement.
- [ ] Define the minimum useful page shape:
      - `Examples` — default use, meaningful variants, important states, and
        composition; no Cartesian product
      - `Sizes` — control-size evidence only where size applies
      - `Densities` — density evidence only where density applies
      - `Conformance` — optional exhaustive case projection, only after an
        `adopt` or bounded `revise` verdict
- [ ] Define one renderer-neutral specimen plan for ordered tabs, sections,
      captions, and shared fixture references. Keep runtime component
      rendering local.
- [ ] Rework Button, RangeSlider, and Tabs with the operator as the pilot.
      Tabs must be treated as a documentation problem, not accepted merely
      because its pre-conformance page was restored.
- [ ] Compile the remaining catalogue into bounded family/tranche roadmap
      files with explicit operator-review checkpoints.

## Audit Rubric

For each component record:

- the first example answers “what is this and how do I normally use it?”
- variants are meaningfully distinct rather than a prop cross-product
- interactive controls work in the specimen
- loading, disabled, empty, error, and narrow states appear only when useful
- sizes and densities do not leak back into `Examples`
- Svelte and React structure/copy agree; GPUI teaches the same component where
  the active runtime supports it
- captions describe user-facing meaning, not internal fixture IDs
- complex examples may use a bounded runtime adapter without forcing their
  layout into a universal schema

## Shared-plan Boundary

Allowed shared data:

- tab and section IDs/order
- titles, captions, descriptions, and example IDs
- references to reusable fixtures or conformance cases
- axis eligibility and capture identity

Not allowed:

- a universal render tree
- executable behaviour or callbacks
- DOM, GPUI, or Jetstream objects
- arbitrary conditional expressions
- forcing every runtime-only teaching detail into the common plan

If the shared plan cannot express a complex page without becoming a scene
language, keep the page's bounded renderer adapter and share only its outline.

## Conformance-tab Decision

The `g14.008` verdict controls this branch:

- **adopt** — pilot a lazy `Conformance` tab that enumerates the full case
  corpus and clearly labels it diagnostic
- **revise** — include only the corrected corpus named by the verdict
- **reject** — omit the tab and retire projection-only catalogue wiring

The conformance tab never becomes the default tab and never contributes
examples to `Examples`, `Sizes`, or `Densities`.

## Deliverables

- complete page-by-page inventory with a curation grade and named defects
- specimen-plan contract and smallest useful schema/adapters
- operator-approved Button, RangeSlider, and Tabs pilot pages
- decision and evidence for the optional `Conformance` tab
- bounded rollout roadmap files for the remaining catalogue
- one August batch log with before/after screenshots and source-cost evidence

## Acceptance

- Every catalogue entry has an explicit disposition; no page is silently
  skipped because a runtime specimen is missing.
- The three pilot `Examples` views are concise, useful, and free of exhaustive
  size/density expansion.
- Shared structure changes reach Svelte, React, and GPUI without copying
  section metadata three times.
- Runtime adapters render real components; the plan contains no behaviour.
- Exhaustive cases, if exposed, live only under `Conformance`.
- The remaining work is split into reviewable tranches rather than one
  catalogue-wide rewrite.

## Stop Conditions

- The plan grows into the rejected universal scene/component IR.
- Human-facing examples become constrained by test-case serializability.
- One worker PR must redesign the whole catalogue to prove the approach.
- Jetstream parity is smuggled into this audit before backend admission.

## Writable Scope

- specimen-plan schema, focused codegen, generated adapters, and tests
- Button, RangeSlider, and Tabs specimens in Svelte, React, and GPUI
- preview tab/section primitives needed by the pilot
- catalogue inventory and new bounded roadmap files
- this roadmap, the g14 index, one August batch log, and `PAPERCUTS.md`

Do not change component public APIs, component semantics, the executable case
corpus, normalized observations, Jetstream runtime code, or release workflows.

## Validation

- focused specimen-plan/codegen tests
- `effigy catalogue:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy test:components`
- `effigy test:parity`
- `effigy docs:check`
- `git diff --check`

Use live Svelte and React previews for operator review. GPUI evidence must use
a non-focus-stealing path. Do not run any `*-windowed` conformance selector.
