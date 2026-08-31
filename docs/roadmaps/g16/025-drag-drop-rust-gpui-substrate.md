# g16.025 — Drag-And-Drop Rust And GPUI Substrate

Status: implemented 2026-08-31 — PR #108 open, orchestrator review pending
Depends on: `024-drag-drop-tree-nested-intent-and-auto-scroll.md`
Governing refs: architecture 011, spec 069, the Node/render architecture, and
the Tabs, EditableList, Tree, and ModelCatalogueEditor contracts

## Goal

Project the g16.021 Rust kernel through renderer-neutral Node construction and
the GPUI backend. Converge existing payload channels rather than replacing
working semantics. Prove custom consumer surfaces plus representative Tabs,
EditableList, Tree, and ModelCatalogueEditor interactions through real mounted
GPUI dispatch.

## Readiness Gate

Resolved 2026-08-31 against crates.io GPUI 0.2.2:

- mouse down/move/up/exit and keyboard down/up/focus are exposed and headlessly
  simulatable;
- typed `on_drag_move` continues outside the source hitbox and
  `on_mouse_up_out` closes release, providing the in-window capture-equivalent
  observable result without a public per-pointer capture handle;
- Escape can stop the active drag and Poodle already observes host rebuild;
- touch contacts, pen identity, and a device-originated pointer-cancel event
  are not exposed. `TouchPhase` belongs to scrolling, not touch contact.

The operator approved the stock-GPUI desktop baseline: certify mouse and
keyboard, advertise pen/touch/device-cancel as unsupported debt, keep full
touch support required on webviews, and never claim pen or touch from mouse
synthesis. Do not add a GPUI fork or platform input beneath GPUI.

## Required Runtime Shape

- Renderer-neutral Node vocabulary carries semantic source/target
  registrations and intent, never GPUI geometry or event types.
- `poodle-render` projects component registrations through reusable builders.
- The GPUI backend owns capture, hit testing, measured bounds, native event
  translation, preview painting, focus, and announcements.
- The public native controller exposes immutable input capabilities. On stock
  GPUI 0.2.2 they report mouse and keyboard support plus in-window captured
  movement; pen, touch, and device-originated cancel report unsupported.
- Source/target disappearance, host rebuild, two scopes, rejection, and
  repeated terminal events use the shared kernel's cleanup.
- Existing Tabs/Tree/ModelCatalogue payload code is removed only after mounted
  replacements pass. Continuous value drag remains separate.

## Acceptance Criteria

- [x] Custom Rust/GPUI source and target fixtures use the same semantic kernel.
- [x] Mounted tests cover mouse, keyboard, Escape/explicit cancellation,
      release outside, rebuild, nested arbitration, and two independent
      sessions through real GPUI dispatch.
- [x] Capability tests prove the exact stock-GPUI matrix and reject any pen,
      touch, or device-cancel claim based only on mouse synthesis.
- [x] Tabs and ModelCatalogueEditor preserve their existing mounted claims.
- [x] Tree moves to mounted on a named real-dispatch test covering selection,
      expand, a keyboard command, a cancelled drag, and a committed nested
      reorder. EditableList does **not** move and stays `missing`: its native
      renderer documents reorder and change as host-owned, it registers no
      drag source or target, and its rows carry no element identity to drive.
      Inventing either would be a new component feature, not this card.
- [x] Deferred Jetstream construction consumes renderer-neutral shape only and
      remains labelled deferred; no Jetstream preview/QA runs.

## Outcome

Delivered. The full account — capability matrix, design decisions, review
oracle, and evidence — is
`docs/logs/2026-08/20260831-g16-025-drag-drop-rust-gpui-substrate.md`.

Public surfaces added: `poodle_node::drag` (`NodeDragSource`,
`NodeDropTarget`, `NodeDragCapabilities`, and their resolver/handler types),
`poodle_render::drag_drop` builders, and
`poodle_gpui_node_backend::{DragDropController, drag_drop_provider,
GPUI_DRAG_CAPABILITIES, NativeDragPayload, DragDropSnapshot,
DragPreviewSnapshot, DragAnnouncementEvent}`.

Public surfaces removed, with no shim: `Interaction::drag_payload`,
`drop_zone`, `on_drag_start`, `on_drag_end`, `on_drop_hover`,
`on_drop_leave`, `on_drop`, and `NodeDropEvent`. `DropEdge` is retained as the
closed component-callback shorthand. No component's public callback changed.

The shared Rust kernel needed no extension: no defect surfaced under mounted
native dispatch.

Ledger: 52 → 53 mounted, 122 → 121 missing.

## Writable Scope

- focused Rust headless extensions only for a proven g16.021 defect;
- Node interaction vocabulary, poodle-render helpers/components, GPUI backend,
  GPUI compatibility/specimens, and named headless regressions;
- relevant component contracts and the parity ledger/checker only for honest
  mounted-cell moves;
- mechanical Jetstream compile maintenance only if renderer-neutral signatures
  move;
- this card, one log, g16 closeout, and `PAPERCUTS.md`.

Do not edit web components, cross-window/file transports, tokens, package
versions, workflows, releases, or sibling repositories.

## Validation

Run focused Rust/kernel/render/backend tests, named mounted regressions,
`effigy probe:gpui-specimens`, `effigy regressions:native`, ledger tests/check,
`effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, one final headless
`effigy qa`, and diff check. Never run `*-windowed`, native visual, or
Jetstream preview/QA.

## Stop Conditions

- Correct behavior needs a forked GPUI, window focus theft, or undocumented
  input support.
- Node vocabulary must expose GPUI geometry/events or durable mutation.
- A component's public callback must change or a ledger cell cannot be proved
  through real dispatch.
- Work expands into host windows, files, continuous value gestures, release,
  or siblings.

## Review Oracle

- **Invariant:** one GPUI provider owns one semantic drag session; stock mouse
  and keyboard input use the shared Rust lifecycle, one deepest live target,
  and exactly one terminal cleanup. Unsupported pen, touch, and device-cancel
  capabilities remain false.
- **Smallest adversarial counterexample:** mount two providers, start a mouse
  drag in one, rebuild or remove its source while an overlapping nested target
  changes eligibility, then release outside; separately perform keyboard
  pickup/drop and press Escape twice. Attempt to enable pen/touch capability
  from the same mouse fixture.
- **Expected failure/stop:** sessions collide across providers, a stale or
  shallower target commits, start/end fires twice, an outside release or
  rebuild leaks state, keyboard bypasses the kernel, or synthesized mouse input
  makes an unsupported capability true.
- **Required proof:** shared Rust vectors; focused Node/render tests; named
  mounted GPUI real-dispatch tests for custom fixtures and retained component
  behavior; capability assertions; renderer-neutral Jetstream construction;
  and the card's headless validation board.

## Continuation

After merge, promote `g16.026` for the cross-window host bridge and DockRegion.
