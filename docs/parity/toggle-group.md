<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok | specimen pass: both Rust previews built out to full contract coverage (build-verified 0 err). GPUI converted to specimen_layout (adds Sizes + Densities panes) keeping Single/Four-options/Multiple/Disabled-group/Disabled-item + an Allow-deactivation group; aria_label added to every group. Jetstream rebuilt onto the contract Grid/List/Board set (dropped B/I/U glyphs): Single, Four options (Left/Center/Right/Justify), Multiple (Design+Docs), Allow-deactivation, Disabled-item, Fully-disabled, full xs–xl Sizes ladder, Compact/Default/Comfortable Densities. Skipped honestly: icon-only items (ToggleGroupOption has no icon field); live on_change readout on Jetstream (render-only per architecture). | pass: both Rust targets closed. Added `allow_deactivation` to ToggleGroupSpec (additive) + a shared `next_value_on_toggle` selection helper (single/multiple/deactivation), unit-tested with 4 tests in poodle-specs. GPUI: decoupled arrow keys from selection (Space/Enter toggle; arrows are roving-focus-only / platform-owned, no longer commit value) and exposed an `allow_deactivation` builder; selected-fill formula confirmed correct. Jetstream: gap now density-driven via `toggle_group_gap_rem` (dropped the *0.5 heuristic), font-size now resolves from the `typography.label.size` token (was flat 0.75rem). 11 tests in jetstream toggle_group.rs (5 new probe-based: per-option buttons, accent-tint selected, multi-select, density≠height, label-size font). Interaction/roving-focus/font-family remain preview-loop / JsEl-channel deltas. -->
# Parity: ToggleGroup

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/toggle-group.md`
- Svelte (authoritative): `packages/svelte/components/src/ToggleGroup.svelte`
- GPUI: `packages/gpui/components/src/primitives/toggle_group.rs`
- Jetstream: `packages/jetstream/components/src/toggle_group.rs`
- Spec: `packages/contracts/components/src/toggle_group.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ToggleGroupSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/toggle_group.rs` · jetstream `packages/jetstream/preview/src/specimens/toggle_group.rs`

## Contract ↔ Svelte

Multiple §8 token values were stale vs Svelte; Svelte is authoritative — contract fixed.

- [x] FIXED **Item background mismatch.** §8 said `surface 72% / background-elevated`; Svelte resolves `surface 93% / text-primary` (`ToggleGroup.svelte:165-168`). Contract §8 item background → `surface 93%, text-primary` (with treatment-var fallback). Surface-elevation note + Tier-2 updated.
- [x] FIXED **Selected background mismatch.** §8 said flat `accent-base 22% / transparent`; Svelte layers a `linear-gradient(accent 22% → accent 22%)` over the unselected fill (`:183-197`), keeping the surface base. Contract selected background → gradient tint layered over the item fill. GPUI notes + Tier-2 updated.
- [x] FIXED **Font-size source.** §8 hardcoded `0.75rem`; Svelte uses `var(--poodle-typography-label-size)` (`:173`). Contract → label-size token. Tier-2 updated.
- [x] FIXED **Treatment-variable indirection.** Documented `--poodle-treatment-interactive-*` (border/fill/radius/shadow/border-active) as the brand-raised theming layer with color-mix fallbacks in §8 tables + §9 Svelte Notes.
- `value` default: contract §3 `undefined`; Svelte `undefined` (`:26`) — matches. `defaultValue` `null` matches. No change.
- [x] FIXED (spec) **`allowDeactivation` added to Rust spec.** `ToggleGroupSpec` now has `allow_deactivation: bool` (default false) + `with_allow_deactivation`, plus a shared `next_value_on_toggle` helper mirroring Svelte `toggle()`. Unit-tested in poodle-specs.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED `allowDeactivation` support — spec field + `allow_deactivation(bool)` builder added. The clear-to-empty resolution lives in `spec.next_value_on_toggle` (shared); the GPUI `on_change(&str)` callback emits the toggled value and the preview loop applies `next_value_on_toggle` (preview-loop, like all GPUI selection wiring).
- [x] FIXED Roving-arrow decoupled — arrow keys no longer call the change callback; only Space/Enter toggle selection on the focused item. Arrow-driven roving focus is platform-owned (contract Tier 3), so arrows are a no-op at the component level rather than committing value (matches Svelte `moveHighlight`, which never fires `onValueChange`).
- [x] (confirmed) Selected fill matches Svelte: `accent.mix(item_fill, 0.22)` layered over the `surface 93% / text-primary` base fill — already correct, kept.
- [x] (confirmed) Multiple-mode toggle-off works through the shared `next_value_on_toggle` logic (independent add/remove), unit-tested. Role distinction (radio vs button) remains accepted-ARIA.
- accepted: no ARIA (gpui has no accessibility API) — radiogroup/group + aria-checked/pressed not emitted.
- accepted: transition timing (180ms ease) not modeled (contract Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] **DONE: item background** — now `surface.mix(text_primary, 0.93)` (Svelte color-mix(surface 93%, text-primary)), was the stale surface/elevated 72%. Locked by `item_fill_uses_svelte_surface_text_mix` probe test.
- [x] **DONE: selected fill** — now `accent.mix(item_fill, 0.22)` (accent tinted over the item fill), was accent-over-transparent.
- [x] FIXED Gap now density-driven via `toggle_group_gap_rem` (0.1875 / 0.25 / 0.375 rem), dropped the `* 0.5` heuristic; matches Svelte + GPUI exactly.
- [x] FIXED Font-size now resolves from the `typography.label.size` token (flat across sizes, as in Svelte — data-size only changes height). The `rem_to_px(0.25)` height reduction and `rem_to_px(0.0625)` border are contract-exact rem (acceptable, not violations).
- [ ] (JsEl gap) No `font-family` token applied — contract §8 + Svelte set `typography-label-family`; JsEl exposes weight/size only, no font-family channel. Noted.
- [x] FIXED `allowDeactivation` — spec field + builder now exist (see contract↔Svelte row); selection-clear resolves through the shared `next_value_on_toggle`.
- accepted: interaction/callback — `js_toggle_group` is render-only; click/key/`on_change` wiring lives in the preview event loop (Jetstream architecture), like every other Jetstream component.
- accepted: roving-focus modeling + per-size specimen path are preview-loop / specimen concerns.
- accepted: no ARIA channel (radiogroup/group, aria-checked/pressed).
- accepted: interaction wiring may live in preview loop, but here there is no wiring at all.

## Specimen parity

- Svelte covers: Single selection (+ live hint), Four options, Multiple selection (+ hint), Disabled, Sizes snippet, Densities snippet (`ToggleGroupSpecimen.svelte`).
- GPUI covers: Single (+ live value), Four options, Multiple, **Allow deactivation**, Disabled group, **Disabled item** (broader than Svelte), plus **Sizes + Densities panes** (now via `specimen_layout`). — no remaining gaps. (The old hand-rolled "Semantic role offsets" triple was dropped in favour of the standard sizes/densities panes; `aria_label` now set on every group.)
- Jetstream covers: Single, Four options (Left/Center/Right/Justify), Multiple (Design+Docs), **Allow deactivation**, With disabled item, Fully disabled, plus **full xs–xl Sizes ladder** and **Compact/Default/Comfortable Densities**. Now on the contract **Grid/List/Board** set (B/I/U glyphs dropped). — no remaining gaps; live `on_change` readout omitted (render-only per Jetstream architecture).

## Notes

- `consv=gap` driver is stale §8 token math (item background `72%/elevated` → should be `93%/text-primary`; selected tint should layer over fill, not transparent) plus the missing `allow_deactivation` spec field. Per "Svelte is parity authority", update the contract for the first two and add the spec field for the third.
- Jetstream is the worse target: wrong fill formulas (copies stale contract), no interaction, hardcoded dimensions. GPUI is token-clean but couples arrow-focus to selection.
- Specimen content drift: Jetstream's B/I/U set is fine functionally but diverges from the contract's documented Grid/List/Board / alignment specimens — minor.
