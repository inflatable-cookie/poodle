<!-- parity consv=fixed gpui=7 jetstream=7 specimen=gap -->
# Parity: Card

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/card.md`
- Svelte (authoritative): `packages/svelte/components/src/Card.svelte`
- GPUI: `packages/gpui/components/src/primitives/card.rs`
- Jetstream: `packages/jetstream/components/src/card.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CardSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/card_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/card.rs`

## Contract ↔ Svelte

Prop/anatomy/state divergences. For each: what differs, which side is right (Svelte unless missing contract-specified functionality), and the action.

- [x] FIXED **card-fill mix ratio.** Contract §8 `--poodle-recipe-card-fill` now reads `color-mix(panel 10%, elevated)`, matching Svelte (`Card.svelte:75-79`). (GPUI `card.rs:124-126` still copies the old 98% — flagged in the gpui gap below.)
- **No selected/media props in `CardSpec`.** Svelte exposes `selected` and `media` (`Card.svelte:11-12`); `CardSpec` (`packages/contracts/components/src/card.rs:20-26`) has `is_selected` but **no `media` field**, and **no `density` field** despite the contract listing `density` as a public prop (`card.md:44`). Svelte carries `density` (`Card.svelte:10,37`). **Fix: add `density` + `media` to `CardSpec` so the Rust targets can express them.**
- **Density token methods absent.** Contract §8 density table (`card.md:215-219`) and Svelte (`Card.svelte:188-207`) drive gap/padding/footer-padding per density; `CardSpec` `gap_token`/`padding_x_token`/`padding_y_token` (`card.rs:122-135`) are density-blind (only branch on `Compact` layout). **Fix: make these token methods density-aware once `density` is added.**
- **Interactive a11y is a documented known gap on BOTH sides** — contract §6/§12 say `role="button"`/`tabindex`/Enter-Space are NOT implemented in Svelte; Svelte only applies `cursor: pointer` (`Card.svelte:155-157`). No divergence here, but it is the contract's standing Known Delta.
- consv=fixed: the one contract↔Svelte divergence (card-fill ratio) is reconciled. The contract already documents `density` and `media` as public props (§3) and the density table (§8) — those are present on the Svelte side and the contract; the missing `density`/`media` fields are on the Rust `CardSpec` struct (code-side), captured in the gpui/jetstream gaps below.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Wrong card-fill ratio — `card.rs:124-126` uses `color_mix(panel, elevated, 0.98)` per the stale contract; Svelte is `panel 10%, elevated`. Match Svelte (`color_mix(panel, elevated, 0.10)`) after the contract is corrected.
- [ ] Hardcoded elevated-shadow color literals `hsla(0.0, 0.0, 0.0, 0.38)` / `hsla(0.0, 0.0, 0.0, 0.24)` at `card.rs:205,211` — resolve from a shadow/elevation token, not raw HSLA.
- [ ] Hardcoded shadow geometry `px(18.0)`/`px(40.0)`/`px(6.0)`/`px(14.0)` at `card.rs:206-213` — derive the elevated drop-shadow offsets/blur from tokens (contract §8 lists exact rem values), not raw px floats.
- [ ] No light-mode elevated shadow branch — only the dark-mode shadow is emitted (`card.rs:203-216`); contract §8 specifies a distinct light-mode `box-shadow` (`card.md:171-175`). Add the light/dark conditional.
- [ ] Compact layout uses the wrong tokens — `card.rs:110-113` substitutes `space.inline.md`/`space.inline.sm`; contract §8 compact padding is `0.5rem 0.625rem` via `space.panel-y-sm`/`space.panel-x-sm` (`Card.svelte:182-186`). Resolve the panel-sm tokens.
- [ ] No `density` axis — GPUI cannot set compact/comfortable density gap/padding/footer-padding (contract §8 density table); blocked on `CardSpec` gaining `density`.
- [ ] No `selected` hover-preserve, no `media` slot — selected-hover (`Card.svelte:165-170`) keeps the accent ring on hover; GPUI hover just swaps fill (`card.rs:235-237`). Media region (`card__media`, overflow-hidden, inset radius) is unimplemented.
- accepted: no ARIA (gpui has no accessibility API) — `aria-label` stored on spec but not emitted; interactive `role="button"`/keyboard is a contract-wide Known Delta.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded border width `border(1.0)` at `card.rs:41` — resolve from a border-width token (`border.width.default`), not a raw `1.0`.
- [ ] Wrong hover-fill mix — `card.rs:24` uses `fill.mix(elevated, 0.92)`; Svelte hover-fill is `color-mix(elevated 94%, panel)` (`Card.svelte:92-95`), i.e. it mixes toward elevated, not a `fill→elevated 92%` blend. Match the Svelte recipe (and the comment "fill 92% + elevated 8%" is also wrong).
- [ ] Wrong card-fill ratio — `js_card` resolves `spec.fill_token()` to a flat surface color (`card.rs:16`); Svelte fill is `color-mix(panel 10%, elevated)`. Apply the mix, don't use the flat token.
- [ ] No elevated variant treatment — `Elevated` gets no multi-layer drop shadow / elevated fill / elevated radius (contract §8, `Card.svelte:122-146`). `js_card` only branches border/selected, never shadow.
- [ ] No `selected` shadow ring — selected only swaps border width/color (`card.rs:44-48`); Svelte adds the accent inset+outset `box-shadow` (`Card.svelte:148-153`). Add the ring.
- [ ] No `density` axis and no compact-layout padding override — `padding_x_token`/`padding_y_token` are density/compact-blind; compact layout (`0.5rem 0.625rem`) and the density table are unimplemented. Blocked on `CardSpec` gaining `density`.
- [ ] No `media` slot — no overflow-clipped media region with inset radius (contract §2/§8 media row); `js_card` only stacks `children`.
- accepted: no ARIA channel for `aria_label`/`role="button"` (contract-wide Known Delta).
- accepted: interaction (click handler) lives in the preview event loop, not the component.

## Specimen parity

- Svelte covers: Default variant (2 cards, header+body+footer / header+body), Outlined, Elevated, Interactive, plus a density matrix via `SpecimenLayout` `densities` snippet (compact/default/comfortable) (`CardSpecimen.svelte:9-71`).
- GPUI covers: Default / Outlined / Elevated / Interactive groups — **but does NOT use the `Card` component at all**. `card_specimen.rs` hand-codes each card with `Surface` + raw `div()` and hardcoded `px(280.0)`/`px(8.0)`/`px(12.0)`/`px(16.0)`/`px(24.0)` literals and a hand-rolled footer divider (`card_specimen.rs:13-162`). — missing: real `Card`-component usage, **Horizontal layout**, **Selected state**, **Density matrix**, **Media slot**. This is a fake specimen per the project "no mockups" rule — rewrite to drive `poodle_gpui_components::Card`.
- Jetstream covers: Default / Outlined / Horizontal layout (`card.rs:15-39`). — missing: **Elevated**, **Interactive**, **Selected**, **Density matrix**, **Media slot**. Horizontal media is faked with a raw `div().w(rem_to_px(3.0))` block (`card.rs:33`) rather than a real media region.

## Notes

- The GPUI specimen building on `Surface` instead of `Card` is the single biggest card finding: it hides that the `Card` component is under-exercised and lets hardcoded px values masquerade as parity.
- `CardSpec` (`packages/contracts/components/src/card.rs`) is missing the `density` and `media` fields entirely; both block density/media parity on the two Rust targets and are the structural prerequisite for the GPUI/Jetstream todos above.
- consv=fixed: the card-fill ratio mismatch (contract 98% → Svelte 10%) is corrected in the contract. The absent `density`/`media` fields on `CardSpec` remain a code-side task (extend `CardSpec` + make the density token methods density-aware), not a contract↔Svelte divergence.
