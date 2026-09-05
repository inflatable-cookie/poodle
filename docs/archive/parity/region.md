<!-- parity consv=ok gpui=1 jetstream=0 specimen=ok | pass: Jetstream js_region rebuilt to match GPUI + contract — dropped children param (contract §3), flex-centered wrapper, 0.125rem border at contract-exact width, label-size token, uppercased label, custom-color override on border+label; specimen rewritten to Default/Custom colors/Layout composition; probe tests added -->
# Parity: Region

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/region.md`
- Svelte (authoritative): `packages/svelte/components/src/Region.svelte`
- GPUI: `packages/gpui/components/src/primitives/region.rs`
- Jetstream: `packages/jetstream/components/src/region.rs`
- Spec: `packages/contracts/components/src/region.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/RegionSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/region.rs` · jetstream `packages/jetstream/preview/src/specimens/region.rs`

## Contract ↔ Svelte

Svelte matches the contract: `label`/`color`/`minHeight` props with matching defaults, `role="presentation"`, dashed border, uppercase muted label, no child content. No divergence.

- Contract §7 names the border token `--poodle-color-border-default`; Svelte uses exactly that. OK.
- Contract §3 explicitly states "Region does not accept child content." Svelte honours this (renders only the label). **Jetstream violates this** — see below.

## GPUI gap (vs Svelte + contract)

GPUI is close: dashed border via `.border_2().border_dashed()`, padding/radius/label-size from tokens, custom color overrides both border and label, uppercases the label.

- [ ] `line_height(relative(1.5))` and `FontWeight::SEMIBOLD` are hardcoded styling (`region.rs:92-94`). Minor — contract has no line-height token and Svelte uses `font-weight: 600`; acceptable but note. Letter-spacing `0.05em` from Svelte is omitted (no token); accepted delta.
- accepted: no ARIA (`role="presentation"` not emitted; gpui has no accessibility API). Decorative-only behavior preserved.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED Border width now `rem_to_px(0.125)` (contract-exact 0.125rem), not solid 1px. **NOTE (JsEl gap):** JsEl exposes only `border(width)` — no dashed border-style channel, so the dash *pattern* is not yet expressible; rendered as a solid border at the correct width. Dash-style support is a runtime follow-up.
- [x] FIXED Dropped the `children` param — `js_region(spec, theme)` now renders only the label (contract §3). Specimen rewritten to label-only states.
- [x] FIXED Label size resolves from `spec.label_text_size_token()` (`--poodle-typography-label-size`); probe-tested against the resolved token value.
- [x] FIXED Label uppercased via `spec.label.to_uppercase()` (contract §2); probe-tested.
- [x] FIXED `spec.color` custom-color override now adopted by both border and label (mirrors GPUI — raw hex passed through `resolve_color`, which parses `#`-prefixed strings). Wrapper is flex-column, items-center, justify-center (matches GPUI layout).
- accepted: interaction n/a (Region is non-interactive); no ARIA channel.

## Specimen parity

- Svelte covers: Default, **Custom colors** (4 colored regions), **Layout composition** (nav/toolbar/content grid).
- GPUI covers: Default, Custom colors (4), Layout composition. — full parity with Svelte.
- Jetstream covers: Default, **Custom colors** (4 colored regions), **Layout composition** (nav/toolbar/content). — full parity with Svelte/GPUI; rewritten to label-only (no forbidden children).

## Notes

- ~~Jetstream specimen passing text children into `js_region` bakes the contract violation into the integration test~~ — **resolved**: `children` param removed; specimen rewritten to Default/Custom colors/Layout composition (label-only).
- `RegionSpec::min_height_px` is an `f32` px prop (default 64.0 = 4rem). It is a caller-supplied dimension, not a token target, so px storage is acceptable; the contract default (`4rem`) is preserved.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/region.rs:31`) — specimen chrome.
