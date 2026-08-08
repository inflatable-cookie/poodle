# g12.001 Shared Styles Package

Status: complete (2026-07-13)
Owner: Poodle core
Depends on: g11.008 (all appearance CSS is recipe-hooked and class-addressed)

## Purpose

One styling source for every web framework. React parity would otherwise
duplicate 132 component stylesheets and drift. All component CSS moves to
`packages/styles` (`@inflatable-cookie/poodle-styles`): plain global CSS, unique `poodle-*`
class names, recipe hooks, zero framework coupling.

## Why this is safe

Proven by the god-files batch 5 extraction (13 components) and the React
pilot: Poodle styles use unique class names + data-attribute states, so
Svelte's scoping adds nothing; `:global(...)` unwraps to the same selector.
Playwright verified identical computed styles after extraction.

## Batches

- [x] 1. Create `packages/styles` (`@inflatable-cookie/poodle-styles`, css-only, no JS).
  Move the 13 already-extracted css files; rewrite Svelte imports.
- [x] 2. Extract the remaining 113 `<style>` blocks with the batch-5
  extractor (unwrap `:global`, kebab-case filenames), straight into
  `@inflatable-cookie/poodle-styles`; add the import line to each component.
- [x] 3. React pilot consumes `@inflatable-cookie/poodle-styles` (delete its css copies).
- [x] 4. Verify: svelte preview builds; Playwright computed-style sample
  across families; recipe-hook cascade still works; consumer typechecks
  (underlay, acme-admin, dairy) and one consumer visual smoke.
- [x] 5. Update architecture 002 (package layout) and recipe-inventory
  scanner source paths.

## Completion Notes (2026-07-13)

126 stylesheets now live in `@inflatable-cookie/poodle-styles`; every Svelte component and
the React pilot import from it. Found and fixed en route:

- **EditableList unstyled since the batch-5 extraction** — its `<style>`
  block was removed without the import line ever being added.
- **data-table.css leaked global element styles** (`table`, `td`,
  `thead th`, `tbody tr:hover` at top level since batch 5) — now scoped
  under `.poodle-data-table`. Package-wide scan shows zero unscoped
  selectors and zero `@keyframes` name collisions.
- Recipe-inventory scanner reads both the component sources and the shared
  styles package, with definition-shape facts accumulated per component
  across files (markup and stylesheet are now separate files).

Verified: svelte + react previews build; Playwright styling sample across
newly-extracted families green; recipe cascade live through the shared
css; underlay + acme-admin typecheck; acme-admin production build resolves
`@inflatable-cookie/poodle-styles` through the `file:` link (bun nests workspace deps inside
the snapshot).

## Non-Goals

- No selector rewrites, no CSS refactors — pure movement.
- GPUI/Jetstream unaffected (they resolve tokens numerically).
