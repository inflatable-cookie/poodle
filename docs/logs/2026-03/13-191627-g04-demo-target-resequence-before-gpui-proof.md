---
title: g04 demo target resequence before gpui proof
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, planning, gpui, svelte]
---

## Summary

Resequenced the remaining `g04` roadmap so the program now pauses to clean up
and contract the shared demo target before trying to prove downstream GPUI
adoption from a messy Svelte surface.

## What changed

- rewrote `g04.012` as `shared demo-app audit, gap register, and target-shape freeze`
- rewrote `g04.013` as `cross-runtime demo-app contract, section model, and parity checklist`
- rewrote `g04.014` as `Svelte demo-app rebuild, component adoption, and coverage upgrade`
- added `g04.015` for GPUI demo-app parity implementation and side-by-side review
- moved the old downstream GPUI proof, docs promotion, and closeout milestones
  back to `g04.016`, `g04.017`, and `g04.018`
- updated the generation README, generation index, repo README surfaces, and
  roadmap references so the repo now points at the demo-audit pause instead of
  jumping straight into downstream GPUI proof

## Rationale

- the current Svelte demo or preview surface is too implementation-heavy and
  uneven to serve as a clean “same UI” target for GPUI
- many built components are still missing, indirect, or mixed into docs-shell
  glue instead of a deliberate target app
- GPUI reference-app or downstream proof would be lower-signal if it tried to
  imitate that surface before the target app itself was rebuilt coherently

## Validation

- `bun run --cwd packages/svelte/preview docs:lint`
- `git diff --check`

## Outcome

`g04` now has an explicit alignment tranche between parity bookkeeping and
downstream GPUI proof. The next work is to audit and contract the shared demo
target, then rebuild the Svelte version before asking GPUI to match it.

## Next

Open `g04.012` and audit the current shared demo surface, freezing the gap
register and target shape that both the rebuilt Svelte demo and later GPUI
demo must satisfy.
