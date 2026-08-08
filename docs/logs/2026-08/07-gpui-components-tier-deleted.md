---
title: The duplicate GPUI component tier is deleted
status: complete
owner: Poodle core
updated: 2026-08-07
tags: [log, g12.019, gpui, batch-c, deletion]
---

## What Was Deleted

`packages/gpui/components` — **170 files, 44,796 lines** — the hand-written GPUI
implementation of the component set, plus `packages/gpui/preview/src/demo_view.rs`,
dead source that had not compiled since an earlier refactor removed the
`app_state::DemoScreen` type it needed.

This closes `g12.019` Batch C and the renderer inversion. Poodle now has one
component implementation (`poodle-render`, emitting `poodle-node` trees) behind
thin per-target backends. No target carries its own copy of a component recipe.

## Preconditions Met Before Deleting

The roadmap gated this on the preview running on the new backend with the visual
gate run, not on a green-everything:

- Every preview specimen and every app-infrastructure file renders through
  `poodle-render` plus `poodle-gpui-node-backend`. The final old-tier import
  census was zero.
- The full native visual gate ran over all 136 components: **96 exact, 39
  failing**, and every one of the 39 is a named, documented residual or a slug
  deliberately excluded from the baseline refresh (see
  `07-gpui-node-backend-waves-41-45.md` and the roadmap's residual bucket).
- Probe tests were mined first (below).
- `effigy check:gpui`, render tests (109), spec tests (227), adapter tests
  (133), `effigy drift:handlers`, and `git diff --check` are green.

## Probes Mined Before Deletion

Only two files in the old tier had tests, and one was a false lead:
`primitives/stepper.rs` carries a comment explaining that a `#[test]` there hits
a `recursion limit reached while expanding #[test]` compiler bug — which is why
`check:gpui` was only a `cargo check` — and noting the logic lives in
`poodle-specs` where it is already tested.

The real find was `presentation/metrics_c.rs`: 20 tests over the Svelte-derived
sizing ladders. Meanwhile `packages/render/src/presentation.rs` had **56 public
functions and zero tests**, despite now being the single module every backend
resolves sizing through. The 15 applicable tests were ported there — semantic
size roles and their clamps, control heights, height offsets, font sizes, min
widths, density ladders, EditableList scales and `rem_to_px`. They pass
unmodified, which independently confirms the shared module reproduces the old
tier's values exactly. (`poodle-render`: 98 → 109 tests. The `duration_*`
helpers live outside that module and were not applicable.)

## Consequential Changes

- `packages/gpui/preview/Cargo.toml` — dependency dropped.
- `tasks/effigy.tasks.toml` — `check:gpui` no longer checks the deleted crate.
  Its comment claimed "GPUI has no unit tests — its components need a live
  window", which stopped being true: the components are now CPU-testable
  `poodle-render` recipes. The task now also runs the node backend's tests, and
  the comment says where the real coverage is.
- `packages/gpui/preview/scripts/dead-handler-drift.ts` — the handler-drift gate
  scanned the deleted tier, so it broke outright. Its rule ("a component that
  accepts a handler must use it") still matters, so it was repointed at the two
  surviving surfaces rather than dropped: `poodle-render`, where handlers are
  `*Handlers` struct fields read as `handlers.on_x`, and the preview's
  `node_compat.rs`, whose builders store them on `self`. Counting had to handle
  both shapes plus `poodle-render`'s plain function-parameter handlers, so a
  read is now any mention not followed by `:`, minus assignments. Verified it
  still has teeth: a deliberately unwired handler is caught, and removing it
  goes green again. Baseline stays empty.
- `packages/release-manifest.json`, `packages/release-operations.json`,
  `packages/ecosystem-acceptance.json` — the crate removed from the package
  lists and coverage sets, since `docs:lint` validates release operations
  against real manifests.
- `packages/gpui/adapter/README.md` — "What This Crate Does NOT Own" pointed at
  the deleted crate; now points at `poodle-render` and the node backend.
- The two `*parity-report.json` files under `packages/` still name the crate.
  They are generated evidence artifacts recording a past run, and were left as
  the historical record. The g08/g09 roadmap cards likewise stay as written.

## Cross-Repo

One line, the only permitted change: `g06.013` in the Jetstream repo notes that
Poodle's last duplicate tier is gone. Nothing else in that repo was touched.

## Follow-Ups, Since Closed

- **Tree** is now exact. Its two divergences resolved opposite ways, both
  settled by reading the contract rather than by preference. The focus ring:
  the tree contract §"Roving tabindex" states plainly that "the Rust runtimes
  track it via `focused_value` on the spec … and render a focus ring on that
  node", so `poodle-render` was already right and the *old tier* was the
  deviation — no focus-visible vocabulary channel was needed after all. The
  guide lines: a real defect, where the row's always-on 1px border inset its
  content box, leaving every indent cell 2px short (broken guides) and every
  row 2px tall (a cumulative vertical offset). The contract draws that ring as
  an `outline`, which does not participate in layout, so it became an
  absolutely-inset overlay — the same shape as the SidebarNav fix. Baseline
  refreshed for those two reasons.
- **`log-list`** refreshed: its baseline captured the literal placeholder
  `"5 entries"` that Wave 45 replaced with real rows.
- **`check:jetstream` / `test:jetstream`** pruned. Their crate was deleted in
  `ee704699`; the render probes they ran live in `cargo test -p poodle-render`,
  which `ci:rust` already runs.
- **`drift:clicks` retired.** Same orphaning: it proved a real pointer gesture
  reached every Jetstream builder, but its subject and its `click_probe` tests
  went with `poodle-jetstream-components`, and its replacement lives in the
  sibling repo, outside this repo's boundary. GPUI's `drift:handlers` survives
  and now covers the shared `poodle-render` handlers both targets render.

## Not Done

- **`block-editor` retains 0.0602%**, and it is not the chrome: the diff sits in
  BlockEditor's own per-block toolbars (the TypeSelect labels, `+`, `Select…`),
  shifted a few px. That is the deferred text-raster bucket, so it stays named
  rather than refreshed.
- **151 unnamed `TextInput` nodes** now fail `effigy test:jetstream-a11y`.
  `poodle_render::text_input` names its root only when `spec.aria_label` is set,
  and 13 specimens do not set one. Pre-existing and previously invisible —
  `ci:native` died at `drift:clicks` long before reaching the audit, so the
  count silently regressed from zero. Pruning the dead tasks unmasked it, and it
  is now the last thing between `ci:native` and green. Recorded in
  `PAPERCUTS.md`; a11y work is held under `g12.015`.
- One consumer of the LogList spec change lived outside the GPUI lane:
  `packages/jetstream/preview` used `with_entry_count`. It now builds real
  stream entries, so both natives render rows.

## Post-Deletion Fix: Node-Backed Clicks Were Dead Across Frames

Reported by the operator from the running preview: the sidebar's component list
did nothing on click, while the catalogue's own buttons worked.

The cause was in the backend, not the sidebar. `element_id` falls back to
`poodle-node-{n}` from a global `AtomicU64` for any node that declares no id —
and that counter was monotonic and never reset, so every generated id was new
on every frame. gpui keeps a click's `pending_mouse_down` in the element state
it keys by `ElementId` (`gpui-0.2.2/src/elements/div.rs:2124`): the press stores
it, the release reads it back. A real click spans many frames, so a node whose
id changes in between has its release land on fresh state with nothing pending,
and the click is dropped silently. Components survived only where something
assigned a stable id — the `node_compat` wrappers (`poodle-btn-*`,
`poodle-input-*`) and the recipes that set their own (`tree:{value}`,
`dock-tab-{value}`).

Fixed at the root: `poodle_gpui_node_backend::reset_element_ids()`, called once
per frame at the top of `PreviewRoot::render`. Ids are then assigned in
deterministic tree order and are stable between frames for a stable tree, which
is what the fallback's own comment always claimed. `sidebar_nav` additionally
carries an explicit `sidebar-nav-{value}` id, matching the old tier.

**Neither existing gate could have caught this**, which is the part worth
keeping: the visual gate compares static frames, and the in-process click driver
posts press and release within a single frame, so it never crosses a rebuild. It
passed identically with the bug present and absent — verified by reverting the
fix and re-running the same click. That means the click-driver proofs recorded
through Waves 1–46 attest that a handler is reachable, not that a human click
reaches it. Recorded in `PAPERCUTS.md`.

