# g15 — v0.2.0 Release Baseline

Status: active — `g15.001`–`g15.010` and `g15.014`–`g15.016` complete;
`g15.011` partial with bounded continuation cards; `g15.017` ready to dispatch
Posture: release-first; no new parity architecture
Opened: 2026-08-16
Governing refs: `../g14/022-generation-closeout.md`,
`../g14/conformance-estate.md`, `../../contracts/001-working-rules.md`,
`../../logs/2026-08/16-g14-022-generation-closeout.md`

## Generation Goal

Ship Poodle v0.2.0 on an honest, complete Svelte roster. The release
denominator is **every public Svelte component export** — frozen from
`packages/svelte/components/src/index.ts` and the package `exports` map, not
from a representative subset. React stays tightly paired through shared CSS
and framework-free web behaviour; the measured shared Rust and GPUI gaps close
before certification; Jetstream remains program-deferred.

This generation is release-first, not architecture-first. Release-first means
the runway closes the measured implementation, specimen, and evidence gaps
before certification; it does not make certification an early exit. Two parity
architectures were tried and rejected (g13's Rust IR, g14's executable
conformance). g15 does not design a third one. It inventories, closes the
honest gaps, and ships.

## Release Denominator and Active-Cohort Closure

The full Svelte roster is the v0.2.0 release denominator. The generation still
completes its active-cohort implementation and evidence cards before release
certification. Experimental package labels remain honest, Jetstream remains
program-deferred, and no runtime borrows another runtime's pass.

## Why It Matters

Longhorn and most projects under `~/Dev/projects` depend on Poodle. v0.2.0
cannot wait for another speculative cross-runtime architecture. The release
baseline must record what is actually certified per component, what remains
unproved, and what each runtime's evidence is — then ship.

## Sequence

`001` and `002` are executed; the inventory froze the denominator and the
first evidence tranche closed 29 paired gaps. Later cards are listed in
dependency order and are not dispatched until the orchestrator reviews and
advances them. `g15.014` was an urgent prerequisite remediation executed out
of numeric order; `g15.013` remains the final card.

## Runway

Measured from `g15.001`'s frozen roster (`release-baseline-roster.md`) and gap
register (`release-gap-register.md`). Dependency order only; orchestration
and status advance are the orchestrator's.

1. [001 — Release-baseline roster inventory](001-release-baseline-roster-inventory.md) — complete
2. [002 — Svelte focused evidence: foundation display & shell](002-svelte-focused-evidence-display-shell.md) — complete; 29 paired evidence gaps closed
3. [003 — Svelte focused evidence: forms, inputs & overlays](003-svelte-focused-evidence-forms-inputs-overlays.md) — complete; 26 Svelte and 25 React evidence gaps closed
4. [004 — Svelte focused evidence: composites & media](004-svelte-focused-evidence-composites-media.md) — complete; 35 paired evidence gaps closed
5. [005 — Svelte focused evidence: workstation & agent](005-svelte-focused-evidence-workstation-agent.md) — complete; final 24 Svelte and 23 React evidence gaps closed
6. [006 — React mirror implementation & gallery closure](006-react-mirror-closure.md) — complete; React implementation/gallery are 175/0 and focused evidence is 152/23
7. [007 — Licence family native completion](007-licence-family-native-completion.md) — complete; PR #32 closed the Licence native family and prerequisites
8. [008 — Model-connection family native completion](008-model-connection-family-native-completion.md) — complete; PR #33 closed the model-connection native family
9. [009 — Update, settings, Radio & context-provider native closure](009-update-settings-radio-native-closure.md) — complete; PR #34 closed the scoped native surfaces and declared UiPresentationProvider's remaining cascade gap
10. [010 — Display, workstation & agent GPUI specimens](010-display-workstation-agent-gpui-specimens.md) — complete; PR #35 closed all 18 measured GPUI specimen gaps
11. [011 — Human-centred specimen catalogue audit](011-specimen-catalogue-audit.md) — partial; screening baseline and three approved pilots delivered
12. [015 — Specimen caption integrity](015-specimen-caption-integrity.md) — complete; PR #37 restored 52 captions and closed the Svelte-preview gate hole
13. [016 — Specimen idiom convergence](016-specimen-idiom-convergence.md) — complete; PR #38 converged all 29 paired routes
14. [017 — Web specimen axis placement](017-specimen-axis-placement.md) — ready; exact 24-route paired-web scope
15. [018 — Overloaded Examples curation](018-overloaded-examples-curation.md) — non-dispatchable parent for exact children `020`–`025`
16. [019 — GPUI specimen structure](019-gpui-specimen-structure.md) — after `017`
17. [020–025 — Overloaded Examples family children](020-curate-model-connection-licence.md) — exact 53-page partition; follow each child's dependencies
18. [026 — Headless native specimen probe](026-native-specimen-probe.md) — native completion lane for `011`
19. [027 — Screen-clear human review](027-screen-clear-human-review.md) — non-dispatchable parent for exact children `028`–`033`
20. [028–033 — Screen-clear family review children](028-review-foundation-controls-entry.md) — exact 56-page partition; after `026`
21. [012 — Primitive-first visual conformance lane](012-visual-conformance-lane.md) — after `011` completes; headless capture required
22. [014 — Release-gate remediation: security advisory prerequisite](014-release-gate-remediation.md) — complete; PR #31 cleared the `bun audit` nanoid advisory
23. [013 — v0.2.0 release certification](013-v020-release-certification.md) — final gate after the full specimen program and `012`

Supporting evidence: [release-baseline-roster.md](release-baseline-roster.md),
[release-gap-register.md](release-gap-register.md)

## Carry-Forward Envelope (recorded, not implemented)

These enter g15 with their g14 dispositions and are not dispatched as g14
work:

- Approved Licence web references (`g14.015`/`g14.016`); native completion
  recompiled from `g14.017`'s component requirements
- Approved model-connection web references (`g14.018`/`g14.019`); native
  completion recompiled from `g14.020`'s component requirements
- Human-centred specimen catalogue audit (`g14.026`), carried forward with its
  rubric and bounded shared specimen-plan boundary intact
- Primitive-first visual conformance, which may reuse the retained headless
  and native capture foundation (`conformance-estate.md`); the seam is
  recorded, not built
- Native completion of any component the inventory finds incomplete

## Dispatch Rule

Follows the [worker dispatch ledger](../dispatch.md) contract: the orchestrator
dispatches one whole card to a fresh thread/worktree when its dependencies are
met. Workers do not write `dispatch.md` or change roadmap status.

## Current Task

Dispatch `g15.017` from its committed worker handoff. `g15.011` remains
partial until `g15.026` and all six screen-clear review children
(`g15.028`–`g15.033`) land; defect-led curation proceeds through
`g15.015`–`g15.025`. `g15.012` follows the completed audit. `g15.013` stays
blocked as the final certification gate. Release mutation remains behind the
explicit operator gate.
