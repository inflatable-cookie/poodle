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

### g16.118 — A1 overlay structure projection

- Card: `g16/118-a1-overlay-structure-projection.md`
- Readiness: ready. Owns the two-role vocabulary addition (`Heading`,
  `Banner`) and the eight overlay rows.
- Prerequisites: A1 tranches merged (satisfied). Completion: PR merged after
  accepted exact-head review; eight empty-diff A1 receipts; cohort re-emitted.
- Owned mutable paths: per card
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.119` (disjoint rows; both repin at their
  rebase, merge one at a time), `109` lanes. Serial edges: `g16.120` waits
  for this merge.
- Worker capability class: capable coding model, high reasoning (Rust
  composition across eight components)
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.119 — A1 focus and state semantics

- Card: `g16/119-a1-focus-and-state-semantics.md`
- Readiness: ready
- Prerequisites: A1 tranches merged (satisfied). Completion: PR merged after
  accepted exact-head review; five empty-diff A1 receipts; cohort re-emitted.
- Owned mutable paths: per card
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.118`, `109` lanes. Serial edges: none.
- Worker capability class: capable coding model, high reasoning (GPUI
  backend focus routing)
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| `g16.120` A1 landmarks and content roles | merged `g16.118` (`Heading`, `Banner` roles) | Chatterbox |
| --- | --- | --- |
| VL-1 Button MVP bootstrap | compiled in the `poodle-lab` repository's own dispatch surfaces, not this manifest | Poodle Chatterbox + coordinator |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Nucleus V1 / V2 / M2 and Nucleus adoption | V1 after the lab Button and icon bundles; V2 after lab `g01.003` and Nucleus seeding; M2 Nucleus-owned | operator, via Chatterbox |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |

## Merged since revision 6

`g16.103` (#208), `104` (#209), `105` (#210), `097` (v0.3.0 published from
`85609d941`), `106` (#211), `108` (#212), `107` (#213), `110` (#214), `111`
(#215), `114` (#216), `051` (#217), `117` (#218), `112` (#219), `116` (#220),
`113` (#221), `115` (#222). Their cards
carry the receipts; the coordinator reconciles README and index lines.

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
