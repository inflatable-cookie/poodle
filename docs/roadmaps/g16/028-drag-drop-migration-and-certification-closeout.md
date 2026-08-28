# g16.028 — Drag-And-Drop Migration And Certification Closeout

Status: planned — final programme card
Depends on: `027-drag-drop-inbound-files-and-drag-out.md`
Governing refs: architecture 011, spec 069, the component continuation register,
and the ModelCatalogueEditor, OrderBy, and BlockEditor contracts

## Goal

Migrate the remaining known bespoke payload-drag components, delete replaced
controllers/side channels, and certify the complete substrate across the
active cohort. This card closes the programme; it does not broaden it into
continuous value gestures, general motion, Jetstream admission, or application
window/file policy.

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

- [ ] All seven programme-owned components use the common substrate on every
      active runtime where they expose payload drag/drop.
- [ ] Replaced local controllers, HTML drag source-of-truth state, and global
      side channels are gone; no compatibility aliases remain.
- [ ] Named mounted regressions move only newly proved GPUI component cells in
      the live ledger.
- [ ] Custom consumer APIs remain usable without a Poodle composite.
- [ ] Accessibility instructions, announcements, focus return, and reduced
      motion/preview behavior are documented and tested.
- [ ] Cross-window and file capability limits are honest; manual downstream OS
      acceptance is recorded separately.
- [ ] One closeout log records code removal, final evidence, unresolved platform
      limits, and the next non-drag programme choice.

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
diff check. Never run `*-windowed`, native visual, Jetstream preview/QA,
release, publication, or sibling mutation selectors.

## Stop Conditions

- A component contract or public callback must break without prior operator
  approval.
- A residual implementation is a continuous gesture rather than payload drag.
- Certification requires a new component/scene authority, specimen matrix,
  focus-taking automation, or claims OS consumption from a callback.
- Another programme or sibling repository is needed to close Poodle-owned work.

## Continuation

After operator-authorized merge, close the drag-and-drop programme in g16 and
choose the next component, accessibility, visual, motion, or Jetstream planning
checkpoint from the continuation register. Do not start it from this card.
