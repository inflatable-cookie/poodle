# 020 — Public Package Consolidation

Status: complete (2026-08-08)
Roadmap: g12
Governing refs: `specs/022-packaging-versioning-and-release-channel-rules.md`;
`architecture/002-token-system-and-package-layout.md`

## Objective

Six publish-intent npm packages become three, grouped by framework binding,
before publication freezes the names.

## Why

### Two framework-agnostic packages were filed under `svelte/`

`packages/svelte/tokens` published as `@inflatable-cookie/poodle-svelte-tokens`
and `packages/svelte/icons-lucide` as `@inflatable-cookie/poodle-icons-lucide`.
Neither is Svelte-specific:

- the icons package holds 1,703 generated `IconNodes` data arrays and **zero
  `.svelte` files**
- `poodle-react` depends on both

So the name and the location were already wrong, and a React consumer was
installing something called `svelte-tokens`. This is the defect; the package
count is the symptom.

### The axis here is framework binding, not peer requirement

Longhorn's Card 164 merged its eighteen TypeScript packages by *peer
requirement*, because its domain packages had no dependency tree worth
pruning. That reasoning does not transfer. Poodle has a real shared base —
headless logic, styles, tokens, icons — consumed by two framework
implementations whose peers are mutually exclusive.

Grouping on that axis gives three packages and no judgement calls:

```
@inflatable-cookie/poodle-core     headless + styles + tokens + icons, no peers
@inflatable-cookie/poodle-svelte   peer: svelte
@inflatable-cookie/poodle-react    peers: react, react-dom
```

### Nobody takes a base package without a framework

Measured across the eighteen portfolio repositories that consume Poodle: every
one of them uses `poodle-svelte`, and none uses `headless`, `styles`, `tokens`
or `icons` without it. The base packages have never been installed on their
own.

### The dependency cost is not a cost — the payload cost was not measured

Folding `icons-lucide` into core looked like it would force `lucide-static` on
every consumer. It does not: the icon modules are generated and gitignored,
and `lucide-static` is referenced only by `generate.mjs`, never by anything
under `src`. It was a build-time dependency declared as a runtime one, and is
now a `devDependency` of core.

**Amended 2026-08-09.** That is true of the dependency and says nothing about
the payload, which this card did not measure and which reads here as though
icons were free. They are not: the catalogue is 245 KB gzipped, 84% of the
`poodle-core` tarball, and a namespace import in `icon-registry.ts` defeats
tree-shaking so all 1,703 reach every consumer bundle. Card 021 removes the
catalogue. Consolidating the icons out of `packages/svelte/` was still correct
— they are framework-agnostic and were misfiled — but core was the wrong
destination.

## Landed Shape

`poodle-core` carries 33 export entries:

| Prefix | From |
| --- | --- |
| `.` | `packages/core` (the former `poodle-headless`) |
| `./styles/*` | `packages/styles` |
| `./tokens`, `./tokens/*` | `packages/svelte/tokens` — 29 entries including every theme, density and control-size CSS file |
| `./icons`, `./icons/*` | `packages/svelte/icons-lucide` |

`poodle-svelte` and `poodle-react` are unchanged in name and surface. The
`0.0.0` internals are untouched: `packages/tokens` (the generator), both
previews, `install-smoke`, and `bridges/underlay`.

The token generator now writes to `packages/core/src/tokens/generated`. GPUI
reads `packages/tokens/artifacts/` directly and is unaffected.

## Fixed In The Same Pass, And One Longhorn Bug Found

`SplitToggleVisibility` was defined in `packages/svelte/components/src/types.ts`
and never re-exported from the root barrel, so a consumer typing `SplitView`'s
`toggleVisibility` prop had to reach past the package root — which Longhorn's
contract 012 forbids. Longhorn had worked around it by deriving the type from
`ComponentProps`. It is now exported.

Rebuilding the packs also settled a long-standing Longhorn test failure that
had been blamed on stale artifacts. `split.test.ts` asserted that a
`primaryHidden` pane sets `data-primary-collapsed`. It does not, and never
will: this contract states that a hidden pane is "absent, not a collapse: no
toggle, no collapsed data attribute". Poodle was right and the Longhorn test
contradicted the very card that introduced it. Fixed on the Longhorn side; no
change here.

## Evidence

- `effigy ci` green: `test:core`, `test:components`, `check:svelte`,
  `docs:lint`, `report:parity`, `report:accessibility`, `test:contracts`
- `test:svelte-pack-install` green over two tarballs rather than five, proving
  a real consumer install of packed `poodle-core` and `poodle-svelte` with no
  workspace dependency surviving into either tarball
- `check:svelte` type-checks 2,178 files through the install-smoke consumer
  with 0 errors

## Consumer Exposure

Eighteen repositories consume Poodle and all of them change, but the change is
small in each: `poodle-svelte` — by far the most-imported package — keeps its
name, so only `headless`, `styles`, `tokens` and `icons` imports move, roughly
200 files across the whole portfolio.

`install-smoke` needed an `overrides` block. It sits outside the workspace on
purpose, so `poodle-svelte`'s dependency on `poodle-core@0.1.0` no longer
resolved once the base packages stopped being separately installable siblings.
Consumers pinning Poodle by `file:` reference need the same block.

## Timing

Before publication. Poodle has no tags, so the names are not frozen and this
is free now; publishing six and then collapsing to three would mean
deprecating three published names.
