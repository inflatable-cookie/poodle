# g10.002 — Component Registry and Specimen Framework

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.001
Primary repos: `pug`

## Goals

- [ ] implement a component registry mapping slugs to metadata
- [ ] define the specimen rendering pattern for Jetstream

## Execution Checklist

- [ ] create `registry.rs` with `ComponentEntry` struct: slug, display_name,
  tier (Primitive/Composite/Workstation), category, description, has_specimen
- [ ] populate registry with all Jetstream-appropriate components from the
  delta register (g08.011)
- [ ] implement `find_component(slug)` lookup function
- [ ] implement `components_for_section(section)` filtered listing
- [ ] define specimen rendering trait:
  ```rust
  trait Specimen {
      fn render(&self, theme: &PugTheme, tree: &mut UiTree) -> UiNodeId;
  }
  ```
- [ ] implement `specimen_card` helper that wraps a specimen in a titled
  container with elevated background and border
- [ ] implement specimen page layout: hero header (component name, tier
  badge, description) above live specimen output
- [ ] implement catalogue landing page showing component count and category
  overview for each section
- [ ] wire sidebar items to load specimen pages in the content area
- [ ] verify clicking a sidebar item renders the corresponding specimen card

## Acceptance Criteria

- [ ] registry contains entries for all Jetstream-appropriate components
- [ ] clicking a component in sidebar renders its specimen card in content area
- [ ] hero header shows correct metadata for each component
- [ ] catalogue landing shows when no component is selected
- [ ] `cargo check` passes

## Next Task

Open `g10.003` and begin building structural primitive specimens.
