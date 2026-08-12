# 12 — g13.040 DockRegion Tab Pass-throughs, And The Underline's Missing Hook (batch log)

Branch: `thread/g13-040-dock-region-tab-passthroughs` (pushed with
`git push -u origin thread/g13-040-dock-region-tab-passthroughs`)
Date: 2026-08-12
Card: `docs/roadmaps/g13/batch-cards/040-dock-region-tab-passthroughs.md`
Status: **DELIVERED**

DockRegion now forwards five Tabs props (`tabActiveEdge`, `tabActiveFill`,
`tabBordered`, `tabFullWidth`, `tabReorderable`) at all three call sites in
both web runtimes, and the underline active-edge gets the recipe hook its
siblings already have (`--poodle-recipe-tabs-active-underline-border`, both
orientations, current value as fallback). Per ruling R4a, the five join the
existing `dock-region` spec-surface tranche in `contract-prop-drift.ts`'s
`BASELINE` as `svelteOnly` (documented in contract prose, not the Public Props
table), and `contract-spec-drift.ts`'s `OPEN_GAPS` stays empty.

## 1. Baseline (step 1)

| Command | Exit | Notes |
|---|---|---|
| `effigy test:components` | 0 | 71 files / 1036 tests |
| `effigy check:svelte` | 0 | 0 errors |
| `effigy docs:lint` | 0 | 172 contracts, 44 operator guides, … |
| `effigy drift:recipes` | 0 | retired Treatment drift: 0 references |
| `git diff --check` | 0 | clean tree |

## 2. Hook name (step 2)

The card's governing ref `docs/contracts/003-recipe-hooks.md` does not exist —
the hook grammar lives in `docs/architecture/007-appearance-recipe-contract.md`
(`--poodle-recipe-<component>[-<variant>]-<slot>[-<state>]`). The underline
spelling follows the sibling exactly: `--poodle-recipe-tabs-active-underline-border`
(slot `underline`, state `border`), one variable for both orientations because
they share the property family (the inline-end border colour). No stop
condition: the grammar has a fitting spelling.

## 3. Svelte (step 3) — `packages/svelte/components/src/DockRegion.svelte`

- Props interface + destructure gain the five, on the `tab` prefix
  `tabVariant` established: `tabActiveEdge = "underline"`,
  `tabActiveFill = "tint"`, `tabBordered = false`, `tabFullWidth = false`,
  `tabReorderable = true` — every default is the value the dock passed
  before, so nothing moves.
- All three call sites (expanded horizontal strip, collapsed horizontal
  icon-strip, collapsed vertical icon-strip) forward all five:
  `activeEdge="underline"` → `activeEdge={tabActiveEdge}`,
  `reorderable={true}` → `reorderable={tabReorderable}`, plus the new
  `activeFill` / `bordered` / `fullWidth`.
- `ActiveEdge` / `ActiveFill` types imported from `./types`.

## 4. CSS (step 4) — `packages/core/src/styles/tabs.css`

- Horizontal selected item (`:423`):
  `border-bottom-color: var(--poodle-recipe-tabs-active-underline-border, var(--poodle-color-accent-base))`.
- Vertical selected item (`:434`):
  `border-right-color: var(--poodle-recipe-tabs-active-underline-border, var(--poodle-color-accent-base))`.
- Fallback is exactly the pre-hook value, so an unset hook renders
  byte-identically. Same shape as `-active-outline-border` / `-solid-fill` /
  `-solid-text`. No other stylesheet touched (the `tabs.css` contention with
  g13-038 did not materialise; `docs:focus-ring-drift` still green).

## 5. React (step 5) — `packages/react/components/src/DockRegion.tsx`

- `DockRegionProps` + destructure gain the same five with identical defaults.
- The single `stripTabs(orientation, withTooltips)` helper (used by all three
  call sites) forwards all five.

## 6. Contracts (step 6)

- `docs/contracts/components/dock-region.md` — new prose section
  "§3 Tab Pass-throughs": the five props with forwarded-Tabs target and
  default, the deliberately-not-forwarded list with reasons (R1a: the
  overflow trio would put two overflow mechanisms on one strip;
  `activationMode`/`historyKey`/`actions`; the load-bearing `showTooltips`
  derivation), and the tranche note: when g13.014 gives `DockRegionSpec` its
  tab fields, the whole entry moves into the §Public Props table and the
  `BASELINE` line is deleted. NOT tabled — the table stays the shared spec
  surface, so `docs:spec-drift` stays green.
- `docs/contracts/components/tabs.md` — new "§8 Recipe hooks — the active
  axis" subsection documenting all four active-axis hooks (outline border,
  underline border, solid fill, solid text) with their fallbacks; the new
  underline hook is the addition, listed with its siblings for context.
- `packages/svelte/preview/scripts/contract-prop-drift.ts` (R4a) — the
  existing `dock-region` BASELINE entry extended to
  `["showTabs", "tabVariant", "tabActiveEdge", "tabActiveFill",
  "tabBordered", "tabFullWidth", "tabReorderable"]` with a comment naming
  this card and the g13.014 tranche. `contract-spec-drift.ts`'s `OPEN_GAPS`
  untouched — still empty by intent.

## 7. Specimens (step 7)

- `packages/svelte/preview/src/specimens/DockRegionSpecimen.svelte` and
  `packages/react/preview/src/gallery/specimens/DockRegionSpecimen.tsx` —
  new "Tab pass-throughs — no underline, no reorder, solid fill" group with
  three docks: `tabActiveEdge="none"`, `tabReorderable={false}`, and
  `tabVariant="pill"` + `tabActiveFill="solid"` (previously unreachable).

## 8. Tests

- `packages/svelte/components/test/DockRegionTabPassThroughs.svelte.test.ts`
  (9 tests) and
  `packages/react/components/test/DockRegionTabPassThroughs.test.tsx`
  (6 tests, mirror): defaults preserve rendering (underline edge, tint fill,
  no border, no full-width, reorderable), all five land on Tabs, edge `"none"`
  vs default, `tabReorderable={false}` vs default, every call site including
  the collapsed vertical and horizontal icon-strips, and R1a (no
  `collapseWhenOverflow`: no measure shell, real tablist renders, labels
  intact).
- The underline-hook contract test reads `tabs.css` and asserts the
  declaration in both orientations with the accent fallback. Computed-style
  assertion is not possible in happy-dom — it cannot resolve a `var()` whose
  fallback is another `var()` (probed empirically), so the declaration itself
  is the assertable contract.

## 9. Validation (step 8)

| Command | Exit | Notes |
|---|---|---|
| `effigy test:components` | 0 | 73 files / 1051 tests (1036 + 15 new) |
| `effigy test:parity` | 0 | 165 tests |
| `effigy check:svelte` | 0 | 0 errors |
| `effigy docs:lint` | 0 | (also ran inside ci:web) |
| `effigy docs:contract-drift` | 0 | 131 checked — baseline extension absorbed the five |
| `effigy docs:callback-drift` | 0 | 102 checked |
| `effigy docs:focus-ring-drift` | 0 | 90 rules, 5 baselined squares |
| `effigy docs:spec-drift` | 0 | 113 checked — nothing tabled, no trip (R4/R4a) |
| `effigy drift:recipes` | 0 | retired Treatment drift: 0 references |
| `effigy svelte:surface-audit` | 0 | 177 files, 0 legacy markers |
| `effigy ci:web` | 0 | icons, tokens, bun test 506, components 1051, surface audit, docs lint, reports |
| `git diff --check` | 0 | clean |

The recipe inventory artifact (`recipe-inventory.json`) is a committed
generated file that no gate compares (it is stale for the pre-existing
`-active-outline-border` hook already) and is outside the card's writable
paths — left untouched.

Svelte preview type-check (`bunx svelte-check --workspace
packages/svelte/preview`) reports 86 pre-existing errors in other specimens
(the documented PAPERCUTS entry — previews are ungated); the
`DockRegionSpecimen.svelte` change adds none.

## 10. Acceptance criteria

- [x] Five props, forwarded at all three call sites, defaults preserving
  today's rendering (tested: data attributes + draggable state + R1a strip
  integrity).
- [x] The underline hook works in both orientations and defaults to the
  current colour (declaration asserted in both orientations; fallback is the
  pre-hook value).
- [x] The contract records what DockRegion deliberately does not forward, and
  why (prose §3, including the R1a overflow ruling).
- [x] All step-8 commands exit 0; no baseline refreshed.

## 11. Stop conditions

None triggered. R4's spec-drift stop condition was resolved by ruling R4a —
the five are documented in prose and baselined `svelteOnly`, never tabled, so
`docs:spec-drift` stays green and `OPEN_GAPS` remains empty. The hook grammar
had a fitting spelling. Defaults preserve rendering at all three call sites.
No Longhorn/Loophole, native adapter, or `dock_region.rs` file touched; no
default changed.
