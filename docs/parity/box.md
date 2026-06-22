<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok | pass: gpui `parse_dimension_px` now resolves `rem` (×16) so `12rem` works; jet `js_box` applies width/height/min_w/min_h (rem/px/100%) + maps all 4 overflow modes (hidden/clip→clip, auto/scroll→scroll-list); BoxSpec gained `aria_label`; both a11y emissions stay accepted runtime limits; jet box specimen rebuilt spec-driven (default/padding/fixed-12x6rem/overflow-hidden); 5 probe tests added -->
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

- [x] DONE: `parse_dimension_px` now resolves `rem` (× 16) in addition to `px`/plain-number; `width="12rem"` resolves to 192px through the spec path. `vh`/`%`-other/`calc()` still return `None` (no absolute px) — `100%` handled via `w_full`/`h_full`.
- [x] DONE: `aria_label` forwarded — `BoxSpec` gained an `aria_label` field + `Box::aria_label()` builder paired with `role`. GPUI has no accessibility-tree API, so the value is stored but not emitted (accepted runtime limit, contract §10).
- accepted: no ARIA emission (gpui has no accessibility API) — role/aria_label are now reachable on the spec; native-tree emission is the accepted limit.

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `js_box` now resolves and applies `width`/`height`/`min_width`/`min_height` via `parse_dimension_px` (rem ×16, px, bare number); `100%` maps to `w_full`/`h_full`. Probe-verified `12rem`→192px / `6rem`→96px.
- [x] DONE: all overflow modes mapped — `Hidden`/`Clip` → `overflow_hidden` (clip), `Auto`/`Scroll` → `overflow_scroll` (List container), `Visible` → default. Probe-verified Auto→List.
- [x] accepted (runtime limit): `role`/`aria_label` — JsEl emits no accessibility tree, so the fields are intentionally not applied. Mirrors GPUI.
- accepted: interaction n/a (layout primitive, no events).

## Specimen parity

- Svelte covers: Default (no padding), With padding (lg), Fixed dimensions (12×6rem + md padding), Overflow hidden (10×3rem clipped) — all 4 contract groups.
- GPUI covers: all 4 groups, now **spec-driven** — Fixed (`with_width("12rem").with_height("6rem")`) and Overflow (`with_width("10rem").with_height("3rem").with_overflow(Hidden)`) resolve dimensions through the Box spec, not hand-coded `div().w(px(...))` wrappers. (Preview not build-verified here per shared-target-lock rule.)
- Jetstream covers: all 4 groups, spec-driven — Default (no padding), With padding (md), Large padding (lg), Fixed 12×6rem (md padding), Overflow hidden (10×3rem). Rebuilt now that `js_box` resolves width/height + overflow modes. (Preview not build-verified here.)

## Notes

- Per repo policy, a specimen that hand-codes dimensions instead of resolving them through the Spec is "worse than no specimen". The GPUI Fixed/Overflow groups and the absence of Jetstream Fixed/Overflow groups both trace to the same root: Rust `js_box`/`Box` don't resolve non-px dimensions. Fix the components first, then make specimens spec-driven.
