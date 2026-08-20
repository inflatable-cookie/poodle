# g15.025 — Overloaded Examples: collections, navigation and the long tail

Status: **complete in PR #49** — operator sign-off and CI repair accepted
Parent: `018-overloaded-examples-curation.md` (method, acceptance, stop
conditions)
Consumes: `g15.011` partial screening baseline
Depends on: `g15.016` and `g15.019` (complete)
Governing refs: `specimen-catalogue-audit.md`, `specimen-plan-outline.md`, and
the ten component contracts named by this card

## Scope

Catalogue families: `data-collections`, `navigation`, `overlays-disclosure`,
`layout`, `media`, and `date-time`.

### Pages this card owns (10)

- `Accordion`
- `Dialog`
- `FilterBuilder`
- `ListCard`
- `ListCardCounter`
- `MediaPreview`
- `SplitView`
- `Stepper`
- `TimeAgo`
- `Tree`

This list remains exact. `g15.016` gave `ListCardCounter` its own page and
converged Dialog captions; `g15.019` repaired native captions and axis panes.
The worker records those prerequisite changes rather than silently shrinking
the original audit partition.

No component, contract, token, or public API change is in scope.

## Remeasured Baseline — 2026-08-20

Counts are captioned groups in `Examples`, excluding size and density panes.

| Page | Svelte | React | Current ruling |
| --- | ---: | ---: | --- |
| Accordion | 2 | 2 | preserve; already concise and contract-shaped |
| Dialog | 9 | 9 | curate |
| FilterBuilder | 7 | 7 | curate |
| ListCard | 19 | 19 | curate heavily |
| ListCardCounter | 2 | 2 | preserve dedicated page from `g15.016` |
| MediaPreview | 3 | 3 | preserve; already matches its contract |
| SplitView | 7 | 7 | curate |
| Stepper | 8 | 8 | curate |
| TimeAgo | 7 | 7 | curate |
| Tree | 8 | 7 | curate and remove paired-web drift |

GPUI must be remeasured and recorded by the worker. It teaches the same
ordered intent, but its groups need not share web-only host mechanics.

## Exact Teaching Outline

Use these ordered `Examples` groups. A group may contain several small cases;
it is one reader question, not one prop value.

### Accordion

1. Single selection
2. Multiple selection

### Dialog

1. Basic and alert dialogs
2. Forms and nested controls
3. Custom header and footer
4. Bare content
5. Scrolling and width presets
6. Dismissal rules

The normal dialog comes first. Keep every trigger live. The alert case uses
Dialog's documented `role="alertdialog"`; it does not substitute the separate
AlertDialog component.

### FilterBuilder

1. Building filters
2. Match all and match any
3. Empty and limited builders
4. Field types and overflow
5. Disabled

Keep the controlled value readout in the first group. Across these groups,
retain the contract's enum, multi-enum, boolean, text, number, range,
`allowMultiple`, max-clause, edit/remove/clear, and overflow teaching.

### ListCard

1. Interactive rows
2. Hierarchy and selection
3. Leading content and layout
4. Badges and counters
5. Visual status
6. Actions and static use

Preserve the contract's interactive, hierarchy, rounded-square leading,
badges, footer counters, solid accent, context-menu, not-live, sash, and static
cases inside those groups. Remove redundant showcase-only copies: inherited
counter repetition, the legacy wrapped-context-menu path, and separate
highlighted/active groups do not each earn a page section. Keep useful click,
selection, reorder, and context-menu feedback live.

### ListCardCounter

1. Static footer counters
2. Linked footer counter

### MediaPreview

1. Image preview
2. Video preview
3. Error state

### SplitView

1. Horizontal split
2. Vertical split
3. Collapse controls
4. Hover-revealed controls
5. Nested workspace
6. Disabled

Put horizontal and vertical collapse cases in the one collapse group. Do not
repair or imply closure of SplitView's known React `divider`/seam and native
both-collapsed deltas; those are component-parity work outside this card.

### Stepper

1. Guided workflow
2. Collapsed progress
3. Running and failed states
4. Re-run
5. Disabled

The normal Soundcheck-style workflow comes first. Combine horizontal/vertical
orientation in that group, collapsed/collapsed-statuses in the second, and
working/failed in the third. Preserve live selection, collapse, and rerun
feedback.

### TimeAgo

1. Recent and future timestamps
2. In running prose
3. Long and static formats
4. ISO input

Keep relative time as the primary teaching surface. Combine the surrounding
copy cases under running prose; do not replace them with static dates.

### Tree

1. File explorer
2. Selection modes
3. Presentation options
4. Loading and large data
5. Editing and reordering
6. Disabled nodes

Selection modes combines multi-select and checkbox cascade. Presentation
options combines flat-tree and no-guides/no-icons. Loading combines lazy and
virtualized cases. Make the missing React flat-tree case and the missing
explicit disabled-node case honest without changing Tree itself.

## Goals

- [x] Every page uses the exact ordered outline above.
- [x] Svelte and React captions and teaching copy match; GPUI teaches the same
      ordered intent.
- [x] The first group answers normal use rather than opening on an edge case.
- [x] Removed and combined examples have named dispositions in the batch log.
- [x] Contract coverage is unchanged or any pre-existing gap is reported.
- [x] No previously wired control regressed. GPUI Stepper selection/re-run was
      already unwired; it is a named release follow-up rather than a curation
      repair.

## Acceptance

- [x] Every page has 2–6 captioned `Examples` groups.
- [x] Accordion, ListCardCounter, and MediaPreview are remeasured and preserved
      unless a concrete defect is found.
- [x] Dedicated size and density panes remain outside `Examples`.
- [x] Paired-web caption parity and the ten-page count/outline receive focused
      regression evidence.
- [x] The changed pages are reviewed live in the Svelte and React previews by
      the operator before this card is called complete. Unreviewed pages stay
      explicit in the PR.

## Stop Conditions

- A target outline requires a component, contract, token, or public API change.
- Curation would remove the only evidence for contract behaviour without a
  named replacement.
- SplitView's known parity debt, Tree runtime behaviour, or another component
  defect becomes implementation work. Record it and return to the
  orchestrator.
- The work grows beyond the ten named pages or becomes an exhaustive reference
  corpus.

## Writable Scope

- the ten named specimen files across Svelte, React, and GPUI
- focused preview/parity evidence for this card
- one August batch log

Planning front doors, component implementations, contracts, shared specimen
infrastructure, and unrelated pages stay orchestrator-owned or out of scope.

## Validation

- focused ten-page outline/caption parity regression
- focused component tests only when a retained interaction needs evidence
- `effigy check:svelte`
- `effigy react:build`
- `effigy check:gpui`
- `effigy catalogue:check`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run windowed, native-visual, or Jetstream selectors.

## Continuation

PR #49 accepted worker head `73211b7a` and merged as `cd952cdc`. Review added
truthful GPUI FilterBuilder, Dialog, and Tree teaching, bounded the large Tree,
and retained the existing Stepper interaction gap explicitly. A tracked GPUI
preview lockfile repaired fresh-runner CI after the `arrayref` yank; GitHub
`active-cohort` passed. This closes the last `g15.018` child. `g15.026`, the
six screen-clear review cards, and release certification remain separate.
