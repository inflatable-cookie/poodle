# g15 — v0.2.0 Release Baseline

Status: blocked — first card waits on the `g14.022` closeout merge
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

## Why It Matters

Longhorn and most projects under `~/Dev/projects` depend on Poodle. v0.2.0
cannot wait for another speculative cross-runtime architecture. The release
baseline must record what is actually certified per component, what remains
unproved, and what each runtime's evidence is — then ship.

## Sequence

Only `001` is created. Nothing is dispatched until the g14 closeout merges and
the orchestrator reviews it. Later cards are compiled into bounded family
tranches only after the inventory exists; no component rollout is pre-authored
from memory.

## Runway

1. [001 — Release-baseline roster inventory](001-release-baseline-roster-inventory.md) — blocked on `g14.022` closeout merge

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

Dispatch `g15.001` — the release-baseline roster inventory — after the
`g14.022` closeout merges and the orchestrator reviews it. Until then the
generation stays blocked.
