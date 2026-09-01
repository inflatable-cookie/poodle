# g16.028 — Drag-And-Drop Migration And Certification Closeout

Status: accepted and merged — PR #118 exact head `2e57e829b`; merge
`17a25d633`
Date: 2026-09-01
PR: https://github.com/inflatable-cookie/poodle/pull/118
Accepted head: `2e57e829b2cd991c9456a77257659ebb5c650d9c`
Merge: `17a25d633b7f953aa0e9cf2e14b8e91a6074ffae`
Review rounds: 4 (five blockers, then three, then one, then one plus a front-door row; all repaired on this branch)
Card: `docs/roadmaps/g16/028-drag-drop-migration-and-certification-closeout.md`
Handoff: `docs/handoffs/20260901-075640-g16-028-drag-closeout.md`
Governing refs: `docs/architecture/011-drag-and-drop-substrate.md`,
`docs/specs/069-dependable-drag-and-drop-substrate.md`
Branch: `codex/g16-028-drag-closeout`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-028-drag-closeout`

## Outcome

The drag-and-drop programme is closed. All seven programme components —
Tabs, EditableList, Tree, DockRegion, ModelCatalogueEditor, OrderBy, and
BlockEditor — reach the common substrate on every active runtime, and the
three that still owned an HTML drag lifecycle on the web no longer do.

Three natives that drew a reorder affordance without a result path now have
one. `poodle_render::editable_list`, `order_by`, and `block_editor` take
handlers, register renderer-neutral sources and targets, and emit the complete
next order — the same payload their web contracts publish. A grip or a move
button is drawn only when it can honour what it promises.

## Decisions worth keeping

- **A component's keyboard route did not change to match the substrate.**
  ModelCatalogueEditor's arrow keys each *commit* a move and emit the whole
  order; the substrate's keyboard sensor moves an intent and commits on the
  second keystroke. Adopting the sensor would have changed a public callback
  sequence, so only the pointer route migrated — exactly what the landed
  native path already does. OrderBy's Alt+Arrow and BlockEditor's move buttons
  keep their bindings and run as real sessions through `requestKeyboardDrop`,
  so eligibility, revalidation, and the single commit are the pointer route's.

- **"Land at the target" is a direction question, not a geometry one.**
  Tabs, OrderBy, BlockEditor, and ModelCatalogueEditor all publish a result
  that puts the dropped row *at* the row it was released on. Which half of the
  target the pointer is over cannot express that; the travelling direction can.
  `crate::drag_drop::arrival_band_resolver` is that rule written once, so the
  same gesture produces one order on web and native. EditableList keeps the
  geometric half-band, because its contract publishes before/after placement.

- **Isolation comes from the subject family, not the controller.** Each
  migrated component joins an ambient `DragDropProvider` when one exists. Two
  catalogues holding the same model ids under one provider are still unreachable
  from each other, because the kind is scoped to the instance and the
  registration ids are scoped separately from the consumer's item ids.

- **The grip is the handle; the body is not.** BlockEditor registers its source
  on the block with `handle: ".poodle-block-editor__drag-grip"`, so a press in
  the textarea or on a toolbar control never arms a drag. Natively there is no
  handle selector, so the source is registered on the grip node itself and the
  keyboard route stays with the move buttons — the same split, expressed with
  what each runtime has.

- **A focusable node the backend does not track is not focusable.** The GPUI
  backend only creates a focus handle for a focusable node that declares a
  focus ring. EditableList's handle, OrderBy's handle, and BlockEditor's move
  buttons all claimed `focusable` and had no ring, so none of them was reachable
  by keyboard at all. They declare one now.

- **A count-only EditableList registers nothing.** `EditableListSpec` can carry
  `item_count` with no items. A row with no identity cannot name what moved, so
  such a list stays readable and inert instead of advertising a reorder it
  cannot report.

- **The certification claim is executable.** "No programme component owns an
  HTML drag lifecycle" is a negative claim, and a negative claim decays the
  first time somebody plants the old path back. `effigy drift:drag-inventory`
  reads all seven components' Svelte, React, and shared-Rust surfaces plus
  their row sub-components, fails on any banned token, and also fails when a
  component reaches no substrate module at all — absence alone would pass a
  component that reorders nothing.

## Review oracle

Every row was falsified by planting the smallest pre-fix behaviour and watching
the named proof fail. The tree was restored and re-run green after each one.

| Invariant | Planted counterexample | Named proof, and what it said |
| --- | --- | --- |
| Instances stay isolated | web: ModelCatalogueEditor's subject kind loses its instance scope. native: `reorder_kind` drops the scope | `chromium: svelte ModelCatalogueEditor instances cannot cross-drop colliding ids` → `FAIL posture=1 orderB=beta,gamma,alpha`; `editable_list_substrate_reorder_rebuilds_the_host_spec` → FAILED |
| One accepted drop, one complete result | web: `handleDrop` emits a second order. native: the reorder emitter calls its handler twice | `chromium: svelte ModelCatalogueEditor drop emits one complete shown order` → `FAIL order=alpha,beta,gamma count=2`; `editable_list_substrate_…` → FAILED at the exact-count assertion |
| Eligibility and commit read live state | the catalogue's four live-state reads at once: both registrations' `disabled`, the handle's `disabled`, and the commit guards | `chromium: svelte ModelCatalogueEditor refuses a drop into a locked catalogue` → `FAIL order=beta,gamma,alpha count=1` |
| Reorder sensors do not steal nested controls | web: BlockEditor's source drops its `handle` constraint. native: the source moves from the grip to the whole block | `chromium: svelte BlockEditor toolbar chrome is not a drag handle` → `FAIL dragging=1 count=2`; `block_editor_grip_drag_and_move_controls_…` → FAILED at the body-press assertion |
| Every terminal clears posture | the catalogue latches its own drop-target id, as it did before the migration | `chromium: svelte ModelCatalogueEditor cancel commits nothing and clears posture` → `FAIL latched=1 count=0` |
| Absence and certification claims are exact | one `draggable` restored in OrderBy; one `dragOverIndex` restored in BlockEditor; one component's substrate import renamed; one ledger row naming a regression that does not exist | `drift:drag-inventory` failed on each of the first three, naming file, line, and reason; `check:parity-evidence-ledger` failed on the fourth as an unresolved evidence reference |
| Platform limits stay honest | not planted: the counterexample is writing a false capability claim into docs | The kernels, the GPUI node backend, and the capability matrix are untouched by this diff. The automated/manual split is stated in *Accepted limits* below. |

Two findings came out of the falsification rather than the implementation.

- **The live-state defence is layered, and only removing all of it commits.**
  Locking a catalogue mid-drag was still refused after the commit guards were
  removed, and again after both registrations were forced enabled: the handle
  is a disabled control by then, and the substrate refuses a drop from one.
  Four independent reads had to go before a locked catalogue accepted a move.

- **The absence inventory's presence half was too loose.** Its first version
  matched the substrate import by substring, so renaming
  `./drag-drop-context` to `./drag-drop-ctx-renamed` still passed —
  `"./drag-drop"` is a prefix of it. The markers are exact import specifiers
  now, and the same plant fails.

## Review round 1

Five blockers, all repaired on this branch and each closed by a proof that
fails against the pre-fix behaviour.

- **An ambient-provider OrderBy drew grips that never dragged.** The sort panel
  is portalled to the document body, and `onPointerDown` refuses a press whose
  source element is outside the root the controller was connected to. A joined
  OrderBy registered rows the ambient controller could never see. OrderBy now
  **always owns its controller** — it is the one programme component that
  cannot join, and the contract says so rather than repeating the blanket rule.
  Planting the join back returns `count=0`: the exact inert affordance.

- **An ambient ModelCatalogueEditor read one drop out twice.** The editor has a
  contract-mandated live region and announces "Moved Alpha to position 3 of 3."
  through it; the provider it joined also announced "Dropped Alpha on Gamma".
  `describeAnnouncement` belongs to whoever created the controller, and a
  joined component did not — so there was no way to silence it. The substrate
  gained `ownsAnnouncements` / `owns_announcements` on the source registration,
  in both languages: the controller narrates nothing for sessions whose source
  narrates itself. The decision is taken once at session start and held for the
  session's whole life, so a throttled intent and a late terminal answer the
  same way as the pickup did. External sessions have no local source and are
  unaffected.

- **Registration semantics were vaguer than the contract.** Three fixes, all
  now written down and tested: `isDragEnabled=false` registers **nothing** —
  not a disabled source — because a registered source is still
  keyboard-reachable and still nameable in an announcement, and the keyboard
  grab and move buttons are untouched; a locked editor disables source *and*
  target; `item.isDisabled` disables that row's **source only**, so a disabled
  model cannot be picked up and is still a place to put one. Refusing drops
  onto it would make a disabled row an unpassable wall in the middle of the
  list, which is the rule Tabs already publishes for a disabled tab. The same
  "register nothing" rule now covers OrderBy and BlockEditor, and the Svelte
  registration actions accept a `null` registration to express it.

- **The terminal oracle was proven for two of its four cases.** Added: the
  dragged model leaving mid-drag (source unmount) ends the session with no
  commit and no stranded posture; the move is announced once, in the editor's
  own region, with the provider's region empty; focus returns to the moved
  model's handle at its new position; and an EditableList keyboard pickup drops
  and commits with no grabbed posture left behind — the "successful drop after
  keyboard pickup" case. Native gained
  `model_catalogue_editor_pointer_drop_is_announced_once_by_the_editor`.

- **The branch was behind main.** Rebased onto `1d834be9f`. No file overlapped,
  so the rebase was mechanical, but the planning claims were stale: `g16.033`
  is now queued with its public API decision promoted and gated on accepted and
  merged `g16.028`, not reserved at a decision gate, and the post-g16 research
  queue sits behind that lane. The milestone status, generation index, roadmap
  front door, card, and this log say that.

### Round-1 falsification

| Repair | Planted counterexample | Named proof, and what it said |
| --- | --- | --- |
| OrderBy owns its controller | OrderBy joins the ambient provider again | `svelte OrderBy reorders inside an ambient provider it cannot join` → `FAIL sort=title,updated,size count=0` |
| The editor narrates its own sessions | `ownsAnnouncements` removed (web); `owns_announcements = false` (native) | `svelte ModelCatalogueEditor announces its move once, in its own region` → `FAIL provider="Dropped Alpha on Gamma"`; `model_catalogue_editor_pointer_drop_is_announced_once_by_the_editor` → FAILED |
| A disabled model is still a place to put one | the target starts reading `item.isDisabled` | `svelte ModelCatalogueEditor disabled model is unpickable but droppable-onto` → `FAIL order= count=0` |
| `isDragEnabled` is pointer drag only | the keyboard grab starts reading `isDragEnabled` | `svelte ModelCatalogueEditor isDragEnabled=false stops pointer drag only` → `FAIL order= count=0` |
| Focus follows the moved model | the focus request is removed from `emitOrder` | `svelte ModelCatalogueEditor returns focus to the moved model's handle` → `FAIL focused=""` |
| Keyboard pickup commits | `keyboardOrder` removed from the EditableList row | `svelte EditableList keyboard pickup commits and leaves no posture` → `FAIL rows=r1,r2,r3 count=0` |

## Review round 2

Three closeout blockers.

- **The authoritative contracts still described mechanisms the code had
  dropped.** BlockEditor's grip was still `draggable="true"` in its anatomy and
  parts table; OrderBy still gave the handle a `draggable` attribute in three
  places, still said "joined or owned controller", and still claimed joining
  changes arbitration; Tabs still described Svelte-owned `dragSourceIndex` /
  `dropTargetIndex`, a reorder sub-machine, and `DRAG_START` / `DRAG_OVER` /
  `DRAG_LEAVE` / `DROP` / `DRAG_END` machine events the Tabs machine has never
  had — it only ever sees `REORDER`. All seven contracts were scanned and only
  stale mechanism claims were removed; ordinary semantic uses of "drag" stayed,
  and a few loose ones ("rows are draggable") were sharpened to "drag sources".

  `drift:drag-inventory` now reads the seven contracts too, with a rule prose
  can carry: a contract may say a mechanism is **absent**, and may not say it
  is **present**. It joins wrapped prose back into paragraphs — a negation and
  the token it negates routinely land on different physical lines — and looks
  for the negation in a short window either side of the token rather than
  anywhere in the paragraph.

- **The terminal oracle was proven on the wrong subset.** The round-1
  focus/one-voice assertions were ModelCatalogueEditor-only and the
  keyboard-pickup case was EditableList-only. OrderBy and BlockEditor now have
  their own post-terminal assertions in both web frameworks and in their native
  regressions, and EditableList's native regression asserts its keyboard
  terminal and focus. Where the honest answer was weaker than the claim, the
  claim moved: see *Accepted limits*.

- **The `ownsAnnouncements` seam claimed a lifecycle it had not proven.** Added
  a focused controller regression for the latch and the reset boundary, and a
  native regression for the same shape. Both bite.

### Round-2 falsification

| Repair | Planted counterexample | Named proof, and what it said |
| --- | --- | --- |
| Contracts carry no removed-mechanism claim | `draggable="true"` restored to BlockEditor's parts table | `drift:drag-inventory` → `block-editor.md:45: contract still claims the HTML draggable attribute` |
| — | Svelte-owned `dragSourceIndex` / `dropTargetIndex` restored to Tabs | `drift:drag-inventory` → `tabs.md:217: contract still claims component-owned drag state` |
| — | "joined or owned controller" restored to OrderBy | `drift:drag-inventory` → `order-by.md:614: contract still claims OrderBy always owns its controller; it cannot join` |
| OrderBy's terminal has exactly one voice | OrderBy's source claims `ownsAnnouncements` | `svelte OrderBy terminal announces once and strands no posture` → `FAIL region=""` |
| BlockEditor's terminal strands no posture | the drop-target class latches on a component-owned id | `svelte BlockEditor terminal announces once and strands no posture` → `FAIL dragOver=1` |
| The announcement flag is latched, not looked up | the guard becomes a live registration lookup (TypeScript) | `silences every announcement of a self-narrating session, and only that session` → FAILED |
| The flag resets per session | the native flag becomes sticky and never clears | `a_self_narrating_source_silences_its_whole_session_and_only_that_session` → `FAILED: the next ordinary session is narrated: []` |

One plant did **not** bite, and the reason is worth keeping: making the *native*
guard a live lookup changes nothing observable, because the native controller
holds `active_source` as a clone taken at session start, so a lookup on it
already behaves like a latch in every flow the driver can produce. The native
latch is therefore the same shape as the TypeScript one rather than a repair;
what is genuinely provable natively is the reset boundary, and that is the plant
above.

## Review round 3

One blocker, and a good one: the announcement latch leaked across a session
boundary the tests had not crossed.

`session_owns_announcements` was set from the local source in
`begin_session` and cleared in `begin_inbound_session`, but `apply_projection`
— the third way a native session starts — set neither. Terminal cleanup clears
`active_source` and not the latch, so the sequence *self-narrating local
terminal → incoming cross-window projection* left the projection silent. A
remote drag inherited the silence a local composite had asked for, and there is
nothing on the far side of a projection to notice: it has no local source, so
this controller's live region is the only voice it will ever have. That
contradicted spec 069's own statement that external sessions are always
narrated. TypeScript resets at both external starts and was never wrong.

The fix is at the boundary rather than the instance. Ownership is now decided
once in `dispatch`, on the `Prepare` event that every entry point sends —
local pickup, inbound batch, incoming projection — derived from whether a local
source is active at that moment. The two per-entry-point assignments are gone.
Deciding this per entry point is what failed; the entry that forgets is the one
nobody is watching.

### Round-3 falsification

| Repair | Planted counterexample | Named proof, and what it said |
| --- | --- | --- |
| An incoming projection is always the controller's to narrate | ownership decided per entry point again, with `apply_projection` forgetting it — the exact pre-fix shape | `a_incoming_projection_is_narrated_after_a_self_narrating_local_session` → `FAILED: an incoming projection is announced: []` |

Under that plant the inbound-batch regressions stayed green, which is the other
half of the reading: inbound was covered, the projection path was not.

## Review round 4

One blocker, surfaced by round 3's own proof, plus a front-door row.

**The native projection path discarded the projection's accessible name.**
`CrossWindowDragProjection.source_label` is the host's own name for a subject
this window has no source for. The web controller stores it in
`externalSourceLabel`; `apply_projection` never did, so announcements fell
through to `session.subject.id`. Round 3's regression asserted `remote-row` and
so codified the defect: it proved the projection was *narrated*, and blessed
the fact that what it narrated was an opaque identifier. Read to the one person
who cannot see the row, "Picked up remote-row" is the failure, not the pass.

`apply_projection` now retains `projection.source_label` as the external source
label, cleared by the same terminal cleanup as an inbound batch's and rewritten
when a superseding projection installs. The regression requires the human label
and rejects the subject id, at pickup, at intent, and at the terminal.

**The g16 front door still called `g16.033` reserved.** Row 33 and the
portfolio-papercut bullet contradicted the promoted card, the README status
line, and the round-2 receipt. Both now read: queued, public API decision
promoted, gated on accepted and merged `g16.028` — not dispatch-ready until
this PR merges. The one remaining "reserved" mention sits inside the historical
`g16.026` paragraph and was put into the past tense rather than rewritten.

### Round-4 falsification

| Repair | Planted counterexample | Named proof, and what it said |
| --- | --- | --- |
| A projection is announced by its accessible name | the `external_source_label` assignment removed from `apply_projection` | `an_incoming_projection_is_narrated_after_a_self_narrating_local_session` → `FAILED: an incoming projection is announced: ["Picked up remote-row.", "remote-row: after Zone A."]` |

## Evidence

| Claim | Proof |
| --- | --- |
| Web migrations behave | `test/drag-drop/components.html` mounted in Chromium and WebKit via `effigy test:drag-drop-browser` |
| Native completions behave | `packages/gpui/preview/tests/headless_regressions.rs#editable_list_substrate_reorder_rebuilds_the_host_spec`, `#order_by_substrate_reorder_and_alt_arrow_rebuild_the_host_spec`, `#block_editor_grip_drag_and_move_controls_rebuild_the_host_spec` |
| One voice per session, natively | `packages/gpui/preview/tests/headless_regressions.rs#model_catalogue_editor_pointer_drop_is_announced_once_by_the_editor` |
| The announcement latch and its reset | `test/headless-dom/drag-drop-controller.test.ts#silences every announcement of a self-narrating session, and only that session`; `packages/gpui/preview/tests/headless_regressions.rs#a_self_narrating_source_silences_its_whole_session_and_only_that_session` |
| An external session is never silenced by the one before it, and is announced by its accessible name | `packages/gpui/preview/tests/headless_regressions.rs#an_incoming_projection_is_narrated_after_a_self_narrating_local_session` |
| Contracts carry no removed-mechanism claim | `effigy drift:drag-inventory`, which reads the seven contracts as well as their sources |
| Shared band/destination arithmetic | `packages/render/src/drag_drop.rs` unit tests for `arrival_band_resolver`, `reorder_destination`, and `apply_reorder` |
| Programme absence | `effigy drift:drag-inventory` (`scripts/check-drag-inventory.ts`), wired into `ci:web` |
| Ledger movement | `effigy check:parity-evidence-ledger`; four cells move, each naming its exact regression |

## Ledger

GPUI mounted behaviour: **52 mounted / 122 missing → 56 mounted / 118
missing**. EditableList, OrderBy, and BlockEditor move on their new
regressions; Tree moves on `tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec`,
which g16.025 landed and which no ledger row had ever named.

## Accepted limits

- **Focus return is proven where it exists, and named where it does not.**
  ModelCatalogueEditor returns focus to the moved model's handle (web), and
  EditableList's native keyboard drop leaves focus on the row it carried.
  OrderBy keeps focus on the handle the chord was pressed on, web and native —
  nothing moves focus, so no return is claimed. BlockEditor's web move control
  commits once and leaves focus *inside the block it moved*, not on the control,
  because the control is re-rendered at its new position; nothing is stranded.
  Returning focus to the control itself is a BlockEditor focus decision no card
  has taken.
- **A native component shortcut is not a substrate session.** The native
  controller exposes no keyboard-drop command to a renderer, so OrderBy's
  `Alt+Arrow` and BlockEditor's move buttons reach their own emitter directly.
  The public result payload is identical to the web route's; the native
  shortcut produces no substrate announcement. Recorded in both contracts and
  in spec 069 rather than papered over.

- GPUI 0.2.2 still exposes no pen, touch, or device-originated pointer cancel.
  The capability matrix is unchanged and nothing here infers those from mouse
  synthesis.
- An operating-system destination consuming an exported file remains manual
  downstream evidence. No callback in this change claims it.
- React axe coverage remains absent, as recorded in the ledger's limitations.
- BlockEditor's native type select, add select, and remove button remain
  host-bound and unwired. They are not drag affordances, so this card left them
  where it found them; they stay in the register as ordinary evidence gaps.
- The Jetstream preview specimens were updated for the new handler signatures
  but could not be compiled here: `poodle-jetstream-preview` path-depends on a
  sibling engine checkout that this worktree does not have.

## Next

The drag-and-drop programme is complete. The ordered runway continues to
`g16.033`, the HistoryCenter rejection surface, which is queued with its public
API decision promoted and gated on this card being accepted and merged. The
post-g16 research queue
(`docs/triage/20260901-080641-post-g16-research-queue.md`) sits behind that lane
and cannot displace it. This log compiles neither.
