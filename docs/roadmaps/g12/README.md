# g12 — React Full Parity

**Status: COMPLETE.** All 132 components ported to `@poodle/react` and
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
- `009-parity-verification-and-closeout.md` — preview coverage sweep,
  Playwright behavior parity samples against the Svelte preview, docs.
