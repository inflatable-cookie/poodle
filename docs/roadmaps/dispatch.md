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
  cards, HistoryCenter docs snippet, one script, one task line, the
  historical-prefix lists of gates that object to the move — item 6, amended
  2026-09-05 — log)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.106`, `g16.107`, `g16.097`. Serial
  edges: none.
- Worker capability class: low-cost documentation model is adequate; medium
  reasoning
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.109 — v0.3.0 consumer adoption wave (15 sibling lanes)

- Card: `g16/109-v030-consumer-adoption-wave.md` (per-repository table is
  the lane inventory; the coordinator writes one handoff per repository)
- Readiness: ready; amended 2026-09-05. Tier 1 lanes run concurrently:
  Longhorn, Underlay, Soundcheck Library. The Underlay lane completes only
  when the coordinator has cut and pushed Underlay `v0.9.8` (operator
  authorized) after the pin PR merges. Tier 2 gates: after tag `v0.9.8`
  exists on Underlay origin → Acowtancy, Compli Me, Contact Patch,
  Songsprout, Underlay Reference (each also moves its Underlay git ref to
  `#v0.9.8`); after the Longhorn pin merges and the sibling checkout is at
  that commit → Bovine Accelerator Desktop, Figmatic, Finch, Jetstream,
  Nucleus, Loophole; after both Longhorn and Soundcheck Library → Soundcheck.
  Every lane runs in its own worktree in its own repository.
- Prerequisites: `g16.097` complete (npm `latest` `0.3.0`, publish run
  `33952493234`). Completion per lane: PR merged on the consumer's `main`
  after accepted exact-head review with the board transcript.
- Owned mutable paths: per lane, the consumer's Poodle declarations, lock,
  and bounded compatibility fallout named in the card; the consumer's
  `PAPERCUTS.md` entries the bump closes. Never Poodle.
- Reserved shared closeout surfaces (coordinator, in Poodle, at wave end):
  `g16/README.md`, `generation-index.md`, README adoption count
- Approved concurrent siblings: all lanes within a tier; Poodle lanes
  `g16.106`–`108` are unaffected. Serial edges: tier 2 behind its named
  foundation.
- Worker capability class: capable coding model, medium reasoning; Loophole
  (HistoryCenter v3 migration) and Acowtancy (19 import rewrites) may take
  the higher end of the ordinary pool
- Acceptance evidence and review oracle: the card's table and lane rules
- Stop conditions: per lane. Escalation owner: Chatterbox

### g16.110 — `gpui-unofficial` feasibility spike

- Card: `g16/110-gpui-unofficial-feasibility-spike.md`
- Readiness: ready; time-boxed to two worker days
- Prerequisites: none. Completion: report PR merged on `main`; spike branch
  pushed and left unmerged as evidence.
- Owned mutable paths: the `spike/gpui-unofficial` branch; on `main` only
  the report under `docs/logs/2026-09/` and `PAPERCUTS.md` (append)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.106`–`108`, `g16.109` lanes. Serial
  edges: `g16.106` touches `packages/render/src/button.rs` on `main`; the
  spike branch may diverge from it, which is acceptable for a spike.
- Worker capability class: capable coding model, high reasoning (Rust,
  gpui API migration); not frontier unless the worker escalates a blocker
- Acceptance evidence and review oracle: the card's table; the reviewer
  checks the licence transcript and that the AccessKit test reads the tree
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.111 — Nucleus A1 accessibility receipt foundation

- Card: `g16/111-nucleus-a1-accessibility-receipt-foundation.md`
- Readiness: ready. Operator-designated critical work: the frontier pool
  (Claude Fable 5.1, medium thinking) is authorized for this lane.
- Prerequisites: merged `g16.062`, M1 complete (both satisfied).
  Completion: PR merged after accepted exact-head review with two validated
  A1 receipts (Switch, Tabs), one recorded Select divergence, and the ledger
  consuming them.
- Owned mutable paths: per card (`test/nucleus-a11y/**`, driver extractor,
  receipt script and schema, two A1 receipts plus divergence artifacts, ledger
  generator A1 path, log, `PAPERCUTS.md` append)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.106`–`110`, `109` lanes. Serial edges:
  `g16.112`–`116` wait for this merge.
- Worker capability class: frontier coding model, medium reasoning
  (operator-designated)
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| --- | --- | --- |
| `g16.051` icon geometry native visual admission | accepted VL-1 Button bootstrap and VL-2A icon adapter in `poodle-lab` | Poodle Chatterbox (lab planning) |
| VL-1 Button MVP bootstrap | compiled in the `poodle-lab` repository's own dispatch surfaces, not this manifest | Poodle Chatterbox + coordinator |
| `g16.112`–`g16.116` Nucleus A1 tranches NP-1…NP-5 | merged `g16.111`; ordinary workers, one PR each, concurrent | Chatterbox |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Nucleus V1 / V2 / M2 and Nucleus adoption | V1 after the lab Button and icon bundles; V2 after lab `g01.003` and Nucleus seeding; M2 Nucleus-owned | operator, via Chatterbox |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |

## Merged since revision 6

`g16.103` (PR #208), `g16.104` (PR #209), `g16.105` (PR #210), `g16.097`
(v0.3.0 published from `85609d941`, closeout `1eadc581a`). Their cards
carry the receipts; the coordinator reconciles README and index lines.

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
