# g04.014 Svelte Demo-App Rebuild, Component Adoption, And Coverage Upgrade

Status: completed
Owner: Pug Core
Updated: 2026-03-13
Depends on: g04.012, g04.013
Primary repos: `pug`

## Goals

- [x] rebuild or substantially upgrade the Svelte demo app so it becomes a
  credible parity target
- [x] make the Svelte demo demonstrate far more of the actual public surface
  through real primitive or composite usage instead of generic HTML or loose
  section glue

## Execution Checklist

- [x] implement the demo-app shell, screens, and region model from the new demo
  contract
- [x] replace ad hoc demo controls with the actual shared primitives,
  composites, and workstation components wherever the contract expects them
- [x] improve coverage for exports that are currently missing or only indirectly
  previewed
- [x] make the rebuilt Svelte demo coherent enough that side-by-side GPUI
  comparison is efficient and defects are obvious
- [x] avoid rebuilding the old preview mess with better paint

## Acceptance Criteria

- [x] rebuilt Svelte demo-app target is explicit and coherent
- [x] Svelte demo coverage is materially stronger than the current preview-only
  posture
- [x] Svelte demo directly previews the full public primitive surface
- [x] the repo is pointed at GPUI demo parity rather than straight to
  downstream proof

## Next Task

Open `g04.015` and implement the same demo app in GPUI with side-by-side review
against the rebuilt Svelte target.
