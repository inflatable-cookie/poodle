# g16.011 — Nucleus Poodle 0.2.2 adoption

Status: **ready — independent first-wave product lane**
Depends on: `g16.007`, `g16.008`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/nucleus`
Target base: `91316dbe3068e4cb6cebd4cf6a2c14f55a7a4601`
Governing refs: `001-consumer-adoption-inventory.md`,
`008-longhorn-poodle-v022-adoption.md`, Nucleus `AGENTS.md`, Nucleus working
rules and Longhorn-consumer boundary

## Outcome

Move Nucleus Desktop from registry Poodle 0.1.0 to exact public 0.2.2 while
preserving its local Longhorn package integration. Prove the app resolves one
Poodle 0.2.2 web identity and still satisfies Nucleus's Longhorn boundary,
desktop checks, and broad headless board.

## Scope

- Pin `apps/desktop` Poodle core and Svelte dependencies to exact `0.2.2`.
- Regenerate `apps/desktop/bun.lock` without unrelated upgrades.
- Confirm the locally consumed `longhorn-poodle-svelte` peer resolves Poodle
  Svelte 0.2.2 after Longhorn PR 9.
- Repair only Nucleus-owned compatibility failures caused by the Poodle bump.

## Out Of Scope

- Do not change, publish, or replace the local Longhorn packages.
- Do not alter Nucleus server, persistence, provider, Tauri, public API, or
  product behavior unless exact 0.2.2 adoption exposes a bounded compile fault.
- Do not edit Poodle or Longhorn, add a compatibility shim, or launch a visible
  desktop application or native proof.

## Acceptance

- Nucleus resolves public Poodle core/Svelte 0.2.2 with registry integrity and
  no 0.1.0, 0.2.1, sibling Poodle path, or duplicate Poodle resolution.
- The Longhorn Poodle adapter's peer resolves the same Svelte 0.2.2 instance.
- The lockfile diff contains no unrelated upgrade.
- `effigy check:longhorn-consumer`, desktop check/build/test, and the broad
  headless `effigy qa` board pass.

## Validation

- Install from the declared sources with the repository's Bun version and
  inspect the effective dependency graph and full lock diff.
- Run `effigy check:longhorn-consumer`, `effigy desktop:check`,
  `effigy desktop:build`, `effigy desktop:test`, and `effigy qa`.
- Run `git diff --check`. Do not run `desktop:dev`, `desktop:proof`, or any
  selector that launches or focuses the application.

## Stop Conditions

- Adoption requires a Longhorn, Poodle, Nucleus public-contract, server, or
  Tauri decision.
- Installation resolves two Poodle versions or a local Poodle checkout.
- Lock regeneration materially changes unrelated dependencies.
- Required evidence needs a visible or focus-taking application run.

## Evidence And Continuation

Record changed files, exact registry versions/integrities, effective peer
resolution, compatibility edits, lock review, and exact validation in the
Nucleus PR. Do not merge. Once this lane lands, Nucleus is complete for g16;
its migration findings feed compilation of the remaining direct-app cards.
