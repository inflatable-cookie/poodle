# g16.006 — Tabs Drag, Keyboard, And Mounted Parity

Date: 2026-08-26
Status: complete — merged in PR #80 (`9e5934af`)
Branch: `t3code/tabs-drag-keyboard-parity`
Card: `docs/roadmaps/g16/006-tabs-drag-keyboard-and-mounted-parity.md`

## Outcome

Tabs has one observable selection, focus, close, keyboard-reorder, and pointer-
reorder contract across Svelte, React, shared Rust, and GPUI. The existing
node payload/drop seam reports a complete semantic lifecycle on stock GPUI:
one start after the 2px threshold, hit-tested target hover/leave, a drop with
the retained edge, and one end after drop or cancellation. One named headless
GPUI regression drives the real mounted tree through controlled host rebuilds.

The generated ledger moves only Tabs' GPUI mounted-behaviour cell from
`missing` to `mounted`. Summary: 35 → 36 mounted; 139 → 138 missing. GPUI
accessibility stays `manual`. Visual comparison stays `missing`. Jetstream
stays deferred. Tree and ModelCatalogueEditor remain regression consumers of
the corrected payload seam; their ledger cells do not move.

## Generic payload lifecycle

- `packages/contracts/node/src/lib.rs` — `NodeKey::Delete`; source
  `on_drag_start` / `on_drag_end`; zone `on_drop_leave`. `NodeDropEvent` is
  unchanged. Coordinates stay out of the component.
- `packages/gpui/node-backend/` — one backend-owned payload session shared by
  the production overlay host and the headless mount host. Hover is
  hit-tested against the zone's own bounds. Drop reuses the last hover edge.
  Root mouse-up cancels an unfinished session after a successful zone drop
  has already ended it. Escape cancels once and stops stock GPUI's drag.
- `packages/render/src/model_catalogue_editor.rs` — leave clears
  `on_drop_target_change(None)`. Public contract unchanged.
- focused proof:
  `packages/gpui/preview/tests/headless_regressions.rs#payload_lifecycle_hit_tests_retains_edge_and_ends_once`

Event order on a successful drop: `start` → hit-tested `hover` / `leave` →
`drop` with retained `DropEdge` → `end`. Outside release and Escape emit
`leave` then exactly one `end`, never `drop`. A disabled source is inert.

## Shared machines and web

- `packages/core/src/tabs.ts`, `packages/core/test/tabs.test.ts`, and
  `packages/contracts/headless/vectors/machines.json` — complete-order
  `REORDER`, bound `REORDER_STEP`, focus-on-moved-tab
- `packages/svelte/components/test/Tabs.test.ts` and
  `packages/react/components/test/Tabs.test.tsx` — automatic/manual
  activation, orientation-aware roving, Delete, Alt+Arrow complete order,
  pointer start/over/leave/drop/end, cancellation cleanup, disabled inertia.
  Public props unchanged.

## Shared Rust Tabs

- `packages/render/src/tabs.rs` — `TabsHandlers` gains `on_reorder`,
  `on_drag_start`, `on_drag_end`, and `on_drop_target_change`. `on_reorder`
  consumes the transition's owned `Vec<String>`. Pointer drop and
  Alt+orientation-arrow run `tabs_transition`. Delete maps to `Close`.
  Reorderable enabled tabs publish payload/drop intent. Disabled tabs stay
  inert.
- `packages/gpui/preview/src/node_compat.rs` and
  `packages/gpui/preview/src/specimens/tabs.rs` — mechanical controlled-state
  migration with stable instance scope. Deferred Jetstream callers compile
  against added Default fields.

## Mounted tests

`packages/gpui/preview/tests/headless_regressions.rs`:

- `payload_lifecycle_hit_tests_retains_edge_and_ends_once` — generic seam
- `tabs_drag_keyboard_and_identity_rebuild_the_host_spec` — automatic and
  manual selection, horizontal and vertical roving, disabled skipping, close
  button and Delete, Alt+Arrow complete-order result, pointer
  start/hover/leave/drop/end, cancellation cleanup, moved-tab focus, two
  same-valued tabsets with independent runtime identity
- `model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window`
  retained; ModelCatalogueEditor and Tree ledger cells do not move

## Remaining gaps

- GPUI accessibility remains `manual`. Node-level tablist/tab/panel roles
  are not broad native assistive-technology proof.
- Tabs visual comparison remains Button-only / missing on GPUI.
- Overflow measurement, history sync, and tooltip timing stay web adapter
  effects.
- Jetstream preview was not compiled in this worktree. Call-site
  compatibility is mechanical and compile-only.
- The next evidence decision belongs to the orchestrator after review.
