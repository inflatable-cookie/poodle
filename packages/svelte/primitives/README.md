# Pug Svelte Primitives

First Svelte implementation surface for contract-backed Pug primitives.

## Public Surface

- `Banner`
- `Checkbox`
- `Field`
- `TextInput`
- `SearchField`
- `FormActions`
- `Skeleton`
- root import: `@pug/svelte-primitives`
- type-only import: `@pug/svelte-primitives/types`

## Purpose

- prove the `g02.001` forms and validation baseline against real Svelte code
- give the preview app a reusable package surface instead of inline demo-only
  controls
- keep semantics and token usage aligned with the docs-first contracts

## Stability Notes

- public entry points are the package root and `./types`
- preview-specific helpers and demo glue do not belong in this package
- additions should follow contract coverage, not demo convenience
- parity maturity is still incomplete outside the shipped Svelte surface

## Next Task

Use this package surface while executing `g02.016`, confirming which primitive
entry points are stable enough to carry into the first downstream-adoption
generation.
