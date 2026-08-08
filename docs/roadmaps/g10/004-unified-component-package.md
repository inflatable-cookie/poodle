# g10.004 Unified Component Package

Status: complete
Owner: Poodle core
Depends on: g10.003
Updated: 2026-04-11

## Context

The primitive/composite distinction creates an artificial binary where many
components sit on a boundary. Tags are more useful for consumers than a
two-tier hierarchy. Both Svelte packages are private with no external
consumers, making this a safe restructuring.

## Goals

- merge the legacy split Svelte packages into `@inflatable-cookie/poodle-svelte`
- consolidate contracts under `docs/contracts/components/`
- unify preview app into a single Components section with tag-based filtering
- merge Rust contract crates (packages/contracts/primitives and composites)
- update all imports, registry, parity, lint expectations

## Non-Goals

- changing the token package
- changing bridges
- changing GPUI/Jetstream component file structure (follows Rust crate boundaries, addressed after Rust merge)

## Execution Plan

### Batch 4.1 — Merge Svelte Packages

- [x] create packages/svelte/components/ with merged package.json as @inflatable-cookie/poodle-svelte
- [x] move all .svelte files from primitives/src/ and composites/src/
- [x] merge index.ts exports and types.ts
- [x] update all specimen imports to @inflatable-cookie/poodle-svelte
- [x] update preview package.json dependency
- [x] remove old primitives/ and composites/ packages

### Batch 4.2 — Merge Contract Directories

- [x] create docs/contracts/components/
- [x] move all contracts from foundation/ and composites/
- [x] update contract READMEs and seed list
- [x] update lint-docs.ts expectations

### Batch 4.3 — Unify Preview App

- [x] merge PrimitivesSection and CompositesSection into ComponentsSection
- [x] remove Primitives/Composites tabs from top nav
- [x] update parity tooling to single component list

### Batch 4.4 — Merge Rust Contract Crates

- [x] merge packages/contracts/primitives/ and packages/contracts/composites/ into poodle-specs
- [x] update Cargo.toml workspace references
- [x] update GPUI and Jetstream adapter/component imports
- [x] verify cargo check passes

### Batch 4.5 — Verification

- [x] run effigy health — 129 contracts validated, build succeeds
- [x] run svelte-check — zero errors
- [x] run cargo check — all crates compile
- [x] update CLAUDE.md references

## Exit Criteria

- single @inflatable-cookie/poodle-svelte package with all components
- single docs/contracts/components/ directory
- single Components section in preview with tag filtering
- single Rust contract crate
- effigy health passes
- svelte-check zero errors
- cargo check passes

## Next Task

`g10.004` is complete. The unified component package is live.
