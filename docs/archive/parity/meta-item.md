<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok pass=meta-item-rust-ports-closed -->
# Parity: MetaItem

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/meta-item.md`
- Svelte (authoritative): `packages/svelte/components/src/MetaItem.svelte`
- GPUI: `packages/gpui/components/src/primitives/meta_item.rs`
- Jetstream: `packages/jetstream/components/src/meta_item.rs`
- Specimens: svelte — **no standalone `MetaItemSpecimen.svelte`**; meta-item is exercised only inside `MetaBarSpecimen.svelte` · gpui — **no standalone specimen file**; `render_meta_item` lives inside `packages/gpui/preview/src/specimens/meta_bar.rs` · jetstream `packages/jetstream/preview/src/specimens/meta_item.rs` (standalone exists)

## Contract ↔ Svelte

Props mostly match (`label` default `null`, `ariaLabel` default `null`, `typography` default `"body"`). Two divergences: an undocumented prop and a wrong inherit ratio.

- [x] FIXED (contract + spec) — Svelte adds `separator?: boolean` (default `true`) → emits `data-separator`. Contract §2/§6 documents it, and `MetaItemSpec` now has the `separator` field (default `true`) + `with_separator()` builder. GPUI exposes `MetaItem::separator(bool)`; both ports forward the flag into MetaBar's per-child separator channel.
- [x] FIXED (contract + spec) — inherit label font-size now `0.6875` on both `Default` and `Inherit` (`MetaItemSpec::label_font_size_rem()`), matching the corrected contract §7 (`0.6875em`). The old `0.7857` is gone — GPUI/Jetstream inherit the correct ratio.
- [x] FIXED (contract + spec) — inherit gap now `0.375` on both branches (`gap_rem()`), matching Svelte (`0.375em`). Old `0.4286` removed.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED — `MetaItemSpec` gained `separator` (default `true`); GPUI `MetaItem::separator(bool)` + `separator_intent()` carry it to MetaBar's per-child channel.
- [x] FIXED — label weight now `FontWeight(spec.label_font_weight() as f32)` where `label_font_weight()` reads `typography.label.weight` (= 500) from the typed const. Was hardcoded `FontWeight::SEMIBOLD` (600, wrong). Mirrors `eyebrow.rs`'s token-resolved weight pattern.
- [x] FIXED — inherit label ratio corrected to `0.6875` in the spec, so GPUI inherits the right size.
- [x] FIXED (partial) — label now sets `font_family` (`typography.label.family` via `label_family_token()`) and `line_height(1)`; value sets `font_family` (`typography.body.family`) and `line_height(1.4)`. `letter-spacing: 0.08em` remains an **accepted GPUI delta** (no text-style channel — same as `eyebrow.rs`).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored, not emitted.
- accepted: placeholder `"Value"` fallback when no value supplied is preview-only convenience.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED — `MetaItemSpec.separator` field added; the bar reads per-child intent via `js_meta_bar_sep`. (Jetstream `js_meta_item` itself does not emit a separate `data-separator` node — the flag is passed at the bar call site.)
- [x] FIXED — label weight now `spec.label_font_weight()` (= `typography.label.weight` = 500 from the typed const), was literal `text_weight(600)`. Colors now resolve via `spec.label_color_token()` / `value_color_token()`.
- accepted: label/value `font-family`, `letter-spacing`, `line-height` omitted — approved Known Delta in contract §10 (Jetstream `JsEl` text surface lacks those controls).
- accepted: interaction is N/A (non-interactive).
- note: gap/size resolve from spec `gap_rem()`/`label_font_size_rem()`/`value_font_size_rem()` via `rem_to_px` — token/spec-derived, no raw px literals. Inherit label ratio now `0.6875` (corrected).

### Probe tests (Jetstream, `meta_item.rs`)

- `renders_uppercased_label_and_value` — label uppercased + value rendered.
- `label_uses_label_secondary_tone` — label color token (secondary) resolves and differs from value (primary) tone.
- `value_only_renders_without_label` — value-only item renders exactly one text node (no leading label).
- `inherit_typography_scales_label_and_value` — `Inherit` value size exceeds `Default`; label sizes present under both.

## Specimen parity

- Svelte covers (inside `MetaBarSpecimen.svelte`): labeled item, value-only item (no label), rich value (`Code` inline + copy), inherit typography. **No dedicated MetaItem specimen** — coverage is incidental to MetaBar.
- GPUI covers (`render_meta_item` in `meta_bar.rs`): Labeled, Rich Value (`Pill` + text), Inherit typography. — label-only lives in the MetaBar specimen; coverage is in the shared file rather than its own.
- Jetstream covers (`specimens/meta_item.rs`): label+text value, value-only, label-only, multiple side-by-side, inherit typography — **most complete specimen of the three**.

## Notes

- `specimen=ok`: Svelte has no standalone MetaItem specimen (coverage is incidental to MetaBar); GPUI's lives inside `meta_bar.rs`; Jetstream has the broadest dedicated file (5 groups).
- `consv=fixed`: contract §7 inherit label-size + gap match Svelte (`0.6875em` / `0.375em`); §2/§6 carry the `separator` prop + `data-separator` attribute. The spec is now realigned: `MetaItemSpec.separator` exists, and `label_font_size_rem()` / `gap_rem()` return the corrected `0.6875` / `0.375` for `Inherit` (one spec fix realigned both ports).
- **Token gap (label weight)**: `typography.label.weight` (= 500) has no string-resolver arm in either adapter, so `label_font_weight()` reads the typed const `poodle_tokens::typed::semantic::TYPOGRAPHY_LABEL_WEIGHT` directly — same precedent as `Code` reading the code adjustment ratio. If a string-resolver channel for numeric typography tokens is added later, switch to it.
