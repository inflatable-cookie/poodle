# g16.025 — Drag-And-Drop Rust And GPUI Substrate

Status: complete 2026-08-31 — merged in PR #108 after four orchestrator review rounds
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
- [x] Neither EditableList nor Tree moves; both stay `missing`, and the
      ledger is unchanged. EditableList registers no drag source or target and
      its rows carry no element identity to drive. Tree's contract puts
      Alt+Up/Down sibling reorder on the component, but the native renderer
      reports those keys through `on_key` and the host executes them, so that
      authored behavior does not run through the shared semantic session. A
      mounted claim would be incomplete, and intercepting the keys here would
      change what `on_key` reports — a public callback change, which is a stop
      condition for this card. Tree's mounted regression lands anyway as
      substrate evidence; the cell moves in the card that migrates the
      keyboard route.
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

Ledger: unchanged at 52 mounted / 122 missing.

## Review Rounds

Round 1 named six gaps, round 2 named three more, and round 3 named two. All
eleven closed before round 4 independently re-ran the focused native, Rust,
docs, and full headless boards and approved the PR for merge. The log records
each fix and its mounted counterexample.

Round 3 also reverted the provider-unmount mechanism a self-audit had added:
the gap is real, but closing it needs window ownership, which `g16.026` owns.
That card now carries the requirement and both counterexamples.

Two changed observable behavior on purpose: a self-drop is now *rejected*
rather than silently accepted, and a reorder surface is ineligible for another
surface's rows.

No component public callback changed. Round 1 added two optional
`TreeHandlers` fields to unlatch Tree's drop indicator after cancellation;
round 2 ruled that a new public field crosses this card's stop condition, so
they are reverted. The latched indicator is recorded as Tree's native gap in
its contract, asserted in the mounted regression, and carried to the card that
migrates Tree's keyboard route.

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
