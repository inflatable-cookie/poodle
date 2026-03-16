# Icon

Status: detailed contract
Updated: 2026-03-15

## 1. Purpose

- Component name: `Icon`
- Layer: `foundation`
- Summary: a sized, accessible inline SVG icon element that resolves names from
  an icon registry
- In scope: three sizes (sm, md, lg), accessible and decorative modes, registry
  resolution, currentColor inheritance
- Out of scope: animated icons, multi-color icons, icon buttons (see IconButton),
  icon registration (see IconProvider)

## 2. Anatomy

```text
[Root .icon]  <svg>
  └── [Path data]  (from registry lookup)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | inline SVG element sized by variant | width, height, stroke, fill |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `name` | `string` | — | yes | icon registry identifier |
| `size` | `"sm" \| "md" \| "lg"` | `"md"` | no | icon dimensions |
| `ariaLabel` | `string \| null` | `null` | no | accessible name; absence triggers decorative mode |

### Controlled And Uncontrolled

- Display primitive only; no internal state.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| decorative | no ariaLabel | `role="presentation"`, `aria-hidden="true"` |
| accessible | ariaLabel provided | `role="img"`, `aria-label` set |
| sm | `size="sm"` | small dimensions |
| md | `size="md"` (default) | medium dimensions |
| lg | `size="lg"` | large dimensions |

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| — | — | — | Icon emits no events |

## 6. Accessibility

### Semantics

- When `ariaLabel` provided: `role="img"`, `aria-label` from prop
- When no `ariaLabel`: `role="presentation"`, `aria-hidden="true"`
- SVG is inline, inherits parent color via `currentColor`

### Keyboard

| Key | Behavior |
|-----|----------|
| — | Icon is not focusable |

### Focus And Announcement

- Not focusable; purely visual or announced via parent context

## 7. Layout

### Sizing

- sm: `var(--pug-size-icon-sm)` square
- md: `var(--pug-size-icon-md)` square
- lg: `var(--pug-size-icon-lg)` square

### Composition

- parent expectations: buttons, labels, menu items, any inline context
- child expectations: none (leaf element)
- resizing: fixed size per variant, does not grow

## 8. Token Usage — Exact Values

### Root (SVG element)

| Property | Value |
|----------|-------|
| `display` | `inline-block` |
| `vertical-align` | `middle` |
| `flex-shrink` | `0` |

### SVG attributes

| Attribute | Value |
|-----------|-------|
| `viewBox` | `"0 0 24 24"` |
| `fill` | `"none"` |
| `stroke` | `"currentColor"` |
| `stroke-width` | `"2"` |
| `stroke-linecap` | `"round"` |
| `stroke-linejoin` | `"round"` |

### Size sm

| Property | Value |
|----------|-------|
| `width` | `var(--pug-size-icon-sm)` |
| `height` | `var(--pug-size-icon-sm)` |

### Size md

| Property | Value |
|----------|-------|
| `width` | `var(--pug-size-icon-md)` |
| `height` | `var(--pug-size-icon-md)` |

### Size lg

| Property | Value |
|----------|-------|
| `width` | `var(--pug-size-icon-lg)` |
| `height` | `var(--pug-size-icon-lg)` |

## 9. Svelte Notes

- Uses `getIconRegistry()` context to resolve icon name to SVG path data
- SVG rendered with `{@html}` for path content from registry
- `data-icon` attribute set to icon name for testing hooks
- `data-size` attribute for size variant

## 10. GPUI Notes

- expected crate/module surface: `pug_gpui::components::icon`
- Spec struct: `IconSpec` in primitives crate
- Component struct: `PugIcon` in components crate
- Icon registry is a shared resource (Arc or global)
- SVG rendering uses GPUI's native SVG support or path rendering
- stroke-based icons may need conversion to filled paths for GPUI

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] name resolves from registry identically
- [ ] ariaLabel controls decorative vs accessible mode
- [ ] size prop produces matching dimensions

### Tier 2: Visual Parity

- [ ] icon dimensions match per size
- [ ] currentColor inheritance works
- [ ] stroke appearance matches (width, linecap, linejoin)

### Tier 3: Implementation Freedom

- [ ] SVG rendering method is platform-owned
- [ ] registry lookup mechanism is platform-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| SVG rendering method | GPUI may use path rendering instead of inline SVG | allowed | same visual result |
| stroke vs fill | GPUI may pre-convert stroke icons to fill | allowed | same visual result |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: Button, IconButton, Menu, NavCard, all icon-bearing components
- future follow-up: animated icon support, multi-color icon support
