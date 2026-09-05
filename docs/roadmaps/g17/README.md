# g17 — Nucleus Switch Path

Status: active — opened 2026-09-06 at the g16 rollover; one held card
(`001`), no ready card; the frontier is `../dispatch.md`
Posture: operator-led planning slate with a fixed programme goal
Opened: 2026-09-06
Governing refs: `../../../README.md`, `../../README.md`,
`../../contracts/001-working-rules.md`, `../g16/README.md`,
`../g16/nucleus-gpui-parity-programme.md`, `../generation-index.md`,
`../../logs/2026-09/20260906-g16-g17-generation-rollover.md`

## Aim

Finish the evidence Nucleus needs to switch to GPUI, then let the operator
make that switch decision on receipts, not on estimates. The operator's
standing decision (2026-09-02): the lack of parity is what stops commitment
to GPUI; real parity switches a bunch of apps, Nucleus first.

## Inherited State

- Nucleus cohort: 29 components; M1 29/29 mounted; A1 29/29 node-tree
  accessibility receipts paired with Svelte DOM ARIA; GPUI visual compared
  1 / missing 174 in the ledger.
- Capture path: Poodle `g16.122` renders any cohort row from its scenario
  file in the non-activating capture window; the lab (`poodle-lab`
  `g01.006`) has the cohort adapter's web legs merged and the GPUI leg
  pending an unlocked display.
- Release: `v0.3.0` published 2026-09-05; all 15 consumers adopted it.
- A2 (platform accessibility tree): route is the upstream AccessKit work via
  the `gpui-unofficial` republish; held on the gpui-apple crates.io build
  defect (`../../triage/20260905-111233-gpui-unofficial-adoption-gates.md`).

## Generation Runway

1. [001 — Nucleus V1 visual receipts](001-nucleus-v1-visual-receipts.md) —
   held; ready when the lab's first validated cohort bundle exists

Planned, not yet compiled (each becomes a card only after Chatterbox
promotion; order is the intended sequence):

- **V2 Nucleus-state capture.** Nucleus-owned seeding of its real app states
  into the lab's pinned Nucleus web build (lab `g01.003` landed the lab
  side). Poodle's part is the V2 receipt level and ledger axis once a bundle
  exists. Gate: a Nucleus seeding request accepted by Nucleus planning.
- **M2 Nucleus journeys.** Nucleus-owned end-to-end journeys on the GPUI
  build. Poodle records the receipt; it does not write Nucleus.
- **Switch decision packet.** One note that puts M1/A1/V1/V2/M2 receipts and
  the open known deltas in front of the operator. Operator decides.
- **A2 platform accessibility.** When gpui-apple builds from crates.io:
  migrate the GPUI backend to the `gpui-unofficial` republish (no vendoring,
  operator rule), then emit platform-tree A2 receipts for the cohort.
- **Web pair composites.** Decide whether to extract duplicated
  Svelte/React composite logic into core while React is retained
  (`../../triage/20260901-233708-holistic-posture-assessment.md`). Operator
  decision before any card.
- **Consumer intake.** Recurring `PAPERCUTS.md` sweep; the single-consumer
  Tabs asks stay in `../../triage/20260904-151947-consumer-sweep-intake.md`
  until a second consumer needs them. A `0.3.x` or `0.4.0` release only
  when a consumer needs a change that is on `main`.

## Held (carried from g16)

| Item | Gate | Owner |
| --- | --- | --- |
| `g16.052` contributor design-guidance pilot | named reviewers, approvals, run custody | operator |
| Jetstream admission | `../../triage/20260902-000959-jetstream-admission-hold.md` | operator |
| Citations, nested menus, CS20, keyboard geometry | `../../triage/20260902-000956-*.md`, `20260902-000957-*.md` | Chatterbox |
| Repository delete-on-merge | operator setting | operator |

## Programmes

| Programme | State (2026-09-06) | Authority |
| --- | --- | --- |
| Nucleus GPUI parity | M1 29/29, A1 29/29, V1 receipts held (`001`), V2/M2/switch planned | `../g16/nucleus-gpui-parity-programme.md` |
| Visual lab | cohort adapter GPUI leg pending; bundle → `001` | `../g16/visual-lab-unblock-runway.md`, lab repo |
| GPUI accessibility route | held on gpui-apple build | contract 003, triage `20260905-111233-*` |
| Consumer adoption | all 15 on `0.3.0`; sweep recurring | `../README.md` rules |

## Next Task

Dispatch from `../dispatch.md` only. The next Chatterbox promotion is `001`
readiness once the lab bundle validates.
