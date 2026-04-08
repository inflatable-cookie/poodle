# Size and Density

Status: stable contract
Updated: 2026-04-01

## 1. Purpose

- Component name: `SizeAndDensity`
- Layer: `foundation`
- Summary: global contract defining how size and density responsibilities
  are partitioned across all Poodle components — not a rendered component

This contract defines the **global rules** for how `size` and `density`
properties partition visual responsibility across all Poodle components.
Every component that accepts `size`, `sizeRole`, or `density` props must
follow these rules. Individual component contracts may add detail but must
not contradict this document.

## 2. The Two Axes

Poodle's semantic presentation model has two independent axes:

| Axis | What it controls | Prop surface | Inherited via |
|------|-----------------|--------------|---------------|
| **Size** | How large controls, text, and icons render | `size`, `sizeRole` | `UiPresentationProvider.sizeScale` |
| **Density** | How much breathing room surrounds content | `density` | `UiPresentationProvider.density` |

These axes are orthogonal. A component can be `size="xl"` at
`density="compact"` (large controls packed tightly) or `size="xs"` at
`density="comfortable"` (small controls with generous spacing). Both
combinations must produce coherent results.

## 3. Size Responsibilities

Size controls the **intrinsic dimensions and typographic scale** of
interactive elements. It answers: "how big is this control?"

### Size governs

| Property | Examples |
|----------|---------|
| Control height | button min-height, input height, icon-button width/height |
| Typography scale | label font-size, summary font-size, placeholder font-size |
| Icon dimensions | icon viewBox size, spinner diameter |
| Control-internal spacing | inline gap between icon and label inside a button |

### Size does NOT govern

| Property | Governed by |
|----------|-------------|
| Container padding (panel insets) | density |
| Gaps between sibling controls | density (or token default) |
| Gaps between content groups | density (or token default) |

### Size stops

| Stop | Control height | Offset from `md` base |
|------|---------------|----------------------|
| `xs` | 1.5rem | -0.5rem |
| `sm` | 1.75rem | -0.25rem (typical) |
| `md` | 2.25rem | base |
| `lg` | 2.75rem | +0.375rem (typical) |
| `xl` | 3.25rem | +0.5rem (typical) |

Exact offsets may vary by component category (e.g., icon buttons use
square dimensions, sliders use track height). The contract for each
component specifies its exact size table. The progression must always
be strictly monotonic: xs < sm < md < lg < xl for every measured
dimension.

### Resolution

```
resolvedSize = size ?? resolveSemanticControlSize(sizeScale, sizeRole)
```

- `size` is an explicit override (absolute)
- `sizeRole` resolves against the inherited `sizeScale` to produce a
  contextual size: `chrome` is one stop smaller, `prominent` one stop
  larger, `control` maps directly.

## 4. Density Responsibilities

Density controls the **spacing around and between** elements. It answers:
"how tightly is content packed?"

### Density governs

| Property | Examples |
|----------|---------|
| Container padding | panel insets, bar padding, card padding, dialog body padding |
| Gaps between sibling elements | action button row gap, breadcrumb item gap, toolbar item gap |
| Gaps between content groups | summary-to-actions gap, section spacing |

### Density does NOT govern

| Property | Governed by |
|----------|-------------|
| Control height / width | size |
| Font size | size |
| Icon size | size |
| Control-internal padding (e.g., button horizontal padding) | size (via `--poodle-space-control-x` offset) |

### Density stops

| Stop | Panel padding (Y / X) | Control spacing X | Character |
|------|-----------------------|-------------------|-----------|
| `compact` | 0.5rem / 0.75rem | 0.5rem | Tight, information-dense |
| `default` | 0.75rem / 1rem | 0.75rem | Balanced, general-purpose |
| `comfortable` | 1rem / 1.25rem | 1rem | Relaxed, spacious |

These values come from the helper functions in `presentation.ts`
(`panelSpaceYRem`, `panelSpaceXRem`, `controlSpaceXRem`) and are the
reference values. Components may use CSS custom properties
(`--poodle-space-panel-x`, `--poodle-space-panel-y`) or hardcoded rem
values as appropriate, but the relative proportions must track these
reference values.

### Resolution

```
resolvedDensity = density ?? uiPresentation.density ?? "default"
```

## 5. Control-Internal Padding: The Boundary

One area requires judgment: **horizontal padding inside a control** (e.g.,
the space between a button's border and its label). This is part of the
control's intrinsic feel, not the layout around it.

**Rule:** control-internal padding belongs to **size**, not density.

Standard control-internal padding offsets from `--poodle-space-control-x`:

| Size | Offset |
|------|--------|
| `xs` | -0.125rem |
| `sm` | -0.0625rem |
| `md` | base |
| `lg` | +0.125rem |
| `xl` | +0.1875rem |

Components that are **containers** (toolbar, bulk-action-bar, dialog,
drawer, accordion) have an additional **container padding** layer that
belongs to density. This is the panel inset, not the control-internal
padding.

## 6. CSS Implementation Pattern

### Size via `data-size`

```css
/* Size controls intrinsic control dimensions and type scale */
.component[data-size="xs"] .component__control {
  min-height: calc(var(--poodle-size-control-height) - 0.5rem);
  font-size: 0.75rem;
}
```

### Density via `data-density`

```css
/* Density controls container padding and sibling gaps */
.component[data-density="compact"] {
  padding: 0.25rem 0.5rem;
}
.component[data-density="comfortable"] {
  padding: 0.625rem 1rem;
}
```

### Both on the same root

```html
<div class="component" data-size="lg" data-density="compact">
  <!-- large controls, tight spacing -->
</div>
```

## 7. Component Categorisation

Not every component uses both axes the same way.

### Controls (Button, TextInput, Select, IconButton, etc.)

- **Size**: height, font-size, internal padding, icon size
- **Density**: minimal or none — controls are typically leaf elements
  without container padding. Their parent layout handles density.

### Containers (Toolbar, BulkActionBar, Dialog, Accordion, etc.)

- **Size**: pass-through to child controls, summary/label typography
- **Density**: container padding, gaps between children

### Inline elements (Breadcrumbs, Pagination, ToggleGroup, etc.)

- **Size**: item typography, item dimensions
- **Density**: gaps between items

### Decorative (Icon, Spinner, Progress, etc.)

- **Size**: intrinsic dimensions
- **Density**: not applicable

## 8. Accessibility

Size and density changes must never break accessibility:

- Minimum touch target of 44×44 CSS pixels must be maintained at all sizes
  including `xs`. If the visible control is smaller, hit-area padding must
  compensate.
- Density changes must not push interactive elements so close together that
  they fail the 24 CSS pixel minimum spacing guideline (WCAG 2.5.8 Target
  Size minimum).
- Font sizes must remain above 12px (0.75rem) at the smallest size stop to
  maintain legibility.
- `data-size` and `data-density` attributes carry no ARIA semantics and must
  not be relied upon for assistive technology.

## 9. Compliance Checklist (Parity)

Every component that accepts `size` or `density` must satisfy:

- [ ] Size controls ONLY intrinsic dimensions and typography
- [ ] Density controls ONLY container padding and sibling gaps
- [ ] Size stops are strictly monotonic (xs < sm < md < lg < xl)
- [ ] Density stops produce visually distinct, evenly-spaced results
- [ ] Container padding does NOT vary with size
- [ ] Control height does NOT vary with density
- [ ] `data-size` and `data-density` attributes are emitted on the root
- [ ] Both axes can be combined independently without visual breakage

## 10. Tokens and Helper Functions

### CSS custom properties (set by density mode overlays)

| Token | Compact | Default | Comfortable |
|-------|---------|---------|-------------|
| `--poodle-space-panel-x` | 0.75rem | 1rem | 1.25rem |
| `--poodle-space-panel-y` | 0.5rem | 0.75rem | 1rem |
| `--poodle-space-control-x` | 0.5rem | 0.75rem | 1rem |
| `--poodle-space-control-y` | 0.25rem | 0.375rem | 0.5rem |
| `--poodle-size-control-height` | — | 2.25rem | — |

Note: `--poodle-size-control-height` is set by size-scale mode, not density.
Density overlays adjust spacing tokens only.

**Important:** Density overlays no longer set `--poodle-size-control-height` or
`--poodle-size-panel-header`. These are size-only tokens set exclusively by the
control-size overlays. Earlier implementations that included these tokens in
density overlays should be updated to remove them.

### TypeScript helpers (presentation.ts)

| Function | Returns | Used for |
|----------|---------|----------|
| `controlHeightRem(size)` | rem value | size → control height |
| `controlSpaceXRem(density)` | rem value | density → control inline spacing |
| `panelSpaceXRem(density)` | rem value | density → panel horizontal inset |
| `panelSpaceYRem(density)` | rem value | density → panel vertical inset |
| `resolveSemanticControlSize(scale, role)` | ControlSize | sizeScale + role → absolute size |

## 11. Anti-Patterns

### Wrong: size controlling container padding

```css
/* DO NOT: padding varies with size */
.toolbar[data-size="xs"] { padding: 0.25rem; }
.toolbar[data-size="xl"] { padding: 1rem; }
```

### Wrong: density controlling font size

```css
/* DO NOT: font-size varies with density */
.label[data-density="compact"] { font-size: 0.75rem; }
.label[data-density="comfortable"] { font-size: 1rem; }
```

### Wrong: density controlling control height

```css
/* DO NOT: control height varies with density */
.button[data-density="compact"] { min-height: 1.5rem; }
```

### Correct: separated concerns

```css
/* Size controls the control */
.button[data-size="xs"] { min-height: 1.5rem; font-size: 0.75rem; }

/* Density controls the container */
.toolbar[data-density="compact"] { padding: 0.25rem 0.5rem; gap: 0.25rem; }
```
