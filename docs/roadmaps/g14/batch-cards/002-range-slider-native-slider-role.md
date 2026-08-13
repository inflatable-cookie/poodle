# 002 RangeSlider Native Slider-role Projection

Status: ready
Milestone: `g14.008` lane, pulled early — main's `ci:native` is red
Owner: Poodle core
Branch: `thread/g14-002-range-slider-native-role`
Depends on: none
Governing refs: `docs/contracts/components/range-slider.md` (role
requirement), `packages/svelte/preview/scripts/contract-role-drift.ts`,
`packages/render/src/range_slider.rs`, `packages/jetstream/preview/src/bin/a11y.rs`

## Goal

Make `docs:contract-role-drift` green on main. The gate fails today:
`range-slider` never projects the contract's `slider` role on the native
path — the Jetstream a11y census reports only
`GenericContainer`/`Label`/`Window` for it. The finding is recorded in the
`g13-b053` ledger row; this card fixes it on the shared `poodle-render`
path so GPUI and Jetstream both inherit the projection.

## Fixed By Ruling

- This is a native hole, not a web change. No Svelte/React file, no
  contract change — the contract already requires the role.
- The fix lands in `poodle-render`'s `range_slider.rs` (the shared native
  implementation), not in a per-runtime adapter. Precedent for the shape:
  `audio.rs` and `color_picker.rs` set `NodeRole::Slider` on their control
  nodes.
- The role must land on a node that already carries an accessible name.
  The existing `el.a11y.label` (range_slider.rs, the grab/control node) is
  the candidate.

## Deliverables

- The `slider` role projected on the appropriate control node in
  `packages/render/src/range_slider.rs`.
- A render test asserting the role is present on the node (precedent:
  `audio.rs` asserts `knob.a11y.role == Some(NodeRole::Slider)`).
- The gate proven green from a clean baseline: run
  `bun packages/svelte/preview/scripts/contract-role-drift.ts` before
  (exit 1, range-slider) and after (exit 0).
- The census re-run: `cargo run --bin a11y -- --json` in
  `packages/jetstream/preview` shows `Slider` for range-slider.

## Acceptance

- [ ] `effigy docs:contract-role-drift` exits 0.
- [ ] `effigy ci:native` exits 0 in the main checkout (not the worktree —
  the Jetstream sibling path-dep does not resolve there).
- [ ] No web file, contract, or adapter changed.
- [ ] The role projection is explainable in a11y terms, not just
  gate-pleasing: the node carrying `Slider` is the thing a screen reader
  should describe as the slider.

## Stop Conditions

- Placing the role raises a real semantics question (a dual-thumb control
  as a single `slider` role). Report the options with tradeoffs; do not
  choose silently.
- The role's node has no accessible name, and attaching both cleanly
  requires touching the adapter layer — report, do not reach.

## Writable Paths

- `packages/render/src/range_slider.rs`
- `packages/render/src/range_slider.rs` tests (same file, `#[cfg(test)]`)
- `docs/logs/2026-08/14-g14-002-range-slider-native-role.md`
- `PAPERCUTS.md` (append only)

## Steps

1. Reset per the Thread Reuse Protocol; confirm the gate is red on the
   fresh branch (baseline).
2. Read the contract's role section and the three precedents
   (`audio.rs`, `color_picker.rs`, `range_slider.rs`'s label site).
3. Project the role; add the render test; run `ci:rust` and
   `docs:contract-role-drift`.
4. Census check via `cargo run --bin a11y -- --json` in the jetstream
   preview.
5. Full validation: `effigy ci:rust`, `effigy docs:contract-role-drift`,
   `effigy test:core`, `effigy test:components`, `git diff --check`.
   (`ci:native` runs at review in the main checkout.)
6. Write the batch log; push with
   `git push -u origin thread/g14-002-range-slider-native-role`. Do not
   merge.
