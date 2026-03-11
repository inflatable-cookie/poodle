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
4. `docs/roadmaps/g01/README.md`

## Current Direction

Pug is planned as:

- one semantic token system
- one docs-first component contract surface
- one Svelte implementation
- one GPUI implementation
- one Underlay bridge
- one explicit extension boundary for app-specific systems such as Loophole's
  DAW widgets

## Next Task

Open `docs/roadmaps/g01/002-token-system-and-artifact-emission.md` and freeze
the canonical token model before starting code packages.
