# g16.122 — Window-capture cohort fixture kind

Status: implementation complete — pending independent exact-head review
Date: 2026-09-05
Card: `docs/roadmaps/g16/122-window-capture-cohort-fixtures.md`
Base: `origin/main` at `a1c58f14389cd8cdd858fbc4264dc8e2ffdca57a` (manifest revision 21)
Implementation source pin: `3c0a4273ba92d7eb70657d1c4ae48a01047a9ec5`
Lock digest: `c86c2d11c36c9fcf9326bae438ee6acc3bcedacbaf01ac017a298c1bd3c2a34c`

## Outcome

Added the closed `cohort` capture kind to `poodle-window-capture`. It accepts
one of the 29 canonical scenario ids and either `initial` or `after-actions`,
rejecting unknown ids, states, flags, and malformed scenario files before a
window is created. Both states render the shared scenario through
`poodle_render` and the GPUI node backend; `after-actions` posts the shared
scenario actions through the live window event queue.

Every scenario now declares a positive logical `capture` viewport. The TypeScript
and Rust scenario readers require the block, reject extra capture keys, and
validate positive integer dimensions. The A1 snapshots and 29 M1/25 A1 receipts
were repinned because the scenario file bytes changed.

The cohort receipt is `poodle.cohort-visual-capture.v1` and carries the scenario
id/state/hash, Poodle source identity, viewport, PNG hash, and the existing
non-activating transport, focus, foreground, and permission evidence. The
activation-boundary scan now covers the new module too.

## Validation

- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin poodle-window-capture --features window-capture` — 61 passed
- `effigy regressions:native` — 233 passed
- `effigy test:nucleus-a11y` — 30 passed
- `effigy test:nucleus-parity-receipts` — 11 passed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy docs:check` — passed; existing informational Svelte diagnostics and ratcheted value-domain findings remain
- `effigy ci:rust` — passed
- `effigy ci:web` — passed; 386 test files and 3,740 tests, plus the package-install consumer checks
- `git diff --check` — clean before final rebase; rerun after rebase

`effigy check:gpui` reached the unchanged `poodle-render` suite but remains red
on the two known `origin/main` failures already recorded in `PAPERCUTS.md`:
`context::tests::the_provider_adds_no_wrapper_node_layout_or_accessibility_entry`
and `segmented_control::tests::icon_only_without_an_icon_keeps_the_visible_label`.
No unrelated repair was attempted.

No windowed selector was run. No merge was performed. The worker will push one
PR and stop for independent exact-head review.
