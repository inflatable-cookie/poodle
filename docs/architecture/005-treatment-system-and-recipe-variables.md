# 005 Treatment System And Recipe Variables

Status: active
Updated: 2026-03-22
Depends on: `002-token-system-and-package-layout.md`
Reference: `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md`

## Purpose

Document the treatment system that sits between canonical semantic tokens and
app-owned wrappers. Treatments let downstream consumers apply cohesive visual
branding across component families without redefining token meaning.

## Three-Layer Architecture

Poodle styling operates in three layers, applied in order:

1. **Canonical semantic tokens** — typed, narrow values (color, spacing, radius).
   Never broadened to hold gradients, textures, or web-only effects.
2. **Appearance recipes and treatment roles** — grouped visual overrides scoped
   to component families. May include web-only effects like gradients and layered
   shadows.
3. **App-owned wrappers and composites** — structural brand expression built by
   composing Poodle primitives.

## Treatment Roles

Six family-level treatment roles are defined. Components map these into local
CSS aliases rather than inventing per-component treatment vocabularies.

### `interactive`

General interactive surfaces — secondary buttons, toggles, menu triggers.

| Variable | Purpose |
|---|---|
| `--poodle-treatment-interactive-radius` | Border radius |
| `--poodle-treatment-interactive-fill` | Resting background |
| `--poodle-treatment-interactive-fill-active` | Hover/active background |
| `--poodle-treatment-interactive-border` | Resting border color |
| `--poodle-treatment-interactive-border-active` | Hover/active border color |
| `--poodle-treatment-interactive-shadow` | Resting shadow |
| `--poodle-treatment-interactive-shadow-active` | Hover/active shadow |

### `interactive-primary`

Primary action buttons and prominent call-to-action surfaces.

| Variable | Purpose |
|---|---|
| `--poodle-treatment-interactive-primary-radius` | Border radius |
| `--poodle-treatment-interactive-primary-fill` | Resting background |
| `--poodle-treatment-interactive-primary-fill-hover` | Hover background |
| `--poodle-treatment-interactive-primary-border` | Resting border color |
| `--poodle-treatment-interactive-primary-shadow` | Shadow |
| `--poodle-treatment-interactive-primary-text` | Text/icon color |

### `interactive-subtle`

Text inputs, selects, search fields, text areas — controls with subtle chrome.

| Variable | Purpose |
|---|---|
| `--poodle-treatment-interactive-subtle-radius` | Border radius |
| `--poodle-treatment-interactive-subtle-fill` | Resting background |
| `--poodle-treatment-interactive-subtle-fill-hover` | Hover background |
| `--poodle-treatment-interactive-subtle-fill-focus` | Focus background |
| `--poodle-treatment-interactive-subtle-border` | Resting border |
| `--poodle-treatment-interactive-subtle-border-hover` | Hover border |
| `--poodle-treatment-interactive-subtle-border-focus` | Focus border |
| `--poodle-treatment-interactive-subtle-shadow` | Resting shadow |
| `--poodle-treatment-interactive-subtle-shadow-hover` | Hover shadow |
| `--poodle-treatment-interactive-subtle-shadow-focus` | Focus shadow |

### `surface`

Panel backgrounds, card frames, and container surfaces.

| Variable | Purpose |
|---|---|
| `--poodle-treatment-surface-radius` | Border radius |
| `--poodle-treatment-surface-fill` | Background |
| `--poodle-treatment-surface-border` | Border color |
| `--poodle-treatment-surface-shadow` | Shadow |
| `--poodle-treatment-surface-hover-fill` | Hover background |
| `--poodle-treatment-surface-hover-border` | Hover border |
| `--poodle-treatment-surface-hover-shadow` | Hover shadow |
| `--poodle-treatment-surface-header-fill` | Card/section header fill |
| `--poodle-treatment-surface-divider` | Internal divider color |

### `surface-elevated`

Elevated surfaces — dialogs, drawers, popovers, elevated cards.

| Variable | Purpose |
|---|---|
| `--poodle-treatment-surface-elevated-radius` | Border radius |
| `--poodle-treatment-surface-elevated-fill` | Background |
| `--poodle-treatment-surface-elevated-border` | Border color |
| `--poodle-treatment-surface-elevated-shadow` | Shadow |

### `focus-ring`

Focus state treatment for keyboard navigation indicators. Currently handled
through the existing `--poodle-color-accent-focusRing` token rather than a
separate treatment role. Reserved for future use if focus styling needs to
diverge from accent color.

## How Components Consume Treatments

Components reference treatment variables using CSS custom property fallbacks.
The treatment variable is tried first; if undefined, the semantic token value
is used.

```css
/* In a component's <style> block */
.text-input {
  --poodle-text-input-fill: var(
    --poodle-treatment-interactive-subtle-fill,
    var(--poodle-color-background-surface)
  );
  background: var(--poodle-text-input-fill);
}
```

This pattern ensures:

- When no treatment is active, components render with standard token values.
- When a treatment is set, the treatment value takes precedence.
- Components never need to know which specific treatment is active.

## Component–Role Mapping

| Treatment Role | Components |
|---|---|
| `interactive` | Button (secondary/ghost), IconButton (secondary), SplitButton, Button (pressed), ToggleGroup, SegmentedControl (track), Tabs (card variant items) |
| `interactive-primary` | Button (primary), SplitButton (primary), IconButton (primary) |
| `interactive-subtle` | TextInput, TextArea, SearchField, Select |
| `surface` | Surface, Card (default/outlined), MetricTile |
| `surface-elevated` | Surface (elevated), Card (elevated), Dialog, Drawer, Popover, Menu (overlay), HoverCard, Tooltip, SplitButton (menu) |
| `focus-ring` | (reserved — currently uses accent token directly) |

## How to Apply Treatments

Treatments are applied by setting CSS custom properties on a container element.
All descendants inherit the treatment values through the CSS cascade.

### Via data attribute (recommended)

```html
<div class="app-shell" data-appearance-treatment="brand-raised">
  <!-- All Poodle components inside inherit treatment values -->
</div>
```

```css
.app-shell[data-appearance-treatment="brand-raised"] {
  --poodle-treatment-interactive-fill: linear-gradient(...), var(--poodle-color-background-elevated);
  --poodle-treatment-interactive-primary-fill: linear-gradient(...), var(--poodle-color-accent-base);
  --poodle-treatment-surface-fill: linear-gradient(...), var(--poodle-color-background-panel);
  /* ... all other treatment variables */
}
```

### Via scoped class

Treatments can also be scoped to a subtree for local branding:

```css
.brand-proof-scope {
  --poodle-treatment-surface-fill: /* branded value */;
  --poodle-treatment-surface-border: /* branded value */;
  --poodle-treatment-interactive-primary-fill: /* branded value */;
}
```

## Creating a New Treatment

To define a new treatment (e.g., `brand-glass`):

### 1. Define treatment variables

Create a CSS rule block that sets all treatment variables. You must define
values for every variable in the roles you want to affect. Unset variables
will fall through to semantic token defaults.

```css
[data-appearance-treatment="brand-glass"] {
  /* Interactive role */
  --poodle-treatment-interactive-fill: /* your value */;
  --poodle-treatment-interactive-border: /* your value */;
  --poodle-treatment-interactive-shadow: /* your value */;
  /* ... repeat for all roles and states */

  /* Surface role */
  --poodle-treatment-surface-fill: /* your value */;
  --poodle-treatment-surface-border: /* your value */;
  --poodle-treatment-surface-shadow: /* your value */;
  /* ... */
}
```

### 2. Add theme-specific adjustments (optional)

Treatments may need per-theme overrides, especially for shadows which look
different on light versus dark backgrounds:

```css
[data-appearance-treatment="brand-glass"][data-theme="light"] {
  --poodle-treatment-interactive-shadow: /* lighter shadow */;
  --poodle-treatment-surface-shadow: /* lighter shadow */;
}
```

### 3. Register in the preview app (for development)

Add the treatment to the display controls in the preview app:

```typescript
// DisplayControls.svelte
const appearanceTreatmentOptions: ToggleGroupOption[] = [
  { value: "system", label: "system" },
  { value: "brand-raised", label: "brand-raised" },
  { value: "brand-glass", label: "brand-glass" },
];
```

And update the type in `App.svelte`:

```typescript
type AppearanceTreatmentName = "system" | "brand-raised" | "brand-glass";
```

## Extension Lanes

### Cross-Runtime Lane

Use for overrides that should remain part of the shared contract and be
plausible for GPUI parity:

- treatment roles
- appearance recipe variables
- stable component-part overrides

### Web-Only Lane

Use for inherently browser-specific overrides:

- gradients
- textures and backdrop-filter
- layered shadows
- web-specific motion or polish

Web-only effects must be scoped above the shared contract and must not
redefine canonical token meaning.

## Rules

1. **Token purity** — semantic tokens must remain typed and narrow. Do not
   broaden a color token to hold a gradient.
2. **Family-level roles** — prefer shared treatment roles over per-component
   treatment variables. Do not invent a new vocabulary for every component.
3. **Fallback chain** — every treatment variable reference must include a
   semantic token fallback so components render correctly with no treatment.
4. **Gradient rule** — gradients are valid appearance treatments, not canonical
   colors. Use treatment roles for 3D or raised effects.
5. **Safe override boundary** — downstream apps may scope recipe overrides to
   subtrees and define reusable treatments. They must not redefine semantic
   token meaning or depend on undocumented internal selectors.

## Evidence

- `packages/svelte/components/src/Button.svelte` — interactive and interactive-primary
- `packages/svelte/components/src/IconButton.svelte` — interactive (secondary) and interactive-primary
- `packages/svelte/components/src/SplitButton.svelte` — interactive, interactive-primary, surface-elevated (menu)
- `packages/svelte/components/src/ToggleGroup.svelte` — interactive
- `packages/svelte/components/src/SegmentedControl.svelte` — interactive (track)
- `packages/svelte/components/src/Tabs.svelte` — interactive (card variant items)
- `packages/svelte/components/src/TextInput.svelte` — interactive-subtle (reference pattern)
- `packages/svelte/components/src/TextArea.svelte` — interactive-subtle
- `packages/svelte/components/src/Select.svelte` — interactive-subtle
- `packages/svelte/components/src/Card.svelte` — surface and surface-elevated
- `packages/svelte/components/src/Surface.svelte` — surface and surface-elevated
- `packages/svelte/components/src/Dialog.svelte` — surface-elevated
- `packages/svelte/components/src/Popover.svelte` — surface-elevated
- `packages/svelte/components/src/Drawer.svelte` — surface-elevated
- `packages/svelte/components/src/Menu.svelte` — surface-elevated (overlay)
- `packages/svelte/components/src/HoverCard.svelte` — surface-elevated
- `packages/svelte/components/src/Tooltip.svelte` — surface-elevated
- `packages/svelte/preview/src/app.css` — treatment variable definitions and brand-raised
