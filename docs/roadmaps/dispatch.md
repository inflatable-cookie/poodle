# Canonical Dispatch Manifest

Status: active
Owner: Chatterbox (planning authority) — the only writer
Consumer: the coordinator, which launches every ready lane listed here and
designs no lanes, edges, or concurrency of its own
Updated: 2026-09-08 (revision 25: g17.003 background-safe non-activation proof ready)
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

| Lane | Card | Depends on | Concurrency | Capability | Completion |
| --- | --- | --- | --- | --- | --- |
| `g17.003` | [`g17/003-background-safe-nonactivation-proof.md`](g17/003-background-safe-nonactivation-proof.md) | none | no Poodle sibling | general | one reviewed PR with native self-activation evidence, windowless validation, and an exact Lab adoption request |

The lab repository's queue-managed `g01.006` thread remains blocked after zero
records were admitted. Its Lab matcher repair can finish independently; its
next foreground capture waits for `g17.003` merge and Lab adoption.

## Held lanes (not dispatchable)

| Lane | Gate | Owner of the gate |
| --- | --- | --- |
| `g17.001` Nucleus V1 visual receipts (was `g16.123`) | first validated poodle-lab `g01.006` cohort bundle | Chatterbox |
| poodle-lab `g01.006` GPUI leg | compiled in the lab repository's own dispatch surfaces, not this manifest | Poodle Chatterbox + coordinator |
| Nucleus V2 / M2 and Nucleus adoption | V2 after Nucleus seeding; M2 Nucleus-owned; switch decision after V1/V2 | operator, via Chatterbox |
| A2 platform accessibility via `gpui-unofficial` | gpui-apple builds from crates.io (`docs/triage/20260905-111233-gpui-unofficial-adoption-gates.md`) | Chatterbox |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Jetstream admission | `docs/triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry, Tabs single-consumer asks | `docs/triage/20260902-000956-*.md`, `20260902-000957-*.md`, `20260904-151947-*.md` | Chatterbox |

## Merged since revision 6

`g16.103` (#208), `104` (#209), `105` (#210), `097` (v0.3.0 published from
`85609d941`), `106` (#211), `108` (#212), `107` (#213), `110` (#214), `111`
(#215), `114` (#216), `051` (#217), `117` (#218), `112` (#219), `116` (#220),
`113` (#221), `115` (#222), `118` (#224), `119` (#223), `120` (#225), `121`
(#226), `122` (#227), `109` (15 consumer PRs). Card status lines were
reconciled to merge truth at the rollover; no README or index line is
outstanding.

## History

The pre-2026-09-04 orchestrator-owned worker ledger is archived at
`archive/2026-08-25-worker-dispatch-ledger.md`. Execution logs under
`../logs/` remain the durable record of every merged lane.
