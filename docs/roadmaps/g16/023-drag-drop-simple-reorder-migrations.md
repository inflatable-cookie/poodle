# g16.023 — Drag-And-Drop Simple Reorder Migrations

Status: planned — blocked on g16.022 and the public migration decision
Depends on: `022-drag-drop-web-custom-surface-substrate.md`,
`../../triage/20260828-221415-drag-drop-public-migration-boundary.md`
Governing refs: architecture 011, spec 069, and the Tabs and EditableList
component contracts

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

Before ready status, the operator must decide the exported `tabs-reorder.ts`
helpers under the named triage gate. Apply the approved clean migration; do not
retain aliases, dual controllers, or fallback behavior.

## Acceptance Criteria

- [ ] Svelte and React Tabs and EditableList consume the shared substrate.
- [ ] Mouse, touch-like pointer, keyboard, cancellation, source removal, target
      removal, and disabled inertia have mounted proof in both runtimes.
- [ ] Tabs callback order/result, EditableList editing controls, focus return,
      announcements, and specimens do not regress.
- [ ] Replaced local session state and approved obsolete exports are removed.
- [ ] No native implementation or parity-ledger cell changes.

## Writable Scope

- Tabs and EditableList Svelte/React implementations, focused tests, types, and
  curated specimens;
- the landed web drag substrate and focused tests only for proven reusable
  defects;
- `packages/core/src/tabs-reorder.ts` and root exports only according to the
  recorded operator decision;
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
