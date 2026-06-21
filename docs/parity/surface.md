<!-- parity consv=fixed gpui=1 jetstream=1 specimen=gap | pass: GPUI+Jetstream fill/border/shadow now token-resolved (SurfaceSpec mix-ratio methods added); only the accepted no-ARIA limit remains per target -->
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

- [x] FIXED Elevated shadow now resolves `elevation.surface` via `elevation_surface_shadow()` (the token-resolved drop shadow), and the base shadow is omitted (contract base = `none`). The old hardcoded `hsla(…)` two-layer shadow and the undocumented non-elevated inset ring (raw `0.18` alpha + `spread px(1.0)`) are both removed.
- [x] FIXED Background mix ratios are spec-resolved: fill base color (`spec.resolved_background_token()`), second color (`spec.fill_mix_over_token()`), and ratio (`spec.fill_mix_ratio()` → 0.98 canvas / 0.96 panel+elevated) all come from `SurfaceSpec`. No raw `0.96` literal in `surface.rs`. (The percentages are CSS color-mix percents centralized on the spec — see Notes; not a `--poodle-*` token.)
- [x] FIXED Border subtle alpha is spec-resolved via `spec.border_mix_ratio()` (0.74 subtle, 1.0 default). No raw `0.74` literal in `surface.rs`.
- [x] FIXED Border width resolves `spec.resolved_border_width()` (`BORDER_WIDTH_DEFAULT` → 0.0625rem) through `resolve_px`; the `border_1()` 1px assumption is gone.
- [x] FIXED Canvas tone now has a real fill: `resolved_background_token()` returns `color.background.canvas` for canvas (and `color.background.surface` for panel/base, `color.background.elevated` for elevated) with the per-tone ratio, so canvas no longer renders identical to panel.
- accepted: no ARIA — `spec.role`/`spec.label` builders exist but `asRole`/`aria-label` are never emitted (gpui has no accessibility API). Contract §10 requires platform-a11y mapping; tracked as accepted runtime limit.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Elevation shadow: elevated surfaces now emit `.shadow_sm()`. JsEl exposes only preset box-shadows (no custom-color / multi-layer), so the token `elevation.surface` (the low tier) is APPROXIMATED with the small preset — elevated and panel no longer render identically. (Approximation noted inline.)
- [x] FIXED color-mix transparency blending: fill uses `base_fill.mix(over, ratio)` for elevated (over panel) and `with_alpha(a * ratio)` for canvas (98%) / panel (96%) — all ratios from `SurfaceSpec`. No flat opaque fills.
- [x] FIXED Background token: `resolved_background_token()` now returns `color.background.surface` for panel/base (was `color.background.panel`), `color.background.canvas` for canvas, `color.background.elevated` for elevated — matching contract §8 / Svelte. The `lib.rs` contracts test that asserted the old overlay-shadow path was updated to `ELEVATION_SURFACE`.
- [x] FIXED Border width resolves `spec.resolved_border_width()` via `resolve_px`; the raw `border(1.0)` literal is gone.
- [ ] No `asRole`/`label` channel — `spec.role`/`spec.label` unused; no region/group semantics. (Accepted-style for native, but contract §10 + surface-elevation §5 require contrast/semantic intent preserved.)
- accepted: no ARIA channel (native renderer, documented pattern across components).

Note: `surface.rs` is clean of pixel/`hsla`/`rgb` literals; the closed gaps were *missing* token resolution (shadow, color-mix, border width), now resolved from `SurfaceSpec`. The one remaining open item is the `asRole`/`label` semantic channel (no native a11y surface).

## Specimen parity

- Svelte covers: Panel tone (default), Canvas tone, Elevated tone, No border (`SurfaceSpecimen.svelte`, 4 groups — matches contract §13 exactly).
- GPUI covers: Panel, Canvas, Elevated, No border (`gpui/.../surface.rs`, all 4 contract groups). — missing: nothing structural; visual parity blocked by the tone/shadow gaps above.
- Jetstream covers: Panel tone, Elevated tone, "With border" (`SurfaceBorder::Default`) (`jetstream/.../surface.rs`, 3 groups). — missing: **Canvas tone** group, **No border** group. Also substitutes a non-contract "With border" group; specimens omit `padding`/`border` args so they render default `padding="md"`, `border="subtle"` regardless of label.

## Notes

- `surface-elevation.md` is the cross-cutting elevation contract, not a second component. It mandates: creators set `--poodle-surface` (Svelte does, line 56), consumers derive contrast from a **tokenised mix ratio** (§3, §6). The GPUI `0.96`/`0.74`/`0.18` literals and Jetstream's absent blending both violate this "mix ratio = tokenised constant" requirement — that contract is the authority for tokenising those magic numbers.
- Svelte propagates `--poodle-surface: var(--poodle-surface-fill)` (line 56) so nested surfaces self-contrast. Neither Rust target propagates a surface-context value to descendants — the surface-elevation creator contract (§4 nested creator) is unimplemented in GPUI and Jetstream. Out of scope for this audit's todo count but flagged for elevation hardening.
- Contract §12 Known Deltas already permits color-mix→direct-alpha and treatment-radius fallback divergence. The hardcoded *ratios* are not covered by that delta — only the blending *mechanism* is.
- `consv=fixed`: the undocumented Svelte treatment-override tokens (six) are now in contract §8 + §9, both elevated tables carry the border var, and base vars show their treatment wrappers. Also corrected a stale §9 note (padding uses Surface-local `surfacePadding`, not shared `scaleToSpace`). The remaining implementation gap is elevation shadow + color-mix blending missing or hardcoded in both Rust targets — code, out of scope.
