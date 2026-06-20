<!-- parity consv=fixed gpui=6 jetstream=5 specimen=gap -->
# Parity: Surface

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/surface.md` (+ `docs/contracts/components/surface-elevation.md`)
- Svelte (authoritative): `packages/svelte/components/src/Surface.svelte`
- GPUI: `packages/gpui/components/src/primitives/surface.rs`
- Jetstream: `packages/jetstream/components/src/surface.rs`
- Spec: `packages/contracts/components/src/surface.rs` · `packages/contracts/components/src/types.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SurfaceSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/surface.rs` · jetstream `packages/jetstream/preview/src/specimens/surface.rs`

## Contract ↔ Svelte

Svelte adds treatment-override layers the contract §8 token tables do not document. Svelte is authoritative — update the contract.

- [x] FIXED Svelte wraps every surface CSS var in a `--poodle-treatment-*` override fallback: `--poodle-treatment-surface-fill` (line 53), `-border` (58), `-shadow` (61), `-elevated-fill` (77), `-elevated-border` (81), `-elevated-shadow` (85). Added the six missing treatment override tokens to contract §8 (new treatment-token table + wrapped var defaults) and §9 Svelte Notes (full seven-token list incl. existing radius token).
- [x] FIXED Svelte's elevated selector (lines 74–88) re-sets `--poodle-surface-border` to `color-mix(border-subtle 74%, transparent)`. Added the `--poodle-surface-border` row to both §8 elevated tables (tone + elevated override).
- [x] FIXED Contract §8 base table showed `--poodle-surface-shadow: none` flat; now `var(--poodle-treatment-surface-shadow, none)`. Fill + border defaults likewise wrapped in their treatment fallbacks.
- **CODE (Rust spec, out of scope for contract):** `surface.rs::resolved_shadow_token()` returns `ELEVATION_OVERLAY` when elevated (line 77); contract §8 elevated tables correctly target `--poodle-elevation-surface` per Svelte. Reconcile the spec to `ELEVATION_SURFACE` in code. Contract is correct (no edit).

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Elevated shadow is fully hardcoded: `hsla(0.0,0.0,0.0,0.08)` + `offset point(px(0.0),px(2.0))` + `blur px(8.0)` at `surface.rs:139-144`, and `hsla(0.0,0.0,0.0,0.04)` + `offset px(0.0),px(1.0)` + `blur px(2.0)` at `surface.rs:145-150`. Contract §8 elevated targets `--poodle-elevation-surface`; resolve `spec.resolved_shadow_token()`, do not invent a two-layer drop shadow.
- [ ] Non-elevated inset shadow ring is hardcoded: `border_subtle.a * 0.18` magic alpha + `spread_radius px(1.0)` at `surface.rs:154-162`. The contract base shadow is `none` — this inset ring is an undocumented GPUI invention with a raw `0.18` literal. Resolve from a token or remove.
- [ ] Background mix ratios hardcoded: `color_mix(elevated_bg, panel, 0.96)` at `surface.rs:98` and `surface_bg.a * 0.96` at `surface.rs:100`. The `0.96` color-mix percentage is a raw literal; per surface-elevation contract the mix ratio is "tokenised constant". Resolve the ratio from a token.
- [ ] Border subtle alpha hardcoded: `border_subtle.a * 0.74` at `surface.rs:121-123`. The `0.74` color-mix percentage (Svelte line 59) is a raw literal — tokenise it.
- [ ] Border width hardcoded to `border_1()` at `surface.rs:130`; contract border is `0.0625rem` via `spec.resolved_border_width()` (`BORDER_WIDTH_DEFAULT`). Resolve the width token, do not assume 1px.
- [ ] No `canvas` tone branch — `is_elevated` is the only tone test (`surface.rs:85,97`); `SurfaceTone::Canvas` falls into the non-elevated path and renders identical to panel. Contract §8 canvas fill (`color-mix(canvas 98%, transparent)`) is never applied.
- accepted: no ARIA — `spec.role`/`spec.label` builders exist (`surface.rs:59-66`) but `asRole`/`aria-label` are never emitted (gpui has no accessibility API). Contract §10 requires platform-a11y mapping; tracked as accepted runtime limit.

## Jetstream gap (vs Svelte + contract)

- [ ] No elevation shadow at all — `js_surface` never reads `spec.resolved_shadow_token()` and emits no shadow (`surface.rs:9-36`). Contract §8 elevated requires `box-shadow: var(--poodle-elevation-surface)`; elevated and panel render identically. (grep: no `shadow` in `surface.rs`.)
- [ ] No color-mix transparency blending — fill is raw `resolve_color(theme, spec.resolved_background_token())` at `surface.rs:10`. Contract §8 every tone uses `color-mix(... %, transparent)`; canvas (98%), panel/surface (96%), elevated (96% over panel) are all flattened to opaque token colors. Apply the mix ratios.
- [ ] Background token mismatch: spec `resolved_background_token()` returns `COLOR_BACKGROUND_SURFACE`-free path — panel maps to `COLOR_BACKGROUND_PANEL` (`types.rs:364`), but contract base fill mixes `--poodle-color-background-surface` (Svelte line 54). Panel tone uses the wrong base color token. Reconcile spec to surface vs panel per contract.
- [ ] Border width hardcoded to `border(1.0)` literal at `surface.rs:18`; resolve `spec.resolved_border_width()` (`BORDER_WIDTH_DEFAULT` → `0.0625rem`) instead of the raw `1.0`.
- [ ] No `asRole`/`label` channel — `spec.role`/`spec.label` unused; no region/group semantics. (Accepted-style for native, but contract §10 + surface-elevation §5 require contrast/semantic intent preserved.)
- accepted: no ARIA channel (native renderer, documented pattern across components).

Note: `surface.rs` itself is clean of `.h([0-9]`, `.w([0-9]`, `text_size([0-9]`, `hsla(`, `rgb(` literals — the violations are *missing* token resolution (shadow, color-mix, border width), not stray pixel constants.

## Specimen parity

- Svelte covers: Panel tone (default), Canvas tone, Elevated tone, No border (`SurfaceSpecimen.svelte`, 4 groups — matches contract §13 exactly).
- GPUI covers: Panel, Canvas, Elevated, No border (`gpui/.../surface.rs`, all 4 contract groups). — missing: nothing structural; visual parity blocked by the tone/shadow gaps above.
- Jetstream covers: Panel tone, Elevated tone, "With border" (`SurfaceBorder::Default`) (`jetstream/.../surface.rs`, 3 groups). — missing: **Canvas tone** group, **No border** group. Also substitutes a non-contract "With border" group; specimens omit `padding`/`border` args so they render default `padding="md"`, `border="subtle"` regardless of label.

## Notes

- `surface-elevation.md` is the cross-cutting elevation contract, not a second component. It mandates: creators set `--poodle-surface` (Svelte does, line 56), consumers derive contrast from a **tokenised mix ratio** (§3, §6). The GPUI `0.96`/`0.74`/`0.18` literals and Jetstream's absent blending both violate this "mix ratio = tokenised constant" requirement — that contract is the authority for tokenising those magic numbers.
- Svelte propagates `--poodle-surface: var(--poodle-surface-fill)` (line 56) so nested surfaces self-contrast. Neither Rust target propagates a surface-context value to descendants — the surface-elevation creator contract (§4 nested creator) is unimplemented in GPUI and Jetstream. Out of scope for this audit's todo count but flagged for elevation hardening.
- Contract §12 Known Deltas already permits color-mix→direct-alpha and treatment-radius fallback divergence. The hardcoded *ratios* are not covered by that delta — only the blending *mechanism* is.
- `consv=fixed`: the undocumented Svelte treatment-override tokens (six) are now in contract §8 + §9, both elevated tables carry the border var, and base vars show their treatment wrappers. Also corrected a stale §9 note (padding uses Surface-local `surfacePadding`, not shared `scaleToSpace`). The remaining implementation gap is elevation shadow + color-mix blending missing or hardcoded in both Rust targets — code, out of scope.
