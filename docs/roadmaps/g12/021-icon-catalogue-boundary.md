# 021 — Icon Catalogue Boundary

Status: ready
Roadmap: g12
Governing refs: `contracts/components/icon-provider.md`;
`architecture/002-token-system-and-package-layout.md`; Card 020
Depends on: Card 020 complete
Blocks: Longhorn Card 166 step 1 (first publication)

## Objective

`poodle-core` stops vendoring the Lucide catalogue. Poodle owns the shape of an
icon and the registry that resolves one; it does not own a library of them.

## Measured 2026-08-09

| | |
| --- | --- |
| icons shipped in `poodle-core` | 1,703 |
| distinct icons used across the sixteen consuming repositories | 134 (8%) |
| icons Poodle's own components use | 12 |
| raw on disk | 6.7 MB |
| gzipped | 245 KB — **84% of the `poodle-core` tarball** |

They are not tree-shaken away in practice. Soundcheck's production bundle
contains `biohazard`, `cassette`, `origami` and `squirrel`. It is an audio
application.

## The Cause Is One Line

`packages/svelte/components/src/icon-registry.ts:3`

```ts
import * as lucideIcons from "@inflatable-cookie/poodle-core/icons";
// …
const nodes = iconModule[exportName] ?? [];
```

A namespace import of the whole barrel, then a lookup by computed string. No
bundler can prove which icons are reachable through a runtime-keyed property
access, so every one is retained.

`generate.mjs` writes "1703 Lucide icons available as tree-shakeable exports".
That is true of the generated modules and defeated by their only consumer. The
per-icon exports were always the right shape; the catalogue-wide convenience
bolted on top is what broke them.

## Card 020 Needs Amending

Card 020 says folding `icons-lucide` into core "is not a cost", on the grounds
that `lucide-static` is referenced only by `generate.mjs` and never at runtime.
That is true of the **dependency** and says nothing about the **payload**,
which was not measured. The claim reads as though icons were free. Amend it to
say what was actually checked.

## The Architecture Is Already Right

`IconProvider` takes an `IconSet`, `setIconSet` exists, `IconNodes` is the
contract type, and the GPUI and Jetstream tiers resolve through their own
registries. Nothing here changes the component contract. What changes is that
Poodle stops shipping a default catalogue behind that seam.

## Target

- **`poodle-core` keeps the twelve icons its own components use** —
  `arrow-up-down`, `check-check`, `chevron-left`, `chevron-right`, `columns-2`,
  `diff`, `ellipsis`, `eye`, `pencil`, `refresh-cw`, `search`, `x` — exported
  as a small default `IconSet` so components render with no consumer wiring.
- **The 1,703-module catalogue and `generate.mjs` are removed**, along with
  `lucide-static` from `poodle-core`'s `devDependencies`.
- **The `import *` fallback in `icon-registry.ts` is deleted.** Resolution is
  the provided `IconSet` plus the built-in twelve, and an unresolved name is a
  visible failure rather than a silent empty array.
- **Consumers take `lucide-static` themselves** and pass an `IconSet`.

## The Risk Is Ergonomic, Not Technical

Sixteen repositories currently write `icon="search"` and rely on the catalogue
resolving it. After this they each wire an explicit set. That is the whole cost
of the change and it is where it will fail if it fails.

Two things make it tractable. The 134 names in use are already enumerated, so
each application's set is derivable rather than discovered. And most
applications need ten to thirty icons, not a catalogue.

Provide a documented pattern — a helper that builds an `IconSet` from
`lucide-static`'s `icon-nodes.json` given a list of names — before migrating
any consumer. If the wiring is unpleasant, consumers will reach for the
catalogue again and this reverts by accident.

## Steps

1. Add the twelve-icon built-in default and the `IconSet` helper.
2. Delete the `import *` fallback, the generated catalogue, `generate.mjs`, and
   the `lucide-static` devDependency.
3. Make an unresolved icon name loud — the current `?? []` renders nothing and
   reports nothing.
4. Verify the GPUI and Jetstream registries never depended on the npm
   catalogue. The parity notes say GPUI uses a global registry and Jetstream
   resolves through `ui_element::icon(name)`; confirm rather than assume.
5. Update the icon-provider contract and parity docs.
6. Migrate the sixteen consumers, using the enumerated names.

## Acceptance Criteria

- `poodle-core` ships no Lucide catalogue and no `lucide-static` dependency
- the tarball drops from 291 KB to roughly 45 KB
- a production bundle of any consumer contains no icon it does not use —
  check `biohazard` against soundcheck as the regression
- Poodle's own components render with no consumer-provided `IconSet`
- an unresolved icon name fails visibly
- every consumer still renders the icons it did before

## Timing

**Before first publication.** This changes what `poodle-core` contains, and
publishing 245 KB of unused icon data as `0.1.0` means either shipping it
forever or a breaking change immediately after the first release.

It also mostly dissolves the clean-clone hazard recorded in Longhorn Card 166:
with no generated catalogue there is little left for a release workflow to
generate before packing, and the risk of publishing an empty `icons/`
directory from CI goes with it.

## Notes

Poodle predates the current agent tooling, and the catalogue was the thing that
made icons work during development. It did its job. The measurement is only
possible now because there are sixteen real consumers to measure against.
