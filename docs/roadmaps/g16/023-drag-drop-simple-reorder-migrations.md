# g16.023 — EditableList Simple Reorder Migration

Status: complete — PR #104 revision awaiting re-review
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
- Register the complete item order as logical keyboard targets. Visible rows
  remain DOM pointer/touch targets; hidden window pages do not need fake DOM.
- Give keyboard sources their global order. Previous resolves to `before`, next
  to `after`; first/last are explicit. Do not derive both directions from the
  mounted target centre point.
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

- [x] Svelte and React EditableList consume the shared substrate.
- [x] Mouse, pen, real touch, keyboard, cancellation, source removal, target
      removal, and disabled inertia have mounted proof in both runtimes.
- [x] Keyboard reorder moves both directions in an ordinary list and across
      both `windowSize` boundaries without paging or unmounting the source
      before commit.
- [x] Resulting-order callbacks, editing controls, selection, focus return,
      announcements, and curated specimens do not regress.
- [x] Active-source search proves EditableList's local `dragstart`, `dragover`,
      `drop`, `dragend`, `DataTransfer`, and parallel drag state are absent.
- [x] Existing custom-surface controller/provider evidence remains green.
- [x] Tabs, DockRegion, native implementations, and parity-ledger cells do not
      change.

## Writable Scope

- EditableList Svelte/React implementations, focused tests, types, contracts,
  and curated specimens;
- the landed web drag substrate and focused tests only for a proven reusable
  defect exposed by the migration, including the approved logical keyboard
  target registration, paired bindings, and focused lifecycle tests;
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
- The logical-target extension cannot preserve one target eligibility/commit
  path without fake DOM, paging during the session, or an EditableList-specific
  keyboard lifecycle.
- Correct behavior requires a compatibility shim or continued HTML drag source
  of truth.
- Work expands into Tabs, nested placement, auto-scroll, GPUI, DockRegion,
  files, release, or sibling repositories.

## Review Oracle

- **Invariant:** pointer/touch use mounted DOM targets; keyboard may use the
  complete ordered logical target set; every sensor reaches the same live
  `canDrop` / `onDrop` result and the source stays mounted until terminal.
- **Smallest adversarial counterexample:** items A/B/C with `windowSize=2`.
  Pick up B, ArrowDown, drop: C is addressable while unmounted and the result is
  A/C/B. From the second page, pick up C, ArrowUp, drop: B is addressed as
  `before` and the result is A/C/B. Repeat ordinary-list ArrowUp so direction
  cannot collapse to the old centre-point `after` intent.
- **Expected failure/stop:** paging cancels the source, the boundary arrow needs
  a second component lifecycle, hidden rows become synthetic DOM, previous and
  next resolve to the same position, or a removed/disabled logical target can
  still commit.
- **Required proof:** core logical-target order, direction, removal, disabled,
  and revalidation tests; paired Svelte/React ordinary and both-boundary mounted
  tests; genuine touch hold-versus-scroll tests; preserved custom-surface and
  Chromium/WebKit evidence.

## Continuation

After merge, promote `g16.024` for Tree's nested intent and auto-scroll proof.
