# g09.013 — Preview App Shell: Section Tabs, Sidebar, Per-Component Pages

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.007, g09.008, g09.009, g09.010, g09.011, g09.012
Primary repos: `pug`

## Goals

- [ ] restructure the GPUI preview app navigation to match the Svelte preview's
  4-section layout with sidebar catalogue and per-component pages
- [ ] replace the current flat list with a structured navigation model

## Execution Checklist

- [ ] implement section tab bar with 4 tabs: Primitives, Composites, Demo,
  Tokens — render at the top of the preview window
- [ ] implement sidebar component catalogue with alphabetical listing, grouped
  by category within each section
- [ ] highlight currently selected component in sidebar
- [ ] implement `ComponentPage` view with hero header showing: component name,
  tier badge (Primitive/Composite/Workstation), package name, description
- [ ] render live specimen below hero header on each component page
- [ ] implement `CatalogueLanding` view shown when no component is selected —
  overview of the section with component count and category summary
- [ ] implement component registry in Rust matching the Svelte
  `component-registry.ts` — slug, display name, tier, description, category
- [ ] wire section tab clicks to show appropriate sidebar content
- [ ] wire sidebar item clicks to show per-component page with specimen
- [ ] implement keyboard navigation: arrow keys in sidebar, tab to switch
  sections
- [ ] verify all 127+ component slugs are registered and routable
- [ ] verify the Tokens section displays a token value inspector (list of
  resolved token names and values from current theme)

## Acceptance Criteria

- [ ] 4 section tabs are visible and clickable
- [ ] sidebar shows correct component list for each section
- [ ] clicking a component in sidebar shows its specimen in the content area
- [ ] hero header displays correct metadata for each component
- [ ] Tokens section shows live token values
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.014` and implement display controls and route state.
