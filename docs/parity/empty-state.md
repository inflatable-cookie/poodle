<!-- parity consv=ok gpui=9 jetstream=5 specimen=ok -->
# Parity: EmptyState

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/empty-state.md`
- Svelte (authoritative): `packages/svelte/components/src/EmptyState.svelte`
- GPUI: `packages/gpui/components/src/composites/empty_state.rs`
- Jetstream: `packages/jetstream/components/src/empty_state.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/EmptyStateSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/empty_state_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/empty_state.rs`

## Contract ↔ Svelte

Svelte implements the full contract — props (`title`, `message`, `variant`, `size`, `density`, `ariaLabel`) + snippets (`visual`, `actions`) with matching defaults (`EmptyState.svelte:10-30`). Anatomy (`<section aria-label>` root, `aria-hidden` visual + variant icon, copy with `<h3>` + optional `<p>`, optional actions), variant background tints (76%/7%/7%), variant icons (inbox/search/plus), compact size, density compact/comfortable, and ARIA (`aria-label` fallback to title, `aria-hidden` visual, `<h3>` heading) all match. consv=ok.

## GPUI gap (vs Svelte + contract)

- [ ] Root is a plain `div` (`empty_state.rs:113-121`), not a `<section>` — no dashed border, no border-radius, no variant background tint, no aria-label.
- [ ] `variant` field is never read in render — variant background tint AND default variant icon both absent. When no illustration passed, there is no visual at all.
- [ ] No circular visual container — default variant icon (inbox/search/plus) not rendered; only the optional `with_illustration` slot shows anything.
- [ ] No `aria-hidden` on visual; title is a plain `div` (`:135-142`), not an `<h3>` heading.
- [ ] No `size` enum (uses bool `compact`); `compact` only affects title font + padding, not message font and (absent) visual circle. No `density` prop.
- [ ] Actions are hand-rolled `div` buttons (`empty_state.rs:157-184`), not real Button components (Jetstream composes `js_button`; GPUI does not).
- [ ] `on_action` callback stored (`empty_state.rs:78`) but never wired to the buttons — dead callback; buttons have no `.on_click`.
- [ ] Hardcoded `.max_w(px(400.0))` at `empty_state.rs:151` (contract message max-width is copy `24rem` — wrong element + raw px); resolve from a token.
- [ ] Hardcoded button padding `.px(px(16.0))` `:168` / `.py(px(8.0))` `:169` and disabled `opacity 0.5` — resolve from control tokens.
- accepted: no ARIA (gpui has no accessibility API) — `<section>` aria-label, `aria-hidden`, heading role.

## Jetstream gap (vs Svelte + contract)

- [ ] Root is a plain `div` (`empty_state.rs:48-50`), not a `<section>` — no dashed border, no border-radius, no variant background tint, no aria-label.
- [ ] Variant background tint not applied — `variant` drives icon selection only (`empty_state.rs:24-28`); circular visual bg is flat `color.background.panel` (`:34`) without the 90% color-mix.
- [ ] No `density` prop; `size` is bool `compact` (full font/icon scaling present — better than GPUI).
- [ ] No custom `visual` slot equivalent.
- [ ] Hardcoded vertical padding `.pt(rem_to_px(2.0)).pb(rem_to_px(2.0))` at `empty_state.rs:50` — contract is `panel_y * 1.5`, density-aware; fixed 2rem ignores density.
- accepted: no ARIA (no `<section>`/aria-label, no `aria-hidden`, no heading).

> Jetstream is the stronger of the two Rust targets: real `js_button` composition (`empty_state.rs:80-88`), variant icon selection, and full compact scaling. Its gaps are border/background/radius, variant tint, density, and aria.

## Specimen parity

- Svelte covers: Neutral (primary action), Search (secondary action), First run (no actions), Compact + custom `visual` snippet. 4 cases incl. visual override.
- GPUI covers: Neutral, Search (aria-label), First run, Compact (4 groups). Note: Search/FirstRun render identically to neutral (variant ignored), so variant differentiation is not actually visible — but state set is complete.
- Jetstream covers: Neutral, Search, FirstRun, Single action, Multiple actions, Compact+search, Compact+message (7 groups). Richest specimen.

specimen=ok: every target's specimen exercises the contract's neutral/search/firstRun/compact/actionable states; Jetstream exceeds Svelte. (Quality caveat: GPUI variants render uniformly because the component ignores `variant`, but the specimen wiring itself covers the states.)

## Notes

- GPUI is the weakest target: ignores `variant` entirely (no tint, no default icon), no border/background/circular-visual/aria, dead `on_action`, hand-rolled buttons with raw px. It barely resembles the contract surface.
- Jetstream is close — main functional gaps are the dashed border + variant background tint + density + aria; its 2rem fixed padding also breaks density.
- The variant background tint (76% neutral / 7% search / 7% firstRun) and dashed border are absent in BOTH Rust targets — the defining visual of this component.
