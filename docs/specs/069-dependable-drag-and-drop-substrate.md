# 069 Dependable Drag And Drop Substrate

Status: active — compiled as g16.021–g16.028; g16.021 merged, g16.022 landed
Updated: 2026-08-30
Depends on: `../architecture/011-drag-and-drop-substrate.md`,
`../contracts/001-working-rules.md`,
`../contracts/components/tabs.md`,
`../contracts/components/tree.md`,
`../contracts/components/editable-list.md`,
`../contracts/components/dock-region.md`

## Purpose

Define a predictable drag-and-drop framework for Poodle components and
consumer-built surfaces. It must support mouse, pen, touch, keyboard,
same-document movement, nested targets, auto-scroll, same-application
cross-window transfer, inbound files, and native file drag-out in browser,
Electron, Tauri, and GPUI contexts.

The framework removes repeated event choreography. It does not centralize
application mutation or window authority.

## Fixed Scope

- One semantic session vocabulary implemented once per language pair:
  TypeScript for Svelte/React and Rust for shared native composition.
- One public custom-surface substrate per active runtime, not only internal
  helpers for Poodle components.
- Internal pointer and keyboard sensors, including touch from the first
  delivery.
- Deterministic nested target selection, edge intent, drag preview, focus,
  announcements, and auto-scroll.
- Explicit adapters for host-owned cross-window transfer, native inbound file
  drop, and native file drag-out.
- Representative migrations before a broad component sweep.
- Jetstream keeps renderer-neutral vocabulary and shared Rust construction but
  remains deferred under the working rules.

## Semantic Types

Names may become idiomatic in each language, but these fields and distinctions
are normative.

```ts
type DragOperation = "move" | "copy" | "link";
type DropPosition = "before" | "inside" | "after" | string;

type DragSubject = {
  kind: string;
  id: string;
};

type DropIntent = {
  targetId: string;
  position: DropPosition;
  operation: DragOperation;
};

type DropEligibility =
  | { accepted: true; intent: DropIntent }
  | { accepted: false; reason?: string };
```

`kind` selects a consumer-defined subject family. `id` resolves the live
subject through consumer state. Neither field is display text or authority.

Implementations may attach session-local presentation metadata behind a
registration. That metadata is not serialized, compared for identity, or sent
through a host boundary.

## Session Lifecycle

The semantic states are:

```text
idle
  -> preparing
  -> armed
  -> dragging
  -> dropping
  -> ended
  -> idle

preparing | armed | dragging | dropping
  -> cancelled
  -> idle
```

- `preparing` may allocate a host session or materialize an export.
- `armed` means required preparation is complete but activation has not begun.
- `dragging` owns one source, current operation, pointer/keyboard sensor, and
  at most one resolved target intent.
- `dropping` revalidates eligibility and waits for the authoritative commit
  result when the transport is asynchronous.
- `ended` and `cancelled` are terminal observations. Cleanup runs exactly once
  before returning to `idle`.

Required events include:

- prepare requested, prepared, declined, failed, or superseded;
- activation threshold reached;
- pointer/keyboard movement;
- target entered, intent changed, target left;
- operation changed;
- drop requested, accepted, rejected, committed, or failed;
- Escape, pointer cancel, release without activation, source unmount, target
  unmount, transport loss, window loss, or host cancellation; and
- native drag ended with the result quality the adapter can honestly report.

Late asynchronous preparation cannot resurrect a cancelled or superseded
session. Every asynchronous completion carries the session identity it was
created for.

## Source Registration

A source registration supplies:

- stable source id;
- subject kind and id;
- allowed operations;
- disabled state;
- optional handle and activation constraints;
- optional preview description or builder;
- optional cross-window preparation bridge;
- optional external export preparation bridge; and
- lifecycle callbacks for start, terminal result, and cancellation.

Source registration does not make an element draggable when its required
transport is unavailable. Capability is resolved before visual affordance and
accessible instructions claim support.

The registration may be used by a component wrapper, a Svelte action, a React
hook/prop getter, or a native node builder. Those framework surfaces translate
to the same semantics.

## Target Registration

A target registration supplies:

- stable target id;
- accepted subject kinds;
- disabled state;
- eligibility resolver;
- geometry-to-intent resolver;
- optional priority for otherwise equivalent nested targets;
- optional auto-scroll ownership; and
- drop callback receiving the revalidated semantic intent.

Eligibility may depend on current consumer state but must not mutate it. It is
run during hover and again at drop. A target removed or disabled before commit
rejects the drop.

Nested arbitration is deterministic:

1. discard targets that do not contain the pointer or keyboard-selected point;
2. discard ineligible targets;
3. prefer the deepest registered target;
4. apply explicit priority only among equivalent-depth targets;
5. use stable registration order as the final tie-break; and
6. report one current intent, never several simultaneous drops.

## Internal Pointer Sensor

The web pointer sensor uses Pointer Events. It must:

- track one primary pointer id;
- capture the pointer after activation;
- apply configurable distance and optional hold constraints;
- cancel a hold when movement exceeds tolerance before activation;
- let ordinary scrolling win before activation;
- use current measured/cached geometry after scroll and resize invalidation;
- coalesce move work to one animation frame without losing the final release;
- clean up capture, listeners, preview, source/target attributes, and scroll
  state exactly once; and
- handle `pointercancel`, lost capture, visibility loss, source unmount, and
  target unmount.

## Public Web Custom-Surface API

`g16.022` exposes one framework-free DOM controller from
`@inflatable-cookie/poodle-core`:

- `createDragDropController(options?) -> DragDropController`;
- `DragDropController.connect(root) -> cleanup`, with one connected root and
  its owner document per controller;
- `registerSource(element, registration) -> DragSourceHandle` and
  `registerTarget(element, registration) -> DropTargetHandle`;
- `getSnapshot()`, `subscribe(listener)`, `invalidateLayout()`, `cancel()`, and
  idempotent `destroy()`;
- immutable `capabilities` for pointer, touch, and keyboard on this internal
  transport; and
- handles with `update(registration)` and idempotent `unregister()`.

There is no default controller, document-global registry, or module singleton.
Two providers in one document own independent controllers and sessions.
Connecting the same controller twice, duplicate live source ids, and duplicate
live target ids are errors rather than last-writer-wins behavior.

The paired public registration names are:

- `DragSourceRegistration`: `sourceId`, opaque `subject`, `allowedOperations`,
  initial `operation`, `disabled`, required accessible `label`, optional
  `instructions`, optional `handle` (`Element` or a selector inside the
  source), per-pointer `activation` (`DragActivationConstraints`),
  `onDragStart`, and `onDragEnd`;
- `DropTargetRegistration`: `targetId`, `acceptedKinds`, `disabled`,
  `priority`, required accessible `label`, `resolvePosition`
  (`DragPositionResolverInput` → `DropPosition | null`), `canDrop` (boolean or
  `DropEligibility`), and `onDrop`; and
- `DragDropCommitResult`: committed, rejected with an optional reason, or
  failed with an optional reason. `onDrop` may return it synchronously or by
  promise. The controller rechecks `canDrop` before invoking it and maps one
  result into the existing kernel terminal event.

`resolvePosition` receives the adapter-owned point, cached target rectangle,
current subject, operation, and input kind. It returns a semantic
`DropPosition` or `null`. `canDrop` receives the resulting intent and live
subject; it cannot mutate. DOM geometry never enters `DragSession` or
`DropIntent`.

`DragActivationConstraints` has explicit mouse, pen, and touch entries. Mouse
and pen activate by distance. Touch activates only after its hold delay while
movement remains within tolerance; movement outside tolerance before the hold
cancels the candidate and leaves scrolling untouched. Pointer capture and
`touch-action` narrow to the registered source/handle only after activation.

`DragDropSnapshot` is an immutable presentation read containing the semantic
phase/session plus adapter-owned input kind, pointer position where present,
active source/target ids, accepted/rejected target posture, and current
preview position. It exposes no controller maps, elements, listeners, timers,
observers, or mutable machine context. `DragPreviewSnapshot` is the subset
passed to custom preview renderers.

Default announcements use the required source and target labels. A single
`describeAnnouncement(event)` option may replace the text for pickup, intent,
clear, committed, rejected, failed, and cancelled observations. The event is a
read-only description; it is not a second lifecycle callback.

Svelte exports `DragDropProvider` and `useDragDrop`. The hook returns the
provider snapshot store, `cancel`, and `dragSource` / `dropTarget` actions.
Actions own register, reactive update, and unregister for their element.

React exports `DragDropProvider`, `useDragDrop`, `useDragSource`, and
`useDropTarget`. The source and target hooks return stable
`getSourceProps` / `getTargetProps` prop getters plus read-only local posture.
The getters compose consumer handlers and refs; they do not overwrite them.

Both providers accept an optional explicit controller, a custom preview
renderer, `describeAnnouncement`, and children. They render one overlay and
one polite live region for their own controller. Provider unmount cancels one
active session, unregisters every source and target, restores focus when the
source survives, removes every document listener/observer/timer/attribute, and
destroys an internally-created controller exactly once. An injected controller
is disconnected but not destroyed.

Mouse, pen, and touch differ only in default activation constraints. Components
may choose a dedicated handle. Poodle does not attach document-wide
`touch-action:none`.

## Touch And Scroll

Touch is required in the first implementation. Defaults should favor scrolling
until intent is clear:

- a handle may use a small distance threshold;
- a whole-row source should normally use a short hold plus movement tolerance;
- release before activation is an ordinary tap;
- a scroll gesture before activation cancels drag preparation;
- after activation, pointer capture owns the gesture until drop or cancel; and
- auto-scroll accelerates near the active scroll container edge and stops on
  leave, cancellation, drop, or unmount.

Nested scroll containers choose the nearest eligible container that can still
scroll in the requested direction. The sensor must not run one timer per
target.

## Keyboard Sensor And Accessibility

Every reorder or move surface that is pointer-draggable has a keyboard route.
The baseline interaction is:

- Space or an authored shortcut picks up the focused source;
- arrow keys or target-navigation commands move the current intent;
- Home/End may choose first/last valid position where the component contract
  already uses them;
- Enter or Space drops;
- Escape cancels; and
- focus returns to the moved subject or the nearest surviving equivalent.

The component contract may choose a more familiar established pattern, but it
must use the same semantic session and commit path.

Announcements include pickup, current position/target, rejection reason,
successful drop, and cancellation. They are throttled so pointer motion does
not flood assistive technology.

Cross-window keyboard movement uses a host-provided target picker. It commits
through the same transfer transaction as pointer movement.

## Drag Preview

Preview content is presentation, not payload. A runtime may use a source
snapshot, lightweight authored preview, native drag image, or host-provided
icon. It must not move the real source node out of layout.

The source and target expose stable state hooks for styling:

- preparing;
- armed;
- dragging source;
- accepted target and position;
- rejected target; and
- dropping.

Reactive changes that can invalidate a WKWebView native drag image are delayed
until the transport reports it is safe. That platform fact belongs in the
adapter rather than every component.

## Cross-Window Host Bridge

The bridge is capability-based:

```ts
type CrossWindowDragBridge = {
  capabilities: {
    pointer: boolean;
    touch: boolean;
    keyboardTargetPicker: boolean;
  };
  prepare(subject: DragSubject, signal: AbortSignal): Promise<ArmedReceipt | null>;
  start(receipt: ArmedReceipt, transport: HostDragTransport): void;
  cancel(receipt: ArmedReceipt, reason: DragCancelReason): void;
  end(receipt: ArmedReceipt, result: HostDragResult): void;
};
```

The exact exported API may split these operations, but it preserves their
ordering and exactly-once terminal rule.

For Longhorn-backed transfer:

- Longhorn creates the session before native `dragstart` needs its payload;
- Poodle carries only protocol version plus opaque session id;
- Longhorn publishes leased target geometry and resolves the authoritative
  destination;
- the target rechecks current eligibility before commit; and
- Longhorn owns mutation, rollback, expiry, and recovery.

Poodle may render projected target state. It does not infer a commit from
hover, pointer release, or `dropEffect` alone.

## Native DataTransfer Adapter

The adapter is optional and isolated. It may:

- write an opaque host session token during synchronous `dragstart`;
- read declared MIME types during `dragover`;
- read accepted data at `drop`;
- receive inbound browser files; and
- map native effect negotiation to semantic operation.

It must not make `DataTransfer` the session store. A module-global or host
side-channel may resolve an in-process session only behind the adapter and
must be cleaned on every terminal path.

External data is untrusted. Adapters validate type, size, count, protocol
version, and host-issued token before presenting eligibility.

## Native File Drag-Out

The public Poodle boundary is capability-based:

```ts
type DragExportCapabilities = {
  files: boolean;
  multipleFiles: boolean;
  promisedFiles: boolean;
  customDataTypes: readonly string[];
};

type PreparedFileExport = {
  receiptId: string;
  displayName?: string;
};
```

The receipt is opaque to Poodle. Filesystem paths, file descriptors, and
temporary-directory handles remain in the host.

The export adapter must distinguish:

- an existing file;
- an eagerly materialized temporary file;
- a promised/lazy file when the platform adapter advertises support; and
- custom external data when both adapter and consumer explicitly opt in.

Preparation starts early enough to arm the native drag, is abortable, and has
visible progress or disabled state when it cannot complete immediately. A
native drag cannot start with an unready receipt.

Electron adapters may map an armed file receipt to `webContents.startDrag`.
Tauri consumers supply a native plugin/application adapter. Poodle packages do
not import Electron, Tauri, shell IPC, filesystem, or application types.

For Loophole, the first proof is an opaque consumer subject resolved to a real
file suitable for a DAW or desktop destination. Whether that subject is an
already-rendered file, a clip requiring materialization, MIDI, a preset, or
multiple files remains consumer policy and does not change the Poodle API.

## Inbound Files

Inbound file drop uses the same target registration and eligibility result,
but the host adapter resolves external file descriptors or paths. Browser
`File` objects and Tauri path events remain adapter-specific inputs.

The target receives opaque accepted-file receipts or a consumer-authored
projection, not an unchecked native path. Tauri's native file-drop capture can
conflict with frontend HTML5 drag events on some platforms, so the host adapter
must advertise which inbound transport owns the window rather than enabling
both silently.

## Failure And Cancellation Rules

The session cancels when:

- preparation declines, fails, is aborted, or completes after supersession;
- the activation sensor cancels;
- Escape or the accessible cancel action runs;
- the source disappears or becomes disabled;
- the transport is lost;
- every valid target disappears before commit;
- drop-time eligibility rejects the intent; or
- the host transaction expires or refuses commit.

A rejected drop may return to `dragging` only when the transport still owns an
active gesture. Pointer release and native OS drop are terminal.

Handlers are idempotent at the boundary. Poodle guarantees at-most-once start,
drop request, and terminal notification per session; hosts still protect their
own mutation commands with session identity.

## Mutation Helpers

The substrate reports intent. Optional pure helpers may calculate common
results such as reordering a flat list or moving a tree node, but they:

- accept immutable input and return a result;
- never mutate consumer state;
- do not bypass eligibility or commit callbacks; and
- are not required to use the substrate.

Component contracts remain authoritative for callback payloads. Migrating a
component must preserve or deliberately revise its public result shape before
implementation.

## Migration Inventory

Initial known bespoke web implementations:

- Tabs — flat reorder plus keyboard reorder;
- EditableList — flat reorder and editable-row interaction;
- Tree — nested before/inside/after movement and auto-scroll;
- ModelCatalogueEditor — reorder plus hidden/shown controls;
- OrderBy — rule reorder;
- BlockEditor — block reorder and nested controls;
- DockRegion — within-region reorder plus host cross-window bridge.

GPUI already has semantic payload start/hover/leave/drop/end channels used by
Tabs, Tree, and ModelCatalogueEditor. The new Rust substrate should converge
those channels rather than discard working semantics.

Migration happens in bounded waves. Old component-local session state and
global side channels are deleted only after their mounted replacement passes.

### Approved public migration boundary

The operator approved a clean pre-1.0 replacement on 2026-08-28:

- delete the DOM-shaped public Tabs reorder state/handlers and framework
  re-export modules in the simple-reorder migration; retain only the existing
  pure `applyReorder` semantic helper from the Tabs machine;
- delete the old DockRegion `DockExternalDrag*` / `DockExternalDrop*` types,
  controller, global `dockPanelDragSession`, and framework re-exports after the
  opaque host bridge passes;
- preserve useful prepare, cancellation, drop-time revalidation, and
  `onPanelDrop` semantics through the new substrate rather than compatibility
  names or DOM-shaped wrappers; and
- make `PanelDragData.sourceZone` required during DockRegion migration and
  remove its older-build fallback.

Current public APIs stay documented until their mounted replacements land.
There is no alias, deprecation wrapper, dual controller, or silent fallback
period.

## Certification Matrix

### Pure machine

- identical lifecycle vectors in TypeScript and Rust;
- late preparation, supersession, repeated terminal events, target removal,
  revalidation, rejection, and operation change;
- deterministic nested arbitration; and
- exactly-once callbacks and cleanup.

### Mounted web

- Svelte and React custom consumer fixtures;
- mouse, pen-shaped Pointer Events, touch-shaped Pointer Events, and keyboard;
- tap/scroll versus drag arbitration;
- pointer capture loss and source/target unmount;
- nested targets, edge intent, overlay previews, and auto-scroll;
- Chromium and WebKit; and
- Electron and Tauri adapter stubs without importing shell code into Poodle.

### Mounted native

- GPUI pointer and keyboard pickup, hover, intent, drop, cancel, and rebuild;
- two independent sessions cannot collide;
- source/target disappearance is safe; and
- renderer-neutral Rust outputs remain consumable by deferred Jetstream.

### Host integration

- two windows under host automation;
- source preparation before activation;
- opaque token only across the boundary;
- moving/resizing target geometry during the drag;
- stale lease and rejected commit;
- source or destination window close;
- pointer and accessible keyboard transfer; and
- file drag-out preparation, native start, cancellation, and retained cleanup
  receipt.

An OS or DAW accepting a dragged file may require a bounded manual platform
smoke. Record it as manual evidence; do not fake it with a component callback.

## Specimens

Create human-facing examples for:

- simple reorder;
- nested before/inside/after placement;
- touch hold versus scroll;
- keyboard pickup and announcements;
- rejected target with reason;
- auto-scroll;
- cross-window bridge simulator;
- inbound file target; and
- drag-out preparing, ready, unavailable, cancelled, and completed states.

Put exhaustive sensor/transport cases in a separate conformance tab or harness,
not the main Examples view.

## Delivery Boundaries

Roadmap compilation should keep these dependency edges:

1. inventory and contract vectors;
2. paired semantic machines;
3. internal web pointer/touch/keyboard substrate and custom-surface API;
4. simple reorder proof in Tabs and EditableList;
5. nested intent and auto-scroll proof in Tree;
6. Rust/GPUI convergence;
7. Longhorn-shaped cross-window bridge and DockRegion migration;
8. inbound files plus Electron/Tauri drag-out adapters;
9. remaining component migrations and deletion of bespoke controllers; and
10. cross-runtime and host certification.

Cards may split those batches further when file overlap or review size demands
it. Cross-window and drag-out contracts shape the base even when their adapters
land after the internal proof.

The compiled runway is `docs/roadmaps/g16/021-drag-drop-semantic-kernel.md`
through `028-drag-drop-migration-and-certification-closeout.md`. `g16.021` is
the dispatched foundation. The public migration gate for `g16.023` and
`g16.026` is resolved; later cards remain planned until their landed
dependencies and any remaining API-shape gates are reconciled by the
orchestrator.

## Non-goals

- application window topology, persistence, authorization, or mutation
- DAW clip rendering, export naming, temp-directory policy, or cleanup jobs
- a universal cross-application custom MIME protocol
- pixel-identical native drag previews
- replacing continuous value gestures
- admitting Jetstream
- using specimen pages as exhaustive conformance fixtures
- creating a new portable component or scene IR

## External Platform References

- W3C Pointer Events: <https://www.w3.org/TR/pointerevents3/>
- WHATWG HTML drag and drop: <https://html.spec.whatwg.org/multipage/dnd.html>
- Electron native file drag-out:
  <https://www.electronjs.org/docs/latest/tutorial/native-file-drag-drop>
- Tauri inbound drag/drop events:
  <https://v2.tauri.app/reference/javascript/api/namespacewindow/#ondragdropevent>
