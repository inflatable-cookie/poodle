# g15.020 — Overloaded Examples: model connections and account lifecycle

Status: **complete** — PR #42 merged as `315ce2b9` on 2026-08-19;
operator explicitly waived the live visual checkpoint when directing merge
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Consumes: `g15.011` partial screening baseline
Depends on: `g15.034` (complete)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`

## Scope

The densest original cluster in the catalogue. Later licence review and axis
work already brought two pages within budget; this card preserves those pages
and curates the six that remain long or overloaded.

Catalogue families: `model-connections`, `account-lifecycle`.
### Pages this card owns (8)

- `LicenceActivation`
- `LicenceSeats`
- `LicenceStatus`
- `ModelCatalogueEditor`
- `ModelConnectionCard`
- `ModelConnectionPicker`
- `ModelConnectionSetup`
- `ModelPicker`

This list is exact and exhaustive: it is every page in these families whose
`Examples` view the audit measured as overloaded (10+ captioned examples) or
long (7–9). No other card owns these pages, and this card owns no others. If a
prerequisite card changes a page's count before this one runs, re-measure and
record the change — do not silently widen or narrow the set.

No component, contract, or public API change.

## Remeasured Baseline

Counts are visible example captions on current `main`, after `g15.034`.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| LicenceActivation | 5 | 5 | 5 | already curated; verify, do not churn |
| LicenceSeats | 6 | 6 | 6 | already curated; verify, do not churn |
| LicenceStatus | 10 | 10 | 10 | curate |
| ModelCatalogueEditor | 9 | 9 | 11 | curate and converge |
| ModelConnectionCard | 9 | 9 | 12 | curate and converge |
| ModelConnectionPicker | 8 | 8 | 10 | curate and converge |
| ModelConnectionSetup | 8 | 8 | 8 | curate |
| ModelPicker | 13 | 13 | 9 | curate and converge |

The two no-op pages remain in the owned set so the original partition stays
honest. They need no source edit unless verification finds real drift.

## Target Teaching Outline

Use these sections in this order. Captions may be polished for plain language,
but Svelte and React copy stays verbatim and GPUI teaches the same sequence.
Each section is one clearly bounded specimen surface. Several component
instances may share a surface only when they answer the same teaching question;
do not rebuild a prop matrix inside it.

| Page | Target sections |
| --- | --- |
| LicenceActivation | keep the current five: embedded account, external account, key, pending/disabled, host copy |
| LicenceSeats | keep the current six: mixed labels, unnamed, this machine, pending release, direct release, empty authority |
| LicenceStatus | active; in grace; use window expired; lease lapsed; clock refused |
| ModelCatalogueEditor | shown/hidden default; reorder and visibility; host-composed actions/metadata; loading; empty; unavailable/error |
| ModelConnectionCard | ready default; readiness/preference states; closed accessory/actions; open details with catalogue; narrow summary |
| ModelConnectionPicker | grouped default; search results; catalogue states; host marks/footer; narrow layout |
| ModelConnectionSetup | choose a connection; API-key configuration; auto-detected local route; OAuth in progress; local endpoint; validation/pending states |
| ModelPicker | cross-provider default; axis control forms; variants/emphasis; content visibility; empty/disabled states |

Preserve these contract-critical stories while consolidating:

- LicenceStatus keeps one distinct surface for every usability state. Coverage
  windows and both trust bases can be distributed through those five fixtures.
- ModelConnectionSetup visibly proves that a route with no required
  configuration skips the credential step.
- ModelPicker's existing contract coverage is consolidated, not deleted: the
  default fixture may cover grouping, provider marks, model-scoped axes, and a
  no-axis model; the axis section covers forced short-list and seven-level-list
  controls; focused tests retain cases that no longer deserve a catalogue
  surface.
- Async/error/empty examples may share one states surface when each instance is
  still visually distinct. Do not merge unrelated examples into an unlabelled
  slab.

## Goals

- [x] Every page in the group meets the parent's method.
- [x] Svelte and React stay identical; GPUI teaches the same set.
- [x] Removals are named, with contract coverage checked first.

## Evidence

- Add a focused structural regression for this exact eight-page set. It must
  assert the final ordered captions, paired Svelte/React equality, the 3–6
  budget, and the two verified no-op pages.
- Record the final GPUI caption order for all eight pages with the narrowest
  deterministic structural evidence available before `g15.026`; do not build
  the native page probe here.
- The batch log maps every removed caption to retained catalogue coverage,
  focused behavior evidence, or an explicit coverage note.

## Acceptance

Per the parent, including its operator-review checkpoint: **the changed pages
are reviewed live in the Svelte and React previews before this card is called
complete.** Unreviewed pages remain an explicit PR item.

Closeout exception: after the two review repair rounds passed the full card
gate, the operator explicitly directed the orchestrator to fix the remaining
issues and merge. That instruction waived this child's live visual checkpoint;
no live visual inspection is claimed.

## Writable Scope

- the specimen files for these families across Svelte, React, and GPUI
- focused specimen-caption/budget evidence for this exact eight-page set
- one August batch log

Do not edit components, component contracts, shared specimen shells, catalogue
navigation, generated scene infrastructure, or pages owned by another child.

## Validation

- focused curation regression
- `effigy test:parity`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run windowed, native-visual, conformance, or Jetstream
selectors.

## Stop Conditions

In addition to the parent stop conditions, stop if:

- preserving a contract-critical story requires a component or contract
  change;
- a page cannot reach the stated outline without hiding materially distinct
  behavior inside an unclear combined surface;
- the web runtimes need different section order or copy;
- GPUI cannot teach the same outline without the `g15.026` page probe;
- work escapes the exact eight-page set.

## Continuation

PR #42 merged after two orchestrator review rounds. `g15.021` is the next
curation child; it was not absorbed into this run.
