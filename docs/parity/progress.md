<!-- parity consv=fixed gpui=0 jetstream=1 specimen=gap -->
<!-- pass: ProgressSpec gained track_fill/track_mix/track_mix_ratio + indicator_gradient_accent_ratio + min_height_rem(size); GPUI now renders the contract accent gradient + spec-owned track mix + size-driven height; Jetstream renders spec-owned track mix + size height ladder + indeterminate 40% bar (gradient on indeterminate bar; determinate widget fill stays solid — runtime quad limit). Probe tests added. -->
# Parity: Progress

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/progress.md`
- Svelte (authoritative): `packages/svelte/components/src/Progress.svelte`
- GPUI: `packages/gpui/components/src/primitives/progress.rs`
- Jetstream: `packages/jetstream/components/src/progress.rs`
- Spec: `packages/contracts/components/src/progress.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ProgressSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/progress.rs` · jetstream `packages/jetstream/preview/src/specimens/progress.rs`

## Contract ↔ Svelte

Props/anatomy/ARIA largely agree. Divergences are in token mix ratios and one internal contradiction in the contract.

- FIXED — **Track background mix three-way mismatch.** Contract §8 Root table already had `color-mix(--poodle-surface 96%, --poodle-color-text-primary)`; the §8 Token Reference and §11 Tier-2 lines said `--poodle-color-background-surface` at 92%. Svelte (`Progress.svelte:60`) is `--poodle-surface` 96% with `--poodle-color-text-primary`. Contract Token Reference and Tier-2 lines corrected to `--poodle-surface` 96% with `text-primary`.
- **Contract §6/§8 contradicts itself on `aria-valuemax` when indeterminate.** §6 says all three aria-value attrs omitted when indeterminate — Svelte matches (`Progress.svelte:43-44` gate every attr on `!indeterminate`). No fix needed; just flagging the §6 line "no aria-valuemin/valuemax/valuenow" is correct and Svelte-aligned.
- FIXED — **`computedValueText` Svelte behavior absent from contract.** Svelte derives a fallback `aria-valuetext` of `"{round(pct*100)}%"` when determinate and no explicit `valueText` (`Progress.svelte:30-32,45`). Documented as `computedValueText` in contract §3 Computed Values and folded into the §6 Optional-attributes `aria-valuetext` rule.
- **Indicator is `<span>` in contract anatomy (§2) and Svelte (`Progress.svelte:47`)** — agree. Note: both Rust impls render the indicator as a `div`, not a span (no semantic impact in Rust, but anatomy-label drift).
- `value` clamp / `safeMax` / `percentage` formulas (§3) match Svelte (`Progress.svelte:27-29`) and the spec's `normalized_progress()` (`progress.rs:46-58`). OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] **Indicator is a solid accent fill, not the contract gradient.** FIXED: indicator now fills `gpui::linear_gradient(90deg, color-mix(accent 88%, white), accent)` via `mix_white(accent, spec.indicator_gradient_accent_ratio())`. Ratio (0.88) is a spec method.
- [x] **Determinate height via inline match.** FIXED: `bar_height` resolves from `ProgressSpec::min_height_rem(effective_size)` (size→height ladder owned by the spec; xs/sm 0.375, md 0.5, lg/xl 0.75rem). Width-vs-transform fill remains an allowed Tier-3 choice.
- [x] **Track bg reassembled from raw lookups.** FIXED: track mix is spec-owned — `color_mix(resolve(track_fill_token), resolve(track_mix_token), track_mix_ratio())`. No raw string lookups in the component.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`value_text`/`value`/`max` stored on spec but no `role="progressbar"`/aria-value emission. Contract §6 + §10 ask for native a11y mapping; not wired.
- accepted: indeterminate animation timing differs (1.2s `ease_in_out`, slide -40%→140%) — contract §12 Known Delta permits this. Matches intent.

## Jetstream gap (vs Svelte + contract)

- [x] **Determinate fill is broken — every value renders a full bar.** FIXED: determinate now renders the runtime `ui_element::progress(frac)` ProgressBar widget, which fills proportionally (JsEl has no percent sizing, so the hand-built child could only ever be 100%). Verified by `determinate_renders_progressbar_widget` via the render_probe harness.
- [x] **Track background wrong color AND wrong mix.** FIXED: now `color_mix(resolve(track_fill_token /*surface*/), resolve(track_mix_token /*text-primary*/), track_mix_ratio() /*0.96*/)` — mirrors GPUI's sRGB mix (Jetstream mixes in linear space, a minor documented cross-target delta). No more `tint(surface, 0.80)`. Verified by `track_background_is_spec_resolved_mix`.
- [x] **Track height ignores effective size.** FIXED: `track_height = rem_to_px(ProgressSpec::min_height_rem(effective_size))` — size ladder owned by the spec, identical to GPUI. Verified by `track_height_follows_size_ladder`.
- [x] **Indeterminate renders a static full-width accent bar.** FIXED: indeterminate now renders a 40%-width accent bar via a flex row (bar `flex_grow 0.4` + trailing spacer `0.6`), so it is distinguishable from a complete determinate bar and the bar carries the contract accent gradient. Continuous slide animation remains a §12 runtime delta. Verified by `indeterminate_renders_partial_width_bar`.
- preview-loop: indeterminate slide animation (`translateX(-100%→250%)`, 1.2s) lives in the runtime loop, not the static component (§12 Known Delta).
- accepted: **determinate fill is solid accent, not the contract gradient.** The runtime `ProgressBar` widget shares one GPU quad between the `background_gradient` channel and the fill-fraction, so forcing a gradient would recolor the whole track rather than just the filled portion. The gradient IS applied on the (hand-built) indeterminate bar where it paints correctly. Spec exposes `indicator_gradient_accent_ratio()` for both targets; GPUI honors it on determinate too.
- accepted: no ARIA channel (`role`/aria-value) — no accessibility surface in Jetstream.
- accepted: continuous indeterminate animation lives in the runtime, not the component (§12 Known Delta) — but see static-width todo above.

## Specimen parity

- Svelte covers: Determinate (0 / 35 / 72 / 100), Indeterminate, Custom max (3 of 5 + `valueText`), Sizes row (xs–xl at value 60) — densities off. (`ProgressSpecimen.svelte`)
- GPUI covers: Determinate (0 / 35 / 72 / 100), Indeterminate, Custom max (3/5), Sizes row (value 60). — missing: `valueText` is not exercised (custom-max spec sets only value+max, `progress.rs:68-75`); otherwise full parity.
- Jetstream covers: Determinate (0.25 / 0.50 / 0.75 / 1.0), Indeterminate. — missing: **Custom max** group, **Sizes** row, **`valueText`**. Determinate values are pre-normalized fractions (0.25…1.0) instead of contract values (0/35/72/100 of max 100), so values diverge from Svelte and the contract specimen table (§13). And per the broken-fill bug above, all four render identically anyway.

## Notes

- Spec gaps driving multiple todos: `ProgressSpec` (`packages/contracts/components/src/progress.rs`) exposes only `indicator_fill_token()` (solid accent). It lacks (a) a `track_fill_token()` / track-mix method — both impls hand-assemble the mix from raw `"color.background.surface"` + `"color.text.primary"` string lookups; (b) an indicator-gradient method — so neither impl can render the contract gradient; (c) a size→min-height method — so height is inlined in GPUI and hardcoded in Jetstream. Add all three to the spec to close the GPUI and most Jetstream todos at the source.
- `consv=fixed`: the §8 Token Reference + §11 Tier-2 track-bg contradiction (`background-surface` 92% vs the §8 Root table / Svelte `surface` 96% with `text-primary`) is resolved in favour of Svelte; `computedValueText` fallback now documented. No Svelte change needed.
- The `rounded(px(999.0))` / `rounded(999.0)` literals in both impls are the contract's literal pill radius (§8 `border-radius: 999px`), not token violations — left as-is.
