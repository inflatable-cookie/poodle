# g16.022 — Drag-And-Drop Web Custom-Surface Substrate

Status: ready
Opened: 2026-08-28
Depends on: `021-drag-drop-semantic-kernel.md`
Governing refs: `../../architecture/011-drag-and-drop-substrate.md`,
`../../specs/069-dependable-drag-and-drop-substrate.md`,
`../../contracts/001-working-rules.md`

## Goal

Expose the landed semantic kernel to consumer-built Svelte and React surfaces.
Implement the same-document web runtime: source/target registration, mouse,
pen, touch, and keyboard sensors, cached geometry, preview overlay, live-region
announcements, focus return, and exactly-once teardown.

Do not migrate an existing Poodle component in this card. The proof is a small
custom consumer fixture in each web runtime so the substrate is genuinely
public rather than component-private.

## Locked Public Surface

Spec 069 fixes the public surface after review of the landed g16.021 types. The
worker implements, documents, and exports:

- core `createDragDropController` and the exact controller, registration,
  handle, snapshot, activation, announcement, preview, capability, and commit
  result types named in spec 069;
- Svelte `DragDropProvider` / `useDragDrop`, with `dragSource` and `dropTarget`
  actions;
- React `DragDropProvider` / `useDragDrop` / `useDragSource` /
  `useDropTarget`, with stable prop getters; and
- one provider-owned overlay and polite live region, optional custom preview,
  default label-based announcements, and `describeAnnouncement` override.

Svelte and React need equivalent semantics, not artificially identical syntax.
No DOM node, framework event, or global singleton belongs in core semantic
state.

Registration identity is strict: duplicate live ids fail, handles update in
place, unregister is idempotent, and two providers in one document stay
independent. An injected controller is disconnected but not destroyed by its
provider.

## Required Behavior

- Pointer Events drive mouse, pen, and touch through one sensor.
- Touch scroll wins until the configured distance/hold boundary activates.
- Pointer capture begins only after activation and survives movement outside
  the source.
- Keyboard pickup, intent movement, drop, and Escape use the same kernel.
- Geometry is measured/cached by the adapter and invalidated on scroll, resize,
  registration change, and unmount; no per-target timer exists.
- Preview, accepted/rejected target hooks, announcements, and focus return are
  projections of semantic state.
- Lost capture, pointer cancel, visibility loss, source/target unmount, provider
  unmount, and repeated cleanup leave no listener, overlay, attribute, timer,
  or active session behind.

## Acceptance Criteria

- [ ] Public Svelte and React custom-surface APIs are documented and exported.
- [ ] Mounted fixtures prove pointer, touch-like pointer, keyboard, rejection,
      cancellation, unmount, and two independent provider scopes.
- [ ] Chromium and WebKit headless probes prove capture, geometry invalidation,
      touch/scroll arbitration, preview cleanup, and focus restoration.
- [ ] The substrate uses g16.021 transitions rather than duplicating session
      state in framework adapters.
- [ ] Examples remain human-facing; exhaustive sensor cases live in tests or a
      dedicated conformance tab.
- [ ] No existing component or ledger row changes.

## Execution Plan

- [ ] **Batch 1 — DOM controller.** Implement the controller, registrations,
      pointer/touch/keyboard sensors, cached geometry, effect runner, snapshots,
      exact cleanup, and focused framework-free tests over the g16.021 kernel.
- [ ] **Batch 2 — Svelte adapter.** Add provider/context, source and target
      actions, overlay/live region, exports, and a mounted custom-surface
      fixture with two independent scopes.
- [ ] **Batch 3 — React adapter.** Add the equivalent provider/hooks/prop
      getters, composed refs/handlers, exports, and the same mounted fixture
      behavior without forcing Svelte syntax onto React.
- [ ] **Batch 4 — browser evidence and closeout.** Prove Chromium/WebKit
      capture, touch arbitration, geometry invalidation, rejection, focus
      return, unmount cleanup, and preview cleanup; update docs/logs without
      moving a component ledger cell.

## Writable Scope

- focused web runtime/controller modules under `packages/core/src/`;
- new Svelte and React provider/action/hook modules, exports, tests, and curated
  custom-surface specimens;
- focused headless Chromium/WebKit test fixtures and Effigy selectors only when
  existing selectors cannot express the proof;
- this card, one execution log, g16/front-door closeout, and `PAPERCUTS.md`.

Do not edit existing component implementations, native Node/render/GPUI code,
DataTransfer/cross-window/file adapters, old Tabs/DockRegion helpers, tokens,
package versions, workflows, releases, or sibling repositories.

## Validation

Run focused core/Svelte/React tests, the headless Chromium and WebKit probes,
`effigy ci:web`, `effigy docs:check`, the unchanged parity-ledger check, one
final headless `effigy qa`, and `git diff --check origin/main...HEAD`.
Never run a windowed/native visual or Jetstream selector.

## Stop Conditions

- The landed semantic kernel must change materially rather than receive a thin
  runtime adapter.
- Public API names/ownership remain ambiguous after g16.021 review.
- Correct touch behavior requires global `touch-action:none`, HTML Drag and
  Drop as session authority, or framework-global mutable state.
- The card expands into auto-scroll, component migration, cross-window/file
  transport, GPUI, release, or sibling-repository work.

## Continuation

After merge, promote `g16.023` for the first simple Poodle component
migrations. Its public-export decision is approved in
`../../triage/20260828-221415-drag-drop-public-migration-boundary.md`; apply the
clean removal only after the mounted substrate replacement passes.
