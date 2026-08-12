# 040 DockRegion — Tab Pass-throughs, And The Underline's Missing Hook

Status: merged (`51b0e76c`)
Milestone: side-quest (component surface, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-040-dock-region-tab-passthroughs`
Depends on: none. (The feared `tabs.css` contention with `g13-b038` did not materialise: the focus gate reports zero findings there, because Tabs draws its ring as an outline on the focused element itself and is correctly exempt.)
Governing refs: `docs/contracts/components/dock-region.md`,
`docs/contracts/components/tabs.md`,
`docs/architecture/007-appearance-recipe-contract.md` (the hook grammar — the
originally cited `docs/contracts/003-recipe-hooks.md` does not exist)

## Goal

Loophole cannot turn DockRegion's active-tab underline off, so it carries a
scoped CSS override on `.poodle-tabs__item[data-selected]` — exactly the
class-targeting tier the recipe contract exists to eliminate. Two Poodle
changes retire it.

The report asked for one prop. The maintainer's instruction is broader: **all
tab variants and style props must be reachable through DockRegion.** R1 answers
that with five pass-throughs rather than adding `tabActiveEdge` and waiting for
the next report — and R1a records why the overflow props are not among them.

## Current State — Measured

`Tabs` declares **29 props**. DockRegion instantiates it at three call sites
and reaches only four of them from outside:

| Group | Props | Today |
|---|---|---|
| forwarded | `variant` (as `tabVariant`), `size`, `sizeRole`, `density` | consumer can set — 4 |
| hard-coded | `activeEdge="underline"`, `reorderable={true}` | unreachable — 2 |
| never passed | `activeFill`, `bordered`, `activationMode`, `fullWidth`, `collapseWhenOverflow`, `overflowStrategy`, `shed`, `collapseLabel`, `historyKey`, `actions` | unreachable, Tabs' defaults apply — 10 |
| derived | `showTooltips` (from `isCompact`) | unreachable, and the derivation is load-bearing — 1 |
| DockRegion's own | `items`, `value`, `defaultValue`, `orientation`, `ariaLabel`, `onValueChange`, `onClose`, `onReorder`, `onDragPrepare`, `onDragStart`, `onDragEnd`, `children` | correctly not a consumer's — 11 |

Nine of those are variants, style or hard-coded behaviour a consumer could
legitimately want; five are in scope here (R1) and four are ruled out (R1a).

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

### R1 — Forward five props. Not the whole surface.

`Tabs` declares **29** props. Four are already forwarded (`variant` as
`tabVariant`, `size`, `sizeRole`, `density`). **Five more become
pass-throughs**, on the `tab` prefix `tabVariant` established:

| New prop | Tabs prop | Default | Why |
|---|---|---|---|
| `tabActiveEdge` | `activeEdge` | `"underline"` | the reported hard-code |
| `tabActiveFill` | `activeFill` | `"tint"` | activeEdge's sibling; exposing one without the other is the same report next week |
| `tabBordered` | `bordered` | `false` | style |
| `tabFullWidth` | `fullWidth` | `false` | style |
| `tabReorderable` | `reorderable` | `true` | the other hard-code, and a functional gap: a dock whose panels must not reorder cannot say so |

Every default is Tabs' current effective value, so nothing moves. Additive
only. Forward all five at **all three** call sites.

**Everything else stays unexposed, deliberately.** The maintainer's ruling:
variants and style props, plus the hard-coded behaviour. Not the rest.

- `activationMode`, `historyKey`, `actions` — behaviour and slots, no
  requested use, and each is a surface to support forever once added.
- `showTooltips` — DockRegion derives it from `isCompact`, and that derivation
  is load-bearing: icon-only tabs are unreadable without tooltips. Leave the
  derivation alone.
- **The overflow trio** — see R1a.
- The twelve DockRegion structurally owns: `items`, `value`, `defaultValue`,
  `orientation` (this *is* the dock layout), `ariaLabel` (already overridable
  through DockRegion's own prop), the six drag/change/close handlers it
  intercepts and re-emits, and `children` (Tabs' panel snippet, where
  DockRegion renders the panel body itself).

Record the non-forwarded list in the contract with its reason, so the next
report can tell a deliberate line from an oversight.

### R1a — The dock's overflow behaviour is already on. Do not add a second one.

Ruled after measuring, because the obvious reading is wrong.

`collapseWhenOverflow` defaults to `false` and DockRegion never passes it, so
**Tabs' own overflow handling is off in every dock today** — and that is
correct, because Tabs' version collapses the entire strip into a single
dropdown (`collapsedMenuItems`, trigger labelled with the selected panel).

DockRegion already handles overflow its own way: the `isCompact` measurement
that hides labels and goes icon-only. That is the dock's overflow story, it is
enabled, and it is the logic the icon-only compaction fix repaired.

So: **do not expose the trio, and do not turn `collapseWhenOverflow` on.**
Enabling it would put two different overflow mechanisms on the same strip —
DockRegion compacting to icons while Tabs tries to collapse to a menu. Leave
`collapseWhenOverflow`, `overflowStrategy`, `shed` and `collapseLabel` exactly
as they are.

State this in the contract too. It reads like an omission and is not one.

### R2 — Give underline the hook its siblings already have.

Two rules, both orientations:

- `tabs.css:423` (horizontal) — `border-bottom-color`
- `tabs.css:434` (vertical) — `border-right-color`

One hook covers both; name it per the grammar in
`docs/architecture/007-appearance-recipe-contract.md` and consistently with
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

### R4a — Ruling: join the existing spec-surface tranche. Do not table them.

Raised as a stop, correctly — R4 named this exact condition. Neither option in
the question is the right one, because the repo already has a pattern for it
and `tabVariant` is in it.

`tabVariant` and `showTabs` are implemented in Svelte today and are **not in
`dock-region.md`'s §Public Props table**. That is why they do not trip
`docs:spec-drift`. They sit in `contract-prop-drift.ts`'s `BASELINE` as
`svelteOnly`, with the reason recorded there:

> web-only or spec-surface-pending props the contract deliberately does not
> table. Tabling them would fail contract-spec-drift until the poodle-specs
> structs carry the fields.

**Do the same for the five.** Extend the existing `"dock-region"` baseline
entry to `["showTabs", "tabVariant", "tabActiveEdge", "tabActiveFill",
"tabBordered", "tabFullWidth", "tabReorderable"]`, and add a comment line
naming this card and the reason, matching the entries around it.

**Do not touch `contract-spec-drift.ts`'s `OPEN_GAPS`.** It is empty by intent
and its own comment says why: *"adding one means a prop shipped to the web
without reaching the shared spec surface, which is the thing this gate exists
to stop."* These props have not escaped that gate — they are being kept out of
the table precisely so the gate keeps meaning what it says.

**Contract-first still holds.** Document all five in `dock-region.md` **prose**
— a section on the tab pass-throughs, alongside the R1a note about what is
deliberately not forwarded — just not in the §Public Props table. That is the
same treatment `tabVariant` has, so this adds no new mechanism and no new
exception, only five names to a list that already exists for this reason.

When `g13.014` gives `DockRegionSpec` its tab fields, the whole entry moves
into the table together and the baseline line is deleted. Say that in the
prose so the next reader knows it is a tranche, not a permanent carve-out.

## Scope

### In scope

- `DockRegion` in both web runtimes: five new pass-throughs, each forwarded at
  all three call sites.
- `tabs.css`: the underline hook, both orientations.
- `dock-region.md` and `tabs.md`: the new props, the new hook, and the
  explicit note about what DockRegion deliberately does not forward.
- Recipe inventory, if the repo tracks hooks there — check `drift:recipes`.
- Specimens showing `tabActiveEdge="none"`, a non-reorderable dock, and at
  least one variant/fill combination that was previously unreachable.

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
- Every one of the five reaches Tabs — assert the forwarded value lands, not
  merely that the prop exists.
- `tabReorderable={false}` produces non-reorderable tabs; the default is still
  reorderable.
- DockRegion's own `isCompact` compaction still behaves — none of the five
  disturbs it, and `collapseWhenOverflow` is still off (R1a).
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
- `packages/svelte/preview/scripts/contract-prop-drift.ts` (R4a — the
  `dock-region` baseline entry only)
- `docs/logs/2026-08/<DD>-g13-040-dock-region-tab-passthroughs.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy test:components`, `check:svelte`, `docs:lint`,
   `drift:recipes`, `git diff --check`. All green.
2. Read `003-recipe-hooks.md`; choose the hook name.
3. Svelte: the five pass-throughs, forwarded at all three call sites.
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
