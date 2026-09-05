# g16.115 — Nucleus A1 NP-4 execution log

Status: blocked at bounded divergence review
Date: 2026-09-05
Base: `origin/main` revision 18 at `7c1837f0fa2fede8fbd476b3362d88dd112290d7`
Branch: `worker/g16-115-nucleus-np4`
During execution, concurrent lane activity advanced `origin/main` to
`f9922fd1d558ae34f8888e524366791f79f942cb`; a fresh rebase is required before
any eventual PR.

## Scope

Added shared A1 scenarios and Svelte snapshots for RadioGroup, TextInput,
Callout, ConfirmAction, and DetailItem. Added mounted GPUI A1 probes for all
five rows. Switch remains the existing A1 receipt.

## Findings

- RadioGroup diverges on `orientation` and focus-order semantics. Evidence is
  under `docs/roadmaps/g16/nucleus-parity-receipts/a1-divergences/radio-group/`.
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

- `test:nucleus-a11y`: previously green for the five scenarios plus the three
  foundation rows before native divergent rows were added.
- Focused native TextInput and Callout probes passed before this continuation.
- ConfirmAction and DetailItem native probes executed and published the
  divergence snapshots/diffs above.
- `effigy docs:check` / parity ledger remains blocked by the unrepinned dirty
  runtime source; final revision-18 cohort repin was not performed.
- `git diff --check`: clean.

## Stop

Chatterbox must create bounded repair decisions for the ConfirmAction and
DetailItem projection gaps (RadioGroup is already the lane's requested repair
card input). Do not repin the cohort or publish a PR until those decisions are
resolved, because the exact-head receipt ledger would otherwise be red.
