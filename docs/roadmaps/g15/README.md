# g15 — v0.2.0 Release Baseline

Status: active — `g15.001` ready for dispatch
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
and framework-free web behaviour; the certified GPUI subset is named honestly;
Jetstream remains program-deferred.

This generation is release-first, not architecture-first. Two parity
architectures were tried and rejected (g13's Rust IR, g14's executable
conformance). g15 does not design a third one. It inventories, closes the
honest gaps, and ships.

## Release Support Versus Parity Completion

The full Svelte roster is the v0.2.0 release denominator. Certifying that
package does not certify active-cohort parity across Svelte, React, Rust, and
GPUI. Missing mirrors and native implementations remain explicit gaps under
the working rules; experimental packages keep experimental labels, and Poodle
does not claim full cross-runtime completion while those gaps remain.

## Why It Matters

Longhorn and most projects under `~/Dev/projects` depend on Poodle. v0.2.0
cannot wait for another speculative cross-runtime architecture. The release
baseline must record what is actually certified per component, what remains
unproved, and what each runtime's evidence is — then ship.

## Sequence

Only `001` is created. Later cards are compiled into bounded family tranches
only after the inventory exists; no component rollout is pre-authored from
memory.

## Runway

Measured from `g15.001`'s frozen roster (`release-baseline-roster.md`) and gap
register (`release-gap-register.md`). All follow-on cards are blocked pending
orchestrator review; they are listed in dependency order and are not
dispatched.

1. [001 — Release-baseline roster inventory](001-release-baseline-roster-inventory.md) — complete; frozen the 175-component denominator
2. [002 — Svelte focused evidence: foundation display & shell](002-svelte-focused-evidence-display-shell.md) — **first executable tranche**; blocked
3. [003 — Svelte focused evidence: forms, inputs & overlays](003-svelte-focused-evidence-forms-inputs-overlays.md) — blocked
4. [004 — Svelte focused evidence: composites & media](004-svelte-focused-evidence-composites-media.md) — blocked
5. [005 — Svelte focused evidence: workstation & agent](005-svelte-focused-evidence-workstation-agent.md) — blocked
6. [006 — React mirror closure](006-react-mirror-closure.md) — blocked; mirrors `002`–`005`
7. [007 — Licence family native completion](007-licence-family-native-completion.md) — blocked; carries `g14.017`
8. [008 — Model-connection family native completion](008-model-connection-family-native-completion.md) — blocked; carries `g14.020`
9. [009 — Update, settings, Radio & context-provider native closure](009-update-settings-radio-native-closure.md) — blocked
10. [010 — Display, workstation & agent GPUI specimens](010-display-workstation-agent-gpui-specimens.md) — blocked; GPUI closure after `007`–`009`
11. [011 — Human-centred specimen catalogue audit](011-specimen-catalogue-audit.md) — blocked; carries `g14.026`
12. [012 — Primitive-first visual conformance lane](012-visual-conformance-lane.md) — blocked; seam recorded, harness not designed
13. [013 — v0.2.0 release certification](013-v020-release-certification.md) — blocked; depends on `002`–`006`

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

Dispatch `g15.001` — the release-baseline roster inventory.
