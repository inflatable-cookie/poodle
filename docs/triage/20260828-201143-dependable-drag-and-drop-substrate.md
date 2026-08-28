# Dependable Drag And Drop Substrate

Status: promoted
Captured: 2026-08-28
Source: operator report of recurring flaky drag/drop in Poodle and consumers
Promoted: `../architecture/011-drag-and-drop-substrate.md` and
`../specs/069-dependable-drag-and-drop-substrate.md`

## Problem

Drag-and-drop is repeatedly reimplemented and repeatedly breaks across Poodle
components and consumer applications. The web components currently mix several
independent HTML Drag and Drop implementations: Tabs has a small shared reorder
helper, DockRegion has a separate external-drag controller and window-global
session, while Tree, EditableList, BlockEditor, OrderBy, and
ModelCatalogueEditor keep local source/target/cleanup state. Shared Rust and
GPUI already expose a more semantic payload lifecycle, but it is not the common
authority for the web runtimes or consumer-built surfaces.

Common failure classes include missed cleanup, stale source/target visuals,
drop eligibility drift, incorrect hit testing or edge calculation, nested
target churn, scroll/overlay interaction, framework timing differences, and
tests that invoke callbacks without proving the mounted gesture.

## Candidate Direction

Build one renderer-neutral drag session machine and four transport families:

- an internal pointer/keyboard transport for reordering and application-local
  payload movement, using Pointer Events and explicit pointer capture on web;
- a host bridge for same-application cross-window movement, with the shell
  owning window geometry, IPC/native observation, and authoritative transfer;
- a bounded native HTML Drag and Drop adapter for browser capabilities such as
  inbound files and an opaque host-issued cross-window session token;
- a native shell export adapter for drag-out, with explicit capability
  negotiation and host-owned file materialisation, OS session start, and
  temporary-artifact lifetime.

Keep payload identity, eligibility, semantic drop intent, lifecycle, and
effects in the shared machine. Keep pointer capture, geometry measurement,
auto-scroll, drag preview rendering, and platform event translation in runtime
adapters. Components and consumers compose source, target, preview, and live
region primitives rather than writing event choreography.

## Consumer Evidence

Longhorn contract 011 already owns the authoritative cross-window boundary:
bounded host-created sessions, leased target rectangles, deterministic target
resolution, and checked commit. Poodle must not duplicate window topology,
authority, durable mutation, or transfer transactions. Its public bridge must
let Longhorn arm a session before drag start, advertise only the opaque session
id, project target eligibility, and receive one terminal drop/cancel result.

Loophole currently proves why this bridge is needed. It combines Poodle's
DockRegion wire, a shell-global drag session, capture-phase ownership, and
Longhorn transfer state. HTML `DataTransfer` payloads are unreadable during
`dragover`, so hover eligibility depends on a side channel. WKWebView aborts a
new native drag when reactive DOM churn occurs during its drag-image snapshot,
so Loophole defers source-window UI changes by two animation frames. Explicit
zones, strip snapping, and window fallback also depend on event propagation
ordering. These are transport facts to centralize, not component behavior to
repeat.

Longhorn's GPUI proof uses another transport entirely: the source host captures
the gesture, observes live managed-window geometry at release, and resolves a
screen point through the same transfer coordinator. This supports a
transport-neutral semantic machine and host bridge rather than a universal
`DataTransfer` implementation.

Drag-out has the same split. Electron exposes a supported
`webContents.startDrag` path for one or more existing file paths. Tauri does
not expose an equivalent first-class API, so a Tauri consumer needs a bounded
native plugin or application adapter. Poodle can describe the export subject,
preparation state, preview, cancellation, and terminal result, but it cannot
portably manufacture files or begin every operating-system drag itself.

The first portable drag-out contract should therefore be file-backed:

- the consumer supplies an opaque export subject, never an eager filesystem
  path in component props;
- the host adapter prepares either an existing file or a temporary materialised
  file before native drag initiation;
- the adapter advertises supported export forms and rejects unsupported ones
  before the gesture appears draggable;
- the session owns one abort path and one terminal cleanup notification, while
  the host owns temporary-file retention and deletion;
- promised/lazy files and arbitrary custom MIME data remain explicit optional
  capabilities, not portable baseline behaviour.

This is especially important for audio applications: interoperability with a
DAW or the desktop is normally a real file transfer. A Poodle drag payload may
identify a clip, render, preset, or other domain subject, but only Loophole can
render or resolve that subject into an exportable artifact.

The likely public surface needs:

- `DragDropProvider` / controller scoped to a document or native host;
- source and target registrations with stable ids and typed payload kinds;
- explicit `idle -> preparing -> dragging -> dropping/cancelled -> idle`
  lifecycle with exactly-once end/cancel cleanup;
- target eligibility and semantic intent (`before`, `inside`, `after`, or a
  consumer-defined intent) separated from raw coordinates;
- deterministic nested-target arbitration and source/target enter/leave;
- keyboard pick-up, move, drop, and cancel through the same machine;
- drag overlay/preview, announcements, focus restoration, and auto-scroll;
- test driver that advances the machine through real mounted pointer and
  keyboard dispatch in Chromium, WebKit, and GPUI without taking operator
  focus.
- an export-source primitive with `preparing`, `ready`, `dragging`, `cancelled`,
  and `ended` presentation states, backed by a host adapter rather than direct
  filesystem access;
- transport capability queries so a consumer can distinguish file drag-out,
  promised-file drag-out, custom data, cross-window transfer, and inbound
  external files without user-agent branching.

## Guardrails

- Do not make HTML `DataTransfer` the internal source of truth.
- Do not expose DOM nodes, framework events, or raw GPUI geometry in the shared
  contract.
- Do not conflate continuous value gestures such as sliders/scrubbing with
  payload drag-and-drop, though they may share low-level pointer capture.
- Do not promise cross-window or OS drag through the internal pointer transport;
  that remains an explicit native transport capability.
- Do not pass filesystem paths through public component props or serialize them
  as the semantic payload. Hosts resolve opaque subjects at the native boundary.
- Do not delete a temporary export merely because the pointer left the source
  window. Native drag completion and destination consumption are not equivalent;
  retention policy belongs to the host adapter.
- Do not move Longhorn-owned session authority, window geometry, leases, target
  resolution, or commit transactions into Poodle. Poodle owns interaction and
  presentation; the host bridge owns external authority.
- Do not migrate every component in the first implementation card. Prove the
  substrate with representative reorder, nested target, and external/file
  cases, then migrate consumers in bounded waves.

## Decisions At Promotion

- Touch is required from the first release, alongside mouse/pen and keyboard.
  Activation constraints and scrolling arbitration therefore belong in the
  base sensor contract rather than a later adapter patch.
- Cross-window transfer is required in the first runway. Longhorn window
  management consumes it and Loophole already has an implementation to inspect.
- DockRegion-style cross-window transfer must shape the base contract even if
  its component migration lands after the first internal reorder proof.
- Native drag-out is required in the first runway because Loophole has concrete
  use for it. File-backed export is the portable baseline; promised files and
  arbitrary custom host payloads are negotiated extensions. File drop remains
  the first inbound external-data proof. Loophole's exact export subjects and
  materialisation remain consumer policy behind the opaque subject contract.
- Longhorn and Loophole currently prove a hybrid: an opaque host-issued session
  crosses the web path through native `DataTransfer`, while Longhorn owns the
  host transaction; GPUI resolves the same authority through captured pointer
  release plus live window geometry.
- Internal touch is required. Cross-window touch is advertised only when the
  host can observe it beyond the source window; otherwise mouse/pen and the
  accessible target picker remain the cross-window routes.
- The substrate returns semantic movement. Optional pure helpers may calculate
  a result but never own consumer mutation.
- Tabs and EditableList prove simple reorder, Tree proves nested intent and
  auto-scroll, and DockRegion proves the external host bridge in later bounded
  batches.

## Promotion Route

Promoted on 2026-08-28 into architecture 011 and spec 069. Roadmap compilation
waits for the active Select lane to close and for the g16 continuation
checkpoint to choose whether this becomes the next generation or a later
bounded programme.
