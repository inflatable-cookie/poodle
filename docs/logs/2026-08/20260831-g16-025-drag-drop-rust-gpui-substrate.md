# g16.025 — Drag-And-Drop Rust And GPUI Substrate

Status: implemented — PR open, orchestrator review pending
Date: 2026-08-31
PR: https://github.com/inflatable-cookie/poodle/pull/108
Card: `docs/roadmaps/g16/025-drag-drop-rust-gpui-substrate.md`
Handoff: `docs/handoffs/20260831-095256-g16-025-rust-gpui-drag-substrate.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`,
`docs/contracts/components/tabs.md`, `docs/contracts/components/tree.md`,
`docs/contracts/components/editable-list.md`,
`docs/contracts/components/model-catalogue-editor.md`
Branch: `t3code/rust-gpui-drag-substrate`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-f3dd05b2`

## Outcome

The g16.021 Rust kernel now reaches GPUI through renderer-neutral Node
construction. `poodle-node` gained `drag_source` / `drop_target` registrations
that re-export the kernel's own `DragSubject`, `DropIntent`, `DropPosition`,
eligibility, and terminal types rather than mirroring them; `poodle-render`
owns the band, traversal, self-rejection, and scoping rules once; and the GPUI
backend gained a public `DragDropController` plus `drag_drop_provider`.

Geometry stops at the vocabulary. A position resolver receives *fractions* of
the target's own bounds — the same division `DropEdge` and `on_scrub` already
use — so no window point, rectangle, event, entity, or backend handle crosses
into a component.

The backend-global `PAYLOAD_SESSION` thread-local is deleted. A controller is
an ordinary value the host constructs, so two providers in one window own two
sessions and neither can reach the other's registrations, bounds, or intent.
`attach_overlay_host` no longer touches drag state at all: a window-global
release/cancel handler cannot tell two providers apart, which was the whole
defect.

Old `drag_payload`, `drop_zone`, `on_drag_start`, `on_drag_end`,
`on_drop_hover`, `on_drop_leave`, `on_drop`, and `NodeDropEvent` are gone with
no alias, wrapper, or fallback. `DropEdge` stays: it is the closed
three-value shorthand component callbacks speak, mapped at the render edge so
the substrate's `DropPosition` can stay open to consumer-defined placements.

Tabs, Tree, and ModelCatalogueEditor migrated with every public callback
unchanged. EditableList did not: its native renderer documents reorder and
change as host-owned, it registers no source or target, and its rows carry no
element identity — there is no honest mounted claim to make inside this card.

## Capability matrix

Published as a constant (`GPUI_DRAG_CAPABILITIES`), not a runtime probe. A
value that could be recomputed could also be talked into being true.

| Capability | crates.io GPUI 0.2.2 | Basis |
| --- | --- | --- |
| `mouse` | yes | `on_drag`, `on_drag_move`, `on_mouse_up`, `on_mouse_up_out` |
| `keyboard` | yes | key dispatch through the focus path; pickup, traversal, drop, Escape |
| `in_window_capture` | yes | `on_drag_move` is capture-phase and hitbox-free for the gesture's lifetime |
| `pen` | no | no pointer identity is exposed |
| `touch` | no | no touch contact is exposed; `TouchPhase` belongs to scrolling |
| `device_cancel` | no | no device-originated pointer-cancel event exists |

Mouse synthesis is never evidence for pen, touch, or device cancel. A mounted
regression drives a complete mouse drag and asserts those three stay false and
that the reported input kind is `Mouse`.

## Design decisions worth keeping

- **Hit testing reads the controller's own bounds**, recorded in the paint pass
  under each registration's `target_id`. Arbitration therefore never depends on
  the order GPUI happened to dispatch per-element listeners in, and one move
  produces exactly one resolved intent.
- **Depth comes from the node tree**, walked before the build. Two equally
  sized nested targets cannot be told apart by measured rectangles, and the
  backend converts each child through the public `to_gpui` entry — so the walk
  re-enters with depth reset and must keep the *first* answer of the frame. An
  earlier version overwrote, flattened every nested target to depth zero, and
  silently handed arbitration to explicit priority.
- **A disabled target stays registered.** Disabled is ineligible, not gone:
  arbitration skips it and the intent moves to a surviving ancestor. An
  unregistered target reads as removal and takes the kernel's `TargetLost`
  cancellation, which is the wrong answer for an eligibility flip.
- **The session holds its own source registration.** The registry is rebuilt
  every frame, so a host that removes the dragged row would otherwise have no
  handler left to receive its own terminal callback.
- **Effects run outside the state borrow**, with anything they cause queued
  rather than dispatched re-entrantly: a `RequestDrop` calls consumer code that
  may rebuild the host.
- **The frame sweep rides a zero-size paint canvas** appended last, not
  `App::defer`. It reaches a `Window`, so a rebuild-driven cancellation can
  also stop GPUI's own drag, and a host that wires the provider cannot forget a
  second call.
- **Registration scope.** The provider stack is exact while a provider's
  closure runs. GPUI renders a `RenderOnce` child or list row during *layout*,
  after every closure has returned, so the frame's top-level provider stays
  current until its sweep. Documented boundary: a provider that needs isolation
  from a sibling must convert its node trees inside its own closure.

## Review oracle

| Adversarial case | Proof |
| --- | --- |
| Two providers, one gesture | `two_providers_own_independent_sessions` — the neighbour stays `Idle`, its callbacks stay silent, its targets are not candidates, and it still works afterwards |
| Nested overlap | `nested_targets_arbitrate_deepest_first_and_follow_a_live_eligibility_change` — deepest wins against a higher-priority ancestor |
| Live eligibility change | same test — disabling the inner target during a rebuild moves the intent to the ancestor with no pointer input, and the stale target never commits |
| Source removal | `removing_the_dragged_source_during_a_rebuild_cancels_once` |
| Target removal | `removing_the_current_target_during_a_rebuild_cancels_once` |
| Host rebuild | both removal tests rebuild the mounted tree mid-drag |
| Outside release | `releasing_outside_every_target_cancels_once_and_commits_nothing` — and movement at (4, 4) still reaches the session |
| Keyboard drop | `keyboard_pickup_traversal_and_drop_use_the_same_session` |
| Repeated Escape | `escape_cancels_once_and_a_second_escape_is_inert` |
| Exactly-once callbacks | every drag test counts `end:` and asserts one |
| Rejected commit | `a_rejected_commit_ends_the_session_with_its_reason` |
| Duplicate live id | `a_duplicate_live_target_id_is_recorded_and_refused` |
| False pen/touch claim | `a_mouse_fixture_cannot_make_an_unsupported_capability_true` |

## Evidence

- Shared kernel unchanged: no edit to `packages/contracts/headless/src/drag_drop.rs`;
  no defect required one.
- Renderer-neutral vocabulary: `packages/contracts/node/src/drag.rs` tests.
- Shared construction: `packages/render/src/drag_drop.rs` tests, plus the
  migrated Tabs and ModelCatalogueEditor renderer tests.
- Controller units (capability matrix, identity, eligibility defaults,
  announcements): `packages/gpui/node-backend/src/drag.rs`.
- Mounted GPUI, real dispatch: `packages/gpui/preview/tests/headless_regressions.rs`
  — eleven custom-surface tests plus
  `tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec`, with
  `tabs_drag_keyboard_and_identity_rebuild_the_host_spec` and
  `model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window` retained
  unchanged.
- Boards: `effigy ci:rust`, `effigy ci:native` (includes `regressions:native`,
  `probe:gpui-specimens`, `check:gpui`, `gpui:test`, `test:jetstream-adapter`,
  the drift gates, and the crates.io GPUI identity proof), `effigy docs:check`,
  `effigy qa`, `effigy check:parity-evidence-ledger`,
  `effigy test:parity-evidence-ledger`.
- No `*-windowed`, native visual, or Jetstream preview/QA selector was run.

## Ledger

52 → 53 mounted, 122 → 121 missing. One cell moved: Tree, on a named
real-dispatch mounted regression covering selection, twisty expand, a keyboard
command, a cancelled drag, and a committed nested reorder. Tabs and
ModelCatalogueEditor keep their existing cells. EditableList stays `missing`
for the reason above.

## Continuation

`g16.026` — the cross-window host bridge, the Tabs public migration, and
DockRegion — is next. Jetstream construction consumes the renderer-neutral
registrations only and remains deferred; no Jetstream preview or QA selector
was run or claimed.
