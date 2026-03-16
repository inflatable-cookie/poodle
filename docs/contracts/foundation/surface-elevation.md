# Surface Elevation — Cross-Cutting Contract

## Purpose

The contextual surface elevation system ensures components automatically contrast with whatever visual surface they're placed on. Without it, components using fixed background tokens (e.g. `--pug-color-background-surface`) lose contrast when placed inside Cards, Dialogs, or other elevated containers whose backgrounds are close to those fixed values.

## Token: `--pug-surface`

A single inheritable CSS custom property that tracks the current container's computed background color.

```css
.app-shell {
  --pug-surface: var(--pug-color-background-canvas);
}
```

**Important:** `--pug-surface` must be declared at or below the element where theme tokens are scoped (e.g. `.app-shell[data-theme]`), not on `:root`. Declaring it on `:root` causes `var(--pug-color-background-canvas)` to resolve before theme tokens are available, producing incorrect values when themes change.

Because CSS custom properties inherit through the DOM tree, all descendants automatically see the nearest ancestor's surface color.

## Background Hierarchy

From darkest to lightest (in dark theme — reversed in light theme):

```
canvas → surface → panel → elevated
```

The mixing formula always blends **toward `elevated`** (the lightest level), guaranteeing visible contrast regardless of the parent surface.

## Surface Creators

A **surface creator** is any component that establishes a new visual surface. It MUST set `--pug-surface` on its root element to match its own computed background.

| Component | CSS target | Background value |
|-----------|-----------|------------------|
| Card | `.card` | `var(--pug-treatment-surface-fill, var(--pug-recipe-card-fill))` |
| Dialog | `.dialog__surface` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| Drawer | `.drawer__surface` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| Popover | `.popover__surface` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| HoverCard | `.hover-card__surface` | `color-mix(in srgb, var(--pug-color-background-elevated) 98%, var(--pug-color-background-panel))` |
| Callout | `.callout` | `var(--pug-callout-fill)` |
| Surface | `.surface` | `var(--pug-surface-fill)` (varies by tone — panel, canvas, or elevated) |

Any app-level container that creates a distinct background surface (e.g. a sidebar panel) should also set `--pug-surface`.

### Implementation pattern

```css
.my-surface {
  background: <computed-background>;
  --pug-surface: <same-computed-background>;
}
```

The `--pug-surface` value MUST exactly match the `background` value so child components see an accurate representation of the surface they're on.

## Surface Consumers

A **surface consumer** is any component that needs to visually contrast with its container. Instead of using fixed background tokens, it derives its background from `--pug-surface` using `color-mix()`:

```css
background: color-mix(in srgb, var(--pug-surface) <ratio>, var(--pug-color-text-primary));
```

The mixing target is `text-primary`, not a background token. This is critical because:

- **In dark mode**, `text-primary` is light (~#e8eef5), so the mix lightens the surface — creating visible contrast even though the background hierarchy (canvas→elevated) spans only ~18 RGB units.
- **In light mode**, `text-primary` is dark (~#1d2428), so the mix darkens the surface — the same formula produces appropriate contrast in both themes without any theme-specific logic.

Mixing toward `elevated` was tried and rejected: in dark mode, `elevated` is only ~5 RGB units above `panel`, producing imperceptible contrast inside cards.

### Mixing Ratio Tiers

| Ratio | Contrast boost | Use case | Components |
|-------|---------------|----------|------------|
| 88% | ~12% text-primary (strong) | Bordered panel-like containers | Accordion, Collapsible, ListCard |
| 91% | ~9% text-primary (medium-strong) | Table headers | Table |
| 93% | ~7% text-primary (moderate) | Interactive controls, inline editable areas | SegmentedControl, ToggleGroup, DetailRow, EditableLabel |
| 96% | ~4% text-primary (subtle) | Indicators, tracks | Badge (muted), Progress, Meter |

### Consumer implementation pattern

```css
.my-component {
  background: color-mix(in srgb, var(--pug-surface) 93%, var(--pug-color-text-primary));
}
```

### Why text-primary works

The formula `color-mix(surface N%, text-primary)` is theme-adaptive by nature. `text-primary` is always at the opposite end of the luminance spectrum from the surface, so even small percentages (4–12%) produce 10–25 RGB units of contrast. This is sufficient for visual differentiation without overpowering the surface hierarchy.

**Important:** Surface creators must NOT use `--pug-surface` in their own `background` or `--pug-surface` declaration on the same element — this creates a CSS circular reference and the browser invalidates both properties. Surface creators should use fixed token formulas for their fill, then set `--pug-surface` to that computed value.

## Nesting Behavior

When surfaces nest (e.g. Card inside Card, Accordion inside Dialog), each level sets `--pug-surface` to its own background. Child consumers then derive from the innermost surface.

At deep nesting levels, shadow and border provide additional visual differentiation.

## GPUI Implementation Notes

GPUI (Rust) has no CSS custom property inheritance. The equivalent must be implemented via a **context value** passed through the render tree:

```rust
// Conceptual — adapt to actual GPUI context API
struct SurfaceContext {
    surface_color: Hsla,
}
```

**Surface creators** push a new `SurfaceContext` with their computed background color.

**Surface consumers** read the nearest `SurfaceContext` and blend toward `text_primary`:

```rust
// Equivalent of color-mix(in srgb, surface 93%, text-primary)
let text_primary = cx.theme().colors().text_primary;
let bg = surface_color.blend(text_primary, 0.07); // 1.0 - 0.93
```

The `color-mix(surface N%, text-primary)` formula maps to:

```rust
// ratio = N / 100 (the surface percentage)
let text_primary = cx.theme().colors().text_primary;
let result = Hsla::blend(surface_color, text_primary, 1.0 - ratio);
```

Where `blend(a, b, t)` produces `a * (1 - t) + b * t`. The key insight: `text_primary` is always at the opposite luminance extreme from the surface, so this formula is inherently theme-adaptive.

### GPUI surface creator pattern

```rust
fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
    let bg = compute_background(cx);
    cx.provide(SurfaceContext { surface_color: bg });
    div().bg(bg).child(/* ... */)
}
```

### GPUI surface consumer pattern

```rust
fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
    let surface = cx.consume::<SurfaceContext>()
        .map(|s| s.surface_color)
        .unwrap_or(cx.theme().colors().background_canvas);
    let text_primary = cx.theme().colors().text_primary;
    let bg = surface.blend(text_primary, 0.07); // 93% surface ratio
    div().bg(bg).child(/* ... */)
}
```

## Jetstream Implementation Notes

Jetstream follows the same conceptual pattern. Track the current surface color in component context and derive child backgrounds via linear interpolation toward `text-primary`.

```typescript
// Surface creator
const surfaceColor = computeBackground();
context.provide('pug-surface', surfaceColor);

// Surface consumer
const surface = context.consume('pug-surface') ?? theme.colors.backgroundCanvas;
const textPrimary = theme.colors.textPrimary;
const bg = colorLerp(surface, textPrimary, 1.0 - ratio);
```

The mixing ratios (88%, 91%, 93%, 96%) and the `text-primary` mixing target are identical across all renderers.

## Parity Checklist

All three renderers (Svelte, GPUI, Jetstream) must produce visually comparable results when:

- [ ] Components placed directly on page canvas show appropriate contrast
- [ ] Components placed inside a Card show visible contrast against the card background
- [ ] Components placed inside a Dialog show visible contrast against the dialog background
- [ ] Components in nested surfaces (Card in Card, Accordion in Dialog) remain legible
- [ ] Both dark and light themes produce adequate contrast at all nesting levels
- [ ] Surface creators correctly propagate their background to descendants
- [ ] The same mixing ratio tiers are applied to the same components across renderers
