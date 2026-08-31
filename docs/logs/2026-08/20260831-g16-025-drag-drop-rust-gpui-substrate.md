# g16.025 — Drag-And-Drop Rust And GPUI Substrate

Status: complete — merged in PR #108 after four orchestrator review rounds
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
unchanged. EditableList did not migrate: its native renderer documents reorder
and change as host-owned, it registers no source or target, and its rows carry
no element identity.

Two behaviors changed on purpose, both after review. A row dropped onto itself
is now *rejected* rather than silently accepted, so the target posture and the
announcement say so. And a reorder surface's subject kind is scoped to its
instance, so one Tabs cannot resolve a target in another Tabs, in Tree, or in
ModelCatalogueEditor when they share a controller.

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
- **The release point decides, not the last hover.** A gesture can reach
  mouse-up with no intervening move, and committing whatever the previous move
  left would drop on a target the pointer is no longer over. Release
  hit-tests its own position first, exactly like a move.
- **One sensor owns an open gesture, with Escape as the deliberate exception.**
  Traversal and drop keys are inert against a mouse-owned drag; Escape is the
  accessible cancel for any session, because a mouse drag a user cannot
  abandon from the keyboard is a trap.
- **Drag keys are taken in the capture phase, and their key-up prevents the
  default.** GPUI synthesizes a click from Enter/Space on *key-up* for any
  focused element with a click listener, so a keyboard pickup would otherwise
  also activate the row it picked up. `prevent_default` resets per dispatched
  event, so the suppression has to be re-applied on the matching key-up rather
  than set on the key-down.
- **Keyboard traversal starts from the source's declared `keyboard_order`, and
  stops at an absent boundary.** A source at order 5 between targets at 1 and 9
  moves Next to 9 and Previous to 1. A source past every target selects
  *nothing* on Next rather than wrapping backwards to the end, and the same in
  reverse — the web controller's `firstTargetAfterSource` /
  `firstTargetBeforeSource` rule, which returns and leaves the intent alone.
- **The pickup key is also the drop key, so with no target chosen it puts the
  row back down.** Reporting the key handled and leaving the drag open would
  strand a keyboard user in a gesture their own pickup key could not close.
- **Activation suppression is keyed by the key it suppresses**, not one flag.
  Any other key-up arriving first — a modifier, a neighbouring shortcut, an
  overlapping Enter while Space is held — would consume a single flag and let
  the real release synthesize the row's click after all.
- **A refusal is presentation, and it is arbitrated like an acceptance.**
  `resolve_drop_target` discards rejected candidates — correctly, a refusal
  must never become an intent — but a surface still has to be able to style the
  target refusing it. The refused set is handed to that *same* resolver as if
  it were acceptable and only the winner's identity and reason are kept, so
  there is no second copy of the deepest / priority / order rule to drift.
  Stricter than the web, which takes the first refusal in iteration order.
- **The controller holds the current target's clear callback**, rather than
  looking it up when the intent moves. The registry is rebuilt every frame, so
  a target removed while it held the intent would otherwise never be told it
  stopped — the one case the registration contract promises it will be.
- **A source that changes subject during a rebuild is a lost source.** Reusing
  one `source_id` for a new row would otherwise leave the old subject dragging
  and let it commit against the new tree.
- **Eligibility carries the instance scope, not just the ids.** Scoped source
  and target ids stop a duplicate-id collision; they do nothing about one
  surface resolving another's target. The subject kind does.
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
| Stale hover at release | `a_release_away_from_the_hovered_target_commits_nothing`, with `a_release_over_another_target_commits_the_one_under_the_pointer` as its mirror |
| Key against a mouse-owned drag | `keys_other_than_escape_cannot_drive_a_mouse_owned_drag` |
| Traversal origin | `keyboard_traversal_starts_from_the_sources_declared_origin` |
| Pickup double-firing activation | `a_keyboard_pickup_does_not_also_activate_its_own_source` |
| Cross-surface drop | `two_reorder_surfaces_under_one_controller_cannot_cross_drop` |
| Self-drop | same test, plus `tabs_drag_keyboard_and_identity_rebuild_the_host_spec` |
| Removed target's clear callback | `removing_the_current_target_during_a_rebuild_cancels_once` |
| Subject change during rebuild | `a_source_that_changes_subject_during_a_rebuild_cancels_once` |
| Traversal past an absent boundary | `keyboard_traversal_selects_nothing_past_an_absent_boundary`, both directions |
| Pickup key with no target chosen | `the_pickup_key_cancels_a_session_that_never_chose_a_target` |
| Unrelated key-up re-enabling activation | `an_unrelated_key_release_cannot_re_enable_the_suppressed_activation` |
| Rejected vs empty target | `the_snapshot_reports_accepted_rejected_and_empty_target_posture` — accepted → rejected → empty → terminal |

Six of these were falsified by planting the pre-fix behavior back and
confirming they fail: stale hover at release, pickup double-firing activation,
traversal boundary, pickup-key cancellation, per-key suppression, and rejected
target posture.

## Evidence

- Shared kernel unchanged: no edit to `packages/contracts/headless/src/drag_drop.rs`;
  no defect required one.
- Renderer-neutral vocabulary: `packages/contracts/node/src/drag.rs` tests.
- Shared construction: `packages/render/src/drag_drop.rs` tests, plus the
  migrated Tabs and ModelCatalogueEditor renderer tests.
- Controller units (capability matrix, identity, eligibility defaults,
  announcements): `packages/gpui/node-backend/src/drag.rs`.
- Mounted GPUI, real dispatch: `packages/gpui/preview/tests/headless_regressions.rs`
  — twenty-two custom-surface tests plus
  `tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec`, with
  `tabs_drag_keyboard_and_identity_rebuild_the_host_spec` and
  `model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window` retained.
  The Tabs test's one changed assertion is the self-drop rule: hovering the
  dragged tab now clears the indicator instead of pointing a tab at itself.
- Boards: `effigy ci:rust`, `effigy ci:native` (includes `regressions:native`,
  `probe:gpui-specimens`, `check:gpui`, `gpui:test`, `test:jetstream-adapter`,
  the drift gates, and the crates.io GPUI identity proof), `effigy docs:check`,
  `effigy qa`, `effigy check:parity-evidence-ledger`,
  `effigy test:parity-evidence-ledger`.
- No `*-windowed`, native visual, or Jetstream preview/QA selector was run.

## Ledger

Unchanged: 52 mounted / 122 missing. No cell moved.

Tree's mounted regression lands as substrate evidence — selection, twisty
expand, a keyboard command, a cancelled drag, and a committed nested reorder —
but the cell stays `missing`. Two native gaps keep it there, and both belong to
the card that migrates Tree's keyboard route:

- Tree's contract puts Alt+Up/Down sibling reorder on the component, but the
  native renderer reports those keys through `on_key` and the host executes
  them, so that authored behavior does not run through the shared semantic
  session. Intercepting the keys here would change what `on_key` reports — a
  public callback change, which is a stop condition for this card.
- `TreeHandlers` has no clear or terminal channel, so a host's `drag_value` /
  `drop_target_value` stay latched after a cancelled drag. Round 1 added two
  optional handlers for this; round 2 correctly ruled that a new public field
  crosses the same stop condition, so they are reverted and the latched state
  is asserted as the documented gap rather than quietly fixed.

EditableList stays `missing` for the reason above. Tabs and
ModelCatalogueEditor keep their existing cells.

## Review round 1

The orchestrator requested changes on PR #108. Six gaps, all closed here:

1. **Release used the stale hover intent.** Fixed: release hit-tests its own
   position; two mounted counterexamples, one of them falsified.
2. **Keyboard traversal ignored its declared origin, keys crossed sensor
   kinds, and pickup could also activate the row.** Fixed: origin-relative
   first step, Escape-only crossing, capture-phase keys with key-up
   `prevent_default`; three mounted counterexamples, one falsified.
3. **Reorder scoping was ids only, and the advertised self-rejection was never
   installed.** Fixed: the subject kind carries the instance scope and both
   reorder builders install `rejects_self`; one mounted and two unit
   counterexamples.
4. **A removed target lost its clear callback.** Fixed: the controller holds
   the callback rather than looking it up in the swept registry; asserted
   exactly once in the target-removal case.
5. **A source could change subject during a rebuild without cancelling.**
   Fixed and given its own mounted counterexample.
6. **Tree's ledger move was not backed by complete authored proof.** The cell
   is reverted to `missing`.

## Review round 2

Three more, all closed:

1. **Traversal wrapped past an absent boundary, and the pickup key could not
   close its own pickup.** Both now match the web controller exactly; three
   mounted counterexamples, all falsified.
2. **Activation suppression was one controller-global flag any key-up could
   consume.** It is now a set keyed by the released key. The counterexample
   holds Space, sends an unrelated key and an overlapping Enter, then releases
   Space; falsified.
3. **The Tree clear hooks were a public API change the card stops on.**
   Reverted, along with the assertions that depended on them. The latched
   indicator is now asserted as Tree's documented native gap, in its contract
   and here. The card was not expanded and no operator decision was assumed.

## Self-audit after review round 2

Both review rounds found real defects of the same class — the Rust path
diverging from the web authority, or a doc claiming something the code did not
do — so I ran the same pass over my own diff rather than waiting. It found one
defect and one hazard, and flagged two gaps I chose not to close unprompted.
Round 3 then ruled on all four; the outcome is recorded there.

- **Provider unmount left the session open.** Real, and reverted: see round 3.
- **The announcement log was unbounded.** A controller lives as long as its
  host, so every hover of every drag accumulated a string forever. Capped at
  `ANNOUNCEMENT_LOG_LIMIT` and unit-tested. There is nothing to throttle on
  this runtime — GPUI ships no accessibility API, so no assistive technology is
  being fed — but the log still had to be bounded.
- **`DragDropSnapshot` had no rejected posture.** Flagged as a gap; round 3
  ruled it a governing contract of this card, so it is now implemented.
- **Native EditableList registers nothing.** Recorded in the card; unchanged.

## Review round 3

1. **The snapshot omitted required rejected-target posture and reason.** Spec
   069 puts accepted/rejected posture on `DragDropSnapshot`, and I had filed it
   as an optional enhancement. It is not: without it a custom surface cannot
   paint a refused target at all. `DragDropSnapshot` now carries
   `target_posture` and `rejected_reason`, `target_id` names the refusing
   target when one is refusing, and `position` is `None` for a refusal because
   a refusal has no placement. Cleared on accepted intent, on leave, and on the
   terminal. Proved accepted → rejected → empty → terminal through mounted
   input, and falsified.

   One existing assertion changed with it: the cross-surface test used
   `target_id == None` as a proxy for "not eligible". A kind mismatch is a
   *refusal* — the web reports it the same way — so the test now asserts the
   posture and the absence of a placement, which is the stronger claim.

2. **The provider-unmount mechanism is reverted.** The gap is real but the fix
   crossed this card's stop condition against expanding into host windows, and
   it was wrong as written: `LIVE_CONTROLLERS` and the sweep mark were
   thread-global, so rendering window A would have swept controllers owned by
   window B and cancelled a live drag there merely because B did not render.
   The terminal also set `pending_stop_active_drag` with no host left to drain
   it, so GPUI's own drag was never proved stopped — and only the preview was
   moved to the new boundary, leaving the developer guide and adapter README
   documenting a root wiring that would silently miss the behavior.

   `LIVE_CONTROLLERS`, the host-frame sweep, `overlay_frame_end_with`, the
   preview/driver changes, and the unmount regression are gone. The requirement
   is carried to `g16.026` with both counterexamples named: the two-window
   false-cancel, and the native-active-drag cleanup proof. That card owns
   window ownership, which is where this belongs.

## Continuation

Round 4 independently re-ran the focused native, Rust, docs, and full headless
boards, confirmed the rejected-target snapshot and removal of the unsafe host
sweep, and approved PR #108. It merged as
`7a39f3c6d143784838e3e5cae4f05d9331c08f85`.

`g16.026` — the cross-window host bridge, the Tabs public migration, and
DockRegion — is next. It carries GPUI provider unmount with the two-window
false-cancel and native-active-drag proofs. Tree's keyboard route and
clear/terminal callback remain a separate component-migration gap; they are not
silently part of `g16.026`. Jetstream construction consumes the
renderer-neutral registrations only and remains deferred; no Jetstream preview
or QA selector was run or claimed.
