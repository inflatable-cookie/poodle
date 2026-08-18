# g15.011 — Human-Centred Specimen Catalogue Audit (August batch log)

Date: 2026-08-17 (revision 2: 2026-08-18)
Card: `docs/roadmaps/g15/011-specimen-catalogue-audit.md`
Handoff: `docs/handoffs/20260817-214451-g15-011-specimen-catalogue-audit.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-f6b446e9`
Branch: `t3code/specimen-catalogue-audit`

## Summary

Screened all 175 frozen catalogue entries as documentation across the two web
runtimes, proved the human-centred standard on Button, RangeSlider, and Tabs in
Svelte, React, and GPUI, and split the remaining work into bounded curation
cards.

**This is a partial delivery of `g15.011`, not its completion.** The card scopes
full Svelte, React, and GPUI pages; GPUI could not be rendered and its grades
are structural and provisional. `g15.026` is the completion child. The audit's
A–D grades are screening grades from measured signals; the rubric's teaching
judgment — does the first example show normal use, are the variants meaningful
— was applied to the three pilots, and each curation tranche applies it to its
own family.

Audit: `docs/roadmaps/g15/specimen-catalogue-audit.md`.
Outline: `docs/roadmaps/g15/specimen-plan-outline.md`.

## Grade Totals

| Runtime | A | B | C | D | n/a |
| --- | ---: | ---: | ---: | ---: | ---: |
| Svelte (live) | 81 | 33 | 52 | 9 | — |
| React (live) | 94 | 26 | 55 | 0 | — |
| GPUI (structural, provisional) | 100 | 68 | 6 | 0 | 1 |
| Worst of the three | 58 | 48 | 60 | 9 | — |

Dispositions: `keep` 56, `pilot-fix` 3, `curation-tranche` 116,
`contract/runtime-blocker` 0. Every entry carries a grade; none was skipped.
175 of 175 pages were measured live in both web runtimes.

## How Grades Were Measured

**Svelte and React are measured live**, page by page, for render, captions
across all four idioms, interaction (up to five controls clicked through the
real event tree, comparing the whole document), narrow layout at 768px, and
axis-pane content. Two supporting passes are static because they are questions
about source: the specimen-map inventory, and axis eligibility read from each
component's `$props()` block — 126 take `size`, 128 take `density`.

**GPUI is not measured live.** `packages/gpui/preview` is a binary crate and
`render_single_specimen` depends on `PreviewRoot`, `AppState`, and the
catalogue sidebar, so nothing outside `main.rs` can construct a specimen page.
Its grades are labelled provisional and structural, and no render, interaction,
or narrow-layout claim in the audit applies to it. `g15.026` builds the seam
and the probe that close this.

Defects are weighted: minor 1, major 2. Score 0 is A, 1 is B, 2+ is C. D is
reserved for pages that fail as documentation outright.

The click probe **nominates, it does not grade**. Each page it flagged was
checked against its specimen source before any defect was recorded: 14 render
controls with no handler at all, and six are not defects — a clipboard write
(`Code`), three hover or right-click surfaces (`Tooltip`, `HoverCard`,
`ContextMenu`), one wired control in a terminal state (`ErrorBoundary`), and
one navigation case (`TextLink`).

Seven measurements were discarded because a later pass contradicted them,
rather than reported: an apparent 47-page "empty Sizes tab" class (pages that
correctly omit `SpecimenLayout`); an apparent GPUI "no captions" class
(captions threaded through local helpers); an apparent set of caption-less
pages (a bare `<Eyebrow>` idiom the first probe did not read); a
`paneText === 0` rule that read panes full of unlabelled form controls as
empty; a pane-level interaction check that read every portalled overlay as
inert; a focus-change signal that would have cleared every page, since clicking
a button moves focus to it; and the click probe's own "20 inert pages", which
became 14 under source checking.

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
| Audit, outline, cards | 1600+ | 0 |

The nine pilot surfaces are 539 lines added against 1059 removed — a net
reduction of 520 lines of specimen source while adding an orientation example
and a tone the catalogue never showed.

## Review Response (revision 2)

The orchestrator requested changes on PR #36. All four items are addressed:

1. **Coverage did not support three-runtime grades.** React is now swept live
   with the same probe as Svelte — 175/175 pages, no timeouts. GPUI is labelled
   provisional with the blocker named, and `g15.026` cards the fix.
2. **A mechanically derived row was wrong.** `ToolCall`'s "(0 vs 4)" was the
   Svelte-versus-React caption count, not React's count — the detector read
   React's four groups correctly — but the difference was charged to React when
   its cause is Svelte's already-D-graded caption failure. Cross-runtime drift
   is now attributed to neither runtime when the Svelte page is hard-failed.
   The whole table is regenerated at the current head, retiring the stale
   pre-pilot Button and Tabs rows.
3. **The Button pilot taught a chevron as disclosure.** Both web pages now
   carry a stateful `ariaExpanded` trigger with its readout, matching GPUI, and
   a focused test asserts the distinction.
4. **The rollout was not dispatchable.** `g15.018` is a non-dispatchable parent
   with six bounded family children (`g15.020`–`025`), each sequenced and each
   carrying a live operator-review checkpoint. The same checkpoint is added to
   `g15.015`–`017` and `019`.

Two defects the re-measurement found in the pilot's own work: `Tabs` overflowed
its pane by 81px at 768px from a fixed `34rem` resize demo, now clamped; and
the Button disclosure gap above.

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
- `g15.018` — overloaded Examples (non-dispatchable parent), with six bounded
  family children `g15.020`–`g15.025`
- `g15.019` — GPUI specimen structure
- `g15.026` — the headless native probe that un-provisions the GPUI column

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

## Review Response (revision 3)

The orchestrator requested changes again. Three blockers, all addressed:

1. **The interaction signal produced known false defects.** Revision 2 scored
   "controls do nothing" as a major defect while its own prose admitted the
   probe cannot see clipboard writes, hover surfaces, or navigation. The probe
   is now a nomination only. Every one of the 20 flagged pages was checked
   against its specimen source: 14 wire no handler at all and score; six are
   recorded as `note —` and score nothing. Grades, totals, and dispositions
   were regenerated — Svelte A 76→81, React A 88→94.

   The rubric's teaching judgment — first example, variant meaning — is still
   not measured for 172 of 175 pages, so the artifact is now labelled a
   **mechanical screening baseline**, and an A is defined as "no measured
   defect" rather than "judged a good teaching page".

2. **`g15.011` was called complete while a required runtime was unaudited.**
   The audit's status is now `partial`, this log says the same, and `g15.026`
   is marked `g15.011`'s **completion child** rather than a follow-on. The card
   is not complete until the native third is measured.

3. **Tranche arithmetic did not reconcile and children had no lists.** The
   audit said 58, the parent said 50, the children summed to 48. All three are
   now 53, and each child carries an exact, exhaustive page list — one
   partition of the same 53 pages, so two workers cannot choose different
   subsets.
