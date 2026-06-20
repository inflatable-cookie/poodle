# UiPresentationProvider

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `UiPresentationProvider`
- Layer: `foundation`
- Summary: a presentation-context provider that scopes semantic density and
  control-size defaults for descendant primitives by setting CSS custom properties
  and a Svelte context store
- In scope: local `density` and `sizeScale` inheritance, CSS custom property
  overrides for descendant primitives (`--poodle-size-control-height`,
  `--poodle-space-control-x`, `--poodle-space-panel-x`, `--poodle-space-panel-y`),
  Svelte context for `resolveSemanticControlSize` consumers
- Out of scope: theme switching, token artifact generation, cross-runtime
  provider parity outside the documented semantic inputs, visual chrome

## 2. Anatomy

```text
[Root .poodle-ui-presentation-provider]  <div style="--poodle-size-control-height: ...; ...">
  └── [Children] (all descendant content)
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | `<div>` wrapper with `display: contents` and inline CSS custom properties | `--poodle-size-control-height`, `--poodle-space-control-x`, `--poodle-space-panel-x`, `--poodle-space-panel-y` |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `density` | `ControlDensity` | `"default"` | no | local density baseline; one of `"compact"`, `"default"`, `"comfortable"` |
| `sizeScale` | `ControlSize` | `"md"` | no | local semantic control-size baseline; one of `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |

### CSS Custom Properties Set

The provider computes and sets these CSS custom properties as inline styles
on the root `<div>`:

| Custom Property | Driven By | Purpose |
|-----------------|-----------|---------|
| `--poodle-size-control-height` | `sizeScale` | base control height for descendant controls |
| `--poodle-space-control-x` | `density` | horizontal padding for controls |
| `--poodle-space-panel-x` | `density` | horizontal padding for panels/containers |
| `--poodle-space-panel-y` | `density` | vertical padding for panels/containers |

### Resolved Values -- sizeScale to control-height

| sizeScale | `--poodle-size-control-height` |
|-----------|-------------------------------|
| `xs` | `1.5rem` |
| `sm` | `1.75rem` |
| `md` | `2.25rem` |
| `lg` | `2.75rem` |
| `xl` | `3.25rem` |

### Resolved Values -- density to spacing

| density | `--poodle-space-control-x` | `--poodle-space-panel-x` | `--poodle-space-panel-y` |
|---------|---------------------------|-------------------------|-------------------------|
| `compact` | `0.5rem` | `0.75rem` | `0.5rem` |
| `default` | `0.75rem` | `1rem` | `0.75rem` |
| `comfortable` | `1rem` | `1.25rem` | `1rem` |

### Svelte Context Store

The provider creates a Svelte context store (`symbol: poodle-ui-presentation`)
containing `{ density, sizeScale }`. Descendant primitives read this via
`getUiPresentation()` and use `resolveSemanticControlSize(sizeScale, sizeRole)`
to compute their resolved size.

### Semantic Size Resolution (`resolveSemanticControlSize`)

Descendant primitives that declare a `sizeRole` prop resolve their effective
size against the provider's `sizeScale`:

| sizeScale | chrome | control | prominent |
|-----------|--------|---------|-----------|
| `xs` | `xs` | `xs` | `sm` |
| `sm` | `sm` | `sm` | `md` |
| `md` | `sm` | `md` | `lg` |
| `lg` | `md` | `lg` | `xl` |
| `xl` | `lg` | `xl` | `xl` |

### Supporting Visual Size Resolution (`resolveSupportingVisualSize`)

A second resolver sizes supporting visuals (e.g. icons rendered inside a
control) one step below the control's resolved size, so glyphs stay
proportionate. It maps `xl → lg`, `lg → md`, `md → sm`, and is identity for `sm`
and `xs`.

| input size | supporting visual size |
|------------|------------------------|
| `xs` | `xs` |
| `sm` | `sm` |
| `md` | `sm` |
| `lg` | `md` |
| `xl` | `lg` |

### Controlled And Uncontrolled

- Both `density` and `sizeScale` are externally controlled props
- Changes to props are reflected immediately in the context store, cascading to
  all descendant consumers

## 4. States

### Visual States

This component has no visual states of its own. It renders no visual chrome.

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | any | `display: contents` wrapper with CSS custom properties; visually invisible |

## 5. Events

No events are dispatched by this component.

## 6. Accessibility

### Semantics

- The provider is accessibility-neutral
- It must not introduce an extra landmark, region, or focusable wrapper
- `display: contents` ensures descendant semantics remain intact and the
  wrapper does not appear in the accessibility tree as a grouping element
- No ARIA attributes on the root element

### Keyboard

No keyboard interactions. The provider is not focusable and does not intercept
any key events.

### Focus And Announcement

- No focus behavior
- No live-region behavior

## 7. Layout

### Sizing

- Root uses `display: contents`, making it layout-neutral
- The provider does not add any dimensions, padding, margin, or gap
- All layout influence is via CSS custom properties inherited by descendants

### Composition

- parent expectations: any container that needs to scope presentation defaults
  for a region (e.g., sidebar at compact density, form area at lg size scale)
- child expectations: any Poodle primitives or composites that consume
  `getUiPresentation()` context or CSS custom properties
- nesting: providers can nest; inner providers override outer ones for their
  subtree

## 8. Token Usage -- Exact Values

### Root `.poodle-ui-presentation-provider`

| Property | Value |
|----------|-------|
| `display` | `contents` |

### Inline Style (computed)

The root `<div>` receives an inline `style` attribute with four CSS custom
properties computed from props. See the "Resolved Values" tables in section 3
for exact mappings.

Example for `density="default"`, `sizeScale="md"`:
```
--poodle-size-control-height: 2.25rem;
--poodle-space-control-x: 0.75rem;
--poodle-space-panel-x: 1rem;
--poodle-space-panel-y: 0.75rem
```

## 9. Svelte Notes

- Implemented as a `<div class="poodle-ui-presentation-provider">` with
  `display: contents` and inline `style` for CSS custom properties
- Each provider creates a fresh scoped context store via `setUiPresentation()`
  (which unconditionally calls `writable(value)` + `setContext`); it does not
  reuse or mutate an outer provider's store
- The store is seeded with a literal `{ density: "default", sizeScale: "md" }`
  and an effect immediately syncs it to the real prop values (and keeps it in
  sync on prop changes)
- Nesting works through Svelte's own context scoping: an inner provider's
  `setContext` shadows the outer store for its subtree, so inner overrides outer
  without touching the outer store
- Helper functions from `presentation.ts`:
  - `controlHeightRem(sizeScale)` computes the control height value
  - `controlSpaceXRem(density)` computes horizontal control spacing
  - `panelSpaceXRem(density)` computes horizontal panel spacing
  - `panelSpaceYRem(density)` computes vertical panel spacing
- No `data-size` or `data-density` attributes on the root; the provider is
  not a visual component
- `children()` renders all descendant content

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::components::ui_presentation_provider`
- GPUI equivalent: a context provider that sets equivalent layout parameters
  for descendant components
- The CSS custom property mechanism does not apply in GPUI; instead, the
  context store values are consumed directly by component specs
- Semantic size resolution must match the `resolveSemanticControlSize` table
- Nesting behavior must match: inner providers override outer ones

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `density` and `sizeScale` context values propagate to descendants
- [ ] semantic size resolution table matches exactly for all sizeScale/sizeRole combinations
- [ ] control-height values match for all five size scales
- [ ] spacing values match for all three densities
- [ ] nesting: inner provider overrides outer provider for its subtree

### Tier 2: Visual Parity

- [ ] provider wrapper is layout-neutral (no visual impact)
- [ ] CSS custom properties produce correct descendant sizing
- [ ] no extra landmark or focusable element introduced

### Tier 3: Implementation Freedom

- [ ] context propagation mechanism is platform-owned (Svelte context vs GPUI context)
- [ ] CSS custom property vs direct value consumption is platform-owned
- [ ] store update semantics (reactive vs imperative) are platform-owned

## 12. Specimen Definitions

This component has no dedicated specimen as it produces no visual output of
its own. Its effects are demonstrated by wrapping other component specimens
with different `density` and `sizeScale` values.

### Integration Demonstration

| Label | Config | Expected Visual |
|-------|--------|-----------------|
| Compact/sm region | `<UiPresentationProvider density="compact" sizeScale="sm">` wrapping buttons, inputs, etc. | Descendant controls render smaller and with tighter spacing |
| Comfortable/lg region | `<UiPresentationProvider density="comfortable" sizeScale="lg">` wrapping same controls | Descendant controls render larger and with more generous spacing |
| Nested override | Outer: `density="default" sizeScale="md"`, Inner: `density="compact" sizeScale="sm"` | Inner region controls are compact/small; outer region controls are default/medium |

## 13. Approval And Adoption Notes

- contract status: `detailed contract`
- approvers: pending
- downstream adopters: all primitives and composites that consume semantic
  size or density (Button, IconButton, Spinner, Toolbar, Tabs, BulkActionBar,
  Breadcrumbs, CollapseToggle, and others)
- future follow-up: keep descendant primitive and composite contracts aligned
  to this provider whenever new components opt into semantic `sizeRole` or
  density inheritance
