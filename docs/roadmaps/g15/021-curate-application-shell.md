# g15.021 — Overloaded Examples: application shell

Status: **active** — PR review remediation in progress on 2026-08-19
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions — this card does not restate them)
Consumes: `g15.011` partial screening baseline
Depends on: `g15.020` (complete)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`, and
the seven component contracts named below

## Scope

Application-shell pages whose teaching surfaces grew alongside the composites
they host.

Catalogue family: `application-shell`.

### Pages this card owns (7)

- `ActionDiscoveryPanel`
- `DetailSection`
- `DetailSectionGroup`
- `DetailShell`
- `DockRegion`
- `HistoryCenter`
- `PageHeader`

This list is exact and exhaustive. It preserves the parent's original
partition even where intervening caption and axis work has already brought a
page inside budget. No other card owns these pages, and this card owns no
others.

The worker scope changes no component, component contract, shared specimen
shell, or public API. The dated orchestrator amendment below narrowly extends
that boundary for the pre-existing Rust HistoryCenter delete gap found during
review.

## Remeasured Baseline

Counts below are visible `Examples` captions on current `main` after
`g15.015`–`g15.020` and `g15.034`. Axis panes are excluded.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| ActionDiscoveryPanel | 3 | 3 | 4 | keep the web budget; converge state teaching |
| DetailSection | 4 | 4 | 5 | keep the web budget; converge and wire the action |
| DetailSectionGroup | 3 | 3 | 3 | verified no-op |
| DetailShell | 4 | 4 | 4 | keep the outline; repair dead Edit/Reset controls |
| DockRegion | 9 | 8 | 4 | curate and converge |
| HistoryCenter | 9 | 9 | 1 | curate and converge |
| PageHeader | 8 | 8 | 8 | curate, converge, and repair dead actions |

The earlier audit counts included axis rows that now live in their own panes.
This card uses the current page shape rather than replaying that stale total.
The remaining interaction findings are still live: DetailShell's Edit/Reset
and PageHeader's header actions have no host-visible effect.

## Target Teaching Outline

Use these sections in this order. Svelte and React captions and explanatory
copy stay verbatim. GPUI teaches the same ordered sections and meaning.
Several component instances may share a section only when they answer the same
question and remain visually distinct.

| Page | Target sections |
| --- | --- |
| ActionDiscoveryPanel | Grouped actions; Descriptions, badges, and shortcuts; Loading and empty states |
| DetailSection | Project details; Section actions; Described detail rows; Two-column details |
| DetailSectionGroup | keep: Grid layout; Stack layout; Column cap |
| DetailShell | keep: Layout structure; Multi-section layout with header; Loading state; Error state |
| DockRegion | Expanded side dock; Collapse and edge placement; Tab strip presentation; Move panels between docks; Static panel stacks |
| HistoryCenter | Linear history; Choosing between continuations; Nested continuation runs; Single continuation and run boundaries; Rename and manage a continuation; Failure and incomplete metadata |
| PageHeader | Page title and summary; Navigation and actions; Hierarchy and count; Contextual status; Operational metadata |

## Required Story Mapping

The reduction must preserve these stories rather than merely preserving a
caption count.

- ActionDiscoveryPanel keeps a realistic grouped default, description/badge/
  shortcut content, and both loading and empty postures. Selecting an action
  produces visible specimen feedback in both web runtimes and an equivalent
  host-state readout in GPUI.
- DetailSection keeps titled/description content, section actions, rich
  DetailItem rows, and fixed two-column layout. The GPUI-only description-only
  fixture may leave the catalogue only after its contract/focused-test
  evidence is named in the batch log. The retained action produces visible
  feedback rather than logging or doing nothing.
- DetailSectionGroup is a true no-op: preserve its three captions and sources
  unless verification finds drift.
- DetailShell keeps the current structure, realistic composed page, loading,
  and error surfaces. Edit and Reset update visible host-owned specimen state;
  they must not remain decorative controls.
- DockRegion keeps every current teaching claim inside five coherent groups:
  expanded flexible layout; interactive side and bottom collapse; iconless
  narrow fallback plus tab pass-through presentation; cross-region transfer;
  and both static stack directions. Collapse, selection, reorder, and transfer
  remain wired. Do not trade the iconless regression story or the g13.040 tab
  pass-through story for a prettier page.
- HistoryCenter maps all nine current fixtures into the six named questions:
  linear; two forks; fork off fork; single continuation; run tail; open single
  fork; rejection; missing timestamp; rename. The page may share fixtures and
  host-feed helpers, but may not collapse these into one opaque mega-example.
  Captions use reader language, not fixture IDs. Existing host feeds and
  continuation actions remain live.
- PageHeader keeps the basic/title-only, back/actions, eyebrow, count,
  section/banner, breadcrumbs, and MetaBar stories inside five groups. Every
  retained button action updates visible specimen feedback. Back links and
  breadcrumbs remain real navigation rather than fake buttons.

Focused component tests remain the exhaustive behavior authority. The
catalogue teaches representative use; it does not become a new test corpus.

## Goals

- [ ] Every page in the group meets the parent's method.
- [ ] Svelte and React stay identical; GPUI teaches the same ordered set.
- [ ] Every retained action control has an observable specimen effect.
- [ ] Removals are named, with contract coverage checked first.

## Evidence

- Add `test/parity/g15-021-application-shell-specimens.test.tsx` for this exact
  seven-page set.
- Assert final ordered captions, paired Svelte/React equality, the 3–6 section
  budget, and DetailSectionGroup's verified no-op status.
- Assert the contract-critical stories beyond captions: ActionDiscoveryPanel's
  state pairing and selection feedback; DetailSection, DetailShell, and
  PageHeader action feedback; DockRegion's retained layout/interaction groups;
  and HistoryCenter's nine-to-six fixture mapping and live host feeds.
- Record the final GPUI caption order for all seven pages with deterministic
  structural evidence. `g15.026` still owns the native page probe; do not build
  it here.
- The August batch log maps every removed or combined caption to retained
  catalogue coverage, focused behavior evidence, or an explicit coverage note.

## Acceptance

Per the parent, including its operator-review checkpoint: **every changed web
page is reviewed live in the Svelte and React previews before this card is
called complete.** DetailSectionGroup requires verification, not churn. Any
unreviewed changed page remains an explicit PR item.

## Writable Scope

- the seven named specimen files under each of:
  - `packages/svelte/preview/src/specimens/`
  - `packages/react/preview/src/gallery/specimens/`
  - `packages/gpui/preview/src/specimens/`
- `packages/gpui/preview/src/app_state.rs` only for specimen-local state needed
  to make a retained GPUI action visibly live; record any use in the batch log
- `test/parity/g15-021-application-shell-specimens.test.tsx`
- one August batch log

Do not edit components, component contracts, shared specimen shells,
catalogue navigation, generated scene infrastructure, or pages owned by
another child.

## Orchestrator Remediation Amendment — 2026-08-19

PR review found that the existing HistoryCenter contract already requires
delete parity, but the Rust headless machine and shared renderer never carried
that command or its confirmation surface. Hiding delete on web would reduce
the catalogue to match a substrate defect. The operator authorised the
orchestrator to fix the defect and merge this PR.

This amendment adds these exact writable files:

- `packages/contracts/headless/src/history_center.rs`
- `packages/render/src/history_center.rs`
- `packages/render/src/lib.rs`
- `docs/contracts/components/history-center.md`

The fix must port the existing TS delete transition, keep delete opt-in, render
one shared danger confirmation dialog, emit nothing on request/cancel, emit
the selected entry only on confirm, invalidate the affected level, and reload
its anchor. Direct Rust machine/renderer tests are required. No other component
or public surface beyond the necessary pre-v1 HistoryCenter Rust types may
change; downstream source impact must be recorded under spec 022.

## Validation

- focused `g15.021` parity regression
- `effigy test:parity`
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run windowed, native-visual, conformance, Jetstream, or
release selectors.

## Stop Conditions

In addition to the parent stop conditions, stop if:

- making a retained control work requires a component or public-contract
  change rather than specimen-local host state;
- preserving DockRegion's iconless, pass-through, collapse, static, or transfer
  stories cannot fit the five named groups without an unclear slab;
- HistoryCenter cannot retain all nine named fixture claims inside six clear
  questions, or a host feed stops being interactive;
- Svelte and React need different section order, copy, or behavior;
- GPUI cannot teach the same outline without component work or the `g15.026`
  page probe;
- work escapes the exact seven-page set.

## Continuation

Push one PR and stop for orchestrator review. The changed web pages require
live paired-preview operator review before merge. `g15.022` is the next
curation child; do not absorb it into this run.
