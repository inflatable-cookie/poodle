<!-- parity consv=ok gpui=0 jetstream=0 specimen=gap | pass: both targets rebuilt — GPUI strips inline-card chrome, composes Progress primitive, gates cancel on can_cancel + correct elevation-overlay token; Jetstream composes ring Spinner + Progress primitive, adds presentation branch, cancel chrome, token border-width (shadow approximated, JsEl delta) -->
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

- [x] Card shadow now resolves from `elevation_overlay_shadow()` (contract §8 `var(--poodle-elevation-overlay)`) — no raw HSLA/px (was wrongly `elevation_dialog`).
- accepted: cancel padding `rem_to_px(0.875)`/`rem_to_px(0.375)` and inline padding `rem_to_px(3.0)`/`rem_to_px(1.0)` are contract-exact rem (`0.375rem 0.875rem`, `3rem 1rem`) — `rem_to_px(<contract rem>)` is not a hardcoded-px violation.
- [x] Inline-vs-overlay chrome: the card now drops border/bg/shadow/padding and caps at max-width 24rem in inline mode (contract §8 inline-card override); overlay keeps full elevated chrome.
- [x] Determinate progress now composes the shared `Progress` primitive (`Progress::from_spec`) inside a full-width wrapper, with ariaLabel = message ?? "Loading progress" — no hand-rolled track / icon-size proxies.
- [x] Cancel button gates on `can_cancel` alone (Svelte parity); the click handler is wired only when one is supplied. Bordered control per contract §8 (`.page-loading__cancel`).
- accepted: no ARIA (gpui has no accessibility API) — `role="status"`/`aria-live` not emitted; backdrop `aria-hidden` n/a.
- accepted: backdrop `blur(2px)` not reproduced (GPUI has no `backdrop-filter`; contract §10 acknowledges this).

## Jetstream gap (vs Svelte + contract)

- [x] Spinner now composes the shared ring `js_spinner` (`variant="ring"`, `sizeRole`→Lg, `tone="accent"`) — no static `icon("loader")`. js_spinner resolves its own diameter, so no hardcoded `rem_to_px(2.0)`.
- [x] Spinner size: resolved inside `js_spinner` from `SpinnerSpec` — no literal diameter in page-loading.
- [x] Border width now `resolve_px(theme, "border.width.default")` — no raw `.border(1.0)`.
- [x] Progress bar now composes the shared `js_progress` primitive (full card width) — no hardcoded bar geometry.
- [x] Presentation branch added: inline renders in-flow (no backdrop, padding `3rem 1rem`, card max-width 24rem, no chrome); overlay renders the full-viewport scrim. Honours `spec.presentation`.
- [x] Cancel is a bordered control (border-width token, radius-control, padding `0.375rem 0.875rem` per contract §8), font from `size_font_rem` not the `* 0.85` heuristic.
- accepted: no card box-shadow — JsEl has no box-shadow primitive; the `elevation-overlay` card shadow is approximated by the border + elevated fill (JsEl delta).
- accepted: no ARIA channel (documented runtime limit).
- accepted: cancel click handler lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers (`PageLoadingSpecimen.svelte`): Inline (toggle), Indeterminate (overlay), Determinate (animated 0→100 progress), With cancel button (interactive `onCancel`). Full §12 coverage plus inline.
- GPUI covers (`page_loading_specimen.rs`): Inline, Indeterminate, Determinate (static `value=64`), With cancel button. Matches Svelte groups (progress is static, not animated — acceptable for build-only preview).
- Jetstream covers (`page_loading.rs`): **Indeterminate**, **Determinate (45%) + cancel** — only 2 groups. Missing: **Inline presentation** group, **standalone With-cancel** group. Both Jetstream specimens wrap in a fixed-height div, so the always-overlay impl is masked.

## Notes

- `consv=ok`: Svelte is a faithful contract implementation here; no contract edits required.
- Biggest cross-target gap is the spinner: Jetstream substitutes a static loader icon for the animated ring, and GPUI hand-rolls the determinate track instead of composing the shared `Progress` primitive. Both undercut the "real component, resolves from tokens" standard.
- Jetstream ignores `presentation` entirely — inline mode is a no-op, so the inline specimen group is also absent.
