# g15.002 — Svelte focused evidence: foundation display & shell primitives

Status: complete — all three batches landed (PR #25)
Date: 2026-08-16
Card: `docs/roadmaps/g15/002-svelte-focused-evidence-display-shell.md`
Governing refs: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`,
`docs/roadmaps/g15/release-baseline-roster.md`,
`docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`

## Batches

The card's three named batches were executed in order, each with a narrow test
round at the end. The register's roster and register evidence rows were updated
once all three batches were green.

- **Batch A — layout & content primitives (10):** Avatar, Box, Card, Code,
  Eyebrow, Grid, Region, Skeleton, Spacer, Stack
- **Batch B — disclosure, navigation & feedback (10):** Accordion,
  Breadcrumbs, CollapseToggle, Collapsible, DetailItem, HoverCard, NavCard,
  Progress, Spinner, Stepper
- **Batch C — selection, chrome & alerting (9):** AlertDialog, BulkActionBar,
  Callout, ListCardCounter, ListGrid, MetaBar, MetaItem, Pill, Rating

## Evidence Landed

Every scoped component now has a named focused test case on the Svelte side
(`packages/svelte/components/test/<Name>.test.ts`) and the mirrored contract
cases on the React side (`packages/react/components/test/<Name>.test.tsx`),
asserting load-bearing observable contract behaviour: state transitions,
emitted events, keyboard/accessibility projection, composed-token output, or
layout intent. The anatomy smoke (`smoke.test.ts`) is not reused as evidence;
each new file asserts behaviour beyond mounting.

Svelte file / React file per component (also recorded in the roster):

| Component | Svelte evidence | React evidence |
| --- | --- | --- |
| Avatar | `Avatar.test.ts` | `Avatar.test.tsx` |
| Box | `Box.test.ts` | `Box.test.tsx` |
| Card | `Card.test.ts` | `Card.test.tsx` |
| Code | `Code.test.ts` | `Code.test.tsx` |
| Eyebrow | `Eyebrow.test.ts` | `Eyebrow.test.tsx` |
| Grid | `Grid.test.ts` | `Grid.test.tsx` |
| Region | `Region.test.ts` | `Region.test.tsx` |
| Skeleton | `Skeleton.test.ts` | `Skeleton.test.tsx` |
| Spacer | `Spacer.test.ts` | `Spacer.test.tsx` |
| Stack | `Stack.test.ts` | `Stack.test.tsx` |
| Accordion | `Accordion.test.ts` | `Accordion.test.tsx` |
| Breadcrumbs | `Breadcrumbs.test.ts` | `Breadcrumbs.test.tsx` |
| CollapseToggle | `CollapseToggle.test.ts` | `CollapseToggle.test.tsx` |
| Collapsible | `Collapsible.test.ts` | `Collapsible.test.tsx` |
| DetailItem | `DetailItem.test.ts` | `DetailItem.test.tsx` |
| HoverCard | `HoverCard.test.ts` | `HoverCard.test.tsx` |
| NavCard | `NavCard.test.ts` | `NavCard.test.tsx` |
| Progress | `Progress.test.ts` | `Progress.test.tsx` |
| Spinner | `Spinner.test.ts` | `Spinner.test.tsx` |
| Stepper | `Stepper.test.ts` | `Stepper.test.tsx` |
| AlertDialog | `AlertDialog.test.ts` | `AlertDialog.test.tsx` |
| BulkActionBar | `BulkActionBar.test.ts` | `BulkActionBar.test.tsx` |
| Callout | `Callout.test.ts` | `Callout.test.tsx` |
| ListCardCounter | `ListCardCounter.test.ts` | `ListCardCounter.test.tsx` |
| ListGrid | `ListGrid.test.ts` | `ListGrid.test.tsx` |
| MetaBar | `MetaBar.test.ts` | `MetaBar.test.tsx` |
| MetaItem | `MetaBar.test.ts` (family cases) | `MetaBar.test.tsx` (family cases) |
| Pill | `Pill.test.ts` | `Pill.test.tsx` |
| Rating | `Rating.test.ts` | `Rating.test.tsx` |

Supporting harnesses: `CardRegionsHarness.svelte` (compiled snippet regions
for Card; raw thunks cannot materialize text under the Svelte 5 runtime) and
`MetaBarPillHarness.svelte` (pill-context inheritance into composed pills).

Representative load-bearing cases per family (full list lives in the files):

- **Batch A**: Avatar initials trim to three uppercase characters and project
  `role="img"`/`aria-label`, decorative mode hides from assistive tech; Box and
  Grid map `SpaceScale` to token variables and opt-in `asRole`/`ariaLabel`;
  Card projects variant/layout/interactive/selected and renders snippet regions
  conditionally; Code splits block lines with 1-based numbering, highlights
  requested lines, swaps the copy button to "Copied" after writing the source
  to the clipboard; Eyebrow renders the requested heading element; Region
  stays `role="presentation"` with min-height/`--region-color`; Skeleton
  resolves circle shape to rem dimensions and preset line counts; Spacer
  applies the grow factor and two-axis minSize; Stack maps direction/gap/
  align/justify/wrap onto the flex container.
- **Batch B**: Accordion single/multiple selection, `collapsible=false`
  deactivation guard, disabled items; Breadcrumbs last-item `aria-current`,
  href/anchor vs button rendering, `onNavigate`, overflow ellipsis;
  CollapseToggle is prop-driven (`onToggle` emits `!collapsed` on every click);
  Collapsible defaultOpen/controlled states and disabled guard; DetailItem
  empty-text fallback, span/layout data, truncate class, action region;
  HoverCard controlled open surface with dialog semantics, Escape close,
  trigger expansion; NavCard anchor/button dual root and disabled guard;
  Progress clamped `aria-valuenow`/computed `aria-valuetext`, indeterminate
  mode; Spinner ring/dots/grid anatomy and live `status` vs `aria-hidden`;
  Stepper first-step fallback, status-suffixed accessible names, rerun
  triggers, vertical-only collapse.
- **Batch C**: AlertDialog `role="alertdialog"`, confirm/cancel flows,
  working state that holds the dialog; BulkActionBar summary/total, per-action
  gating at zero selection, select-all affordance, action-id and clear events;
  Callout tone data, `announceMode` → `alert`/`status` live regions, dismiss;
  ListCardCounter count+icon, anchor/span roots, `onClick`, tooltip wrap;
  ListGrid auto-fill/maxColumns formulas, compact single-column, actions
  header; MetaBar/MetaItem data attributes and pill-context inheritance;
  Pill tone/appearance/size/accent/muted/adaptive-width projection; Rating
  whole-star radios with `aria-checked`, fractional slider mode, allowClear,
  disabled guard.

## Contract Ambiguity Resolved (no fix, no contract change)

- **ListCardCounter `onClick` scope** — an initial reading suggested the
  unlinked counter should also invoke `onClick` (the component's shared
  `handleClick` stops propagation conditionally on `href` and invokes the
  callback unconditionally), and a first attempt wired the handler onto the
  span branches on both runtimes. Review of the contract's full text changed
  that call: the props table scopes `onClick` to linked counters ("invoked
  after `stopPropagation` runs for linked counters", §3), the event table's
  "For linked counters … the `onClick` prop (if provided) is then invoked"
  (§5) reads the same way, the Jetstream note makes it explicit ("an unlinked
  counter is a statistic, not a control", §10a), and both web runtimes
  shipped the same linked-only wiring — with Svelte the reference, the
  runtimes were not in disagreement. The speculative wiring was reverted on
  both sides; no implementation or contract change was made. The evidence
  cases assert the documented behaviour: a linked counter invokes `onClick`,
  an unlinked counter does not (it stays a statistic).

## Bounded Fixes (contract-first)

No scoped implementation defect was found. Every final assertion passes
against the shipped implementations, and no contract changed for any
component.

## Observations (no change made)

- Svelte's `transition:slide` on Accordion panels and Collapsible content
  requires the Web Animations API, which happy-dom does not implement; the
  existing `Element.prototype.animate` polyfill precedent from
  `DrawerDismissOutside.svelte.test.ts` was reused. The friction is already
  recorded in `PAPERCUTS.md` (2026-08-14).
- Svelte 5 test thunks (`asSnippet`) cannot materialize text content under
  happy-dom — a snippet thunk returning a string renders as a comment node.
  Content-bearing assertions use compiled snippet harnesses instead
  (`CardRegionsHarness.svelte` uses `createRawSnippet`, which keeps the
  branded type through conditional props and materializes text).
- `Rating.svelte` compiles a `binding_property_non_reactive` warning for
  `bind:this={itemElements[index]}`. Pre-existing, non-behavioural, and not
  within the card's fix scope; recorded here for the native/gpui passes.

## Validation

| Command | Result |
| --- | --- |
| Batch A narrow round (`vitest run` svelte-components + react-components, touched files) | pass (33 + 33 tests) |
| Batch B narrow round (touched files) | pass (39 + 39 tests) |
| Batch C narrow round (touched files) | pass (31 + 31 tests) |
| `effigy check:svelte` | pass |
| `effigy react:build` | pass |
| `effigy test:components` | pass |
| `effigy docs:check` | pass |
| `git diff --check` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Register and Roster Updates

- `release-baseline-roster.md`: the 29 components' Focused Svelte test cells
  and Focused React test cells now name the case files; summary counts moved
  to Focused Svelte 90 present / 85 missing and Focused React 87 present /
  88 missing.
- `release-gap-register.md`: the Svelte focused-evidence blocker class count
  moved 114 → 85; the "Foundation display & shell" family row (29 components)
  is closed with evidence recorded in the roster. No status line was changed.
- `docs/roadmaps/g15/002-…` card, `README.md`, and `dispatch.md` were not
  modified by the worker.

## Change Footprint

`packages/svelte/components/test/` (28 new test files + 2 harnesses) and
`packages/react/components/test/` (29 new test files), plus the two
focused-evidence docs. No component implementation, contract, specimen,
package export, workflow, or downstream repository changed.
