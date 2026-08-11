# 016 Shared ActiveFill Type And NavigationMenu Switches

Status: ready
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-016-active-fill-navigation-menu`
Governing refs: `docs/contracts/004-shared-control-types.md` (`T1`–`T4`),
`docs/contracts/components/navigation-menu.md`,
`docs/contracts/components/tabs.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority

## Goal

Give `NavigationMenu` the same selection-rendering flexibility `Tabs` now has —
`activeOutline` and `activeFill` — and promote the shared type before it
fragments.

## Maintainer Rulings (already decided — do not re-litigate)

1. **`activeOutline: boolean`, default `false`** on NavigationMenu. Same
   semantics and same default as Tabs.
2. **This is an accepted visual change.** NavigationMenu's trigger currently
   carries a border by default (`navigation-menu.css:23`) and accents it when
   open. With `activeOutline` defaulting to `false`, the default rendering
   loses that border; consumers opt back in. NavigationMenu has **zero**
   consumers outside Poodle — verified across every project — so nothing
   breaks. Record the change in the contract; do not preserve the old default.
3. **`activeFill: ActiveFill`, default `"tint"`.** `tint` is the existing
   accent-tinted open state; `solid` fills with `accent-base` and switches the
   foreground to `text-inverse`.
4. **Promote the type.** `TabActiveFill` becomes `ActiveFill`, defined once in
   `004-shared-control-types.md` and referenced by both contracts. The TS side
   is currently an inline union in `Tabs.svelte`; give it a named type in both
   web runtimes' `types.ts`. Two components sharing a type defined in neither
   is exactly how `ButtonTone` fragmented.
5. **Do not add a third `activeFill` member.** `tint | solid` only.

## Known State (verified — build on this)

- NavigationMenu is fully four-runtime: contract, Svelte, React,
  `navigation_menu.rs` spec, `render/src/navigation_menu.rs`, and specimens in
  both native previews.
- Its trigger is a `<button class="poodle-navigation-menu__trigger">` directly
  in the `<nav>` list — there is no item wrapper and no close button, so unlike
  Tabs the trigger **is** the chip. Put the fill and outline on the trigger.
- Existing trigger styling to reconcile, not delete blindly:
  `navigation-menu.css:23` base border, `:34-36` `[data-open="true"]`
  background `accent 16%` + border-color `accent 42%`, `:39-41` hover/focus
  background `accent 12%`.
- Rust: `TabActiveFill` lives in `tabs.rs:21`, is re-exported from `lib.rs:280`,
  and is used in `render/src/tabs.rs:21,207,480`. All four sites move.
- Tabs' own switch CSS is the reference for behaviour — including that solid
  fill must survive `:hover`, which Tabs handles with a variant-agnostic rule
  one step more specific than any component's own hover.

## Scope

### In scope

- `ActiveFill` in `004-shared-control-types.md`; `TabActiveFill` renamed to
  `ActiveFill` in Rust with all references updated; named TS type in both web
  runtimes; `tabs.md` updated to reference the shared type rather than restate
  the union.
- `NavigationMenu`: both props in contract, CSS, Svelte, React,
  `NavigationMenuSpec`, `poodle-render`, and specimens in all four runtimes.
- Contract record of the default-appearance change.

### Out of scope — stop conditions if reached

- Changing Tabs' rendering. Only its *type reference* changes.
- A third `activeFill` value, or an `activeOutline` value other than boolean.
- Fixing the contract's recorded Svelte gap (trigger does not render
  `item.icon`). Leave it; it is unrelated debt.
- Any consumer repository.
- Refreshing visual baselines. NavigationMenu's default appearance changes, so
  diffs are expected — enumerate and classify them, refresh nothing.
- `poodle-ir`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read Tabs' `activeOutline`/`activeFill` CSS first and mirror its behaviour,
  including the hover-survival rule and the transparent reserve border that
  keeps layout stable.
- Svelte and React must stay at parity — same props, defaults, semantics.
- Contracts are authority and must be updated in the same commit.
- Do not refresh baselines; a diff outside navigation-menu is a stop condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-016-active-fill-navigation-menu`. Do not merge.

## Writable Paths

- `docs/contracts/004-shared-control-types.md`
- `docs/contracts/components/navigation-menu.md`
- `docs/contracts/components/tabs.md` (type reference only)
- `packages/core/src/styles/navigation-menu.css`
- `packages/svelte/components/src/{NavigationMenu.svelte,Tabs.svelte,types.ts}`
- `packages/react/components/src/{NavigationMenu.tsx,Tabs.tsx,types.ts}`
- `packages/contracts/components/src/{navigation_menu.rs,tabs.rs,lib.rs}`
- `packages/render/src/{navigation_menu.rs,tabs.rs}`
- `packages/{svelte,react}/preview/src/**/NavigationMenuSpecimen.*`
- `packages/{gpui,jetstream}/preview/src/specimens/navigation_menu.rs`
- `packages/svelte/preview/src/component-docs.ts` (`navigation-menu` entry only)
- Tests for either component in either web runtime
- `docs/logs/2026-08/<DD>-g13-016-active-fill-navigation-menu.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Baseline: `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `cargo test -p poodle-render`, `git diff --check`.
2. Promote the type: define `ActiveFill` in `004`, rename the Rust enum and fix
   all four reference sites, add the named TS type in both runtimes, and point
   `tabs.md` at the shared definition. Tabs' rendering must not change — prove
   it with the existing Tabs tests.
3. NavigationMenu CSS: reserve a transparent border on the trigger only when
   `activeOutline` is on, colour it on `[data-open="true"]`, and remove the
   unconditional base border. Add `activeFill="solid"` on the open trigger with
   `text-inverse` foreground, and make it survive `:hover` and `:focus-visible`
   — the existing hover rule at `:39-41` will otherwise override it.
4. NavigationMenu Svelte + React: both props, identical defaults, emitting
   `data-active-outline` and `data-active-fill` on the root.
5. Rust: both fields on `NavigationMenuSpec`, honoured in
   `render/src/navigation_menu.rs`.
6. Specimens in all four runtimes covering: default (no outline), outline on,
   solid fill, and solid fill hovered. Svelte and React labels identical.
7. Contract: document both props, their defaults, and the changed default
   appearance under a clear heading.
8. Visual enumeration: run the gate in report mode, table every changed
   baseline with slug/axis/ratio/cause, classify expected vs unexpected.
   NavigationMenu diffs are expected; anything else is a stop condition.
   **Refresh nothing.**
9. Validate:
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

- [ ] `ActiveFill` is defined once in `004` and referenced by both contracts;
  no inline `"tint" | "solid"` union remains in either web runtime.
- [ ] `TabActiveFill` is gone; all four Rust reference sites updated; Tabs
  rendering unchanged and its tests still pass.
- [ ] NavigationMenu has `activeOutline` (default `false`) and `activeFill`
  (default `"tint"`) in contract, both web runtimes, and `NavigationMenuSpec`.
- [ ] The unconditional trigger border is gone; the outline appears only when
  `activeOutline` is set, with no layout shift between states.
- [ ] Solid fill survives hover and focus-visible.
- [ ] Specimens in all four runtimes cover default, outline, solid, and solid
  hovered; Svelte and React labels identical.
- [ ] The contract records the changed default appearance explicitly.
- [ ] Visual diffs enumerated and classified; **no baseline file modified**.
- [ ] All step-9 commands exit 0.
- [ ] Batch log records commands, exit states, and the diff table.

## Stop Conditions

- A visual diff appears outside navigation-menu.
- Renaming `TabActiveFill` changes Tabs' rendering or fails its tests.
- `docs:value-domain-drift` reports a new finding for either component.
- Removing the base border cannot be done without a layout shift between
  `activeOutline` states.

Stop with exact selectors, paths, commands, and the smallest unresolved
question.
