<!-- parity consv=fixed gpui=3 jetstream=3 specimen=gap | pass: ButtonTone::Success added cross-cutting; GPUI hover-shadow token-resolved + success branches; Jetstream icon-inset/gap tokenized, has_leading/has_trailing + chevron + success tone. Remaining GPUI: truncate/fit/maxWidth, warning tone, interactive toggle. Remaining Jetstream: pressed/toggle, truncate/fit/maxWidth, warning tone. -->
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

- [x] FIXED `truncate?: boolean` (default false): added to §3 props; added `[data-truncate]` overflow/ellipsis CSS to §8; noted `data-truncate` presence-only emit in §9.
- [x] FIXED `fit?: "default" | "content"` (default `"default"`): added to §3 props; added `[data-fit="content"]` (`min-width:0`, `padding-inline:0.375rem`) CSS to §8 + §7 sizing; noted `data-fit` emit in §9.
- [x] FIXED `maxWidth?: string | null`: added to §3 props; §9 documents composition into inline `style` as `max-width`.
- [x] FIXED `warning` tone: added `"warning"` to the §3 `tone` union; added full secondary/primary/ghost × warning idle/hover/active token tables to §8 (mirrors danger with `status-warning`); §1 in-scope updated.
- [x] FIXED `defaultPressed` default: changed contract §3 default `false` → `null`; toggle-activation notes in §3/§8/§9 reconciled to `pressed !== null || defaultPressed !== null`.
- [x] FIXED Icon-padding per-size offsets: replaced the flat "−0.125rem each side" rule in §8 with the per-size icon-inset table (xs −0.1875, sm −0.25, md −0.125, lg 0, xl +0.0625) and documented `data-has-leading`/`data-has-trailing` (loading counts as leading, chevron as trailing). Also corrected the §7/§8 size-table padding (flat `0 space-control-x` across sizes, fixed-rem heights) and icon-only widths (explicit `1.5/1.75/2.25/2.75/3.25rem`) to Svelte's actual values.

## GPUI gap (vs Svelte + contract)

- [x] FIXED Hardcoded hover shadow literal `hsla(0.0, 0.0, 1.0, 0.10)` — replaced with `theme_ext::button_hover_shadow()` encoding contract §8 `inset 0 0.0625rem 0 color-mix(white 8%, transparent)` (white via `gpui::white()`, offset via `rem_to_px(0.0625)`). No raw HSLA in the hover path.
- [x] FIXED `success` tone — explicit `Success` branches added to both `(variant, tone)` matches (base colors + hover/active), mirroring danger with `color.status.success`. Shared `ButtonTone::Success` resolves base tokens; component applies the secondary 16%/24%/32% color-mixes.
- [ ] No `truncate` / `fit` / `maxWidth` support (props absent from builder + spec usage).
- [ ] No `warning` tone branch — `warning` falls through to default. (`ButtonTone` has no `Warning` arm; separate cross-cutting pass.)
- [ ] `pressed`/toggle: spec exposes `is_toggle_mode()`/`current_pressed()` and pressed fill is applied (`button.rs:228-237`), but there is no toggle builder method (`pressed`/`default_pressed`) and click does not flip pressed state — toggle is render-only, not interactive.
- accepted: no ARIA (gpui has no accessibility API) — `aria_expanded` stored but not emitted (documented in file header).
- accepted: active `translateY` omitted (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Hardcoded icon-inset literal `pad_x - 2.0` — now `resolve_px(theme, ButtonSpec::icon_side_inset_token())` (`space.button.iconInset` = 0.125rem).
- [x] FIXED Ad-hoc gap `control_space_x_rem * 0.5` — now `resolve_px(theme, ButtonSpec::content_gap_token())` (`space.button.gap` = 0.375rem); `* 0.5` heuristic dropped.
- [x] FIXED Icon-side padding now uses `has_leading` (leading icon OR loading) and `has_trailing` (trailing icon OR chevron), matching Svelte `data-has-leading`/`data-has-trailing`.
- [x] FIXED `chevron` rendering — `js_button` now reads `spec.chevron` and emits a trailing `chevron-down` glyph at 0.5 opacity (contract §2/§8).
- [x] FIXED `success` tone — danger/success share a `status_token` path; secondary success applies `color-mix(success 16%, surface)` fill + `color-mix(success 46%, border-default)` border; primary/ghost success via shared token methods.
- [ ] No `warning` tone — `ButtonTone` has no `Warning` arm (separate cross-cutting pass).
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
