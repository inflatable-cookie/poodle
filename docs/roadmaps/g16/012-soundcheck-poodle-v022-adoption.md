# g16.012 — Soundcheck Poodle 0.2.2 adoption

Status: **ready — independent first-wave product lane**
Depends on: `g16.007`, `g16.008`, `g16.010`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/soundcheck`
Target base: `ddec155b0963d8dc0a6aba2ed9774c826cdc375c`
Governing refs: `001-consumer-adoption-inventory.md`,
`008-longhorn-poodle-v022-adoption.md`,
`010-soundcheck-library-poodle-v022-follow-up.md`, Soundcheck `AGENTS.md` and
working rules

## Outcome

Move Soundcheck from committed sibling Poodle sources and registry 0.1.0
declarations to exact public Poodle 0.2.2. Preserve its local Longhorn and
Soundcheck Library integration while proving all three dependency layers agree
on the corrected Poodle release.

## Scope

- Pin root Poodle core and Svelte dependencies to exact `0.2.2`.
- Remove only the committed Poodle core/Svelte `file:` overrides. Keep the
  Longhorn and Soundcheck Library local development dependencies unchanged.
- Regenerate `bun.lock` without unrelated upgrades and confirm the local
  Longhorn/Soundcheck Library peers resolve Poodle Svelte 0.2.2.
- Repair only Soundcheck-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not publish or version Soundcheck, Longhorn, or Soundcheck Library.
- Do not replace local Longhorn/Soundcheck Library links, change plugin-library
  behavior, alter the Tauri agent-control feature, or touch product APIs.
- Do not edit Poodle or sibling repositories, add compatibility shims, or run
  the visible Tauri application.

## Acceptance

- Soundcheck resolves public Poodle core/Svelte 0.2.2 with registry integrity;
  no active 0.1.0, 0.2.1, or sibling Poodle path remains.
- Local Longhorn and Soundcheck Library packages resolve the same Poodle Svelte
  0.2.2 instance through their updated peer lines.
- The lockfile diff contains no unrelated dependency upgrade.
- Soundcheck build, test, relevant integration checks, docs checks, and broad
  headless `effigy qa` pass, or any pre-existing failure is reproduced on the
  target base and separated from the adoption result.

## Validation

- Install from declared sources using the repository's pinned Bun version;
  inspect the effective dependency graph and full lock diff.
- Run `effigy build`, `effigy qa`, and any narrower headless selector exposed
  by `effigy tasks` for Longhorn or library integration.
- Run `git diff --check`. Do not run dev, Tauri, assistant proof, or another
  selector that launches or focuses the application.

## Stop Conditions

- Adoption needs a public product, Longhorn, Soundcheck Library, Poodle, or
  Tauri contract decision.
- Installation resolves duplicate Poodle versions or a sibling Poodle source.
- Lock regeneration materially changes unrelated dependencies.
- Validation exposes a Poodle 0.2.2 release defect or requires a visible run.

## Evidence And Continuation

Record resolved versions/integrities, peer convergence, changed files,
compatibility edits, baseline comparison, lock review, and exact validation in
the Soundcheck PR. Do not merge. Once this lane lands, Soundcheck is complete
for g16 and its three-layer evidence informs the remaining Longhorn products.
