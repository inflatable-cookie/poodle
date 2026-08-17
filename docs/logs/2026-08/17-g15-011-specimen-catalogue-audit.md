# g15.011 — Human-Centred Specimen Catalogue Audit (August batch log)

Date: 2026-08-17
Card: `docs/roadmaps/g15/011-specimen-catalogue-audit.md`
Handoff: `docs/handoffs/20260817-214451-g15-011-specimen-catalogue-audit.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-f6b446e9`
Branch: `t3code/specimen-catalogue-audit`

## Summary

Audited all 175 frozen catalogue entries as documentation, proved the
human-centred standard on Button, RangeSlider, and Tabs across Svelte, React,
and GPUI, and split the remaining work into five bounded curation cards.

Audit: `docs/roadmaps/g15/specimen-catalogue-audit.md`.
Outline: `docs/roadmaps/g15/specimen-plan-outline.md`.

## Grade Totals

| Runtime | A | B | C | D | n/a |
| --- | ---: | ---: | ---: | ---: | ---: |
| Svelte | 87 | 35 | 44 | 9 | — |
| React | 136 | 21 | 18 | 0 | — |
| GPUI | 116 | 52 | 6 | 0 | 1 |
| Worst of the three | 61 | 54 | 51 | 9 | — |

Dispositions: `keep` 60, `pilot-fix` 3, `curation-tranche` 112,
`contract/runtime-blocker` 0. Every entry carries a grade; none was skipped.

## How Grades Were Measured

Four passes, all reproducible:

1. Inventory from the three runtime specimen maps.
2. Axis eligibility read from each component's `$props()` block — 126 take
   `size`, 128 take `density` — compared with what its pages advertise.
3. A live sweep of all 175 pages in the running Svelte preview, with the
   `Sizes` and `Densities` tabs opened.
4. A live caption sweep across all four caption idioms in use.

Defects are weighted: minor 1, major 2. Score 0 is A, 1 is B, 2+ is C. D is
reserved for pages that fail as documentation outright.

Three static signals were discarded because the live passes contradicted them,
rather than reported: an apparent 47-page "empty Sizes tab" class (pages that
correctly omit `SpecimenLayout`), an apparent GPUI "no captions" class
(captions threaded through local helpers), and an apparent set of caption-less
pages (a bare `<Eyebrow>` idiom the first probe did not read).

## Headline Finding

Nine agent-surface pages render all 52 example captions blank. `SpecimenGroup`
takes `label`; those pages pass `title` and `description`, and Svelte drops
unknown props silently. Three independent measurements agree on the same 52
instances: the source scan, the live sweep, and `svelte-check`.

`check:svelte` covers `packages/svelte/install-smoke` and
`packages/svelte/components`, never `packages/svelte/preview`. Running
`svelte-check` there reports 428 errors — 348 from one generated-catalogue type
artifact, 52 from this defect, 28 other. That gate scope hole is why the class
shipped, and closing it is `g15.015`.

## Pilot Changes

### Button — `packages/svelte/preview/src/specimens/ButtonSpecimen.svelte`, `packages/react/preview/src/gallery/specimens/ButtonSpecimen.tsx`, `packages/gpui/preview/src/specimens/button.rs`

- 8 example groups to 6.
- `Variants` / `Danger tone` / `Success tone` — a variant x tone cross-product
  across three groups — becomes one tone row on one variant.
- That row adds `warning`, which `ButtonTone` defines and no runtime showed.
- Opens on a realistic action row (`Save changes` / `Cancel`) instead of a
  variant showcase.
- GPUI additionally folds its separate `Toggle` and
  `Disclosure trigger (aria_expanded)` sections into `States` and
  `Icons, disclosure, and icon-only`; the `aria_expanded` readout is kept as
  native evidence.

### RangeSlider — `RangeSliderSpecimen.svelte`, `RangeSliderSpecimen.tsx`, `range_slider.rs`

- Adds a `Vertical` section. The contract covers vertical orientation with its
  own sizing and styling rules and no page taught it.
- The two embedded-polarity sections merge into one.
- The `Sizes` pane drops from three controls per step (15 sliders) to one.
- GPUI drops `Custom min / max + step`, a duplicate of `Stepped`, and keeps
  `Positions`, which is the only evidence for how the between-fill window
  resolves at the extremes.

### Tabs — `TabsSpecimen.svelte`, `TabsSpecimen.tsx`, `tabs.rs`

- 18 example groups to 6 on the web; 13 to 8 on GPUI.
- Six `activeEdge` / `activeFill` permutation groups become one section. The
  two axes are variant-agnostic, so the product taught one idea six times.
- `bordered` is taught as the pair it is: above content, and flush for
  titlebars and toolbars.
- **GPUI: the `Sizes (xs -> xl)` and
  `Densities (compact / default / comfortable)` sweeps moved out of the page
  body into `SpecimenLayout` panes.** The native page was breaking the rule the
  pilot exists to prove; it now uses `specimen_layout` like the other 64.
- Removed: the hand-rolled collapse-toggle demo (composite behaviour, not a
  Tabs prop; vertical orientation is still taught) and the duplicate icon-tab
  groups. GPUI keeps `Full-width` and the native-only `Reorder drag states`,
  which is contract §4 evidence with no web equivalent.

Nothing removed was the sole evidence for a contract behaviour except the
collapse-toggle demo, which was not a contract behaviour.

## Source Cost

| Surface | + | - |
| --- | ---: | ---: |
| `TabsSpecimen.svelte` | 92 | 288 |
| `TabsSpecimen.tsx` | 90 | 216 |
| `tabs.rs` | 69 | 131 |
| `button.rs` | 69 | 187 |
| `range_slider.rs` | 63 | 94 |
| `RangeSliderSpecimen.svelte` | 69 | 50 |
| `RangeSliderSpecimen.tsx` | 51 | 31 |
| `ButtonSpecimen.svelte` | 18 | 31 |
| `ButtonSpecimen.tsx` | 18 | 31 |
| New preview tests + harness | 97 | 0 |
| Audit, outline, five cards | 1004 | 0 |

The nine pilot surfaces are 539 lines added against 1059 removed — a net
reduction of 520 lines of specimen source while adding an orientation example
and a tone the catalogue never showed.

## Operator Review

**Approved.** The operator reviewed all three reworked pages live in both web
previews on 2026-08-17 (Svelte on `:4188`, React on `:4189`) and accepted them,
including four flagged judgement calls: remaining tab-control density on the
Tabs page, removal of its collapse-toggle demo, GPUI Tabs staying at eight
sections, and sentence-style captions.

## Proposed Curation Tranches

Planned, orchestrator-review-required, not dispatched:

- `g15.015` — specimen caption integrity and the type-check gate hole
- `g15.016` — caption idiom convergence; two borrowed pages get their own
- `g15.017` — axis placement and evidence
- `g15.018` — overloaded Examples curation
- `g15.019` — GPUI specimen structure

## Rejected Machinery Stays Rejected

The live tab set is exactly `Examples · Sizes · Densities`. No `Conformance`
tab and no corpus projection wiring exists in any of the three previews; the
only repository matches for "conformance" are an accessibility caveat
sentence, a demo activity string, and a comment noting the retained headless
infrastructure. The specimen plan is an outline-level document with no schema,
codegen, generated adapter, or runtime consumer.

## Selectors Run

- `effigy catalogue:check`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy drift:handlers`, `effigy drift:events`, `effigy drift:adapter-manifests`
- `effigy test:components` (335 files, 2604 tests)
- `bunx vitest run --project svelte-preview` (25 tests, including the new
  pilot guards)
- `effigy docs:lint`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

No `*-windowed` selector, no `test:native-visual`, no `qa:jetstream`, and no
Jetstream selector was run. Jetstream remains program-deferred.
