# 050 GPUI Structural Primitives Baseline

Status: active
Updated: 2026-03-12
Depends on: `048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`, `049-gpui-theme-runtime-and-native-preview-app-baseline.md`

## Purpose

Freeze the first real GPUI runtime package beyond token bindings. This spec
defines the structural primitive baseline that later GPUI field, composite,
and workstation work must build on instead of re-inventing layout, surface,
separator, and scrolling semantics component by component.

## Package Rule

`flint-gpui-primitives` is now a preview-channel public-intent Rust crate.

Its `g04.003` baseline owns:

- `BoxSpec`
- `StackSpec`
- `GridSpec`
- `SurfaceSpec`
- `SeparatorSpec`
- `ScrollShellSpec`

These exports are the first contract-backed GPUI primitive surface after
`flint-gpui-tokens`.

## Contract Coverage Rule

The crate must stay aligned to the existing foundation contracts for:

- `box`
- `stack`
- `grid`
- `surface`
- `separator`
- `scroll-shell`

Later GPUI implementation tranches may deepen this surface, but they should
not redefine the meaning of these primitives outside the existing contracts.

## Runtime Honesty Rule

The current baseline is intentionally structural rather than overclaimed.

`g04.003` is allowed to expose contract-backed structural specifications and
token-resolution helpers before the repo contains mounted GPUI widget nodes for
every primitive. It is not allowed to imply that:

- mounted GPUI rendering parity already exists
- native accessibility-tree mapping is complete
- scroll viewport behavior is fully proved in runtime code

Those deeper proofs belong to later `g04` milestones.

## Token Rule

Structural GPUI primitives must resolve layout, surface, border, elevation, and
focus-treatment semantics from `flint-gpui-tokens`, not from local Rust-only
constants.

At minimum the baseline should freeze:

- padding and gap mapping
- surface background, border, and elevation mapping
- separator stroke and tone mapping
- scroll-shell focus treatment mapping

## Package Boundary Rule

`flint-gpui-primitives` now sits between token bindings and later GPUI runtime
depth:

- it is narrower than the future full GPUI primitive family
- it is broader than token-only bindings
- it should be treated as the structural contract substrate for `g04.004` and
  later GPUI tranches

## Known Deltas

The current structural baseline still allows these explicit deltas:

- the crate may expose structural specification types before mounted GPUI nodes
  exist in-repo
- native accessibility-node mapping remains follow-up work for later GPUI
  runtime implementation tranches
- scroll-shell behavior is frozen semantically here even though rendered
  viewport proof still belongs to later work

## Seed Evidence

- `packages/gpui/structural-primitives-baseline.json`
- `packages/gpui/primitives/Cargo.toml`
- `packages/gpui/primitives/README.md`
- `packages/gpui/primitives/src/lib.rs`
- `docs/contracts/foundation/box.md`
- `docs/contracts/foundation/stack.md`
- `docs/contracts/foundation/grid.md`
- `docs/contracts/foundation/surface.md`
- `docs/contracts/foundation/separator.md`
- `docs/contracts/foundation/scroll-shell.md`

## Next Task

Carry this structural GPUI baseline into `g04.004`, adding action, text-entry,
and field primitives on top of the same contract and token posture.
