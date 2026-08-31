# g16.026 — Drag-And-Drop Cross-Window Bridge, Tabs, And DockRegion

Status: planned — dependency merged and public migration approved; exact paired bridge API still needs orchestrator promotion before dispatch
Depends on: `025-drag-drop-rust-gpui-substrate.md`
Governing refs: architecture 011, spec 069, the resolved
`../../triage/20260828-221415-drag-drop-public-migration-boundary.md`,
`../../triage/20260830-180816-tabs-drag-host-bridge-sequencing.md`, and the Tabs
and DockRegion contracts

## Goal

Implement the capability-based host bridge for same-application cross-window
transfer and migrate Tabs plus DockRegion without importing Longhorn, Tauri,
Electron, window topology, or durable layout authority into Poodle.

## Required Boundary

- Host preparation completes before native drag activation and yields only an
  opaque armed receipt/session id.
- Poodle advertises protocol version plus opaque token across the bounded
  DataTransfer adapter; it never stores the authoritative transaction there.
- Targets project host-supplied eligibility, revalidate before commit, and
  report one terminal success/refusal/cancel result.
- The host owns leases, window geometry, target resolution, authorization,
  mutation, rollback, expiry, and recovery.
- Cross-window keyboard movement uses a host target picker. Touch capability is
  advertised only when the host can observe it outside the source window.
- Tabs moves its internal reorder path to the landed substrate and replaces its
  DOM-event host callbacks with the new semantic preparation/terminal bridge.
- DockRegion preserves within-region reorder, zones, collapse, tab callbacks,
  and current mounted evidence while replacing its global side channel.

The public migration is locked. After Tabs and the new opaque bridge pass,
delete `ReorderState`, `createReorderState`, `handleDragStart`,
`handleDragOver`, `handleDrop`, the DOM-shaped `tabs-reorder.ts` module, its
root exports, and both framework re-export files. Retain only the existing pure
`applyReorder` helper from `tabs.ts`. Also delete every old `DockExternalDrag*`
and `DockExternalDrop*` export,
`createDockExternalDragController`, `dockPanelDragSession`, their framework
re-exports, and the DOM-shaped controller module. Preserve asynchronous
prepare/cancel/revalidation as new paired host-bridge semantics, not as aliases
or wrappers. Preserve `onPanelDrop`'s semantic purpose, make
`PanelDragData.sourceZone` required, and remove the older-build fallback. Fix
the exact new bridge names from the landed kernel/GPUI substrate before this
card becomes ready.

## Acceptance Criteria

- [ ] Paired TypeScript/Rust bridge contracts carry only opaque authority.
- [ ] Svelte and React Tabs preserve reorder results, keyboard behavior, focus,
      disabled inertia, and curated specimens on the shared substrate.
- [ ] A deterministic host simulator proves prepare, moving target geometry,
      stale lease, rejection, commit, cancel, window close, and late completion.
- [ ] Svelte, React, and GPUI DockRegion projections preserve component
      behavior and use the same lifecycle semantics.
- [ ] Headless web multi-context and GPUI host-stub tests take no operator focus.
- [ ] Poodle imports no Longhorn/shell package and owns no window transaction.
- [ ] Existing DockRegion ledger claim remains honest; no unrelated row moves.
- [ ] Active-source search proves the old controller, session side channel,
      types, re-exports, and optional-source-zone fallback are absent.
- [ ] GPUI provider unmount closes an active session and drops that provider's
      registrations. Carried from `g16.025` (see below).

## Carried From g16.025 — Provider Unmount

A GPUI `DragDropController`'s own per-frame sweep is the only thing that can
close a session it holds, and an unmounted provider never sweeps again. A host
that removes a provider mid-drag therefore keeps a `Dragging` session with live
registrations and no terminal callback, so the consumer's own drag state
latches with nothing left to clear it. Spec 069 makes provider unmount a
cancellation.

`g16.025` proved the gap and then reverted its fix: closing it needs a host
frame boundary that knows which window owns which controller, and window
ownership is this card's subject. Two proofs are required, and the first is the
one that sank the earlier attempt:

- **Two windows, no false cancel.** A thread-global "did this controller sweep
  this frame" mark is wrong: rendering window A resets and sweeps controllers
  owned by window B, so an active drag in B is cancelled merely because B did
  not render during A's frame. Prove two windows with independent live sessions
  survive each other's frames.
- **Native drag actually stops.** A terminal reached with no provider left has
  no controller host to drain `pending_stop_active_drag`, so semantic idle and
  empty registries are not enough — prove GPUI's own active drag and preview
  are cleared too.

Whatever integration lands must reach every consumer, not only the in-repo
preview: `docs/guides/gpui-developer-guide.md` and
`packages/gpui/adapter/README.md` document the root wiring, and a boundary that
only the preview calls is a claim the guides quietly miss.

## Writable Scope

- focused core/headless bridge types and host simulators;
- bounded native DataTransfer opaque-token adapter;
- Tabs web implementations, contracts, tests, specimens, and deletion of the
  obsolete Tabs reorder module and exports after replacement proof;
- DockRegion web, render, GPUI, types, contracts, tests, and specimens;
- deletion of the old DockRegion external-drag module, controller, session side
  channel, types, and framework/root exports under the approved clean migration;
- focused headless multi-context/native host-stub evidence;
- this card, migration triage, one log, g16 closeout, and `PAPERCUTS.md`.

Do not edit Longhorn, Loophole, Tauri/Electron packages, application window
policy, file drag-out, package versions, releases, workflows, or siblings.

## Validation

Run bridge/kernel/DockRegion tests, headless Chromium/WebKit multi-context
proof, mounted GPUI regressions, active-cohort drift and ledger checks, web/Rust/
native/docs boards, one final headless `effigy qa`, and diff check. Never run a
windowed/native visual or Jetstream selector.

## Stop Conditions

- Poodle must know window topology, filesystem paths, credentials, or durable
  mutation to complete the bridge.
- Longhorn-owned semantics need to be copied rather than represented by opaque
  host capabilities.
- DockRegion cannot migrate without a compatibility path or a public behavior
  break beyond the approved clean replacement.
- Proof needs sibling-repository mutation or focus-taking automation.

## Continuation

After merge, promote `g16.027` for inbound files and native file drag-out.
