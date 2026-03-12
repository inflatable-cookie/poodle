# g02.012 Docs Site Examples And Component Discoverability Baseline

Status: completed
Date: 2026-03-11
Owner: Pug Core

## Summary

- completed `g02.012`
- turned the Svelte preview into the first catalog-style docs surface with
  section navigation, family grouping, example metadata, and explicit package
  and contract discoverability
- added a structured catalog registry at `packages/svelte/preview/src/catalog.ts`
  and wired it into the live surface in `packages/svelte/preview/src/App.svelte`
- added docs entry-point aliases at the repo root with `bun run docs:dev` and
  `bun run docs:build`
- added the normative discoverability baseline at
  `docs/specs/020-docs-site-example-and-component-discoverability-rules.md`

## Validation

- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche intentionally uses the existing preview as the first docs-site
  baseline instead of splitting effort across two thin browser surfaces
- the docs surface now answers the practical adopter questions directly:
  what layer owns this, what package implements it, and what example types are
  visible here

## Next Task

Open
`docs/roadmaps/g02/013-underlay-adoption-tranche-and-wrapper-preservation.md`
and use the new docs/example baseline to support the first real Underlay
adoption tranche.
