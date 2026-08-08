# Composites And Workstation Surface Lint Hardening

Status: completed
Date: 2026-03-12
Owner: Pug Core

## Summary

- extended package-surface lint hardening beyond primitives to
  `@pug/svelte-composites` and `@pug/svelte-workstation`
- made workstation root helper exports explicit in the workstation README so
  the documented public surface matches the actual package root
- generalized the docs completeness baseline so public Svelte package surfaces
  are machine-checked against contracts and README inventories

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Risks

- the package-surface hardening is still Svelte-only; GPUI package ergonomics
  and parity discipline remain a separate problem
- the docs catalog still works at family and section granularity rather than
  one explicit preview specimen per export

## Next Task

If hardening continues, tighten parity evidence against the now-stable package
surface, or deliberately stop docs-gate expansion and move into manual parity
review cleanup.
