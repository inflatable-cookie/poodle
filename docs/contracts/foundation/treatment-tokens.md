# Treatment Tokens

Status: detailed contract
Updated: 2026-03-30

- Layer: `foundation`
- Kind: cross-cutting system contract (not a rendered component)

## 1. Purpose

- Component name: `TreatmentTokens`
- Layer: `foundation`
- Summary: cross-cutting system contract for the treatment token layer —
  not a rendered component

Treatment tokens are an intermediate CSS custom-property layer that sits
between canonical semantic tokens and component CSS. They allow
downstream consumers to apply cohesive visual branding (gradients,
layered shadows, tinted borders) across entire component families
without redefining the meaning of semantic tokens.

### Why treatments exist

Semantic tokens are deliberately narrow-typed: a color token holds a
single color, a radius token holds a single length. This keeps the
token layer portable across renderers (Svelte, GPUI, Jetstream).
However, web-specific design effects like gradient backgrounds and
composite box-shadows cannot be expressed as a single color or length
value.

Treatment tokens bridge this gap:

```text
Semantic Tokens          Treatment Tokens          Component CSS
(narrow, typed,    --->  (grouped visual     --->  (resolves treatment
 cross-renderer)          overrides, may             with semantic
                          use gradients              fallback)
                          and shadows)
```

Components reference treatment variables through CSS custom-property
fallbacks. When a treatment is active, the treatment value is used.
When no treatment is set, the semantic token fallback provides the
default appearance. Components never need to know which specific
treatment is active.

## 2. Architecture

### Three-layer model

| Layer | Scope | Examples |
|-------|-------|---------|
| 1. Canonical semantic tokens | Typed, narrow, cross-renderer | `--poodle-color-background-surface`, `--poodle-radius-control` |
| 2. Treatment tokens | Grouped visual overrides per component family | `--poodle-treatment-interactive-fill`, `--poodle-treatment-surface-shadow` |
| 3. App-owned wrappers | Brand expression via composites and layout | `--poodle-recipe-page-header-fill`, `--brand-proof-accent` |

### Activation mechanism

Treatments are activated by setting a `data-appearance-treatment`
attribute on a container element. All descendant components inherit the
treatment values through the CSS cascade:

```html
<div data-appearance-treatment="brand-raised">
  <!-- All Poodle components inside inherit treatment values -->
</div>
```

A CSS rule block sets the treatment variables when the attribute is
present:

```css
[data-appearance-treatment="brand-raised"] {
  --poodle-treatment-interactive-fill:
    linear-gradient(180deg, rgba(255,255,255,0.18), rgba(255,255,255,0.02)),
    color-mix(in srgb, var(--poodle-color-background-elevated) 92%,
              var(--poodle-color-background-surface));
  /* ... */
}
```

### Fallback pattern

Every treatment token reference in component CSS must include a
semantic token fallback. This is the core contract rule:

```css
/* Correct: treatment with semantic fallback */
background: var(
  --poodle-treatment-interactive-subtle-fill,
  var(--poodle-color-background-surface)
);

/* Correct: treatment with explicit none fallback */
box-shadow: var(--poodle-treatment-interactive-shadow, none);

/* Wrong: treatment without fallback */
background: var(--poodle-treatment-interactive-fill);
```

## 3. Treatment Roles

Six family-level roles are defined. Components map to the role that
matches their interaction class rather than inventing per-component
treatment vocabularies.

### 3.1 interactive

General interactive surfaces: secondary buttons, toggles, menu
triggers, segmented controls.

| Token | Purpose |
|-------|---------|
| `--poodle-treatment-interactive-radius` | Border radius |
| `--poodle-treatment-interactive-fill` | Resting background |
| `--poodle-treatment-interactive-fill-active` | Hover/active background |
| `--poodle-treatment-interactive-border` | Resting border color |
| `--poodle-treatment-interactive-border-active` | Hover/active border color |
| `--poodle-treatment-interactive-shadow` | Resting shadow |
| `--poodle-treatment-interactive-shadow-active` | Hover/active shadow |

### 3.2 interactive-primary

Primary action buttons and prominent call-to-action surfaces.

| Token | Purpose |
|-------|---------|
| `--poodle-treatment-interactive-primary-radius` | Border radius |
| `--poodle-treatment-interactive-primary-fill` | Resting background |
| `--poodle-treatment-interactive-primary-fill-hover` | Hover background |
| `--poodle-treatment-interactive-primary-border` | Resting border color |
| `--poodle-treatment-interactive-primary-shadow` | Shadow |
| `--poodle-treatment-interactive-primary-text` | Text/icon color |

### 3.3 interactive-subtle

Text inputs, selects, search fields, comboboxes: controls with subtle
chrome.

| Token | Purpose |
|-------|---------|
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

### 3.4 surface

Panel backgrounds, card frames, and container surfaces.

| Token | Purpose |
|-------|---------|
| `--poodle-treatment-surface-radius` | Border radius |
| `--poodle-treatment-surface-fill` | Background |
| `--poodle-treatment-surface-border` | Border color |
| `--poodle-treatment-surface-shadow` | Shadow |
| `--poodle-treatment-surface-hover-fill` | Hover background |
| `--poodle-treatment-surface-hover-border` | Hover border |
| `--poodle-treatment-surface-hover-shadow` | Hover shadow |
| `--poodle-treatment-surface-header-fill` | Card/section header fill |
| `--poodle-treatment-surface-divider` | Internal divider color |

### 3.5 surface-elevated

Elevated surfaces: dialogs, drawers, popovers, menus, tooltips,
elevated cards.

| Token | Purpose |
|-------|---------|
| `--poodle-treatment-surface-elevated-radius` | Border radius |
| `--poodle-treatment-surface-elevated-fill` | Background |
| `--poodle-treatment-surface-elevated-border` | Border color |
| `--poodle-treatment-surface-elevated-shadow` | Shadow |

### 3.6 focus-ring (reserved)

Focus state treatment for keyboard navigation indicators. Currently
uses the accent token directly; reserved for future divergence where
treatment-level override of focus ring color, width, or style may be
needed.

No tokens are currently defined for this role.

## 4. Component Usage

### interactive role

| Component | Tokens consumed |
|-----------|----------------|
| Button (secondary/ghost variants) | radius, fill, fill-active, border, border-active, shadow, shadow-active |
| IconButton (secondary/ghost variants) | radius, fill, border, shadow, shadow-active |
| SplitButton (secondary action + trigger) | radius, fill, border, shadow |
| Toggle | radius, fill, fill (pressed), border, border-active, shadow |
| ToggleGroup (track) | radius, fill, fill (active item), border, border-active, shadow |
| SegmentedControl (track) | radius, fill, border, shadow |
| Tabs (card variant items) | radius, fill, border, shadow |

### interactive-primary role

| Component | Tokens consumed |
|-----------|----------------|
| Button (primary variant) | fill, fill-hover, border, shadow, text |
| IconButton (primary variant) | fill, border, shadow, text |
| SplitButton (primary variant) | fill, border, shadow, text |

### interactive-subtle role

| Component | Tokens consumed |
|-----------|----------------|
| TextInput | radius, fill, fill-focus, border, border-focus, shadow, shadow-focus |
| TextArea | radius, fill, fill-focus, border, border-focus, shadow, shadow-focus |
| SearchInput | radius (via calc), fill-hover |
| Select | radius, fill, fill-focus, border, shadow, shadow-focus |

### surface role

| Component | Tokens consumed |
|-----------|----------------|
| Surface (default) | radius, fill, border, shadow |
| Card (default/outlined) | radius, fill, shadow, hover-fill, hover-border, hover-shadow, divider |
| MetricTile (composite) | fill |

### surface-elevated role

| Component | Tokens consumed |
|-----------|----------------|
| Surface (elevated) | fill, border, shadow |
| Card (elevated) | radius, border, fill |
| Dialog | radius, fill, border, shadow |
| Drawer | fill, border, shadow |
| Popover | radius, fill, border, shadow |
| Menu (overlay) | radius, fill, border, shadow |
| Menubar (dropdown panel) | radius, fill, border, shadow |
| ContextMenu | radius, fill, border, shadow |
| HoverCard | radius, fill, border, shadow |
| Tooltip | radius, fill, border, shadow |
| SplitButton (menu part) | radius, fill, border, shadow |

## 5. Theme Integration

### Default treatment (no attribute)

When no `data-appearance-treatment` attribute is set, the treatment
tokens may still be defined at the app shell level to provide baseline
cosmetic values. Components fall back to their semantic token defaults
if no treatment value is set at all.

### brand-raised treatment

The built-in `brand-raised` treatment adds gradient fills, layered
inset-highlight shadows, and pronounced drop shadows to all component
families. It has a light-theme override block that adjusts shadow
opacity values for lighter backgrounds.

```css
.app-shell[data-appearance-treatment="brand-raised"] {
  /* Gradient fills, layered shadows for all roles */
}

.app-shell[data-appearance-treatment="brand-raised"][data-theme="light"] {
  /* Adjusted shadow opacity for light backgrounds */
}
```

### Custom treatments

Downstream apps create custom treatments by defining a CSS rule block
that sets treatment variables for every role they want to affect.
Unset variables fall through to semantic token defaults:

1. Define treatment variables for each role
2. Add per-theme overrides where needed (shadows look different on
   light vs dark backgrounds)
3. Scope the treatment via `data-appearance-treatment` attribute on a
   container element

### Theme-specific shadow adjustment

Shadow values often need per-theme tuning because the same shadow
opacity looks different against light and dark backgrounds. The
established pattern is:

```css
[data-appearance-treatment="my-treatment"] {
  --poodle-treatment-interactive-shadow:
    inset 0 0.0625rem 0 rgba(255, 255, 255, 0.1),
    0 0.125rem 0.375rem rgba(9, 13, 18, 0.1);
}

[data-appearance-treatment="my-treatment"][data-theme="light"] {
  --poodle-treatment-interactive-shadow:
    inset 0 0.0625rem 0 rgba(255, 255, 255, 0.48),
    0 0.125rem 0.375rem rgba(49, 66, 85, 0.06);
}
```

## 6. Accessibility

Treatment tokens are a visual-only layer and must not affect accessibility:

- Treatment overrides must not reduce contrast below WCAG AA minimums
  (4.5:1 for normal text, 3:1 for large text and UI components)
- Gradient fills used as button backgrounds must maintain sufficient
  contrast against the text color at all gradient stops
- Box-shadow treatments must not be the sole focus indicator — the
  standard focus ring (`--poodle-border-width-focus` outline) must
  remain functional regardless of active treatment
- Treatment tokens carry no ARIA semantics and must not be relied upon
  by assistive technology

## 7. Rules

1. **Token purity**: Semantic tokens must remain typed and narrow. Do
   not broaden a color token to hold a gradient.
2. **Family-level roles**: Prefer shared treatment roles over
   per-component treatment variables.
3. **Fallback chain**: Every treatment variable reference must include a
   semantic token fallback so the component renders correctly when no
   treatment is active.
4. **Gradient rule**: Gradients are valid treatment values, not
   canonical colors. They belong in treatment tokens, never in semantic
   tokens.
5. **Safe override boundary**: Downstream apps may scope treatment
   overrides to subtrees. They must not redefine semantic token meaning.
6. **Web-only scope**: Treatment tokens are a CSS-specific mechanism.
   GPUI and Jetstream implementations achieve equivalent visual
   variation through their native theming APIs, not through this
   custom-property layer.

## 8. Parity Checklist

### Svelte implementation

- [x] All 36 treatment tokens from the five active roles are consumed
- [x] Every component reference includes a semantic token fallback
- [x] Default treatment values defined on app shell root
- [x] `brand-raised` treatment fully defined with light-theme overrides
- [x] `data-appearance-treatment` attribute drives activation
- [x] Treatment section in preview app documents all roles and tokens

### GPUI implementation

- [ ] Treatment-equivalent visual variation achievable through theme API
- [ ] No hardcoded gradient or shadow values in component code

### Jetstream implementation

- [ ] Treatment-equivalent visual variation achievable through theme API
- [ ] No hardcoded gradient or shadow values in component code

### Contract coverage

- [x] All six treatment roles documented with token tables
- [x] All consuming components listed per role
- [x] Fallback pattern documented with correct/incorrect examples
- [x] Theme integration pattern documented
- [x] Custom treatment creation process documented
