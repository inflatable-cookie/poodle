# g12 — React Full Parity And Web Verification Depth

**Status: active.** The React parity program (`001`–`008`) is complete;
the generation continues with cross-framework verification depth.

**React parity: COMPLETE.** All 132 components ported to `@poodle/react` and
Playwright-verified against the Svelte preview. The React preview is a full
per-component gallery matching the Svelte preview (shell, controls, Tokens
inspector, usage docs, 131/131 specimen slugs) plus docs / parity /
accessibility report generators. `@poodle/react` has a consumer README
(`packages/react/components/README.md`). Docs and parity data are authored
canonically in the Svelte preview and re-exported live into React — no fork.

Goal: `@poodle/react` grows from the 3-component pilot (g11.007) to full
library parity (132 components), as a strategic second web target — no
consuming app yet, so the acceptance bar is the React preview plus
interaction verification, not a consumer rollout.

Decisions inherited from g11.007:

- Hand-written TSX shells (~90 LOC each) over `@poodle/headless` machines.
  Mitosis rejected twice (spec 062); no compiler layer.
- Tokens, recipes, and the contrast axis are plain CSS — shared unchanged.
- The Svelte implementation remains the visual proof reference.

Runway:

- `001-shared-styles-package.md` — extract every remaining Svelte
  `<style>` block and move all component CSS to `@poodle/styles`, imported
  by both frameworks. Single styling source; React never duplicates CSS.
- `002-react-infra-and-conversion-playbook.md` — React preview harness
  (hash-routed specimens like the Svelte preview), shared type strategy,
  documented conversion recipe.
- `003`–`008` — family batches: primitives/display, controls/forms,
  overlays/navigation, layout/form shells, data/date, media/workstation
  composites.
- `008-parity-verification.md` — preview coverage sweep, Playwright
  behavior parity samples against the Svelte preview, docs. **Complete.**
- `009-visual-regression-gate.md` — cross-framework pixel diff (Svelte vs
  React, same slug, same axes). The structural gates cannot see a
  component render at the wrong size; the ListCard `data-size` bug proved
  it. **Complete** — 256/256 sweep pairs and 180/180 axis pairs green,
  after 14 divergences found and fixed across specimens, shells and
  `@poodle/styles`.
- `010-agent-composer.md` — the agent composer family across all four
  targets: `Meter shape="ring"`, `ModelPicker` (model + host-declared
  capability axes in one popover) and `AgentChatInput`. **Complete.**

- Ongoing: `check:svelte` (svelte-check over `@poodle/svelte`, driven
  through the isolated `install-smoke` consumer) now runs in `ci:web` and
  `ci-web.yml`. Added after five type errors in `Rating.svelte` were found
  from a consumer repo rather than from Poodle's own gates.

## Next Task

`g12.009` is complete. Pick the next seam for this generation — the
visual gate now guards every web-side change, so the open ground is the
Rust targets (GPUI/Jetstream have not tracked g12-era Svelte changes) or
deeper `@poodle/headless` machine coverage.
