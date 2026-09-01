# 069 Dependable Drag And Drop Substrate

Status: active — compiled as g16.021–g16.028; g16.021–g16.027 merged;
g16.028 is ready as the final migration and certification checkpoint
Updated: 2026-09-01
Depends on: `../architecture/011-drag-and-drop-substrate.md`,
`../contracts/001-working-rules.md`,
`../contracts/components/tabs.md`,
`../contracts/components/tree.md`,
`../contracts/components/editable-list.md`,
`../contracts/components/dock-region.md`

## Purpose

Define a predictable drag-and-drop framework for Poodle components and
consumer-built surfaces. It must support same-document movement, nested
targets, auto-scroll, same-application cross-window transfer, inbound files,
and native file drag-out. Web adapters in browsers, Electron, and Tauri
webviews support mouse, pen-shaped pointers, touch, and keyboard. The stock
GPUI adapter consumes the same semantic lifecycle through the input
capabilities crates.io GPUI actually exposes.

The framework removes repeated event choreography. It does not centralize
application mutation or window authority.

## Fixed Scope

- One semantic session vocabulary implemented once per language pair:
  TypeScript for Svelte/React and Rust for shared native composition.
- One public custom-surface substrate per active runtime, not only internal
  helpers for Poodle components.
- Web pointer and keyboard sensors, including touch from the first delivery;
  native adapters publish their real input capabilities rather than inferring
  support from synthesized mouse events.
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
  state exactly once;
- skip interactive descendants (`button`, `input`, `textarea`, `select`,
  `a[href]`, `[role='button']`, `contenteditable`) and `[data-poodle-no-drag]`
  hosts so a whole-row source does not steal their pointerdown; and
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
- `registerKeyboardTarget(registration) -> KeyboardDropTargetHandle` for an
  ordered semantic target that has no DOM element and never participates in
  pointer hit-testing;
- `requestKeyboardDrop(command) -> boolean` for an established one-keystroke
  component shortcut that already knows its semantic source, target, and
  position;
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
  optional `keyboardOrder` (Space/Enter pickup opt-in and ordered logical
  traversal origin), at most one of `crossWindowSourceBridge` or
  `fileExportBridge`, `onDragStart`, and `onDragEnd`;
- `DropTargetRegistration`: `targetId`, `acceptedKinds`, `disabled`,
  `priority`, required accessible `label`, `resolvePosition`
  (`DragPositionResolverInput` → `DropPosition | null`), `canDrop` (boolean or
  `DropEligibility`), optional `inboundFiles` constraints, and `onDrop`, which
  receives the revalidated intent plus a `DropCommitContext` carrying the live
  subject and the external file batch when there is one;
- `KeyboardDropTargetRegistration`: the same target id, accepted kinds,
  disabled posture, priority, label, `canDrop`, and `onDrop`, plus a stable
  numeric `order` and `resolvePosition` over `previous | next | first | last`,
  subject, and operation rather than a DOM rectangle;
- `KeyboardDropCommand`: `sourceId`, `targetId`, and semantic `position`; the
  source registration supplies the subject and operation; and
- `DragDropCommitResult`: committed, rejected with an optional reason, or
  failed with an optional reason. `onDrop` may return it synchronously or by
  promise. The controller rechecks `canDrop` before invoking it and maps one
  result into the existing kernel terminal event.

`resolvePosition` receives the adapter-owned point, cached target rectangle,
current subject, operation, and input kind. It returns a semantic
`DropPosition` or `null`. `canDrop` receives the resulting intent and live
subject; it cannot mutate. DOM geometry never enters `DragSession` or
`DropIntent`.

Ordinary Space/Enter pickup is opt-in through `keyboardOrder`. Sources that
omit it leave those keys to the host component. When a keyboard source
declares `keyboardOrder` and matching logical keyboard targets exist, keyboard
traversal uses that ordered registry. `previous` and `next` are distinct
resolver inputs; a linear list normally maps them to `before` and `after`.
`first` and `last` remain explicit rather than being inferred from a synthetic
centre point. A logical target and a mounted DOM
target may share a `targetId`: the logical registration is keyboard authority,
the DOM registration is pointer/touch authority, and each registry rejects
duplicates within itself. Without logical targets, the existing spatial DOM
keyboard path remains available to custom surfaces.

Logical target removal clears or rejects its live intent by the same phase
rules as DOM target removal. Drop revalidates the logical registration before
calling `onDrop`. Source removal still cancels the session. Components do not
page or unmount the active source during intent navigation; after commit they
may reveal the resulting index.

`requestKeyboardDrop` starts only from idle with a live enabled source and a
matching live target. A matching logical keyboard target is authoritative when
present; otherwise the DOM target is used. The command creates an ordinary
keyboard session, runs the target's current `canDrop`, invokes its normal
`onDrop`, and reaches the same announcement, focus-return, async revalidation,
and terminal cleanup as pickup-mode keyboard input. It returns `true` when the
command entered that lifecycle, not when an asynchronous drop eventually
commits. A missing, disabled, mismatched, or busy registration returns `false`
without creating a partial session. Framework bindings expose the same method
from their existing drag-drop context surface.

`DragActivationConstraints` has explicit mouse, pen, and touch entries. Mouse
and pen activate by distance. Touch activates only after its hold delay while
movement remains within tolerance; movement outside tolerance before the hold
cancels the candidate and leaves scrolling untouched. Pointer capture and
`touch-action` narrow to the registered source/handle only after activation.

`DragDropSnapshot` is an immutable presentation read containing the semantic
phase/session plus adapter-owned input kind, pointer position where present,
active source/target ids, accepted/rejected target posture, current preview
position, the current file export, and the external files this window is being
offered. It exposes no controller maps, elements, listeners, timers,
observers, or mutable machine context. `DragPreviewSnapshot` is the subset
passed to custom preview renderers.

Default announcements use the required source and target labels. A single
`describeAnnouncement(event)` option may replace the text for pickup, intent,
clear, committed, rejected, failed, and cancelled observations. The event is a
read-only description; it is not a second lifecycle callback.

Svelte exports `DragDropProvider` and `useDragDrop`. The hook returns the
provider snapshot store, `cancel`, `dragSource` / `dropTarget` actions, and a
logical-keyboard-target registration helper whose handle owns reactive update
and unregister. DOM actions continue to own their element registrations.

React exports `DragDropProvider`, `useDragDrop`, `useDragSource`,
`useDropTarget`, and `useKeyboardDropTarget`. The DOM source and target hooks
return stable `getSourceProps` / `getTargetProps` prop getters plus read-only
local posture. The getters compose consumer handlers and refs; they do not
overwrite them. The logical hook registers no element and follows normal hook
cleanup.

Both providers accept an optional explicit controller, a custom preview
renderer, `describeAnnouncement`, and children. They render one overlay and
one polite live region for their own controller. Provider unmount cancels one
active session, unregisters every source and target, restores focus when the
source survives, removes every document listener/observer/timer/attribute, and
destroys an internally-created controller exactly once. An injected controller
is disconnected but not destroyed.

On web, mouse, pen, and touch differ only in default activation constraints.
Components may choose a dedicated handle. Poodle does not attach
document-wide `touch-action:none`.

## Touch And Scroll

Touch is required in the first web implementation. Defaults should favor
scrolling until intent is clear:

- a handle may use a small distance threshold;
- a whole-row source should normally use a short hold plus movement tolerance;
- release before activation is an ordinary tap;
- a scroll gesture before activation cancels drag preparation;
- after activation, pointer capture owns the gesture until drop or cancel; and
- auto-scroll accelerates near the active scroll container edge and stops on
  leave, direction exhaustion, cancellation, drop, or unmount. The frame loop
  is demand-driven: it does not keep a queued frame while the pointer is off
  an edge or the owner cannot move, and later pointer or layout movement may
  restart it.

Nested scroll containers choose the nearest eligible container that can still
scroll in the requested direction. The sensor must not run one timer per
target.

## Keyboard Sensor And Accessibility

Every reorder or move surface that is pointer-draggable has a keyboard route.
The baseline interaction is:

- Space or Enter pick up the focused source when it declares `keyboardOrder`;
  otherwise an authored shortcut or `requestKeyboardDrop` is the keyboard
  route;
- arrow keys or target-navigation commands move the current intent;
- Home/End may choose first/last valid position where the component contract
  already uses them;
- Enter or Space drops;
- Escape cancels; and
- focus returns to the moved subject or the nearest surviving equivalent.

The component contract may choose a more familiar established pattern, but it
must use the same semantic session and commit path.

Tree keeps its established Alt+Up/Down one-keystroke sibling move. It registers
its complete visible row catalogue as logical keyboard targets, resolves the
sibling target and before/after position with its pure Tree helper, and calls
`requestKeyboardDrop`; it does not call `onReorder` directly. Space and Enter
remain Tree selection/activation keys and do not enter pickup mode.

### Headless browser evidence boundary

Chromium's CDP leg proves native touch scrolling wins before hold and native
touch drag owns the gesture after hold. Playwright's desktop WebKit transport
cannot inject a native touch-move/scroll gesture. Its headless leg therefore
proves touch-shaped Pointer Event hold/tolerance behavior plus real WebKit
mouse/keyboard geometry, auto-scroll, and cleanup. It must say that this is not
native touch-scroll proof. Native WKWebView/iOS touch scrolling remains an
integration-certification item; it does not block the bounded Tree migration.

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

The bridge is capability-based and split by ownership. A source preparation is
owned by one draggable subject. Incoming projection, commit, and accessible
target picking are owned by one host window. They share only an opaque receipt
and semantic drag types; there is no controller-wide object that combines both
lifetimes.

The paired TypeScript names are normative. Rust uses the same public type and
trait names with idiomatic snake-case fields and host-supplied completion
callbacks where TypeScript returns a `Promise`:

```ts
type CrossWindowDragCapabilities = {
  pointer: boolean;
  touch: boolean;
  keyboardTargetPicker: boolean;
};

type CrossWindowDragReceipt = {
  protocolVersion: number;
  token: string;
};

type CrossWindowDragTransport =
  | "data-transfer"
  | "window-capture"
  | "keyboard-picker";

type CrossWindowDragPrepareRequest = {
  sessionId: string;
  sourceId: string;
  subject: DragSubject;
  operation: DragOperation;
  allowedOperations: readonly DragOperation[];
};

type CrossWindowDragProjection = {
  receipt: CrossWindowDragReceipt;
  sourceId: string;
  sourceLabel: string;
  subject: DragSubject;
  operation: DragOperation;
  inputKind: "pointer" | "touch" | "keyboard";
  targetId: string | null;
  position: DropPosition | null;
};

type CrossWindowDragTargetEvent =
  | { type: "projection"; projection: CrossWindowDragProjection }
  | { type: "left"; receipt: CrossWindowDragReceipt }
  | {
      type: "cancelled";
      receipt: CrossWindowDragReceipt;
      reason: DragCancelReason;
    };

type CrossWindowDragCommitRequest = {
  receipt: CrossWindowDragReceipt;
  subject: DragSubject;
  intent: DropIntent;
};

type CrossWindowDragSourceBridge = {
  readonly capabilities: CrossWindowDragCapabilities;
  prepare(
    request: CrossWindowDragPrepareRequest,
    signal: AbortSignal,
  ): Promise<CrossWindowDragReceipt | null>;
  start(
    receipt: CrossWindowDragReceipt,
    transport: CrossWindowDragTransport,
    onTerminal: (outcome: DragTerminalOutcome) => void,
  ): () => void;
  cancel(
    receipt: CrossWindowDragReceipt,
    reason: DragCancelReason,
  ): void | Promise<void>;
};

type CrossWindowDragTargetBridge = {
  readonly capabilities: CrossWindowDragCapabilities;
  subscribe(listener: (event: CrossWindowDragTargetEvent) => void): () => void;
  commit(
    request: CrossWindowDragCommitRequest,
    signal: AbortSignal,
  ): Promise<DragDropCommitResult>;
  pickTarget?(
    receipt: CrossWindowDragReceipt,
    signal: AbortSignal,
  ): Promise<CrossWindowDragProjection | null>;
};
```

`CrossWindowDragSourceBridge` is optional on one source registration. Its
preparation starts on the accepted pre-drag gesture, before activation. A
decline or failure cancels only that attempted cross-window session. A source
without the bridge keeps the internal transport's immediate preparation. A
source with the bridge cannot advertise or start a native cross-window gesture
until its matching receipt is armed. `start` installs one authoritative host
terminal subscription and returns its cleanup. Native `dragend`, pointer
release, or `dropEffect` never manufactures a committed result. Poodle calls
`cancel` only while the receipt is still live; late preparation and repeated
host terminal events are rejected by the kernel session id.

`CrossWindowDragTargetBridge` is optional on one document or native window
controller. The host resolves a receipt to the local semantic projection shown
above. `subject`, `sourceId`, and `sourceLabel` are a host-local projection;
they are not serialized beside the receipt. A projection names at most one
registered Poodle target and position. Poodle re-runs that target's kind,
disabled, and `canDrop` gates before `commit`, then maps the returned
`DragDropCommitResult` through the ordinary kernel terminal path. Target
removal, a changed projection, a stale receipt, or a mismatched drop envelope
cannot reuse hover acceptance. `pickTarget` is required exactly when
`keyboardTargetPicker` is true and uses the same projection, revalidation,
commit, announcement, and terminal path.

The web-only `CrossWindowDataTransferAdapter` is exported from the DOM adapter
surface. `createCrossWindowDataTransferAdapter()` uses
`application/x-poodle-cross-window-drag+json` and exposes `write`, `accepts`,
and `read`. The encoded body is exactly `{ protocolVersion, token }` with
bounded string and integer validation. `write` is valid only during native
`dragstart`; `accepts` consults the MIME type during `dragover`; `read` decodes
at `drop`. The live target bridge remains hover authority because the
`DataTransfer` body is unavailable then. A custom MIME option is allowed for a
host protocol, but it cannot change the normalized receipt shape or make
`DataTransfer` the session store.

Svelte and React `DragDropProvider` pass an optional
`crossWindowTargetBridge` to their controller. `DragSourceRegistration` gains
optional `crossWindowSourceBridge`; Tabs exposes the same semantic prop and
drops `onDragPrepare`, `onDragStart`, and `onDragEnd`. Tabs also exposes
optional `dragSubjectKind`: absent means an instance-scoped family, while an
explicit value lets an owning composite use a shared semantic family.
`TabItem.value` remains the subject id. Source and target registration ids are
always scoped to the Tabs instance, so repeated tab values in sibling strips do
not collide in an ambient controller. The renderer-neutral `TabsSpec` carries
the same optional semantic kind.

Tabs joins the nearest ambient provider and owns a private controller when none
exists. Its default instance kind keeps ordinary sibling Tabs mutually
ineligible even under one provider. With an explicit shared kind, each reorder
target must reject a subject id absent from that Tabs instance during
eligibility, not only at commit, so an eligible ancestor composite target can
win nested arbitration.

DockRegion replaces
`externalDragSource` / `externalDropTarget` with
`crossWindowDragSource` / `crossWindowDropTarget`. The latter names the
window-owned bridge; local same-document panel drops continue to use
`canAcceptPanel` and `onPanelDrop`. A host receipt uses the bridge's commit and
does not also invoke `onPanelDrop`, matching the old external-target split.

Same-document DockRegion transfer uses the ordinary shared drag controller,
not the cross-window bridge or a document-global panel session. A DockRegion
joins the nearest `DragDropProvider` when present and otherwise owns a private
controller for its own reorder behavior. Two sibling regions cross-drop only
when a common provider owns both registrations; consumers that need that
behavior wrap them in one provider. Two independently self-provided regions do
not discover each other, and no MIME, module singleton, or global registry
restores that link implicitly.

For a flexible DockRegion strip, the composite passes
`poodle.dock-panel` as `dragSubjectKind` and maps each public panel value to a
Tabs-internal subject id containing panel id, source edge, and required source
zone. DockRegion decodes at every public callback boundary. The encoded value
is substrate identity only: it cannot leak through active-value, close,
reorder, or `onPanelDrop` results.

On GPUI, `DragDropWindowHost` is an ordinary value owned one-per-window.
`drag_drop_window_host(&host, || root)` establishes that window's provider
census, appends the window-reaching end-of-frame sweep, and owns native drag
stop. Existing `drag_drop_provider(&controller, || subtree)` remains the
per-controller registration boundary and registers itself with the current
window host. A provider absent from that host's next completed frame is
cancelled once, its registrations are dropped, and
`App::stop_active_drag(window)` runs before the host forgets it. No thread-wide
controller registry or preview-only hook is allowed. The adapter README and
GPUI developer guide show both calls at every consumer root.

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

type DragExportForm =
  | "existing-file"
  | "materialized-file"
  | "promised-file"
  | "custom-data";

type PreparedFileExport = {
  receiptId: string;
  displayName?: string;
  form: DragExportForm;
  fileCount?: number;
  dataTypes?: readonly string[];
};

type DragExportTerminal =
  | { status: "ended" }
  | { status: "cancelled"; reason: DragCancelReason }
  | { status: "failed"; reason?: string };

type DragExportBridge = {
  readonly capabilities: DragExportCapabilities;
  prepare(
    request: DragExportPrepareRequest,
    signal: AbortSignal,
  ): Promise<PreparedFileExport | null>;
  start(
    prepared: PreparedFileExport,
    onTerminal: (terminal: DragExportTerminal) => void,
  ): () => void;
  cancel(prepared: PreparedFileExport, reason: DragCancelReason): void | Promise<void>;
};
```

The receipt is opaque to Poodle. Filesystem paths, file descriptors, and
temporary-directory handles remain in the host. `form` and `fileCount` are the
distinctions the adapter has to keep, and they are validated against that
adapter's own advertised capabilities before a receipt can arm anything: a
receipt beyond them is refused *and returned*, so an artifact made for a drag
that will never start is not abandoned. A `displayName` that is a path, a
drive letter, or a URL is refused rather than trimmed.

`DragExportBridge` is optional on one `DragSourceRegistration.fileExportBridge`
and mutually exclusive with `crossWindowSourceBridge`: one gesture leaves one
way, and a source declaring both would need a silent precedence rule. Export
preparation runs only for mouse and pen — there is no keyboard or touch route
to a desktop — and an adapter that can carry neither files nor an agreed
custom type stays inert.

There is no committed export terminal. A native drag ending does not prove a
destination consumed anything, so the honest qualities are `ended`,
`cancelled`, and `failed`. The kernel records the truth it can check — nothing
local committed — while the export's own state carries what the host reported,
and announcements use that one so a successful desktop drop is not announced
as a cancellation.

The export's visible state is separate from the session phase and outlives it:

```ts
type DragExportState =
  | "unavailable"
  | "idle"
  | "preparing"
  | "armed"
  | "dragging"
  | "ended"
  | "cancelled"
  | "failed";
```

It is published on the presentation snapshot and as a
`data-poodle-drag-export` attribute on the source, so `unavailable` (before
any gesture) and the terminal states (after one) are both showable. It also
travels on the announcement description in both runtimes, which is what makes
the states *accessible* rather than merely visible: ended, cancelled,
declined, and failed exports reach the same kernel cancellation, and each is
announced in its own words rather than as "cancelled".

Installing the native drag is safe against a host that answers inside `start`:
the subscription it returns is closed rather than stored on a session that is
already over. A `start` that throws leaves the export visibly `failed`; the
release that follows returns the receipt without overwriting that result.

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

```ts
const INBOUND_FILE_SUBJECT_KIND = "poodle.external-file";

type InboundFileTransport = "data-transfer" | "host";

type InboundFileCapabilities = {
  files: boolean;
  multipleFiles: boolean;
  transport: InboundFileTransport;
  customDataTypes: readonly string[];
};

type InboundFileReceipt = {
  receiptId: string;
  name: string | null;
  mediaType: string;
  size: number | null;
};

type InboundFileBatch = {
  protocolVersion: number;
  batchId: string;
  transport: InboundFileTransport;
  files: readonly InboundFileReceipt[];
};

type InboundFileEvent =
  | { type: "entered"; batch: InboundFileBatch; x: number; y: number }
  | { type: "moved"; batchId: string; x: number; y: number }
  | { type: "dropped"; batch: InboundFileBatch; x: number; y: number }
  | { type: "cancelled"; batchId: string };

type InboundFileHostBridge = {
  readonly capabilities: InboundFileCapabilities;
  subscribe(listener: (event: InboundFileEvent) => void): () => void;
  release(batchId: string, outcome: InboundFileOutcome): void;
};
```

One bridge per document or native window, on the controller. A batch becomes an
ordinary session under `INBOUND_FILE_SUBJECT_KIND` with `copy` as its only
operation, and reaches the same hit-testing, nested arbitration, eligibility,
revalidation, commit, announcement, and terminal path as any other subject. A
local gesture always wins: a batch arriving mid-drag does not supersede a drag
the user is still making. `release` is the single terminal notification per
batch — a notification, not a command, because retention is the host's.

The target receives opaque accepted-file receipts or a consumer-authored
projection, not an unchecked native path. A target declares
`inboundFiles: { maxFiles?, maxSize?, accept? }`, using the same `accept`
vocabulary as the file-upload surfaces, and it is validated *before* the
target's own eligibility resolver on every hover and again at drop. Protocol
version, transport identity, host-issued receipt identity, count, size,
declared type, and name shape are all checked there, so a consumer resolver
never has to defend itself against a hostile batch.

`protocolVersion` is checked first and must equal
`INBOUND_FILE_PROTOCOL_VERSION`, the one version a build accepts. A batch is
assembled by an adapter that ships separately from Poodle — a shell plugin
pinned to an older release, a bridge nobody updated — and a shape this build
cannot fully understand is one it cannot honestly claim to have validated. It
is refused before any other field is read, because none of them is trustworthy
yet.

Every observed batch reaches exactly one release. A batch refused for any
reason, one arriving while a local gesture or another batch owns the
controller, one published by a bridge that has since been replaced, and one
that arrives after the surface is gone are all *answered* — a silently ignored
batch would leave the host holding material for a gesture nobody will finish.
Repeating an id already owned is one observation rather than two, and an id
this installation has already answered stays answered: news for a released
batch can neither commit nor cancel, and a re-published `entered` for it opens
nothing. That tombstone is scoped to the publishing installation, so a
*replacement* host may legitimately use the same opaque text — an id is one
host's own name for something, not a global identity. It holds for the whole
of that installation's lifetime, with no threshold: an id that stopped being
inert after enough later ids would be a false negative in an exactness rule,
and the id evicted first is the one a repeating host is most likely to send
again.
Replacing a window's bridge ends the outgoing batch's session rather than
releasing under it, and the outgoing host's queued news is answered through the
host that published it.

`name` and `size` are `null` while the platform is still hiding them: a browser
discloses only item kinds and declared types during `dragover`. The unknown is
modelled rather than guessed, undecidable rules defer, and the drop-time batch
— where every answer exists — is validated again before it can commit.

Tauri's native file-drop capture can conflict with frontend HTML5 drag events
on some platforms, so the host adapter must advertise which inbound transport
owns the window rather than enabling both silently. The claim is exclusive and
checked at connection: a `data-transfer` bridge that cannot observe the
document, and a `host` bridge that also binds document drag events, are both
errors rather than a silently ignored half.

The web adapter is `createInboundFileDataTransferBridge`. It holds the `File`
objects; a consumer that needs them supplies its own `project`, and one that
does not never sees them. It claims the drop for the whole document while a
batch is live, because an unclaimed file drop navigates the window to the file
and destroys the surface the user was dragging onto.

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
  re-export modules when Tabs and DockRegion migrate together in the
  cross-window host-bridge lane; retain only the existing pure `applyReorder`
  semantic helper from the Tabs machine;
- add optional renderer-neutral `Tabs.dragSubjectKind`, with instance-scoped
  default behavior, ambient-provider participation, collision-free
  registration ids, and foreign-subject eligibility rejection as the bounded
  composition seam required by DockRegion;
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

- GPUI mouse and keyboard pickup, hover, intent, drop, cancel, and rebuild;
- stock `on_drag_move` proves the in-window capture-equivalent result;
- the adapter advertises pen, touch, and device-originated pointer cancel as
  unsupported on crates.io GPUI 0.2.2; tests must not infer them from mouse
  synthesis;
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
4. simple reorder proof in EditableList;
5. nested intent and auto-scroll proof in Tree;
6. Rust/GPUI convergence;
7. Tabs host-preparation migration, Longhorn-shaped cross-window bridge, and
   DockRegion migration;
8. inbound files plus Electron/Tauri drag-out adapters;
9. remaining component migrations and deletion of bespoke controllers; and
10. cross-runtime and host certification.

Cards may split those batches further when file overlap or review size demands
it. Cross-window and drag-out contracts shape the base even when their adapters
land after the internal proof.

The compiled runway is `docs/roadmaps/g16/021-drag-drop-semantic-kernel.md`
through `028-drag-drop-migration-and-certification-closeout.md`.
`g16.021`–`g16.027` are merged. `g16.028` is ready: the remaining web HTML-drag
owners are ModelCatalogueEditor, OrderBy, and BlockEditor; native
EditableList, OrderBy, and BlockEditor still need their contract reorder result
paths before final certification.

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
