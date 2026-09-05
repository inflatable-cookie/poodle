# g16.114 — Nucleus A1 Tranche NP-3 Agent Workflow

Status: complete — merged in PR #216 at `a3c2b1b62`
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

## Runtime Identity (rule, 2026-09-05)

The receipt checker binds every receipt to `manifest.resolution.source_commit`
and its lock digest, and verifies the runtime source paths are unchanged since
that commit. A lane that changes any runtime source path (this includes
`packages/gpui/preview` A1 tests and extractor edits) therefore MUST, at its
final exact head after rebasing onto `main`: repin `resolution.source_commit`
and the lock digest, re-emit the entire Nucleus cohort (all M1 receipts and
every A1 receipt already on `main`) through the real selectors, and validate
the cohort. This is the `g16.105`/`106`/`111` practice, not a scope widening.
Manifest edits are limited to the resolution block; the 29-row cohort and
scenario ids never change. When several tranches are open, the coordinator
merges them one at a time and each later head re-emits at its rebase; the
reviewer checks the cohort validates at the exact merged head. The lane's
PR also commits the regenerated parity ledger
(`docs/roadmaps/g16/parity-evidence-ledger.md`) at that head; the ledger is
generated evidence, not a coordinator-reserved closeout surface.

A row whose paired snapshots diverge on real semantics is recorded (diff,
both snapshots, exact attributes) with no receipt and no `mounted` cell,
exactly as `g16.111` did for Select. If the cause is a missing value that
`poodle-render` already has the input for, the one-line projection fix is in
scope; anything else becomes a bounded repair card (`g16.117` shape).

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
