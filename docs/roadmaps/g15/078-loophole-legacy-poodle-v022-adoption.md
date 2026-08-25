# g15.078 — Loophole Legacy Poodle 0.2.2 adoption

Status: **cancelled — repository removed by operator on 2026-08-25**
Depends on: `g15.061`, `g15.062`; schedule after active product lanes
Target repository: `/Users/tom/Dev/projects/loophole-legacy`
Target base: `2047e81c81b25c3dbc8db4b72de5960e305f6aea`
Governing refs: `055-consumer-adoption-inventory.md`, Loophole Legacy root and
Aura `AGENTS.md`, Chorus authority, and active Aura package contracts

## Disposition

The target repository no longer exists and is no longer an authoritative
Poodle consumer. Do not dispatch this card, recreate the repository, or require
its adoption for `g15` completion. The remaining content is preserved as the
cancelled plan and historical scope record.

## Outcome

Move active Aura from Poodle Svelte 0.1.x to exact public 0.2.2 while leaving
reference archives and other Loophole Legacy components untouched.

## Scope

- Pin Aura's Poodle Svelte dependency to exact `0.2.2`.
- Retain local Longhorn packages and regenerate `aura/bun.lock` narrowly.
- Prove the Longhorn adapter peer converges on the same Svelte identity.
- Repair only Aura-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not touch `aura/reference/legacy-app`, Chorus, Pulse, Spark, Echo, Signal,
  native runtimes, or product behavior beyond bounded compile repairs.
- Do not add sibling Poodle sources, aliases, or unrelated upgrades.
- Do not launch visible Electron/Tauri applications.

## Acceptance

- Active Aura resolves registry Poodle Svelte 0.2.2 with published integrity.
- Longhorn peer converges on the same Svelte identity.
- No active old/local Poodle remains; reference archives are unchanged.
- Aura and workspace validation pass or a reproduced baseline is separated.

## Validation

- Use the workspace Effigy-owned install path; inspect Aura's graph and lock.
- Run `effigy aura/check`, `effigy aura/validate`, `effigy validate`, and
  `effigy qa`; add headless Aura tests selected by `effigy test --plan`.
- Run `git diff --check`. Do not run visible app selectors.

## Stop Conditions

- Adoption needs a Loophole Legacy/Longhorn/public API decision.
- Install resolves duplicate/local Poodle.
- Lock churn touches reference archives or unrelated components.
- Evidence needs a visible application.

## Evidence And Continuation

No worker or PR is required. Jetstream `g15.077` is the sole remaining
authoritative consumer lane.
