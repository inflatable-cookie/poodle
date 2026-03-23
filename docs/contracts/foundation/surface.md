# Surface

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Surface`
- Layer: `foundation`
- Summary: a semantic visual container providing background tiers, border
  treatments, elevation, and optional region semantics
- In scope: background tones (panel, canvas, elevated), border presence and
  emphasis, elevation shadow, interior padding, optional ARIA region/group
  semantics
- Out of scope: panel-header chrome, dock behavior, scroll ownership, toolbar
  layout

## 2. Anatomy

```text
[Root .surface]  <div>
  └── [Children...] (slot)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | background and boundary container | background fill, border, radius, elevation shadow |
| Children | no | arbitrary slotted content | caller-owned |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `tone` | `SurfaceTone: "panel" \| "canvas" \| "elevated"` | `"panel"` | no | semantic surface background tier |
| `border` | `SurfaceBorder: "subtle" \| "default" \| "none"` | `"subtle"` | no | shell boundary emphasis level |
| `padding` | `SpaceScale: "none" \| "sm" \| "md" \| "lg"` | `"md"` | no | interior spacing via scaleToSpace utility |
| `isElevated` | `boolean` | `false` | no | applies elevated fill and shadow regardless of tone |
| `asRole` | `"region" \| "group" \| null` | `null` | no | semantic role opt-in |
| `label` | `string \| null` | `null` | no | accessible label; required when asRole="region" and no visible heading |

### Slots

| Slot | Purpose |
|------|---------|
| default | arbitrary child content |

### Controlled And Uncontrolled

- no controlled value model

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| panel (default) | `tone="panel"` | panel-tier background with subtle border |
| canvas | `tone="canvas"` | canvas-tier background |
| elevated | `tone="elevated"` or `isElevated=true` | elevated background, elevation shadow |
| border-none | `border="none"` | border becomes transparent |
| border-default | `border="default"` | stronger border using default border color |
| border-subtle | `border="subtle"` (default) | low-emphasis mixed border |

### Component States

No internal state.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| none | n/a | n/a | container only |

## 6. Accessibility

### Semantics

- Role: none by default; `group` or `region` only by explicit `asRole` opt-in
- Required attributes: `aria-label` (from `label` prop) when `asRole="region"`
  and no visible heading is associated externally
- Optional attributes: `aria-labelledby`, `aria-describedby` via native
  equivalents
- Labeling rules: decorative surfaces (no asRole) must stay
  accessibility-neutral and not appear as landmarks

### Keyboard

| Key | Behavior |
|-----|----------|
| none | no intrinsic keyboard behavior |

### Focus And Announcement

- focus entry: surface itself is not focusable
- focus exit: n/a
- live-region behavior: none

## 7. Layout

### Sizing

- follows parent constraints and child content
- `min-width: 0` and `min-height: 0` prevent flex overflow
- does not own scroll behavior (pair with ScrollShell for scrolling)

### Composition

- parent expectations: any layout primitive, shell region, or page container
- child expectations: arbitrary content, nested primitives, composite
  components
- resizing rules: visual chrome (border, radius, shadow) scales with size but
  does not change semantics

## 8. Token Usage — Exact Values

### CSS Custom Properties (on .surface)

| Var | Default Value |
|-----|---------------|
| `--poodle-surface-fill` | `color-mix(in srgb, var(--poodle-color-background-surface) 96%, transparent)` |
| `--poodle-surface` | `var(--poodle-surface-fill)` — propagates surface context to descendants (see [surface-elevation](./surface-elevation.md)) |
| `--poodle-surface-border` | `color-mix(in srgb, var(--poodle-color-border-subtle) 74%, transparent)` |
| `--poodle-surface-shadow` | `none` |

### Root (.surface) — base styles

| Property | Value |
|----------|-------|
| `min-width` | `0` |
| `min-height` | `0` |
| `border` | `0.0625rem solid var(--poodle-surface-border)` |
| `border-radius` | `var(--poodle-treatment-surface-radius, var(--poodle-radius-surface))` |
| `background` | `var(--poodle-surface-fill)` |
| `box-shadow` | `var(--poodle-surface-shadow)` |

### Tone: canvas — .surface[data-tone="canvas"]

| Var | Value |
|-----|-------|
| `--poodle-surface-fill` | `color-mix(in srgb, var(--poodle-color-background-canvas) 98%, transparent)` |

### Tone: elevated — .surface[data-tone="elevated"]

| Var | Value |
|-----|-------|
| `--poodle-surface-fill` | `color-mix(in srgb, var(--poodle-color-background-elevated) 96%, var(--poodle-color-background-panel))` |
| `--poodle-surface-shadow` | `var(--poodle-elevation-surface)` |

### isElevated override — .surface[data-elevated="true"]

| Var | Value |
|-----|-------|
| `--poodle-surface-fill` | `color-mix(in srgb, var(--poodle-color-background-elevated) 96%, var(--poodle-color-background-panel))` |
| `--poodle-surface-shadow` | `var(--poodle-elevation-surface)` |

### Border: none — .surface[data-border="none"]

| Property | Value |
|----------|-------|
| `border-color` | `transparent` |

### Border: default — .surface[data-border="default"]

| Var | Value |
|-----|-------|
| `--poodle-surface-border` | `var(--poodle-color-border-default)` |

### Padding (Surface-specific mapping, applied as inline style)

Surface uses its own padding scale rather than the shared `scaleToSpace`
utility, providing roomier defaults suited to container-level spacing.

| Scale | Value |
|-------|-------|
| `none` | `0` |
| `sm` | `var(--poodle-space-panel-y)` (0.5rem) |
| `md` | `1rem` |
| `lg` | `1.5rem` |

### Data Attributes

| Attribute | Source |
|-----------|--------|
| `data-tone` | `tone` prop |
| `data-border` | `border` prop |
| `data-elevated` | `isElevated` prop (string "true"/"false") |

## 9. Svelte Notes

- rendered as a `<div>` with data attributes driving CSS selector overrides
- padding applied as inline style via shared `scaleToSpace` utility
- when `asRole` is set, the `role` attribute is applied to the root element
- when `label` is set, `aria-label` is applied to the root element
- treatment token `--poodle-treatment-surface-radius` allows theme-level radius
  overrides with fallback to `--poodle-radius-surface`
- CSS custom properties (`--poodle-surface-fill`, `--poodle-surface-border`,
  `--poodle-surface-shadow`) are set on the root and overridden by data-attribute
  selectors for tone, border, and elevation variants
- `color-mix` blending creates semi-transparent fills that layer naturally
  when surfaces are nested

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::surface`
- Spec struct: `SurfaceSpec` in primitives crate
- tone and border variants must produce matching fill colors using equivalent
  color-mix logic
- elevation shadow (`--poodle-elevation-surface`) maps to GPUI shadow drawing
- treatment radius fallback: use treatment token if set, else radius-surface
- region/group semantics must map into platform accessibility APIs when
  `asRole` is provided
- decorative surfaces (no asRole) must not surface as named accessibility nodes

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] tone semantics (panel, canvas, elevated) produce matching backgrounds
- [ ] border semantics (subtle, default, none) match
- [ ] isElevated override applies elevated fill and shadow
- [ ] region/group opt-in meaning matches
- [ ] decorative surfaces remain accessibility-neutral

### Tier 2: Visual Parity

- [ ] background fill color-mix values match across runtimes
- [ ] border color and width match (0.0625rem solid)
- [ ] border-radius matches (treatment fallback chain)
- [ ] elevation shadow matches
- [ ] padding scale values match

### Tier 3: Implementation Freedom

- [ ] CSS custom properties vs GPUI theme access stays internal
- [ ] CSS data-attribute selectors vs Rust match stays internal
- [ ] color-mix implementation may differ in internal API

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| color-mix transparency blending | GPUI may use direct alpha blending instead of CSS color-mix | allowed | same visual result required |
| Treatment radius fallback chain | CSS var fallback vs Rust conditional | allowed | same visual result |

## 13. Specimen Definitions

### Group: Panel tone (default)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Panel surface | `<Surface padding="md" border="subtle">` with text content | Panel-tier background with subtle border, medium padding; the standard container appearance |

### Group: Canvas tone

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Canvas surface | `<Surface tone="canvas" padding="md" border="subtle">` with text content | Canvas-tier background (sits behind panels as a background layer) with subtle border and medium padding |

### Group: Elevated tone

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Elevated surface | `<Surface tone="elevated" padding="md" border="subtle" isElevated>` with text content | Elevated-tier background with elevation shadow, subtle border, and medium padding; visually lifted above surrounding content |

### Group: No border

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Borderless surface | `<Surface padding="md" border="none">` with text content | Panel-tier background with no visible border; only padding and background fill are visible |

## 14. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: cards, panels, dialogs, shell sections, detail views,
  PanelSurface
- future follow-up: add high-contrast appearance guidance during accessibility
  hardening
