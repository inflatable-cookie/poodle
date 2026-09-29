# Evidence

Generated or validated evidence that repository scripts read and check. It
records what was observed; it does not replace contracts, architecture or
knowledge. GPUI census evidence is regenerated with its script and never
edited by hand. Nucleus receipts are frozen history: validate them, do not
rewrite them.

- [`gpui/`](gpui/gpui-functionality-census.md) — contract-bound GPUI
  functionality census, capability manifest, expected-test execution record and
  mounted receipts (`scripts/gpui-functionality-census.ts`).
- [`nucleus/`](nucleus/README.md) — frozen Nucleus cohort, receipt schemas,
  M1/A1/V1 receipts, the imported V1 lab bundle and the generated parity ledger
  (`scripts/nucleus-parity-receipts.ts`, `scripts/parity-evidence-ledger.ts`).
- [`visual/`](visual/g15-047-button-comparison/summary.json) — retained g15.047
  Button comparison run cited by the parity ledger and parity reports. Frozen.
- [`releases/`](releases/README.md) — release evidence still consumed by local
  package checks.
