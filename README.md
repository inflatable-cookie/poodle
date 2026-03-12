# Pug

Pug is a generalized UI system for teams that need the same documented
component contract implemented in both Svelte and GPUI.

It is intended to serve:

- desktop/workstation apps such as Loophole through shared tokens, primitives,
  and reusable shell components
- Underlay-based web apps through internal token and component adoption without
  leaking Pug into app code

## Documentation System

Pug uses a Northstar-shaped documentation surface under `docs/`.

Start here:

1. `docs/vision/001-pug-vision.md`
2. `docs/architecture/001-pug-system-shape.md`
3. `docs/roadmaps/README.md`
4. `docs/roadmaps/g03/README.md`

## Current Direction

Pug is planned as:

- one semantic token system
- one docs-first component contract surface
- one Svelte implementation
- one GPUI implementation
- one Underlay bridge
- one explicit extension boundary for app-specific systems such as Loophole's
  DAW widgets

## Local Preview

The repo now includes a browser inspection surface at
`packages/svelte/preview`.

Run:

```sh
bun install
bun run tokens:build
bun run docs:dev
```

Then open `http://localhost:4173`.

The docs surface now includes theme inspection, form and composite baselines,
command discovery, workstation shell depth, dock regions, split views, surface
tabs, token provenance, family navigation, and a host-owned layout snapshot
readout.

## Next Task

`g03` is complete. If a future generation is opened, start from the explicit
closeout in `docs/roadmaps/g03/014-generation-closeout-and-next-program-cutover.md`
instead of reopening parity, accessibility, release, acceptance, or adoption
baselines by implication.
