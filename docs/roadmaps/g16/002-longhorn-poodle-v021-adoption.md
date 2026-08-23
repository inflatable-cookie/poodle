# g16.002 — Longhorn Poodle 0.2.1 Adoption

Status: **ready**
Depends on: `g16.001`, published npm `0.2.1`, immutable tag `v0.2.1`
Target repository: `/Users/tom/Dev/projects/longhorn`
Governing refs: `README.md`, `../../../README.md`,
`../../logs/2026-08/20260823-g15-013-v021-release-certification.md`,
Longhorn `AGENTS.md`, Longhorn contract 012

## Outcome

Make Longhorn's Poodle integration consume the published web packages and the
matching Rust tag. Preserve Longhorn/Poodle ownership boundaries and prove the
adapter and examples against one coherent `0.2.1` source.

## Scope

- Set root Poodle web development dependencies to exact `0.2.1`.
- Move `@inflatable-cookie/longhorn-poodle-svelte`'s exact Poodle Svelte peer
  from `0.1.0` to `0.2.1`.
- Move active Longhorn example consumers from exact `0.1.0` to exact
  `0.2.1`.
- Move `longhorn-poodle`'s `poodle-specs` requirement and tag together to
  version `0.2.1` / tag `v0.2.1`.
- Move both GPUI prototypes' direct Poodle git tags together to `v0.2.1`.
- Regenerate Bun and Cargo locks without unrelated upgrades.
- Repair only compatibility issues exposed by Poodle `0.2.1`; update
  Longhorn-owned tests/docs where observable integration guidance changed.

## Out Of Scope

- Do not bump or publish Longhorn itself.
- Do not change Poodle, weaken a peer requirement, add aliases, or introduce a
  local Poodle patch/override.
- Do not rewrite historical release fixtures or translation memos merely
  because they mention `0.1.0`.

## Acceptance

- No active Longhorn manifest or lock resolution uses Poodle `0.1.0`.
- All direct Rust Poodle git dependencies use the same `v0.2.1` source.
- The Poodle Svelte adapter peer is exact `0.2.1`.
- A clean install resolves public npm packages, not a sibling Poodle tree.
- Longhorn's relevant package, boundary, example, Rust, docs, and broad
  headless QA selectors pass.

## Stop Conditions

- Poodle `0.2.1` requires a public Longhorn API or wire-contract decision.
- Lock regeneration changes unrelated registry packages materially.
- A test failure reveals a Poodle release defect rather than a bounded Longhorn
  migration.
- Validation would require a visible/focus-taking application run.

## Evidence

Record changed manifests/locks, resolved Poodle versions and sources, exact
selectors, and any bounded migration in the Longhorn PR. Do not merge.
