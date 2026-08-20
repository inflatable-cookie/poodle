# g15.025 — Collections, navigation and long-tail specimen curation

Date: 2026-08-20
Card: `docs/roadmaps/g15/025-curate-collections-navigation-tail.md`
Parent: `docs/roadmaps/g15/018-overloaded-examples-curation.md`
Handoff: `docs/handoffs/20260820-000438-g15-025-collections-navigation-tail.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-6c17484d`
Branch: `t3code/curate-collections-navigation-tail`
Worker base: `73b285a1a0de7cfd46625c4c7611f1964b47f566` (`origin/main` at
dispatch; confirmed as an ancestor)

## Summary

Ten collection, navigation, overlay, layout, media, and date-time Examples
pages now follow the card's ordered 2–6 group outline. Svelte and React
captions match verbatim. GPUI uses the same ordered captions. Accordion,
ListCardCounter, and MediaPreview stayed at their preserved web outlines;
GPUI Examples on those pages dropped axis and showcase extras so they match.
Tree gained a React flat-tree case and an explicit Disabled nodes group on
every host. No component, contract, token, or public API change.

## Change class

- **Change class:** specimen curation
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-gpui-preview`
- **Public entry points:** none
- **Downstream re-check:** none
- **app_state.rs:** unchanged

## Baseline recount at the worker base

Web counts matched the card. GPUI was recounted from `origin/main` Examples
captions (excluding size/density axis panes). Dialog GPUI had no extractable
group helper strings at the base — the page was a trigger cluster rather than
six named teaching groups.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| Accordion | 2 | 2 | 5 | preserve web; GPUI had size/density inside Examples |
| Dialog | 9 | 9 | — | curate to six live-trigger groups |
| FilterBuilder | 7 | 7 | 4 | curate; keep controlled readout and field types |
| ListCard | 19 | 19 | 20 | heavy curate; keep every contract story |
| ListCardCounter | 2 | 2 | 2 | preserve dedicated page; GPUI group 2 was Inherited typography |
| MediaPreview | 3 | 3 | 5 | preserve web; GPUI Examples mixed document/states/variants |
| SplitView | 7 | 7 | 7 | combine collapse; leave known divider/seam debt alone |
| Stepper | 8 | 8 | 8 | combine orientation, collapsed, and running/failed |
| TimeAgo | 7 | 7 | 6 | combine recent/future, prose, long/static |
| Tree | 8 | 7 | 5 | remove paired-web drift; give disabled nodes their own group |

## After

| Page | Svelte | React | GPUI |
| --- | ---: | ---: | --- |
| Accordion | 2 | 2 | 2 |
| Dialog | 6 | 6 | 6 |
| FilterBuilder | 5 | 5 | 5 |
| ListCard | 6 | 6 | 6 |
| ListCardCounter | 2 | 2 | 2 |
| MediaPreview | 3 | 3 | 3 |
| SplitView | 6 | 6 | 6 |
| Stepper | 5 | 5 | 5 |
| TimeAgo | 4 | 4 | 4 |
| Tree | 6 | 6 | 6 |

Svelte and React captions are verbatim identical on every page. GPUI uses the
same ordered captions. Tree GPUI Loading and large data is lazy-only; native
Tree has no virtualized window, which is host-truthful.

## Final ordered captions

**Accordion** — Single selection; Multiple selection

**Dialog** — Basic and alert dialogs; Forms and nested controls; Custom
header and footer; Bare content; Scrolling and width presets; Dismissal rules

**FilterBuilder** — Building filters; Match all and match any; Empty and
limited builders; Field types and overflow; Disabled

**ListCard** — Interactive rows; Hierarchy and selection; Leading content
and layout; Badges and counters; Visual status; Actions and static use

**ListCardCounter** — Static footer counters; Linked footer counter

**MediaPreview** — Image preview; Video preview; Error state

**SplitView** — Horizontal split; Vertical split; Collapse controls;
Hover-revealed controls; Nested workspace; Disabled

**Stepper** — Guided workflow; Collapsed progress; Running and failed
states; Re-run; Disabled

**TimeAgo** — Recent and future timestamps; In running prose; Long and
static formats; ISO input

**Tree** — File explorer; Selection modes; Presentation options; Loading
and large data; Editing and reordering; Disabled nodes

## Named removals and combinations

- **Accordion (GPUI)** — Sizes and Densities left Examples. They already live
  on the axis panes. Web page unchanged.
- **Dialog — nine → six.** Informational dialog + Dialog `role="alertdialog"`
  share **Basic and alert dialogs** (not the AlertDialog component). Nested
  popover stays in **Forms and nested controls**. Width and scrolling share
  one group. Every trigger stays live.
- **FilterBuilder — seven → five.** Combinator cases share **Match all and
  match any**. Empty + max-clause share **Empty and limited builders**. Enum,
  multi-enum, boolean, text, number, range, and overflow share **Field types
  and overflow**. Controlled value readout stays in **Building filters**.
- **ListCard — nineteen → six.** Inherited footer-counter copies, the legacy
  wrapped-context-menu path, and separate Highlighted/Active sections left
  Examples. Highlighted + active remain inside **Visual status**. Last-click
  readout stays inside **Interactive rows**, not a seventh group. Contract
  stories (interactive, hierarchy, rounded-square leading, badges, footer
  counters, solid accent, context-menu, not-live, sash, static) remain.
- **ListCardCounter (GPUI)** — Inherited typography became **Linked footer
  counter** (href), matching the dedicated-page outline.
- **MediaPreview (GPUI)** — Document, loading/empty, and variant copies left
  Examples. Image, video, and error remain.
- **SplitView — seven → six.** Horizontal and vertical collapse share
  **Collapse controls**. GPUI extra "Drag to resize" eyebrow became helper
  text so it is not a seventh caption. React `divider`/seam and native
  both-collapsed debt were not repaired.
- **Stepper — eight → five.** Horizontal + vertical share **Guided
  workflow**. Collapsed + mixed-status collapsed share **Collapsed
  progress**. Working + failed share **Running and failed states**.
- **TimeAgo — seven → four.** Recent + future share **Recent and future
  timestamps**. Surrounding copy + inherit share **In running prose**. Long
  format + static share **Long and static formats**.
- **Tree — eight/seven → six.** Multi-select + checkbox cascade share
  **Selection modes**. Flat-tree + no-guides/no-icons share **Presentation
  options**. React gained the missing flat-tree (`collapseTwistyWhenFlat`).
  Lazy + virtualized share **Loading and large data** (GPUI lazy only).
  Rename/reorder stay in **Editing and reordering**. `node_modules`
  `disabled` moved into its own **Disabled nodes** group.

## Contract coverage

Preserved. Size and density ladders stay in dedicated panes. No public
component prop or token changed. Tree work is specimen-only. SplitView known
parity debt is recorded, not closed.

## Changed files

- `packages/svelte/preview/src/specimens/{Dialog,FilterBuilder,ListCard,SplitView,Stepper,TimeAgo,Tree}Specimen.svelte`
- `packages/react/preview/src/gallery/specimens/{Dialog,FilterBuilder,ListCard,SplitView,Stepper,TimeAgo,Tree}Specimen.tsx`
- `packages/gpui/preview/src/specimens/{accordion,dialog,filter_builder_specimen,list_card,list_card_counter,media_preview_specimen,split_view_specimen,stepper,time_ago_specimen,tree}.rs`
- `test/parity/g15-025-collections-navigation-tail-specimens.test.tsx`
- this log

Accordion, ListCardCounter, and MediaPreview web specimens were remeasured and
left in place.

## Validation

- focused `g15-025` parity regression: 64 passed
- `effigy check:svelte`: 0 errors
- `effigy react:build`: passed
- `effigy check:gpui`: passed (`poodle-gpui-preview` compiled)
- `effigy catalogue:check`: passed
- `effigy docs:check`: passed
- `git diff --check origin/main...HEAD`: passed

Headless only. No windowed, native-visual, conformance, Jetstream, or
release selectors. `effigy test:parity` was not re-run in full; the focused
file is the card's required regression.

## Operator review

Paired Svelte/React live review is pending. Unreviewed pages stay explicit in
the PR. Do not merge until the operator inspects the ten pages.
