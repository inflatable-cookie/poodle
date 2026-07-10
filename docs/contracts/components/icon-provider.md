# IconProvider

Status: detailed contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `IconProvider`
- Layer: `foundation`
- Summary: a context provider that makes an icon registry available to all
  descendant Icon components
- In scope: setting icon registry context, rendering child content
- Out of scope: icon rendering (see Icon), icon registration APIs, lazy loading
  of icon sets

## 2. Anatomy

```text
[Root]  (no DOM element)
  └── [Children]  (descendant content)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | context boundary (no visual output) | none |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `icons` | `IconSet` | — | yes | icon set instance providing name-to-SVG mappings |

### Composition

| Snippet | Purpose |
|---------|---------|
| `children()` | child components that may contain Icon instances |

### Controlled And Uncontrolled

- Pure context provider; no internal state beyond holding the registry reference.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| — | — | No visual states; component produces no visual output |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| — | — | — | IconProvider emits no events |

## 6. Accessibility

### Semantics

- No DOM element emitted; no ARIA semantics
- Purely a data/context boundary

### Keyboard

| Key | Behavior |
|-----|----------|
| — | Not applicable |

### Focus And Announcement

- Not applicable; no visual or interactive output

## 7. Layout

### Sizing

- No visual output; does not participate in layout

### Composition

- parent expectations: application root, layout shells, isolated widget trees
- child expectations: any component tree that may contain Icon components
- resizing: not applicable

## 8. Token Usage — Exact Values

IconProvider has no CSS properties. It is a pure context provider with no
visual output.

| Property | Value |
|----------|-------|
| — | No CSS properties |

## 9. Svelte Notes

- Uses `setIconSet(icons)` to provide context via Svelte's `setContext`
- Seeds an empty set with `setIconSet({})`, then syncs the `icons` prop into
  the stored set via an `$effect`
- Renders only `children()` with no wrapper element
- `icons` prop is reactive; the icon set updates if the prop changes
- Import: `import { setIconSet } from './icon-registry'`
- String-based icon lookups resolve from this set first; if not found and
  `@poodle/icons-lucide` is installed, icons are lazily auto-imported
  (implementation detail beyond the contract surface)

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::icon_provider`
- Context modeled as GPUI's context system or a shared `Arc<IconRegistry>`
- No visual component; acts as a scope boundary for registry access
- May be unnecessary in GPUI if registry is globally available

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] registry is accessible to all descendant Icon components
- [ ] registry updates propagate to descendants

### Tier 2: Visual Parity

- [ ] No visual output (nothing to match)

### Tier 3: Implementation Freedom

- [ ] context mechanism is platform-owned (Svelte context vs GPUI context vs global)

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI may use global registry instead of scoped context | GPUI's context model differs from Svelte | allowed | same functional result |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: application shells, widget hosts, any tree containing Icons
- future follow-up: lazy icon set loading, registry merging for nested providers
