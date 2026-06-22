<!-- parity consv=ok gpui=2 jetstream=1 specimen=ok | jet-specimen: real js_icon groups — sizes (sm/md/lg), color inheritance (6 tones), accessibility (aria vs decorative), 48-icon gallery; mirrors gpui; both previews build clean -->
# Parity: Icon

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/icon.md`
- Svelte (authoritative): `packages/svelte/components/src/Icon.svelte`
- GPUI: `packages/gpui/components/src/primitives/icon.rs`
- Jetstream: `packages/jetstream/components/src/icon.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/IconSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/icon.rs` · jetstream `packages/jetstream/preview/src/specimens/icon.rs`

## Contract ↔ Svelte

Svelte faithfully implements the contract. Anatomy (single `<svg>` root), SVG
attributes (viewBox/width/height/fill/stroke/stroke-width/linecap/linejoin), all
five sizes, decorative-vs-accessible role switch on `ariaLabel`, `name`
deprecated alias, `data-size`/`data-density`. All match.

- `size` default: contract §3 says `null`; Svelte default is `null` and resolves
  via `resolveSemanticControlSize(sizeScale, sizeRole)` → md. Consistent.
- No divergence found. `consv=ok`.

## GPUI gap (vs Svelte + contract)

- [ ] `with_px_size(size_px: f32)` accepts a raw float (`icon.rs:78`); callers
  pass arbitrary px (e.g. `list_card_counter.rs` passes `icon_size_rem()` → px).
  Acceptable when fed a token-derived value, but the public `f32` API invites
  hardcoding — prefer a size-token-based setter so no caller can inject a literal.
- [ ] `button_icon()` hardcodes `IconSize::Sm` (`icon.rs:67`) as the in-button
  size; verify against contract supporting-size mapping rather than a fixed enum.
- accepted: no ARIA (gpui has no accessibility API) — decorative/accessible role
  switch is not emitted; color comes from `color.icon.primary` token (`icon.rs:98`).
- note: GPUI sets `text_color` explicitly because `svg()` does not inherit
  currentColor — accepted platform delta (contract §12 allows path-render method).

## Jetstream gap (vs Svelte + contract)

- [ ] Decorative vs accessible mode not modeled — `spec.aria_label` is ignored
  (`icon.rs:21`). Accepted as runtime delta (no a11y channel) but note once.
- size resolves cleanly from `spec.size_token()` (`icon.rs:16`); no hardcoded px.
- accepted: SVG rasterized by engine IconCache, tinted via inherited `text_color`
  (contract §12 allows platform-owned render method).

## Specimen parity

- Svelte covers: Sizes (xs–xl rows), Color inheritance (4 tones), All icons grid
  with click-to-copy (`IconSpecimen.svelte`).
- GPUI covers: sizes + color inheritance + grid (`specimens/icon.rs`). — verify
  click-to-copy parity (interactive).
- Jetstream covers: sizes + color inheritance. — missing: **All icons** registry
  grid with name labels (contract §13 "All icons").

## Notes

- Contract §8 lists `display: inline-block` / `vertical-align: middle` /
  `flex-shrink: 0`; Rust targets use flex containers instead — acceptable layout
  delta in non-CSS runtimes.
- `density` prop has no visual effect in any target (icon has no spacing); only
  Svelte emits `data-density`. Fine.
