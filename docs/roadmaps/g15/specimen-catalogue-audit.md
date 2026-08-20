# g15 — Human-Centred Specimen Catalogue Audit

Status: **partial** — mechanical screening baseline plus live measurement on
all three runtimes. Human teaching judgment has been applied to the three
pilots and to 30 screen-clear pages (`g15.028`, `g15.029`, `g15.030`); 26
screen-clear pages and the remaining defect-led rows still await it.
Date: 2026-08-20 (revision 10 — `g15.030` foundation-layout review records a
GPUI `ResizeHandle` keyboard/focus contract gap as `contract/runtime-blocker`;
revision 9 — `g15.039` closed the DateTimeZonePicker nested-layer
pointer blocker; revision 8 was the `g15.029` review-round totals and D
correction; revision 7 recorded the date/time family review; revision 6 closed
SegmentedControl native option parity via `g15.038`; revision 5 measured the
GPUI column live via the `g15.026` probe; first pass 2026-08-17)
Card: `docs/roadmaps/g15/011-specimen-catalogue-audit.md`
Handoff: `docs/handoffs/20260817-214451-g15-011-specimen-catalogue-audit.md`
Governing refs: `release-baseline-roster.md`, `specimen-plan-outline.md`,
`../g14/026-human-centred-specimen-catalogue-audit.md`,
`../../contracts/001-working-rules.md`

## What Was Audited

All 175 catalogue entries in the frozen v0.2.0 denominator — the 174 portable
components in `packages/codegen/fixtures/preview-catalogue.json` plus the
web-only `MeterSurface` from `component-registry.ts`. Every entry carries a
separate Svelte, React, and GPUI grade, named defects, and one disposition. No
entry is omitted for having a missing or unusable runtime page.

The audit reads specimens as documentation. A green board does not make a page
useful, and a page that looks plausible in source is not proof it renders.

## What Was Measured, And How

**Svelte and React are measured live.** Every one of the 175 pages was loaded
in the running preview for that runtime and probed for the four things the
carried rubric asks about:

| Signal | Method |
| --- | --- |
| renders | page loads, is not the "specimen not yet available" placeholder, and its Examples pane has height, text, or controls |
| captions | every caption idiom read from the DOM — `SpecimenGroup`, bare `<Eyebrow>`, `<section><h3>`, and React's `AudioSpecimenGroup` — counting named against blank |
| interaction | up to five controls inside the specimen body clicked through the real event tree, comparing the whole document before and after. **This nominates, it does not grade** — every flagged page was then checked against its specimen source and classified (see finding 2) |
| narrow layout | viewport reduced to 768px, recording how far the pane's content overflows its own width |
| axis panes | the `Sizes` and `Densities` tabs opened and their content measured |

The interaction probe is scoped to the specimen body, not the page, so it
cannot click the layout's own tab bar and measure the navigation it caused.

Two supporting passes are static because they are questions about source, not
about a rendered page: the specimen-map inventory, and axis eligibility read
from each component's `$props()` block — 126 components take `size`, 128 take
`density`.

**GPUI is now measured live for construction, headlessly.** The `g15.026`
probe (`effigy probe:gpui-specimens`) mounts the production `PreviewRoot` on
GPUI's in-memory test platform — the in-binary `TestAppContext` seam, no
`lib.rs` — and walks the canonical 174-entry registry directly at a
768px-wide viewport. Every route must paint a real specimen card rather than
the `missing_specimen` fallback, and every `Sizes` or `Densities` tab the
mounted page advertises is clicked through the real pointer event path with
its pane asserted to paint. The measured result: **174/174 routes construct,
none reach the fallback, and all 126 `Sizes` and 127 `Densities` tabs open
their panes.** `MeterSurface` remains the single native `n/a`.

The GPUI column therefore grades what source proves (dispatch reachability,
caption presence, `specimen_layout` usage) plus live construction and
axis-pane navigation. The probe does not judge copy, visual quality, arbitrary
component interactions, or horizontal overflow, and no grade here asserts
those: **interaction-liveness and narrow-layout claims in this document still
cover only Svelte and React.**

Measurements that a later pass contradicted were discarded rather than
reported. Seven did: an apparent 47-page "empty Sizes tab" class was pages that
correctly omit `SpecimenLayout`; an apparent GPUI "no captions" class was
captions threaded through local helpers; an apparent set of caption-less pages
used a bare `<Eyebrow>` idiom the first probe did not read; a
`paneText === 0` rule read panes full of unlabelled form controls as empty; a
pane-level interaction check read every portalled overlay as inert; a
focus-change signal would have cleared every page, since clicking a button
moves focus to it; and the click probe's own "20 inert pages" became 14 once
each page was checked against its source.

## Grades, And What They Do Not Say

The card's fixed vocabulary:

- **A — ready:** concise teaching page, meaningful interaction, no named
  curation defect.
- **B — usable:** teaches the component, one small named defect.
- **C — curate:** overloaded, unclear, misleading, substantially drifted, or
  missing important teaching value.
- **D — missing/broken:** no real specimen, dead primary interaction, or the
  page cannot be used as documentation.

**Screening grades are the starting point.** They are computed from measured
signals — caption rendering, example count, axis eligibility, narrow overflow,
and a source-checked interaction verdict. They do not by themselves assert
that a page's first example teaches normal use or that its variants are
meaningfully distinct. A later human review may change a grade or disposition
in the existing row; it does not add a second table.

A person has now answered those questions for the three pilots (Button,
RangeSlider, Tabs) and for the 30 screen-clear pages owned by `g15.028`–
`g15.030`. The remaining 26 screen-clear pages still sit in `g15.031`–
`g15.033`. Every child carries a live operator-review checkpoint for changes.

So read an **A** that has no human verdict as "no measured defect", not as
"judged a good teaching page". A page can screen A and still open on a prop
showcase. Read an **A** that names a `g15.028`–`g15.030` human verdict as the
reviewer's keep. **D** remains the dead-primary-interaction or unusable-as-
documentation grade even when the defect is a component blocker rather than a
caption hole.

Defects are weighted. A minor defect (one missing axis, a hand-rolled caption
idiom, narrow overflow) scores 1; a major one (captions that do not render, an
overloaded `Examples`, an advertised tab that renders nothing, controls
confirmed unwired at source) scores 2. Score 0 is A, 1 is B, 2 or more is C. D
is reserved for pages that fail as documentation outright. Observations that
are not defects — a clipboard write, a hover surface — are recorded on the row
as `note —` and score nothing.

A cross-runtime caption-count difference is attributed to neither runtime when
the Svelte page is itself hard-failed: the difference *is* that defect, already
graded, and charging React for it marked correct pages down.

Dispositions used in the rows: `keep`, `pilot-fix`, `curation-tranche`,
`curation-complete`, `verified-no-op`, or `contract/runtime-blocker`.
`curation-complete` and `verified-no-op` name the closing card in the row
(`g15.020` today).

## Totals

Mechanical recount of the 175 inventory rows at this revision, after the
`g15.030` ResizeHandle GPUI keyboard/focus blocker. These numbers are the rows,
not a pre-pilot or pre-curation baseline.

| Runtime | A | B | C | D | n/a |
| --- | ---: | ---: | ---: | ---: | ---: |
| Svelte (live) | 89 | 33 | 44 | 9 | — |
| React (live) | 102 | 26 | 47 | 0 | — |
| GPUI (headless render + structural) | 102 | 66 | 6 | 0 | 1 |
| **Worst of the three** | **65** | **49** | **52** | **9** | — |

| Disposition | Count |
| --- | ---: |
| `keep` | 55 |
| `pilot-fix` | 3 |
| `curation-tranche` | 108 |
| `curation-complete` | 6 |
| `verified-no-op` | 2 |
| `contract/runtime-blocker` | 1 |

175 of 175 pages were measured live in both web runtimes. The `g15.028`
contract/runtime blocker on GPUI `SegmentedControl` is closed by `g15.038`.
The `g15.029` web `DateTimeZonePicker` nested-layer pointer blocker is closed
by `g15.039`: a portalled timezone option now commits through the composite
dismiss stack, and a genuine outside press still dismisses the whole picker
in one gesture. Sv/Rc return to A; disposition is `keep`.
The `g15.030` review records a new GPUI `ResizeHandle` contract/runtime
blocker: the render path wires drag only and does not project focus,
keyboard-step, or `aria_value_*` semantics from the contract
(`docs/contracts/components/resize-handle.md` §5–6). Sv/Rc specimen repairs
land in PR #55; the native renderer gap is routed separately and is not
hidden as specimen work.

`MeterSurface` is the single `n/a` on GPUI — web-only by fixed decision
(spec 068), with no native counterpart. It is still graded on the two runtimes
it has.

## What The Audit Found

### 1. Nine pages where no caption renders — the original D class

`SpecimenGroup` accepts one caption prop, `label`. Nine agent-surface pages
pass `title` and `description` instead. Svelte drops unknown props silently,
so all 52 example captions render as empty `Eyebrow` elements, and the
authored explanations — which are good, specific, and describe exactly what
each example teaches — never appear.

Three independent measurements agree on the same 52 instances: the source scan,
the live sweep (nine pages, every caption blank), and `svelte-check`.

The cause is a gate scope hole. `check:svelte` covers
`packages/svelte/install-smoke` and `packages/svelte/components`. It has never
covered `packages/svelte/preview`, where every specimen lives. Running
`svelte-check` there reports 428 errors: 348 from one generated-catalogue type
artifact, 52 from this defect, and 28 others.

The React versions of the same nine pages pass `label` correctly. This is
Svelte-only, and it is the one class where the catalogue actively lies about
being fine.

### 2. Fourteen pages whose controls are not wired at all

The click probe flagged 20 pages where up to five controls changed nothing in
the document, in **both** web runtimes. That result is a review lead, not a
grade: a clipboard write, a hover surface, and a wired control already in its
terminal state all look identical to it, and cross-runtime agreement proves
nothing when both previews share one specimen design.

Every flagged page was therefore checked against its specimen source. The
verdicts:

| Verdict | Pages | What the source shows |
| --- | ---: | --- |
| **dead** — scored as a defect | 14 | the specimen renders the control and wires no handler to it |
| outside the DOM | 1 | `Code` — the component owns the copy button and calls `navigator.clipboard.writeText` |
| wrong modality | 3 | `Tooltip`, `HoverCard` open on hover; `ContextMenu` on right-click |
| wired, terminal state | 1 | `ErrorBoundary` — "Throw again" sets `shouldThrow = true`, but the boundary was already showing its error |
| navigation | 1 | `TextLink` — the anchors navigate; the control clicked is an explicit `onClick={() => undefined}` demo |

The 14 with no handler at all:

`IconButton`, `Toolbar`, `PageHeader`, `FormActions`, `FormLayout`,
`DetailShell`, `RemediationBanner`, `UiPresentationProvider`, `ListGrid`,
`MetaBar`, `MetaItem`, `InlineListSection`, `AgentMessage`, `Callout`.

A static scan agrees independently: 122 Button-family instances across 24
Svelte specimen files carry no handler, no `type="submit"`, no `href`, and no
state prop — `AppHeader` 20 of 20, `Toolbar` 15 of 15, `PageHeader` 14 of 14.
These are pages that look plausible and do nothing when a reader tries them,
which is exactly the failure the handoff asked the audit to catch.

The six non-defects are recorded on their rows as `note —` and score nothing.
They are kept in the table because the next probe will flag them again, and the
next reader should not have to re-derive why they are fine.

### 3. Four caption idioms, one of which rendered nothing

The dispatch baseline now has 27 paired routes with a divergent or missing
caption shell: 13 Svelte pages use bare `<Eyebrow>` caption chrome, 13 audio
pages use direct `<section><h3>` in Svelte and React's separate
`AudioSpecimenGroup title=`, and `SettingsShell` has no example captions in
either web runtime. The remaining pages use `SpecimenGroup label=`. These
idioms differ in spacing, weight, and surface treatment, so pages that should
look like siblings do not. The divergence is what let the now-fixed g15.015
caption-prop failure hide in plain sight.

### 4. Size and density matrices in the main view

The twelve audio components put their full size **and** density sweeps into the
page body. React has no `Examples`/`Sizes`/`Densities` tabs for them at all —
`AudioSpecimenPage` is a plain grid and `AudioAxes` appends the two matrices as
ordinary sections. The native pages do the same through
`poodle_render::audio_specimens`'s `axis_groups`. Only Svelte puts them in
tabs.

This is the specific thing the operator asked not to happen, and it is
currently the majority position for that family across runtimes.

### 5. Missing axis evidence where the axis applies

126 components take `size` and 128 take `density`. Seventeen pages show no
`Sizes` evidence for a component that takes `size`, and eighteen show no
`Densities` evidence for one that takes `density` — most of the overlay family,
all five licence and update surfaces, and a handful of others.

Five pages have the opposite problem and advertise an axis tab for a component
with no such prop: `Avatar`, `Tooltip`, `ListCardCounter`, `PickerShell`, and
`MeterSurface`. `MeterSurface` also advertises both tabs and renders them
empty — the only genuinely empty tab in the catalogue.

### 6. Overloaded Examples

Fifty-three pages carry an overloaded or long `Examples` view — ten or more
captioned examples, or a 7–9 view whose length is a prop cross-product rather
than a set of distinct things worth seeing. `g15.018` partitions exactly those
53 across six family children.

### 7. Six pages that overflow a narrow viewport

At a 768px window, six pages render content wider than their own pane:
`EmbedInput` (582px), `LicenceActivation` (345px), `Icon` (119px),
`EnvelopeEditor` (32px), `AppHeader` (31px), and — before the pilot fixed it —
`Tabs` (81px, from a fixed `34rem` resize demo now clamped to
`min(34rem, 100%)`).

### 8. Two pages that teach a different component

`ListCardCounter` renders `ListCardSpecimen` and `MetaItem` renders
`MetaBarSpecimen`. The page title names one component; the content teaches
another.

### 9. GPUI structure

Every component in the active cohort has a native page — `g15.010` closed the
last gap. What those pages *show* still differs: 59 native pages have no
`Sizes`/`Densities` panes although their component takes that axis, 12 more
keep their axis matrices inside `Examples`, and six render their examples with
no captions at all. A post-`g15.017` readiness recheck also found three pages
that advertise an unsupported axis through the native helper. `g15.019`
therefore owns 74 axis corrections plus the six overlapping caption repairs.

PR #40 completed those 74 corrections and six caption repairs. Its full caller
audit also found two pre-existing axis-domain gaps outside that denominator:
EmptyState's native renderer ignores its two-value `size`, while Icon's native
size domain has only `sm`/`md`/`lg` and its renderer ignores `density`. Their
fake panes were removed and the gaps returned for planning. Native layout
mechanics are runtime-owned and are not a defect; absent or fabricated evidence
is. These structural findings predate the live probe; `g15.026` has since
confirmed that all 174 pages — including every corrected axis pane — construct
and navigate headlessly. The probe adds no caption or axis-domain judgment of
its own.

The operator resolved the authority on 2026-08-18: `g15.034` owns a clean
pre-v1.0 migration. EmptyState keeps only its real `default | compact` size,
Icon gains all five real sizes and loses dead density, and each specimen shell
must consume the component's explicit ordered domain.

## What The Audit Did Not Find

- **No `Conformance` tab, anywhere.** The live tab set is exactly
  `Examples · Sizes · Densities` on all 175 pages in both web runtimes. A
  repository-wide search for conformance or corpus projection wiring in the
  three previews returns three incidental matches: a sentence in
  `accessibility.ts` about assistive-technology claims, a demo activity string
  in `component-docs.ts`, and a comment in `headless_driver.rs` noting the
  retained headless infrastructure. `g14.021`'s removal holds.
- **No unrendered pages in any active runtime.** All 175 pages render in both
  web runtimes, and the `g15.026` probe constructs all 174 portable GPUI
  routes: none falls through to the "specimen not yet available" placeholder
  (`missing_specimen`) anywhere. Fourteen
  pages do render controls that are not wired — see finding 2 — but the pages
  themselves are alive.
- **The live web sweep found no contract or semantic defects.** The later
  `g15.019` native caller audit did expose the two axis-domain gaps above;
  their clean breaking repair is ready as `g15.034`.

## Corrections From Earlier Revisions

Revision 1 was measured with a Svelte-only sweep and static React and GPUI
inspection. Four results changed:

- **React is now measured live**, not inferred from source. Its grades moved
  from A 136 / B 21 / C 18 to A 94 / B 26 / C 55, almost entirely because the
  interaction and narrow-layout signals did not exist before.
- **`ToolCall` React was wrongly marked down.** Its "(0 vs 4)" evidence was the
  Svelte-versus-React caption count — the detector read React's four groups
  correctly — but the difference was charged to React when its cause is
  Svelte's already-D-graded caption failure. Cross-runtime caption drift is now
  attributed to neither runtime when the Svelte page is hard-failed, and React
  `ToolCall` grades A.
- **Button and Tabs rows were stale.** The table is regenerated at the current
  head, so both describe the reworked pages. Both pilots now grade A, A, A and
  A, B, B respectively.
- **GPUI grades are now labelled provisional** rather than presented alongside
  live-measured ones without qualification, and `g15.026` is named as the
  native completion lane rather than an optional follow-on.
- **The interaction result is now source-checked, not heuristic.** Revision 2
  scored "20 pages whose controls do nothing" as a defect while its own prose
  admitted the probe cannot see clipboard writes, hover surfaces, or
  navigation. Each flagged page was checked against its specimen source; 14 are
  genuinely unwired and six are not defects at all.
- **The grades are labelled as screening grades.** They do not assert the
  rubric's teaching judgment, which a person applied to the three pilots,
  defect-led tranches apply to their families, and `g15.028`–`g15.033` apply
  to the 56 pages that screened clear.

Revision 5 replaces the provisional GPUI basis with the live `g15.026` result:

- **GPUI construction is live-measured, not provisional.** All 174 portable
  routes mount through the production preview root on the in-memory test
  platform, paint a real specimen card, and open every advertised axis pane
  through pointer input (126 `Sizes`, 127 `Densities`). No route fell back and
  no grade changed — the probe exposed no construction defect. The two known
  native blockers in the release-gap register (Stepper selection re-run,
  UiPresentationProvider cascade) are interaction defects, not construction
  defects, and keep their existing owners.

Revision 8 reconciles the published totals with a mechanical recount of the
175 rows, adds the `curation-complete` and `verified-no-op` dispositions the
rows already used, and records `DateTimeZonePicker` as D on both web runtimes
for a dead primary pointer workflow.

Revision 9 closes that blocker via `g15.039`. DateTimeZonePicker returns to
A/A/A `keep` after the nested TimeZoneSelect portal joined the composite
dismiss stack. Totals are recounted from the rows.

## Pilot Findings

The three pilots were chosen to be representative, and they were. Their rows in
the table carry the mechanically measured defects; these are the judgments the
mechanical pass cannot make.

**Review state: approved.** The operator reviewed all three reworked pages in
the live Svelte and React previews on 2026-08-17 and accepted them, including
the four judgement calls flagged for them: the remaining tab-control density on
the Tabs page, the removal of its hand-rolled collapse-toggle demo, GPUI Tabs
staying at eight sections to keep `Full-width` and the native-only reorder drag
states, and the sentence-style captions.

### Button

- `Variants`, `Danger tone`, and `Success tone` are a variant × tone
  cross-product spread across three groups — nine buttons teaching one idea.
- The contract's `ButtonTone` set is `default | danger | success | warning`.
  The page shows danger and success and omits warning, so the cross-product is
  simultaneously redundant and incomplete.
- No example answers "how do I normally use this?" The page opens on a variant
  showcase.

### RangeSlider

- The five `Examples` groups are reasonable and each earns its place.
- The `Sizes` pane renders three sliders per size step — fifteen sliders. That
  is an exhaustive matrix inside the tab that is supposed to replace one.
- The contract covers vertical orientation with its own sizing and styling
  rules. Neither web page shows it.
- The GPUI page carries one example the web pages do not
  (`Custom min / max + step`), so the three runtimes teach slightly different
  sets.

### Tabs

- 18 example groups, and more than seventy tab controls on one page. Six of
  them are `variant × activeEdge × activeFill` permutations.
- Several captions describe interactions the reader must discover
  ("drag the handle", "hover the active tab", "click to toggle orientation")
  without the page making that affordance obvious.
- The GPUI page has no `SpecimenLayout`: its `Sizes (xs → xl)` and
  `Densities (compact / default / comfortable)` sweeps are sections in the main
  body, so the pilot's axis rule is violated on the native side of the same
  component.
- Tabs is itself a documentation and navigation component. Restoring its
  pre-conformance page did not make it good.

### What the pilots changed, and what the re-measurement shows

All three pilot pages now grade A on both web runtimes except `Tabs`, which
holds a B for the GPUI page's remaining structure. The pilot's own narrow-layout
defect — an 81px overflow from a fixed `34rem` resize demo — was found by this
audit's own probe and fixed.

The Button pilot originally captioned a row "Icons, disclosure, and icon-only"
while rendering only `chevron`. The contract separates the visual indicator
from `ariaExpanded`, the disclosure state a screen reader hears, and only GPUI
taught it. Both web pages now carry a stateful `ariaExpanded` trigger with its
readout, and a focused test asserts the distinction rather than accepting a
chevron as evidence.

## Continuation Runway

The 108 `curation-tranche` entries group into bounded, reviewable families.
Each is a planned card requiring orchestrator review before dispatch, and each
that changes specimen presentation carries a live operator-review checkpoint.

| Card | Family | Scale |
| --- | --- | ---: |
| [`g15.015`](015-specimen-caption-integrity.md) | Caption integrity + the type-check gate hole that hid it | 9 pages, 52 captions |
| [`g15.016`](016-specimen-idiom-convergence.md) | One caption idiom; two borrowed pages get their own | 29 pages |
| [`g15.017`](017-specimen-axis-placement.md) | Axis matrices out of the main view; axis evidence where the prop exists | 12 audio + ~22 others |
| [`g15.018`](018-overloaded-examples-curation.md) | Overloaded `Examples` — **parent, not dispatchable** | 53 pages |
| ↳ [`g15.020`](020-curate-model-connection-licence.md)–[`g15.025`](025-curate-collections-navigation-tail.md) | six bounded family children, one exact page list each; complete through PR #49 | 6–11 each, 53 total |
| [`g15.019`](019-gpui-specimen-structure.md) | Native axis panes and captions | complete — 74 axis + 6 caption corrections; two axis-domain gaps returned |
| [`g15.034`](034-component-specific-specimen-axis-domains.md) | Exact component domains and truthful specimen-axis evidence | complete — PR #41 |
| [`g15.026`](026-native-specimen-probe.md) | The headless native probe that un-provisions the GPUI column | 174 pages |
| [`g15.027`](027-screen-clear-human-review.md) | Human teaching review for mechanically clear pages — **parent, not dispatchable** | 56 pages |
| ↳ [`g15.028`](028-review-foundation-controls-entry.md)–[`g15.033`](033-review-composition-forms-data-media.md) | six bounded family children, one exact page list each | 7–14 each, 56 total |

Ordering matters: `g15.015` first, because it closes the gate that let the
worst class ship. `g15.017` before `g15.019`, because the native axis work
depends on `audio_specimens` separating its axis groups.

**`g15.011` has two completion lanes, not one.** `g15.026` has replaced the
provisional GPUI column with live headless evidence: 174/174 routes construct
and every advertised axis pane navigates. `g15.028`–`g15.033` apply
the human teaching rubric to the 56 pages that screened clear and therefore do
not belong to a defect-led tranche. The card is complete only when both lanes
land; until then this artifact is the partial baseline named at the top.

The interaction probe nominated 20 pages. Source review confirmed 14 as
unwired; those pages sit inside the defect-led family tranche that already owns
them. The other six are recorded as non-defect notes and stay in the
screen-clear review partition where applicable.

## Per-Component Inventory

Grades are per runtime: **Sv** Svelte (live), **Rc** React (live),
**Gp** GPUI (headless render + structural — see the measurement note). Evidence
names the defects that decided the grade; a row with no named defect is A.

† GPUI grades combine source structure with the `g15.026` live construction
and axis-navigation result; they carry no interaction or narrow-layout signal.

### Actions & selection — Foundations (12)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Button` | A | A | A | pilot-fix | no named defect |
| `Checkbox` | A | A | A | keep | human verdict (`g15.028`): keep — live default trio teaches normal use; Sv/Rc paired verbatim; Gp mirrors all three sections live |
| `CollapseToggle` | A | A | A | keep | human verdict (`g15.028`): keep — four live directions with state readout plus Disabled; all runtimes agree |
| `ConfirmAction` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `IconButton` | C | C | A | curation-tranche | **Sv:** controls do nothing — specimen wires no handler on any IconButton; takes `density`, but the page shows no Densities evidence · **Rc:** controls do nothing — specimen wires no handler on any IconButton; takes `density`, but the page shows no Densities evidence |
| `Radio` | A | A | A | keep | human verdict (`g15.028`): keep — live three-option group plus States; Gp matches (custom-color hex is fixture data, not copy) |
| `RadioGroup` | A | A | A | keep | human verdict (`g15.028`): keep — vertical/horizontal/disabled/custom color all live; Gp adds a bounded disabled-option visual |
| `SegmentedControl` | A | A | A | keep | human verdict (`g15.028`) plus `g15.038` closeout: **Gp:** icon-only options now use a dedicated `SegmentedControlOption` (icon, icon-only, title, accessible-name fallback) through shared render and a live Effects/Instruments specimen section · **Sv/Rc:** keep — live default, disabled option, content fit, icon-only, fully disabled |
| `SplitButton` | C | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 7 captioned examples |
| `Switch` | A | A | A | keep | human verdict (`g15.028`): keep — live trio, States, custom colors, dual labels/tones; Gp mirrors all four sections |
| `ToggleGroup` | A | A | A | keep | human verdict (`g15.028`): keep after Gp specimen repair — "Multiple selection" was inert (node tier wires `on_activate` only when a handler exists) with a hardcoded readout; wired live with a real readout, and the inert static "Allow deactivation" section removed. Sv/Rc unchanged |
| `TriStateSwitch` | B | A | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Text & value entry — Foundations (15)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `CodeInput` | A | A | A | keep | human verdict (`g15.028`): keep after Gp specimen repair — static "Partial (3 of 6)", "Complete", and "Numbers only" fill-state sections duplicated what the live default demonstrates; removed and reordered to the web section order (9 → 6 sections). Sv/Rc unchanged |
| `ColorPicker` | A | A | A | keep | human verdict (`g15.028`): keep — basic/swatches/alpha/default-open/preview-only/disabled; Gp mirrors all six with live open and value state |
| `DragNumberField` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 10 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `EditableLabel` | A | A | A | keep | human verdict (`g15.028`): keep after Gp specimen repair — 10 sections exceeded the outline budget; removed the "Display mode" and "Flush variant (editing)" duplicates, leaving the web section set plus the renderer-owned live editing example. Sv/Rc unchanged |
| `EmbedInput` | B | B | B | curation-tranche | **Sv:** overflows its pane by 582px at a 768px viewport · **Rc:** overflows its pane by 582px at a 768px viewport · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FileUpload` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `NumberInput` | A | A | A | keep | human verdict (`g15.028`): keep — numeric, steppers, string-form binding, disabled, invalid; string-form binding is web-only, Gp adds prefix/suffix/precision within the section budget |
| `RangeSlider` | A | A | A | pilot-fix | no named defect |
| `Rating` | A | A | A | keep | human verdict (`g15.028`): keep — live default, 10-star, half-step, clearable, disabled; Gp mirrors and adds readonly |
| `RefSelect` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 captioned examples |
| `Select` | C | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Slider` | A | A | A | keep | human verdict (`g15.028`): keep — live volume/step/disabled/embedded with paired axis panes; Gp adds two static fill-evidence sections within budget |
| `TextInput` | C | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 7 captioned examples |
| `ThemeSelect` | A | A | A | keep | human verdict (`g15.028`): keep after Gp specimen repair — added the compact-trigger (no label) and four-column sections the contract's specimen definition requires (`theme-select.md` §14); both were already portable Rust inputs (`show_label`, `columns`). Sv/Rc unchanged |
| `TokenInput` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |

### Date & time — Foundations (10)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Calendar` | A | A | A | keep | human verdict (`g15.029`): keep after Gp specimen repair — section order now matches web (Disabled before Range); live range no longer seeds a filled range that duplicated "Range with pre-selected range"; readouts appear only after a pick. Sv/Rc unchanged |
| `DatePicker` | A | A | A | keep | human verdict (`g15.029`): keep — live default with Selected readout, pre-filled Mar 14, disabled; Sv/Rc paired; Gp mirrors all three with live open/select |
| `DateRangePicker` | A | A | A | keep | human verdict (`g15.029`): keep — live default range gesture with readout, pre-filled Mar 1–14, disabled; Gp adds a static Open (range calendar) so the composed surface is visible without interaction |
| `DateTimePicker` | A | A | A | keep | human verdict (`g15.029`): keep — default, pre-filled Mar 14 2:30 PM, disabled; trigger shows the committed date/time; Gp adds Open (calendar + time) |
| `DateTimeRangePicker` | A | A | A | keep | human verdict (`g15.029`): keep — default, pre-filled Mar 10 9:00–Mar 14 5:00, disabled; start/end time fields visible when open; Gp adds Open (range calendar + start/end time) |
| `DateTimeZonePicker` | A | A | A | keep | human verdict (`g15.029`) plus `g15.039` closeout: **Sv/Rc:** nested `TimeZoneSelect` option press now commits through the shared dismiss stack without closing the picker; a genuine outside press still dismisses the whole composite in one gesture, including while the timezone list is open. Specimen sections unchanged (default, pre-filled, disabled). **Gp:** empty default matches web; Open section still shows calendar + time + zone |
| `DurationInput` | A | A | A | keep | human verdict (`g15.029`): keep after specimen repair — removed the interaction-only "Last change" group (Total already teaches live binding); Gp dropped Empty/zero and Invalid extras, aligned captions/fixtures to the web three-section set, and made the Total readout follow stored state instead of a hardcoded 01:30:00 |
| `TimeAgo` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 captioned examples |
| `TimeInput` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |
| `TimeZoneSelect` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |

### Layout — Foundations (11)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Box` | A | A | A | keep | human verdict (`g15.030`): keep — default teaches containment and padding; fixed dimensions and overflow clip are distinct; Sv/Rc paired verbatim; Gp mirrors all four sections |
| `Grid` | A | A | A | keep | human verdict (`g15.030`): keep — three column layouts teach arrangement, not a prop matrix; Sv/Rc paired; Gp mirrors all three sections |
| `ListGrid` | C | C | A | curation-tranche | **Sv:** controls do nothing — Export and the row action are unwired · **Rc:** controls do nothing — Export and the row action are unwired |
| `Region` | A | A | A | keep | human verdict (`g15.030`): keep — default placeholder, labeled stack, and nav/toolbar/content composition teach boundary role; Sv/Rc paired; Gp mirrors all three sections |
| `ResizeHandle` | A | A | B | contract/runtime-blocker | human verdict (`g15.030`): **Sv/Rc:** keep after specimen repair — interactive splits were inert; now apply drag/keyboard deltas with specimen bounds (`48–280` horizontal, `40–120` vertical) passed through `ariaValueNow/min/max`. **Gp:** drag-only render path — `packages/render/src/resize_handle.rs` never makes the node focusable, installs keyboard handling, or projects `ResizeHandleSpec.aria_value_*`; native page cannot teach the same keyboard/value evidence as web. Routed component-semantic blocker; renderer fix out of scope for this card |
| `ScrollShell` | A | A | A | keep | human verdict (`g15.030`): keep — vertical and horizontal scroll with realistic overflow content; Sv/Rc paired; Gp mirrors both sections |
| `Separator` | A | A | A | keep | human verdict (`g15.030`): keep — horizontal, vertical, and decorative separators with surrounding context; Sv/Rc paired; Gp mirrors all three sections |
| `Spacer` | A | A | A | keep | human verdict (`g15.030`): keep — toolbar-style push-apart compositions teach flex spacing; Sv/Rc paired; Gp mirrors both sections |
| `SplitView` | C | C | B | curation-tranche | **Sv:** Examples long — 7 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples long — 7 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Stack` | A | A | A | keep | human verdict (`g15.030`): keep — column, row, alignment, and wrap teach arrangement within the section budget; Sv/Rc paired; Gp mirrors all five sections |
| `Surface` | A | A | A | keep | human verdict (`g15.030`): keep — panel/canvas/elevated/no-border teach tone and container role; Sv/Rc paired; Gp adds renderer-owned border/padding/role sections while preserving the tone evidence |

### Content & identity — Foundations (14)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Avatar` | B | B | A | curation-tranche | **Sv:** Densities tab shown for a component with no `density` prop · **Rc:** Densities tab shown for a component with no `density` prop |
| `Card` | B | B | B | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Code` | A | A | A | keep | **Sv:** note — 4 clicks changed nothing: Code owns the copy button and writes to the clipboard (navigator.clipboard.writeText); no markup change is expected · **Rc:** note — 4 clicks changed nothing: Code owns the copy button and writes to the clipboard (navigator.clipboard.writeText); no markup change is expected |
| `DetailItem` | B | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `EmbedPreview` | A | A | A | keep | no named defect |
| `Eyebrow` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; takes `size`, but the page shows no Sizes evidence; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 8 captioned examples; takes `size`, but the page shows no Sizes evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Icon` | C | C | A | curation-tranche | **Sv:** overflows its pane by 119px at a 768px viewport; takes `density`, but the page shows no Densities evidence · **Rc:** overflows its pane by 119px at a 768px viewport; takes `density`, but the page shows no Densities evidence |
| `IconProvider` | A | A | A | keep | no named defect |
| `MetaBar` | C | C | A | curation-tranche | **Sv:** controls do nothing — the specimen's only control is unwired · **Rc:** controls do nothing — the specimen's only control is unwired |
| `MetaItem` | C | C | A | curation-tranche | **Sv:** controls do nothing — shares MetaBarSpecimen, whose control is unwired; page is `MetaBarSpecimen.svelte` — it teaches a different component · **Rc:** controls do nothing — shares MetaBarSpecimen, whose control is unwired; page is `MetaBarSpecimen.svelte` — it teaches a different component |
| `Pill` | A | A | A | keep | no named defect |
| `Text` | B | B | C | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence · **Rc:** takes `size`, but the page shows no Sizes evidence · **Gp:** examples carry no captions at all; no Sizes/Densities panes — axis evidence the web page shows is absent |
| `TextLink` | A | A | C | curation-tranche | **Sv:** note — 1 clicks changed nothing: the anchors navigate; the clicked control is an explicit `onClick={() => undefined}` demo · **Rc:** note — 1 clicks changed nothing: the anchors navigate; the clicked control is an explicit `onClick={() => undefined}` demo · **Gp:** examples carry no captions at all |
| `UiPresentationProvider` | C | C | B | curation-tranche | **Sv:** controls do nothing — both Save buttons are unwired; takes `density`, but the page shows no Densities evidence · **Rc:** controls do nothing — both Save buttons are unwired; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Status & progress — Foundations (14)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Callout` | C | C | A | curation-tranche | **Sv:** controls do nothing — generated scene; the dismiss control has no handler · **Rc:** controls do nothing — generated scene; the dismiss control has no handler |
| `EmptyState` | B | B | A | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 captioned examples |
| `ErrorBoundary` | A | A | A | keep | **Sv:** note — 2 clicks changed nothing: Throw again is wired (`shouldThrow = true`); the boundary was already in its error state, so nothing changed · **Rc:** note — 2 clicks changed nothing: Throw again is wired (`shouldThrow = true`); the boundary was already in its error state, so nothing changed |
| `Meter` | B | B | B | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MetricTile` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `PageLoading` | A | A | A | keep | no named defect |
| `Progress` | A | A | A | keep | no named defect |
| `RemediationBanner` | C | C | A | curation-tranche | **Sv:** controls do nothing — Try again, View details and dismiss are unwired · **Rc:** controls do nothing — Try again, View details and dismiss are unwired |
| `Skeleton` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 captioned examples |
| `Spinner` | A | A | A | keep | no named defect |
| `StateTile` | A | A | A | keep | no named defect |
| `StatusIndicator` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ToastHost` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ToastStack` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Navigation — Composition (9)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Breadcrumbs` | A | A | A | keep | no named defect |
| `NavCard` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `NavigationMenu` | A | A | A | keep | no named defect |
| `Pagination` | A | A | A | keep | no named defect |
| `PaginationSummary` | A | A | A | keep | no named defect |
| `SidebarNav` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Stepper` | B | B | A | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 captioned examples |
| `Tabs` | A | A | B | pilot-fix | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Tree` | B | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 7 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Overlays & disclosure — Composition (14)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Accordion` | B | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `AlertDialog` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence; hand-rolled captions instead of SpecimenGroup · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Collapsible` | A | A | A | keep | no named defect |
| `CommandPalette` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ContextMenu` | A | A | A | keep | **Sv:** note — 1 clicks changed nothing: a right-click surface; a left click is the wrong gesture · **Rc:** note — 1 clicks changed nothing: a right-click surface; a left click is the wrong gesture |
| `DebugDialog` | A | A | A | keep | no named defect |
| `Dialog` | C | C | B | curation-tranche | **Sv:** Examples long — 9 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 9 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Drawer` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence; hand-rolled captions instead of SpecimenGroup · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FormDialog` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `HoverCard` | A | A | A | keep | **Sv:** note — 2 clicks changed nothing: a hover surface; a click is the wrong gesture · **Rc:** note — 2 clicks changed nothing: a hover surface; a click is the wrong gesture |
| `Menu` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |
| `Menubar` | A | A | A | keep | no named defect |
| `Popover` | A | A | A | keep | no named defect |
| `Tooltip` | C | C | A | curation-tranche | **Sv:** note — 5 clicks changed nothing: a hover surface; a click is the wrong gesture; Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop · **Rc:** note — 5 clicks changed nothing: a hover surface; a click is the wrong gesture; Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop |

### Forms & validation — Composition (9)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `BlockEditor` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Field` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FieldSet` | A | A | A | keep | no named defect |
| `FormActions` | C | C | B | curation-tranche | **Sv:** controls do nothing — Cancel, Save, Back, Continue and Delete are all unwired · **Rc:** controls do nothing — Cancel, Save, Back, Continue and Delete are all unwired · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FormLayout` | C | C | A | curation-tranche | **Sv:** controls do nothing — every form action in the specimen is unwired · **Rc:** controls do nothing — every form action in the specimen is unwired |
| `InlineListSection` | C | C | A | curation-tranche | **Sv:** controls do nothing — the section's add action is unwired · **Rc:** controls do nothing — the section's add action is unwired |
| `MarkdownEditor` | B | A | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `PasswordRequirements` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ValidationSummary` | A | A | A | keep | no named defect |

### Data & collections — Composition (16)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `BulkActionBar` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `CardRadioGroup` | A | A | A | keep | no named defect |
| `CardToggleGroup` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DataTable` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `EditableList` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FilterBuilder` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 captioned examples |
| `FilterToolbar` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ListCard` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 19 captioned examples · **Rc:** Examples overloaded — 19 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ListCardCounter` | C | C | C | curation-tranche | **Sv:** Examples overloaded — 19 captioned examples; Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop; page is `ListCardSpecimen.svelte` — it teaches a different component · **Rc:** Examples overloaded — 19 captioned examples; Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop; page is `ListCardSpecimen.svelte` — it teaches a different component · **Gp:** examples carry no captions at all |
| `ListContainer` | A | A | A | keep | no named defect |
| `LogList` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `OrderBy` | A | A | A | keep | no named defect |
| `PickerShell` | B | B | A | curation-tranche | **Sv:** Densities tab shown for a component with no `density` prop · **Rc:** Densities tab shown for a component with no `density` prop |
| `RelationPicker` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `SelectionSummary` | A | A | A | keep | no named defect |
| `Table` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Media — Composition (6)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `AudioPlayer` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaBrowsePanel` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaPicker` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaPreview` | B | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaThumbnail` | A | A | A | keep | no named defect |
| `VideoPlayer` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Application shell — Systems (12)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `ActionDiscoveryPanel` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 captioned examples |
| `AppHeader` | B | B | B | curation-tranche | **Sv:** overflows its pane by 31px at a 768px viewport · **Rc:** overflows its pane by 31px at a 768px viewport · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DetailSection` | B | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DetailSectionGroup` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 12 captioned examples · **Rc:** Examples overloaded — 12 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DetailShell` | C | C | A | curation-tranche | **Sv:** Examples long — 8 captioned examples; controls do nothing — Edit and Reset are unwired · **Rc:** Examples long — 8 captioned examples; controls do nothing — Edit and Reset are unwired |
| `DockRegion` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 8 captioned examples |
| `HistoryCenter` | B | B | C | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 captioned examples · **Gp:** examples carry no captions at all; no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MessageCenter` | A | A | C | curation-tranche | **Gp:** examples carry no captions at all; no Sizes/Densities panes — axis evidence the web page shows is absent |
| `PageHeader` | C | C | B | curation-tranche | **Sv:** Examples long — 9 captioned examples; controls do nothing — specimen wires no handler on any header action · **Rc:** Examples long — 9 captioned examples; controls do nothing — specimen wires no handler on any header action · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `SettingsShell` | C | C | C | curation-tranche | **Sv:** examples carry no captions at all; hand-rolled captions instead of SpecimenGroup · **Rc:** examples carry no captions at all · **Gp:** examples carry no captions at all |
| `StatusBar` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Toolbar` | C | C | A | curation-tranche | **Sv:** controls do nothing — specimen wires no handler on any toolbar control · **Rc:** controls do nothing — specimen wires no handler on any toolbar control |

### Agent & tools — Systems (11)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `AgentChatInput` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 captioned examples |
| `AgentMessage` | D | C | A | curation-tranche | **Sv:** all 8 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** Examples long — 9 captioned examples; controls do nothing — the message actions are unwired |
| `AgentPlan` | D | A | A | curation-tranche | **Sv:** all 4 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentPlanRecord` | D | A | A | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentQuestion` | D | C | B | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** Examples overloaded — 12 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `AgentQuestionRecord` | D | C | A | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** Examples overloaded — 10 captioned examples |
| `AgentSubagent` | D | A | A | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentTranscript` | B | A | B | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ChangedFiles` | D | B | A | curation-tranche | **Sv:** all 7 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** Examples long — 7 captioned examples |
| `ToolCall` | D | A | A | curation-tranche | **Sv:** all 4 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `ToolCallGroup` | D | A | A | curation-tranche | **Sv:** all 5 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |

### Model connections — Systems (5)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `ModelCatalogueEditor` | A | A | A | curation-complete (`g15.020`) | no named defect |
| `ModelConnectionCard` | A | A | A | curation-complete (`g15.020`) | `unknown` and `error` readiness stories remain a recorded catalogue coverage note, not a curation defect |
| `ModelConnectionPicker` | A | A | A | curation-complete (`g15.020`) | no named defect |
| `ModelConnectionSetup` | A | A | A | curation-complete (`g15.020`) | no-credential direct add and detected/missing outcomes pinned beyond captions |
| `ModelPicker` | A | A | A | curation-complete (`g15.020`) | no named defect |

### Audio & music — Systems (12)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `AudioMeter` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 10 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `AudioSwitch` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 8 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `EnvelopeEditor` | C | C | B | curation-tranche | **Sv:** Examples long — 7 captioned examples; overflows its pane by 32px at a 768px viewport; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 9 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `Fader` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 10 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `GainReductionMeter` | C | C | B | curation-tranche | **Sv:** Examples long — 9 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `Keyboard` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 8 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `Knob` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 10 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 12 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `ModMatrixGrid` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 7 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `ValueReadout` | C | C | B | curation-tranche | **Sv:** Examples long — 9 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `WaveformDisplay` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 8 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `XYPad` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 10 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `MeterSurface` | C | A | n/a | curation-tranche | **Sv:** Sizes tab is advertised but renders nothing; Densities tab is advertised but renders nothing; Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop; hand-rolled captions instead of SpecimenGroup |

### Account & lifecycle — Systems (5)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `LicenceActivation` | A | A | A | verified no-op (`g15.020`) | no named defect |
| `LicenceSeats` | A | A | A | verified no-op (`g15.020`) | no named defect |
| `LicenceStatus` | A | A | A | curation-complete (`g15.020`) | no named defect |
| `UpdateCenter` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `UpdateStatus` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
