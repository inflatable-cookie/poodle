# 11 — g13.016 ActiveFill Shared Type And NavigationMenu Switches (batch log)

Branch: `thread/g13-016-active-fill-navigation-menu` (dedicated worktree, pushed
with `git push -u origin thread/g13-016-active-fill-navigation-menu`)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/016-active-fill-shared-type-and-navigation-menu.md`
Status: **DELIVERED**

`TabActiveFill` is now `ActiveFill`, defined once in
`004-shared-control-types.md` and referenced by both contracts; NavigationMenu
gains `activeOutline` (bool, default `false`) and `activeFill` (`"tint" |
"solid"`, default `"tint"`) in the contract, both web runtimes, and
`NavigationMenuSpec`. The trigger's unconditional border is gone (accepted
visual change per the maintainer ruling); the outline and solid fill are
opt-in, and solid fill survives hover/focus.

## 1. Baseline (step 1)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages installed |
| `effigy test:components` | 0 | 44 files / 855 tests |
| `effigy test:parity` | 0 | 2 files / 163 tests |
| `effigy docs:lint` | 0 | 170 contracts, 12 parity targets, … |
| `cargo test -p poodle-render` (from `packages/render`) | 0 | 152 tests |
| `git diff --check` | 0 | clean tree |

## 2. Type promotion (step 2)

- `docs/contracts/004-shared-control-types.md`: new `ActiveFill` section
  (`type ActiveFill = "tint" \| "solid"`, default `"tint"`, consumers
  `components/tabs.md` + `components/navigation-menu.md`). Semantics note: two
  members only; a third value is a T2 violation.
- Rust: `TabActiveFill` renamed `ActiveFill` in
  `packages/contracts/components/src/tabs.rs` (all in-file uses at
  `tabs.rs:52,113,220,379,387-389`), re-export updated at `lib.rs:280`
  (`pub use tabs::{ActiveFill, …}`), `render/src/tabs.rs:21,207,480,536`
  updated. No `TabActiveFill` remains in any source.
- `packages/contracts/components/src/navigation_menu.rs` imports
  `crate::tabs::ActiveFill` (same crate — no new module).
- TypeScript: `ActiveFill = "tint" | "solid"` added to both
  `packages/svelte/components/src/types.ts` and
  `packages/react/components/src/types.ts`; `Tabs.svelte` / `Tabs.tsx`
  reference it. No inline `"tint" | "solid"` union remains in either runtime
  (grep-verified).
- `tabs.md` §3: `activeFill` type is now `ActiveFill` with a pointer to 004
  instead of a restated union.
- **Tabs rendering unchanged**: existing Tabs tests pass unmodified in both
  runtimes (5 svelte + 1 react); full suites green (see §9).
- Non-listed paths touched (required by the rename — same in-repo precedent
  as g13-013): `packages/gpui/preview/src/specimens/tabs.rs` and
  `packages/jetstream/preview/src/specimens/tabs.rs` use
  `with_active_fill(TabActiveFill::Solid)`; renamed so the native previews
  still compile.

## 3. CSS (step 3) — `packages/core/src/styles/navigation-menu.css`

- Base trigger: unconditional `border: 0.0625rem solid …` removed, replaced
  by `border: 0` (see §7 finding 1 — without the explicit `0`, the UA default
  button border leaks and renders differently across the two preview shells).
- `[data-active-outline="true"]`: `border: 0.0625rem solid transparent` on
  every trigger (reserve — no layout shift); the open trigger's border-color
  is `accent-base` 42% mixed with `border-default` (the former default open
  border value), via
  `--poodle-recipe-navigation-menu-active-outline-border`.
- `[data-active-fill="solid"]`: open trigger fills `accent-base` and switches
  the foreground to `--poodle-color-text-inverse`
  (`--poodle-recipe-navigation-menu-active-solid-fill` / `-solid-text`); the
  fill survives `:hover` and `:focus-visible` via a rule one step more
  specific than the existing hover/focus rule at the old `:39-41`
  (specificity 5 vs 3), matching Tabs' hover-survival pattern.
- Open-trigger rule loses its `border-color` (no border exists without
  `activeOutline`).

## 4. Web components (step 4)

- `packages/svelte/components/src/NavigationMenu.svelte` and
  `packages/react/components/src/NavigationMenu.tsx`: `activeOutline?: boolean`
  (default `false`) and `activeFill?: ActiveFill` (default `"tint"`); root
  emits `data-active-outline` (only when set) and `data-active-fill`
  identically. Defaults and attribute emission at parity.
- `packages/svelte/components/test/interactions.test.ts` and
  `packages/react/components/test/interactions.test.tsx`: two new tests each —
  defaults (`data-active-fill="tint"`, no `data-active-outline`) and opted-in
  emission (`"true"` / `"solid"`).

## 5. Rust (step 5)

- `packages/contracts/components/src/navigation_menu.rs`:
  `NavigationMenuSpec` gains `active_outline: bool` (default `false`) and
  `active_fill: ActiveFill` (default `Tint`) plus
  `with_active_outline` / `with_active_fill` builders; two new tests
  (defaults, builders).
- `packages/render/src/navigation_menu.rs`: per-trigger logic —
  - `solid = is_active && spec.active_fill == ActiveFill::Solid`; open solid
    trigger: background `accent-base`, label + icon foreground
    `color.text.inverse`;
  - `active_outline`: 1px border on every trigger — transparent reserve on
    idle, `mix_srgb(accent, border-default, 0.42)` on the open trigger; no
    border at all otherwise (width 0);
  - hover patch keeps `accent-base` for solid open triggers (no 12% revert),
    `hover_bg` everywhere else (matches the web cascade).
  - Four new node-level tests (defaults borderless/tint; outline reserve +
    open color; solid accent + inverse text; solid hover survival).
- `cargo test -p poodle-render`: 156 passed (was 152).
`cargo test -p poodle-specs`: 241 passed (was 239).

## 6. Specimens (step 6)

All four runtimes cover default (the existing "Horizontal navigation" group),
outline, solid, and solid-hovered. Svelte and React group labels identical:

- "Navigation menu (active outline)" — `activeOutline`, `value="components"`
- "Navigation menu (solid fill)" — `activeFill="solid"`, `value="components"`
- "Navigation menu (solid fill — hover the open trigger)" — same props,
  dashed frame + hover hint (Tabs precedent; neither the web gate nor the
  native previews simulate hover — the render test proves the hover patch).

GPUI: three new groups with `with_active_outline(true)` /
`with_active_fill(ActiveFill::Solid)`; the shared `items` needed
`items.clone()` per group. Jetstream: three new `group(...)` blocks.
- `packages/svelte/preview/src/component-docs.ts`: navigation-menu entry gains
  `activeOutline` and `activeFill` prop rows.

## 7. Findings and deviations (card assumptions vs repository reality)

1. **UA default border leak — the one real defect.** Removing the trigger's
   unconditional `border` declaration without an explicit replacement exposed
   the browser's default `2px outset buttonborder` on the `<button>` triggers.
   The two preview shells resolve that UA default differently (Svelte preview
   dark mode: black; React preview: white), producing a 0.564% Svelte↔React
   pixel diff on the navigation-menu slug only — the gate's one new failure.
   Fixed with explicit `border: 0` on the base trigger (the established
   pattern — `.poodle-tabs__tab` and the Menubar trigger both declare
   `border: 0`). After the fix the navigation-menu pair is clean. Recorded in
   `PAPERCUTS.md` as non-duplicate friction (border-removal on button styles
   must declare `border: 0`).
2. **"Visual baseline" is Svelte↔React parity here.** The web gate
   (`test/visual/run.ts`) diffs the two previews at the same slug/axis; it has
   no committed baseline files for the web. The card's "expected NavigationMenu
   diffs" therefore materialised as the single UA-border parity diff above,
   which was a real defect and was fixed rather than accepted. Native baseline
   dirs are gitignored/machine-local (as in g13-013 finding 4) — no native
   gate signal; the Rust rendering rests on the node-level tests.
3. **Jetstream preview build blocked (pre-existing).** `cargo check -p
   poodle-jetstream-preview` fails resolving `poodle-node` through
   `jetstream-poodle` (missing `poodle-wt/poodle/packages/contracts/node`
   path) — identical to g13-013 finding 5. GPUI preview checks clean; the
   Jetstream specimen edits are mechanical.
4. **Non-listed paths touched (all inside this repo, all required).**
   `packages/{gpui,jetstream}/preview/src/specimens/tabs.rs` — the renamed
   enum's remaining consumers (native Tabs specimens); without the rename the
   native previews cannot compile. Nothing staged outside the explicit-path
   set beyond these two files, `PAPERCUTS.md`, and the batch log.
5. **`docs:check` regenerates out-of-set artifacts — restored, not staged.**
   `packages/tokens/artifacts/rust/*` (per the card's worker rules) and
   `packages/react/preview/artifacts/component-docs.json` (which also carries
   a stopped card's uncommitted `initialFocus` drift, as in g13-013). The
   writable source (`component-docs.ts`) carries the new props; the artifact
   refreshes on the next `docs:check`.

## 8. Visual enumeration (step 8) — web gate

`effigy visual:report` (`bun test/visual/run.ts --tier=sweep --report`), run
after the `border: 0` fix: **308 pairs compared, 54 failing — zero on
navigation-menu** (both axes, eclipse-compact-md + iceberg-compact-md).

| Slug | Axis | Kind | Ratio / Note | Classification |
|---|---|---|---|---|
| agent-message | both | size | svelte 992x1650 vs react 992x160 | pre-existing (g13-013 set) |
| agent-question | both | size | 992x2054 vs 992x2347 | pre-existing |
| agent-question-record | both | size | 992x979 vs 992x1078 | pre-existing |
| agent-subagent | both | size | 992x901 vs 992x1000 | pre-existing |
| agent-transcript | both | size | 992x2464 vs 992x1627 | pre-existing |
| audio-meter | both | size | 992x1828 vs 992x2293 | pre-existing |
| audio-switch | both | size | 992x507 vs 992x740 | pre-existing |
| changed-files | both | size | 992x1295 vs 992x160 | pre-existing |
| drag-number-field | both | size | 992x585 vs 992x831 | pre-existing |
| envelope-editor | both | size | 992x1604 vs 992x2295 | pre-existing |
| fader | both | size | 992x1761 vs 992x2226 | pre-existing |
| gain-reduction-meter | both | size | 992x1576 vs 992x1977 | pre-existing |
| keyboard | both | size | 992x1291 vs 992x1960 | pre-existing |
| knob | both | size | 992x1063 vs 992x1272 | pre-existing |
| mod-matrix-grid | both | size | 992x760 vs 992x1067 | pre-existing |
| remediation-banner | both | size | 992x299 vs 992x315 | pre-existing |
| tool-call | both | size | 992x797 vs 992x160 | pre-existing |
| tool-call-group | both | size | 992x677 vs 992x160 | pre-existing |
| tree | both | size | 992x2899 vs 992x2521 | pre-existing |
| validation-summary | both | size | 992x332 vs 992x348 | pre-existing |
| value-readout | both | size | 992x648 vs 992x906 | pre-existing |
| waveform-display | both | size | 992x1051 vs 992x1736 | pre-existing |
| xy-pad | both | size | 992x1761 vs 992x2242 | pre-existing |
| agent-plan / agent-plan-record | both | capture | capture error, no pair | pre-existing |
| dock-region | eclipse / iceberg | pixels | 0.443% / 0.436% | pre-existing |
| embed-preview | eclipse / iceberg | pixels | 14.569% / 19.912% | pre-existing |
| navigation-menu | — | — | **no diff** | expected-change pair; clean after `border: 0` fix |

Breakdown: 46 size / 4 capture / 4 pixels — byte-for-byte the same slug set as
the g13-013 baseline (54 failing), so nothing outside navigation-menu changed.
The navigation-menu pair failed once (0.564% pixels, eclipse) mid-branch due
to finding 1 and is clean in the final run. **No baseline file was modified**
(the gate's `test/visual/out/` is gitignored; the web gate keeps no committed
baselines; native baseline dirs untouched).

## 9. Validation (step 9)

| Command | Exit | Notes |
|---|---|---|
| `effigy test:components` | 0 | 44 files / 859 tests (855 + 4 new) |
| `effigy test:parity` | 0 | 2 files / 163 tests |
| `effigy docs:lint` | 0 | (also runs inside docs:check) |
| `effigy docs:contract-drift` | 0 | every documented public prop implemented |
| `effigy docs:spec-drift` | 0 | every documented prop reaches poodle-specs |
| `effigy docs:value-domain-drift` | 0 | no new finding for either component (the tabs `variant` strip gap is the pre-existing g13-013 record) |
| `cargo test -p poodle-render` | 0 | 156 tests (152 + 4 new) |
| `cargo test -p poodle-specs` | 0 | 241 tests (239 + 2 new) |
| `cargo check -p poodle-gpui-preview` | 0 | clean (specimen + spec changes) |
| `effigy docs:check` | 0 | incl. `vite build` |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | tokens artifacts rewritten by docs:check → restored, not committed |
| `git diff --check` | 0 | clean |

## 10. Acceptance criteria

- [x] `ActiveFill` defined once in `004`; both contracts reference it; no
  inline `"tint" | "solid"` union remains in either web runtime.
- [x] `TabActiveFill` gone; all four Rust reference sites updated
  (`tabs.rs:21`, `lib.rs:280`, `render/src/tabs.rs:21,207,480`); Tabs
  rendering unchanged and its tests pass unmodified.
- [x] NavigationMenu has `activeOutline` (default `false`) and `activeFill`
  (default `"tint"`) in contract, both web runtimes, and
  `NavigationMenuSpec`.
- [x] The unconditional trigger border is gone; the outline appears only when
  `activeOutline` is set, with a transparent reserve border keeping layout
  stable between states.
- [x] Solid fill survives hover and focus-visible (CSS survival rule + render
  hover patch, both test-proven).
- [x] Specimens in all four runtimes cover default, outline, solid, and solid
  hovered; Svelte and React labels identical.
- [x] The contract records the changed default appearance under the "Default
  Appearance Change (g13.016)" heading.
- [x] Visual diffs enumerated and classified; no baseline file modified.
- [x] All step-9 commands exit 0.
- [x] Batch log records commands, exit states, and the diff table.

## 11. Stop conditions

None triggered. The one navigation-menu gate diff was a real parity defect
introduced mid-branch (UA button-border leak) and was fixed, leaving the pair
clean; nothing outside navigation-menu changed (final failure set is the
g13-013 baseline exactly). `TabActiveFill` rename left Tabs rendering
untouched (tests pass). `docs:value-domain-drift` reports no new finding for
either component. The border removal causes no layout shift (transparent
reserve border under `activeOutline`; `border: 0` otherwise).
