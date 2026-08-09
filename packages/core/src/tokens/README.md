# Poodle Svelte Tokens

Public Svelte-facing token surface for emitted Poodle token artifacts.

## Public Surface

- root import: `@inflatable-cookie/poodle-core/tokens`
- runtime helpers: `@inflatable-cookie/poodle-core/tokens/runtime`
- CSS helper exports: `@inflatable-cookie/poodle-core/tokens/css`
- CSS asset export: `@inflatable-cookie/poodle-core/tokens/styles.css`
- aggregate theme CSS: `@inflatable-cookie/poodle-core/tokens/themes.css`
- individual theme CSS: `@inflatable-cookie/poodle-core/tokens/theme-<name>.css`
- density CSS: `@inflatable-cookie/poodle-core/tokens/density-<mode>.css`
- control-size CSS: `@inflatable-cookie/poodle-core/tokens/control-size-<size>.css`
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
- this is a pre-1.0 source-preview surface; public registry publication has not
  started

The canonical token schema lives in `packages/tokens/schema/`. Rebuild all
generated targets with `effigy tokens:build` rather than editing mirrored
artifacts here.
