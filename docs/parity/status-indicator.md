<!-- parity consv=fixed gpui=4 jetstream=5 specimen=gap -->
# Parity: Status Indicator

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/status-indicator.md`
- Svelte (authoritative): `packages/svelte/components/src/StatusIndicator.svelte`
- GPUI: `packages/gpui/components/src/primitives/status_indicator.rs`
- Jetstream: `packages/jetstream/components/src/status_indicator.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/StatusIndicatorSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/status_indicator.rs` · jetstream `packages/jetstream/preview/src/specimens/status_indicator.rs`

## Contract ↔ Svelte

Contract and Svelte mostly agree on the prop surface. The live divergence is an internal contradiction in the contract over the `info` color, and the shared Rust spec encodes the wrong side of it.

- [x] FIXED **`info` color contradiction.** Contract §4 + §8 already mapped `info` → `--poodle-color-status-info` (fallback `#3b82f6`), matching Svelte (`StatusIndicator.svelte:66-68`). Only the §13 specimen row said "Info | Dot in **accent-base** color" — corrected to "status-info color". The shared Rust spec `StatusTone::color_token` still follows the wrong row (`Info => COLOR_ACCENT_BASE`, `types.rs:280`) — code fix tracked in GPUI/Jetstream todos, out of scope for contract.
- `size` / `sizeRole` / `density` props: contract §3 lists `size`, `sizeRole` (default `"control"`), `density`. Svelte implements all three and reflects `data-size` / `data-density` on the root (`StatusIndicator.svelte:31-39`), with full per-size and per-density CSS (`:107-196`). Aligned. **No fix.**
- `typography` inherit-mode metrics: Svelte emits the per-size `em` table for `inherit` (`StatusIndicator.svelte:141-174`) matching contract §8 (`:199-205`). Aligned.
- Anatomy: Svelte root `<span>` + dot `<span aria-hidden>` + optional label `<span>` (`StatusIndicator.svelte:35-49`) matches contract §2 exactly. Dot is correctly `aria-hidden` (not spelled out in contract but consistent with "color alone must never be the only signal"). Aligned.
- ARIA: Svelte applies `aria-label={ariaLabel ?? undefined}` and `data-status` on root (`StatusIndicator.svelte:37,41`) per contract §6. Aligned.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done.

- [ ] `info` resolves to **accent-base**, not `status-info`. `spec.status_color_token()` (`status_indicator.rs:64`) delegates to `StatusTone::color_token`, which maps `Info | Pending => COLOR_ACCENT_BASE` (`packages/contracts/components/src/types.rs:280`). Contract §4/§8 + Svelte require `--poodle-color-status-info`. **Fix in the shared spec; affects both Rust targets.**
- [ ] No `size` / `sizeRole` / `density` support. Builder only forwards `status`/`label`/`aria_label`/`typography` (`status_indicator.rs:39-54`); spec struct has no size/density fields (`packages/contracts/components/src/status_indicator.rs:5-10`). Dot/gap/label metrics are fixed at the `md` preset regardless of requested size — contract §8 size table (`:189-195`) unimplemented.
- [ ] No pending pulse animation. Contract §4 (`:63`) + §8 (`:152-167`) specify `status-pulse` on `status="pending"`; GPUI renders the pending dot statically (`status_indicator.rs:75-92`, no animation branch). Accepted-but-track per Known Delta §12 — motion internals are runtime-specific, so this is a soft gap.
- [ ] `aria-label` not emitted. `aria_label` is stored on the spec and settable (`status_indicator.rs:47-50`) but never applied to the element (`into_element` ignores it, `:60-107`), so dot-only indicators have no accessible name — violates contract §6 "not color-only". Distinct from the blanket no-ARIA delta because the contract explicitly requires a text/label path (§6, §10) and the data exists.
- accepted: no ARIA roles/attributes channel (gpui has no accessibility API) — but the labeling path above is a real gap the contract calls out.

## Jetstream gap (vs Svelte + contract)

- [ ] `info` resolves to **accent-base**, not `status-info` — same shared-spec root cause (`status_indicator.rs:30` → `StatusTone::color_token`, `packages/contracts/components/src/types.rs:280`).
- [ ] No `size` / `sizeRole` / `density` support — spec lacks the fields; `js_status_indicator` uses only `dot_size_rem`/`gap_rem`/`label_font_size_rem` md presets (`status_indicator.rs:34-36`). Contract §8 size/density tables unimplemented.
- [ ] No dot box-shadow glow. Contract §8 (`:150`) + Svelte (`StatusIndicator.svelte:92`) require `0 0 0 0.125rem` ring at 18% — `js_status_indicator` builds the dot with no shadow (`status_indicator.rs:39-43`). Documented runtime delta (§12, `JsEl` lacks box-shadow), so soft gap — track until shadow support lands.
- [ ] No pending pulse animation (`status="pending"` renders static; no animation branch in `status_indicator.rs`). Soft gap per §12.
- [ ] No `aria_label` / accessible-name path — `js_status_indicator` reads `spec.label` only (`status_indicator.rs:56`), never `spec.aria_label`; dot-only indicators carry no text meaning. Contract §6/§10 require a perceivable status path.
- accepted: no ARIA channel (documented runtime limit); label line-height 1.3 not applied (`status_indicator.rs:62`, `JsEl` text-metric limit per §12).
- accepted: non-interactive component — no event loop concern (contract §5 events = none).

## Specimen parity

- Svelte covers: All statuses (6), Without labels / dot-only (5, with `ariaLabel`), Child slot content, Inherited typography, Sizes snippet, Densities snippet (`StatusIndicatorSpecimen.svelte`).
- GPUI covers: All statuses (6), Without labels (4, via `aria_label`), Inherit typography, Slot content (`status_indicator.rs`). — missing: **Sizes group**, **Densities group** (contract §13 + Svelte demonstrate size/density; GPUI omits, consistent with the unimplemented props).
- Jetstream covers: Tones (6), Without label (3), Inherit typography (`status_indicator.rs`). — missing: **Slot/child content** group, **Sizes** group, **Densities** group; dot-only row has 3 vs Svelte's 5 and sets no `aria_label`.

## Notes

- `consv=fixed`: the §13 specimen row claiming `info` = accent-base (contradicting §4/§8 and authoritative Svelte) is corrected to status-info. The shared-spec bug (`StatusTone::color_token` mapping `Info => COLOR_ACCENT_BASE`, `packages/contracts/components/src/types.rs:280`) remains a code fix that clears the color todo on both Rust targets.
- `pending` correctly maps to accent-base in all four surfaces; only `info` is mis-mapped.
- Pending pulse and Jetstream dot glow are pre-approved Known Deltas (§12) but listed as open todos so they stay tracked until runtime support lands.
- No token-literal violations found in either Rust impl: dot/gap/label dimensions all resolve via `rem_to_px(spec.*_rem())` and colors via `resolve_color(theme, spec.*_token())`. The `px(999.0)` / `rounded(999.0)` radius is a full-circle sentinel matching the contract's `border-radius: 999px`, not a hardcoded design value.
