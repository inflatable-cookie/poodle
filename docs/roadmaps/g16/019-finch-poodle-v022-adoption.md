# g16.019 — Finch Poodle 0.2.2 adoption

Status: **ready — independent Longhorn-product lane**
Depends on: `g16.007`, `g16.008`, `g16.011`
Target repository: `/Users/tom/Dev/projects/finch`
Target base: `ab4f5a6072ee70391ed6d2b8513c8d035dbc7609`
Governing refs: `001-consumer-adoption-inventory.md`,
`011-nucleus-poodle-v022-adoption.md`, Finch `AGENTS.md`, active Tauri app
contracts, and Finch planning authority

## Outcome

Move Finch's active Tauri app from registry Poodle 0.1.x to exact public 0.2.2
while retaining its intentional local Longhorn packages.

## Scope

- Pin active `app-tauri` core/Svelte dependencies to exact `0.2.2`.
- Regenerate `app-tauri/bun.lock` narrowly and prove the Longhorn adapter peer
  converges on the same Svelte identity.
- Update dependency comments only when needed for truthfulness.
- Repair only Finch-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not touch `archive/`, local Longhorn packages, controller behavior, Tauri
  APIs, or product semantics.
- Do not add sibling Poodle links or compatibility shims.
- Do not launch the visible Tauri app.

## Acceptance

- Active app resolves registry core/Svelte 0.2.2 with published integrity.
- Longhorn adapter peer converges on the same Svelte instance.
- No active old or local Poodle identity remains; archive stays untouched.
- Lock churn is bounded; Finch root validation and QA pass or baseline is
  reproduced.

## Validation

- Use Finch's Effigy-owned install flow; inspect the active app graph and lock.
- Run `effigy validate` and `effigy qa`, plus narrower active-app selectors
  exposed by `effigy tasks` after checkout.
- Run `git diff --check`. Do not run a visible app selector.

## Stop Conditions

- Adoption needs a Finch/Longhorn/Tauri/public API decision.
- Install resolves duplicate or local Poodle.
- Lock churn touches archive or unrelated packages.
- Evidence requires launching the application.

## Evidence And Continuation

Record exact identities, Longhorn peer convergence, lock review, compatibility
edits, and validation in the Finch PR. Do not merge. Independent of `020`-`022`.
