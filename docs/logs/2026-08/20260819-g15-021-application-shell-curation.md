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
HistoryCenter fixture IDs become six plain-language questions. No component,
contract, or public API changed.

## Change class

- **Change class:** none (documentation surfaces only)
- **Packages touched:** `poodle-svelte` preview, `poodle-react` preview,
  `poodle-gpui-preview` — specimen files only; `mod.rs` for DetailShell
  `render(state, cx)` signature
- **Public entry points:** unchanged
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
| single-fork-open | Rename and manage a continuation (disclose + delete on web; rename sibling) |
| rename | Rename and manage a continuation |
| rejection | Failure and incomplete metadata |
| no-timestamp | Failure and incomplete metadata |

Mount-time auto-open of single-fork-open removed; examples stay interactable.
Host feeds remain live. Undo and navigate write a page-level Last host
command readout. Rename and web delete write a section Last command readout.
GPUI teaches disclose + rename only: `HistoryCenterHandlers` still has no
delete command surface. Restoring web delete without a GPUI delete handler is
an intentional three-runtime gap that needs a planning amendment / focused
parity slice — not a catalogue reduction on Svelte/React.

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
- `git diff --check origin/main...HEAD`: pass

Headless only. No windowed, native-visual, conformance, Jetstream, or release
selectors.

## Open PR checkpoint

**Live paired-preview operator review** of every changed web page in Svelte and
React previews is required before merge. This worker does not claim that review
passed.

## Unresolved / known gaps

- GPUI DockRegion tab pass-throughs and live cross-region panel transfer remain
  structural / affordance-only until native gesture vocabulary expands; expanded
  dock copy no longer claims closable/reorderable (web wires those handlers)
- `g15.026` still owns the native page probe
- **GPUI HistoryCenter delete** — web manage story restores `onDeleteContinuation`;
  GPUI has no matching handler field. Needs orchestrator planning amendment /
  focused prerequisite parity slice before three-runtime delete parity. Do
  not hide this by removing the web delete story.
- Size/density axes for HistoryCenter and DockRegion are presentation-only (no
  enabled undo/redo/close/reorder without local handlers)
- Live paired-preview operator review remains open

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
4. LOW — batch log recount + honest unresolved GPUI delete gap

## Changed files

- `packages/svelte/preview/src/specimens/{ActionDiscoveryPanel,DetailSection,DetailShell,DockRegion,HistoryCenter,PageHeader}Specimen.svelte`
- `packages/react/preview/src/gallery/specimens/{ActionDiscoveryPanel,DetailSection,DetailShell,DockRegion,HistoryCenter,PageHeader}Specimen.tsx`
- `packages/gpui/preview/src/specimens/{action_discovery_panel,detail_section_specimen,detail_section_group_specimen,detail_shell,dock_region,history_center_specimen,page_header_specimen}.rs`
- `packages/gpui/preview/src/specimens/mod.rs`
- `test/parity/g15-021-application-shell-specimens.test.tsx`
- `docs/logs/2026-08/20260819-g15-021-application-shell-curation.md`
