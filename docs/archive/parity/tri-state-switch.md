<!-- parity consv=fixed gpui=1 jetstream=2 specimen=ok | pass 41: canvas/black track recipe + 14/8/14 fills, density track-inset, contract min-content-width+x*2 segments, density segment padding, token shadow (gpui)/shadow_md (jet), typography-label tokens, TriStateValue surface; specimens custom-labels+custom-colors added -->
# Parity: TriStateSwitch

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/tri-state-switch.md`
- Svelte (authoritative): `packages/svelte/components/src/TriStateSwitch.svelte`
- GPUI: `packages/gpui/components/src/primitives/tri_state_switch.rs`
- Jetstream: `packages/jetstream/components/src/tri_state_switch.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/TriStateSwitchSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/tri_state_switch.rs` · jetstream `packages/jetstream/preview/src/specimens/tri_state_switch.rs`

## Contract ↔ Svelte

Svelte and the contract's exact-value table (§8) disagree on the track/fill recipe, and Svelte adds a min-width var the contract omits. Svelte is authoritative — update the contract.

- [x] FIXED **Track background mismatch.** Contract §8 root `background` was `color-mix(text-primary 18%, background-surface)`. Svelte uses `color-mix(canvas 75%, black)` (line 125), per-state tracks blend over `color-mix(canvas 70%, black)` (lines 110–112). Rewrote §8 root `background` + the three `--poodle-tri-state-*-track` rows to the canvas/black recipe.
- [x] FIXED **Track-fill ratios mismatch.** Contract §8 + Tier-2 said 18% / 88%-elevated / 18%. Svelte uses 14% / 8% / 14% over the canvas-black base (lines 110–112). Updated §8 mix percentages and the Tier-2 lines to 14/8/14.
- [x] FIXED **Selected border ratio mismatch.** Svelte tints excluded/included selection border `58%` toward the per-state color (lines 196, 206) and keeps default at `border-default` (line 201). Added a "Selection — per-state fill and border" subsection to §8 + a Tier-2 line.
- [x] FIXED **`min-width` var.** Svelte uses `--poodle-tri-state-min-content-width` per size (lines 116, 130–153) and segment `min-width = min-content-width + x*2` (line 236). Renamed the root var, added the per-size table (2.5 / 2.625 / 3 / 3.375 / 3.75 rem), and updated the segment `min-width` formula.
- [x] FIXED **Size scale.** Added the per-size `--poodle-tri-state-height` table (xs 1.5 / sm 1.75 / md 2.25 / lg 2.75 / xl 3.25 rem, lines 130–153) plus the density `x`/track-inset values to §8.
- Aligned: props (`value` default `"default"`, `options` record + defaults, `size`/`sizeRole` default `"control"`/`density`/`disabled`, `ariaLabel` required, per-state color overrides), anatomy (`radiogroup` → selection capsule + option/label → hidden radio + segment), `onValueChange` payload, focus ring (`border-width-focus` + accent-focusRing, offset `0.125rem`), disabled opacity, fixed excluded/default/included order. These all match.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] PARTIAL **`TriStateValue` surface added.** Added a real `TriStateValue` enum (excluded/default/included) to `types.rs` plus `value()`/`selected_index()` accessors on `TriStateSwitchSpec`. The component now matches on `spec.value()` (reads `Excluded`/`Default`/`Included`), not raw `CheckState`. Storage stays `state: CheckState` for backward compat with existing call sites + the `on_change(&CheckState)` callback signature (additive-only rule; full enum rename is a larger cross-cutting change deferred). The `track_fill_token()` accent-mapping helper (wrong for this component) was removed and replaced by per-state color + canvas/black track helpers.
- [x] FIXED **Segment widths.** Now `min-content-width + x*2` using a contract-exact tri-state min-content-width table (2.5/2.625/3/3.375/3.75 rem) + density `x` (`control_space_x_rem`); dropped the bespoke 3.5–5.5 ladder.
- [x] FIXED **Selection shadow.** Replaced `hsla` literals with `white@8%`/`black@18%` (via `gpui::white()`/`gpui::black()` with contract alphas) and contract-rem offsets (`rem_to_px(0.0625)` inset, `rem_to_px(0.125)`/`rem_to_px(0.5)` drop) — no raw px geometry.
- [x] FIXED **Segment padding.** Now density-driven `.px(rem_to_px(control_space_x_rem(density)))` (0.5/0.75/1 rem), replacing the fixed `0.875rem`.
- [x] FIXED **Track inset + recipe.** `track_padding` is density-driven (0.0625/0.125/0.1875 rem); root bg = `color-mix(canvas 75%, black)`, per-state fills = 14/8/14 over `color-mix(canvas 70%, black)` — matches the corrected contract §8.
- accepted: no native radiogroup — GPUI implements arrow/Space keyboard + per-segment focus explicitly (`tri_state_switch.rs:267-312`), the pre-approved §12 delta.
- accepted: no ARIA tree (gpui has no a11y API) — exclusive-choice semantics conveyed visually + via keyboard only.

## Jetstream gap (vs Svelte + contract)

- [x] PARTIAL **`TriStateValue` surface** (shared spec; see GPUI todo 1). `js_tri_state_switch` now matches on `spec.value()` (`TriStateValue`). Storage stays `CheckState` for compat — deferred full rename, additive-only.
- [x] FIXED **Min segment width.** Now `min-content-width + x*2` (contract-exact tri-state table + density `x`); dropped the `* 0.4` magic factor.
- [x] FIXED **Label size/weight from tokens.** `label_size`/`label_weight` now resolve from `typography.label.size` / `typography.label.weight` via `resolve_space_px(spec.label_size_token()/label_weight_token())` — no inlined `0.8125`/`500`.
- [ ] JsEl-gap **Selection capsule.** The immediate-mode runtime has no absolutely-positioned sliding capsule (segments are flex-row siblings), so the active segment paints its own selection fill + border + a preset `shadow_md` drop (closest to the contract's `0 0.125rem 0.5rem black@18%`; no inset highlight). Sliding `translateX` affordance + inset highlight are documented JsEl approximations.
- [x] FIXED **Selected border ratio.** Excluded/included selection border now `color-mix(state 58%, border-default)` (was `alpha * 0.38`), matching Svelte.
- [x] FIXED **Track inset.** Density inset now 0.0625/0.125/0.1875 rem (matches Svelte). `.rounded(999.0)` remains the max-pill sentinel (not token-derived) — accepted.
- accepted: interaction (segment click + arrow keys) lives in the preview event loop, not the component (no `on_change` wiring inside `js_tri_state_switch`).
- accepted: no ARIA / radiogroup (immediate-mode runtime; pre-approved §12 delta).

## Specimen parity

- Svelte covers: Default (interactive, live value readout), Custom labels, Disabled — `TriStateSwitchSpecimen.svelte` (per contract §13).
- GPUI covers: Default (interactive + value readout), Custom labels, Semantic sizes (xs–xl), Chrome-vs-prominent role offset, Disabled, **Custom semantic colors** (`#ef4444`/`#64748b`/`#22c55e`) — `gpui/.../tri_state_switch.rs`. Broader than Svelte; at/above parity.
- Jetstream covers: States (all three), Sizes (sm/md/lg), **Custom labels** (Hide/All/Show), **Custom semantic colors** (#ef4444/#64748b/#22c55e), Disabled — `jetstream/.../tri_state_switch.rs`. Custom-labels + custom-colors groups added this pass → specimen=ok. The interactive live-value readout stays absent (no `onValueChange` wiring in static specimens — preview-loop). Existing groups still use the non-contract `label` caption prop on a couple of rows.

## Notes

- The `label`/`with_label` field on `TriStateSwitchSpec` (`tri_state_switch.rs:9`, used as a trailing caption in both Rust impls) is **not in the contract or Svelte**. Either add it to the contract as a Rust-runtime affordance or drop it — currently it is undocumented surface used only to caption specimens.
- Root cause of most `consv` churn: the contract §8 exact-value table was written before the Svelte canvas/black track recipe landed. Per "Svelte is parity authority," the contract should be updated to the 75%-canvas/black background and 14/8/14 fill ratios — not the other way round.
- The shared-spec `CheckState` aliasing is now partly addressed: a `TriStateValue` enum + `value()`/`selected_index()` accessors give the component its real ternary surface, and the wrong `track_fill_token()` accent helper was removed in favor of per-state color tokens (`excluded/default/included_color_token`) + canvas/black track helpers (`root_bg_token`, `track_base_token`). Storage stays `state: CheckState` and `on_change(&CheckState)` for backward compat (additive-only rule); a full rename of the stored field + callback is the remaining follow-up. The adapter `RenderComponent` impls (gpui + jetstream `render_selection.rs`) were repointed from `track_fill_token()` to `root_bg_token()`.
- Spec additions this pass (all additive): `value()`, `selected_index()`, `excluded_color_token()`, `default_color_token()`, `included_color_token()`, `track_base_token()`, `root_bg_token()`, `border_token()`, `focus_ring_color_token()`, `unselected_text_token()`, `disabled_opacity_token()`, `label_size_token()`, `label_weight_token()`.
