---
title: g04.003 gpui structural primitives baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, gpui, rust, primitives]
---

## Summary

Completed `g04.003` by landing the first real GPUI runtime crate after token
bindings, freezing the structural primitive baseline for layout, surface,
separator, and scrolling semantics.

## What changed

- added the normative baseline `docs/specs/050-gpui-structural-primitives-baseline.md`
- completed `docs/roadmaps/g04/003-gpui-layout-surface-scrolling-and-structural-primitives.md`
- added the machine-readable artifact `packages/gpui/structural-primitives-baseline.json`
- added the preview-channel Rust crate `packages/gpui/primitives` with:
  - `BoxSpec`
  - `StackSpec`
  - `GridSpec`
  - `SurfaceSpec`
  - `SeparatorSpec`
  - `ScrollShellSpec`
- bound the new crate to `pug-gpui-tokens` for spacing, surface, border,
  elevation, and focus-treatment mapping
- added crate tests that pin the initial structural defaults and token
  resolution behavior
- promoted `pug-gpui-primitives` into `packages/release-manifest.json` and
  `packages/release-operations.json`
- extended `packages/svelte/preview/scripts/lint-docs.ts` so the GPUI
  structural baseline artifact is machine-checked
- updated package and roadmap surfaces so the repo now points at `g04.004`

## Validation

- `cargo check --manifest-path packages/gpui/primitives/Cargo.toml`
- `bun run --cwd packages/svelte/preview docs:lint`
- `bun run --cwd packages/svelte/preview build`
- `git diff --check`

## Outcome

`g04.003` is now explicit. Pug has a real GPUI structural primitive crate that
later GPUI runtime work can build on, and the repo now treats that crate as a
public-intent preview package instead of leaving GPUI as mostly token-only
evidence.

## Next

Open `g04.004` and implement the GPUI action, text-entry, and field primitive
tranche on top of the new structural GPUI baseline.
