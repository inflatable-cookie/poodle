# g17.001 — Nucleus V1 visual receipts

Status: ready for review
Date: 2026-09-09
Card: `docs/roadmaps/g17/001-nucleus-v1-visual-receipts.md`
Base: `origin/main` at `f622e2a8d2929f3cee13e5226cac9af485b1ae43`
Branch: `ns-e6426f01-9e5a-43dd-9c35-9437d3021919` (queue-owned)

## Outcome

The validated Poodle Lab cohort bundle is imported immutably, 29 V1 receipts
cover every Nucleus row, and the generated ledger moves all 29 receipt-backed
GPUI visual cells to `compared` with the 160 reported findings retained as
open evidence. No finding is adjudicated; no pixel, threshold, component, or
comparison policy changed.

## Imported bundle

Copied byte-for-byte from poodle-lab closeout
`13ddc2fcbc0897a9f2ec78ee0dd061ce74c7f46d` (merged Lab commit
`f99465f048d7c5c58603b99ae51f3209e581848e`) to
`docs/logs/2026-09/08-140648-g01-006-cohort-batch-bundle/` (`summary.json`,
`report.md`; per-file SHA-256 verified against the Lab source before and
after the copy). Citation, printed by the Lab validator before the copy and
recomputed from the Poodle copy by `v1DirectoryHash`:

```
poodle-lab bundle 0512b830e94bc30a7f1d2c623c6e081c85d4aa0086a60f9adfd905badc612a99, validator 1.0.0, run 2026-09-08T14-06-48
```

Form `in-repo` (`imageBytes: not-present`); `admission: none`. Contents:
174 captures (58 fixtures × 3 runtimes × 2 agreeing repeats, every
foreground verdict `proved`), 116 comparisons (58 fixtures ×
`svelte-react`/`svelte-gpui`), 160 findings, 48/116 pairs fully passing.
The bundle Poodle pin is `8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983`; its
policy table is byte-equal to the fixed g15.047 table in
`test/visual/button-comparison/policy.ts`.

## Receipts

`bun scripts/nucleus-parity-receipts.ts --write-v1` emitted 29
`docs/evidence/nucleus/nucleus-parity-receipts/*--v1.json` receipts
(`proof_level: "V1"`, `outcome: "compared"`). Each carries the bundle
identity above, its two row fixture ids (`cohort/<slug>/initial`,
`cohort/<slug>/after-actions`), the four pair verdicts verbatim
(including pixels `metrics`), and its finding count. Total retained
findings: 160. The closed `poodle.g16-nucleus-parity-receipt.v1` contract
grew only by the V1 branch: M1/A1 required keys, consts, and refusal
messages are unchanged, and the 58 pre-existing M1/A1 receipts validate
untouched (87/87).

## Ledger

`bun scripts/parity-evidence-ledger.ts --write` regenerated
`docs/evidence/nucleus/parity-evidence-ledger.md`. GPUI visual:
`compared` 1 → 29, `missing` 174 → 146. The 29 Nucleus execution rows are
`compared` on their V1 receipts; Button additionally retains its g15.047
inventory citation. Every V1 cell links the receipt and the bundle summary
and states findings are open evidence; no cell carries acceptance language.

## Oracle evidence

| Invariant | Counterexample | Result |
| --- | --- | --- |
| Bundle is validated, not trusted | tampered summary byte | `refuses a tampered bundle by its bytes before any receipt is derived`; Lab `validate` exit 0 on the source, Poodle dir-hash mismatch on the copy |
| Import is immutable and traceable | bundle without run id, closed batch, or Poodle pin | `refuses an import without Lab traceability` |
| Receipts map one-to-one | unknown, duplicate, or mismatched fixture identity | `refuses unknown, duplicate, and mismatched fixture identities`; 29 slugs biject to 29 scenario files and 29 manifest rows |
| Findings are not adjudicated | dropped pixel finding | `refuses a dropped finding: verdict counts no longer recompute`; `refuses V1 receipts with substituted bundle identity, outcome, or dropped findings` |
| Ledger moves only on evidence | `compared` cell without a V1 receipt | `rejects an unbacked compared GPUI visual cell` |
| Existing M1/A1 evidence survives | V1 regeneration rewrites an M1/A1 receipt | all prior receipt/ledger tests unchanged and green; committed V1 files byte-match re-derivation |

## Validation

- `effigy test:nucleus-parity-receipts` — 17 passed, 0 failed
- `effigy test:parity-evidence-ledger` — 9 passed, 0 failed
- `effigy check:parity-evidence-ledger` — 176 rows validated
- `effigy test:nucleus-a11y` — 31 passed; `effigy test:parity` — 551 passed
- `effigy docs:lint`, `docs:snippet-check`, `report:parity`,
  `report:accessibility` — green (after the documented
  `core:build`/`svelte:package`/`react:package` prerequisites)
- `git diff --check` — clean. No windowed selector was run. No merge performed.
