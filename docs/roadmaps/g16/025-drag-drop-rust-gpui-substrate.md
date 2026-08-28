# g16.025 — Drag-And-Drop Rust And GPUI Substrate

Status: planned — promote after web nested proof and GPUI input-capability check
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

Before ready status, record which mouse, pen, touch, pointer-cancel, capture,
and keyboard events crates.io GPUI 0.2.2 actually exposes. Implement supported
routes. Any missing active-cohort capability needs an explicit contract delta
and operator decision; do not silently label it complete.

## Required Runtime Shape

- Renderer-neutral Node vocabulary carries semantic source/target
  registrations and intent, never GPUI geometry or event types.
- `poodle-render` projects component registrations through reusable builders.
- The GPUI backend owns capture, hit testing, measured bounds, native event
  translation, preview painting, focus, and announcements.
- Source/target disappearance, host rebuild, two scopes, rejection, and
  repeated terminal events use the shared kernel's cleanup.
- Existing Tabs/Tree/ModelCatalogue payload code is removed only after mounted
  replacements pass. Continuous value drag remains separate.

## Acceptance Criteria

- [ ] Custom Rust/GPUI source and target fixtures use the same semantic kernel.
- [ ] Mounted tests cover pointer, keyboard, cancellation, rebuild, nested
      arbitration, and two independent sessions.
- [ ] Tabs and ModelCatalogueEditor preserve their existing mounted claims.
- [ ] EditableList and Tree move to mounted only if named real-dispatch tests
      prove their complete authored behavior; ledger changes are limited to
      those honest cells.
- [ ] Deferred Jetstream construction consumes renderer-neutral shape only and
      remains labelled deferred; no Jetstream preview/QA runs.

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

## Continuation

After merge, promote `g16.026` for the cross-window host bridge and DockRegion.
