# g16.026 — Drag-And-Drop Cross-Window Bridge, Tabs, And DockRegion

Status: in progress — Paseo worker dispatched from pushed main; public migration,
paired bridge API, Tabs subject-family composition, and GPUI window ownership
fixed
Depends on: `025-drag-drop-rust-gpui-substrate.md`
Governing refs: architecture 011, spec 069, and the Tabs and DockRegion
contracts

## Goal

Implement the capability-based host bridge for same-application cross-window
transfer and migrate Tabs plus DockRegion without importing Longhorn, Tauri,
Electron, window topology, or durable layout authority into Poodle.

## Locked Public API

The operator chose split ownership on 2026-08-31. Do not replace it with one
controller-wide bridge.

- `CrossWindowDragReceipt` is exactly protocol version plus opaque token.
- `CrossWindowDragSourceBridge` is per source. It owns abortable `prepare`,
  transport `start`, one authoritative terminal subscription, and idempotent
  host `cancel`.
- `CrossWindowDragTargetBridge` is per document/native window. It owns live
  host projection, authoritative `commit`, and optional `pickTarget` when its
  capability says keyboard selection exists.
- `CrossWindowDragProjection` contains the host-local source id, label,
  semantic subject, operation, input kind, and at most one target id/position.
  None of those fields cross the wire beside the receipt.
- `createCrossWindowDataTransferAdapter()` is the bounded web codec. Its
  default MIME type is `application/x-poodle-cross-window-drag+json`; its body
  is exactly `{ protocolVersion, token }`; `accepts` is the dragover path and
  `read` is the drop path.
- `DragSourceRegistration.crossWindowSourceBridge` is optional. Svelte and
  React providers accept `crossWindowTargetBridge`.
- Tabs replaces `onDragPrepare`, `onDragStart`, and `onDragEnd` with optional
  `crossWindowSourceBridge`.
- Tabs also exposes optional `dragSubjectKind`. Its default stays scoped to the
  Tabs instance. An explicit kind lets an owning composite place the strip in a
  shared semantic family without taking over Tabs reorder. `TabItem.value` is
  the semantic subject id; renderer registration ids remain instance-scoped.
- DockRegion replaces `externalDragSource` / `externalDropTarget` with
  `crossWindowDragSource` / `crossWindowDropTarget`. Local panel moves retain
  `canAcceptPanel` / `onPanelDrop`; a host receipt commits through the target
  bridge and does not also call `onPanelDrop`.
- Same-document cross-region panel transfer uses one ambient
  `DragDropProvider`. A DockRegion joins that provider when present and creates
  a private controller otherwise. Two self-provided sibling regions keep local
  reorder but do not cross-drop; no document-global session or second MIME
  wire remains.
- Rust exports the same public type and trait names with idiomatic field names.
  Host-supplied completion callbacks may replace TypeScript promises; the
  lifecycle and result shapes may not differ.

The full field/method contract is spec 069's Cross-Window Host Bridge section.

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
- Tabs joins the nearest ambient drag provider and owns a private controller
  otherwise. Plain Tabs remain isolated by their instance-scoped default
  subject kind and registration ids. Its reorder targets reject a same-kind
  subject absent from their own item set, allowing an accepted ancestor
  composite target to win instead of swallowing a foreign drop.
- DockRegion passes `poodle.dock-panel` as the strip's subject kind. It encodes
  panel id, source edge, and required source zone into the Tabs-internal subject
  id and decodes every value/callback at the DockRegion boundary; that internal
  encoding must never leak through its public tab, close, reorder, or panel-drop
  results.
- DockRegion preserves within-region reorder, zones, collapse, tab callbacks,
  and current mounted evidence while replacing its global side channel.
- GPUI consumers own one `DragDropWindowHost` per window and wrap their root in
  `drag_drop_window_host(&host, || root)`. Existing
  `drag_drop_provider(&controller, || subtree)` registers with that current
  host. The window host, not a thread-global registry, owns missing-provider
  cancellation and native drag stop.

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
`PanelDragData.sourceZone` required, and remove the older-build fallback.

## Ordered Work

1. Add the paired TypeScript/Rust receipt, capability, transport, projection,
   source-bridge, target-bridge, commit, and target-event contracts. Give late,
   repeated, mismatched, declined, failed, and cancelled completions one
   deterministic kernel mapping.
2. Integrate the split bridge with the existing web and GPUI controllers.
   Preparation starts before activation, one receipt is bound to one kernel
   session, host projection drives the existing target registry, and commit
   revalidates the live target before calling host authority.
3. Add the bounded web DataTransfer codec and headless multi-context host
   simulator. The adapter carries only the normalized receipt; no host record,
   panel data, geometry, event, or mutable session enters the payload.
4. Migrate Svelte and React Tabs to the shared pointer/keyboard substrate plus
   `crossWindowSourceBridge` and `dragSubjectKind`. Join an ambient provider or
   self-provide, scope registration ids independently of semantic subject ids,
   and make foreign same-kind subjects fall through its reorder targets.
   Preserve reorder, focus, disabled, close, overflow, specimens, and keyboard
   behavior; carry the semantic prop through `TabsSpec`; then delete the old
   DOM-shaped reorder helpers and framework re-exports.
5. Migrate Svelte and React DockRegion. Keep static/flexible local reorder,
   zones, collapse, callbacks, and projection, but replace the external
   controller and `dockPanelDragSession` with the split bridge. Make
   `PanelDragData.sourceZone` required and delete the old fallback and public
   exports only after mounted replacement proof passes. Register local panel
   sources and targets with the nearest shared provider, falling back to one
   private controller only for the region's own local reorder.
6. Add renderer-neutral DockRegion registrations and the GPUI projection path
   without importing host geometry or authority. Keep Jetstream at compile-only
   renderer-neutral maintenance.
7. Add `DragDropWindowHost` / `drag_drop_window_host`, wire the preview,
   headless driver, adapter README, and GPUI developer guide, and prove
   provider unmount against two independent windows plus GPUI native drag
   state/preview cleanup.
8. Run the focused adversarial matrix, active-cohort boards, final headless QA,
   absence searches, and closeout surfaces. Do not move the parity ledger.

## Acceptance Criteria

- [ ] Paired TypeScript/Rust bridge contracts carry only opaque authority.
- [ ] The public names and split ownership match the locked API above and spec
      069 exactly; there is no unified or DockRegion-specific replacement
      controller.
- [ ] Cross-window source preparation is bound to one kernel session, starts
      before activation, and ignores late or repeated completions.
- [ ] Host target projection uses the existing registry and revalidates the
      exact live target before one authoritative commit.
- [ ] The bounded DataTransfer adapter writes and reads only protocol version
      plus opaque token and rejects malformed, oversized, future, or mismatched
      envelopes.
- [ ] Svelte and React Tabs preserve reorder results, keyboard behavior, focus,
      disabled inertia, and curated specimens on the shared substrate.
- [ ] Plain Tabs remain instance-isolated under a common provider, while an
      explicit `dragSubjectKind` composes with an ancestor target without
      leaking or colliding registration ids; shared Rust carries the same
      semantic input.
- [ ] A deterministic host simulator proves prepare, moving target geometry,
      stale lease, rejection, commit, cancel, window close, and late completion.
- [ ] Svelte, React, and GPUI DockRegion projections preserve component
      behavior and use the same lifecycle semantics.
- [ ] Two sibling web DockRegions under one provider cross-drop through the
      normal target path and call `onPanelDrop` once; without a common provider
      they retain same-region reorder but do not discover one another.
- [ ] Headless web multi-context and GPUI host-stub tests take no operator focus.
- [ ] Poodle imports no Longhorn/shell package and owns no window transaction.
- [ ] Existing DockRegion ledger claim remains honest; no unrelated row moves.
- [ ] Active-source search proves the old controller, session side channel,
      types, re-exports, and optional-source-zone fallback are absent.
- [ ] GPUI provider unmount closes an active session, drops that provider's
      registrations, and stops the native drag/preview. Carried from
      `g16.025` (see below).

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

The fixed seam is one public `DragDropWindowHost` per window plus
`drag_drop_window_host`. A window host inventories only the controllers
registered under its own root. Its end-of-frame element reaches `Window` and
`App`, cancels an absent provider, prunes its registrations, and calls
`stop_active_drag` before forgetting the controller. Do not revive
`LIVE_CONTROLLERS`, a thread-global sweep, or an `App`-only deferred cleanup.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Required proof |
| --- | --- | --- |
| One receipt belongs to one kernel session | preparation A resolves after A was superseded by B | A is cancelled once, cannot arm/start B, and B alone reaches `dragging` |
| Native end is not host commit | `dragend` reports `move` before the host refuses a stale lease | the session ends rejected from the host result; no committed callback or mutation is inferred |
| Drop revalidates live authority | host projects an accepted target, then removes/disables it before drop | commit is never called, target posture clears/rejects, terminal runs once |
| Wire is opaque and bounded | valid MIME with extra panel data, bad version/token, or receipt different from live projection | decode/match rejects before eligibility or commit; exact valid envelope passes |
| Projection follows current host geometry | host moves the target without local pointer input | the one projected target/position changes and stale geometry cannot commit |
| Window loss is terminal | source or target window closes while preparing, armed, dragging, or dropping | host/session cancellation and cleanup run once; late completion is inert |
| Keyboard is the same transaction | picker returns a target that becomes stale before commit | normal revalidation rejects; no arrow-key window simulation or second callback path appears |
| Touch claims are capability-bound | host cannot observe touch outside the source window | internal touch remains true; cross-window touch advertises false and never starts |
| Local reorder stays independent | ordinary Tabs has no source bridge or the host declines | pointer and Alt+Arrow reorder still use the shared local lifecycle with no native payload |
| Tabs composition does not capture a foreign subject | two Tabs instances share one provider and subject kind; the dragged subject id belongs only to A, while B sits inside an accepting composite target | B's reorder targets reject it during eligibility, the composite target wins, and no B reorder callback fires; ordinary Tabs with no explicit kind remain mutually ineligible |
| DockRegion has no hidden local bus | two sibling regions mount first under one provider, then without one | the shared-provider pair cross-drops once; the self-provided pair keeps local reorder and exposes no cross-region target |
| Two GPUI windows are isolated | A renders while B has an active controller and provider | A's frame cannot cancel, prune, or stop B |
| Provider unmount stops the real GPUI drag | B's provider disappears mid-drag | B reaches semantic idle, registrations and preview vanish, and GPUI reports no active native drag |
| Clean migration is complete | active-source search over old names and optional `sourceZone` | zero definitions, imports, exports, aliases, wrappers, global session uses, or fallback reads |

## Writable Scope

- focused core/headless bridge types and host simulators;
- bounded native DataTransfer opaque-token adapter;
- Tabs web implementations, contracts, tests, specimens, and deletion of the
  obsolete Tabs reorder module and exports after replacement proof;
- DockRegion web, render, GPUI, types, contracts, tests, and specimens;
- deletion of the old DockRegion external-drag module, controller, session side
  channel, types, and framework/root exports under the approved clean migration;
- focused headless multi-context/native host-stub evidence;
- GPUI window-host wiring, the adapter README, and GPUI developer guide;
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
- The split bridge cannot be paired without a second semantic session or
  controller.
- GPUI provider cleanup needs a thread-global registry, fork, OS input backend,
  or a boundary the public root cannot wire.
- Proof needs sibling-repository mutation or focus-taking automation.

## Continuation

After merge, promote `g16.027` for inbound files and native file drag-out.
