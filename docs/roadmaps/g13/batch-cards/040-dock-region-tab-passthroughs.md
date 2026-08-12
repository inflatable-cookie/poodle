# 040 DockRegion — Tab Pass-throughs, And The Underline's Missing Hook

Status: ready (hold dispatch until `g13-b038` merges — both write `tabs.css`)
Milestone: side-quest (component surface, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-040-dock-region-tab-passthroughs`
Depends on: `g13-b038` — file contention only, not a logical dependency
Governing refs: `docs/contracts/components/dock-region.md`,
`docs/contracts/components/tabs.md`,
`docs/contracts/003-recipe-hooks.md` (the hook grammar)

## Goal

Loophole cannot turn DockRegion's active-tab underline off, so it carries a
scoped CSS override on `.poodle-tabs__item[data-selected]` — exactly the
class-targeting tier the recipe contract exists to eliminate. Two Poodle
changes retire it.

The report asked for one prop. The maintainer's instruction is broader: **a way
to customise the dock's tabs fully, not just this one prop.** R1 answers that
by ruling on the whole surface rather than adding `tabActiveEdge` and waiting
for the next report.

## Current State — Measured

DockRegion instantiates `Tabs` at three call sites and passes eleven props.
Sorted by who should own them:

| Prop | Today | Owner |
|---|---|---|
| `variant` | forwarded as `tabVariant` | consumer ✅ already |
| `size` / `sizeRole` / `density` | forwarded | consumer ✅ already |
| `activeEdge` | **hard-coded `"underline"`** | consumer ❌ the report |
| `activeFill` | **never passed** — Tabs' default | consumer ❌ |
| `bordered` | **never passed** — Tabs' default | consumer ❌ |
| `reorderable` | **hard-coded `true`** | consumer ❌ |
| `orientation` | per branch | **DockRegion** — it is the layout |
| `items` / `value` / `onValueChange` / `onClose` / `onReorder` | wired internally | **DockRegion** — it owns the panel model |
| `ariaLabel` | derived from `edge` | **DockRegion**, with the existing `ariaLabel` prop already overriding |
| `showTooltips` | derived from `isCompact` | **DockRegion** — compaction is its logic (`g13` dock-tabs fix) |

And the hook gap, which is narrower than "the underline bypasses recipes":

- `data-active-edge="outline"` **has** a hook —
  `--poodle-recipe-tabs-active-outline-border` (`tabs.css:409`).
- `activeFill="solid"` **has** hooks — `--poodle-recipe-tabs-active-solid-fill`
  and `-text` (`tabs.css:441`, `:454`).
- `data-active-edge="underline"` has **none**, in either orientation:
  `tabs.css:423` sets `border-bottom-color: var(--poodle-color-accent-base)`
  and `tabs.css:434` sets `border-right-color` the same way.

Underline is the only active-edge treatment missing the hook its siblings have.

## Fixed By Ruling (do not re-decide)

### R1 — Forward the four presentation props. Nothing else.

Add, all following the established `tab`-prefix convention that `tabVariant`
set:

- `tabActiveEdge?: ActiveEdge` — default `"underline"`
- `tabActiveFill?: ActiveFill` — default matching Tabs' current effective value
- `tabBordered?: boolean` — default matching Tabs' current effective value
- `tabReorderable?: boolean` — default `true`

Every default preserves today's rendering exactly. This is additive; no
existing consumer changes.

Forward all four at **all three** call sites. `reorderable` being hard-coded
`true` is a real limitation in its own right — a dock whose panels must not be
reordered currently has no way to say so.

**Do not expose the DockRegion-owned rows in the table above.** `orientation`,
the panel model, and `showTooltips` are DockRegion's, and handing them to a
consumer would let it break the dock's own layout and the compaction logic.
Say so in the contract, so the next report knows the line was drawn on purpose
rather than missed.

### R2 — Give underline the hook its siblings already have.

Two rules, both orientations:

- `tabs.css:423` (horizontal) — `border-bottom-color`
- `tabs.css:434` (vertical) — `border-right-color`

One hook covers both; name it per the grammar in
`docs/contracts/003-recipe-hooks.md` and consistently with
`--poodle-recipe-tabs-active-outline-border`. Read that contract before
choosing the name — do not invent a spelling.

Same shape as its siblings: `var(--poodle-recipe-…, <current value>)`. The
fallback is the current `var(--poodle-color-accent-base)`, so nothing moves
visually.

### R3 — This is Poodle's half only.

`longhorn-poodle-svelte`'s `LayoutDockRegion` forwarding `tabActiveEdge` is
Longhorn's change, on their side, after this lands. Do not touch any Longhorn
or Loophole file.

### R4 — Both web runtimes. No native work.

Svelte first, React mirrors exactly. `DockRegionSpec` in
`packages/contracts/components/src/dock_region.rs` is out of scope: the natives
are deferred to `g13.014`, and `docs:spec-drift` only checks contracts that
already have a spec — check whether adding a documented prop to `dock-region.md`
trips it, and if it does, **stop and say so** rather than editing the spec.

## Scope

### In scope

- `DockRegion` in both web runtimes: four props, forwarded at three call sites.
- `tabs.css`: the underline hook, both orientations.
- `dock-region.md` and `tabs.md`: the new props, the new hook, and the
  explicit note about what DockRegion deliberately does not forward.
- Recipe inventory, if the repo tracks hooks there — check `drift:recipes`.
- Specimens showing `tabActiveEdge="none"` and a non-reorderable dock.

### Out of scope — stop conditions if reached

- Any Longhorn or Loophole file (R3).
- Native adapters or `dock_region.rs` (R4).
- Changing any default — this card is additive only.
- `packages/core/src/styles/*.css` beyond `tabs.css`.
- Refreshing visual baselines. Defaults are unchanged, so nothing should move;
  if a baseline shifts, **stop** — it means a default was not preserved.

## Required Tests

Both runtimes:

- `tabActiveEdge="none"` renders no active underline; the default still does.
- `tabActiveFill` and `tabBordered` reach Tabs.
- `tabReorderable={false}` produces non-reorderable tabs; the default is still
  reorderable.
- All four apply at every call site — including the collapsed icon-strip and
  the vertical-edge strip, not just the main branch. The dock-tabs papercut
  landed because a branch was missed.
- The underline hook overrides the colour in both orientations, and its absence
  leaves the current colour exactly.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read `docs/contracts/003-recipe-hooks.md` before naming the hook.
- **Run `effigy check:svelte`**, plus `docs:contract-drift`, `drift:recipes`
  and `docs:callback-drift`.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-040-dock-region-tab-passthroughs`. Do not
  merge.

## Writable Paths

- `packages/{svelte,react}/components/src/DockRegion.{svelte,tsx}`
- `packages/{svelte,react}/components/test/DockRegion*.test.*`
- `packages/core/src/styles/tabs.css`
- `packages/{svelte,react}/preview/src/**/DockRegionSpecimen.*`
- `docs/contracts/components/dock-region.md`
- `docs/contracts/components/tabs.md`
- `docs/logs/2026-08/<DD>-g13-040-dock-region-tab-passthroughs.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:components`, `check:svelte`, `docs:lint`,
   `drift:recipes`, `git diff --check`. All green.
2. Read `003-recipe-hooks.md`; choose the hook name.
3. Svelte: the four props, forwarded at all three call sites.
4. `tabs.css`: the hook, both orientations, current value as fallback.
5. Mirror React exactly.
6. Contracts, including the deliberate-non-forwarding note (R1).
7. Specimens.
8. Validate:
   ```sh
   effigy test:components
   effigy test:parity
   effigy check:svelte
   effigy docs:lint
   effigy docs:contract-drift
   effigy docs:callback-drift
   effigy docs:focus-ring-drift
   effigy drift:recipes
   effigy svelte:surface-audit
   effigy ci:web
   git diff --check
   ```

## Acceptance Criteria

- [ ] Four props, forwarded at all three call sites, defaults preserving
  today's rendering.
- [ ] The underline hook works in both orientations and defaults to the
  current colour.
- [ ] The contract records what DockRegion deliberately does not forward, and
  why.
- [ ] All step-8 commands exit 0; no baseline refreshed.

## Stop Conditions

- Adding a documented prop to `dock-region.md` trips `docs:spec-drift` against
  `DockRegionSpec` (R4).
- The hook grammar has no spelling that fits a shared horizontal/vertical
  border colour.
- A default cannot be chosen that preserves current rendering at all three
  call sites.

Stop with exact paths, commands, and the smallest unresolved question.
