# Nucleus Parity Evidence

Status: active evidence
Programme: [`g17`](../../roadmaps/g17/README.md)

## Boundary

The fixed denominator is the 29 Poodle components rendered by Nucleus.
`IconProvider` is a mandatory construction prerequisite but not a rendered row.
Poodle owns reusable component evidence; Nucleus owns application data,
callbacks, journeys, and adoption; the lab and platform accessibility authority
own V2 and A2.

## Evidence levels

| Level | Meaning | Authority |
| --- | --- | --- |
| M1 | production GPUI path mounted and driven through real input | Poodle |
| M2 | actual Nucleus journey with Nucleus data and callbacks | Nucleus |
| A1 | mounted GPUI node-tree semantics equal mounted Svelte DOM ARIA for the same scenario | Poodle |
| A2 | independent platform accessibility tree | external authority |
| V1 | deterministic component comparison for Nucleus-used states | Poodle |
| V2 | actual Nucleus or Nucleus-owned harness captured in the lab | lab + Nucleus |

M1 and A1 are complete 29/29. V1 is the next active task. Each row needs
M1+A1+A2+V1 before the switch packet; the composed target also needs M2+V2.
Missing external evidence blocks the switch decision, not unrelated Poodle
work.

## Artifacts

- `nucleus-parity-manifest.json` — fixed cohort and exact dependency identity.
- `nucleus-parity-receipt.schema.json` — closed M1/A1 receipt contract.
- `nucleus-parity-receipts/` — validated M1 and A1 receipts.
- `parity-evidence-ledger.md` — generated component evidence denominator.

Generators and checks live in `scripts/nucleus-parity-receipts.ts` and
`scripts/parity-evidence-ledger.ts`. A source import, test name, or route is not
execution evidence; only validated receipts advance the ledger.

