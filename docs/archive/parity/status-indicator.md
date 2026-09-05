<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- pass: StatusIndicatorSpec gained size/size_role/density + size-aware dot/gap/label ladders (contract §8); both targets resolve the effective size via resolve_semantic_size. Jetstream render_probe tests cover tone variants, label, and size scaling; specimens add Sizes/Densities/Slot/dot-only(aria) groups. Pulse, GPUI aria-label, Jetstream dot-glow remain accepted runtime deltas (§12). -->
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

- [x] `info` resolves to **accent-base**, not `status-info` — FIXED at the source: added the `color.status.info` token (dark/light blue.500 `#2d86f3`, loophole `#6ea9d4`); `StatusTone::color_token` now maps `Info => COLOR_STATUS_INFO`. Both Rust targets + Svelte resolve the status-info blue (it was the gold accent in dark theme).
- [x] DONE — `size` / `sizeRole` / `density` support. `StatusIndicatorSpec` gained `size: Option<ControlSize>`, `size_role`, `density: Option<ControlDensity>`. The builder forwards `.size()`/`.size_role()`/`.density()`, and dot/gap/label metrics resolve from the effective size (`resolve_semantic_size`) + density via `dot_size_rem_for` / `gap_rem_for` / `label_font_size_rem_for` (contract §8 size + density tables).
- accepted: no pending pulse animation — contract §4/§8 `status-pulse`; GPUI motion is runtime-specific (Known Delta §12). Soft gap, tracked.
- accepted: `aria-label` not emitted — GPUI has no accessibility API to attach a name to an element. The contract's text path is satisfied by the visible `label`; the dot-only `aria_label` data is stored on the spec but cannot reach an a11y tree that does not exist. Runtime limit.

## Jetstream gap (vs Svelte + contract)

- [x] `info` resolves to **accent-base**, not `status-info` — FIXED via the shared `StatusTone::color_token` → `COLOR_STATUS_INFO` change + new token (see GPUI item above).
- [x] DONE — `size` / `sizeRole` / `density` support. `js_status_indicator` resolves the effective size (`spec.size` → `size_role` via `resolve_semantic_size`) and density, driving `dot_size_rem_for` / `gap_rem_for` / `label_font_size_rem_for` (contract §8 size + density tables).
- accepted: no dot box-shadow glow — `JsEl` exposes no box-shadow (Known Delta §12). Track until shadow support lands.
- accepted: no pending pulse animation — runtime motion delta (§12).
- accepted: no `aria_label` accessible-name path — Jetstream has no ARIA channel (§12). The visible `label` provides the §6 text path; dot-only `aria_label` is stored on the spec but has no a11y tree to reach.
- accepted: label line-height 1.3 not applied (`JsEl` text-metric limit per §12).
- accepted: non-interactive component — no event loop concern (contract §5 events = none).
- tests: `render_probe` covers the dot size at md, neutral→text-secondary, the five tone→status-token mappings, label text, and the xs/xl dot-size ladder.

## Specimen parity

- Svelte covers: All statuses (6), Without labels / dot-only (5, with `ariaLabel`), Child slot content, Inherited typography, Sizes snippet, Densities snippet (`StatusIndicatorSpecimen.svelte`).
- GPUI covers: All statuses (6), Without labels (4, via `aria_label`), Inherit typography, Slot content (`status_indicator.rs`). **Sizes/Densities groups still TODO in the GPUI preview specimen** — the GPUI component now supports size/density, but the preview specimen wasn't rebuilt this pass (shared gpui/preview target lock). Follow-up: add Sizes + Densities groups.
- Jetstream covers: All statuses (6), Without labels / dot-only (4, with `aria_label`), Slot content, **Sizes (xs–xl)**, **Densities (compact/default/comfortable)**, Inherit typography. Rebuilt this pass to the §13 specimen set. (`status_indicator.rs`)

## Notes

- `consv=fixed`: the §13 specimen row claiming `info` = accent-base (contradicting §4/§8 and authoritative Svelte) is corrected to status-info. The shared-spec bug (`StatusTone::color_token` mapping `Info => COLOR_ACCENT_BASE`, `packages/contracts/components/src/types.rs:280`) remains a code fix that clears the color todo on both Rust targets.
- `pending` correctly maps to accent-base in all four surfaces; only `info` is mis-mapped.
- Pending pulse and Jetstream dot glow are pre-approved Known Deltas (§12) but listed as open todos so they stay tracked until runtime support lands.
- No token-literal violations found in either Rust impl: dot/gap/label dimensions all resolve via `rem_to_px(spec.*_rem())` and colors via `resolve_color(theme, spec.*_token())`. The `px(999.0)` / `rounded(999.0)` radius is a full-circle sentinel matching the contract's `border-radius: 999px`, not a hardcoded design value.
