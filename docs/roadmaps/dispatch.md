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

### g16.106 — Button leading-inset edge delta diagnosis

- Card: `g16/106-button-leading-inset-edge-delta.md`
- Readiness: ready
- Prerequisites: none. Completion: PR merged after accepted exact-head
  review; outcome is either an exact-inset repair with a headless test or a
  contracted `gpui-snaps-subpixel-edge` role finding.
- Owned mutable paths: per card (render button/presentation, policy role
  finding, ledger known-delta inputs, log, `PAPERCUTS.md` append)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.107`, `g16.108`, `g16.097`. Serial
  edges: none.
- Worker capability class: capable coding model, medium reasoning (Rust)
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.107 — Validation hygiene bundle

- Card: `g16/107-validation-hygiene-bundle.md`
- Readiness: ready
- Prerequisites: none. Completion: PR merged after accepted exact-head
  review; `effigy doctor` exits 0 on `main`; boards leave the tree clean.
- Owned mutable paths: per card (task catalogue, scan config, gate guard,
  pack-install script, two drift scripts, GPUI harness files, Jetstream
  adapter README, tests, log, `PAPERCUTS.md`)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.106`, `g16.108`, `g16.097`. Serial
  edges: none (`108` adds one selector line to `docs:check`; `107` edits
  other task entries — coordinator rebases the later merge).
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card, per item. Escalation owner: Chatterbox

### g16.108 — Docs spine compaction

- Card: `g16/108-docs-spine-compaction.md`
- Readiness: ready
- Prerequisites: none. Completion: PR merged after accepted exact-head
  review; `docs:check` and the new `docs:snippet-check` green.
- Owned mutable paths: per card (handoffs, parity → archive, specs, guides,
  contracts index, docs README retention rule, parity edit lines in g16
  cards, HistoryCenter docs snippet, one script, one task line, log)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.106`, `g16.107`, `g16.097`. Serial
  edges: none.
- Worker capability class: low-cost documentation model is adequate; medium
  reasoning
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| --- | --- | --- |
| Consumer adoption wave (16 sibling repos move pins to `0.3.0`; Loophole also maps the five HistoryCenter rejection codes) | proven npm `latest` `0.3.0` (`g16.097` step 5); then compiled as per-repo handoffs | Chatterbox compiles; coordinator dispatches into sibling repos |
| Loophole adoption of `0.3.0` | proven npm `latest` `0.3.0` (`g16.097` step 5); Loophole-owned planning | Loophole planning, via Chatterbox |
| `g16.051` icon geometry native visual admission | accepted VL-1 Button bootstrap and VL-2A icon adapter in `poodle-lab` | Poodle Chatterbox (lab planning) |
| VL-1 Button MVP bootstrap | compiled in the `poodle-lab` repository's own dispatch surfaces, not this manifest | Poodle Chatterbox + coordinator |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Nucleus A1 / V1 / V2 / M2 and Nucleus adoption | next programme boundary not yet chosen | operator, via Chatterbox |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |

## Merged since revision 6

`g16.103` (PR #208), `g16.104` (PR #209), `g16.105` (PR #210). Their cards
carry the receipts; the coordinator reconciles README and index lines.

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
