# Pug Svelte Tokens

Public Svelte-facing token surface for emitted Pug token artifacts.

## Public Surface

- root import: `@pug/svelte-tokens`
- runtime helpers: `@pug/svelte-tokens/runtime`
- CSS helper exports: `@pug/svelte-tokens/css`
- generated themes: `@pug/svelte-tokens/themes`
- generated metadata: `@pug/svelte-tokens/metadata`
- relative-unit helpers: `@pug/svelte-tokens/units`

## Stability Notes

- token values come from generated artifacts under `packages/tokens/artifacts/`
- runtime helpers may set dataset attributes, but token naming remains owned by
  the canonical artifact set rather than ad hoc Svelte convenience APIs
- this package is the strongest current public candidate in the JS layer, but
  downstream adoption still remains gated until `g03`

## Next Task

Use this package surface while executing `g02.016`, confirming which token
entry points are stable enough to carry into the first downstream-adoption
generation.
