<!-- parity consv=gap gpui=4 jetstream=7 specimen=gap -->
# Parity: Button

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/button.md`
- Svelte (authoritative): `packages/svelte/components/src/Button.svelte`
- GPUI: `packages/gpui/components/src/primitives/button.rs`
- Jetstream: `packages/jetstream/components/src/button.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ButtonSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/button.rs` · jetstream `packages/jetstream/preview/src/specimens/button.rs`

## Contract ↔ Svelte

Svelte has props/behavior the contract does not document. Svelte is authoritative — update the contract.

- Svelte adds `truncate?: boolean` (default false) → emits `data-truncate`, sets `overflow:hidden` + ellipsis on label. Not in contract §3. **Fix: add to contract props + anatomy.**
- Svelte adds `fit?: "default" | "content"` (default `"default"`) → `data-fit="content"` drops `min-width` and sets `padding-inline: 0.375rem`. Not in contract. **Fix: add to contract.**
- Svelte adds `maxWidth?: string | null` → composes into inline `style` (`max-width`). Not in contract. **Fix: add to contract.**
- Svelte adds a `warning` tone (full idle/hover/active CSS for secondary/primary/ghost × warning, lines 452–481). Contract §3 only lists `tone: "default" | "danger"`. **Fix: add `"warning"` to contract `tone` union + token tables.**
- `defaultPressed` default: contract says `false`; Svelte default is `null` (line 84) and `isToggle` derives from `pressed !== null || defaultPressed !== null`. Contract §3/§8 say `defaultPressed` default `false` and "toggle mode when defaultPressed is set". Minor divergence in default value. **Fix: reconcile contract default to `null`.**
- Icon-padding xs/sm/lg/xl adjustments in Svelte (lines 319–349) use different per-size offsets than the contract's flat "−0.125rem each side" rule (§8 says reduce by 0.125rem; Svelte xs reduces by 0.1875rem, sm by 0.25rem, lg by 0, xl by +0.0625rem). **Fix: document the per-size icon-inset table in contract §8.**

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded hover shadow color literal `hsla(0.0, 0.0, 1.0, 0.10)` at `button.rs:362` — resolve from a token, not a raw HSLA.
- [ ] No `truncate` / `fit` / `maxWidth` support (props absent from builder + spec usage).
- [ ] No `warning` tone branch — only `Danger` handled in the variant/tone match (`button.rs:196-279`); warning falls through to default.
- [ ] `pressed`/toggle: spec exposes `is_toggle_mode()`/`current_pressed()` and pressed fill is applied (`button.rs:228-237`), but there is no toggle builder method (`pressed`/`default_pressed`) and click does not flip pressed state — toggle is render-only, not interactive.
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded` stored but not emitted (documented in file header).
- accepted: active `translateY` omitted (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded icon-inset literal `pad_x - 2.0` at `button.rs:79-80` — use `ButtonSpec::icon_side_inset_token()` resolved to px, not raw `2.0`.
- [ ] Gap is ad-hoc `control_space_x_rem * 0.5` at `button.rs:83` — contract gap is `0.375rem` via a content-gap token (GPUI uses `ButtonSpec::content_gap_token()`); resolve from token, drop the `* 0.5` heuristic.
- [ ] Icon-side padding only reduced when `leading_icon`/`trailing_icon` present; does not account for `is_loading` (leading) or `chevron` (trailing) like Svelte/`has_leading`/`has_trailing`.
- [ ] No `chevron` rendering — contract §2 anatomy + specimen require a trailing chevron; `js_button` never reads `spec.chevron`.
- [ ] No `warning` tone — only `is_danger_tone` branch exists (`button.rs:38-63`).
- [ ] No pressed/toggle accent treatment (`is_toggle_mode`/`current_pressed` unused).
- [ ] No `truncate`/`fit`/`maxWidth` support.
- accepted: no ARIA channel for `aria_expanded` (documented in header).
- accepted: interaction (click handler) lives in preview event loop, not the component.

## Specimen parity

- Svelte covers: Variants, Danger tone, With icons, With chevron, Sizes, States, Click counter (`ButtonSpecimen.svelte`).
- GPUI covers: Variants, Danger, icons, chevron, sizes, states (502 lines, broad). — missing: verify click-counter parity (interactive).
- Jetstream covers: Variants, Danger tone, With icons, Sizes, States. — missing: **With chevron** group, **Click counter** text. (`jetstream/.../button.rs`)

## Notes

- Svelte trailing-icon specimen uses unicode glyphs (`→`, `□`, `✓`) in Jetstream instead of registry icon names (`external-link`, `save`, `check`) — Jetstream icon registry coverage gap, not a contract gap.
- The big `consv=gap` driver is undocumented Svelte surface (`truncate`/`fit`/`maxWidth`/`warning` tone). All four belong in the contract per "Svelte is parity authority".
