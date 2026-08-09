---
title: GPUI node backend Batch B continuation handoff
status: active
owner: Poodle core
updated: 2026-08-07
tags: [coordination, handoff, g12.019, gpui]
---

## What This Thread Was Doing

This thread continued `g12.019`, the GPUI half of Poodle's renderer inversion:
move the GPUI preview from the duplicate
`/Users/example/Dev/projects/poodle/packages/gpui/components` tier to
`poodle-render` Nodes interpreted by the GPUI node backend. Waves 1–39 were
already present in the working tree when this thread resumed. Wave 40 moved
EditableLabel across the same seam and added the missing host-edit interaction
contract for submit and cancel.

The intended end state is one canonical component implementation with thin
backends, followed by deletion of
`/Users/example/Dev/projects/poodle/packages/gpui/components` after the preview
and native gate prove the migration.

## Why It Matters

Two GPUI component tiers currently encode the same product surface. Keeping both
means every contract or recipe change can diverge or be implemented twice.
The node backend makes `poodle-render` the shared implementation and leaves
GPUI-specific dispatch at one boundary. This is the remaining half of the
Poodle ↔ Jetstream inversion.

## Current State

- Done so far:
  - Batch A's `/Users/example/Dev/projects/poodle/packages/gpui/node-backend/`
    interpreter is present and tested.
  - Batch B Waves 1–39 are recorded in the roadmap and use preview-local node
    compatibility wrappers for the migrated families.
  - Wave 40 migrated EditableLabel. `Interaction` now carries
    `on_submit` (Enter/Tab) and `on_cancel` (Escape); the GPUI backend dispatches
    those intents, and the preview queues text/commit events through
    `NodeSpecimenEvent`.
  - The EditableLabel focused capture is a documented 0.5334% residual. Text
    raster parity is intentionally deferred by operator instruction.
  - The Wave 40 full native run compared 136 components, skipped six
    nondeterministic slugs, reported 36 documented residual failures, and had
    no capture failures or baseline writes.
- Still open:
  - Batch B has remaining parked imports: LogList, BlockEditor, Tree, SplitView,
    DockRegion, IconProvider, UiPresentationProvider, plus the nondeterministic
    AudioPlayer, VideoPlayer, and TimeAgo paths.
  - Batch C is not started: remove the dependency, delete
    `/Users/example/Dev/projects/poodle/packages/gpui/components`, port mined
    probes, log the deletion, and leave
    the permitted Jetstream roadmap pointer.
  - EditableLabel's lightweight node input still lacks native editor/IME,
    selection/caret behavior, and generic `max_length` enforcement. Treat this
    as an explicit backend risk; do not silently claim full native editing.
  - Active spec lane: none. The canonical execution surface is the promoted
    roadmap; there is no separate ready batch card under
    `/Users/example/Dev/projects/poodle/docs/roadmaps/g12/`.
- Canonical refs:
  - [`/Users/example/Dev/projects/poodle/docs/roadmaps/g12/019-gpui-node-backend.md`](/Users/example/Dev/projects/poodle/docs/roadmaps/g12/019-gpui-node-backend.md)
  - [`/Users/example/Dev/projects/poodle/docs/roadmaps/generation-index.md`](/Users/example/Dev/projects/poodle/docs/roadmaps/generation-index.md)
  - [`/Users/example/Dev/projects/poodle/docs/contracts/001-working-rules.md`](/Users/example/Dev/projects/poodle/docs/contracts/001-working-rules.md)
  - [`/Users/example/Dev/projects/poodle/docs/architecture/001-poodle-system-shape.md`](/Users/example/Dev/projects/poodle/docs/architecture/001-poodle-system-shape.md)
  - Prior lane handoff: [`/Users/example/Dev/projects/poodle/docs/logs/2026-08/05-225348-gpui-node-backend-batch-b-handoff.md`](/Users/example/Dev/projects/poodle/docs/logs/2026-08/05-225348-gpui-node-backend-batch-b-handoff.md)
- Remaining continuation envelope: continue Batch B one bounded composite at a
  time. LogList is the next practical candidate if its shared `LogEntry`
  metadata contract is promoted first. BlockEditor, Tree, SplitView, and
  DockRegion need slot/host-event design before migration. Batch C remains
  outside the next component wave.
- Lane budget / pause signal: no technical pause and no budget exhaustion. The
  operator explicitly requested a fresh thread handoff. Nothing is committed;
  preserve the current dirty tree and ask before committing or publishing.
- Key files:
  - [`/Users/example/Dev/projects/poodle/packages/contracts/node/src/lib.rs`](/Users/example/Dev/projects/poodle/packages/contracts/node/src/lib.rs)
  - [`/Users/example/Dev/projects/poodle/packages/gpui/node-backend/src/lib.rs`](/Users/example/Dev/projects/poodle/packages/gpui/node-backend/src/lib.rs)
  - [`/Users/example/Dev/projects/poodle/packages/render/src/editable_label.rs`](/Users/example/Dev/projects/poodle/packages/render/src/editable_label.rs)
  - [`/Users/example/Dev/projects/poodle/packages/gpui/preview/src/node_compat.rs`](/Users/example/Dev/projects/poodle/packages/gpui/preview/src/node_compat.rs)
  - [`/Users/example/Dev/projects/poodle/packages/gpui/preview/src/specimens/editable_label.rs`](/Users/example/Dev/projects/poodle/packages/gpui/preview/src/specimens/editable_label.rs)
  - [`/Users/example/Dev/projects/poodle/PAPERCUTS.md`](/Users/example/Dev/projects/poodle/PAPERCUTS.md)

## Boundaries

- Stay inside `g12.019` and the Poodle repository. Do not modify the sibling
  Jetstream repository; the eventual one-line pointer is the only permitted
  cross-repo change.
- Do not delete `/Users/example/Dev/projects/poodle/packages/gpui/components` until
  every in-scope specimen is migrated, the native gate has been run, and the
  deletion is logged.
- Do not update baselines, raise tolerance, or chase text/icon raster parity;
  the operator explicitly deferred that work. Every residual must remain
  named and explainable.
- Keep node vocabulary changes additive. `poodle-node` must not name GPUI or
  another backend.
- Preserve unrelated work in the dirty tree, especially the Agent Plan files
  and broad existing
  `/Users/example/Dev/projects/poodle/packages/render/src/lib.rs` module/export
  churn. Do not
  reset, checkout, or clean the workspace to make the audit easier.
- Follow [`/Users/example/Dev/projects/poodle/AGENTS.md`](/Users/example/Dev/projects/poodle/AGENTS.md): use Effigy where it covers the job, group changes into meaningful waves, use `apply_patch`, and run `git diff --check`.

## Important Context

- Planning lineage: this is `g12.019`, promoted from the Batch B handoff at
  [`/Users/example/Dev/projects/poodle/docs/logs/2026-08/05-225348-gpui-node-backend-batch-b-handoff.md`](/Users/example/Dev/projects/poodle/docs/logs/2026-08/05-225348-gpui-node-backend-b-handoff.md). The roadmap's Decision Log says the old GPUI tier is the recipe reference; reconcile shared render recipes to axis-faithful token behavior rather than re-baselining the old tier.
- Spec-to-canonical relationship: component contracts under
  `/Users/example/Dev/projects/poodle/docs/contracts/components/` and the shared
  specs crates define semantics; `poodle-render` emits Nodes; the GPUI backend
  owns dispatch, shaping, and platform mapping. The preview wrapper is only a
  migration seam, not a new public component tier.
- Non-obvious decisions:
  - User preference is to defer text-rendering parity and keep moving through
    components.
  - Node input editing is deliberately lightweight because a pure
    `Node -> AnyElement` backend cannot create a native GPUI Editor entity.
  - `EditableLabelHandlers` preserves the old renderer signature and adds an
    additive `editable_label_with_handlers` path.
  - The focused EditableLabel residual is expected; do not refresh its
    baseline.
- Parked contract shapes:
  - LogList's old `LogEntry` carries actor/resource/action audit metadata not
    represented by `LogListSpec`; do not discard that payload just to remove an
    import.
  - BlockEditor and DockRegion/SplitView accept consumer-owned GPUI elements;
    a backend-neutral slot or host bridge is needed before a faithful migration.
  - Tree needs rename/drag/context-menu host events and inline editor behavior.
  - IconProvider and UiPresentationProvider need a shared provider contract;
    utility/provider imports in app infrastructure should be audited separately
    from specimen constructor census.
- Audit caution: `rg '[A-Z]::from_spec'` counts node compatibility wrappers too.
  The meaningful audit is old-tier imports and the constructor census in the
  roadmap, not raw type names alone. The current working tree has roughly 307
  changed/untracked paths, many unrelated to this lane.
- Known validation friction is already recorded in
  [`/Users/example/Dev/projects/poodle/PAPERCUTS.md`](/Users/example/Dev/projects/poodle/PAPERCUTS.md): `effigy docs:check` aborts because
  `/Users/example/Dev/projects/poodle/packages/jetstream/components/Cargo.toml` is
  absent; `effigy doctor` also
  reports the unsupported `isolation` manifest key and existing scan findings.
  These are not reasons to alter the GPUI migration.

## Suggested Next Move

Start with an audit, not a reset:

1. Read this handoff, the canonical roadmap, the prior handoff, and
   [`/Users/example/Dev/projects/poodle/AGENTS.md`](/Users/example/Dev/projects/poodle/AGENTS.md); inspect `git status --short` and `git diff --check`.
2. Review the Wave 40 diff in the five key code files above. Confirm the new
   submit/cancel handlers are additive, disabled inputs stay inert, and the
   preview event queue is the only state bridge. Re-run the focused EditableLabel
   gate without updating its baseline.
3. Run the established checks: render tests, `effigy gpui:test`,
   `effigy gpui:build`, and `effigy drift:handlers`. Treat the docs-check
   missing-path failure as the known papercut unless the operator asks for it.
4. For the next migration, shape LogList's `LogEntry` metadata in the shared
   component spec first, then add the node wrapper and specimen migration. Run
   the focused native gate and record any residual without baseline changes.
5. Revisit BlockEditor/Tree/SplitView/DockRegion only after choosing an
   additive, backend-neutral slot/event shape. If that shape is not ready, keep
   the component parked and improve the contract rather than faking a GPUI-only
   node.

## Completion Protocol

1. Update [`/Users/example/Dev/projects/poodle/docs/roadmaps/g12/019-gpui-node-backend.md`](/Users/example/Dev/projects/poodle/docs/roadmaps/g12/019-gpui-node-backend.md) with the wave proof, constructor census, residuals, and next parked shape.
2. Keep [`/Users/example/Dev/projects/poodle/docs/roadmaps/generation-index.md`](/Users/example/Dev/projects/poodle/docs/roadmaps/generation-index.md), [`/Users/example/Dev/projects/poodle/docs/roadmaps/g12/README.md`](/Users/example/Dev/projects/poodle/docs/roadmaps/g12/README.md), [`/Users/example/Dev/projects/poodle/docs/roadmaps/README.md`](/Users/example/Dev/projects/poodle/docs/roadmaps/README.md), and [`/Users/example/Dev/projects/poodle/docs/contracts/001-working-rules.md`](/Users/example/Dev/projects/poodle/docs/contracts/001-working-rules.md) current.
3. Record the bounded batch in
   `/Users/example/Dev/projects/poodle/docs/logs/2026-08/` and leave one clear next
   task. No batch card currently exists under
   `/Users/example/Dev/projects/poodle/docs/roadmaps/g12/`.
4. Validation before handoff onward: `git diff --check`, render tests,
   `effigy gpui:test`, `effigy gpui:build`, `effigy drift:handlers`, and the
   native gate. Report the known docs/doctor failures honestly.
5. Do not mark Batch B complete until all in-scope preview specimens are
   node-backed and the gate evidence supports the closeout. Batch C then needs
   the deletion log, probe tests, and the permitted Jetstream pointer.
6. Leave unresolved risks explicit: lightweight input/IME behavior,
   EditableLabel `max_length`, parked slot/provider contracts, and the deferred
   text/icon raster bucket.
