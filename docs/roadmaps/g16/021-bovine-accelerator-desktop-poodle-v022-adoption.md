# g16.021 — Bovine Accelerator Desktop Poodle 0.2.2 adoption

Status: **handoff ready — isolated Longhorn-product lane**
Depends on: `g16.007`, `g16.008`, `g16.011`
Target repository: `/Users/tom/Dev/projects/bovine-accelerator-desktop`
Target base: `ac7487fd82e9792b14f1c499f4342182914501da`
Governing refs: `001-consumer-adoption-inventory.md`,
`011-nucleus-poodle-v022-adoption.md`, Bovine Accelerator Desktop `AGENTS.md`,
working rules, and dependency-release checks

## Outcome

Move Bovine Accelerator Desktop from committed sibling Poodle sources to exact
public 0.2.2 while preserving its Rust-owned domain and local Longhorn bridge.

## Scope

- Replace core/Svelte `file:` dependencies with exact registry `0.2.2`.
- Remove only Poodle overrides; retain Longhorn local dependency/override.
- Keep the published icon-builder path working through Poodle core.
- Regenerate `bun.lock` narrowly; repair only app-owned compatibility fallout.

## Out Of Scope

- Do not use or alter the operator checkout or existing product worktrees;
  start from target base in a fresh isolated worker worktree.
- Do not edit Longhorn/Poodle, Rust domain behavior, Tauri commands, or content.
- Do not add aliases or launch visible proof/dev applications.

## Acceptance

- Desktop resolves public core/Svelte 0.2.2 with published integrity.
- Longhorn peer converges on the same Svelte identity.
- Poodle icon generation works from the packed package.
- No sibling Poodle path or old version remains; lock churn is bounded.

## Validation

- Use repository Effigy dependency preparation and inspect the full lock diff.
- Run `effigy check:dependencies:release`,
  `effigy check:dependencies:release:source-independent`,
  `effigy check:frontend`, `effigy test:desktop`, and `effigy qa`.
- Run `git diff --check`. Avoid visible proof selectors.

## Stop Conditions

- Adoption needs a Bovine/Longhorn/Tauri/public API decision.
- The isolated worktree boundary cannot be proved.
- Install resolves duplicate/local Poodle or icon generation needs source-only
  files absent from the publication.
- Lock churn is unrelated or evidence needs a visible app.

## Evidence And Continuation

Record source-independent registry proof, icon generation, peer convergence,
lock review, compatibility edits, and validation in the Bovine PR. Do not
merge. Independent of the other product cards.
