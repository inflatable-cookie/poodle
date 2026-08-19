# g15.021 — Application Shell Curation (August batch log)

Date: 2026-08-19
Card: `docs/roadmaps/g15/021-curate-application-shell.md`
Parent: `docs/roadmaps/g15/018-overloaded-examples-curation.md`
Handoff: `docs/handoffs/20260819-090350-g15-021-application-shell-curation.md`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-e18e170e`
Branch: `t3code/curate-application-shell`
Worker base: `ecbaa9f539015532ee488c7411291aa01e09aadd` (`origin/main` at dispatch;
handoff planning base `055641feebceeef91db0fc4678a01c8f498b04f9` is an ancestor)

## Summary

Application-shell catalogue pages curated to the outline's 3–6 section
budget across Svelte, React, and GPUI. DetailSectionGroup verified no-op.
Dead Edit/Reset and PageHeader actions now update visible specimen feedback.
HistoryCenter fixture IDs become six plain-language questions. Review then
closed the pre-existing Rust/GPUI delete-parity gap named by the contract.

## Change class

- **Change class:** specimen curation plus behavioral parity correction. The
  Rust HistoryCenter surface is additive in capability but source-breaking for
  exhaustive enum matches and struct literals without `..Default`; this is the
  operator-approved pre-v1 migration, with no compatibility shim.
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-gpui-preview`, `poodle-headless`, and `poodle-render`
- **Public entry points:** `poodle-render` adds `HistoryCenterDelete` and delete
  request/confirm/cancel handler fields; `poodle-headless` adds the documented
  delete event/effect variants
- **Downstream re-check:** direct Rust consumers must recompile exhaustive
  `HistoryCenterEvent` / `HistoryCenterEffect` matches and any literal
  `HistoryCenterView` / `HistoryCenterHandlers` construction. GPUI consumers
  should confirm request → dialog → confirm/cancel timing and anchor reload.
- **app_state.rs:** unused — specimen-local keys go through existing
  `specimens.text` / `node_events`

## Baseline recount at the worker base

Matched the card's remeasured table.

| Page | Svelte | React | GPUI | Ruling |
| --- | ---: | ---: | ---: | --- |
| ActionDiscoveryPanel | 3 | 3 | 4 | keep web budget; converge state teaching |
| DetailSection | 4 | 4 | 5 | keep web budget; converge; wire action |
| DetailSectionGroup | 3 | 3 | 3 | verified no-op |
| DetailShell | 4 | 4 | 4 | keep outline; repair Edit/Reset |
| DockRegion | 9 | 8 | 4 | curate and converge |
| HistoryCenter | 9 | 9 | 1 | curate and converge |
| PageHeader | 8 | 8 | 8 | curate, converge, repair actions |

## After

| Page | Svelte | React | GPUI |
| --- | ---: | ---: | ---: |
| ActionDiscoveryPanel | 3 | 3 | 3 |
| DetailSection | 4 | 4 | 4 |
| DetailSectionGroup | 3 | 3 | 3 |
| DetailShell | 4 | 4 | 4 |
| DockRegion | 5 | 5 | 5 |
| HistoryCenter | 6 | 6 | 6 |
| PageHeader | 5 | 5 | 5 |

Web totals: 40 → 30 captions. GPUI: 29 → 30. All three runtimes agree on the
ordered section set for every page.

## Final ordered captions

Svelte and React are verbatim identical. GPUI teaches the same ordered set.

**ActionDiscoveryPanel** — Grouped actions; Descriptions, badges, and
shortcuts; Loading and empty states

**DetailSection** — Project details; Section actions; Described detail rows;
Two-column details

**DetailSectionGroup** (unchanged) — Grid layout; Stack layout; Column cap

**DetailShell** — Layout structure; Multi-section layout with header; Loading
state; Error state

**DockRegion** — Expanded side dock; Collapse and edge placement; Tab strip
presentation; Move panels between docks; Static panel stacks

**HistoryCenter** — Linear history; Choosing between continuations; Nested
continuation runs; Single continuation and run boundaries; Rename and manage
a continuation; Failure and incomplete metadata

**PageHeader** — Page title and summary; Navigation and actions; Hierarchy and
count; Contextual status; Operational metadata

## Old-to-new story map

### ActionDiscoveryPanel

| Old | Disposition |
| --- | --- |
| Grouped actions | retained (wired selection feedback) |
| With descriptions and badges | renamed → Descriptions, badges, and shortcuts |
| Empty state | combined with Loading into Loading and empty states |
| Loading (GPUI-only) | combined into Loading and empty states |

### DetailSection

| Old | Disposition |
| --- | --- |
| With title and rows | renamed → Project details |
| With actions | renamed → Section actions; Edit updates visible feedback |
| DetailItem with description | renamed → Described detail rows |
| Two-column details | retained |
| Description only (no title) (GPUI-only) | removed from catalogue; covered by `packages/svelte/components/test/DetailSection.test.ts` ("omits the header entirely without a title, description, or actions" / "renders an h3 title and description when provided") and React `DetailSection.test.tsx` |

### DetailSectionGroup

No-op. Captions and sources unchanged aside from GPUI "Column cap" label
parity (dropped parenthetical).

### DetailShell

| Old | Disposition |
| --- | --- |
| Layout structure | retained |
| Multi-section layout with header | retained; Edit toggles Theme Light/Dark; Reset restores defaults; both set Last action readout |
| Loading state | retained |
| Error state | retained |

### DockRegion

| Old | Disposition |
| --- | --- |
| Flexible dock — expanded | → Expanded side dock (with iconless narrow) |
| Flexible dock — icon-less panels, narrow | combined into Expanded side dock |
| Tab pass-throughs (g13-040) | → Tab strip presentation |
| Flexible dock — collapsed icon-strip | → Collapse and edge placement |
| Interactive collapse toggle | combined into Collapse and edge placement |
| Bottom edge dock | combined into Collapse and edge placement |
| Cross-region drag-and-drop | → Move panels between docks |
| Static dock — horizontal | → Static panel stacks |
| Static dock — vertical | combined into Static panel stacks |

React previously omitted the iconless example; it now matches Svelte.

GPUI tab strip section cannot mirror `tabActiveEdge` / `tabVariant`
pass-throughs (not on `DockRegionSpec`); uses Standard / Quiet / Strong +
`can_accept_panel` side-by-side variants. Cross-region transfer shows drop
affordance only — native panel drag is out of vocabulary (recorded in
dock-region contract §12).

### HistoryCenter

| Old fixture ID | New section |
| --- | --- |
| linear | Linear history |
| two-forks | Choosing between continuations |
| fork-off-fork | Nested continuation runs |
| single-continuation | Single continuation and run boundaries |
| run-tail | Single continuation and run boundaries |
| single-fork-open | Rename and manage a continuation (disclose + delete; rename sibling) |
| rename | Rename and manage a continuation |
| rejection | Failure and incomplete metadata |
| no-timestamp | Failure and incomplete metadata |

Mount-time auto-open of single-fork-open removed; examples stay interactable.
Host feeds remain live. Undo and navigate write a page-level Last host
command readout. Rename and delete write a section Last command readout. The
orchestrator remediation ports the existing delete transition into the Rust
machine and adds the opt-in shared confirmation surface, so GPUI now teaches
the same disclose/delete story without reducing the web catalogue.

### PageHeader

| Old | Disposition |
| --- | --- |
| Basic | → Page title and summary (with Title only) |
| Title only | combined into Page title and summary |
| With back link and actions | → Navigation and actions |
| With breadcrumbs | combined into Navigation and actions |
| With eyebrow and actions | → Hierarchy and count |
| With count | combined into Hierarchy and count |
| Section and banner | → Contextual status |
| With MetaBar | → Operational metadata |

Every retained IconButton action updates a visible Last action readout. Back
links and breadcrumbs stay real `href` navigation.

## Interaction wiring

- ActionDiscoveryPanel: `activeId` + `onItemSelect` → "Selected action: …"
- DetailSection: Edit → "Last action: Edit billing"
- DetailShell: Edit/Reset update Theme values and "Last action: …"
- PageHeader: IconButton actions → per-section Last action readout
- GPUI: selection/action keys via existing `specimens.text` /
  `NodeSpecimenEvent` (action-discovery-active, detail-section-action,
  detail-shell-*, page-header-*)

## Evidence

- `test/parity/g15-021-application-shell-specimens.test.tsx` — ordered captions,
  Svelte/React equality, 3–6 budget, GPUI structural captions, DetailSectionGroup
  no-op, ActionDiscoveryPanel selection feedback, DetailSection/DetailShell/
  PageHeader action feedback, HistoryCenter nine-to-six mapping, HistoryCenter
  undo/navigate/delete host feedback, DockRegion collapse state change, close,
  reorder, and cross-dock transfer, PageHeader Icon chevrons (no text `›`)

## Validation

- focused `g15.021` parity file: pass (52/52)
- `effigy test:parity -- g15-021`: pass (365)
- `effigy check:svelte`: 0 errors (existing warnings only)
- `effigy react:build`: pass
- `effigy check:gpui`: pass
- `effigy docs:check`: pass
- focused Rust HistoryCenter machine tests: pass (20)
- focused shared-renderer HistoryCenter tests: pass (15)
- `git diff --check origin/main...HEAD`: pass

Headless only. No windowed, native-visual, conformance, Jetstream, or release
selectors.

## Operator checkpoint disposition

The review report explicitly told the operator that live paired Svelte/React
review remained before merge. The operator then instructed the orchestrator to
fix the remaining issues and merge. That is an informed waiver of this card's
live-review checkpoint; no visual pass is claimed. Both preview apps loaded,
but collaborative-preview automation and the in-app browser fallback failed.
The deterministic headless gate below remained green.

## Unresolved / known gaps

- GPUI DockRegion tab pass-throughs and live cross-region panel transfer remain
  structural / affordance-only until native gesture vocabulary expands; expanded
  dock copy no longer claims closable/reorderable (web wires those handlers)
- `g15.026` still owns the native page probe
- Size/density axes for HistoryCenter and DockRegion are presentation-only (no
  enabled undo/redo/close/reorder without local handlers)

## Review remediation (2026-08-19)

First pass (a114a978): breadcrumbs, Examples undo/navigate, main DockRegion
close/reorder. Re-review still requested delete restoration and remaining
inert affordances.

Second pass:

1. HIGH — restore web `onDeleteContinuation` + Last command feedback; record
   GPUI delete as planning-blocked (not catalogue normalization)
2. MEDIUM — strip enabled undo/redo from HistoryCenter size/density axes
   (Svelte/React/GPUI)
3. MEDIUM — transfer docks and size/density axes no longer mark closable without
   handlers; focused tests assert collapse `data-collapsed`, reorder tab order,
   transfer panel counts, and delete confirmation
4. LOW — batch log recount + explicit GPUI delete prerequisite, resolved by
   the orchestrator remediation below

## Orchestrator remediation

The operator authorised the remaining native parity fix on the PR. The
g15.021 roadmap now carries the narrow scope amendment.

- `poodle-headless::history_center` now handles `DeleteContinuation`: it
  accepts only a fork offered by an open picker, emits delete + anchor reload,
  and invalidates the cached level so deleted entries cannot keep rendering
- `poodle-render::history_center` now exposes an opt-in danger Delete item,
  host-owned `HistoryCenterDelete` target state, and one shared AlertDialog;
  request/cancel emit no destructive command and confirm names the selected
  entry exactly once
- the GPUI manage specimen wires request, confirm, cancel, and visible
  `delete <entry>` feedback; other GPUI HistoryCenter stories do not grow a
  delete item
- direct Rust tests cover opt-in rendering, confirmation timing, exact payload,
  level invalidation/reload, and closed/unknown inertness

## Changed files

- `packages/svelte/preview/src/specimens/{ActionDiscoveryPanel,DetailSection,DetailShell,DockRegion,HistoryCenter,PageHeader}Specimen.svelte`
- `packages/react/preview/src/gallery/specimens/{ActionDiscoveryPanel,DetailSection,DetailShell,DockRegion,HistoryCenter,PageHeader}Specimen.tsx`
- `packages/gpui/preview/src/specimens/{action_discovery_panel,detail_section_specimen,detail_section_group_specimen,detail_shell,dock_region,history_center_specimen,page_header_specimen}.rs`
- `packages/gpui/preview/src/specimens/mod.rs`
- `packages/contracts/headless/src/history_center.rs`
- `packages/render/src/{history_center,lib}.rs`
- `docs/contracts/components/history-center.md`
- `docs/roadmaps/g15/021-curate-application-shell.md`
- `test/parity/g15-021-application-shell-specimens.test.tsx`
- `docs/logs/2026-08/20260819-g15-021-application-shell-curation.md`
