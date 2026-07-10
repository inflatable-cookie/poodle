# 007 Appearance Recipe Contract

Status: active
Updated: 2026-07-10
Depends on: `005-treatment-system-and-recipe-variables.md`, `006-headless-core-and-machine-model.md`
Promoted from: `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md` (g11.005)

## What A Recipe Is

A recipe is the set of **component-scoped CSS custom properties** a component
resolves its appearance from. Recipes are the third override depth in the
sanctioned ladder (tokens → treatment roles → per-component recipe
variables), and the mechanism apps use to restyle individual components
without forking them or redefining shared meaning.

Decision (g11.005): recipes are **CSS custom-property contracts, not a
JavaScript API**. They are plain data by nature — a variable name and a
value — which keeps them framework-free, SSR-safe, zero-runtime, and
mappable to the Rust runtimes (each public recipe variable corresponds to a
spec field / token override on the GPUI/Jetstream side).

## Naming And Resolution

- Public variable form: `--poodle-recipe-<component>[-<variant>]-<slot>[-<state>]`
  (e.g. `--poodle-recipe-button-fill-hover`,
  `--poodle-recipe-button-primary-fill`, `--poodle-recipe-pill-border`).
- The `--poodle-recipe-*` namespace is **read-only for components**: they
  consume it but never define values in it, so an app-scoped value always
  wins through the cascade. (Component-local `--poodle-<component>-*`
  variables are internal resolution variables — components define them, so
  ancestor overrides cannot reach them. This distinction was verified live
  in g11.005.)
- Components resolve each internal appearance variable with the chain:
  **recipe hook → treatment role variable → semantic token.** With no
  override active, rendering is identical to the token default — recipes
  are strictly additive.
- Variant-specific rules get variant-scoped hooks (e.g.
  `-button-primary-*`); semantic tone rules (danger/success) keep token
  semantics unless a variant hook is added on demand.
- Slots correspond to the component's contract anatomy parts; states to its
  documented visual states.

## Public Surface vs Internal Variables

Two classes, machine-classified by
`packages/svelte/preview/scripts/build-recipe-inventory.ts` into
`packages/svelte/preview/artifacts/recipe-inventory.json`:

- **Recipe hooks** (`--poodle-recipe-*`, public): the supported override
  surface. Stable: renaming or removing one is a breaking change handled
  via the g11.001 wave process. Coverage is **library-wide** as of g11.008:
  every appearance property in every component resolves through a hook
  (973 hooks across 116 scanned components, `candidates` empty). Two
  mechanical shapes: components with local appearance variables wrap the
  variable definition; components without them hook inline at the property.
  Slot names come from the component's anatomy class parts; qualifiers from
  data-attributes and pseudo-state in the defining selector.
- **Metric variables** (internal): sizing, padding, gaps, font sizes. These
  belong to the size/density system and are NOT part of the recipe
  contract. Overriding them is unsupported (density must never change
  component height — see the size/density contract).

Regenerate the inventory after styling changes:
`bun packages/svelte/preview/scripts/build-recipe-inventory.ts`.

## How Apps Override

Scope overrides to a subtree; the CSS cascade delivers them. No imports, no
registration.

```css
/* app-owned stylesheet */
.checkout-flow {
  /* family-level: treatment role */
  --poodle-treatment-interactive-primary-fill: linear-gradient(#7c5cff, #5533ff);

  /* component-level: recipe variables */
  --poodle-button-shadow: 0 2px 0 rgb(0 0 0 / 0.35);
  --poodle-pill-fill: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
}
```

Rules (unchanged from spec 026): never redefine semantic token meaning in
place; web-only effects (gradients, backdrop-filter) are valid recipe values
but do not imply cross-runtime parity; structural brand expression belongs
in app-owned wrappers, not recipes.

## Part Selectors

Machine-backed components additionally emit
`data-scope` / `data-part` / `data-state` attributes (architecture 006).
These are a **read-only styling hook** for app CSS where a recipe variable
does not exist yet — stable in name, but prefer recipe variables when
available; new recipe variables are added on demonstrated need rather than
exposing every internal property.

## Growing The Surface

When an app needs an appearance property that has no recipe variable, the
correct move is to add the variable to the component (with the standard
fallback chain) and update its contract — not to target internal selectors.
Additions are non-breaking.
