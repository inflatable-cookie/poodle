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

### g16.121 — A1 focus-model alignment (last four rows)

- Card: `g16/121-a1-focus-model-alignment.md`
- Readiness: ready
- Prerequisites: merged `g16.119`. Completion: PR merged after accepted
  exact-head review; ledger GPUI accessibility 29/29 `mounted`.
- Owned mutable paths: per card (extractor law and test, transcript
  scenario, Svelte/React Menu roving focus, four receipts, manifest
  `resolution`, ledger, log)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.122`, `109` lanes. Serial edges: none
  (122 touches the capture binary and scenario `capture` blocks; 121 touches
  the extractor and one scenario's actions — coordinator rebases the later
  merge and both re-emit at their heads).
- Worker capability class: capable coding model, medium reasoning
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

### g16.122 — Window-capture cohort fixture kind

- Card: `g16/122-window-capture-cohort-fixtures.md`
- Readiness: ready
- Prerequisites: merged `g16.105`, `g16.111` (satisfied). Completion: PR
  merged after accepted exact-head review; headless tests prove the closed
  registry and that `after-actions` equals the A1 end state.
- Owned mutable paths: per card (capture binary kind and tests, scenario
  `capture` blocks and schema, log)
- Reserved shared closeout surfaces (coordinator at merge): `g16/README.md`,
  `generation-index.md`
- Approved concurrent siblings: `g16.121`, `109` lanes. Serial edges:
  poodle-lab `g01.006`'s GPUI leg waits for this merge.
- Worker capability class: capable coding model, medium reasoning (Rust,
  GPUI)
- Acceptance evidence and review oracle: the card's table
- Stop conditions: per card. Escalation owner: Chatterbox

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| `g16.123` Nucleus V1 visual receipts | merged `g16.122` and the first validated poodle-lab `g01.006` cohort bundle | Chatterbox |
| --- | --- | --- |
| VL-1 Button MVP bootstrap | compiled in the `poodle-lab` repository's own dispatch surfaces, not this manifest | Poodle Chatterbox + coordinator |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Nucleus V2 / M2 and Nucleus adoption | V2 after Nucleus seeding (lab `g01.003` landed the lab side); M2 Nucleus-owned; switch decision after V1/V2 | operator, via Chatterbox |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |

## Merged since revision 6

`g16.103` (#208), `104` (#209), `105` (#210), `097` (v0.3.0 published from
`85609d941`), `106` (#211), `108` (#212), `107` (#213), `110` (#214), `111`
(#215), `114` (#216), `051` (#217), `117` (#218), `112` (#219), `116` (#220),
`113` (#221), `115` (#222), `118` (#224), `119` (#223), `120` (#225). Their cards
carry the receipts; the coordinator reconciles README and index lines.

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
