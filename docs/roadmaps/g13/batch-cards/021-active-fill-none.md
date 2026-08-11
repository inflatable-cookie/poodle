# 021 ActiveFill Gains `none`

Status: merged (`239e9776` → `33a307d3`)
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-021-active-fill-none`
Depends on: `g13-b020` merged (`2115e5c1`)
Governing refs: `docs/contracts/004-shared-control-types.md`,
`docs/contracts/components/tabs.md`,
`docs/contracts/components/navigation-menu.md`

## Problem

`g13-b020` deleted the `strip` variant on the premise that
`block` + `activeEdge="underline"` reproduces it. It does not, quite: strip
marked selection with **an underline and no fill**, and `block` always fills.

The two axes came out asymmetric:

```
ActiveEdge = "none" | "outline" | "underline"
ActiveFill =          "tint"    | "solid"
```

The edge axis can be switched off; the fill axis cannot. That is an oversight in
`020`, not a deliberate asymmetry.

## Ruling

```
type ActiveFill = "none" | "tint" | "solid"
```

Default stays `"tint"`. With this, `block` + `activeFill="none"` +
`activeEdge="underline"` is exactly the old strip, and the two axes are
symmetric: each has an off value, and they compose freely.

**Amend `004`.** Its `ActiveFill` section currently reads "There are exactly two
members; a third value is a contract violation (T2), not an extension." That
line was written before `ActiveEdge` existed and is now the odd one out.
Replace it — the constraint that matters is that members are added by ruling
and recorded here, not that the count is frozen. `T2` forbids a *contract*
restating a shared type with fewer members; it does not freeze the type.

## Consumer Impact

None. `"none"` is additive and the default is unchanged, so no existing usage
renders differently.

This card must land **before the consumer sweep**: the sweep migrates
`variant="strip"` to its block equivalent, and without `activeFill="none"` it
would write a fill those call sites never had.

## Scope

### In scope

- `ActiveFill` gains `"none"` in `004`, both contracts, both web runtimes'
  `types.ts`, the Rust enum, and both renderers.
- CSS: `[data-active-fill="none"]` suppresses the selected fill for **both**
  Tabs (all variants) and NavigationMenu. Selected text colour and the edge
  treatment are unaffected — only the fill goes.
- Specimens in all four runtimes showing the strip equivalent:
  `block` + `activeFill="none"` + `activeEdge="underline"`.
- A test in each of Svelte, React and Rust that `"none"` renders no selected
  fill while the underline still renders.

### Out of scope — stop conditions if reached

- Changing the `"tint"` default.
- Reinstating the `strip` variant.
- Touching `TabStrip` (component, contract, spec, native specimens).
- Any consumer repository.
- Refreshing visual baselines.
- `poodle-ir`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read how `activeEdge="none"` is implemented and mirror it — the two axes
  should be symmetric in code as well as in the type.
- Svelte, React and Rust must agree exactly on members and default.
- Contracts update in the same commit.
- Do not refresh baselines. Tabs and navigation-menu diffs are expected; a diff
  on any other component is a stop condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- The writable list below has been incomplete on three cards running. Compile-
  breaking callers — `DockRegion`, `node_compat.rs`, jetstream `compat.rs`,
  specimens — are in scope to fix; record every such edit in the batch log.
- Stage changed paths explicitly. **Never `git add -A`**, and never stage a file
  containing conflict markers.
- Commit and push with `git push -u origin thread/g13-021-active-fill-none`.
  Do not merge.

## Writable Paths

- `docs/contracts/004-shared-control-types.md`
- `docs/contracts/components/{tabs.md,navigation-menu.md}`
- `packages/core/src/styles/{tabs.css,navigation-menu.css}`
- `packages/svelte/components/src/{Tabs.svelte,NavigationMenu.svelte,types.ts}`
- `packages/react/components/src/{Tabs.tsx,NavigationMenu.tsx,types.ts}`
- `packages/contracts/components/src/{tabs.rs,navigation_menu.rs,lib.rs}`
- `packages/render/src/{tabs.rs,navigation_menu.rs}`
- `packages/{gpui,jetstream}/preview/src/specimens/{tabs.rs,navigation_menu.rs}`
- `packages/{svelte,react}/preview/src/**/{TabsSpecimen.*,NavigationMenuSpecimen.*}`
- `packages/svelte/preview/src/component-docs.ts` (`tabs`, `navigation-menu`)
- Compile-breaking callers as needed
- Tests for either component in either web runtime
- `docs/logs/2026-08/<DD>-g13-021-active-fill-none.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `cargo test -p poodle-render`, `git diff --check`.
2. Add `"none"` to the type in all surfaces; amend `004` including the
   "exactly two members" line.
3. CSS: suppress the selected fill under `[data-active-fill="none"]` for Tabs
   and NavigationMenu, leaving text colour and edge treatment intact.
4. Specimens in all four runtimes for the strip equivalent.
5. Tests in Svelte, React and Rust.
6. Visual enumeration in report mode; classify. Refresh nothing.
7. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:spec-drift
   effigy docs:value-domain-drift
   cargo test -p poodle-render
   cargo test -p poodle-specs
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```

## Acceptance Criteria

- [ ] `ActiveFill` is `"none" | "tint" | "solid"` in `004`, both contracts, both
  `types.ts`, the Rust enum and both renderers; default still `"tint"`.
- [ ] `004`'s "exactly two members" line is replaced.
- [ ] `activeFill="none"` suppresses the selected fill on Tabs and
  NavigationMenu without affecting text colour or the edge.
- [ ] `block` + `activeFill="none"` + `activeEdge="underline"` reproduces the
  former strip look, with a specimen showing it in all four runtimes.
- [ ] Tests in all three surfaces.
- [ ] No consumer repo, no baseline refreshed, `TabStrip` untouched.
- [ ] All step-7 commands exit 0.
- [ ] Batch log records commands, exit states, the diff table, and any
  compile-breaking caller edited.

## Stop Conditions

- Suppressing the fill cannot be done without also losing the edge or the
  selected text colour.
- The three runtimes cannot agree on members or default.
- A visual diff appears outside tabs and navigation-menu.

Stop with exact selectors, paths, commands, and the smallest unresolved
question.
