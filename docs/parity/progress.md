<!-- parity consv=gap gpui=3 jetstream=4 specimen=gap -->
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

- **Track background mix is a three-way mismatch.** Contract §8 Root table: `color-mix(in srgb, var(--poodle-surface) 96%, var(--poodle-color-text-primary))`. Contract §8 Token Reference table + §11 Tier 2: `--poodle-color-background-surface` at **92%** mix. Svelte (`Progress.svelte:60`): `--poodle-surface` **96%** with `--poodle-color-text-primary`. Svelte is authoritative. **Fix: correct contract Token Reference (line 179) and Tier-2 line (line 208) to `--poodle-surface` 96% with `text-primary`, not `background-surface` 92%.**
- **Contract §6/§8 contradicts itself on `aria-valuemax` when indeterminate.** §6 says all three aria-value attrs omitted when indeterminate — Svelte matches (`Progress.svelte:43-44` gate every attr on `!indeterminate`). No fix needed; just flagging the §6 line "no aria-valuemin/valuemax/valuenow" is correct and Svelte-aligned.
- **`computedValueText` is a Svelte behavior absent from contract.** Svelte derives a fallback `aria-valuetext` of `"{round(pct*100)}%"` when determinate and no explicit `valueText` (`Progress.svelte:30-32,45`). Contract §3 lists `valueText` but never documents the computed `%` fallback. **Fix: document the computed `valueText` fallback in contract §3 Computed Values + §6 Optional attributes.**
- **Indicator is `<span>` in contract anatomy (§2) and Svelte (`Progress.svelte:47`)** — agree. Note: both Rust impls render the indicator as a `div`, not a span (no semantic impact in Rust, but anatomy-label drift).
- `value` clamp / `safeMax` / `percentage` formulas (§3) match Svelte (`Progress.svelte:27-29`) and the spec's `normalized_progress()` (`progress.rs:46-58`). OK.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] **Indicator is a solid accent fill, not the contract gradient.** Contract §8 indicator background = `linear-gradient(90deg, color-mix(accent 88%, white), accent)`; GPUI fills flat `accent` (`progress.rs:91,116,127`). No gradient surface on the spec — add an `indicator_gradient_*` token method, then apply it.
- [ ] **Determinate fill uses width, not `scaleX`.** Contract §8 + §11 Tier-3 allow width-vs-transform freedom, but the size token resolution is bypassed: `bar_height` is hardcoded via inline `match` on size (`progress.rs:85-89`) instead of a `ProgressSpec` height token method. Add a size→min-height token method on the spec and resolve from it.
- [ ] **Track bg mix ratio is correct (0.96) but `track_bg` reassembles the mix from two raw token lookups** (`progress.rs:92-95`) instead of a single `track_fill_token()` on the spec. Add `ProgressSpec::track_fill_token()` so the mix is spec-owned, not component-owned.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label`/`value_text`/`value`/`max` stored on spec but no `role="progressbar"`/aria-value emission. Contract §6 + §10 ask for native a11y mapping; not wired.
- accepted: indeterminate animation timing differs (1.2s `ease_in_out`, slide -40%→140%) — contract §12 Known Delta permits this. Matches intent.

## Jetstream gap (vs Svelte + contract)

- [x] **Determinate fill is broken — every value renders a full bar.** FIXED: determinate now renders the runtime `ui_element::progress(frac)` ProgressBar widget, which fills proportionally (JsEl has no percent sizing, so the hand-built child could only ever be 100%). Verified by `determinate_renders_progressbar_widget` via the render_probe harness.
- [ ] **Track background is wrong color AND wrong mix.** Contract/Svelte: `color-mix(surface 96%, text-primary)`. Jetstream: `tint(surface, 0.80)` (`progress.rs:38`), which only multiplies surface alpha by 0.8 (`theme_ext.rs:29-31`) — it never mixes toward `text-primary`, producing a translucent surface instead of the contrast track. Header comment also mislabels it "color-mix(surface 80%, elevated)" (`progress.rs:27,37`). **Fix: resolve `text-primary` and color-mix at 96%, mirroring GPUI `color_mix`.**
- [ ] **Indicator is solid accent, not the contract gradient** (`progress.rs:60-63,68-73`) — same gap as GPUI; needs an indicator-gradient token.
- [ ] **Track height ignores effective size.** `track_height = rem_to_px(0.5)` hardcodes the `md` value (`progress.rs:41`); `_effective_size` is computed then discarded (`progress.rs:33`). xs/sm (0.375rem) and lg/xl (0.75rem) all render at 8px. **Fix: resolve height from effective size like GPUI's match.**
- [ ] **Indeterminate renders a static full-width accent bar** (`progress.rs:65-74`) — no width-40% and no animation, so it is visually identical to a completed determinate bar. Animation is an accepted runtime limit, but the missing `width: 40%` static treatment is not — apply the 40% width so it is at least distinguishable.
- accepted: no ARIA channel (`role`/aria-value) — no accessibility surface in Jetstream.
- accepted: continuous indeterminate animation lives in the runtime, not the component (§12 Known Delta) — but see static-width todo above.

## Specimen parity

- Svelte covers: Determinate (0 / 35 / 72 / 100), Indeterminate, Custom max (3 of 5 + `valueText`), Sizes row (xs–xl at value 60) — densities off. (`ProgressSpecimen.svelte`)
- GPUI covers: Determinate (0 / 35 / 72 / 100), Indeterminate, Custom max (3/5), Sizes row (value 60). — missing: `valueText` is not exercised (custom-max spec sets only value+max, `progress.rs:68-75`); otherwise full parity.
- Jetstream covers: Determinate (0.25 / 0.50 / 0.75 / 1.0), Indeterminate. — missing: **Custom max** group, **Sizes** row, **`valueText`**. Determinate values are pre-normalized fractions (0.25…1.0) instead of contract values (0/35/72/100 of max 100), so values diverge from Svelte and the contract specimen table (§13). And per the broken-fill bug above, all four render identically anyway.

## Notes

- Spec gaps driving multiple todos: `ProgressSpec` (`packages/contracts/components/src/progress.rs`) exposes only `indicator_fill_token()` (solid accent). It lacks (a) a `track_fill_token()` / track-mix method — both impls hand-assemble the mix from raw `"color.background.surface"` + `"color.text.primary"` string lookups; (b) an indicator-gradient method — so neither impl can render the contract gradient; (c) a size→min-height method — so height is inlined in GPUI and hardcoded in Jetstream. Add all three to the spec to close the GPUI and most Jetstream todos at the source.
- `consv=gap` driver: the contract's own §8 Token Reference + §11 Tier-2 say track bg = `background-surface` at 92%, contradicting the §8 Root table and Svelte (`surface` at 96% with `text-primary`). Internal contract contradiction, fixable without touching Svelte.
- The `rounded(px(999.0))` / `rounded(999.0)` literals in both impls are the contract's literal pill radius (§8 `border-radius: 999px`), not token violations — left as-is.
