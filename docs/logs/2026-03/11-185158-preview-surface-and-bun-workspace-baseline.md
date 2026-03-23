# Preview Surface And Bun Workspace Baseline

Status: completed
Date: 2026-03-11
Owner: Poodle Core

## Summary

- added a real browser inspection surface at `packages/svelte/preview`
- wired the preview to emitted Poodle token CSS artifacts and the Svelte token
  runtime package
- added root Bun scripts for preview build and local development
- corrected the root Bun workspace list so only JavaScript packages are treated
  as workspaces
- promoted `applyThemeAttributes` from the Svelte token runtime into the public
  package surface so preview and downstream consumers can use it directly

## Validation

- `bun install`
- `bun run tokens:build`
- `bun run preview:build`
- `git diff --check`

## Notes

- this is an inspection harness, not the full docs site planned in `g02.012`
- the current surface focuses on token visibility, theme overlays, density,
  control size, and accessibility-relevant native control states

## Next Task

Implement the first real Svelte primitives in `g02.001` so the preview can move
from token inspection into contract-backed forms and validation behavior.
