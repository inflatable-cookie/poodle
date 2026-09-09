# g18 GPUI planning-readiness repair

Status: complete
Date: 2026-09-09
Mode: Northstar planning readiness review and Chatterbox promotion

## Verdict

Lifecycle state was `drifted`. g17 had completed every approved task, but its
empty frontier was incorrectly read as project completion even though current
evidence still reported 146/175 portable GPUI components without validated
mounted-behaviour receipts. Planning was not up to scratch: the active roadmap
carried broad horizons but no executable path for full GPUI functionality.

## Evidence

- `docs/evidence/nucleus/parity-evidence-ledger.md`: 176 public components, 175
  portable native components, 175/175 constructing routes, 29 receipt-backed
  mounted rows, and 146 missing rows.
- `scripts/parity-evidence-ledger.ts`: 57 components in the historical mounted
  expectation map; after the 29-row Nucleus overlap, 44 missing rows cite
  expected tests and 102 have no admitted receipt or expected entry.
- `packages/gpui/cross-runtime-parity-report.json`: construction is explicitly
  not a 175-component behaviour pass; broad accessibility remains manual.
- Current triage retains upstream A2, composition, consumer, repository, and
  Jetstream gates but held no full-catalogue GPUI completion runway.

## Promotion

- Compacted completed g17 to `docs/roadmaps/archive/g17.md`.
- Opened g18 with full portable GPUI functional completion as its outcome.
- Approved `g18.001` as the sole queue frontier: build a contract-bound,
  capability-level census and admit only executed mounted evidence.
- Preserved Nucleus V2/M2/A2 and switch work as external horizons.
- Corrected the stale Nucleus V1 status, triage index, completed `0.3.0`
  consumer disposition, and `gpui-unofficial` next check.
- Narrowed the native accessibility hold: A2 remains blocked, but component-
  level semantics, keyboard, and focus work below the platform tree continues.

No product code changed and no implementation was dispatched. The accepted
census must return to Chatterbox before the first repair tranche is compiled.

## Validation

- `effigy docs:check`
- `git diff --check`
