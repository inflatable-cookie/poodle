<!-- parity consv=fixed gpui=0 jetstream=1 specimen=ok -->
<!-- jetstream-specimen pass: created `packages/jetstream/preview/src/specimens/inline_list_section.rs` (registered in mod.rs) mirroring GPUI — Framed (count pill + `js_icon_button` add action + 3 status rows), Header actions (no count, ghost icon button), Empty state (italic empty message), Unframed (count + rows, no card). Rows compose real `js_text` + `js_pill`; section is real `js_inline_list_section`. No fakes. Both previews build 0 errors. -->
<!-- pass 46: GPUI inline_list_section rebuilt to the contract — compact muted row chrome
     (color_mix(surface 93%, text-primary), padding 0.5/0.625rem, radius calc(radius.surface −
     0.1875rem)), full count pill (min-w 1.875rem, h 1.375rem, bg background.elevated, border
     0.0625rem, label-size/weight), title at typography.label.size + SEMIBOLD uppercase, empty
     at typography.body.size + italic, items gap space.stack.sm. Zero hardcoded px/hsla — all
     token or contract-rem. Jetstream impl aligned to the same token values (label/body size,
     count bg+geometry, row chrome). gpui builds, jet probe tests pass. Remaining jetstream
     (1): the specimen. -->
# Parity: InlineListSection

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/inline-list-section.md`
- Svelte (authoritative): `packages/svelte/components/src/InlineListSection.svelte`
- GPUI: `packages/gpui/components/src/composites/inline_list_section.rs`
- Jetstream: **missing** — no `packages/jetstream/components/src/inline_list_section.rs`
- Spec: `packages/contracts/components/src/inline_list_section.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/InlineListSectionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/inline_list_section_specimen.rs` · jetstream — none

## Contract ↔ Svelte

Svelte has two props the contract Props section did not list, plus no Token Usage section. Svelte is authoritative — contract reconciled.

- [x] FIXED `count?: number | string | null` (default `null`) — added to contract Props, Anatomy (Count `<span>`), and a full Count token table in Token Usage (min-width `1.875rem`, height `1.375rem`, padding `0 0.5rem`, pill radius `999rem`, `1px solid border`, `surface-elevated` bg, label typography). Source `InlineListSection.svelte:13,33-35,120-134`.
- [x] FIXED `framed?: boolean` (default `true`) — added to contract Props + Anatomy note (Card only when `framed=true`) + Accessibility note. Source `InlineListSection.svelte:14,27-89`.
- [x] FIXED Token Usage section backfilled from Svelte CSS: Root (`grid`, `space-stack-md`), Header, Heading, Title (`label-size/weight`, uppercase, `letter-spacing 0.05em`, `text-secondary`), Count, Header Actions (`0.375rem`, `0.25rem` at `45rem`), Items (`space-stack-sm`), Item chrome (`color-mix(surface 93%, text-primary)`, padding `0.5rem 0.625rem`, radius `calc(radius-surface - 0.1875rem)`), Empty (italic, `body-size`, `text-secondary`). Also added a full Anatomy block (was absent).

## GPUI gap (vs Svelte + contract)

The compact muted **row chrome is entirely missing** and most dimensions are hardcoded px.

- [ ] Missing item-row chrome: items are pushed raw into a flex column (`inline_list_section.rs:84-88`) with no per-row background (`color-mix(surface 93%, text-primary)`), no padding (`0.5rem 0.625rem`), no radius (`calc(radius-surface - 0.1875rem)`). Svelte's `.poodle-inline-list-section__item` chrome is absent. Add it, resolving from tokens.
- [ ] Hardcoded px throughout: header gap `px(8.0)` (`:54`), count badge `px(8.0)`/`px(2.0)`/`px(999.0)` (`:66-67`), items gap `px(6.0)` (`:84`). Contract/Svelte use `space.stack.sm`, count padding `0 0.5rem`, pill radius `999rem`. Resolve from tokens (items gap = `space.stack.sm`, header heading gap = `0.5rem` token).
- [ ] Count badge incomplete vs Svelte: missing `min-width 1.875rem`, `height 1.375rem`, `background surface-elevated`, label-size/weight typography (`:62-73`). Only border + text-xs applied.
- [ ] Title typography hardcoded as `text_xs()` + `FontWeight::SEMIBOLD` (`:57-58`) instead of resolving `typography.label.size`/`typography.label.weight`; uppercase done via `.to_uppercase()` on the string (`:60`) rather than a text-transform — loses letter-spacing `0.05em`.
- [ ] Empty message uses `text_sm()` (`:81`) not `typography.body.size`, and omits the `font-style: italic` posture from Svelte.
- [ ] Spec exposes no token methods (`inline_list_section.rs` has only data fields) — GPUI has nothing to resolve from, forcing the hardcoding. Add `*_token()` helpers to the spec once the contract grows a Token Usage section.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label={title}` on the section not emitted.

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `js_inline_list_section(spec, theme, items, action)` created — Card wrapper when `framed` (composes `js_card`), uppercase title, optional count pill, empty-message posture, item list; gap from `space.stack.md`, other dims from contract exact-rem. Registered in lib.rs, probe-tested.
- [ ] Add the Jetstream specimen (framed/unframed, with count, empty, with items + action).
- accepted: interaction (none required; rows are host-owned) — N/A.

## Specimen parity

- Svelte covers: framed section with title + actions + populated rows + count, and empty-state variant (64 lines).
- GPUI specimen DONE; Jetstream pending engine recovery: **Framed — count pill + header action** (uppercase title, count pill, `IconButton` add action, three muted rows with `Pill` status), **Header actions (no count)** (ghost `IconButton`), **Empty state** (italic empty message), **Unframed** (count + rows, no card). Rows render the contract's compact muted row chrome from the rebuilt component (pass 46). All real `InlineListSection::from_spec` / `IconButton` / `Pill` / `Text` — no hand-rolled boxes. `gpui/preview` builds 0 errors. Note: divider lines are not a contract part — rows are spaced muted boxes (`space.stack.sm` gap), which the component renders.
- Jetstream covers: **none** — component + specimen deferred (engine externally build-blocked).

## Notes

- This contract is unusually thin: it has no "Token Usage — Exact Values" section, only Props/Rules/Accessibility. That is the root cause of the Rust hardcoding — there is no authoritative token table to resolve against. The highest-leverage fix is to backfill the contract's Token Usage section from the Svelte CSS, then add matching `*_token()` methods to `InlineListSectionSpec`, then fix GPUI and build Jetstream against them.
- The Svelte item-row uses `color-mix(surface 93%, text-primary)` — a subtle muted tint; GPUI must approximate via opacity-mix (same pattern as other surface-tint components).
