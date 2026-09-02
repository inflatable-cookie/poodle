# g16.056 — Web Distribution Contract

Status: complete — awaiting orchestrator review
Type: implementation — contract and migration boundary
Opened: 2026-09-02
Depends on: accepted [compiled web package distribution](../../architecture/014-compiled-web-package-distribution.md)
Governing refs: `../../architecture/014-compiled-web-package-distribution.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../contracts/001-working-rules.md`

## Goal

Freeze the complete compiled web distribution contract before build work:
package inventories, stable names, Svelte client/server conditions, declaration
shape, Svelte floor, CSS and parser ownership, receipt schema, source-free
archive law, and the accepted `./markdown` breaking migration.

## Fixed Boundary

- Core and Svelte are the public release set. React is compiled and certified
  but stays private and unpublished.
- `dist/` is the only code boundary. Raw `.svelte`, non-declaration
  `.ts`/`.tsx`, maps, and `src/` are forbidden.
- Svelte root/direct/markdown exports use `browser` for client and `default` for
  server. `import` is absent as an environment selector. `./types` retains
  `dist/types.js` plus `dist/types.d.ts`.
- Svelte peer floor is `>=5.56.8 <6`. `marked: ^18.0.9` is optional and required
  only for `./markdown`.
- Core owns CSS. All three packages use `sideEffects: ["**/*.css"]`.
- `AgentMessage` and `MarkdownEditor` leave shell root barrels for
  `./markdown`. No compatibility alias or fallback is allowed.
- This card does not build packages, change versions, edit `0.3.0` release
  notes/history, tag, publish, dispatch workflows, or mutate registries.

## Ordered Work

1. Document the exact core, Svelte, and private React `dist` inventories,
   declaration suffixes, files arrays, CSS subpaths, and dependency ownership.
2. Freeze sorted public entry inventories and stable output/chunk naming rules.
3. Freeze the Svelte client/server export map and the core single-lane map,
   including every existing token/icon/style target.
4. Record the deterministic `.poodle-build.json` and installed receipt schemas.
5. Update scoped package/roster and migration documentation to name the root
   markdown break and one canonical public denominator. Leave actual release
   notes and candidate history to `g16.054`.
6. Compile exact review oracles and writable scopes for `g16.057`–`g16.059`.

## Acceptance

- No export, SSR, floor, CSS, parser, declaration, tarball, receipt, roster, or
  dependency choice remains open.
- The condition map states browser → client and non-browser default → server;
  worker-like/unknown SSR is covered explicitly.
- Every export target has an intended compiled JavaScript and declaration
  target. `./types` preserves both reachability contracts.
- Package contents, forbidden source forms, deterministic provenance, exact
  CSS side effects, and optional `marked` behavior are normative.
- Migration documentation names the breaking root-to-`./markdown` move without
  adding a shim or editing `0.3.0` release notes.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| SSR choice is explicit | Svelte export uses `import` before `default` | contract review rejects it |
| Public type/runtime reachability is complete | `./types` is declarations-only | target inventory fails |
| Source-free boundary is exact | wildcard could expose `.svelte` | forbidden-content matrix fails |
| Parser remains isolated | root graph owns `marked` | dependency table fails |
| Release authority stays separate | card edits `0.3.0` notes | scope check fails |

## Writable Scope

Architecture/package contracts, package/export inventory documentation,
migration and roster documentation, this card, one execution log, and new
papercuts. Do not edit build scripts, package outputs, versions, release notes,
candidate history, workflows, tags, registries, sibling repositories, or
component behavior.

## Validation

Run focused docs drift/link checks, `effigy docs:check`, and `git diff --check
origin/main...HEAD`. Inspect all three package manifests and current public
inventories. No release or windowed selector is authorized.

## Stop Conditions

Stop on an unresolved export/SSR/floor/CSS/dependency choice, requested source
fallback or compatibility alias, ambiguous roster denominator, release-history
edit, workflow/release mutation, or a contract that cannot express both
browser and server Svelte artifacts.

## Continuation

Accepted merge unlocks `g16.057`. It does not unblock `g16.054`; only completed
installed certification in `g16.059` does that.

## Evidence

Exact inventories, export maps, receipts, markdown migration, and successor
oracles: `docs/specs/070-compiled-web-distribution-contract.md`. Architecture
014 points at that spec and records the React single-lane map. Spec 022 names
core+Svelte as the web public set. The g15 roster keeps the 176-name
denominator through the root markdown break. Execution log:
`docs/logs/2026-09/20260902-g16-056-web-distribution-contract.md`.
