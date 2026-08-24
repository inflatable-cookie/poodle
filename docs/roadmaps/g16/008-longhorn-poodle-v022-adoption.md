# g16.008 — Longhorn Poodle 0.2.2 adoption

Status: **ready — fresh worker lane; do not resume the stopped 0.2.1 branch**
Depends on: `g16.001`, `g16.005`, `g16.007`, published npm `0.2.2`,
immutable tag `v0.2.2`
Target repository: `/Users/tom/Dev/projects/longhorn`
Target base: `4780e7fadbeae40fc88b31a02146a0e83cac8677`
Governing refs: `001-consumer-adoption-inventory.md`,
`005-gpui-cratesio-recovery.md`, `007-v022-release-certification.md`,
Longhorn `AGENTS.md`, Longhorn contracts 012 and 013

## Outcome

Move every active Longhorn Poodle dependency to the corrected public 0.2.2
release. Prove that Longhorn's direct crates.io GPUI dependency and Poodle's
GPUI graph resolve one compatible crate identity, then validate the web
adapter, examples, Rust adapter, and native prototypes without opening a
visible application window.

## Scope

- Pin root Poodle core and Svelte development dependencies to exact `0.2.2`.
- Move `@inflatable-cookie/longhorn-poodle-svelte`'s exact Poodle Svelte peer
  from `0.1.0` to `0.2.2`.
- Move every active Longhorn proof/example Poodle core or Svelte dependency
  from exact `0.1.0` to exact `0.2.2`.
- Move `longhorn-poodle`'s `poodle-specs` requirement and git tag together to
  version `0.2.2` / tag `v0.2.2`.
- Move both GPUI prototypes' direct Poodle git tags together to `v0.2.2`.
- Regenerate the root Bun/Cargo locks and both prototype Cargo locks without
  unrelated upgrades.
- Repair only compatibility failures exposed by the 0.2.2 adoption. Update
  Longhorn-owned tests or current integration guidance when the public package
  or Rust surface proves it stale.

## Required Source-Identity Proof

- All Rust Poodle packages resolve from tag `v0.2.2` at
  `d5607def24c6833913df1b5dcfa06372fcd5dd81`.
- `gpui` and `gpui_platform` resolve from crates.io, not a Poodle/Zed fork,
  patch, replace, path, or second git source.
- Each active Rust graph contains one GPUI 0.2.2 crate identity. The direct
  prototype path and the transitive Poodle path accept the same Poodle spec
  types without conversion or aliases.
- No committed Poodle path, patch, or source override is introduced.

## Out Of Scope

- Do not bump, tag, or publish Longhorn.
- Do not edit Poodle, weaken a peer, change Longhorn wire/public contracts, or
  add compatibility aliases, source patches, or duplicate-type conversion.
- Do not rewrite historical evidence or old release checkpoints merely because
  they mention 0.1.0.
- Do not launch the GPUI prototypes or any visible/focus-taking application.

## Acceptance

- No active Longhorn manifest or lock resolution uses Poodle 0.1.0 or 0.2.1.
- Every active web consumer resolves public npm core/Svelte 0.2.2; the Svelte
  adapter peer is exact 0.2.2.
- Every direct Rust Poodle dependency uses tag `v0.2.2`, and every lock records
  the accepted candidate SHA.
- Root and prototype Rust graphs prove one crates.io GPUI 0.2.2 identity with
  no fork source.
- A clean Bun install resolves registry packages rather than a sibling Poodle
  checkout.
- Longhorn's package, binding, boundary, proof/example, Rust, prototype, docs,
  and broad headless QA surfaces pass.

## Validation

- Inspect the effective web and Rust dependency graphs after lock regeneration.
- Run `effigy check:bindings`, `effigy check:packages`,
  `effigy check:consumer-isolation`, `effigy check:prototypes`, and the
  Poodle-related proof selectors exposed by `effigy tasks`.
- Run `effigy qa` as the broad headless board. Do not run prototype binaries or
  any selector that creates or activates a window.
- Run `git diff --check` and inspect every lockfile diff for unrelated churn.

## Stop Conditions

- Adoption still produces two GPUI crate identities or a fork-sourced GPUI.
- A required fix changes a Longhorn public/wire contract, Poodle, or a sibling
  consumer.
- Lock regeneration materially updates unrelated dependencies.
- Validation requires a visible application run or reveals a Poodle 0.2.2
  release defect rather than a bounded Longhorn migration.

## Evidence And Continuation

Record changed manifests and locks, exact resolved package/tag/SHA/source
identity, compatibility edits, and exact validation in the Longhorn PR. Do not
merge. Once this PR lands, the orchestrator may compile Jetstream plus the
Longhorn-dependent product cards.
