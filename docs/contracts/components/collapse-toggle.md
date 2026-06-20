# CollapseToggle

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `CollapseToggle`
- Layer: `foundation`
- Summary: a standalone collapse/expand toggle button with a directional chevron
  icon that indicates and controls the collapsed state of an adjacent region
- In scope: toggle interaction, collapsed/expanded visual state, four directional
  orientations, icon rotation based on state, accessible labeling with
  `aria-expanded`, disabled state, size and density scaling
- Out of scope: what gets collapsed (determined by host), collapse animation,
  layout reflow policy, the collapsible content itself

## 2. Anatomy

```text
[Root .collapse-toggle]  <button type="button" aria-expanded="..." aria-label="...">
  └── [Icon]  chevron-{direction} (from Icon component)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<button>` element serving as the toggle trigger | padding, border-radius, background, color, cursor |
| Icon | yes | chevron icon whose direction reflects collapsed state | inherits color from root |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `collapsed` | `boolean` | `false` | no | current collapse state |
| `direction` | `CollapseDirection` | `"left"` | no | collapse direction (`"left"`, `"right"`, `"up"`, `"down"`) |
| `disabled` | `boolean` | `false` | no | suppresses interaction |
| `ariaLabel` | `string \| null` | `null` | no | accessible label; defaults to `"Collapse"` when expanded or `"Expand"` when collapsed |
| `size` | `ControlSize \| null` | `null` | no | explicit control size override; when null, resolves from inherited presentation |
| `sizeRole` | `SemanticControlSizeRole` | `"chrome"` | no | semantic size offset from inherited presentation |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override for padding |
| `onToggle` | `((isCollapsed: boolean) => void) \| null` | `null` | no | called when the collapsed state changes |

### CollapseDirection Type

```typescript
type CollapseDirection = "left" | "right" | "up" | "down";
```

### Controlled And Uncontrolled

- `collapsed` is externally managed; the component calls `onToggle` with the
  next state and the parent updates the prop

### Icon Direction Logic

- When expanded (`collapsed=false`): icon points in the `direction` (e.g., `chevron-left` for `direction="left"`)
- When collapsed (`collapsed=true`): icon points in the opposite direction (e.g., `chevron-right` for `direction="left"`)
- Opposite mapping: left<->right, up<->down

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| expanded | `collapsed=false` | icon points toward the `direction` value; `aria-expanded="true"` |
| collapsed | `collapsed=true` | icon points opposite to `direction`; `aria-expanded="false"` |
| hover | pointer over button | background changes to surface-hover, color changes to text-default |
| disabled | `disabled=true` | opacity 0.4, cursor default, non-interactive |
| focus-visible | keyboard focus | focus ring outline |

## 5. Callbacks

| Callback | When It Runs | Payload | Notes |
|----------|--------------|---------|-------|
| `onToggle` | user clicks the button (when not disabled) | `boolean` | receives the new collapsed state (toggled from current) |

## 6. Accessibility

### Semantics

- Root: `<button type="button">` with `aria-expanded` reflecting the expanded state
- `aria-expanded="true"` when `collapsed=false`; `aria-expanded="false"` when `collapsed=true`
- `aria-label` defaults to `"Collapse"` when expanded, `"Expand"` when collapsed;
  override via `ariaLabel` prop for contextual labels like `"Collapse left dock"`
- `data-collapsed` attribute present (without value) when collapsed; absent when expanded

### Keyboard

| Key | Behavior |
|-----|----------|
| `Enter` | toggles collapse state |
| `Space` | toggles collapse state |
| `Tab` | moves focus to/from the button in document order |

### Focus And Announcement

- focus entry: button participates in normal tab order
- focus ring: `outline` with `--poodle-border-width-focus` solid `--poodle-color-accent-focusRing`, offset `0.0625rem`
- state change: `aria-expanded` update announced by screen reader

## 7. Layout

### Sizing

- Compact inline-flex button sized to icon plus padding
- No explicit width or height; dimensions determined by icon size + padding
- Positioned by host at region boundary (sidebar edge, panel header, divider)

### Composition

- parent expectations: any collapsible region boundary, divider, panel header, sidebar edge
- child expectations: none (self-contained)
- resizing: button size scales via the `size` prop / density padding

## 8. Token Usage -- Exact Values

### Root `.collapse-toggle`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `padding` | `0.125rem` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-sm, 0.25rem)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-muted)` |
| `cursor` | `pointer` |
| `line-height` | `1` |

### Root -- hover

| Property | Value |
|----------|-------|
| `background` | `var(--poodle-color-surface-hover)` |
| `color` | `var(--poodle-color-text-default)` |

### Root -- disabled

| Property | Value |
|----------|-------|
| `opacity` | `0.4` |
| `cursor` | `default` |

### Root -- focus-visible

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

### Size adjustments

Size affects button padding only. The `sm` and `md` sizes share the same
padding value (`0.125rem`).

| Size | padding |
|------|---------|
| `xs` | `0.0625rem` |
| `sm` | `0.125rem` (default) |
| `md` | `0.125rem` (default) |
| `lg` | `0.1875rem` |
| `xl` | `0.25rem` |

### Density adjustments

Density controls `padding-inline` only (it does not touch vertical padding or
button height, per the size/density orthogonality rule). `compact` and
`default` both equal the base `0.125rem`, so only `comfortable` widens the
button.

| Density | padding-inline |
|---------|----------------|
| `compact` | `0.125rem` (= base, no change) |
| `default` | `0.125rem` (base) |
| `comfortable` | `0.375rem` |

## 9. Svelte Notes

- `data-size` attribute on root `<button>` reflects the resolved size
- `data-density` attribute on root `<button>` reflects the resolved density (`compact`, `default`, or `comfortable`)
- `data-collapsed` attribute present (truthy) when collapsed; absent when expanded
- `data-direction` attribute reflects the `direction` prop value
- Root is a native `<button type="button">` element, not an IconButton composite
- Icon is the `Icon` component with computed `name` based on direction and collapsed state
- `disabled` HTML attribute set directly on the `<button>` when disabled
- Size and density resolve from `UiPresentationProvider` context when not explicitly set

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::collapse_toggle`
- Spec struct: `CollapseToggleSpec` in primitives crate
- Icon rotation via transform or icon name swap matching Svelte logic
- `aria-expanded` must be mapped to GPUI accessibility API

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `<button>` with `aria-expanded` correctly reflects collapsed state
- [ ] `aria-label` defaults to "Collapse"/"Expand" based on state
- [ ] `onToggle` callback receives the new collapsed state
- [ ] icon direction flips between collapsed and expanded states
- [ ] disabled state prevents interaction and applies opacity

### Tier 2: Visual Parity

- [ ] all five sizes visually match per size table (padding)
- [ ] all three densities match per density table (padding-inline)
- [ ] idle: transparent background, muted text color
- [ ] hover: surface-hover background, default text color
- [ ] disabled: 0.4 opacity
- [ ] focus-visible: accent focus ring with 0.0625rem offset
- [ ] border-radius matches `--poodle-radius-sm`

### Tier 3: Implementation Freedom

- [ ] icon component internals stay renderer-specific
- [ ] whether icon swaps name or rotates via transform is renderer-specific

## 12. Specimen Definitions

### Group: Directions

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Left | `direction="left"`, `collapsed=false` (toggleable) | Chevron pointing left when expanded; toggles on click; label shows "(collapsed)" or "(expanded)" |
| Right | `direction="right"`, `collapsed=false` (toggleable) | Chevron pointing right when expanded; toggles on click; label shows state |
| Up | `direction="up"`, `collapsed=false` (toggleable) | Chevron pointing up when expanded; toggles on click; label shows state |
| Down | `direction="down"`, `collapsed=false` (toggleable) | Chevron pointing down when expanded; toggles on click; label shows state |

### Group: Sizes

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Size variants | `direction="left"`, one button per size (xs, sm, md, lg, xl) | Five toggle buttons at increasing padding/icon sizes |

### Group: Densities

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Density variants | `direction="left"`, one per density (compact, default, comfortable) with labels | Three toggle buttons with increasing padding |

### Group: Disabled

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Disabled left | `direction="left"`, `disabled=true` | Muted toggle, non-interactive, left-pointing chevron |
| Disabled right | `direction="right"`, `disabled=true` | Muted toggle, non-interactive, right-pointing chevron |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: sidebar panels, split views, collapsible sections,
  dock regions
- future follow-up: none anticipated
