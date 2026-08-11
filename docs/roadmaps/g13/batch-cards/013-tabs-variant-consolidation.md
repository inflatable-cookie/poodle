# 013 Tabs Variant Consolidation

Status: ready
Milestone: side-quest (component API change, outside the g13 IR lane)
Owner: Poodle core
Branch: `thread/g13-013-tabs-variant-consolidation`
Governing refs: `docs/contracts/components/tabs.md`,
`docs/contracts/001-working-rules.md` §Runtime Parity Authority,
`docs/contracts/004-shared-control-types.md`

## Goal

Collapse the Tabs variant set from five to four and make the two decorations
that distinguished `card` into opt-in switches.

Today `text` and `card` differ mostly in *where* the selection is drawn (tab vs
item) and whether the item carries an outline. That is one variant with two
switches, not two variants.

## Maintainer Rulings (already decided — do not re-litigate)

1. **Variant set becomes `Card | Pill | Block | Strip`.** The current `text`
   variant is renamed `card`. The current `card` variant is deleted.
2. **`text` and `underline` are removed outright.** No aliases, no deprecation
   window. Consumers are migrated separately; nothing is in production.
3. **`activeOutline: boolean`, default `false`.** Opt-in outline on the active
   tab — the decoration the old `card` variant had by default.
4. **`activeFill: "tint" | "solid"`, default `"tint"`.** `tint` is the current
   accent-tinted selection. `solid` is new: a fully accent-filled active tab.
5. **Existing `variant="card"` usages intentionally change appearance** — they
   lose the outline and become flat. That is the point of the change.
6. **Naming:** do **not** call the new prop `outlined` or `bordered`.
   `bordered` already exists and means the *list* bottom border. Two similar
   names for different surfaces is the `ButtonTone` failure mode. Both new
   props are prefixed `active` to scope them to the selected tab.

## Known State (verified — build on this, do not re-derive)

- Web accessories are **not** variant-gated. `Tabs.svelte` renders
  `item.icon`, `item.count` and `item.closable` in both render branches, so the
  renamed variant already supports every accessory on web.
- **Rust is the bulk of this card.** `render_underline` renders **no**
  accessories at all — no icon, no count, no close. Only `render_card` receives
  `on_close` (`packages/render/src/tabs.rs:152-155`).
- Rust `TabVariant` is `Underline | Card | Pill | Block`
  (`packages/contracts/components/src/types.rs:780`). There is **no `Strip`** —
  a pre-existing gap, see below.
- `tabs.md` line 765 documents the `underline`/`text` naming mismatch. That
  paragraph goes away with this change.

## Scope

### In scope

- `tabs.md`: variant union, the two new props, accessory support on the
  renamed variant, removal of the naming-mismatch note, `data-*` table.
- `tabs.css`: rename `text` rules to `card`, delete the old `card` rules, add
  `activeOutline` and `activeFill="solid"` treatments.
- `Tabs.svelte` and `Tabs.tsx`: variant type, both new props, identical
  semantics and defaults.
- Rust: rename `TabVariant::Underline` → `Card`, delete the old `Card` member
  and `render_card`, add both new props to `TabsSpec`, and **give the renamed
  renderer full accessory support** (icon, count, close) matching the web.
- Specimens in all four runtimes: replace card-variant specimens, add coverage
  for `activeOutline` and `activeFill="solid"`.

### Out of scope — stop conditions if reached

- Consumer repositories. The 29-site consumer migration is handled separately
  by the orchestrator. Do not edit anything outside this repo.
- Adding a `Strip` Rust variant. Real gap, deliberately deferred — record it,
  do not build it.
- Changing `bordered`, or any variant other than the renamed one.
- Refreshing visual baselines. Enumerate and classify; do not `--update`.
- Touching `poodle-ir`. This is component work, not IR work.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Svelte and React must stay at parity — same props, semantics, defaults.
- The contract is authority and must be updated in the same commit.
- Rust `TabsSpec` must carry both new props; this is not a web-only change.
- Do not refresh baselines. A diff on a component other than tabs is a stop
  condition.
- `effigy docs:check` rewrites `packages/tokens/artifacts/rust/*`; restore with
  `git checkout -- packages/tokens/artifacts/rust/` and never commit it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-013-tabs-variant-consolidation`. Do not merge.

## Writable Paths

- `docs/contracts/components/tabs.md`
- `packages/core/src/styles/tabs.css`
- `packages/svelte/components/src/Tabs.svelte`
- `packages/react/components/src/Tabs.tsx`
- `packages/contracts/components/src/{types.rs,tabs.rs}`
- `packages/render/src/tabs.rs`
- `packages/{svelte,react}/preview/src/**/TabsSpecimen.*`
- `packages/{gpui,jetstream}/preview/src/specimens/tabs.rs`
- `packages/svelte/preview/src/component-docs.ts` (`tabs` entry only)
- Tests for Tabs in either web runtime
- `docs/logs/2026-08/<DD>-g13-013-tabs-variant-consolidation.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. **Baseline.** `bun install`, `effigy test:components`, `effigy test:parity`,
   `effigy docs:lint`, `cargo test -p poodle-render`, `git diff --check`.
   Record exit states.
2. **Contract first.** Amend `tabs.md`: union to
   `"card" | "pill" | "block" | "strip"`, add `activeOutline` and `activeFill`
   with types/defaults/behaviour, state that the renamed variant supports icon,
   count and close, and delete the line-765 naming note.
3. **CSS.** Rename the `[data-variant="text"]` rules to `card`, delete the old
   `[data-variant="card"]` rules, then add:
   - `[data-active-outline="true"]` — outline on the selected item, reusing the
     old card selected-border value so the opted-in look matches what card gave.
   - `[data-active-fill="solid"]` — fully accent-filled active tab with
     accessible foreground contrast against `accent-base`.
4. **Web components.** Both props in Svelte and React, identical defaults,
   emitting the two new `data-*` attributes.
5. **Rust.** Rename the enum member, delete old `Card` and `render_card`, add
   both props to `TabsSpec`, and extend the renamed renderer to render icon,
   count and close — wiring `on_close` the way `render_card` did.
6. **Specimens** in all four runtimes, including the two new switches.
7. **Visual enumeration.** Run the gate in report mode. Table of every changed
   baseline: slug, axis, diff ratio, cause, classified expected/unexpected.
   Any diff outside tabs is a stop condition. Refresh nothing.
8. **Validate.**
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

- [ ] Variant union is exactly `card | pill | block | strip` in contract, both
  web runtimes, and Rust; `text`, `underline`, and the old card variant are
  gone everywhere.
- [ ] `activeOutline` and `activeFill` exist in contract, both web runtimes,
  and `TabsSpec`, with the ruled defaults.
- [ ] `activeFill="solid"` renders a fully accent-filled active tab with
  legible foreground.
- [ ] The renamed Rust renderer renders icon, count, and close, wired to
  `on_close`.
- [ ] Svelte and React are at parity; specimens cover both new switches in all
  four runtimes.
- [ ] No consumer repo, no `Strip` Rust variant, no baseline refreshed, no
  `poodle-ir` change.
- [ ] All step-8 commands exit 0.
- [ ] Batch log records commands, exit states, the visual diff table, and the
  `Strip` gap.

## Stop Conditions

- A visual diff appears on a component other than tabs.
- Accessory rendering in Rust needs a `poodle-node` capability that does not
  exist.
- `activeFill="solid"` cannot reach accessible contrast with existing tokens.
- Removing the old card variant breaks a non-Tabs component.

Stop with exact paths, selectors, commands, and the smallest unresolved
question.
