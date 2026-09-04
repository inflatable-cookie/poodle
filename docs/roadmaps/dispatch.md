# Canonical Dispatch Manifest

Status: active
Owner: Chatterbox (planning authority) — the only writer
Consumer: the coordinator, which launches every ready lane listed here and
designs no lanes, edges, or concurrency of its own
Updated: 2026-09-04 (revision 5: 096 checkout-fetch revision)
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

### g16.098 — Cold-checkout web board repair

- Card: `g16/098-cold-checkout-web-board-repair.md`
- Readiness: ready. Highest priority: `g16.096` and `g16.097` are serial
  behind it.
- Prerequisites: none. Completion: PR merged after accepted exact-head
  review with the cold-path proof committed before the fix.
- Owned mutable paths: `vitest.config.ts`, `tasks/effigy.tasks.toml`
  (`ci:web` sequence only), one cold-path proof, `docs/logs/2026-09/` log,
  root `PAPERCUTS.md` (append only)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: `g16.099`. Serial edges: `g16.096`
  re-run and `g16.097` re-certification wait for this merge.
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's Review Oracle table
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.099 — React prop port tranche

- Card: `g16/099-react-prop-port-tranche.md`
- Readiness: ready — `g16.095` merged at `f297774f4` with five
  `pending-port` baseline entries naming this card.
- Prerequisites: none remaining. Completion: PR merged after accepted
  exact-head review; every `pending-port` entry removed; gate green.
- Owned mutable paths: `packages/react/components/src/{Button,Calendar,SplitView,AppHeader,DockRegion}.tsx`
  and their tests; the `BASELINE` register in
  `packages/svelte/preview/scripts/react-prop-drift.ts` (removals only); one
  runtime-note sentence in `docs/contracts/components/app-header.md` if the
  React `element` form needs it; `docs/logs/2026-09/` log; root
  `PAPERCUTS.md` (append only)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: `g16.098` (no path overlap: it owns
  `vitest.config.ts` and `tasks/effigy.tasks.toml`). Serial edges: none.
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's Review Oracle table
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.096 — Linux headless PR and main board

- Card: `g16/096-linux-headless-pr-board.md`
- Readiness: ready — revision on PR #201 (head `2c7cb6f2d`, already rebased
  onto the `098` merge): add `fetch-depth: 0` or an explicit
  `origin/main` fetch to the checkout step in both workflows so
  `test:web-pack-install` can compute its base on `pull_request` runs
  (run `33881115094` failed at `web-preview.ts:167`). Same worker, same
  reviewer on the new exact head. Workflow-edit authority is the operator's
  explicit 2026-09-02 approval recorded in the card; the 2026-09-04 widening
  stays inside the two owned files
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
- Readiness: ready for step 0 now (retract the failed `v0.3.0` tag; operator
  decision 2026-09-04); steps 1–7 serial behind merged `g16.098`. Release
  mutation authority is the operator's 2026-09-04 authorization recorded in
  the card. Never dispatched to a worker.
- Prerequisites: merged `g16.054` (PR #165) and merged `g16.098`. Completion:
  `v0.3.0` tag at the re-certified `main` tip, green dry-run then publish
  runs, npm `latest` `0.3.0` for core and Svelte, fresh-consumer install
  proof, closeout commit on `main`.
- Owned mutable paths: tag `v0.3.0`; `docs/logs/2026-09/` log; this card;
  `CHANGELOG.md` and `docs/release-notes/0.3.0.md` published headers;
  README status paragraphs naming the latest version
- Reserved shared closeout surfaces (coordinator at closeout): `g16/README.md`,
  `generation-index.md`, this manifest
- Approved concurrent siblings: `g16.099`. Serial edges: `g16.098`
  before step 1; `g16.096` may merge before or after, it does not touch the
  certified tree's gates.
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
