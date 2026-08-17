# g15 — Human-Centred Specimen Catalogue Audit

Status: complete — measured by `g15.011`
Date: 2026-08-17
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

## How It Was Measured

Four passes, each mechanical and reproducible:

1. **Inventory.** The three runtime specimen maps resolved to files:
   `packages/svelte/preview/src/specimens/registry.ts`,
   `packages/react/preview/src/gallery/specimen-map.ts`, and the
   `render_single_specimen` dispatch in
   `packages/gpui/preview/src/specimens/mod.rs`.
2. **Axis eligibility.** Whether each component's Svelte implementation
   actually takes `size` and `density`, read from its `$props()` block, then
   compared with what its pages advertise. 126 components take `size`; 128 take
   `density`.
3. **Live page sweep.** Every one of the 175 pages loaded in the running Svelte
   preview, with the `Sizes` and `Densities` tabs opened, recording what
   rendered.
4. **Live caption sweep.** Every page's example captions read from the DOM
   across all four caption idioms the catalogue uses, so a caption is counted
   when a reader can see it and not when the source merely contains one.

Static signals that the live passes contradicted were discarded rather than
reported. Three did: an apparent 47-page "empty Sizes tab" class was pages that
correctly omit `SpecimenLayout`; an apparent GPUI "no captions" class was
captions threaded through local helpers; and an apparent set of caption-less
pages turned out to use a bare `<Eyebrow>` idiom the first probe did not read.

## Grades

- **A — ready:** concise teaching page, meaningful interaction, no named
  curation defect.
- **B — usable:** teaches the component, one small named defect.
- **C — curate:** overloaded, unclear, misleading, substantially drifted, or
  missing important teaching value.
- **D — missing/broken:** no real specimen, dead primary interaction, or the
  page cannot be used as documentation.

Defects are weighted. A minor defect (one missing axis, a hand-rolled caption
idiom) scores 1; a major one (captions that do not render, an overloaded
`Examples`, an advertised tab that renders nothing) scores 2. Score 0 is A, 1
is B, 2 or more is C. D is reserved for pages that fail as documentation
outright.

Dispositions: `keep`, `pilot-fix`, `curation-tranche`, or
`contract/runtime-blocker`.

## Totals

| Runtime | A | B | C | D | n/a |
| --- | ---: | ---: | ---: | ---: | ---: |
| Svelte | 87 | 35 | 44 | 9 | — |
| React | 136 | 21 | 18 | 0 | — |
| GPUI | 116 | 52 | 6 | 0 | 1 |
| **Worst of the three** | **61** | **54** | **51** | **9** | — |

| Disposition | Count |
| --- | ---: |
| `keep` | 60 |
| `pilot-fix` | 3 |
| `curation-tranche` | 112 |
| `contract/runtime-blocker` | 0 |

No entry is a contract or runtime blocker. Nothing found here needs a component
semantic change; the defects are documentation defects.

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

The React versions of the same nine pages pass `label` correctly and grade A or
B. This is Svelte-only, and it is the one class where the catalogue actively
lies about being fine.

### 2. Four caption idioms, one of which renders nothing

The catalogue captions its examples four different ways: `SpecimenGroup label=`
(140 pages), a bare `<Eyebrow>` inside a hand-rolled `Surface` (21 pages),
`<section><h3>` in the audio family (14 pages), and React's separate
`AudioSpecimenGroup title=`. They differ in spacing, weight, and surface
treatment, so pages that should look like siblings do not.

The divergence is what let the broken fifth idiom hide in plain sight.

### 3. Size and density matrices in the main view

The twelve audio components put their full size **and** density sweeps into the
page body. React has no `Examples`/`Sizes`/`Densities` tabs for them at all —
`AudioSpecimenPage` is a plain grid and `AudioAxes` appends the two matrices as
ordinary sections. The native pages do the same through
`poodle_render::audio_specimens`'s `axis_groups`. Only Svelte puts them in
tabs.

This is the specific thing the operator asked not to happen, and it is
currently the majority position for that family across runtimes.

### 4. Missing axis evidence where the axis applies

126 components take `size` and 128 take `density`. Seventeen pages show no
`Sizes` evidence for a component that takes `size`, and eighteen show no
`Densities` evidence for one that takes `density` — most of the overlay family,
all five licence and update surfaces, and a handful of others.

Four pages have the opposite problem and advertise an axis tab for a component
with no such prop.

One page, `MeterSurface`, advertises both tabs and renders them empty. It is
the only genuinely empty tab in the catalogue.

### 5. Overloaded Examples

Fourteen pages show ten or more captioned examples in one view. `Tabs` shows
18 groups and more than seventy tab controls on a single page. `ListCard` shows
20. A further 41 pages sit in the 7–9 band, many of them because a variant ×
tone cross-product was expanded rather than a set of distinct things worth
seeing.

### 6. Two pages that teach a different component

`ListCardCounter` renders `ListCardSpecimen` and `MetaItem` renders
`MetaBarSpecimen`. The page title names one component; the content teaches
another.

### 7. GPUI structure

Every component in the active cohort has a native page — `g15.010` closed the
last gap. What those pages *show* still differs: 43 native pages have no
`Sizes`/`Densities` panes although their web counterpart teaches that axis, and
six render their examples with no captions at all.

Native layout mechanics are runtime-owned and are not a defect. The absent
evidence is, per the working rules' parity authority.

## What The Audit Did Not Find

- **No `Conformance` tab, anywhere.** The live tab set is exactly
  `Examples · Sizes · Densities`. A repository-wide search for conformance or
  corpus projection wiring in the three previews returns three incidental
  matches: a sentence in `accessibility.ts` about assistive-technology claims,
  a demo activity string in `component-docs.ts`, and a comment in
  `headless_driver.rs` noting the retained headless infrastructure. `g14.021`'s
  removal holds.
- **No dead pages.** All 175 pages render. None falls through to
  `missing_specimen` or the "specimen not yet available" placeholder, and no
  page threw a runtime error during the sweep.
- **No contract or semantic defects.** Nothing here needs a component change.

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

## Proposed Curation Tranches

The 112 `curation-tranche` entries group into five bounded, reviewable
families. Each is a planned card requiring orchestrator review before dispatch.

| Card | Family | Scale |
| --- | --- | ---: |
| [`g15.015`](015-specimen-caption-integrity.md) | Caption integrity + the type-check gate hole that hid it | 9 pages, 52 captions |
| [`g15.016`](016-specimen-idiom-convergence.md) | One caption idiom; two borrowed pages get their own | ~35 pages |
| [`g15.017`](017-specimen-axis-placement.md) | Axis matrices out of the main view; axis evidence where the prop exists | 12 audio + ~21 others |
| [`g15.018`](018-overloaded-examples-curation.md) | Overloaded and cross-product `Examples` | 14 + 41 pages |
| [`g15.019`](019-gpui-specimen-structure.md) | Native axis panes and captions | 43 + 6 pages |

Ordering matters: `g15.015` first, because it closes the gate that let the
worst class ship. `g15.017` before `g15.019`, because the native axis work
depends on `audio_specimens` separating its axis groups.

## Per-Component Inventory

Grades are per runtime: **Sv** Svelte, **Rc** React, **Gp** GPUI. Evidence
names the defects that decided the grade; a row with no named defect is A.

### Actions & selection — Foundations (12)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Button` | B | B | A | pilot-fix | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 examples |
| `Checkbox` | A | A | A | keep | no named defect |
| `CollapseToggle` | A | A | A | keep | no named defect |
| `ConfirmAction` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `IconButton` | B | A | A | curation-tranche | **Sv:** takes `density`, but the page shows no Densities evidence |
| `Radio` | A | A | A | keep | no named defect |
| `RadioGroup` | A | A | A | keep | no named defect |
| `SegmentedControl` | A | A | A | keep | no named defect |
| `SplitButton` | C | A | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup |
| `Switch` | A | A | A | keep | no named defect |
| `ToggleGroup` | A | A | A | keep | no named defect |
| `TriStateSwitch` | B | A | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Text & value entry — Foundations (15)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `CodeInput` | A | A | A | keep | no named defect |
| `ColorPicker` | A | A | A | keep | no named defect |
| `DragNumberField` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 8 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `EditableLabel` | A | B | A | curation-tranche | **Rc:** Examples long — 7 examples |
| `EmbedInput` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FileUpload` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `NumberInput` | A | A | A | keep | no named defect |
| `RangeSlider` | A | A | A | pilot-fix | no named defect |
| `Rating` | A | A | A | keep | no named defect |
| `RefSelect` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 examples |
| `Select` | C | A | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Slider` | A | A | A | keep | no named defect |
| `TextInput` | C | A | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup |
| `ThemeSelect` | A | A | A | keep | no named defect |
| `TokenInput` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |

### Date & time — Foundations (10)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Calendar` | A | A | A | keep | no named defect |
| `DatePicker` | A | A | A | keep | no named defect |
| `DateRangePicker` | A | A | A | keep | no named defect |
| `DateTimePicker` | A | A | A | keep | no named defect |
| `DateTimeRangePicker` | A | A | A | keep | no named defect |
| `DateTimeZonePicker` | A | A | A | keep | no named defect |
| `DurationInput` | A | A | A | keep | no named defect |
| `TimeAgo` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 examples |
| `TimeInput` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |
| `TimeZoneSelect` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |

### Layout — Foundations (11)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Box` | A | A | A | keep | no named defect |
| `Grid` | A | A | A | keep | no named defect |
| `ListGrid` | A | A | A | keep | no named defect |
| `Region` | A | A | A | keep | no named defect |
| `ResizeHandle` | A | A | A | keep | no named defect |
| `ScrollShell` | A | A | A | keep | no named defect |
| `Separator` | A | A | A | keep | no named defect |
| `Spacer` | A | A | A | keep | no named defect |
| `SplitView` | C | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples long — 7 examples |
| `Stack` | A | A | A | keep | no named defect |
| `Surface` | A | A | A | keep | no named defect |

### Content & identity — Foundations (14)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Avatar` | B | A | A | curation-tranche | **Sv:** Densities tab shown for a component with no `density` prop |
| `Card` | B | A | B | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Code` | A | A | A | keep | no named defect |
| `DetailItem` | B | A | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `EmbedPreview` | A | A | A | keep | no named defect |
| `Eyebrow` | C | A | A | curation-tranche | **Sv:** Examples long — 8 captioned examples; takes `size`, but the page shows no Sizes evidence; hand-rolled captions instead of SpecimenGroup |
| `Icon` | B | A | A | curation-tranche | **Sv:** takes `density`, but the page shows no Densities evidence |
| `IconProvider` | A | A | A | keep | no named defect |
| `MetaBar` | A | A | A | keep | no named defect |
| `MetaItem` | C | C | A | curation-tranche | **Sv:** page is `MetaBarSpecimen.svelte` — it teaches a different component · **Rc:** page is `MetaBarSpecimen.svelte` — it teaches a different component |
| `Pill` | A | A | A | keep | no named defect |
| `Text` | C | A | C | curation-tranche | **Sv:** a size/density matrix is repeated inside Examples; takes `size`, but the page shows no Sizes evidence · **Gp:** examples carry no captions at all |
| `TextLink` | A | A | C | curation-tranche | **Gp:** examples carry no captions at all |
| `UiPresentationProvider` | B | A | A | curation-tranche | **Sv:** takes `density`, but the page shows no Densities evidence |

### Status & progress — Foundations (14)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Callout` | A | A | A | keep | no named defect |
| `EmptyState` | B | A | A | curation-tranche | **Sv:** Examples long — 8 captioned examples |
| `ErrorBoundary` | A | A | A | keep | no named defect |
| `Meter` | C | B | B | curation-tranche | **Sv:** Examples long — 7 captioned examples; a size/density matrix is repeated inside Examples · **Rc:** Examples long — 7 examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MetricTile` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `PageLoading` | A | A | A | keep | no named defect |
| `Progress` | A | A | A | keep | no named defect |
| `RemediationBanner` | A | A | A | keep | no named defect |
| `Skeleton` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 examples |
| `Spinner` | A | A | A | keep | no named defect |
| `StateTile` | A | A | A | keep | no named defect |
| `StatusIndicator` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ToastHost` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `ToastStack` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Navigation — Composition (9)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Breadcrumbs` | A | A | A | keep | no named defect |
| `NavCard` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `NavigationMenu` | A | A | A | keep | no named defect |
| `Pagination` | C | A | A | curation-tranche | **Sv:** a size/density matrix is repeated inside Examples |
| `PaginationSummary` | A | A | A | keep | no named defect |
| `SidebarNav` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Stepper` | B | B | A | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 8 examples |
| `Tabs` | C | C | B | pilot-fix | **Sv:** Examples overloaded — 18 captioned examples · **Rc:** Examples overloaded — 18 examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Tree` | B | B | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Rc:** Examples long — 7 examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Overlays & disclosure — Composition (14)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `Accordion` | B | A | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `AlertDialog` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence; hand-rolled captions instead of SpecimenGroup |
| `Collapsible` | A | A | A | keep | no named defect |
| `CommandPalette` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ContextMenu` | A | A | A | keep | no named defect |
| `DebugDialog` | A | A | A | keep | no named defect |
| `Dialog` | C | A | A | curation-tranche | **Sv:** Examples long — 9 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence; hand-rolled captions instead of SpecimenGroup |
| `Drawer` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence; hand-rolled captions instead of SpecimenGroup |
| `FormDialog` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `HoverCard` | A | A | A | keep | no named defect |
| `Menu` | B | A | A | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup |
| `Menubar` | A | A | A | keep | no named defect |
| `Popover` | A | A | A | keep | no named defect |
| `Tooltip` | C | A | A | curation-tranche | **Sv:** Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop |

### Forms & validation — Composition (9)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `BlockEditor` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `Field` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FieldSet` | A | A | A | keep | no named defect |
| `FormActions` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FormLayout` | A | A | A | keep | no named defect |
| `InlineListSection` | A | A | A | keep | no named defect |
| `MarkdownEditor` | B | A | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `PasswordRequirements` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ValidationSummary` | A | A | A | keep | no named defect |

### Data & collections — Composition (16)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `BulkActionBar` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `CardRadioGroup` | C | A | A | curation-tranche | **Sv:** a size/density matrix is repeated inside Examples |
| `CardToggleGroup` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DataTable` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `EditableList` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `FilterBuilder` | B | B | A | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Rc:** Examples long — 7 examples |
| `FilterToolbar` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ListCard` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 19 captioned examples; a size/density matrix is repeated inside Examples · **Rc:** Examples overloaded — 20 examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ListCardCounter` | C | C | C | curation-tranche | **Sv:** Examples overloaded — 19 captioned examples; page is `ListCardSpecimen.svelte` — it teaches a different component; a size/density matrix is repeated inside Examples; Sizes tab shown for a component with no `size` prop; Densities tab shown for a component with no `density` prop · **Rc:** Examples overloaded — 20 examples; page is `ListCardSpecimen.svelte` — it teaches a different component · **Gp:** examples carry no captions at all; no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ListContainer` | A | A | A | keep | no named defect |
| `LogList` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `OrderBy` | A | A | A | keep | no named defect |
| `PickerShell` | B | A | B | curation-tranche | **Sv:** Densities tab shown for a component with no `density` prop · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `RelationPicker` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `SelectionSummary` | A | A | A | keep | no named defect |
| `Table` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |

### Media — Composition (6)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `AudioPlayer` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaBrowsePanel` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaPicker` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaPreview` | B | A | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MediaThumbnail` | A | A | A | keep | no named defect |
| `VideoPlayer` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |

### Application shell — Systems (12)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `ActionDiscoveryPanel` | B | A | A | curation-tranche | **Sv:** Examples long — 9 captioned examples |
| `AppHeader` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DetailSection` | B | A | B | curation-tranche | **Sv:** Examples long — 8 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DetailSectionGroup` | C | A | B | curation-tranche | **Sv:** Examples overloaded — 12 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `DetailShell` | B | A | A | curation-tranche | **Sv:** Examples long — 8 captioned examples |
| `DockRegion` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 8 examples |
| `HistoryCenter` | B | B | C | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 examples · **Gp:** examples carry no captions at all; no Sizes/Densities panes — axis evidence the web page shows is absent |
| `MessageCenter` | A | A | C | curation-tranche | **Gp:** examples carry no captions at all; no Sizes/Densities panes — axis evidence the web page shows is absent |
| `PageHeader` | B | B | B | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 8 examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `SettingsShell` | C | A | C | curation-tranche | **Sv:** examples carry no captions at all; hand-rolled captions instead of SpecimenGroup · **Gp:** examples carry no captions at all |
| `StatusBar` | A | A | B | curation-tranche | **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `Toolbar` | A | A | A | keep | no named defect |

### Agent & tools — Systems (11)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `AgentChatInput` | B | B | A | curation-tranche | **Sv:** Examples long — 9 captioned examples · **Rc:** Examples long — 9 examples |
| `AgentMessage` | D | B | A | curation-tranche | **Sv:** all 8 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** Examples long — 8 examples |
| `AgentPlan` | D | A | A | curation-tranche | **Sv:** all 4 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentPlanRecord` | D | A | A | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentQuestion` | D | A | B | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `AgentQuestionRecord` | D | A | A | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentSubagent` | D | A | A | curation-tranche | **Sv:** all 6 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |
| `AgentTranscript` | B | A | B | curation-tranche | **Sv:** Examples long — 7 captioned examples · **Gp:** no Sizes/Densities panes — axis evidence the web page shows is absent |
| `ChangedFiles` | D | B | A | curation-tranche | **Sv:** all 7 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** Examples long — 7 examples |
| `ToolCall` | D | B | A | curation-tranche | **Sv:** all 4 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` · **Rc:** example count differs from Svelte (0 vs 4) |
| `ToolCallGroup` | D | A | A | curation-tranche | **Sv:** all 5 example captions render blank — SpecimenGroup takes `label`, the page passes `title`/`description` |

### Model connections — Systems (5)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `ModelCatalogueEditor` | C | B | A | curation-tranche | **Sv:** Examples overloaded — 21 captioned examples · **Rc:** Examples long — 9 examples |
| `ModelConnectionCard` | C | B | A | curation-tranche | **Sv:** Examples overloaded — 20 captioned examples · **Rc:** Examples long — 9 examples |
| `ModelConnectionPicker` | C | B | A | curation-tranche | **Sv:** Examples overloaded — 22 captioned examples · **Rc:** Examples long — 8 examples |
| `ModelConnectionSetup` | C | B | A | curation-tranche | **Sv:** Examples overloaded — 12 captioned examples · **Rc:** Examples long — 8 examples |
| `ModelPicker` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 13 captioned examples · **Rc:** Examples overloaded — 13 examples |

### Audio & music — Systems (12)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `AudioMeter` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 10 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 9 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `AudioSwitch` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs · **Gp:** size and density matrices sit in the page body, not in panes |
| `EnvelopeEditor` | C | C | B | curation-tranche | **Sv:** Examples long — 7 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 7 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `Fader` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 8 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `GainReductionMeter` | C | C | B | curation-tranche | **Sv:** Examples long — 9 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 9 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `Keyboard` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs · **Gp:** size and density matrices sit in the page body, not in panes |
| `Knob` | C | C | B | curation-tranche | **Sv:** Examples overloaded — 10 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples overloaded — 10 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `ModMatrixGrid` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs · **Gp:** size and density matrices sit in the page body, not in panes |
| `ValueReadout` | C | C | B | curation-tranche | **Sv:** Examples long — 9 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 9 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `WaveformDisplay` | B | C | B | curation-tranche | **Sv:** hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs · **Gp:** size and density matrices sit in the page body, not in panes |
| `XYPad` | C | C | B | curation-tranche | **Sv:** Examples long — 8 captioned examples; hand-rolled captions instead of SpecimenGroup · **Rc:** size and density matrices render in the main view — the page has no Examples/Sizes/Densities tabs; Examples long — 8 examples · **Gp:** size and density matrices sit in the page body, not in panes |
| `MeterSurface` | C | A | n/a | curation-tranche | **Sv:** Sizes tab is advertised but renders nothing; Densities tab is advertised but renders nothing; hand-rolled captions instead of SpecimenGroup |

### Account & lifecycle — Systems (5)

| Component | Sv | Rc | Gp | Disposition | Evidence |
| --- | :-: | :-: | :-: | --- | --- |
| `LicenceActivation` | C | A | A | curation-tranche | **Sv:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `LicenceSeats` | C | A | A | curation-tranche | **Sv:** Examples overloaded — 11 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `LicenceStatus` | C | C | A | curation-tranche | **Sv:** Examples overloaded — 20 captioned examples; takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence · **Rc:** Examples overloaded — 10 examples |
| `UpdateCenter` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
| `UpdateStatus` | C | A | A | curation-tranche | **Sv:** takes `size`, but the page shows no Sizes evidence; takes `density`, but the page shows no Densities evidence |
