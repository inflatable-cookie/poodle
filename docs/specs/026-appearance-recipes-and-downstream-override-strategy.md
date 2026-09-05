# 026 Appearance Recipes And Downstream Override Strategy

Status: promoted — productized as `docs/architecture/007-appearance-recipe-contract.md` (g11.005); this spec remains the strategy rationale
Updated: 2026-07-10
Depends on: `archive/023-svelte-visual-hierarchy-and-contrast-baseline.md`, `archive/024-token-evolution-migration-and-compatibility-policy.md`, `025-parity-automation-and-harness-boundary.md`

## Purpose

Freeze how downstream consumers may restyle Poodle components without redefining
canonical token meaning or forcing Poodle to expose its entire CSS surface as
foundational tokens.

## Core Rule

Poodle styling now operates in three layers:

- canonical semantic tokens
- public component-scoped appearance Recipes
- app-owned wrappers and composites

Downstream branding should move downward through those layers in order. It must
not jump straight to redefining token meaning.

## Token Purity Rule

Foundational and semantic tokens stay typed and narrow.

Examples:

- a color token must remain a color
- a spacing token must remain spacing
- a radius token must remain radius

Do not broaden token meaning just to satisfy a web-only styling trick such as:

- gradients
- texture overlays
- gloss or bevel effects
- backdrop-filter presentation

If a brand appearance needs those, it belongs in the Recipe layer,
not the canonical token layer.

## Appearance Recipe Rule

Components may expose a small, stable set of recipe variables for their public
parts and states.

These recipe variables should:

- map cleanly from semantic tokens by default
- stay much smaller than the full internal CSS surface
- describe stable parts and states rather than implementation selectors
- remain mappable to GPUI or other runtimes later where possible

Examples of acceptable recipe concerns:

- fill
- border
- text/icon tone
- shadow
- radius
- hover or active appearance

Examples of unacceptable recipe exposure:

- arbitrary internal gap values for every nested element
- implementation-only wrapper selectors
- DOM-shape assumptions consumers must target manually

## Component Scope Rule

Public overrides use `--poodle-recipe-<component>-*` hooks. Each hook names a
stable component part, variant, or state and falls back directly to a semantic
token or documented formula. Family-level intermediate roles are retired.

## Extension Lane Rule

Two extension lanes are allowed:

### Cross-Runtime Lane

Use this when the override should remain part of the shared contract:

- appearance recipe variables
- stable component-part overrides
- variant-level appearance changes

This lane should stay plausible for GPUI parity work later.

### Web-Only Lane

Use this when the override is inherently browser-specific:

- gradients
- textures
- layered shadows
- backdrop-filter
- web-specific motion or polish

This lane is acceptable only when it is scoped above the shared contract and
does not redefine canonical meaning.

## Wrapper Rule

When a consumer needs structural brand expression rather than recipe-level
restyling, the correct answer is an app-owned wrapper or composite.

Examples include:

- marketing CTAs
- pricing cards
- branded hero actions
- campaign-specific layouts

These should compose Poodle primitives and composites instead of forcing Poodle core
to become a marketing-site kit.

## Safe Override Boundary

Downstream apps may:

- scope recipe-variable overrides to a subtree
- define reusable app-owned brand variables
- map those variables into component Recipe hooks consistently
- create app-owned branded wrappers above Poodle primitives

Downstream apps must not:

- redefine semantic token meaning in place
- depend on undocumented internal selectors as the primary styling contract
- assume browser-only effects imply cross-runtime parity
- claim canonical parity from a web-only branded skin

## Gradient Rule

Gradients are valid Recipe values, not canonical colors.

If a downstream app wants consistent 3D or raised appearance across clickable
surfaces, it should point the relevant component Recipe hooks at shared
app-owned variables rather than converting a color token into a gradient.

## Seed `g03.005` Baseline

The current seed implementation proves this strategy on the Svelte side by:

- defining scoped component Recipe overrides in the preview surface
- mapping component Recipe hooks into buttons, tabs, and text-entry controls
- mapping component Recipe hooks into cards, panel framing, and page-header
  presentation
- showing one branded website-style wrapper that consumes recipe variables
  without redefining semantic tokens
- keeping the base semantic token set unchanged

## Evidence

- `packages/svelte/components/src/TextInput.svelte`
- `packages/svelte/components/src/Card.svelte`
- `packages/svelte/components/src/PageHeader.svelte`
- `packages/svelte/workstation/src/PanelTabs.svelte`
- `packages/svelte/workstation/src/PanelSurface.svelte`
- `packages/svelte/workstation/src/SurfaceTabs.svelte`
- `packages/svelte/preview/src/app.css`
- `packages/svelte/preview/src/App.svelte`
