# Handoff — Poodle Icon Catalogue Boundary (Card 021)

Work in `~/Dev/projects/poodle`. Read `AGENTS.md` / `CLAUDE.md` first, then:

- `docs/roadmaps/g12/021-icon-catalogue-boundary.md`
- `docs/contracts/components/icon-provider.md`
- `docs/parity/icon-provider.md`
- `docs/roadmaps/g12/020-public-package-consolidation.md` (context, and its
  2026-08-09 amendment)

Northstar repo — cards, contracts and parity docs are the spine. Use the
`northstar` and `effigy` skills.

## Task

`poodle-core` stops vendoring the Lucide catalogue. Poodle owns the shape of an
icon and the registry that resolves one; it does not own a library of them.

## Already Measured — Do Not Re-derive

| | |
| --- | --- |
| icons shipped in `poodle-core` | 1,703 |
| distinct icons used across sixteen consuming repos | 134 (8%) |
| icons Poodle's own components use | 12 |
| raw on disk | 6.7 MB |
| gzipped | 245 KB — 84% of the `poodle-core` tarball |

They are **not** tree-shaken in practice. Soundcheck's production bundle
contains `biohazard`, `cassette`, `origami` and `squirrel`. It is an audio
application. That is the regression check for this card.

## The Cause Is One Line

`packages/svelte/components/src/icon-registry.ts:3`

    import * as lucideIcons from "@inflatable-cookie/poodle-core/icons";
    // …
    const nodes = iconModule[exportName] ?? [];

A namespace import of the whole barrel, then a lookup by computed string. No
bundler can prove reachability through a runtime-keyed property access, so
every icon is retained.

`generate.mjs` advertises "1703 Lucide icons available as tree-shakeable
exports" — true of the generated modules, defeated by their only consumer. The
per-icon exports were always the right shape; the catalogue-wide convenience
bolted on top is what broke them.

## Target

- **Keep the twelve icons Poodle's own components use**, exported as a small
  default `IconSet` so components render with no consumer wiring:
  `arrow-up-down`, `check-check`, `chevron-left`, `chevron-right`, `columns-2`,
  `diff`, `ellipsis`, `eye`, `pencil`, `refresh-cw`, `search`, `x`
- **Delete** the generated catalogue, `packages/core/src/icons/generate.mjs`,
  the `import *` fallback, and `lucide-static` from `poodle-core`'s
  `devDependencies`
- **Make an unresolved icon name loud.** Today `?? []` renders nothing and
  reports nothing
- **Consumers take `lucide-static` themselves** and pass an `IconSet`
- Tarball 291 KB → roughly 45 KB

## The Risk Is Ergonomic, Not Technical

Sixteen repositories currently write `icon="search"` and rely on the catalogue
resolving it. After this each wires an explicit set. That is the whole cost of
the change, and where it will fail if it fails.

**Land the helper and the documented pattern before migrating any consumer** —
something that builds an `IconSet` from `lucide-static`'s `icon-nodes.json`
given a list of names. If the wiring is unpleasant, consumers will reach for a
catalogue again and this reverts by accident.

Per-repo need is small. Loophole is the outlier at 38 icons; every other repo
is between 2 and 14. Enumerate a repo's names with:

    grep -rhoE 'icon=["'\''][a-z0-9-]+' <repo> --include='*.svelte' --include='*.ts' \
      | sed 's/.*[="'\'']//' | sort -u

That is the extraction the 134 figure came from. It catches the `icon="name"`
prop form only — check for other call sites (registry seeds, dynamically
constructed names) before declaring a repo migrated.

## Verify, Do Not Assume

The parity notes say GPUI uses a global registry and Jetstream resolves through
`ui_element::icon(name)`. Confirm neither depends on the npm catalogue before
deleting it. GPUI reads design tokens from `packages/tokens/artifacts/`
directly, so it is probably clean — but check rather than assume.

## Consumers

Sixteen repositories under `~/Dev/projects`: nucleus, loophole, soundcheck,
soundcheck-library, jetstream, figmatic, underlay, underlay-reference,
compli-me, composer, contact-patch, songsprout, finch, acowtancy/dairy,
acowtancy/froyo, acowtancy/cream.

All are clean and green as of 2026-08-09. Each pins Poodle by `file:` today —
leave those pins alone. A separate thread owns the move to published versions.

## Sequencing

This **blocks Longhorn Card 166** (first publication). Publishing 245 KB of
unused icon data as `0.1.0` means carrying it forever or breaking immediately
after the first release.

It also removes most of a clean-clone hazard recorded in that card: the
catalogue is gitignored and generated, so a CI publish today would ship
`poodle-core` with an empty `icons/` directory and nothing would fail.

## Other Active Threads — Do Not Collide

- **Longhorn GPUI host adapter** (Longhorn Card 163) — Rust only, no overlap
- **Release runway** (Longhorn Card 166 / g02.014) — owns npm publication,
  version pins, and consumer manifests

Touch neither. Confine changes to Poodle plus the consumer icon wiring.

## Verification

- `effigy ci` — currently green end to end
- `effigy test:svelte-pack-install` — the packed-consumer proof
- regression: rebuild a consumer bundle and `grep biohazard` it, expecting
  nothing
- `effigy tasks` for the selector list
