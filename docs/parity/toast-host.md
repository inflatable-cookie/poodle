<!-- parity consv=fixed gpui=6 jetstream=7 specimen=gap -->
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

- [ ] No store subscription / item normalization — `ToastHost` takes a pre-built `Vec<Toast>` via `.toasts()` (toast_host.rs:90); `resolveTone`/`normalizeToast`/`variant`-to-tone mapping all absent. Add normalization or document store ownership as app-side.
- [ ] No auto-dismiss timers and no `sticky_tones` enforcement — `auto_dismiss_ms`/`sticky_tones` live on the spec (toast_host.rs:64-71) but are never read; nothing dismisses non-sticky toasts.
- [ ] Missing `z-index: 80` — contract §7/§8 require it; `container` (toast_host.rs:125) sets `.absolute()` but no z-index/stacking control.
- [ ] Hardcoded width `px(448.0)` at toast_host.rs:122 instead of the `min(28rem, calc(100vw - 2rem))` clamp — resolve 28rem from rem scale and apply the viewport clamp (or document clamp as web-only).
- [ ] Hardcoded inset `px(16.0)` at toast_host.rs:121 — 1rem placement inset should resolve from a token/rem helper, not a raw literal.
- [ ] No narrow-viewport (`max-width: 40rem`) responsive treatment — `width: calc(100vw - 1rem)` / `width:auto` overrides absent.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored, not emitted; live-region delegated to ToastStack which also lacks it.
- accepted: timer/store mechanism is platform-owned per contract §10 + Tier 3 — but the spec advertises `auto_dismiss_ms`/`sticky_tones` so behavior is currently a no-op, not a clean delegation.

## Jetstream gap (vs Svelte + contract)

- [ ] No store subscription / normalization — `js_toast_host` takes a pre-built `&ToastStackSpec` (toast_host.rs:11-15); `variant`-to-tone mapping, title fallback, `normalizeToast` all absent.
- [ ] No auto-dismiss timers / sticky enforcement — `spec.auto_dismiss_ms` and `spec.sticky_tones` never read; interaction loop (preview `main.rs`) would have to own this and currently does not.
- [ ] Missing `z-index: 80` — `container` (toast_host.rs:24) sets `.absolute()` only; no z-index.
- [ ] Hardcoded width `rem_to_px(28.0)` at toast_host.rs:22 — applies a flat 28rem with no `min(…, calc(100vw - 2rem))` clamp.
- [ ] Hardcoded inset `rem_to_px(1.0)` at toast_host.rs:21 — 1rem placement inset should come from a token/rem helper.
- [ ] No narrow-viewport responsive override (`max-width: 40rem`).
- [ ] `onDismiss`/`onAction` callbacks not wired — Svelte forwards `handleDismiss`/`handleAction` into the stack; `js_toast_host` passes no handlers, so dismiss/action are inert.
- accepted: no ARIA channel (documented across jetstream impls).
- accepted: interaction (timers, click handling) lives in preview event loop, not the component — but no timer logic exists there yet.

## Specimen parity

- Svelte covers: Runtime host with live store (success/warning/error seeds), "Add toast" button cycling info/success/warning/error, sticky error toast, auto-dismiss, fixed positioning in a relative surface (`ToastHostSpecimen.svelte`).
- GPUI covers: single bottom-end host with three static toasts (success/warning/danger-default) in a dashed surface. — missing: **placement variants** (only default), **"Add toast" interactivity** (button is decorative, no handler), **auto-dismiss/sticky demonstration**, **with-action toast** (contract §12 "With Action").
- Jetstream covers: Bottom-end (default) + Top-start placement groups, two static toasts each (success/warning). — missing: **With Action** toast (contract §12), **error/danger sticky** toast, any interactivity.

## Notes

- Both Rust hosts are visual-placement-only wrappers: they accept a ready `Vec<Toast>`/`ToastStackSpec` and skip the entire runtime layer (store subscription, timers, sticky, variant normalization) that is the contract's stated purpose (§1). This is the dominant `consv`/impl gap — the spec structs carry `auto_dismiss_ms`/`sticky_tones`/`is_sticky_tone()`/`auto_dismiss_enabled()` (toast_host.rs spec lines 40-68) but no implementation reads them, so they are dead surface today.
- `ToastHostStoreItem.variant` union in contract §3 lists `"danger"`, but Svelte's `resolveTone` never maps `variant="danger"` → only explicit `tone` or `variant="error"` does. Worth a contract clarification so consumers don't expect `variant:"danger"` to render as danger.
- Light theme: contract §8 says "None"; nothing to verify.
