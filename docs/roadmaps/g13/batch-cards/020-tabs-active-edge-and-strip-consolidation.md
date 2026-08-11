# 020 ActiveEdge, Strip Consolidation, And Tabs Defaults

Status: ready
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-020-active-edge-strip-consolidation`
Supersedes: `018-tabs-bordered-default-false.md` (folded in — three branches on
the same Tabs files would only conflict with each other)
Governing refs: `docs/contracts/004-shared-control-types.md` (`T1`–`T4`),
`docs/contracts/components/tabs.md`,
`docs/contracts/components/navigation-menu.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority

## Goal

Finish the Tabs and NavigationMenu selection API in one pass:

1. Replace `activeOutline: boolean` with a shared `ActiveEdge` type.
2. Delete the `strip` variant; its look becomes `block` + `activeEdge="underline"`.
3. Flip `bordered` to default `false`.

## Maintainer Rulings (already decided — do not re-litigate)

### R1 — `ActiveEdge` replaces `activeOutline`

```
type ActiveEdge = "none" | "outline" | "underline"
```

Default `"none"`. Shared control type, defined in
`004-shared-control-types.md` alongside `ActiveFill`, consumed by **both**
`tabs.md` and `navigation-menu.md`. `activeOutline` is removed from both.

**Why a single axis, not a third boolean.** Outline and underline are both
borders on the item — they conflict on the same property. That conflict already
bit once: `activeOutline` silently destroyed `strip`'s indicator, and it was
patched with strip-specific suppression rules. Adding `activeUnderline` as a
third boolean would re-create the conflict deliberately and admit nonsense
combinations. One enum makes the conflict unrepresentable.

`activeFill` (the fill axis) and `activeEdge` (the border axis) are orthogonal
and compose freely.

### R2 — `strip` is deleted; `block` absorbs it

Variant set becomes `card | pill | block`. `strip`'s look is
`block` + `activeEdge="underline"`.

`block` must absorb what `strip` had and `block` lacks:

- the list's inline padding (`--poodle-tabs-strip-inline-padding` becomes a
  block-side custom property; keep the value)
- the item hover background
- the close-button margin-end tweak
- `strip`'s vertical-orientation handling, where `block`'s is thinner

`block` keeps its own separators, full-width behaviour, and — verified in the
maintainer's screenshot — its correct rounded-corner handling against a rounded
container, which is one of the reasons `strip` is the one being removed.

Delete the strip-specific `activeOutline`/`activeFill` suppression rules added
earlier. With `underline` as an `activeEdge` value they are unnecessary: the
edge axis is mutually exclusive by construction.

**`TabStrip` the component is untouched.** It is a different thing — the
tablist-only primitive underneath Tabs, native-only by design, with its own
contract and `TabStripSpec`. Do not modify, delete, or merge it. The name
similarity is coincidental.

### R3 — `bordered` defaults to `false`

`bordered` is currently the only Tabs decoration on by default. After R1 and R3,
`card` is a plain baseline with three opt-in decorations: `bordered`,
`activeEdge`, `activeFill`.

The failure modes are asymmetric: a tab strip over content with no separating
line is plainer but functional; a strip in a titlebar with an unwanted border
and its padding is visibly broken. The milder failure belongs in the default.

## Consumer Impact — record, do not fix here

All three are **silent** visual changes except the `strip` removal, which is a
loud type error.

| Change | Sites | Failure mode |
|---|---|---|
| `strip` removed | 7 props (soundcheck 6, longhorn 1) + 5 `[data-variant="strip"]` CSS overrides in loophole-legacy | loud for props, **silent** for CSS |
| `activeOutline` → `activeEdge` | 0 consumers | none |
| `bordered` default false | ~45 panel-rendering usages | **silent** |

The consumer sweep handles all of it. **Do not edit any consumer repository.**

## Scope

### In scope

- `ActiveEdge` in `004`; `activeOutline` removed from Tabs and NavigationMenu
  in contract, both web runtimes, both Specs, and `poodle-render`.
- `strip` removed from the variant union everywhere; `block` absorbs the listed
  behaviours; strip-specific suppression rules deleted.
- `bordered` default flipped in Svelte, React, and `TabsSpec`.
- Specimens in all four runtimes: `block` + each `activeEdge` value, panel
  specimens passing `bordered` explicitly, no strip specimens.
- Contracts for both components.

### Out of scope — stop conditions if reached

- `TabStrip` (component, contract, spec, or native specimens).
- Any consumer repository.
- A fourth `activeEdge` value, or reintroducing a boolean form.
- Refreshing visual baselines.
- `poodle-ir`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read the existing `activeOutline`/`activeFill` CSS and the strip suppression
  rules first — you are replacing them, so understand what they solved.
- Svelte, React and Rust must agree exactly on every default.
- Contracts update in the same commit.
- Do not refresh baselines. Tabs and navigation-menu diffs are expected;
  a diff on any other component is a stop condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- The writable list below may be incomplete for compile-breaking callers —
  `node_compat.rs` and jetstream `compat.rs` have needed edits twice now.
  Making them compile is in scope; record any such edit in the batch log.
- Stage only changed paths by explicit path. **Never `git add -A`**, and never
  stage a file containing conflict markers.
- Commit and push with
  `git push -u origin thread/g13-020-active-edge-strip-consolidation`.
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
- Compile-breaking callers as needed (`node_compat.rs`, `compat.rs`)
- Tests for either component in either web runtime
- `docs/logs/2026-08/<DD>-g13-020-active-edge-strip-consolidation.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `cargo test -p poodle-render`, `git diff --check`.
2. Define `ActiveEdge` in `004`; replace `activeOutline` in both components
   across all four runtimes.
3. Port strip's absorbed behaviours into `block`, then delete the `strip`
   variant and its suppression rules.
4. Flip the `bordered` default in all three type surfaces.
5. Specimens in all four runtimes; Svelte and React labels identical.
6. Tests: defaults asserted in all three surfaces (`activeEdge` `"none"`,
   `bordered` `false`); `block` + `activeEdge="underline"` renders the
   underline; `outline` and `underline` cannot both apply.
7. Visual enumeration in report mode; classify. Refresh nothing.
8. Validate:
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

- [ ] `ActiveEdge` defined once in `004`, consumed by both contracts; no
  `activeOutline` remains anywhere.
- [ ] Variant union is `card | pill | block` in contract, both web runtimes and
  Rust; no `strip` remains in Tabs.
- [ ] `block` + `activeEdge="underline"` reproduces strip's indicator, and
  block retains separators, full-width, and correct rounded-corner handling.
- [ ] Strip-specific suppression rules are gone.
- [ ] `bordered` defaults to `false`, asserted in all three surfaces.
- [ ] `TabStrip` untouched.
- [ ] Specimens cover every `activeEdge` value in all four runtimes.
- [ ] No consumer repo, no baseline refreshed.
- [ ] All step-8 commands exit 0.
- [ ] Batch log records commands, exit states, the diff table, and any
  compile-breaking caller edited.

## Stop Conditions

- `block` cannot reproduce strip's indicator without reintroducing a
  strip-specific rule.
- Removing `strip` breaks `TabStrip` or a non-Tabs component.
- The three runtimes cannot agree on a default.
- A visual diff appears outside tabs and navigation-menu.

Stop with exact selectors, paths, commands, and the smallest unresolved
question.
