<!-- parity consv=gap gpui=6 jetstream=1 specimen=gap -->
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

Svelte has two props the contract Props section does not list. Svelte is authoritative — update the contract.

- Svelte adds `count?: number | string | null` (default `null`) → renders a pill-style count badge next to the title (`InlineListSection.svelte:13,33-35,120-134`). Not in contract Props. **Fix: add `count` to contract Props + document the count-badge anatomy + its tokens (min-width 1.875rem, height 1.375rem, pill radius, border, elevated bg).**
- Svelte adds `framed?: boolean` (default `true`) → when false, drops the outer `Card` wrapper and renders the bare section (`InlineListSection.svelte:14,27-89`). Not in contract Props. **Fix: add `framed` to contract Props + describe the unframed posture.**
- Anatomy/tokens absent from contract: the contract has no Token Usage section, but Svelte defines concrete tokens — item-row chrome `background: color-mix(surface 93%, text-primary)`, padding `0.5rem 0.625rem`, radius `calc(radius-surface - 0.1875rem)`; title `typography-label-size/weight`, uppercase, `letter-spacing 0.05em`, `color text-secondary`; section gap `space-stack-md`; items gap `space-stack-sm`. **Fix: add a Token Usage section to the contract capturing these (currently the contract only has Props/Rules/Accessibility, so Rust targets have nothing authoritative to resolve against).**

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

- [ ] **No implementation.** No `inline_list_section.rs` in `packages/jetstream/components/src/`, no specimen, no `InlineListSection` reference in Jetstream source. Top-priority gap: implement `js_inline_list_section(spec, theme, items, actions)` mirroring Svelte — Card wrapper (when `framed`), uppercase label-token title, optional count badge, muted item-row chrome, empty-message posture — resolving every dimension from tokens.
- accepted: interaction (none required; rows are host-owned) — N/A.

## Specimen parity

- Svelte covers: framed section with title + actions + populated rows + count, and empty-state variant (64 lines).
- GPUI covers: title/actions/items + empty (50 lines) — but since the row chrome is missing, the specimen does not visually demonstrate the contract's compact muted rows. Verify `framed=false` and `count` variants are shown.
- Jetstream covers: **none**.

## Notes

- This contract is unusually thin: it has no "Token Usage — Exact Values" section, only Props/Rules/Accessibility. That is the root cause of the Rust hardcoding — there is no authoritative token table to resolve against. The highest-leverage fix is to backfill the contract's Token Usage section from the Svelte CSS, then add matching `*_token()` methods to `InlineListSectionSpec`, then fix GPUI and build Jetstream against them.
- The Svelte item-row uses `color-mix(surface 93%, text-primary)` — a subtle muted tint; GPUI must approximate via opacity-mix (same pattern as other surface-tint components).
