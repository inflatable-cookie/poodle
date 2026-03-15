# g09.007 — Specimen Upgrade: Structural and Layout Primitives

Status: complete
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.002
Primary repos: `pug`

## Goals

- [ ] replace all hand-built mockup specimens for structural components with
  specimens using real `Pug*` component instances
- [ ] each specimen should demonstrate variants, sizes, and states matching
  the Svelte specimen

## Execution Checklist

- [ ] rewrite `bx.rs` specimen to use `PugBox` with padding variants and
  overflow demonstrations
- [ ] rewrite `stack.rs` specimen to use `PugStack` with gap, alignment,
  and nested children examples
- [ ] rewrite `grid.rs` specimen to use `PugGrid` with column count and
  responsive behavior
- [ ] rewrite `surface.rs` specimen to use `PugSurface` showing all tone
  variants (Default, Subtle, Elevated, Overlay)
- [ ] rewrite `separator.rs` specimen to use `PugSeparator` in both
  horizontal and vertical orientations with tone variants
- [ ] rewrite `scroll_shell.rs` specimen to use `PugScrollShell` with
  scrollable content demonstrating overflow behavior
- [ ] rewrite `banner.rs` specimen to use `PugBanner` showing info, warning,
  error, and success tones with dismissible variant
- [ ] rewrite `callout.rs` specimen to use `PugCallOut` showing all tone
  variants with title and content
- [ ] create `inline.rs` specimen using `PugInline` with wrapped children
- [ ] create `spacer.rs` specimen using `PugSpacer` between flex items
- [ ] update `mod.rs` slug routing for any new specimen files
- [ ] verify all specimen slugs render without panic

## Acceptance Criteria

- [ ] zero hand-built `div()` chains remain in structural specimen files
- [ ] every structural specimen uses real `Pug*` component constructors
- [ ] variants shown match the Svelte specimen (tone, size, orientation)
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.008` and upgrade action and input specimens.
