# Canonical Dispatch Manifest

Status: active
Owner: Chatterbox (planning authority) — the only writer
Consumer: the coordinator, which launches every ready lane listed here and
designs no lanes, edges, or concurrency of its own
Updated: 2026-09-04 (revision 7: urgent 103 release verifier repair)
Promoted commit: the commit that last touched this file
(`git log -1 --format=%H -- docs/roadmaps/dispatch.md`); the coordinator
verifies it is an ancestor of current `origin/main` before dispatch

Each ready lane below names its card, readiness, prerequisites and completion
conditions, owned mutable paths, reserved shared closeout surfaces, approved
concurrent siblings and serial edges, worker capability class, acceptance
evidence and review oracle, stop conditions, and escalation owner. The card
file is the complete worker handoff body; this manifest is the frontier.

Lanes not listed under **Ready frontier** are not dispatchable, whatever a
card or triage note says.

## Ready frontier

### g16.103 — Release tarball dist verification repair

- Card: `g16/103-release-tarball-dist-verification.md`
- Readiness: ready — highest priority. Explicit workflow-edit approval recorded
  2026-09-04. Completion: PR merged after accepted exact-head review; targeted
  verifier proofs pass; automatic push-to-`main` web and Rust boards pass.
- Prerequisites: none. Blocks `g16.097` re-certification.
- Owned mutable paths: `.github/workflows/release.yml` (`Pack and verify
  contents` only), `scripts/check-release-automation.ts`, one
  `docs/logs/2026-09/` log, root `PAPERCUTS.md` append-only.
- Reserved shared closeout surfaces: `g16/README.md`, `generation-index.md`,
  this manifest, and the `g16.097` candidate-state update.
- Approved concurrent siblings: none required. Serial edge: `g16.103` merge and
  green main boards before `g16.097` restarts.
- Worker capability class: capable coding model, medium reasoning.
- Acceptance evidence and review oracle: the card's Validation and Review
  Oracle sections. The PR-head `ci-web` workflow-scope rejection is accepted
  only as documented; every other failure blocks.
- Stop conditions: per card. Escalation owner: Chatterbox.

### g16.097 — v0.3.0 release certification (coordinator-executed)

- Card: `g16/097-v030-release-certification.md`
- Readiness: ready, serially blocked on merged `g16.103` and its green
  push-to-`main` boards. Release authority remains the operator's 2026-09-04
  authorization. Never dispatched to a worker.
- Prerequisites: merged `g16.054`, `g16.098`, and `g16.103`. Completion:
  re-certify the repaired `main` tip, tag that exact SHA, run green dry-run then
  publish, prove npm `latest` 0.3.0 for core and Svelte, and close out.
- Owned mutable paths: tag `v0.3.0`; `docs/logs/2026-09/` log; this card;
  `CHANGELOG.md`; `docs/release-notes/0.3.0.md`; README status paragraphs.
- Reserved shared closeout surfaces: `g16/README.md`, `generation-index.md`,
  and this manifest.
- Approved concurrent siblings: none required. Serial edge: `g16.103` first.
- Worker capability class: none — coordinator action.
- Acceptance evidence and review oracle: the card's Acceptance checklist and
  Review Oracle table.
- Stop conditions: per card. Escalation owner: operator, via Chatterbox.

### g16.104 — Release workflow checkout base ref and pre-tag dry run

- Card: `g16/104-release-workflow-checkout-base-ref.md`
- Readiness: ready, priority — operator approved the `release.yml` edit and
  the second `v0.3.0` retraction on 2026-09-04
- Prerequisites: none. Completion: PR merged after accepted exact-head
  review; `check:release-automation` green; the card's own PR shows the
  g16.096 exception behaviour on `ci-web` (workflow diff rejected by the
  scope guard) and green `ci-rust`, per the accepted rule
- Owned mutable paths: `.github/workflows/release.yml`,
  `scripts/check-release-automation.ts` (assertion extension only), log,
  `PAPERCUTS.md` (append only)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: none needed; nothing else is ready. Serial
  edge: `g16.097` waits for this merge.
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's Review Oracle table;
  the branch dry run is executed by `g16.097` step 1b, not by this lane
- Stop conditions: per card. Escalation owner: operator, via Chatterbox

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| --- | --- | --- |
| Loophole adoption of `0.3.0` | proven npm `latest` `0.3.0` (`g16.097` step 5); Loophole-owned planning | Loophole planning, via Chatterbox |
| `g16.051` icon geometry native visual admission | accepted VL-1 Button bootstrap and VL-2A icon adapter in `poodle-lab` | Poodle Chatterbox (lab planning) |
| VL-1 Button MVP bootstrap | compiled in the `poodle-lab` repository's own dispatch surfaces, not this manifest | Poodle Chatterbox + coordinator |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Nucleus A1 / V1 / V2 / M2 and Nucleus adoption | next programme boundary not yet chosen | operator, via Chatterbox |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
