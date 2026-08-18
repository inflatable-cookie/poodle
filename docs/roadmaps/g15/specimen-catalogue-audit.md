# g15 — Human-Centred Specimen Catalogue Audit

Status: **partial** — a mechanical screening baseline for the web catalogue.
Not the completed human-centred audit: GPUI was not rendered (`g15.026`), and
the rubric's teaching judgment was applied to the three pilots only, not to all
175 pages.
Date: 2026-08-18 (revision 2; first pass 2026-08-17)
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

**GPUI is not measured live, and its grades are provisional.** No headless path
renders a GPUI specimen page: `packages/gpui/preview` is a binary crate and
`render_single_specimen` depends on `PreviewRoot`, `AppState`, and the
catalogue sidebar, so nothing outside `main.rs` can construct one. The retained
headless driver mounts a `poodle-node` tree, which is the component tier rather
than the page tier.

The GPUI column therefore grades what source can prove — dispatch reachability,
caption presence, and whether the page uses `specimen_layout` — and nothing
about render, interaction, or narrow behaviour. `g15.026` builds the seam and
the probe that close this; until it lands, **no claim in this document about
pages rendering, interaction being live, or narrow behaviour applies to GPUI.**

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

**These are screening grades.** They are computed from measured signals —
caption rendering, example count, axis eligibility, narrow overflow, and a
source-checked interaction verdict. They are not the carried rubric's human
judgment. In particular, no grade in this table asserts that a page's *first
example teaches normal use* or that its *variants are meaningfully distinct*.
Those two questions need a person, and a person answered them for three pages:
Button, RangeSlider, and Tabs.

So read an **A** as "no measured defect", not as "judged a good teaching page".
A page can screen A and still open on a prop showcase. Each curation tranche
applies the human judgment to its own family, which is why every tranche card
carries a live operator-review checkpoint.

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

Dispositions: `keep`, `pilot-fix`, `curation-tranche`, or
`contract/runtime-blocker`.

## Totals

Measured against this branch's head — after the three approved pilot pages
landed, so these grades describe the catalogue as it now stands, not a
pre-pilot baseline.

| Runtime | A | B | C | D | n/a |
| --- | ---: | ---: | ---: | ---: | ---: |
| Svelte (live) | 81 | 33 | 52 | 9 | — |
| React (live) | 94 | 26 | 55 | 0 | — |
| GPUI (structural, provisional) | 100 | 68 | 6 | 0 | 1 |
| **Worst of the three** | **58** | **48** | **60** | **9** | — |

| Disposition | Count |
| --- | ---: |
| `keep` | 56 |
| `pilot-fix` | 3 |
| `curation-tranche` | 116 |
| `contract/runtime-blocker` | 0 |

175 of 175 pages were measured live in both web runtimes. No entry is a
contract or runtime blocker; nothing found here needs a component semantic
change.

`MeterSurface` is the single `n/a` on GPUI — web-only by fixed decision
(spec 068), with no native counterpart. It is still graded on the two runtimes
it has.

## What The Audit Found

### 1. Nine pages where no caption renders — the only D grade

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

### 3. Four caption idioms, one of which renders nothing

The catalogue captions its examples four different ways: `SpecimenGroup label=`
(140 pages), a bare `<Eyebrow>` inside a hand-rolled `Surface` (21 pages),
`<section><h3>` in the audio family (14 pages), and React's separate
`AudioSpecimenGroup title=`. They differ in spacing, weight, and surface
treatment, so pages that should look like siblings do not. The divergence is
what let the broken fifth idiom hide in plain sight.

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

### 9. GPUI structure (provisional)

Every component in the active cohort has a native page — `g15.010` closed the
last gap. What those pages *show* still differs: 59 native pages have no
`Sizes`/`Densities` panes although their component takes that axis, and six
render their examples with no captions at all.

Native layout mechanics are runtime-owned and are not a defect. The absent
evidence is, per the working rules' parity authority. These counts come from
source, not from a rendered page — see the measurement note above.

## What The Audit Did Not Find

- **No `Conformance` tab, anywhere.** The live tab set is exactly
  `Examples · Sizes · Densities` on all 175 pages in both web runtimes. A
  repository-wide search for conformance or corpus projection wiring in the
  three previews returns three incidental matches: a sentence in
  `accessibility.ts` about assistive-technology claims, a demo activity string
  in `component-docs.ts`, and a comment in `headless_driver.rs` noting the
  retained headless infrastructure. `g14.021`'s removal holds.
- **No unrendered pages in the web catalogue.** All 175 pages render in both
  runtimes. None falls through to the "specimen not yet available" placeholder.
  This claim does **not** extend to GPUI, which was not rendered. Fourteen
  pages do render controls that are not wired — see finding 2 — but the pages
  themselves are alive.
- **No contract or semantic defects.** Nothing here needs a component change.

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
  completion child rather than a follow-on.
- **The interaction result is now source-checked, not heuristic.** Revision 2
  scored "20 pages whose controls do nothing" as a defect while its own prose
  admitted the probe cannot see clipboard writes, hover surfaces, or
  navigation. Each flagged page was checked against its specimen source; 14 are
  genuinely unwired and six are not defects at all.
- **The grades are labelled as screening grades.** They do not assert the
  rubric's teaching judgment, which a person applied to the three pilots and
  which each curation tranche applies to its own family.

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

## Proposed Curation Tranches

The 120 `curation-tranche` entries group into bounded, reviewable families.
Each is a planned card requiring orchestrator review before dispatch, and each
that changes specimen presentation carries a live operator-review checkpoint.

| Card | Family | Scale |
| --- | --- | ---: |
| [`g15.015`](015-specimen-caption-integrity.md) | Caption integrity + the type-check gate hole that hid it | 9 pages, 52 captions |
| [`g15.016`](016-specimen-idiom-convergence.md) | One caption idiom; two borrowed pages get their own | ~35 pages |
| [`g15.017`](017-specimen-axis-placement.md) | Axis matrices out of the main view; axis evidence where the prop exists | 12 audio + ~22 others |
| [`g15.018`](018-overloaded-examples-curation.md) | Overloaded `Examples` — **parent, not dispatchable** | 53 pages |
| ↳ [`g15.020`](020-curate-model-connection-licence.md)–[`g15.025`](025-curate-collections-navigation-tail.md) | six bounded family children, one exact page list each | 6–11 each, 53 total |
| [`g15.019`](019-gpui-specimen-structure.md) | Native axis panes and captions | 59 + 6 pages |
| [`g15.026`](026-native-specimen-probe.md) | The headless native probe that un-provisions the GPUI column | 174 pages |

Ordering matters: `g15.015` first, because it closes the gate that let the
worst class ship. `g15.017` before `g15.019`, because the native axis work
depends on `audio_specimens` separating its axis groups.

**`g15.026` is `g15.011`'s completion child, not a follow-on.** `g15.011`
scopes full Svelte, React, and GPUI pages and carries the all-runtime rubric.
This document delivers the web two-thirds. The card is not complete until the
native third is measured to the same standard, and this artifact should be read
as the partial baseline it says it is at the top.

The 20 inert-control pages are not their own card: each sits inside the family
tranche that already owns its page, and the tranche decides per page whether a
control should be wired or removed.

## Per-Component Inventory

Grades are per runtime: **Sv** Svelte (live), **Rc** React (live),
**Gp** GPUI (structural, provisional — see the measurement note). Evidence
names the defects that decided the grade; a row with no named defect is A.

† GPUI grades are not live-measured. `g15.026` closes that gap.

### Actions & selection — Foundations (12)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Button` | A | A | A | pilot-fix | no named defect |
| `Checkbox` | A | A | A | keep | no named defect |
| `CollapseToggle` | A | A | A | keep | no named defect |
| `ConfirmAction` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `IconButton` | C | C | A | curation-tranche | **Sv:** controls do nothing — specimen wires no handler on any IconButton; takes `density`, but the page shows no Densities evidence · **Rc:** controls do nothing — specimen wires no handler on any IconButton; takes `density`, but the page shows no Densities evidence |
| `Radio` | A | A | A | keep | no named defect |
| `RadioGroup` | A | A | A | keep | no named defect |
| `SegmentedControl` | A | A | A | keep | no named defect |
| `SplitButton` | C | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 7 captioned examples |
| `Switch` | A | A | A | keep | no named defect |
| `ToggleGroup` | A | A | A | keep | no named defect |
| `TriStateSwitch` | B | A | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Text & value entry — Foundations (15)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `CodeInput` | A | A | A | keep | no named defect |
| `ColorPicker` | A | A | A | keep | no named defect |
| `DragNumberField` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples overloaded — 10 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** size and density matrices sit in the page body, not in panes |
| `EditableLabel` | A | A | A | keep | no named defect |
| `EmbedInput` | B | B | B | curation-tranche | **Sv:** overflows its pane by 582px at a 768px viewport · **Rc:** overflows its pane by 582px at a 768px viewport · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FileUpload` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `NumberInput` | A | A | A | keep | no named defect |
| `RangeSlider` | A | A | A | pilot-fix | no named defect |
| `Rating` | A | A | A | keep | no named defect |
| `RefSelect` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 captioned examples |
| `Select` | C | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Slider` | A | A | A | keep | no named defect |
| `TextInput` | C | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** Examples long — 7 captioned examples |
| `ThemeSelect` | A | A | A | keep | no named defect |
| `TokenInput` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |

### Date & time — Foundations (10)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Calendar` | A | A | A | keep | no named defect |
| `DatePicker` | A | A | A | keep | no named defect |
| `DateRangePicker` | A | A | A | keep | no named defect |
| `DateTimePicker` | A | A | A | keep | no named defect |
| `DateTimeRangePicker` | A | A | A | keep | no named defect |
| `DateTimeZonePicker` | A | A | A | keep | no named defect |
| `DurationInput` | A | A | A | keep | no named defect |
| `TimeAgo` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 captioned examples |
| `TimeInput` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |
| `TimeZoneSelect` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |

### Layout — Foundations (11)

| Component | Sv | Rc | Gp† | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Box` | A | A | A | keep | no named defect |
| `Grid` | A | A | A | keep | no named defect |
| `ListGrid` | C | C | A | curation-tranche | **Sv:** controls do nothing — Export and the row action are unwired · **Rc:** controls do nothing — Export and the row action are unwired |
| `Region` | A | A | A | keep | no named defect |
| `ResizeHandle` | A | A | A | keep | no named defect |
| `ScrollShell` | A | A | A | keep | no named defect |
| `Separator` | A | A | A | keep | no named defect |
| `Spacer` | A | A | A | keep | no named defect |
| `SplitView` | C | C | B | curation-tranche | **Sv:** Examples long — 7 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples long — 7 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Stack` | A | A | A | keep | no named defect |
| `Surface` | A | A | A | keep | no named defect |

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
| `ModelCatalogueEditor` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 21 captioned examples · **Rc:** Examples overloaded — 21 captioned examples |
| `ModelConnectionCard` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 20 captioned examples · **Rc:** Examples overloaded — 20 captioned examples |
| `ModelConnectionPicker` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 22 captioned examples · **Rc:** Examples overloaded — 22 captioned examples |
| `ModelConnectionSetup` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 12 captioned examples · **Rc:** Examples overloaded — 12 captioned examples |
| `ModelPicker` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 13 captioned examples · **Rc:** Examples overloaded — 13 captioned examples |

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
| `LicenceActivation` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 11 captioned examples; overflows its pane by 345px at a 768px viewport; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples overloaded — 11 captioned examples; overflows its pane by 345px at a 768px viewport; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `LicenceSeats` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `LicenceStatus` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 20 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples overloaded — 20 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `UpdateCenter` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `UpdateStatus` | C | C | B | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
