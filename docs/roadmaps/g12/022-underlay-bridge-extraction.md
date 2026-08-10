# 022 — Underlay Bridge Extraction

Status: ready
Roadmap: g12
Governing refs: `architecture/002-token-system-and-package-layout.md`; Card 020;
Card 021
Depends on: Card 021 complete
Blocks: nothing — the two consumers are already off every other path reference

## Objective

`packages/bridges/underlay` leaves Poodle. A design system must not carry a
package named after one of its consumers.

## The Dependency Points The Wrong Way

Poodle is the shared layer. Underlay is a product built on it. A package inside
Poodle called `poodle-bridge-underlay`, whose modules are `theme-map`,
`token-map`, `component-wrappers` and `nightfire-block-editor`, inverts that:
Poodle ends up holding knowledge of a specific consumer's block editor and its
`nightfire` concept, which exists nowhere else in the design system.

Its own manifest already says so — `publicIntent: false`, `channel: internal`,
`stability: internal-bridge`, `version: 0.0.0`, `private: true`. It was never
in the publish-intent set, and it is the only Poodle package a consumer still
references by path after the 2026-08-10 repoint.

## Measured 2026-08-10

| | |
| --- | --- |
| TypeScript | 490 lines across 6 modules |
| CSS | 1 file, `poodle-to-underlay.css` |
| Import sites in `underlay` | 1 |
| Import sites in the second consumer | 0 |

The single import is
`ts/src/nightfire/editor/NightfireFieldBlockShell.svelte`, which takes
`/nightfire-block-editor`. Everything else in the package is reachable only
through the barrel.

**The second consumer does not use it at all.** It declared the dependency and
aliased it in `vite.config.ts` and `svelte.config.js`, and no source file ever
imported it. That wiring was removed on 2026-08-10 with no code change, which
is the cheapest possible confirmation that the dependency was never real.

## Target

The bridge moves into the `underlay` repository, next to the `nightfire` code
that is its only caller. Poodle keeps no directory named after a consumer.

Nothing needs to be published. The bridge becomes internal source in the
repository that owns the concept, so the `file:` reference disappears rather
than being replaced by a version.

## Steps

1. Move the six modules and the stylesheet into `underlay`, under the
   `nightfire` area that already owns the only caller.
2. Repoint that one import from
   `@inflatable-cookie/poodle-bridge-underlay/nightfire-block-editor` to the
   new internal path.
3. Delete `packages/bridges/underlay` from Poodle, and the `bridges` directory
   if nothing else lives in it.
4. Drop the `file:` dependency from `underlay`'s manifest.
5. Check whether `poodle-to-underlay.css` maps tokens that the published
   `poodle-core` already exports. If it does, the mapping is a consumer-side
   theme and belongs in Underlay's own styles rather than being carried across
   verbatim.

## Acceptance Criteria

- no directory in Poodle is named after a consumer product
- `underlay` renders the Nightfire field block shell unchanged
- no repository references `@inflatable-cookie/poodle-bridge-underlay`
- Poodle's package list contains only publish-intent packages plus previews and
  fixtures

## Notes

Found while repointing consumers onto the published packages. Every other
consumer moved from `file:` to `^0.1.0` cleanly; this one could not, because
the package it pins was never going to be published. That is the useful signal
— a path reference that survives a publication sweep is usually pointing at
something that is in the wrong repository, not something that needs publishing.

The alternative was to publish it as a fourth Poodle package, which would have
made the layering error permanent and public.
