# g15.066 — Soundcheck Poodle 0.2.2 adoption

Status: **complete — Soundcheck PR 11 merged at `b1c5937d`**
Depends on: `g15.061`, `g15.062`, `g15.064`, published npm `0.2.2`
Target repository: `/Users/tom/Dev/projects/soundcheck`
Target base: `ddec155b0963d8dc0a6aba2ed9774c826cdc375c`
Governing refs: `055-consumer-adoption-inventory.md`,
`062-longhorn-poodle-v022-adoption.md`,
`064-soundcheck-library-poodle-v022-follow-up.md`, Soundcheck `AGENTS.md` and
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
for g15 and its three-layer evidence informs the remaining Longhorn products.

## Closeout

Soundcheck PR [#11](https://github.com/inflatable-cookie/soundcheck/pull/11)
merged on 2026-08-24 at
`b1c5937dd774e711b78d4d5eef7b274e8d47b41d`. The app now pins public Poodle
core/Svelte 0.2.2 exactly, carries no committed sibling Poodle override or Vite
alias, and preserves its intentional local Longhorn and Soundcheck Library
links. Their peers converge on the same Svelte 0.2.2 identity. Registry
integrities match the published npm records and the lock refresh contains no
unrelated upgrade.

Independent review passed the Rust workspace build, the headless docs/Northstar
board, frozen install, dependency and lock inspection, and `git diff --check`.
The existing frontend baseline reproduced unchanged on target base and PR head:
three failed and nine passed Vitest files with 46 passing tests, 90 Svelte
errors, and the same Vite failure resolving local Longhorn command sources.
Doctor's generated-source and god-file scan baseline also reproduced on both
heads. The canonical verdict is
[recorded on PR 11](https://github.com/inflatable-cookie/soundcheck/pull/11#issuecomment-5402418314).
