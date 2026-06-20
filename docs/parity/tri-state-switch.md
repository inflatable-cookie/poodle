<!-- parity consv=fixed gpui=5 jetstream=6 specimen=gap -->
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

- [ ] **Spec models the wrong value type.** GPUI/Jetstream `TriStateSwitchSpec` (`packages/contracts/components/src/tri_state_switch.rs`) stores `state: CheckState` (Unchecked/Mixed/Checked) instead of a dedicated `TriStateValue` (excluded/default/included). The whole component aliases checkbox semantics. Callbacks emit `&CheckState`. Add a real `TriStateValue` enum (or rename) so the API reads `excluded`/`default`/`included` — currently every call site maps `Unchecked→excluded` etc. by hand (`gpui/preview/.../tri_state_switch.rs:10-32`).
- [ ] **Segment widths are hardcoded rem literals**, not token/contract-derived. `segment_min_w` = 3.5/4.0/4.5/5.0/5.5 rem by size (`tri_state_switch.rs:116-122`); contract uses `min-content-width + x*2` (Svelte 2.5–3.75 rem content + density `x`). Resolve from `size_min_width_rem` + density `x`, drop the bespoke ladder.
- [ ] **Hardcoded selection shadow literals.** `hsla(0.0,0.0,1.0,0.08)` and `hsla(0.0,0.0,0.0,0.18)` with `px(1.0)`/`px(2.0)`/`px(8.0)` offsets/blur (`tri_state_switch.rs:179-189`). These are raw white/black mixes; contract §8 selection `box-shadow` is `color-mix(white 8%, transparent)` + `color-mix(black 18%, transparent)`. Resolve via `color_mix` over a token, not literal `hsla`.
- [ ] **Hardcoded segment padding.** `.px(px(rem_to_px(0.875)))` (`tri_state_switch.rs:199`) ignores density. Contract segment `padding` is `0 var(--poodle-tri-state-x)` (density-driven: 0.5/0.75/1 rem). Resolve from `control_space_x_rem(density)`.
- [ ] **Track inset / default-track ratio drift.** `track_padding = px(rem_to_px(0.125))` fixed (`tri_state_switch.rs:114`) ignores density (should be 0.0625/0.125/0.1875 rem); default fill uses `color_mix(..., 0.08)` (`tri_state_switch.rs:148`) which matches Svelte 8% but the comment claims "88% background-elevated" intent — reconcile once the contract is fixed.
- accepted: no native radiogroup — GPUI implements arrow/Space keyboard + per-segment focus explicitly (`tri_state_switch.rs:267-312`), the pre-approved §12 delta.
- accepted: no ARIA tree (gpui has no a11y API) — exclusive-choice semantics conveyed visually + via keyboard only.

## Jetstream gap (vs Svelte + contract)

- [ ] **Same `CheckState`-not-`TriStateValue` spec problem** (shared spec; see GPUI todo 1). `js_tri_state_switch` matches on `CheckState::Unchecked/Mixed/Checked` (`tri_state_switch.rs:74-103`).
- [ ] **Min segment width is an ad-hoc heuristic.** `size_min_width_rem(effective_size) * 0.4` (`tri_state_switch.rs:61`) — the `* 0.4` is a magic factor, not the contract's `min-content-width + x*2`. Resolve properly.
- [ ] **Hardcoded label size/weight.** `label_size = rem_to_px(0.8125)` and `label_weight = 500` as literals with "typography-label-size/weight" comments (`tri_state_switch.rs:70-71`). Contract segment font is `var(--poodle-typography-label-size/weight)`. Resolve from the typography tokens, do not inline `0.8125`/`500`.
- [ ] **No selection capsule / sliding affordance.** Svelte + GPUI render an absolutely-positioned capsule that slides via `translateX(index*100%)` with an inset+drop shadow. Jetstream just fills the active segment's own background (`tri_state_switch.rs:132-142`) with no shadow and no shared sliding capsule. Contract anatomy requires the `Selection` part with its `box-shadow`. Add the capsule + shadow.
- [ ] **Selected border ratio drift.** Excluded/included selection border uses `alpha * 0.38` (`tri_state_switch.rs:80-82,96-99`); Svelte tints `58%` toward the state color mixed with `border-default`. Match the 58% recipe.
- [ ] **Track-inset and `999.0` radius.** `track_inset_rem` (0.125/0.1875/0.25, `tri_state_switch.rs:20-26`) does not match Svelte's density inset (0.0625/0.125/0.1875, lines 156–167); `.rounded(999.0)` is a literal pill radius (`tri_state_switch.rs:117,149`) — acceptable as a "max pill" sentinel but note it is not token-derived.
- accepted: interaction (segment click + arrow keys) lives in the preview event loop, not the component (no `on_change` wiring inside `js_tri_state_switch`).
- accepted: no ARIA / radiogroup (immediate-mode runtime; pre-approved §12 delta).

## Specimen parity

- Svelte covers: Default (interactive, live value readout), Custom labels, Disabled — `TriStateSwitchSpecimen.svelte` (per contract §13).
- GPUI covers: Default (interactive + value readout), Custom labels, Semantic sizes (xs–xl), Chrome-vs-prominent role offset, Disabled, **Custom semantic colors** (`#ef4444`/`#64748b`/`#22c55e`) — `gpui/.../tri_state_switch.rs`. Broader than Svelte; at/above parity.
- Jetstream covers: States (all three), Sizes (sm/md/lg), Disabled — `jetstream/.../tri_state_switch.rs`. **Missing vs Svelte/contract: Custom labels group, Custom semantic colors group, and the interactive live-value readout** (specimen is fully static, no `onValueChange` wiring). Uses the non-contract `label` prop ("Excluded"/"Small"/etc.) as a row caption.

## Notes

- The `label`/`with_label` field on `TriStateSwitchSpec` (`tri_state_switch.rs:9`, used as a trailing caption in both Rust impls) is **not in the contract or Svelte**. Either add it to the contract as a Rust-runtime affordance or drop it — currently it is undocumented surface used only to caption specimens.
- Root cause of most `consv` churn: the contract §8 exact-value table was written before the Svelte canvas/black track recipe landed. Per "Svelte is parity authority," the contract should be updated to the 75%-canvas/black background and 14/8/14 fill ratios — not the other way round.
- The shared-spec `CheckState` aliasing is the single highest-leverage fix: it makes both Rust targets read as checkboxes, forces every call site to translate enums by hand, and leaves the `track_fill_token()` helper (`tri_state_switch.rs:111-117`, mapping to accent tokens) entirely unused/wrong relative to the danger/text/success semantic coloring the component actually needs.
