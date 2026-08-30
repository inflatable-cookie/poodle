# g16.022 — Drag-And-Drop Web Custom-Surface Substrate

Status: complete — PR #101
Date: 2026-08-30
PR: https://github.com/inflatable-cookie/poodle/pull/101
Card: `docs/roadmaps/g16/022-drag-drop-web-custom-surface-substrate.md`
Handoff: `docs/handoffs/20260830-153354-g16-022-drag-drop-web-substrate.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/architecture/006-headless-core-and-machine-model.md`
Branch: `t3code/review-drag-drop-web-substrate`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-fd96cda0`

## Outcome

The g16.021 semantic kernel is now usable from arbitrary same-document web
surfaces. One provider-scoped DOM controller runs Pointer Events (mouse, pen,
touch) and a keyboard sensor, caches geometry, executes kernel effects, and
projects an immutable snapshot. Svelte and React bind that controller
idiomatically. Headless Chromium and WebKit prove capture, geometry
invalidation, touch/scroll arbitration, preview cleanup, and focus return.

No component, contract, token, native runtime, or ledger row changed. The
parity evidence ledger stays at 52 mounted / 122 missing.

## Landed public API

`@inflatable-cookie/poodle-core` — `packages/core/src/dom/drag-drop-controller.ts`:

- `createDragDropController`
- `DragDropController.connect` / `registerSource` / `registerTarget` /
  `getSnapshot` / `subscribe` / `invalidateLayout` / `cancel` / `destroy`
- `DragSourceRegistration`, `DropTargetRegistration`, `DragSourceHandle`,
  `DropTargetHandle`
- `DragActivationConstraints`, `DragPositionResolverInput`,
  `DragDropCommitResult`, `DragDropSnapshot`, `DragPreviewSnapshot`,
  `DragAnnouncementEvent`, `DragDropCapabilities`

Svelte (`packages/svelte/components/src/drag-drop.ts`, exported from the
package root without joining the 175-component roster):

- `DragDropProvider`
- `useDragDrop` → snapshot store, `cancel`, `dragSource` / `dropTarget` actions

React (`packages/react/components/src/drag-drop.tsx`):

- `DragDropProvider`
- `useDragDrop` / `useDragSource` / `useDropTarget`
- `getSourceProps` / `getTargetProps` compose consumer refs and handlers

There is no module singleton. Duplicate live ids fail. Unregister and destroy
are idempotent. An injected controller is disconnected, not destroyed, when its
provider unmounts. The React names are classified in
`test/package-install/roster.ts` as non-component root exports so they do not
join the frozen 175-component denominator.

## Behaviour that is now true

- Pointer Events are the internal transport. HTML Drag and Drop is prevented
  on registered sources and is not session authority.
- Mouse and pen activate by distance (default 4px). Touch activates after a
  hold while movement stays inside tolerance; movement outside tolerance before
  the hold leaves scrolling untouched and never enters the kernel.
- Pointer capture and `touch-action: none` apply only after activation.
- Keyboard Space/Enter pickup, arrows/Home/End intent, Enter/Space drop, and
  Escape cancel use the same kernel as pointer drags. Focus returns to the
  surviving source.
- Geometry is measured and cached; `invalidateLayout`, scroll, and resize
  remeasure. There is no per-target timer.
- `onDrop` may be sync or a promise. Late results naming a cancelled session
  are inert. A target disabled or unregistered while dropping rejects
  `target-unavailable`. Cleanup of listeners, overlay, attributes, timers, and
  capture runs once.
- Pre-activation hold/distance candidates die on disconnect, cancel,
  unregister, disable, visibility loss, and Escape. They never enter the
  kernel.
- Pointer capture works on any `Element` with `setPointerCapture` (HTML and
  SVG). After activation, non-passive `touchmove` `preventDefault` keeps a
  scroller from cancelling the gesture.
- Authored `tabindex` / `aria-label` / `aria-description` / `draggable` /
  `user-select` restore exactly. Post-destroy handles are inert.
- React ownership is frozen at first render. Owned controllers destroy on a
  microtask so StrictMode remount can reconnect. Injected controllers never
  destroy. Source/target registration follows the host node.

## Evidence

- Framework-free: `test/headless-dom/drag-drop-controller.test.ts` (25).
- Svelte mounted fixture: `packages/svelte/components/test/DragDropProvider.test.ts`
  plus `DragDropCustomSurface.svelte` (7).
- React mounted fixture: `packages/react/components/test/DragDropProvider.test.tsx`
  plus `DragDropCustomSurface.tsx` (11), including StrictMode, injected
  ownership, and host replacement.
- Chromium and WebKit: `effigy test:drag-drop-browser` /
  `test/drag-drop/probe.ts` — real mouse input and `hasPointerCapture`,
  outside-source routing, both sides of touch hold vs scroll, automatic
  scroll/resize invalidation, keyboard focus return.

## Review repair

PR #101 review (comment on `d3d91f179`) blocked on eight findings. All eight
are closed in this follow-up: candidate lifetime, release hit-testing, async
drop disable/unregister, live-region notify, React StrictMode/ownership/host
node, exact restore, SVG capture, and real browser evidence.

Repair validation: focused controller/Svelte/React tests, Chromium/WebKit
probe, `effigy ci:web`, `effigy docs:check`, `effigy check:parity-evidence-ledger`,
`effigy qa`, `git diff --check origin/main...HEAD`. Ledger still 175 rows.

## Non-claims

Auto-scroll, cross-window transport, DataTransfer, inbound files, drag-out,
Rust/Node/GPUI, and migration of Tabs, EditableList, Tree, BlockEditor,
OrderBy, ModelCatalogueEditor, or DockRegion remain later cards. Old Tabs and
DockRegion helpers are still public.

## Continuation

`g16.023` is the next drag card after merge: the first simple Poodle component
migrations, with the approved clean public break.
