# g16.023 — Drag-And-Drop Simple Reorder Migrations

Status: blocked — Tabs host-bridge sequencing needs an operator decision
Depends on: `022-drag-drop-web-custom-surface-substrate.md`
Governing refs: architecture 011, spec 069, the resolved
`../../triage/20260828-221415-drag-drop-public-migration-boundary.md`, and the
Tabs and EditableList component contracts

## Planning Blocker

Tabs' public DOM drag callbacks are still consumed by DockRegion's external
drag bridge. Removing HTML drag authority while preserving those callbacks and
leaving DockRegion untouched is contradictory. Resolve
`../../triage/20260830-180816-tabs-drag-host-bridge-sequencing.md` before
dispatch.

## Goal

Make Tabs and EditableList the first real Svelte/React consumers of the web
substrate. Preserve their component-owned callback payloads, keyboard behavior,
focus, editing, disabled state, and curated specimens while deleting their
replaced component-local drag lifecycle.

## Required Migration

- Tabs uses one substrate source/target registration per reorderable item and
  continues to report the complete resulting order through its existing
  semantic transition.
- EditableList uses the same simple-reorder path without allowing a drag sensor
  to steal row editing, buttons, selection, or ordinary touch scrolling.
- Pointer, touch, and keyboard routes commit through one component result path.
- Disabled/non-reorderable controls are not advertised as draggable.
- Source and target state comes from the provider; no parallel local session or
  HTML `DataTransfer` side channel remains.

The public migration is locked: delete `ReorderState`, `createReorderState`,
`handleDragStart`, `handleDragOver`, and `handleDrop`, the DOM-shaped
`tabs-reorder.ts` module, its root exports, and both framework re-export files
after the mounted substrate replacement passes. Retain `applyReorder` only as
the existing pure semantic helper exported from `tabs.ts`; it is not an alias
for the removed module. Do not retain dual controllers or fallback behavior.

## Acceptance Criteria

- [ ] Svelte and React Tabs and EditableList consume the shared substrate.
- [ ] Mouse, touch-like pointer, keyboard, cancellation, source removal, target
      removal, and disabled inertia have mounted proof in both runtimes.
- [ ] Tabs callback order/result, EditableList editing controls, focus return,
      announcements, and specimens do not regress.
- [ ] Replaced local session state and approved obsolete exports are removed.
- [ ] Active-source search proves the removed helper names and component-local
      re-export modules are gone while the semantic `applyReorder` export and
      callback result remain.
- [ ] No native implementation or parity-ledger cell changes.

## Writable Scope

- Tabs and EditableList Svelte/React implementations, focused tests, types, and
  curated specimens;
- the landed web drag substrate and focused tests only for proven reusable
  defects;
- deletion of `packages/core/src/tabs-reorder.ts`, its obsolete root exports,
  and the Svelte/React component-local re-export files under the approved clean
  migration;
- the Tabs/EditableList contracts only to document the substrate without
  changing public component semantics;
- this card, the migration triage note, one execution log, g16 closeout, and
  `PAPERCUTS.md`.

Do not edit Tree, DockRegion, other drag components, Rust/GPUI, cross-window or
file adapters, package versions, releases, workflows, or siblings.

## Validation

Run focused paired component tests, web custom-surface preservation tests,
headless Chromium/WebKit drag probes, relevant contract/callback/capability
drift checks, `effigy ci:web`, `effigy docs:check`, unchanged ledger checks, one
final headless `effigy qa`, and `git diff --check origin/main...HEAD`.

## Stop Conditions

- A public component callback or interaction contract must change.
- Tabs and EditableList cannot share the simple-reorder substrate without a
  component-specific event choreography branch.
- Correct behavior requires a compatibility shim or continued HTML drag source
  of truth.
- Work expands into nested placement, auto-scroll, GPUI, DockRegion, files,
  release, or sibling repositories.

## Continuation

After merge, promote `g16.024` for Tree's nested intent and auto-scroll proof.
