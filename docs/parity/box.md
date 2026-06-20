<!-- parity consv=ok gpui=2 jetstream=3 specimen=gap -->
# Parity: Box

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/box.md`
- Svelte (authoritative): `packages/svelte/components/src/Box.svelte`
- GPUI: `packages/gpui/components/src/primitives/bx.rs`
- Jetstream: `packages/jetstream/components/src/bx.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/BoxSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/bx.rs` · jetstream `packages/jetstream/preview/src/specimens/bx.rs`

## Contract ↔ Svelte

Svelte matches the contract. All props present with correct defaults: `padding="none"`, `width/height/minWidth/minHeight=null`, `overflow="visible"`, `asRole=null`, `ariaLabel=null`. Padding scale maps `none→0`, `sm→space-inline-sm`, `md→space-panel-y`, `lg→space-panel-x` (matches contract §8 token map). Base `min-width:0`/`min-height:0` present. `role`/`aria-label` set conditionally.

- No divergences. `consv=ok`.

## GPUI gap (vs Svelte + contract)

- [ ] `parse_dimension_px` (`bx.rs:77-85`) only handles `px`/plain-number and a literal `"100%"` special case; `rem`, `vh`, `%`-other, `calc()` etc. are silently dropped. Contract allows "any CSS value" for width/height/minWidth/minHeight. **Parse `rem` (× 16) at minimum** so `width="12rem"` works (the GPUI box specimen hand-codes `192px` precisely because this fails).
- [ ] `ariaLabel` / `asRole`: spec has `role` builder but `aria_label` is not forwarded and no role is mapped into the GPUI accessibility tree (contract §10 requires `asRole` → native a11y tree). 
- accepted: no ARIA emission (gpui has no accessibility API) — overlaps the role mapping above; the missing builder is still a real gap.

## Jetstream gap (vs Svelte + contract)

- [ ] `js_box` (`bx.rs`) ignores `width`, `height`, `min_width`, `min_height` entirely — only padding + `overflow==Hidden` are applied. Contract §3/§7 require explicit dimension constraints. **Resolve and apply `spec.width/height/min_width/min_height`.**
- [ ] Overflow only handles `Hidden`; `Auto`/`Scroll`/`Clip` modes are not mapped (Visible is the implicit default). Contract `OverflowMode` has 4 values.
- [ ] No `role`/`aria_label` handling.
- accepted: interaction n/a (layout primitive, no events).

## Specimen parity

- Svelte covers: Default (no padding), With padding (lg), Fixed dimensions (12×6rem + md padding), Overflow hidden (10×3rem clipped) — all 4 contract groups.
- GPUI covers: all 4 groups — but Fixed dimensions and Overflow are **faked**: dimensions come from a hand-coded outer `div().w(px(192.0)).h(px(96.0))` / `div().w(px(160.0)).h(px(48.0)).overflow_hidden()` wrapper, NOT from the Box spec (because the Box can't resolve `12rem`/clip). Visually covers, but does not exercise the component path. — missing: real spec-driven width/height/overflow.
- Jetstream covers: With padding (md), Large padding (lg) only. — missing: **Default (no padding)**, **Fixed dimensions**, **Overflow hidden**. (Can't show Fixed/Overflow until `js_box` supports width/height/overflow modes.)

## Notes

- Per repo policy, a specimen that hand-codes dimensions instead of resolving them through the Spec is "worse than no specimen". The GPUI Fixed/Overflow groups and the absence of Jetstream Fixed/Overflow groups both trace to the same root: Rust `js_box`/`Box` don't resolve non-px dimensions. Fix the components first, then make specimens spec-driven.
