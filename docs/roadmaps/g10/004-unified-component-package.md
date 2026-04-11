# g10.004 Unified Component Package

Status: active
Owner: Poodle core
Depends on: g10.003
Updated: 2026-04-11

## Context

The primitive/composite distinction creates an artificial binary where many
components sit on a boundary. Tags are more useful for consumers than a
two-tier hierarchy. Both Svelte packages are private with no external
consumers, making this a safe restructuring.

## Goals

- merge @poodle/svelte-primitives and @poodle/svelte-composites into @poodle/svelte
- merge docs/contracts/components/ and docs/contracts/components/ into docs/contracts/components/
- unify preview app into a single Components section with tag-based filtering
- merge Rust contract crates (packages/contracts/primitives and composites)
- update all imports, registry, parity, lint expectations

## Non-Goals

- changing the token package
- changing bridges
- changing GPUI/Jetstream component file structure (follows Rust crate boundaries, addressed after Rust merge)

## Execution Plan

### Batch 4.1 — Merge Svelte Packages

- [ ] create packages/svelte/components/ with merged package.json as @poodle/svelte
- [ ] move all .svelte files from primitives/src/ and composites/src/
- [ ] merge index.ts exports and types.ts
- [ ] update all specimen imports to @poodle/svelte
- [ ] update preview package.json dependency
- [ ] remove old primitives/ and composites/ packages

### Batch 4.2 — Merge Contract Directories

- [x] create docs/contracts/components/
- [x] move all contracts from foundation/ and composites/
- [x] update contract READMEs and seed list
- [x] update lint-docs.ts expectations

### Batch 4.3 — Unify Preview App

- [ ] merge PrimitivesSection and CompositesSection into ComponentsSection
- [ ] add tags to component-registry.ts entries
- [ ] add tag-based filtering to sidebar
- [ ] remove Primitives/Composites tabs from top nav
- [ ] update parity tooling to single component list

### Batch 4.4 — Merge Rust Contract Crates

- [ ] merge packages/contracts/primitives/ and packages/contracts/components/ into packages/contracts/components/
- [ ] update Cargo.toml workspace references
- [ ] update GPUI and Jetstream adapter/component imports
- [ ] verify cargo check passes

### Batch 4.5 — Verification

- [ ] run effigy health and resolve failures
- [ ] run svelte-check — zero errors
- [ ] run cargo check — all crates compile
- [ ] update architecture docs (001, 002)
- [ ] update CLAUDE.md references

## Exit Criteria

- single @poodle/svelte package with all components
- single docs/contracts/components/ directory
- single Components section in preview with tag filtering
- single Rust contract crate
- effigy health passes
- svelte-check zero errors
- cargo check passes

## Next Task

Execute Batch 4.1: merge Svelte packages.
