# g16.115 — Nucleus A1 NP-4 execution log

Status: complete — ready for independent exact-head review
Date: 2026-09-05
Base: `origin/main` at `7c720368e13ce16e367d7c8ee1a6863287c8a1d0`
Branch: `worker/g16-115-nucleus-np4`
Runtime checkpoint: `3f2d0a0d43d55e3f7cbcfffd5e8ce716bf980bc5`

## Scope

Added shared A1 scenarios and Svelte snapshots for RadioGroup, TextInput,
Callout, ConfirmAction, and DetailItem. Added mounted GPUI A1 probes for all
five rows. Switch remains the existing A1 receipt.

## Findings

- RadioGroup diverges on `orientation` at node index 0 and `focus_order` at
  node index 2. The exact current GPUI snapshot and two-entry diff are under
  `docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/radio-group/`;
  the paired Svelte snapshot is
  `test/nucleus-a11y/snapshots/radio-group.svelte.json`. The stale duplicate
  `radio-group-run.*` artifact was removed.
- ConfirmAction diverges on the mounted dialog projection: backdrop, dialog
  naming/relationships, close affordance, and action ordering. Evidence is
  under `.../a1-divergences/confirm-action/`.
- DetailItem diverges because the Svelte description action opens a Popover
  dialog while the current GPUI renderer exposes only the action button.
  Evidence is under `.../a1-divergences/detail-item/`.

These are real semantic/component-shape findings, not missing scalar values
available for a one-line projection repair. No A1 receipt or mounted cell was
created for the three divergent rows.

## Validation

- `effigy regressions:native`: 208 passed, 8 ignored: 5 inherited from
  `origin/main` and 3 NP-4 divergence probes (RadioGroup, ConfirmAction, and
  DetailItem).
- `effigy test:nucleus-a11y`: 14 passed.
- `effigy test:nucleus-parity-receipts`: 11 passed.
- `effigy check:parity-evidence-ledger`: 176 component rows validated.
- `effigy docs:check`: passed.
- `git diff --check`: clean.
- Full 34-receipt cohort re-emitted at `3f2d0a0d`; manifest resolution was
  repinned to that source commit and the existing lock digest.

## Stop

The branch is ready for independent exact-head review. Do not merge this PR or
run windowed selectors from the worker lane.
