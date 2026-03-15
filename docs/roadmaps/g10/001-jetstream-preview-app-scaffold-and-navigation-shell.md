# g10.001 — Jetstream Preview App Scaffold and Navigation Shell

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g08.014
Primary repos: `pug`, `jetstream`

## Goals

- [ ] create a standalone Jetstream preview application for showcasing Pug
  components in the Jetstream runtime
- [ ] establish the window, navigation shell, and build target

## Execution Checklist

- [ ] create `packages/jetstream/preview/` directory with Cargo.toml
  depending on `pug-jetstream`, `jetstream-runtime`, `jetstream-renderer`
- [ ] implement main entry point that creates a Jetstream window with title
  "Pug — Jetstream Preview"
- [ ] implement navigation shell using `UiTree`:
  - [ ] top section tab bar with 4 tabs: Primitives, Composites, Demo, Tokens
  - [ ] left sidebar area for component catalogue listing
  - [ ] main content area for specimen display
- [ ] implement tab switching via `UiEvent` click handling on tab nodes
- [ ] implement sidebar scroll for long component lists using `Widget::List`
- [ ] implement content area with scroll support
- [ ] wire keyboard navigation: Tab cycles sections, arrow keys navigate
  sidebar
- [ ] set up build script in Cargo.toml and verify `cargo run` launches
  the preview window
- [ ] implement basic theme loading from Pug tokens via `PugTheme` bridge

## Acceptance Criteria

- [ ] `cargo run -p pug-jetstream-preview` opens a window with visible
  tab bar, sidebar, and content area
- [ ] clicking section tabs switches sidebar content
- [ ] theme colors from Pug tokens are applied to the shell background,
  borders, and text
- [ ] window resizing reflows the layout correctly
- [ ] `cargo check` passes with zero errors

## Next Task

Open `g10.002` and implement the component registry and specimen framework.
