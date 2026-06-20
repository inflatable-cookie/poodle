<!-- parity consv=fixed gpui=3 jetstream=5 specimen=gap -->
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

- FIXED — `adaptiveWidth?: boolean` (default `false`) added to contract §3 props with a §8 `data-adaptive-width` → `min-width: 0` table.
- FIXED — per-size `min-width` base (`sm 2.5rem`, `xs 2.125rem`, `md 2.875rem`, `lg 3.25rem`, `xl 3.625rem`) added as `min-width` rows to every §8 size table (rem and `em`-inherit), plus the Root `min-width` calc.
- FIXED — `padding-x` rewritten to Svelte's +0.125rem-wider scale across all sizes (md `0.625`, sm `0.5`, xs `0.4375`, lg `0.75`, xl `0.9375`); the Root/md `padding` now reads `0.1875rem 0.625rem`.
- FIXED — content `gap` (`--poodle-pill-gap`: md `0.25rem`, sm `0.1875rem`, xs `0.15625rem`, compact `0.125rem`) documented in Root + per-size tables; optional inline-icon child (`svg`/`.poodle-icon` → `1em` square, `flex-shrink:0`) added to anatomy §2.
- FIXED — badge `letter-spacing: 0.04em` added; §8 badge table rewritten with the neutral-badge fill (`color-mix(--poodle-surface 96%, text-primary)`, text-secondary) and the tone-badge 18%/42%-toward-transparent mixes.
- FIXED (justified) — density `padding-y-adjust` (compact `-0.0625rem`, comfortable `+0.0625rem`) documented in a new §8 Density table with explicit justification under the Size/Density exception (pill is a sub-text-line chip), alongside the min-width/padding-x/gap adjusts.
- FIXED — pill-context composition surface (`setPillContext({ size?, typography? })`, context wins over props) documented in contract §9 (Svelte Notes).
- Also fixed in passing: the stale `typography="inherit"` `em` size tables were superseded by Svelte (e.g. md font `0.7071em` not `0.7857em`); all five inherit tables updated to Svelte's exact `em` values + `min-width` rows.

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

- `consv=fixed`: former drivers (undocumented `adaptiveWidth`, missing `min-width`/`gap`/badge-letter-spacing, pad-x mismatch contract 0.5rem md vs Svelte 0.625rem, pill-context, density padding-y, stale inherit `em` tables) are all reconciled into the contract per "Svelte is parity authority".
- Both Rust impls hardcode the contract pad-x (`0.5rem` md) rather than the Svelte-rendered `0.625rem`. Once the contract is corrected to Svelte's scale, both Rust metric tables must follow — that is why each has a pad-x todo.
- `PillSpec` carries `is_selected`/`is_removable`/`on_remove`/`is_disabled` (interactive-chip surface) that exist in neither Svelte nor the contract. Per the contract's "out of scope: removable chips" line this is intentional future work; reconcile by either contracting these or removing from `PillSpec`. Plus the legacy `BadgeSpec` noted in contract §"Rust Spec Note" pending removal.
