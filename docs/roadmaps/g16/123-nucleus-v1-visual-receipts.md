# g16.123 — Nucleus V1 Visual Receipts

Status: held — ready when poodle-lab `g01.006` has produced its first
validated cohort bundle
Type: evidence consumption — V1 receipts and ledger column
Opened: 2026-09-05
Depends on: merged `g16.122`; poodle-lab `g01.006` bundle
Governing refs: `nucleus-gpui-parity-programme.md` (V1: deterministic
component comparison for Nucleus-used states, Poodle-owned),
`docs/logs/2026-09/…-g16-051-…` (how an imported lab bundle is cited),
poodle-lab `docs/contracts/004-receipt-import.md`, `scripts/nucleus-parity-receipts.ts`
Dispatch manifest: `../dispatch.md`

## Goal

Turn a validated lab cohort bundle into per-row V1 receipts and move the
"GPUI visual" ledger column from `missing` to `compared` for each row whose
web↔GPUI comparison passes under the g15.047 tolerance table, with findings
recorded per row for those that do not.

## Fixed Boundary

- Import the sanitized bundle by directory hash and validator version under
  the execution log (the `051` shape). Never edit it.
- Extend the receipt schema with `proof_level: "V1"` carrying the bundle
  hash, the row's fixture ids, the pair verdicts, and the findings. Emit a
  V1 receipt only for rows the bundle covers; findings do not block the
  receipt, they are recorded in it.
- Ledger: a validated V1 receipt moves the row's "GPUI visual" cell to
  `compared`; a row with findings stays `compared` with the findings listed
  in the known-delta axis if they are contracted, or as open findings if not.
- Poodle decides nothing about pixels here; the tolerance table is the
  g15.047 authority and does not change in this card.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Bundle is validated, not trusted | tampered PNG hash | lab validator exit 1 recorded |
| Receipts trace to fixtures | a receipt naming a fixture not in the bundle | validation fails |
| Ledger moves only on receipts | a `compared` cell without a receipt | ledger check fails |

## Validation

`effigy check:parity-evidence-ledger`, `effigy docs:check`, `git diff
--check origin/main...HEAD`.

## Owned Paths

`scripts/nucleus-parity-receipts.ts` and schema (V1 level), receipts,
ledger generator V1 path, execution log with the imported bundle,
`PAPERCUTS.md` (append).

## Stop Conditions

Stop if the bundle's fixture ids do not map to scenario ids one to one.
Escalation owner: Chatterbox.
