# Architecture

Status: active
Updated: 2026-03-23

Architecture defines how Poodle is structured, where package ownership lives, and how Svelte, GPUI, Jetstream, tokens, and the Underlay bridge fit together.

## Active Baseline

- `001-poodle-system-shape.md`
- `002-token-system-and-package-layout.md`
- `003-component-docs-ia-and-implementation-substrates.md`
- `004-underlay-bridge-and-adapter-ownership.md`
- `005-treatment-system-and-recipe-variables.md`

## Working Rule

Keep architecture focused on ownership, layering, package boundaries, renderer responsibilities, and bridge constraints.
Push milestone sequencing and execution detail into `roadmaps/`.

## Next Task

Reconcile any architecture page that still assumes old repo names or superseded generation structure, then keep new structural changes landing here before they spread into roadmap prose.
