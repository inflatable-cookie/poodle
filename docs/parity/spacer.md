<!-- parity consv=ok gpui=2 jetstream=2 specimen=gap -->
# Parity: Spacer

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/spacer.md`
- Svelte (authoritative): `packages/svelte/components/src/Spacer.svelte`
- GPUI: `packages/gpui/components/src/primitives/spacer.rs`
- Jetstream: `packages/jetstream/components/src/spacer.rs`
- Spec: `packages/contracts/components/src/spacer.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/SpacerSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/spacer.rs` · jetstream `packages/jetstream/preview/src/specimens/spacer.rs`

## Contract ↔ Svelte

Aligned. No divergence.

- Props match: contract §3 `grow` (`number`, default `1`) + `minSize` (`string | null`, default `null`); Svelte `grow = 1`, `minSize = null` (`Spacer.svelte:5-12`). Same names, types, defaults.
- Anatomy matches: single root `<div class="poodle-spacer" aria-hidden="true">`, no children (`Spacer.svelte:20`). Contract §2 specifies exactly this.
- States match: no states. Contract §4 lists only `default`.
- ARIA matches: `aria-hidden="true"` on root (`Spacer.svelte:20`) = contract §6 / §8 attributes table.
- Token/style match: Svelte resets `min-width:0; min-height:0` via `.poodle-spacer` class (`Spacer.svelte:23-26`) and applies inline `flex: {grow} 1 0%` + conditional `min-width/min-height: {minSize}` (`Spacer.svelte:14-17`) — exactly contract §8 root + inline-style tables.
- Spec (`spacer.rs:5-21`) carries `grow: f32` / `min_size: Option<f32>` with `grow` default `1.0` — matches contract intent. (`min_size` typed as `f32` px not CSS string — runtime adaptation, see Notes.)

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] `grow` weight is stored but never applied — `into_element` calls bare `flex_grow()` (`spacer.rs:48-49`) which gpui hardcodes to weight `1.0` (`gpui-0.2.2/src/styled.rs:207-210`). `grow=2` and `grow=1` render identically. Breaks contract §7 weighted distribution + Tier-1 "growth weight semantics match". The file comment (`spacer.rs:46-47`) already admits this. Apply the weight via gpui's style API (or `flex_basis`/explicit `flex_grow` field) instead of the fixed-1 helper.
- [ ] `min_size` typed `Option<f32>` px (`spacer.rs:15`); Svelte/contract `minSize` is a CSS dimension string (`"64px"`, `"2rem"`). px-only loses rem/% units. Accept px adaptation or thread a length type — reconcile and note in contract Known Deltas.
- accepted: no ARIA (gpui has no accessibility API) — `aria-hidden` not emitted; spacer is presentation-only so the accessibility-neutral requirement (contract §6) is met by absence, not violated.
- note: no hardcoded color/pixel literals in the component — `min_w(px(size))`/`min_h(px(size))` (`spacer.rs:53`) resolve `size` from the spec, not a magic number. Clean on the token-literal axis.

## Jetstream gap (vs Svelte + contract)

- [ ] `grow` weight is stored but never applied — `js_spacer` calls bare `flex_grow()` (`spacer.rs:11-13`) which hardcodes `flex_grow = 1.0` in the runtime (`jetstream-runtime/src/ui_element.rs:262-270`). `grow` weight is discarded; weighted distribution between sibling spacers is impossible. Breaks contract §7 + Tier-1. Set `layout.flex_grow = spec.grow` instead of the fixed-1 helper.
- [ ] `grow=0` path skips the `min-width:0 / min-height:0` reset. Contract §8 mandates the root always reset both to `0`. `JsEl::new` defaults `min_size` to `auto`, not `0` (`jetstream-runtime/src/ui_element.rs:148-155`). When `grow <= 0.0` the `flex_grow()` branch (which resets min_size to 0) is skipped (`spacer.rs:11-12`) and, if `min_size` is `None`, `min_w/min_h` are never called (`spacer.rs:14-16`) — leaving min_size at `auto`, diverging from the contract reset. Always emit `.min_w(0.0).min_h(0.0)` (then override with `minSize` when set).
- accepted: no ARIA channel — spacer is presentation-only; accessibility-neutrality (contract §6) is satisfied by absence.
- note: no hardcoded color/pixel literals in the component itself — `min_w(size)`/`min_h(size)` (`spacer.rs:14-16`) read `size` from the spec. Clean. (Hardcoded values in the *specimen* are separate; see Specimen parity.)

## Specimen parity

- Svelte covers: "Push items apart" (Logo / Spacer / Sign in) + "Between three items" (Left / Spacer / Center / Spacer / Right) — `SpacerSpecimen.svelte`. Matches contract §13 exactly. Default `grow=1`, no `minSize`.
- GPUI covers: "Push items apart" + "Between three items" — `specimens/spacer.rs`. Matches Svelte and contract §13 one-for-one. — missing: nothing vs contract; but note both groups only exercise default `grow=1` (consistent with Svelte), so the unimplemented weight bug is invisible here.
- Jetstream covers: "Flex-row with growing spacer", "Spacer with min-size", "No-grow fixed spacer (min-size only)" — `specimens/spacer.rs`. — **diverges from the contract §13 / Svelte specimen set**: it does NOT render the "Push items apart" or "Between three items" groups, and instead invents `min-size` / no-grow demos with hardcoded `64.0`/`32.0` sizes and decorative `bg(tint(...))` fills that make the (invisible) spacer visible. Different layout demos than the authoritative reference → cross-target visual comparison is impossible. Re-author to mirror Svelte's two groups (then optionally keep a min-size demo as an extra).

## Notes

- `specimen=gap` driver: Jetstream's specimen shows a different demo set than Svelte/contract §13. GPUI matches; Svelte is the reference.
- `consv=ok`: contract and Svelte are fully in sync — nothing to reconcile on that axis.
- Shared cross-target bug: **both** Rust targets discard the `grow` weight because each runtime's `flex_grow()` helper hardcodes `1.0`. This is the single biggest functional gap — weighted distribution (the component's stated reason to exist, contract §1/§7) does not work in either Rust runtime. Two sibling spacers with different weights distribute space equally instead of proportionally.
- `minSize` type: Svelte/contract use a CSS string; both Rust specs use `f32` px. px-only cannot express `rem`/`%`. Reasonable runtime adaptation but should be recorded in contract §12 Known Deltas (currently "none").
- Jetstream specimen contains hardcoded sizing/color literals (`px(12.0)`, `text_size(11.0)`, `with_min_size(64.0)`, `tint(accent, 0.12)`), but these live in preview/specimen scaffolding, not component code, so they are outside the CLAUDE.md no-literals rule for components. The `js_spacer` component itself is literal-clean.
