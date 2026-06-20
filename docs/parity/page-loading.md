<!-- parity consv=ok gpui=5 jetstream=7 specimen=gap -->
# Parity: PageLoading

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/page-loading.md`
- Svelte (authoritative): `packages/svelte/components/src/PageLoading.svelte`
- GPUI: `packages/gpui/components/src/composites/page_loading.rs`
- Jetstream: `packages/jetstream/components/src/page_loading.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PageLoadingSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/page_loading_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/page_loading.rs`

## Contract ↔ Svelte

Svelte matches the contract: props (`visible`, `presentation`, `value`, `max`, `message`, `canCancel`, `ariaLabel`), defaults, anatomy (root `role="status"` + `aria-live="polite"`, backdrop overlay-only, card, Spinner ring/prominent/accent, Progress, message, cancel button), and the `onCancel` callback all align. No divergence.

- Minor: contract §3 omits the `onCancel` prop from the public-props table (it appears only in §5 callbacks). Svelte declares `onCancel?: (() => void) | undefined` (line 24). Not a parity defect — noting for completeness.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded card shadow built from raw `hsla(0,0,0,0.12)`/`hsla(0,0,0,0.08)` + `px(8.0)`/`px(24.0)`/`px(2.0)` offsets at `page_loading.rs:101-112` — contract §8 card shadow is `var(--poodle-elevation-overlay)`; resolve from an elevation token, not literal HSLA + pixel blur.
- [ ] Cancel padding uses literal rem args `rem_to_px(0.875)`/`rem_to_px(0.375)` at `page_loading.rs:192-193`, and inline-mode padding `rem_to_px(3.0)`/`rem_to_px(1.0)` at `:214-215` — these bypass tokens (Svelte values `0.375rem 0.875rem` and `3rem 1rem`); resolve from space tokens.
- [ ] No inline-vs-overlay chrome difference on the card. Contract §8 says the inline card drops border/bg/shadow (`min-width:auto`, `padding:0`, `border:none`, `background:transparent`). GPUI builds one card with full chrome (border/bg/shadow, `page_loading.rs:94-119`) for both modes; inline only changes the outer wrapper padding (`:208-217`). **Strip card chrome in inline mode.**
- [ ] No determinate Progress primitive — track is hand-rolled `div().w(track_width).h(track_height)` (`page_loading.rs:151-165`) using `size.icon.xs` / `size.control.minWidth` proxies (admitted in comment `:147-149`); Svelte composes the shared `Progress` primitive. No ariaLabel, no `Progress` token chrome.
- [ ] Cancel button only renders when BOTH `can_cancel` AND an `on_cancel` handler are set (`page_loading.rs:180-181`); Svelte renders the button on `canCancel` alone (handler optional). Visibility should gate on `can_cancel` only.
- accepted: no ARIA (gpui has no accessibility API) — `role="status"`/`aria-live` not emitted; backdrop `aria-hidden` n/a.
- accepted: backdrop `blur(2px)` not reproduced (GPUI has no `backdrop-filter`; contract §10 acknowledges this).

## Jetstream gap (vs Svelte + contract)

- [ ] No spinner — renders a static `icon("loader")` (`page_loading.rs:42-46`), not an animated ring Spinner. Contract §2 requires the shared Spinner primitive (`variant="ring"`). No motion = wrong loading affordance.
- [ ] Spinner size hardcoded `rem_to_px(2.0)` at `page_loading.rs:31` — resolve from an icon/size token.
- [ ] Border width literal `.border(1.0)` at `page_loading.rs:36` — resolve from `border.width` token, not raw `1.0`.
- [ ] Progress bar geometry fully hardcoded: `bar_h = rem_to_px(0.25)`, `bar_w = rem_to_px(12.0)`, `bar_radius = rem_to_px(0.125)` at `page_loading.rs:58-60` — no Progress primitive, no token resolution.
- [ ] No presentation branch — `js_page_loading` always renders the full-viewport backdrop (`page_loading.rs:88-92`) and ignores `spec.presentation`. Inline mode (no backdrop, in-flow, `max-width:24rem`, no chrome) is unimplemented. Contract §3/§4 require both modes.
- [ ] Cancel is a non-interactive `label("Cancel")` (`page_loading.rs:78-85`) — no button chrome (border/radius/padding per contract §8 `.page-loading__cancel`), no hover, no click wiring. Size uses heuristic `* 0.85` multiplier, not a token.
- [ ] No card shadow / elevation — card has border + bg but omits the `elevation-overlay` shadow the contract §8 card requires.
- accepted: no ARIA channel (documented runtime limit).
- accepted: cancel click handler would live in preview event loop, not the component — but the cancel element is currently a plain label with no id/affordance to wire.

## Specimen parity

- Svelte covers (`PageLoadingSpecimen.svelte`): Inline (toggle), Indeterminate (overlay), Determinate (animated 0→100 progress), With cancel button (interactive `onCancel`). Full §12 coverage plus inline.
- GPUI covers (`page_loading_specimen.rs`): Inline, Indeterminate, Determinate (static `value=64`), With cancel button. Matches Svelte groups (progress is static, not animated — acceptable for build-only preview).
- Jetstream covers (`page_loading.rs`): **Indeterminate**, **Determinate (45%) + cancel** — only 2 groups. Missing: **Inline presentation** group, **standalone With-cancel** group. Both Jetstream specimens wrap in a fixed-height div, so the always-overlay impl is masked.

## Notes

- `consv=ok`: Svelte is a faithful contract implementation here; no contract edits required.
- Biggest cross-target gap is the spinner: Jetstream substitutes a static loader icon for the animated ring, and GPUI hand-rolls the determinate track instead of composing the shared `Progress` primitive. Both undercut the "real component, resolves from tokens" standard.
- Jetstream ignores `presentation` entirely — inline mode is a no-op, so the inline specimen group is also absent.
