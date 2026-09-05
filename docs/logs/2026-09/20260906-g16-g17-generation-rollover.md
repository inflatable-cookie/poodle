# g16/g17 Generation Rollover

Date: 2026-09-06
Posture: planning closeout
Operator direction: "roll over when ready" (2026-09-06)

## Decision

g16 closed with its aim met: active-cohort parity is measured from executed
evidence, no third conformance authority was built, v0.3.0 shipped and was
adopted everywhere, and Nucleus reached M1 29/29 and A1 29/29. What remains
is one programme, the Nucleus switch path, so g17 opens as a slate with that
goal and one held card.

## Dispositions

- Complete: every lane `001`–`122` except those below. Card status lines
  for `054`, `095`, `098`–`105`, `112`–`117`, `121`, `122` were stale
  ("ready", "candidate certified", "implementation complete") although their
  PRs had merged; each now records its PR and merge SHA.
- Closed partial: `002`.
- Research-complete: `037`–`044`.
- Gated and carried: `052` (operator-owned reviewers and custody).
- Rehomed: `123` → `g17/001-nucleus-v1-visual-receipts.md`; held on the
  lab's first validated cohort bundle. `122` and the programme file point to
  the new id.
- Programme files (`nucleus-gpui-parity-programme.md`,
  `parity-evidence-ledger.md`, manifest, receipts, `visual-lab-unblock-runway.md`)
  stay under `g16/` as the authority; g17 references them.
- Specs: `docs/specs/` was purged in `g16.108` on 2026-09-05; the two specs
  with no live architecture, contract, or g16 reference (`001` token
  contract, `068` batched audio meter) describe shipped surfaces and stay.
- Triage notes stay open with their held items listed in the g17 README and
  the dispatch manifest.

## Surfaces touched

`docs/roadmaps/g16/README.md` (closed), `docs/roadmaps/g17/README.md`
(opened), `docs/roadmaps/generation-index.md`, `docs/roadmaps/README.md`,
`docs/roadmaps/dispatch.md` (revision 22, empty frontier), the reconciled
g16 cards, and this log.

## Next move

Lab `g01.006` GPUI cohort batch on an unlocked display → validated bundle →
`g17.001` ready.
