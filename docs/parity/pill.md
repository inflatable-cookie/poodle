<!-- parity consv=gap gpui=3 jetstream=5 specimen=gap -->
# Parity: Pill

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/pill.md`
- Svelte (authoritative): `packages/svelte/components/src/Pill.svelte` (+ `packages/svelte/components/src/pill-context.ts`)
- GPUI: `packages/gpui/components/src/primitives/pill.rs`
- Jetstream: `packages/jetstream/components/src/pill.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/PillSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/pill.rs` · jetstream `packages/jetstream/preview/src/specimens/pill.rs`

## Contract ↔ Svelte

Svelte has props/behavior the contract does not document, and the contract's padding token table disagrees with what Svelte actually renders. Svelte is authoritative — update the contract.

- Svelte adds `adaptiveWidth?: boolean` (default `false`) → emits `data-adaptive-width`, sets `min-width: 0` (lines 19, 56, 254–256). Not in contract §3 props. **Fix: add `adaptiveWidth` to contract props + a `data-adaptive-width` data-attr note.**
- Svelte gives every size a `min-width` base (`sm 2.5rem`, `xs 2.125rem`, `md 2.875rem`, `lg 3.25rem`, `xl 3.625rem`, lines 132–168) plus per-typography `em` min-widths. Contract §8 size tables list only `min-height`/`padding`/`font-size` — no `min-width` row anywhere. **Fix: add `min-width` rows to each contract size table.**
- Svelte md `padding-x` = `0.625rem` (base `--poodle-pill-padding-x-base`, md only overrides min-width, lines 73, 150–152). Contract §8 md row says `padding 0.1875rem 0.5rem`. The pad-x values diverge for every size (Svelte base 0.625 vs contract 0.5; sm 0.5 vs 0.375; xs 0.4375 vs 0.3125; lg 0.75 vs 0.625; xl 0.9375 vs 0.75 — Svelte is ~0.125rem wider each). **Fix: rewrite contract §8 padding-x to match Svelte (the +0.125rem-wider scale).**
- Svelte adds a content `gap` (`--poodle-pill-gap`: md `0.25rem`, sm `0.1875rem`, xs `0.15625rem`, compact `0.125rem`, lines 77, 138, 147, 263) and an icon sizing rule (`svg`/`.poodle-icon` → `1em` square, lines 285–290). Contract anatomy §2 only lists Root + Content slot — no icon/gap. **Fix: document gap token + optional inline-icon child in contract.**
- Svelte badge adds `letter-spacing: 0.04em` (line 222); contract §8 badge table lists only `font-weight 700` + `text-transform uppercase`, no letter-spacing. Neutral badge fill uses `--poodle-surface` mix (line 226), not the generic "accent-tinted" wording in contract §8. **Fix: add badge `letter-spacing` + neutral-badge fill formula to contract.**
- Svelte density variants override `padding-y` (`--poodle-pill-padding-y-adjust`: compact `-0.0625rem`, comfortable `+0.0625rem`, lines 261, 269) — a vertical-padding density override, which the CLAUDE.md size/density rule forbids except by explicit justification. Contract §8 has no density padding-y table. **Fix: either drop padding-y from density (preferred per density rule) or explicitly justify it in contract; document the resolved table either way.**
- `pill-context.ts` exposes a `PillContext` (`size`, `typography`) that lets a parent force child pill size/typography (consumed at `Pill.svelte:38,41,44`). Not mentioned in contract §3/§9. **Fix: document the pill-context composition surface in contract §9.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded hover/elevation-free pad-x: md `px(rem_to_px(0.5))` at `pill.rs:113` (and every size row 106–115) uses contract's `0.5rem`, but Svelte renders `0.625rem` md pad-x — port the +0.125rem-wider Svelte pad-x scale (all 10 rows).
- [ ] No `accent` (custom color) support — `PillSpec` builder has no `accent`/`with_accent_color` path in this file; custom-accent fill/border/text (Svelte lines 122–126) never rendered. Specimen passes accent via `with_accent_color` but the `into_element` color match (`pill.rs:132–188`) ignores it.
- [ ] No `min-width` floor — Svelte sets per-size `min-width` (e.g. md `2.875rem`); `pill.rs:202–218` sets `min_h`/`px`/`py` but never `min_w`. Pills shrink narrower than Svelte. Add `min_w` from the per-size base.
- accepted: no ARIA (gpui has no accessibility API) — pill is non-interactive text-like metadata per contract §6.
- accepted: `font="mono"` code-family + letter-spacing not applied — GPUI text surface delta (contract §12 Known Delta covers Jetstream; same limit here).
- accepted: extra spec surface (`is_selected`/`is_removable`/`on_remove`, `pill.rs:71–90,244–263`) is non-contract chip behavior carried by `PillSpec`; render-gated, not a Svelte regression. Flag for contract reconciliation (see Notes).

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded pad-x scale matches contract `0.5rem` md not Svelte `0.625rem` — `pill_metrics` rows at `pill.rs:11–24` need the +0.125rem-wider Svelte pad-x for all 10 rows.
- [ ] No `accent` custom-color path — `pill_colors` (`pill.rs:26–72`) only branches on tone/appearance; `accent`-tinted fill/border/text (Svelte 122–126) absent.
- [ ] No `min-width` floor — `js_pill` (`pill.rs:87–97`) sets `min_h` but no `min_w`; Svelte per-size min-width unmet.
- [ ] No content `gap` between label and any inline child — `js_pill` renders a single `label()` (`pill.rs:87`); Svelte gap token (lines 77,138,147) unused. Acceptable only while pill stays label-only; note if icon children land.
- [ ] No badge `letter-spacing` (`0.04em`, Svelte line 222) — Jetstream text API lacks tracking; same class of delta as `font="mono"`.
- accepted: `font="mono"` code-family + letter-spacing — contract §12 Known Delta (Jetstream `JsEl` text surface exposes no font-family/letter-spacing).
- accepted: interaction n/a — pill is non-interactive per contract §5.

## Specimen parity

- Svelte covers: Tones (5), Code font (mono ×3), Muted (×3), Badge (×5 incl. neutral), Inherited typography (inline), Custom accent (×4), plus size/density render-snippets (`PillSpecimen.svelte`).
- GPUI covers: Tones (5), Code font (×3), Muted (×3), Badge (×5), Inherit typography (inline), Custom accent (×3), size + density via `specimen_layout` (`gpui/.../pill.rs`). — matches Svelte well; only delta: accent visually no-ops until the GPUI accent gap above is fixed.
- Jetstream covers: Default (×3 plain labels), Selected (×1), Disabled (×1), Inherit typography (inline) (`jetstream/.../pill.rs`). — missing: **Tones**, **Code font**, **Muted**, **Badge**, **Custom accent** groups; shows non-contract **Selected**/**Disabled** states instead. Largest specimen gap of the three.

## Notes

- `consv=gap` drivers: undocumented `adaptiveWidth`, missing `min-width`/`gap`/badge-letter-spacing in contract, and a real pad-x mismatch (contract 0.5rem md vs Svelte 0.625rem). All belong in the contract per "Svelte is parity authority".
- Both Rust impls hardcode the contract pad-x (`0.5rem` md) rather than the Svelte-rendered `0.625rem`. Once the contract is corrected to Svelte's scale, both Rust metric tables must follow — that is why each has a pad-x todo.
- `PillSpec` carries `is_selected`/`is_removable`/`on_remove`/`is_disabled` (interactive-chip surface) that exist in neither Svelte nor the contract. Per the contract's "out of scope: removable chips" line this is intentional future work; reconcile by either contracting these or removing from `PillSpec`. Plus the legacy `BadgeSpec` noted in contract §"Rust Spec Note" pending removal.
