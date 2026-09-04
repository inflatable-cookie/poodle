# Canonical Dispatch Manifest

Status: active
Owner: Chatterbox (planning authority) — the only writer
Consumer: the coordinator, which launches every ready lane listed here and
designs no lanes, edges, or concurrency of its own
Updated: 2026-09-04
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

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| --- | --- | --- |
| `g16.054` v0.3.0 release candidate | release mutation authority | operator |
| `g16.051` icon geometry native visual admission | operational Button lab (`g16/visual-lab-unblock-runway.md` VL-0 repository authority) | operator |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Nucleus A1 / V1 / V2 / M2 and Nucleus adoption | next programme boundary not yet chosen | operator, via Chatterbox |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
