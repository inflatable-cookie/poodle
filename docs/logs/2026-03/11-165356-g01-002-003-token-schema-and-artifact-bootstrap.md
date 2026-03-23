# 2026-03-11 g01.002 And g01.003 Token Schema And Artifact Bootstrap

## Changed

- added the first real `packages/` workspace surface to Flint with:
  - `packages/tokens/` for schema, scripts, and generated artifacts
  - `packages/svelte/tokens/` as the first browser/Svelte consumer stub
  - `packages/gpui/tokens/` as the first Rust/GPUI consumer stub
  - `packages/bridges/underlay/` as the reserved Underlay-owned bridge surface
- added a first DTCG-shaped schema slice under `packages/tokens/schema/` across
  `primitives/`, `semantic/`, `modes/`, `metadata/`, and `manifest.json`
- covered the required bootstrap token families: color, typography, spacing,
  sizing, radius, border, elevation, motion, density, icon, overlay, and state
- added named themes for `light`, `dark`, and `loophole-studio`, plus density
  and control-size overlays
- tuned `loophole-studio` toward the real downstream shell surfaces seen in
  Aura and Spark first: dark workstation canvases, panel chrome, elevated
  surfaces, readable neutral text, and accent/focus roles that map cleanly to
  shell primitives without baking DAW-specific widgets into Flint core
- added a bootstrap artifact emitter at
  `packages/tokens/scripts/build-tokens.ts`
- generated initial CSS, TypeScript, and Rust artifacts under
  `packages/tokens/artifacts/`
- aligned the emitted shapes to the documented contract surface so Svelte and
  GPUI consumers can start from one source of truth instead of hand-maintained
  duplicate token constants
- updated the active `g01` roadmap statuses so the token tranche now closes in
  the roadmap itself rather than only in a log note

## Validation

- `bun packages/tokens/scripts/build-tokens.ts`
- `git diff --check`

## Remaining

- define the component contract template and docs IA in `g01.004`
- define the Svelte substrate and Bits posture in `g01.005`
- define the GPUI substrate and Rust token-binding posture in `g01.006`

## Next Task

Open `docs/roadmaps/g01/004-component-contract-template-and-documentation-ia.md`
and turn the now-live token taxonomy into the canonical per-component contract
template plus first primitive/composite examples.
