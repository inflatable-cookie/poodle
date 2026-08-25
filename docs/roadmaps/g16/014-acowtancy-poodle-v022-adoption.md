# g16.014 — Acowtancy Underlay 0.9.2 and Poodle 0.2.2 adoption

Status: **complete — PRs 54, 55, and 56 merged**
Depends on: `g16.007`, `g16.009`, `g16.013`, Underlay tag `v0.9.2`
Target repository: `/Users/tom/Dev/projects/acowtancy`
Target base: `a535969bab2a1b919d382e3e129d8eb95043b2b8`
Governing refs: `001-consumer-adoption-inventory.md`,
`013-underlay-reference-poodle-v022-adoption.md`, Acowtancy root and scoped
`AGENTS.md`, Acowtancy working rules, and Froyo package contract

## Outcome

Move Acowtancy's web and Rust Underlay dependencies from `v0.9.1` to
`v0.9.2`, then move Cream, Dairy, and Froyo from Poodle 0.1.x to public 0.2.2.
Preserve shared-package boundaries and the single root web lock.

## Scope

- Pin Cream and Dairy core/Svelte dependencies to exact `0.2.2`.
- Move Froyo's Svelte peer from `^0.1.0` to `^0.2.2`.
- Move every active JavaScript Underlay dependency to
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.2`.
- Move every active Farmyard Underlay crate to
  `ssh://git@github.com/inflatable-cookie/underlay.git`, tag `v0.9.2`, without
  changing its existing feature set.
- Regenerate root `bun.lock` and Rust locks narrowly. Prove one Poodle runtime
  identity and one Underlay source revision.
- Repair only Acowtancy-owned compatibility failures caused by the bump.

## Out Of Scope

- Do not edit Underlay or Poodle, change Farmyard/API behavior, or change
  product semantics beyond dependency compatibility repairs.
- Do not add sibling Poodle overrides, compatibility aliases, or package-local
  lockfiles.
- Do not launch the live stack or another visible application.

## Acceptance

- Cream and Dairy resolve exact registry core/Svelte 0.2.2.
- Froyo advertises `^0.2.2`; all consumers converge on one Svelte instance.
- Every active Underlay web and Rust dependency resolves tag `v0.9.2`
  (`ddba26400f480638829917cf72eecc62be4b978d`), with no active `v0.9.1` or
  sibling Underlay path.
- No active old Poodle version or sibling Poodle source remains.
- Lock churn is limited to the Underlay/Poodle upgrade and mechanically
  required metadata.

## Validation

- Use `effigy workspace:js:prepare`; inspect the dependency graph and full lock
  diff.
- Run `effigy cream/validate`, `effigy dairy/validate`, `effigy froyo/check`,
  `effigy validate`, and `effigy qa`.
- Run `git diff --check`.

## Stop Conditions

- Adoption needs an Underlay, Froyo, public API, or product decision.
- Install resolves duplicate Poodle versions or a local Poodle checkout.
- Lock regeneration changes unrelated packages.
- Required evidence needs the live or visible app stack.

## Evidence And Continuation

Record the exact Underlay tag/commit, Poodle versions, peer convergence, lock
review, compatibility edits, and validation in the Acowtancy PR. Do not merge.
This lane is independent of `015`-`022` and `025`.

## Post-Merge Correction

Acowtancy PR [#54](https://github.com/acowtancy/market/pull/54) merged the web
Poodle/Underlay manifests at `6e76b943`; PR
[#55](https://github.com/acowtancy/market/pull/55) merged the Rust Underlay tag
at `03ef5736`. The declared versions and locks are correct, but Cream and Dairy
still alias Underlay imports to `../underlay/ts/src` through SvelteKit and Vite,
while Froyo and Cattle Grid retain equivalent TypeScript paths. Those mappings
are active application sources, so the tagged web package is not yet the code
under validation.

The corrective PR removes only those application compilation aliases and path
mappings. Keep explicit sibling mounts and scripts used solely by workspace
conformance or template QA. Regenerate the root lock only if supported tooling
changes it, and reject unrelated churn. Prove Cream, Dairy, Froyo, and Cattle
Grid validate against the installed `v0.9.2` package before closing this card.

PR [#56](https://github.com/acowtancy/market/pull/56) removed the final eight
application-source bypasses and merged at `1ab977f8`. Cream, Dairy, Froyo, and
Cattle Grid now resolve Underlay through the installed `v0.9.2` package at
`ddba2640`; explicit sibling paths survive only in conformance and guardrail QA.
The root lock did not change. Direct Poodle remains registry 0.2.2 with one
Svelte identity.
