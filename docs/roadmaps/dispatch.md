# Canonical Dispatch Manifest

Status: active
Owner: Chatterbox (planning authority) — the only writer
Consumer: the coordinator, which launches every ready lane listed here and
designs no lanes, edges, or concurrency of its own
Updated: 2026-09-04 (revision 2: release certification lane)
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

### g16.095 — Svelte↔React public prop drift gate

- Card: `g16/095-react-prop-drift-gate.md`
- Readiness: ready
- Prerequisites: none. Completion: PR merged after accepted exact-head review;
  execution log written; grouped finding set recorded in the log.
- Owned mutable paths: `packages/svelte/preview/scripts/react-prop-drift.ts`,
  `packages/svelte/preview/test/react-prop-drift.test.ts`, `tasks/effigy.tasks.toml` (new `docs:react-prop-drift`
  selector and the `docs:check` sequence only), `docs/logs/2026-09/` log,
  root `PAPERCUTS.md` (append only)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: `g16.096`. Serial edges: none.
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's Review Oracle table;
  biting counterexamples committed before the script
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.096 — Linux headless PR and main board

- Card: `g16/096-linux-headless-pr-board.md`
- Readiness: ready — workflow-edit authority is the operator's explicit
  2026-09-02 approval recorded in the card
- Prerequisites: none. Completion: PR merged after accepted exact-head review
  with both workflow runs green on the PR head; execution log records run URLs.
- Owned mutable paths: `.github/workflows/ci-web.yml`,
  `.github/workflows/ci-rust.yml`, `scripts/check-release-automation.ts`
  (per-workflow trigger assertions), `docs/logs/2026-09/` log, root `PAPERCUTS.md`
  (append only)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: `g16.095`. Serial edges: none.
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's Review Oracle table; the
  PR's own workflow runs are the executable proof
- Stop conditions: per card. Escalation owner: operator, via Chatterbox

### g16.097 — v0.3.0 release certification (coordinator-executed)

- Card: `g16/097-v030-release-certification.md`
- Readiness: ready — release-mutation authority is the operator's explicit
  2026-09-04 authorization recorded in the card. Never dispatched to a
  worker; the coordinator performs the ordered actions from a clean `main`.
- Prerequisites: merged `g16.054` (PR #165). Completion: `v0.3.0` tag at
  `9b451c48d`, green dry-run then publish runs, npm `latest` `0.3.0` for core
  and Svelte, fresh-consumer install proof, closeout commit on `main`.
- Owned mutable paths: tag `v0.3.0`; `docs/logs/2026-09/` log; this card;
  `CHANGELOG.md` and `docs/release-notes/0.3.0.md` published headers;
  README status paragraphs naming the latest version
- Reserved shared closeout surfaces (coordinator at closeout): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: `g16.095`, `g16.096` (they touch no release
  surface). Serial edge: none; the tag targets a commit already on `main`.
- Worker capability class: none — coordinator action
- Acceptance evidence and review oracle: the card's Acceptance checklist and
  Review Oracle table
- Stop conditions: per card; a red dry run or a failed-closed publish is an
  escalation capsule to Chatterbox. Escalation owner: operator, via Chatterbox

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
