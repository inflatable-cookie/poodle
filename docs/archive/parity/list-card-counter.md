<!-- parity consv=fixed gpui=2 jetstream=2 specimen=ok -->
# Parity: ListCardCounter

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/list-card-counter.md`
- Svelte (authoritative): `packages/svelte/components/src/ListCardCounter.svelte`
- GPUI: `packages/gpui/components/src/primitives/list_card_counter.rs`
- Jetstream: `packages/jetstream/components/src/list_card_counter.rs`
- Spec: `packages/contracts/components/src/list_card_counter.rs`
- Specimens: svelte — none standalone (shown inside `ListCardSpecimen.svelte` "With footer counters") · gpui — within `list_card.rs` specimen · jetstream — within `list_card.rs` specimen

## Contract ↔ Svelte

The contract's token table had drifted from the Svelte CSS. Svelte is authoritative — contract updated to match (the Rust spec still follows the old values; tracked under the Rust gaps).

- [x] FIXED Font size: contract §8 now `0.6875rem` (was `0.75rem`), matching Svelte (`ListCardCounter.svelte:61`).
- [x] FIXED Icon size: contract §8 now `0.75rem` icon + documented `0.82` icon opacity, matching Svelte (`:62,75`).
- [x] FIXED Color: contract §8/§4 now `color-mix(currentColor 36%, transparent)` default and `color-mix(currentColor 58%, transparent)` on linked hover (Svelte `:66,86`).
- [x] FIXED `typography="inherit"` icon size: contract §8 now `1em` (was `1.3333em`), matching Svelte (`:81`).
- [x] FIXED `onClick` prop added to contract §3/§5/§9; runs after `stopPropagation` for linked counters, matching Svelte (`:13,26-29`).

## GPUI gap (vs Svelte + contract)

GPUI is structurally faithful (tooltip wrap, linked hover, rem-derived metrics via spec helpers — no raw px literals). Gaps are the inherited stale values + tabular-nums.

- [ ] Color/size follow the stale spec: `font_size_rem()=0.75`, `icon_size_rem()=1.0`, color from `text_secondary_token()` (`list_card_counter.rs:84-94`). Per Svelte these should be `0.6875rem` font, `0.75rem` icon, `color-mix(currentColor 36%)`. The fix is in the spec (`list_card_counter.rs:82-92` + `text_secondary_token`), then GPUI inherits it. Track here so the GPUI render is corrected once the spec is.
- [ ] `font-variant-numeric: tabular-nums` not applied — documented in the file header (`list_card_counter.rs:5`). Contract §8 requires it; GPUI has no numeric font-feature API yet. accepted-ish but listed as open per contract.
- accepted: no ARIA (gpui has no accessibility API) — anchor/link semantics handled via click callback + `aria-describedby` from Tooltip not emitted.
- accepted: tooltip needs `on_tooltip_open_change` wired by the host to open; without it the row renders alone (documented `:150-152`).

## Jetstream gap (vs Svelte + contract)

- [ ] Same stale spec values as GPUI: font `0.75rem` / icon `1.0rem` / `text_secondary_token` (`list_card_counter.rs:21-24,30-31`). Should be `0.6875rem` / `0.75rem` / currentColor-36% per Svelte. Fix in the spec; Jetstream inherits.
- [ ] No tabular-nums and no anchor semantics — both documented runtime deltas (`list_card_counter.rs:33-35`, §12 Known Deltas allow this). Linked styling (cursor + hover color) is present; literal `<a>` navigation and tooltip-trigger wrapping are deferred.
- accepted: tooltip trigger wrapping unsupported (Jetstream tooltip is panel-only — §12 Known Delta).
- accepted: link navigation is a shell concern (§12 Known Delta).

## Specimen parity

- Svelte covers: three counters (incl. one linked with href) + two-counter group, inside `ListCardSpecimen` "With footer counters" (contract §13).
- GPUI covers: footer counters within the list_card specimen — present.
- Jetstream covers: footer counters within the list_card specimen — present (though Jetstream's list_card specimen is thin overall; verify the counter group is actually rendered).

specimen=ok: this is a helper with no standalone specimen by design (contract §13); all targets demonstrate it inside their ListCard footer.

## Notes

- Token discipline here is actually good: both Rust targets resolve metrics from spec helper methods (`gap_rem`/`font_size_rem`/`icon_size_rem`) and color tokens rather than hardcoding raw literals. The `rem_to_px(spec.gap_rem())` form is the sanctioned proportional-inherit pattern (contract §10).
- The real issue is a three-way drift: **Svelte → contract → Rust spec**. Svelte changed its CSS (0.6875rem font, 0.75rem icon, currentColor-relative color) but neither the contract token table nor `ListCardCounterSpec`'s rem helpers were updated. Per "Svelte is parity authority", fix the contract §8 table and the spec's `font_size_rem`/`icon_size_rem`/color-token to match Svelte; both Rust components then correct automatically.
- consv=gap is driven entirely by this stale token table, not by missing functionality.
