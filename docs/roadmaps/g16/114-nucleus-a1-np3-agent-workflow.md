# g16.114 — Nucleus A1 Tranche NP-3 Agent Workflow

Status: held — ready when `g16.111` merges
Type: A1 evidence tranche — paired accessibility receipts for the rows below
Opened: 2026-09-05
Depends on: merged `g16.111`
Governing refs: `111-nucleus-a1-accessibility-receipt-foundation.md` (A1
definition, snapshot shape, comparison law), `nucleus-gpui-parity-programme.md`,
`nucleus-parity-manifest.json`
Dispatch manifest: `../dispatch.md`

## Rows

AgentTranscript, AgentChatInput, AgentPlan, AgentQuestion, ModelPicker, StatusIndicator (6 rows)

## Fixed Boundary

- For each row: write the shared scenario file from the row's M1 scenario
  (same props, same actions), run both extractors, and emit one validated A1
  receipt. Rows already receipted by `g16.111` are skipped.
- A divergence is a finding, recorded in the receipt and the log with the
  exact attribute; it is not repaired in this card unless the repair is a
  missing node accessibility record value that `poodle-render` already has
  the input for (one-line projection fix), in which case fix it and say so.
- No contract change, no Svelte change, no backend behaviour change.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Every row has an executed receipt | a row with a scenario file but no run | ledger checker reports it missing |
| Findings are honest | a diff hidden by widening the exclusion list | reviewer checks each exclusion against `WEB_ONLY_PROPS` |
| Foundation unchanged | snapshot shape or comparison law edited | lane is red |

## Validation

`effigy regressions:native`, the `test/nucleus-a11y` project,
`effigy check:parity-evidence-ledger`, `effigy docs:check`,
`git diff --check origin/main...HEAD`.

## Owned Paths

`test/nucleus-a11y/scenarios/` files for these rows, the A1 regression tests
for these rows, their receipts under `nucleus-parity-receipts/`,
`packages/render/src/<row>.rs` only for a one-line projection fix, execution
log, `PAPERCUTS.md` (append only).

## Stop Conditions

Stop when a divergence needs a contract or backend decision. Escalation
owner: Chatterbox.
