# 11 — g13.013 Tabs Variant Consolidation (batch log)

Branch: `thread/g13-013-tabs-variant-consolidation` (dedicated worktree, pushed
with `git push -u origin thread/g13-013-tabs-variant-consolidation`)
Date: 2026-08-11
Card: `docs/roadmaps/g13/batch-cards/013-tabs-variant-consolidation.md`
Status: **DELIVERED**

Variant set is now `card | pill | block | strip`; the old `text` variant is
renamed `card`, the old `card` variant is deleted, `text`/`underline` are gone
with no aliases, and `activeOutline` (bool, default false) + `activeFill`
("tint" | "solid", default "tint") are new props in the contract, both web
runtimes, and `TabsSpec`.

## 1. Baseline (step 1)

| Command | Exit | Notes |
|---|---|---|
| `bun install` | 0 | 234 packages installed |
| `effigy test:components` | 0 | 42 files / 834 tests |
| `effigy test:parity` | 0 | 2 files / 157 tests |
| `effigy docs:lint` | 0 | 170 contracts, 12 parity targets, … |
| `cargo test -p poodle-render` (from `packages/render`) | 0 | 148 tests |
| `git diff --check` | 0 | clean tree |

## 2. Contract (step 2) — `docs/contracts/components/tabs.md`

- §1 + §3: union is now `"card" \| "pill" \| "block" \| "strip"`, default
  `"card"`, no underline alias.
- §3: added `activeOutline` (bool, default `false` — "opt-in outline on the
  active tab, the decoration the former card variant had by default") and
  `activeFill` (`"tint" \| "solid"`, default `"tint"` — solid fills with
  `accent-base` and swaps foreground to `text-inverse`).
- `bordered` note now names the card variant.
- §7/§8: text→card rename across layout and token tables; old card item
  tables replaced by `activeOutline` tables (transparent 1px border on every
  item + accent 32% selected border) and an `activeFill="solid"` table.
- §3 attribute table: root gains `data-active-outline` / `data-active-fill`.
- §9 Svelte note: `resolvedVariant`/underline normalization removed.
- §10 GPUI notes: line-765 naming note deleted; notes now describe the
  `Card | Pill | Block` enum, accessory support, `activeOutline` mapping
  (`mix_srgb(accent, border-subtle, 0.32)`), solid mapping, and the deferred
  `Strip` gap.
- §11: "all five variants" → "all four"; percentages updated.
- §13: specimens renamed; new "Card variant (active outline)" and "Card
  variant (solid fill)" definitions.

## 3. CSS (step 3) — `packages/core/src/styles/tabs.css`

- Renamed every `[data-variant="text"]` rule to `card` (list border, vertical
  list, tab radius, selected tab fill, panel override). Recipe hooks renamed
  `--poodle-recipe-tabs-text-*` → `--poodle-recipe-tabs-card-*` (old card
  hooks deleted with the old card).
- Removed `card` from the nowrap/overflow group (now Pill + Strip + Block).
- Deleted the old card item rules (border 68%, bg 92%, selected accent
  32%/14%) and the old card tab rule.
- Added `[data-active-outline="true"]` (transparent 1px border on every item;
  selected item border = accent 32% + border-subtle — the old card
  selected-border value) and `[data-active-fill="solid"]` (selected tab fills
  `accent-base`, foreground `--poodle-color-text-inverse`, with
  `--poodle-recipe-tabs-active-outline-border` / `-solid-fill` / `-solid-text`
  recipe hooks). Both blocks are placed after the variant rules so they win
  the equal-specificity cascade against strip/block item borders.

Solid-fill contrast uses `--poodle-color-text-inverse` — the same token the
primary Button uses on `accent-base` (§4 of the card's stop conditions:
contrast IS reachable with existing tokens, via the established pattern).

## 4. Web components (step 4)

- `packages/svelte/components/src/{Tabs.svelte,types.ts}` and
  `packages/react/components/src/{Tabs.tsx,types.ts}`:
  - `TabVariant = "card" | "pill" | "block" | "strip"` in both type files.
  - Both props added with identical defaults (`false` / `"tint"`); default
    variant `"card"`; `resolvedVariant` (the underline→text normalization)
    deleted in both runtimes.
  - Root emits `data-variant`, `data-active-outline`, `data-active-fill`
    identically.
- `packages/svelte/components/test/interactions.test.ts`: two new tests
  asserting the defaults (`data-variant="card"`, `data-active-fill="tint"`,
  no `data-active-outline`) and the opted-in emission
  (`data-active-outline="true"`, `data-active-fill="solid"`).

## 5. Rust (step 5)

- `packages/contracts/components/src/types.rs`: `TabVariant` is now
  `Card | Pill | Block` (old `Underline` renamed to `Card`, old `Card`
  deleted).
- `packages/contracts/components/src/tabs.rs`: new `TabActiveFill { Tint,
  Solid }` enum; `TabsSpec` gains `active_outline: bool` (default false) and
  `active_fill: TabActiveFill` (default Tint) plus `with_active_outline` /
  `with_active_fill` builders; default variant `TabVariant::Card`. Three new
  tests (defaults off/tint, builders set both).
- `packages/contracts/components/src/lib.rs`: exports `TabActiveFill`
  (required so the spec surface is reachable).
- `packages/render/src/tabs.rs`: old `render_card` deleted; old
  `render_underline` renamed to `render_card`, now takes `on_close` and:
  - renders icon + count (already present via `build_tab_label` — see §7
    finding 1) and the close button wired to `on_close` (inert when unwired,
    ported from the deleted renderer),
  - `active_outline`: 1px border on every tab, accent-32% on the selected,
    transparent elsewhere,
  - `active_fill == Solid`: selected tab background = `accent-base`,
    foreground = `color.text.inverse`.
  - 4 new tests: close wired to `on_close` fires with the tab value; icon +
    count render; solid fill uses accent + inverse; outline borders only the
    selected tab (and none by default).

`cargo test -p poodle-render`: 152 passed (was 148).
`cargo test -p poodle-specs`: 239 passed.
`cargo check -p poodle-gpui-preview`: clean.

## 6. Specimens (step 6)

All four runtimes rename the text specimens to card, replace the old card
specimens, and add outline + solid coverage:

- Svelte / React `TabsSpecimen.*`: groups renamed ("Card variant (default,
  with indicator line)", "Card variant (no border)", "Card variant (with
  icons, no panel)"); "Card variant (active outline)" and "Card variant
  (solid fill)" added.
- GPUI `specimens/tabs.rs`: underline specimen → card default (with panel);
  old card closable specimen stays on card and now wires `on_close` (close
  display: "Last closed: …", mirroring the strip specimen); counts stay;
  outline + solid specimens added; icon specimen → card, no border; drag pair
  is now card + card-with-outline. Needed `on_close` on the
  `Tabs` compat bridge in `node_compat.rs` (was a field without a builder).
- Jetstream `specimens/tabs.rs` + `specimens/mod.rs`: default/underline
  groups → card; the old explicit card pair replaced by outline + solid
  groups; labels renamed; `TabVariant::Underline` → `Card` in the specimen
  view switcher.
- `packages/svelte/preview/src/component-docs.ts`: tabs entry — variant
  default `'"card"'`, both new props documented.

## 7. Findings and deviations (card assumptions vs repository reality)

1. **Known State mismatch (accessories).** The card's Known State says
   `render_underline` renders "no accessories at all — no icon, no count, no
   close". Repository reality: `build_tab_label` (called by every renderer,
   `packages/render/src/tabs.rs`) already renders icon and count; only the
   close button was missing. End state is unaffected (the renamed renderer
   renders all three), but the Known State claim is stale. No workaround was
   needed; recorded per "repository reality is authoritative".
2. **Strip gap recorded, not built.** No `TabVariant::Strip` member; the
   native targets render the strip through the separate
   `TabStripSpec`/`TabStrip` component. Noted in the render module docs and
   the contract's GPUI notes.
3. **Non-listed paths touched (all inside this repo, all required).** The
   deleted variant had in-repo consumers outside the writable list, and one
   new export was required:
   - `packages/svelte/preview/src/components/SpecimenLayout.svelte` and
     `packages/react/preview/src/gallery/SpecimenLayout.tsx` passed
     `variant="text"` — removed variant; changed to `"card"`.
   - `packages/gpui/preview/src/node_compat.rs` — `on_close` builder on the
     `Tabs` compat struct (field existed, no builder).
   - `packages/contracts/components/src/lib.rs` — `TabActiveFill` export.
   - `packages/jetstream/preview/src/specimens/mod.rs` — `TabVariant::Underline`
     in the specimen-page switcher → `Card`.
   - `packages/react/preview/artifacts/component-docs.json` — regenerated by
     `docs:check` (`react:docs`) from the writable `component-docs.ts`, then
     **restored** with `git checkout`: the regeneration also carries a
     stopped card's uncommitted `initialFocus` drift (g13.009 dialog
     initial-focus), and the artifact is outside the writable set, so it is
     not staged. The source (`component-docs.ts`) carries the new tabs
     props; the artifact is refreshed on the next `docs:check` run.
   None of these are consumer repos, roadmap/card status files, or
   `docs/roadmaps/dispatch.md`.
4. **Native visual gate not runnable as a diff on this machine.** Both
   baseline dirs are empty (`packages/gpui/preview/baselines/`,
   `packages/jetstream/preview/baselines/` — gitignored, machine-local).
   Running the gate without `--update` writes "first time" baselines for
   every slug, i.e. refreshes baselines, which the card forbids. No
   before/after signal exists to compare against, so the native enumeration
   is: not run. Verification of the Rust rendering instead rests on the four
   new node-level render tests (§5) and the shared-renderer test suite.
5. **Jetstream preview build blocked (pre-existing).** `cargo check
   --manifest-path packages/jetstream/preview/Cargo.toml` fails resolving
   `poodle-node` through the sibling `jetstream-poodle` crate, whose manifest
   points at `poodle-wt/poodle/packages/contracts/node` (missing). Unrelated
   to this card; the jetstream adapter crate (`poodle-jetstream`) checks
   clean and the specimen edits are mechanical.
6. **Environment cleanup.** Two stale Vite dev servers from the
   `g13-008-split-button-tone` worktree were squatting ports 4174/4180
   (react preview serving 404 from a different repo root), blocking the
   visual gate's strict-port spawn. Killed both.

## 8. Visual enumeration (step 7) — web gate

`effigy visual:report` (`bun test/visual/run.ts --tier=sweep --report`):
**308 pairs compared, 54 failing** — **zero on tabs** (both axes,
eclipse-compact-md + iceberg-compact-md).

| Breakdown | Count | Slugs |
|---|---|---|
| size | 46 | agent-message (iceberg), agent-question, agent-question-record, agent-subagent, agent-transcript, audio-meter, audio-switch, changed-files, drag-number-field, envelope-editor, fader, gain-reduction-meter, keyboard, knob, mod-matrix-grid, remediation-banner, tool-call, tool-call-group, tree, validation-summary, value-readout, waveform-display, xy-pad |
| pixels | 4 | dock-region (0.443% / 0.436%), embed-preview (14.569% / 19.912%) |
| capture | 4 | agent-plan, agent-plan-record (iceberg) |

Classification: **all 54 pre-existing** — the slug set matches the
parity-debt inventory in `PAPERCUTS.md` (2026-08-11 entry: "53 failing
Svelte↔React pairs at HEAD (308 compared; 46 size / 4 capture / 3 pixels)…
tree, xy-pad, tool-call, fader, agent-plan"), and the four pixel failures
were re-run at HEAD in a detached worktree (`git worktree add` at `892c2e51`,
`bun install`, gate with `--slug=dock-region,embed-preview`): identical
ratios (0.443% / 0.436% / 14.569% / 20.383%). No tabs pair appears in the
failure set, so the Svelte/React tabs specimens and components are at parity
after the change. Nothing was refreshed; the gate's diff artifacts live in
`test/visual/out/` (gitignored).

Native gates: see finding 4 (no baselines on this machine; running would
create them). Jetstream gate: see finding 5 (preview can't build here).

## 9. Validation (step 8)

| Command | Exit | Notes |
|---|---|---|
| `effigy test:components` | 0 | 42 files / 836 tests (834 + 2 new) |
| `effigy test:parity` | 0 | 2 files / 157 tests |
| `effigy docs:lint` | 0 | (also ran inside docs:check) |
| `effigy docs:contract-drift` | 0 | 128 checked — every documented public prop implemented |
| `effigy docs:spec-drift` | 0 | 112 checked — every documented prop reaches poodle-specs |
| `cargo test -p poodle-render` | 0 | 152 tests (148 + 4 new) |
| `cargo test -p poodle-specs` | 0 | 239 tests (+3 new) |
| `cargo check -p poodle-gpui-preview` | 0 | clean |
| `effigy docs:check` | 0 | incl. `vite build` |
| `git checkout -- packages/tokens/artifacts/rust/` | 0 | tokens artifacts rewritten by docs:check → restored, not committed |
| `git diff --check` | 0 | clean |

## 10. Acceptance criteria

- [x] Union exactly `card | pill | block | strip` in contract, both web
  runtimes, Rust (`Card | Pill | Block`; strip on natives is
  `TabStripSpec`); `text`, `underline`, old card gone everywhere.
- [x] `activeOutline` / `activeFill` in contract, both web runtimes, and
  `TabsSpec`, with the ruled defaults.
- [x] `activeFill="solid"` renders a fully accent-filled active tab with
  legible foreground (`text-inverse`, the primary-Button token).
- [x] Renamed Rust renderer renders icon, count, close wired to `on_close`
  (test-proven).
- [x] Svelte/React parity; specimens cover both switches in all four
  runtimes.
- [x] No consumer repo, no `Strip` Rust variant, no baseline refreshed, no
  `poodle-ir` change.
- [x] All step-8 commands exit 0.
- [x] Batch log records commands, exit states, the visual diff table, and
  the `Strip` gap.

## 11. Stop conditions

None triggered. The visual gate's non-tabs failures were proven pre-existing
(same slugs, same ratios at HEAD). `activeFill="solid"` contrast is reachable
via `color.text.inverse`. Accessory rendering needed no new `poodle-node`
capability. No non-Tabs component depends on the deleted card variant.
