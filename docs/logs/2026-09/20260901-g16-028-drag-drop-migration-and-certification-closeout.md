# g16.028 — Drag-And-Drop Migration And Certification Closeout

Status: delivered — awaiting orchestrator review
Date: 2026-09-01
PR: pending
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

## Evidence

| Claim | Proof |
| --- | --- |
| Web migrations behave | `test/drag-drop/components.html` mounted in Chromium and WebKit via `effigy test:drag-drop-browser`: 12 checks per framework, 48 across both engines |
| Native completions behave | `packages/gpui/preview/tests/headless_regressions.rs#editable_list_substrate_reorder_rebuilds_the_host_spec`, `#order_by_substrate_reorder_and_alt_arrow_rebuild_the_host_spec`, `#block_editor_grip_drag_and_move_controls_rebuild_the_host_spec` |
| Shared band/destination arithmetic | `packages/render/src/drag_drop.rs` unit tests for `arrival_band_resolver`, `reorder_destination`, and `apply_reorder` |
| Programme absence | `effigy drift:drag-inventory` (`scripts/check-drag-inventory.ts`), wired into `ci:web` |
| Ledger movement | `effigy check:parity-evidence-ledger`; four cells move, each naming its exact regression |

## Ledger

GPUI mounted behaviour: **52 mounted / 122 missing → 56 mounted / 118
missing**. EditableList, OrderBy, and BlockEditor move on their new
regressions; Tree moves on `tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec`,
which g16.025 landed and which no ledger row had ever named.

## Accepted limits

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

The drag-and-drop programme is complete. The next checkpoint is an ordinary
continuation choice from `docs/roadmaps/g16/component-continuation-register.md`
— component evidence, accessibility, visual comparison, motion, or Jetstream
admission. `g16.033` stays reserved at its public API decision gate. This log
does not compile that card.
