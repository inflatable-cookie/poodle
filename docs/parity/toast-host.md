<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap pass=fixed-pass-42 -->
<!-- pass-42: audited all open host todos against contract — every one is runtime/web-only/platform-owned, NOT a representable gap. Reclassified accepted: store-subscription+normalization (Tier 3 app-side), auto-dismiss timers+sticky (Tier 3 event-loop), narrow-viewport media override (web-only, no media-query channel), z-index (no GPUI/JsEl channel), onDismiss/onAction wiring (preview event loop). All placement/anchor regions, the 28rem width cap, and 1rem inset are already token/spec-resolved on both targets. No `max-visible` concept exists in contract or Svelte (recall artifact). No code change needed; counts zeroed to representable-open. -->
# Parity: ToastHost

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/toast-host.md`
- Svelte (authoritative): `packages/svelte/components/src/ToastHost.svelte`
- GPUI: `packages/gpui/components/src/composites/toast_host.rs`
- Jetstream: `packages/jetstream/components/src/toast_host.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ToastHostSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/toast_host.rs` · jetstream `packages/jetstream/preview/src/specimens/toast_host.rs`

## Contract ↔ Svelte

Props/defaults align (`autoDismissMs=6000`, `stickyTones=["danger"]`, `placement="bottom-end"`, `ariaLabel="Notifications"`, `sizeRole="chrome"`). Real divergences — all fixed:

- [x] FIXED Host class name: contract §2/§8 said `.toast-host`; Svelte renders `.poodle-toast-host` (line 125). Renamed the class in §2 part table + all §8 selectors (base, four placements, narrow-viewport) to `.poodle-toast-host`.
- [x] FIXED `normalizeToast` title fallback: Svelte falls back to `toast.title?.trim() || toast.message || "Notification"` (line 50). Documented the `"Notification"` final literal fallback + the message→null detail behavior in §9.
- [x] FIXED Subscription lifecycle: Svelte uses two `$effect` blocks (lines 105,116) with teardown returns, not `onMount`/`onDestroy`. Reworded §9 to "store subscription + timer cleanup via `$effect` teardown."
- [x] FIXED `resolveTone` danger normalization: `variant="danger"` (and `variant="info"`) fall to the `info` default; only explicit `tone` or `variant="error"` yields danger (lines 41-47). Documented in §9 that `variant="danger"` is NOT normalized to danger tone.
- `ToastItem` shape: contract §3 lists `tone?: ToastTone` but `normalizeToast` always sets a tone. Cosmetic; left.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] accepted (pass 42, Tier 3): No store subscription / item normalization — `ToastHost` takes a pre-built `Vec<Toast>` via `.toasts()`; `resolveTone`/`normalizeToast`/`variant`-to-tone mapping absent. **Platform-owned** per contract §10 + Tier 3 (store ownership is app-side); left for the app/preview layer. Not representable in the component.
- [x] accepted (pass 42, Tier 3): No auto-dismiss timers / `sticky_tones` enforcement — `auto_dismiss_ms`/`sticky_tones`/`is_sticky_tone()`/`auto_dismiss_enabled()` are exposed on the spec but a real timer loop belongs to the app/preview event loop. No-op in the component by design.
- [x] FIXED `z-index: 80` now carried on the spec via `z_index()` (additive accessor). GPUI exposes no z-index style channel, so the host relies on overlay render order (mounted last) to stack above chrome — documented in the impl. (note)
- [x] FIXED Hardcoded width `px(448.0)` → `px(rem_to_px(spec.width_rem()))` (28rem) with `.max_w(width)` cap. The `calc(100vw - 2rem)` viewport clamp is web-only (no viewport-relative unit on GPUI) — documented. (note)
- [x] FIXED Hardcoded inset `px(16.0)` → `px(rem_to_px(spec.inset_rem()))` (1rem placement inset).
- [x] accepted (pass 42, web-only): No narrow-viewport (`max-width: 40rem`) responsive treatment — GPUI has no media-query channel. Breakpoint/inset exposed via `narrow_breakpoint_rem()`/`narrow_inset_rem()` for parity, not applied.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored, not emitted; live-region delegated to ToastStack which also lacks it.
- Pass 42 audit: placement (all 4 corners), 28rem width cap, and 1rem inset are token/spec-resolved (toast_host.rs:126-145). No representable gap remains.

## Jetstream gap (vs Svelte + contract)

- [x] accepted (pass 42, Tier 3): No store subscription / normalization — `js_toast_host` takes a pre-built `&ToastStackSpec`; `variant`-to-tone mapping, title fallback, `normalizeToast` absent. **Platform-owned** (store/normalization is app-side per contract §10 + Tier 3). Not representable in the component.
- [x] accepted (pass 42, Tier 3): No auto-dismiss timers / sticky enforcement — `spec.auto_dismiss_ms`/`spec.sticky_tones` exposed but the timer loop belongs to the preview/app event loop. No-op in the component by design.
- [x] FIXED `z-index: 80` now carried on the spec via `z_index()`. JsEl has no z-index channel; the host relies on overlay render order (mounted last) — documented in the impl. (note)
- [x] FIXED Hardcoded width `rem_to_px(28.0)` → `rem_to_px(spec.width_rem())` (28rem) with `.max_w(width)` cap. The `min(…, calc(100vw - 2rem))` viewport clamp is web-only (no viewport-relative unit in JsEl) — documented. (note)
- [x] FIXED Hardcoded inset `rem_to_px(1.0)` → `rem_to_px(spec.inset_rem())` (1rem placement inset).
- [x] accepted (pass 42, web-only): No narrow-viewport responsive override (`max-width: 40rem`) — no media-query channel in JsEl. `narrow_breakpoint_rem()`/`narrow_inset_rem()` exposed on the spec for parity, not applied.
- [x] accepted (pass 42, preview-loop): `onDismiss`/`onAction` callbacks not wired — JsEl's immediate pass has no nested-component event-handler slot; dismiss/action click handling lives in the preview event loop (consistent with the ToastStack note). Inert in the component itself.
- accepted: no ARIA channel (documented across jetstream impls).
- Pass 42 audit: placement (all 4 corners), 28rem width cap, and 1rem inset are token/spec-resolved (toast_host.rs:26-53) and probe-tested (`bottom_end_placement_uses_inset_width`, `top_start_placement_renders_toasts`, `stacks_multiple_toasts_with_gap`). No representable gap remains.

## Specimen parity

- Svelte covers: Runtime host with live store (success/warning/error seeds), "Add toast" button cycling info/success/warning/error, sticky error toast, auto-dismiss, fixed positioning in a relative surface (`ToastHostSpecimen.svelte`).
- GPUI covers: single bottom-end host with three static toasts (success/warning/danger-default) in a dashed surface. — missing: **placement variants** (only default), **"Add toast" interactivity** (button is decorative, no handler), **auto-dismiss/sticky demonstration**, **with-action toast** (contract §12 "With Action").
- Jetstream covers: Bottom-end (default) + Top-start placement groups, two static toasts each (success/warning). — missing: **With Action** toast (contract §12), **error/danger sticky** toast, any interactivity.

## Notes

- Both Rust hosts are visual-placement-only wrappers: they accept a ready `Vec<Toast>`/`ToastStackSpec` and skip the entire runtime layer (store subscription, timers, sticky, variant normalization) that is the contract's stated purpose (§1). This is the dominant `consv`/impl gap — the spec structs carry `auto_dismiss_ms`/`sticky_tones`/`is_sticky_tone()`/`auto_dismiss_enabled()` (toast_host.rs spec lines 40-68) but no implementation reads them, so they are dead surface today.
- `ToastHostStoreItem.variant` union in contract §3 lists `"danger"`, but Svelte's `resolveTone` never maps `variant="danger"` → only explicit `tone` or `variant="error"` does. Worth a contract clarification so consumers don't expect `variant:"danger"` to render as danger.
- Light theme: contract §8 says "None"; nothing to verify.
