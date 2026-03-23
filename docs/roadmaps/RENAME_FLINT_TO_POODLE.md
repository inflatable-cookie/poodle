# Rename: Flint -> Poodle

## Context

This project was renamed from `pug` to `flint`, then renamed again to `poodle`.
The rename is a straight namespace and repository swap with no intended product or architecture change.

The project now lives at `/Users/betterthanclay/Dev/projects/poodle`.
The GitHub repository is `inflatable-cookie/poodle`.

## Scope

- repository folder: `flint` -> `poodle`
- Git remote: `inflatable-cookie/flint` -> `inflatable-cookie/poodle`
- Rust crates: `flint-*` -> `poodle-*`
- Rust module paths: `flint_*` -> `poodle_*`
- npm packages: `@flint/*` -> `@poodle/*`
- token and CSS asset prefixes: `flint-*` / `--flint-*` -> `poodle-*` / `--poodle-*`
- docs, specs, roadmaps, and metadata references: `Flint` -> `Poodle`

## Validation

- `bun install`
- `bun packages/tokens/scripts/build-tokens.ts`
- `bun run --cwd packages/svelte/preview build`
- `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
- `cargo check --manifest-path packages/jetstream/preview/Cargo.toml`
- `effigy health`

## Notes

- The rename is intentionally mechanical. Any surviving `flint` references after validation should be treated as cleanup defects.
- Downstream repositories that consume this project will need a follow-up dependency and import-path update batch.

## Next Task

Update dependent repositories to consume `poodle` package scopes, crate names, and local paths.
