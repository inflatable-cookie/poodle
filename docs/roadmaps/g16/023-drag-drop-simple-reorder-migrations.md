# g16.023 — EditableList Simple Reorder Migration

Status: ready — Tabs sequencing resolved; g16.022 merged in PR #101
Depends on: `022-drag-drop-web-custom-surface-substrate.md`
Governing refs: architecture 011, spec 069, the resolved
`../../triage/20260830-180816-tabs-drag-host-bridge-sequencing.md`, and the
EditableList component contract

## Goal

Make EditableList the first real Svelte/React component consumer of the web
drag substrate. Preserve its complete resulting-order callback, editing,
buttons, selection, focus, disabled state, touch scrolling, and curated
specimens while deleting its component-local HTML drag lifecycle.

Tabs is deliberately deferred to `g16.026`. Its public DOM drag callbacks are
part of DockRegion's external-drag bridge and must migrate with that consumer.

## Required Migration

- Register one substrate source/target pair per enabled reorderable row.
- Pointer, touch, and keyboard routes commit through one component result path
  and continue to report the complete resulting item order.
- A drag sensor cannot steal inline editing, row buttons, selection, ordinary
  taps, or pre-activation touch scrolling.
- Disabled or non-reorderable rows are not advertised as draggable and remain
  inert across every sensor.
- Source and target posture comes from the provider. Remove EditableList's
  parallel local drag session and HTML `DataTransfer` path.
- Preserve the current public component API. Do not add a compatibility path,
  a second controller, or a component-specific lifecycle fork in the shared
  substrate.

## Acceptance Criteria

- [ ] Svelte and React EditableList consume the shared substrate.
- [ ] Mouse, touch-like pointer, keyboard, cancellation, source removal, target
      removal, and disabled inertia have mounted proof in both runtimes.
- [ ] Resulting-order callbacks, editing controls, selection, focus return,
      announcements, and curated specimens do not regress.
- [ ] Active-source search proves EditableList's local `dragstart`, `dragover`,
      `drop`, `dragend`, `DataTransfer`, and parallel drag state are absent.
- [ ] Existing custom-surface controller/provider evidence remains green.
- [ ] Tabs, DockRegion, native implementations, and parity-ledger cells do not
      change.

## Writable Scope

- EditableList Svelte/React implementations, focused tests, types, contracts,
  and curated specimens;
- the landed web drag substrate and focused tests only for a proven reusable
  defect exposed by the migration;
- this card, the resolved sequencing triage note, one execution log, g16
  closeout, and `PAPERCUTS.md`.

Do not edit Tabs, `tabs-reorder.ts`, DockRegion, Tree, other drag components,
Rust/GPUI, cross-window or file adapters, package versions, releases,
workflows, or siblings.

## Validation

Run focused paired EditableList tests, web custom-surface preservation tests,
headless Chromium/WebKit drag probes, relevant contract/callback/capability
drift checks, `effigy ci:web`, `effigy docs:check`, unchanged ledger checks,
one final headless `effigy qa`, and `git diff --check origin/main...HEAD`.

## Stop Conditions

- EditableList's public callback or interaction contract must change.
- Pointer, touch, and keyboard cannot share one component result path without a
  component-specific choreography branch in the shared substrate.
- Correct behavior requires a compatibility shim or continued HTML drag source
  of truth.
- Work expands into Tabs, nested placement, auto-scroll, GPUI, DockRegion,
  files, release, or sibling repositories.

## Continuation

After merge, promote `g16.024` for Tree's nested intent and auto-scroll proof.
