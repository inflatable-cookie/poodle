# g17 — Nucleus switch evidence

Status: active
Opened: 2026-09-06
Updated: 2026-09-09
Governing refs: `../../../README.md`, `../../README.md`,
`../../contracts/001-working-rules.md`, `../../evidence/nucleus/README.md`,
`../generation-index.md`

## Generation outcome

Finish the evidence Nucleus needs to switch to GPUI, then put the switch
decision in front of the operator on receipts rather than estimates. Poodle
owns reusable M1/A1/V1 evidence; Nucleus owns application journeys and M2; the
lab and external accessibility authority own V2 and A2.

## Current state

- Fixed cohort: 29 rendered components; M1 29/29; A1 29/29; V1 29/29 (PR #232).
- Poodle Lab run `2026-09-08T14-06-48`: 174 captures, 116 comparisons, two
  agreeing repeats, proved foreground evidence, 160 reported findings.
- `g17.002`–`g17.004` are complete. Their web modality, native non-activation,
  and GPUI append repairs are live-proven by the Lab run.
- A2 remains held on the `gpui-unofficial`/`gpui-apple` publication boundary.

## Generation runway

| Task or planning horizon | State | Dependency or checkpoint |
| --- | --- | --- |
| [`g17.001`](001-nucleus-v1-visual-receipts.md) — import Lab bundle and emit V1 receipts | complete | PR #232 (merge `34104e792d8f4a8c522e92b2d1c4794b01671292`) |
| [`g17.002`](002-web-focus-ring-input-modality.md) — web modality focus | complete | PR #228 |
| [`g17.003`](003-background-safe-nonactivation-proof.md) — native non-activation proof | complete | PR #230 |
| [`g17.004`](004-gpui-cohort-programmatic-append.md) — closed append replay | complete | PR #231 |
| V2 Nucleus-state capture | planning horizon | Nucleus-owned seeding request accepted by Nucleus planning |
| M2 Nucleus journeys | planning horizon | Nucleus-owned GPUI build and journeys |
| A2 platform accessibility | held | `gpui-apple` builds from crates.io; external tree proof available |
| Operator switch packet | planning horizon | M1+A1+A2+V1 per row and composed M2+V2 |
| GPUI keyboard-origin focus | planning horizon | separate task compilation after V1 checkpoint |
| Web-pair composite extraction | operator checkpoint | first React consumer or affected composite touched |

No row is approved for queue dispatch. Planned horizons are not
tasks and carry no execution authority.

## Held and recurring work

- Contributor design-guidance pilot: operator-gated on named reviewers,
  approvals, and run custody.
- Jetstream admission: `../../triage/20260902-000959-jetstream-admission-hold.md`.
- Citations, nested menus, HistoryCenter policy, keyboard geometry, consumer
  Tabs asks, repository settings, and web-pair architecture remain in current
  triage notes.
- Consumer `PAPERCUTS.md` intake remains a recurring Chatterbox sweep.

## Approved frontier

No approved frontier. `g17.001` merged in PR #232 and is closed out in the
integration checkout. After closeout, return to Chatterbox for the
next planning checkpoint; no successor auto-starts.
