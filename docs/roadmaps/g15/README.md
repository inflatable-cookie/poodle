# g15 — v0.2.0 Release Baseline

Status: active — `g15.001`–`g15.008` and `g15.014` complete; `g15.009`
ready next
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
9. [009 — Update, settings, Radio & context-provider native closure](009-update-settings-radio-native-closure.md) — ready next
10. [010 — Display, workstation & agent GPUI specimens](010-display-workstation-agent-gpui-specimens.md) — GPUI closure after `007`–`009`
11. [011 — Human-centred specimen catalogue audit](011-specimen-catalogue-audit.md) — carries `g14.026`
12. [012 — Primitive-first visual conformance lane](012-visual-conformance-lane.md) — per the `g14.022` decision; headless capture required
13. [014 — Release-gate remediation: security advisory prerequisite](014-release-gate-remediation.md) — complete; PR #31 cleared the `bun audit` nanoid advisory
14. [013 — v0.2.0 release certification](013-v020-release-certification.md) — final gate after `008`–`012`

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

Prepare and dispatch `g15.009`. `g15.010`–`g15.012` follow in runway order;
`g15.013` stays blocked as the final certification gate until every earlier
card is complete. Release mutation remains behind the explicit operator gate.
