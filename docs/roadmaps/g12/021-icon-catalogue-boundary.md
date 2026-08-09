# 021 — Icon Catalogue Boundary

Status: complete
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
| icons Poodle's own components use | 54 |
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

## Corrected Internal Census

The first census counted only literal `icon="name"` props and reported twelve.
It missed literal `name="name"` uses plus bounded mappings in components such
as MarkdownEditor, Callout, ToolCall, MediaThumbnail, RefSelect, and
CollapseToggle. The verified default dependency set is 54 icons. Keeping only
twelve would break Poodle components without consumer wiring, contradicting
the acceptance criteria.

## Target

- **`poodle-core` keeps the 54 Lucide icons its own components can emit**, as
  a scoped default `IconSet` so component chrome renders with no consumer
  wiring. This is Poodle's default Lucide adapter, not a general built-in
  catalogue.
- **The 1,703-module catalogue and `generate.mjs` are removed**, along with
  `lucide-static` from `poodle-core`'s `devDependencies`.
- **The `import *` fallback in `icon-registry.ts` is deleted.** Resolution is
  the provided `IconSet` plus the default set, and an unresolved name is a
  visible failure rather than a silent empty array.
- **Consumers take `lucide-static` themselves** and generate a scoped `IconSet`
  from an explicit name list. The full JSON catalogue never enters application
  source or the runtime module graph.

## The Risk Is Ergonomic, Not Technical

Sixteen repositories currently write `icon="search"` and rely on the catalogue
resolving it. After this they each wire an explicit set. That is the whole cost
of the change and it is where it will fail if it fails.

Two things make it tractable. The 134 names in use are already enumerated, so
each application's set is derivable rather than discovered. And most
applications need ten to thirty icons, not a catalogue.

Provide a documented pattern before migrating any consumer:

```json
["search", "trash-2"]
```

```sh
bun x poodle-icons --names icons.json --out src/icons.generated.ts
```

```ts
import { icons } from "./icons.generated";
```

The generated module contains only those node arrays. A runtime pick from a
default JSON import does not tree-shake, and Vite cannot expose kebab-case JSON
keys as named imports, so extraction must happen before bundling. If the wiring
is unpleasant, consumers will reach for a catalogue again and this reverts by
accident.

## Steps

1. Add the 54-icon default Lucide set and the `IconSet` helper.
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
- the tarball drops materially from the 291,471-byte baseline
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

## Completion — 2026-08-09

The default Lucide adapter contains the verified 54-icon Poodle dependency
set. Application extensions are generated from a JSON name list into a
self-contained TypeScript module. The generated module has no runtime import
of `lucide-static` or `poodle-core`.

Consumer posture:

| Posture | Repositories |
| --- | --- |
| generated application set + root provider | nucleus, loophole, soundcheck, jetstream, figmatic, underlay-reference, compli-me, composer, finch, private-consumer/dairy |
| generated direct library nodes | underlay |
| host sets cover reusable library names | soundcheck-library |
| default set already covers production use; no wiring added | contact-patch, songsprout, private-consumer/froyo, private-consumer/cream |

Measured results:

- `poodle-core` packed size: 291,471 → 159,553 bytes, a 45% reduction. The
  original roughly-45-KB estimate did not account for the rest of core's
  current source and style payload.
- Soundcheck production JavaScript: 1,400.40 → 1,033.35 KB; gzip 366.36 →
  282.97 KB.
- `biohazard`, `cassette`, `origami`, and `squirrel` are absent from the clean
  Soundcheck bundle.
- `effigy ci` and `effigy test:svelte-pack-install` pass.

## Notes

Poodle predates the current agent tooling, and the catalogue was the thing that
made icons work during development. It did its job. The measurement is only
possible now because there are sixteen real consumers to measure against.
