# Poodle

Poodle is a generalized UI system for teams that need the same documented
component contract implemented in both Svelte and GPUI.

It is intended to serve:

- desktop/workstation apps such as Loophole through shared tokens, primitives,
  and reusable shell components
- Underlay-based web apps through internal token and component adoption without
  leaking Poodle into app code

## Documentation System

Poodle uses a Northstar-shaped documentation surface under `docs/`.

Start here:

1. `docs/vision/001-poodle-vision.md`
2. `docs/architecture/001-poodle-system-shape.md`
3. `docs/roadmaps/README.md`
4. `docs/roadmaps/g04/README.md`

## Current Direction

Poodle is planned as:

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

Open `docs/roadmaps/g04/015-gpui-demo-app-parity-implementation-and-side-by-side-review.md`
and implement the same shared demo app in GPUI against the rebuilt Svelte
target, with side-by-side review as the primary parity check.
