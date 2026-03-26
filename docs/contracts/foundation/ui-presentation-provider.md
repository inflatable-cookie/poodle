# UiPresentationProvider

Status: seed contract
Updated: 2026-03-26

## 1. Purpose

- Component name: `UiPresentationProvider`
- Layer: `foundation`
- Summary: a presentation-context provider that scopes semantic density and
  control-size defaults for descendant Svelte primitives
- In scope: local `density` and `sizeScale` inheritance, CSS custom-property
  overrides for descendant primitives, semantic sizing support for `sizeRole`
- Out of scope: theme switching, token artifact generation, cross-runtime
  provider parity outside the documented semantic inputs

## 2. Public Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `density` | `"compact" \| "default" \| "comfortable"` | `"default"` | local density baseline |
| `sizeScale` | `"xs" \| "sm" \| "md" \| "lg" \| "xl"` | `"md"` | local semantic control-size baseline |

## 3. Contract Notes

- Descendant primitives may resolve `sizeRole="chrome" | "control" | "prominent"`
  against this provider rather than requiring explicit absolute `size` props.
- The provider sets local CSS custom properties for control height and panel /
  control spacing so token-driven descendants inherit the scoped presentation.
- The provider renders no visual chrome of its own and should be effectively
  layout-neutral.

## 4. Svelte Notes

- Implemented as a context provider plus a `display: contents` wrapper.
- Exposes the shared semantic presentation helpers used by Button, IconButton,
  Spinner, Toolbar, Tabs, and other upgraded primitives.

## 5. Accessibility

- The provider should remain accessibility-neutral.
- It must not introduce an extra landmark, region, or focusable wrapper around
  its children.
- `display: contents` is preferred so descendant semantics remain intact.

## 6. Next Task

Keep descendant primitive and composite contracts aligned to this provider
whenever new components opt into semantic `sizeRole` or density inheritance.
