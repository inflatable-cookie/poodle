# Rendering style typography

The Svelte specimens compare the default and quiet styles in Eclipse and
Iceberg. Each sheet has one row per control size, from xs through xl.

| Component | Dark | Light |
| --- | --- | --- |
| PageHeader | [Eclipse](eclipse-page-header.png) | [Iceberg](iceberg-page-header.png) |
| SidebarNav | [Eclipse](eclipse-sidebar-nav.png) | [Iceberg](iceberg-sidebar-nav.png) |
| Dialog | [Eclipse](eclipse-dialog.png) | [Iceberg](iceberg-dialog.png) |
| DataTable | [Eclipse](eclipse-data-table.png) | [Iceberg](iceberg-data-table.png) |
| Button | [Eclipse](eclipse-button.png) | [Iceberg](iceberg-button.png) |
| TextInput | [Eclipse](eclipse-text-input.png) | [Iceberg](iceberg-text-input.png) |

[Computed roles at md](computed-roles.md) covers all 19 roles, including
those that the six components do not use yet. The [capture receipt](receipt.json)
records exact RGBA comparisons against base `837260b15`, Button and TextInput
heights in every density, and the root font-family override on every component.
The capture also checks a subtree override on each component and a default
scope nested under quiet. SidebarNav’s library specimen receives the same
`data-size` attribute that an explicit size prop sets.

## Migration decisions

- DataTable assigns separate roles to column headings, built-in cell text,
  filters and footer. The first built-in cell takes the primary weight, so
  the row reads from its name to the secondary summary. Styled headers use
  the theme's neutral secondary ink. The existing size-specific values
  supply the default fallbacks.
- Lower-level PageHeader titles and titleless subtitles use section title.
  The body and subtitle use secondary copy. Dormant local variables and
  typography owned by the composed Pill or action components stay in place.
- The character counter uses caption and tabular figures. Its existing
  shorthand remains available to consumers who define it. Slugs use the
  input role's size and leading with the mono family reserved for the
  identifier.
- Button glyph and spinner boxes keep their dimensions. Quiet gives text
  glyphs the sans family. Dialog descriptions use secondary copy and
  navigation end labels use caption.

## Reproduce

Build the Svelte preview at the base and at the revision under test with
`effigy core:build` followed by `effigy svelte:build`. Save each preview's
`dist` to a distinct directory, then run:

```sh
effigy visual:typography -- BEFORE_DIST AFTER_DIST OUTPUT
```
