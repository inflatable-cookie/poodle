# Public Package Surface Documentation

Poodle remains `strict-ready`. Public package metadata and packed export maps
were sound, but the operator documentation surface had one uncovered group.

## Findings

- Publint accepted `packages/core/package.json`,
  `packages/svelte/components/package.json`, and
  `packages/react/components/package.json` without warnings.
- The packed-install proof and manifest lint already cover tarball contents,
  concrete export targets, clean consumer installation, and runtime mounting.
- `packages/svelte/components/scripts/surface-audit.ts` found 10 exported
  components without entries in
  `packages/svelte/preview/src/component-docs.ts`: `Radio` and nine agent,
  transcript, tool-call, or changed-file components.
- The surface audit was available as a standalone Effigy task but was absent
  from both `docs:check` and `ci:web`.

## Repaired

- Added complete prop tables and runnable usage examples for all 10 missing
  components.
- Regenerated `packages/react/preview/artifacts/component-docs.json`, keeping
  React's operator docs on the same canonical source.
- Added `svelte:surface-audit` to the broad documentation and web CI gates.

## Validated

- `bunx publint` for all three public-intent npm packages
- `effigy svelte:surface-audit`
- `effigy docs:check`
- `effigy test:web-pack-install`
- `effigy ci:web`
- `git diff --check`
