# 017 AppHeader Centre Region

Status: ready
Milestone: side-quest (component API, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-017-app-header-center-region`
Governing refs: `docs/contracts/components/app-header.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority

## Problem

`AppHeader` lays out three regions with
`grid-template-columns: minmax(0, 1fr) auto auto`, so `identity` takes the
leading space and `actions` then `utility` pack right. There is no way to place
a region in the true centre.

Five Tauri apps are migrating onto `AppHeader`. Four fit. `soundcheck` does
not: its header centres a destination `Tabs` group with

```css
grid-template-columns: minmax(9rem, 1fr) auto minmax(9rem, 1fr);
```

Verified at `~/Dev/projects/soundcheck/src/App.svelte:809`. The symmetric side
columns are what centres the middle; no arrangement of the existing three
regions reproduces it, and the `9rem` floor stops a wide side dragging the
centre off.

## Maintainer Rulings (already decided — do not re-litigate)

1. **An optional `center` snippet.** Its presence is the signal; there is no
   `layout` prop. A `layout: "centered"` with an empty middle would be a state
   with no meaning.
2. **When `center` is supplied**, the grid becomes symmetric:
   ```css
   grid-template-columns:
     minmax(var(--poodle-app-header-side-min, 0), 1fr)
     auto
     minmax(var(--poodle-app-header-side-min, 0), 1fr);
   ```
   with `actions` and `utility` sharing the trailing column, justified to the
   end exactly as now.
3. **When `center` is absent, nothing changes.** `minmax(0, 1fr) auto auto`
   applies unchanged and no current consumer shifts by a pixel. This is a
   hard requirement, not a preference — verify it.
4. **`--poodle-app-header-side-min` is exposed**, defaulting to `0`, so a
   consumer can set soundcheck's `9rem` collapse guard without hard-coding it.
5. **Narrow-width behaviour: reflow, do not stack.** At `max-width: 45rem` the
   default header collapses to `grid-template-columns: 1fr`. A header *with* a
   centre region must instead become `auto minmax(0, 1fr) auto` — one row, the
   centre absorbing the free space and no longer strictly centred.

   Evidence for the ruling: soundcheck already does exactly this at its own
   breakpoint (`App.svelte:931`, `≤760px` → `auto minmax(0, 1fr) auto`). It is
   also the only defensible behaviour for a titlebar-grade component — stacking
   four regions makes the bar tall, and a titlebar cannot grow. The default
   three-region collapse to `1fr` is unchanged.
6. **All four runtimes.** Not optional and not a judgement call: the Runtime
   Parity Authority rule states every component ships in Svelte, React, GPUI
   and Jetstream. The handoff offers this as "your call"; the standing rule
   already decided it.
7. **No free-form `columns` prop.** That leaks CSS through the API.

## Known State (verified)

- Grid at `packages/core/src/styles/app-header.css:13`; the `≤45rem` collapse at
  `:102-110`, which also switches `__utility` to `justify-content: flex-start`.
- Region classes today: `__identity`, `__actions`, `__utility` (plus
  `__title-group`, `__subtitle`). Rendered as three flat siblings in
  `AppHeader.svelte:53,67,73`.
- `AppHeader` already exposes a bindable `element` prop (`g13-b014`); do not
  disturb it.
- Two grid items cannot share one column side by side without a wrapper, so
  the trailing column needs one. Introduce it **only when `center` is present**,
  so the default DOM is untouched — ruling 3.

## Scope

### In scope

- `center` snippet/slot in Svelte and React; the equivalent optional region in
  `AppHeaderSpec` and `poodle-render`.
- Symmetric grid and the trailing wrapper, applied only when `center` exists.
- `--poodle-app-header-side-min`, documented.
- The reflow rule at `≤45rem`.
- Contract: the region, the custom property, the narrow-width behaviour, and a
  note that presence is the signal.
- Specimens in all four runtimes: with centre, without centre, and narrow.

### Out of scope — stop conditions if reached

- Changing the default three-region layout, its grid, or its DOM.
- A `columns` or `layout` prop.
- Migrating `soundcheck` or any other consumer.
- Refreshing visual baselines.
- `poodle-ir`.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Prove ruling 3.** A header without `center` must render byte-identical DOM
  and computed grid to before. Add a test asserting the default
  `grid-template-columns` and the unchanged region markup.
- Svelte and React must stay at parity — same slot name, same semantics.
- Contracts update in the same commit.
- Do not refresh baselines; a diff on a component other than app-header is a
  stop condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Another worker holds NavigationMenu, Tabs and `004-shared-control-types.md`.
  Touch none of them.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-017-app-header-center-region`. Do not merge.

## Writable Paths

- `packages/core/src/styles/app-header.css`
- `packages/svelte/components/src/AppHeader.svelte`
- `packages/react/components/src/AppHeader.tsx`
- `packages/contracts/components/src/app_header.rs`
- `packages/render/src/app_header.rs`
- `packages/{gpui,jetstream}/preview/src/specimens/app_header*.rs`
- `packages/{svelte,react}/preview/src/**/AppHeaderSpecimen.*`
- `docs/contracts/components/app-header.md`
- `packages/svelte/preview/src/component-docs.ts` (`app-header` entry only)
- Tests for AppHeader in either web runtime
- `docs/logs/2026-08/<DD>-g13-017-app-header-center-region.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Baseline: `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `cargo test -p poodle-render`, `git diff --check`.
2. Read `AppHeader.svelte`, `AppHeader.tsx`, `app-header.css`, and
   `app_header.rs` (spec and render) before changing anything.
3. Svelte + React: optional `center` region. When present, wrap `actions` and
   `utility` in a trailing container; when absent, emit today's DOM exactly.
4. CSS: symmetric grid and `--poodle-app-header-side-min` (default `0`) gated on
   the centre being present; trailing container justified to the end; the
   `≤45rem` reflow from ruling 5.
5. Rust: optional centre region on `AppHeaderSpec`, honoured in
   `poodle-render`, with the same presence-driven layout switch.
6. Specimens in all four runtimes: default, centred, and centred at narrow
   width. Svelte and React labels identical.
7. Contract: region, custom property, narrow-width reflow, and the
   presence-is-the-signal rationale.
8. Visual enumeration in report mode; classify every diff. Only app-header
   diffs are expected, and a header without `center` should produce **none**.
9. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:spec-drift
   cargo test -p poodle-render
   effigy docs:check
   git checkout -- packages/tokens/artifacts/rust/
   git diff --check
   git status --porcelain
   ```

## Acceptance Criteria

- [ ] `center` exists in Svelte, React, `AppHeaderSpec`, and `poodle-render`.
- [ ] With `center`, the grid is symmetric and `actions`/`utility` share the
  trailing column, justified end.
- [ ] Without `center`, DOM and computed grid are unchanged — asserted by test,
  and no visual diff.
- [ ] `--poodle-app-header-side-min` works and defaults to `0`.
- [ ] At `≤45rem`, a centred header reflows to `auto minmax(0, 1fr) auto`; the
  default header still collapses to `1fr`.
- [ ] Specimens in all four runtimes cover default, centred, and narrow.
- [ ] Contract documents all of the above.
- [ ] No baseline refreshed; diffs enumerated and classified.
- [ ] All step-9 commands exit 0.

## Stop Conditions

- The default header cannot be kept pixel-identical without a `layout` prop.
- The trailing wrapper forces a DOM change when `center` is absent.
- A visual diff appears outside app-header, or on a header without `center`.
- `poodle-node` cannot express the optional region for the native renderers.

Stop with exact paths, selectors, commands, and the smallest unresolved
question.
