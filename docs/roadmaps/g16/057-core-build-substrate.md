# g16.057 — Core Build Substrate

Status: blocked — requires accepted `g16.056`
Type: implementation — build substrate
Opened: 2026-09-02
Depends on: accepted `g16.056`
Governing refs: `../../architecture/014-compiled-web-package-distribution.md`,
`056-web-distribution-contract.md`

## Goal

Implement the repo-owned compiled-distribution foundation for core: stable
JavaScript, CSS/token/icon assets, declarations, clean staging, sorted
inventories, source rejection, and deterministic build receipts. Do not compile
Svelte or React components in this card.

## Fixed Boundary

- Use a repo-owned driver around Vite library mode and separate TypeScript
  declaration emit. Public entries and chunk/asset templates are explicit and
  sorted.
- Preserve every current core entry and exact CSS/token subpath while rewriting
  targets to `dist`.
- Declarations use `emitDeclarationOnly`, `declarationMap: false`, and must
  resolve under Bundler and NodeNext.
- Stage only compiled JavaScript, declarations, CSS/assets, package docs and
  licences, and `dist/.poodle-build.json`.
- The receipt has no timestamp or absolute path and records locked tools,
  source commit, inventories, hashes, CSS/parser decisions, and source-map law.
- Do not change Svelte/React compilation, markdown public imports, versions,
  release notes, workflows, tags, registries, or sibling repositories.

## Ordered Work

1. Add the reusable clean-staging/build-driver foundation with stable entry,
   chunk, and asset naming.
2. Compile the full core JavaScript entry inventory and copy the exact
   CSS/token/icon/licensing assets required by the contract.
3. Emit core declarations and verify all export targets under Bundler and
   NodeNext without source mapping.
4. Fail closed on raw source, maps, missing targets, unsorted inventories,
   source/workspace paths, or unrelated parser dependencies.
5. Emit the deterministic build receipt and prove two clean builds have the
   same inventory and file hashes.

## Acceptance

- Core staged output is typed, source-free, CSS-complete, stable-name,
  deterministic, and receipt-backed.
- Every existing core public export, `./icons`, style wildcard, and token CSS
  target resolves inside `dist`.
- No `marked`, Svelte, React, raw source, source map, absolute path, timestamp,
  or workspace-only target enters core output.
- Two clean builds match file-for-file and hash-for-hash.
- The foundation is reusable by shell builds without owning shell semantics.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Stable output is explicit | Vite hashes one public filename | installed target check fails |
| Core CSS inventory is complete | one token/style subpath is absent | export/content test fails |
| Output is source-free | staged non-declaration `.ts` | source audit fails |
| Receipt is reproducible | timestamp enters JSON | second-build hash differs |
| Card stays core-only | Svelte component compilation appears | diff-scope check fails |

## Writable Scope

Core build/staging driver, core package manifest/export targets, core declaration
configuration, core asset-copy and receipt logic, focused build tests, this
card, one log, and new papercuts. Do not edit Svelte/React component builds,
the permanent installed certification harness, versions, release notes,
workflows, tags, registries, or sibling repositories.

## Validation

Run focused driver/unit tests, core declaration checks under Bundler and
NodeNext, two clean builds and inventory/hash comparison, core export/content
inspection, relevant Effigy core/build selectors, `effigy docs:check`, and
`git diff --check origin/main...HEAD`. No release or windowed selector.

## Stop Conditions

Stop if stable names require hand-edited output, declarations need source paths
or diagnostic suppression, CSS/assets cannot remain core-owned, raw source or
maps enter staging, receipts are nondeterministic, a shell change is required,
or any release/workflow/registry mutation appears.

## Continuation

Accepted merge unlocks `g16.058`. `g16.054` remains blocked.
