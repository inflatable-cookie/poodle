# g15.065 — Nucleus Poodle 0.2.2 adoption

Status: **complete — Nucleus PR 1 merged at `9b3f67c9`**
Depends on: `g15.061`, `g15.062`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/nucleus`
Target base: `91316dbe3068e4cb6cebd4cf6a2c14f55a7a4601`
Governing refs: `055-consumer-adoption-inventory.md`,
`062-longhorn-poodle-v022-adoption.md`, Nucleus `AGENTS.md`, Nucleus working
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
Nucleus PR. Do not merge. Once this lane lands, Nucleus is complete for g15;
its migration findings feed compilation of the remaining direct-app cards.

## Closeout

Nucleus PR [#1](https://github.com/inflatable-cookie/nucleus/pull/1) merged on
2026-08-25 at `9b3f67c9c7d57700449ef26b5124d1b092093925`. Nucleus Desktop now pins
public Poodle core and Svelte 0.2.2 exactly while retaining its three intended
local Longhorn packages. The Bun lock contains the published npm integrity
values, no older or sibling Poodle source, and one Poodle/Svelte runtime. The
Longhorn Poodle adapter peer converges on the same exact 0.2.2 identity.

The bounded compatibility tail aligned the settings test with Poodle 0.2.2 and
the consumer verifier with Longhorn's post-Card-179 layout package shape. The
verifier still forbids hosted-Surface renderer imports and transfer/windowing
crates. Independent review passed `effigy check:longhorn-consumer`, desktop
check/build/test, the broad headless `effigy qa` board, registry-integrity and
lock inspection, and `git diff --check`. The canonical verdict is
[recorded on PR 1](https://github.com/inflatable-cookie/nucleus/pull/1#issuecomment-5408030177).
