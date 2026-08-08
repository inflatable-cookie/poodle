# g11 Workstation Substrate Generation Opened

Date: 2026-03-17
Area: roadmaps, workstation

## Summary

Opened `g11` as the next active Pug generation. The new generation is focused
on generalized workstation substrate depth for real downstream workstation
applications: window hosts, region snapshots, strip rails, resize/collapse
affordances, richer docks, panel variants, and hosted external surfaces.

## Why This Exists

Recent downstream audit work in Loophole made the current Pug gap concrete.
Pug already has useful workstation primitives, but it still undersupplies the
shared substrate required by serious multi-region application shells. The new
generation turns that into a bounded program instead of an open-ended wishlist.

## Key Decisions

- `g11` is active immediately
- the generation stays generalized and explicitly avoids DAW-specific widgets
  or product shell semantics
- Svelte and GPUI are both in scope, with Svelte implementation landing first
  and GPUI following on the same documented contract
- downstream proof is required before closeout so the generation does not end
  as a purely internal library exercise

## Surfaces Updated

- `docs/roadmaps/README.md`
- `docs/roadmaps/generation-index.md`
- `docs/roadmaps/g11/README.md`
- `docs/roadmaps/g11/001-workstation-downstream-gap-audit-and-generalized-target-shape-freeze.md`
- `docs/roadmaps/g11/002-workspace-window-host-model-and-surface-ownership-contract.md`
- `docs/roadmaps/g11/003-region-grammar-and-layout-snapshot-expansion.md`
- `docs/roadmaps/g11/004-strip-rail-family-and-orientation-variants.md`
- `docs/roadmaps/g11/005-resize-handles-split-dividers-and-collapse-affordances.md`
- `docs/roadmaps/g11/006-dock-region-depth-collapsed-posture-and-active-panel-emphasis.md`
- `docs/roadmaps/g11/007-window-aware-surface-tabs-and-panel-tab-orchestration.md`
- `docs/roadmaps/g11/008-panel-variants-and-utility-versus-focused-surface-treatments.md`
- `docs/roadmaps/g11/009-hosted-external-surface-and-plugin-editor-container-contract.md`
- `docs/roadmaps/g11/010-svelte-workstation-implementation-batch-1-windows-regions-strips.md`
- `docs/roadmaps/g11/011-svelte-workstation-implementation-batch-2-docks-tabs-panels-and-hosted-surfaces.md`
- `docs/roadmaps/g11/012-gpui-workstation-implementation-batch-1-windows-regions-strips.md`
- `docs/roadmaps/g11/013-gpui-workstation-implementation-batch-2-docks-tabs-panels-and-hosted-surfaces.md`
- `docs/roadmaps/g11/014-docs-specimens-parity-evidence-and-downstream-reference-adoption-proof.md`
- `docs/roadmaps/g11/015-generation-closeout-and-next-program-cutover.md`

## Next

Execute `g11.001` and turn the current downstream workstation pressure into one
explicit, generalized Pug target shape before any implementation milestone
opens.
