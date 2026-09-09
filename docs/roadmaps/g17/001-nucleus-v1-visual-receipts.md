# 001 — Nucleus V1 visual receipts

Status: ready
Owner: Poodle core
Created: 2026-09-05
Updated: 2026-09-09
Governing refs: `../../evidence/nucleus/README.md`,
`../../contracts/001-working-rules.md`, poodle-lab
`docs/contracts/004-receipt-import.md`
Depends on: `g17.004`; poodle-lab `g01.006` complete

## Outcome

Import the validated Poodle Lab cohort bundle into immutable Poodle evidence,
emit traceable V1 receipts for all covered Nucleus rows, and move the generated
ledger's GPUI visual cells to `compared` without adjudicating renderer findings.

## Ready-State Rubric

- [x] Objective is bounded and needs no fresh planning decision.
- [x] Governing refs point at current canonical surfaces.
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] The high-risk import and ledger claims have an adversarial review oracle.
- [x] No automatic successor is enabled; Chatterbox resumes after closeout.
- [x] The poodle-lab prerequisite is complete and its closed bundle is available.

## Decisions

- Import poodle-lab run `2026-09-08T14-06-48` from merged Lab commit
  `f99465f048d7c5c58603b99ae51f3209e581848e` and closeout commit
  `13ddc2fcbc0897a9f2ec78ee0dd061ce74c7f46d`.
- Findings remain evidence. They neither block V1 receipt emission nor become
  accepted deltas.
- The g15.047 tolerance table remains fixed; this task does not change pixels,
  thresholds, component behavior, or comparison policy.

## Dispatch manifest

- **State:** ready; one serial lane; no concurrent sibling or automatic successor.
- **Completion:** imported bundle validates by directory hash and validator
  version; every covered row has a valid V1 receipt; the generated ledger moves
  only receipt-backed GPUI visual cells to `compared`; findings remain traceable.
- **Owned mutable paths:** `scripts/nucleus-parity-receipts.ts`, its tests and
  V1 schema extension, `docs/evidence/nucleus/`, one immutable imported bundle
  under `docs/logs/2026-09/`, one execution log, `PAPERCUTS.md` append-only.
- **Reserved closeout surfaces:** `docs/roadmaps/g17/README.md`,
  `docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`, this task's
  final status/evidence block.
- **Worker:** general implementation; exact evidence and schema work.
- **Excluded:** Poodle Lab, Nucleus, Longhorn, component/pixel repair, tolerance
  changes, V2/M2/A2, release work, windowed capture, and finding adjudication.
- **Escalation:** Chatterbox owns scenario/fixture mapping or evidence-policy
  decisions; operator owns any scope expansion.

## Work

1. Import the sanitized Lab bundle immutably and record its source commit,
   directory hash, validator version, run id, capture count, and comparison count.
2. Extend the closed Nucleus receipt schema and generator with
   `proof_level: "V1"`, bundle identity, row fixture ids, pair verdicts, and
   findings.
3. Emit V1 receipts only for rows covered by validated fixtures. Refuse unknown,
   duplicate, missing, or mismatched scenario/fixture identities.
4. Regenerate the parity ledger so only validated V1 receipts move GPUI visual
   cells to `compared`; retain findings as open or contract-linked evidence.
5. Add focused tamper, mapping, and unbacked-ledger regressions. Record exact
   validation and evidence in the execution log.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Bundle is validated, not trusted | tampered PNG or summary hash | validator exits non-zero; no receipt emitted |
| Import is immutable and traceable | copied files without Lab commit, run id, validator, or directory hash | schema/check refuses the import |
| Receipts map one-to-one | receipt names a fixture absent from the bundle, duplicates a state, or maps the wrong scenario | generator/check fails closed |
| Findings are not adjudicated | reported pixel/role finding silently dropped or converted to accepted delta | receipt retains it; ledger links it without acceptance language |
| Ledger moves only on evidence | `compared` cell without a valid V1 receipt | ledger check fails |
| Existing M1/A1 evidence survives | V1 regeneration rewrites or invalidates an M1/A1 receipt | receipt and ledger tests fail |

## Stop conditions

- Stop if Lab fixture ids do not map one-to-one to the Poodle scenarios.
- Stop if the bundle does not validate byte-for-byte from the named Lab commit.
- Stop if a V1 schema change would weaken M1/A1 validation or require a
  compatibility alias.
- Stop rather than adjudicating any of the 160 reported findings.

## Evidence

Prerequisite evidence: Lab run `2026-09-08T14-06-48`; 174 captures covering
58 fixtures × 3 runtimes with two agreeing repeats; 116 comparisons; 160
reported findings; every foreground proof valid. Final evidence must add the PR,
reviewed exact head, merge commit, imported bundle hash, emitted receipt count,
ledger result, and validation actually run.

## Next task

Return to Chatterbox after closeout. The generation runway names V2, M2, A2,
and the operator switch packet, but none is automatically dispatchable.
