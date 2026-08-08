# Poodle Svelte Tokens

Public Svelte-facing token surface for emitted Poodle token artifacts.

## Public Surface

- root import: `@inflatable-cookie/poodle-core/tokens`
- runtime helpers: `@inflatable-cookie/poodle-core/tokens/runtime`
- CSS helper exports: `@inflatable-cookie/poodle-core/tokens/css`
- CSS asset export: `@inflatable-cookie/poodle-core/tokens/styles.css`
- legacy-compatible CSS asset export: `@inflatable-cookie/poodle-core/tokens/css/poodle-tokens.css`
- generated themes: `@inflatable-cookie/poodle-core/tokens/themes`
- generated metadata: `@inflatable-cookie/poodle-core/tokens/metadata`
- relative-unit helpers: `@inflatable-cookie/poodle-core/tokens/units`

## Stability Notes

- token values are generated from `packages/tokens/artifacts/` and mirrored into
  this package's `src/generated/` surface so sibling repos can install it
  directly without workspace-only path assumptions
- runtime helpers may set dataset attributes, but token naming remains owned by
  the canonical artifact set rather than ad hoc Svelte convenience APIs
- this package is the strongest current public candidate in the JS layer, but
  downstream adoption still remains gated until `g03`

## Next Task

Use this package surface while executing `g02.016`, confirming which token
entry points are stable enough to carry into the first downstream-adoption
generation.
