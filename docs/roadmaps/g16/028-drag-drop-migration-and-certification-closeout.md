# g16.028 — Drag-And-Drop Migration And Certification Closeout

Status: complete — final programme card; delivered on `codex/g16-028-drag-closeout`
Depends on: complete and merged
`027-drag-drop-inbound-files-and-drag-out.md`
Governing refs: architecture 011, spec 069, the component continuation register,
and the seven programme component contracts

## Goal

Migrate the remaining known bespoke payload-drag components, delete replaced
controllers/side channels, and certify the complete substrate across the
active cohort. This card closes the programme; it does not broaden it into
continuous value gestures, general motion, Jetstream admission, or application
window/file policy.

## Landed Inventory

State at dispatch. `g16.021`–`g16.027` are merged. EditableList, Tree, Tabs,
and DockRegion use the common web substrate. Tree, Tabs, ModelCatalogueEditor,
and DockRegion project payload drag through renderer-neutral Rust
registrations.

Three web components still owned native HTML drag state and events:

- ModelCatalogueEditor;
- OrderBy; and
- BlockEditor.

Three native components still lacked their contract's reorder result path:
EditableList, OrderBy, and BlockEditor. Their visible handles or move controls
cannot stand in for an unwired payload lifecycle. ModelCatalogueEditor's
landed Rust path is the reference migration, not a second design.

## Migration Set

- ModelCatalogueEditor;
- OrderBy;
- BlockEditor; and
- any residual drag lifecycle in Tabs, EditableList, Tree, or DockRegion that
  earlier cards intentionally left behind and can now be deleted safely.

Preserve each component's authored callback result, controls, selection,
editing, focus, disabled/pending state, accessibility, and curated specimen.
Do not migrate sliders, faders, knobs, resize handles, scrubbing, envelope
points, or XY pads; those are continuous gestures, not payload drag/drop.

## Ordered Work

1. Update the three component contracts before implementation. Replace
   HTML-drag mechanism notes with renderer-neutral substrate behavior while
   preserving each existing public result: complete model order, complete sort
   order, and complete block order.
2. Migrate ModelCatalogueEditor, OrderBy, and BlockEditor in both web
   frameworks. Each instance joins an ambient provider when present and owns
   an isolated controller otherwise. Pointer, touch-shaped pointer, and
   keyboard routes share eligibility, revalidation, commit, terminal cleanup,
   announcements, and focus return.
3. Complete renderer-neutral Rust reorder registrations and host handlers for
   EditableList, OrderBy, and BlockEditor. Use the same full-result callback
   semantics as the web contracts. Remove dead native drag affordances rather
   than leaving a grip or move control that cannot produce its promised
   result.
4. Add mounted GPUI proof for every newly completed component path and one
   focused Tree regression if its existing substrate path still lacks a named
   mounted cell. Move only the ledger cells backed by those exact regressions.
5. Delete the three web HTML drag lifecycles and any now-unreachable local
   state. Run an exact source inventory over all seven programme components;
   every retained drag-shaped path needs a named non-payload reason grounded
   in its contract.
6. Reconcile spec 069, the continuation register, card, closeout log, g16
   front doors, and final certification evidence. Do not create a new
   conformance authority or exhaustive specimen surface.

## Certification

- Shared TypeScript/Rust vectors cover the final lifecycle.
- Mounted Svelte/React custom and component fixtures cover mouse, pen-shaped,
  touch-shaped, keyboard, cancellation, nested intent, auto-scroll, and files.
- Chromium and WebKit run headlessly.
- Mounted GPUI tests cover custom surfaces and migrated native components.
- Host simulator covers cross-window preparation, opaque transfer, target
  movement/revalidation, commit/refusal, cancellation, window loss, and export
  receipts.
- One inventory proves every known bespoke payload-drag implementation either
  migrated or deliberately retained with a documented non-payload reason.

## Acceptance Criteria

- [x] All seven programme-owned components use the common substrate on every
      active runtime where they expose payload drag/drop. EditableList,
      OrderBy, and BlockEditor expose a working native reorder result rather
      than presentational handles.
- [x] Replaced local controllers, HTML drag source-of-truth state, and global
      side channels are gone; no compatibility aliases remain.
- [x] Named mounted regressions move only newly proved GPUI component cells in
      the live ledger.
- [x] Custom consumer APIs remain usable without a Poodle composite.
- [x] Accessibility instructions, announcements, focus return, and reduced
      motion/preview behavior are documented and tested.
- [x] Cross-window and file capability limits are honest; manual downstream OS
      acceptance is recorded separately.
- [x] One closeout log records code removal, final evidence, unresolved platform
      limits, and the next non-drag programme choice.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Expected failure or stop | Required proof |
| --- | --- | --- | --- |
| Component instances remain isolated under one ambient provider | two instances contain the same item ids; start in A and hover/drop in B | B accepts, highlights, or commits A's subject | paired mounted web tests and renderer-neutral/native tests using colliding ids |
| One accepted drop emits one complete authored result | reorder B over C, then repeat terminal news or rebuild before release | duplicate callback, partial intent payload, or stale pre-rebuild order commits | exact callback-count and full-order assertions in Svelte, React, Rust, and mounted GPUI |
| Eligibility and commit use live component state | hover an enabled target, then disable/remove it or replace the order before release | removed/disabled target commits or an old index moves the wrong item | paired removal, disable, replacement, and drop-time revalidation cases |
| Reorder sensors do not steal nested controls | press a remove, Select, editor, or ordinary row action inside each component | a drag starts, focus moves, or the control action is lost | paired mounted web control-preservation tests and focused native control cases |
| Every terminal clears posture and preserves focus/announcements | cancel, source-unmount, target-unmount, and successful drop after keyboard pickup | dragging/drop-target state survives, focus is stranded, or terminal copy duplicates | mounted terminal/focus/announcement cases across the three migrations and native completions |
| Programme absence and certification claims are exact | restore one `draggable`, `dragstart`, `DataTransfer`, local drag index, global side channel, or dead native grip | inventory still passes or the ledger moves without named mounted evidence | executable absence inventory, falsified planted-token checks, ledger reproduction, and named evidence map |
| Platform limits remain honest | infer GPUI pen/touch/device-cancel or OS destination consumption from mouse/callback evidence | closeout reports unsupported behavior as certified | unchanged capability matrix plus explicit automated/manual evidence split |

## Writable Scope

- named component web/render/native implementations, tests, contracts, and
  curated specimens;
- landed drag substrate/adapters only for focused reusable defects;
- obsolete bespoke drag modules/exports after their approved migrations;
- certification fixtures/selectors/reports and the parity ledger/checker only
  for exact mounted-cell changes;
- this card, one closeout log, g16/front doors, and `PAPERCUTS.md`.

Do not edit continuous-value controls, motion architecture, broad accessibility
or visual programmes, Jetstream admission, package versions, releases,
workflows, downstream repositories, or siblings.

## Validation

Run focused component/substrate tests, all shared vectors, Chromium/WebKit
headless probes, mounted GPUI regressions/specimen probe, inventory and ledger
checks, active web/Rust/native/docs boards, one final headless `effigy qa`, and
`git diff --check origin/main...HEAD`. For each universal, exact, or negative
oracle claim, plant the smallest pre-fix behavior and confirm its named proof
fails before restoring the implementation. Never run `*-windowed`, native
visual, Jetstream preview/QA, release, publication, or sibling mutation
selectors.

## Stop Conditions

- A component contract or public callback must break without prior operator
  approval.
- A residual implementation is a continuous gesture rather than payload drag.
- A complete native reorder result needs a new public component semantic rather
  than an additive renderer-neutral handler matching the existing contract.
- Certification requires a new component/scene authority, specimen matrix,
  focus-taking automation, or claims OS consumption from a callback.
- Another programme or sibling repository is needed to close Poodle-owned work.

## Delivered

- Contracts first: `order-by.md`, `block-editor.md`,
  `model-catalogue-editor.md`, and `editable-list.md` now describe substrate
  behaviour and the complete result each callback carries.
- Web migrations: ModelCatalogueEditor, OrderBy, and BlockEditor in Svelte and
  React. Each joins an ambient provider when present and owns an isolated
  controller otherwise; registration ids and subject kinds are instance-scoped.
- Native completion: `poodle_render::editable_list`, `order_by`, and
  `block_editor` take handlers, register renderer-neutral sources and targets,
  and emit the complete next order. No grip or move control is drawn that
  cannot produce it.
- Shared arithmetic moved into `crate::drag_drop`: `arrival_band_resolver`,
  `reorder_destination`, and `apply_reorder`, so four surfaces stop restating
  the same rule.
- Evidence: mounted Svelte and React component fixtures in the headless
  Chromium/WebKit probe, three named mounted GPUI regressions, and an
  executable absence inventory (`effigy drift:drag-inventory`).
- Ledger: EditableList, OrderBy, BlockEditor, and Tree move from `missing` to
  `mounted`, each backed by its exact named regression.

## Continuation

After operator-authorized merge, close the drag-and-drop programme in g16 and
choose the next component, accessibility, visual, motion, or Jetstream planning
checkpoint from the continuation register. Do not start it from this card.
