<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok pass=meta-bar-rust-ports-closed -->
# Parity: MetaBar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/meta-bar.md`
- Svelte (authoritative): `packages/svelte/components/src/MetaBar.svelte`
- GPUI: `packages/gpui/components/src/primitives/meta_bar.rs`
- Jetstream: `packages/jetstream/components/src/meta_bar.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MetaBarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/meta_bar.rs` (`render`) · jetstream `packages/jetstream/preview/src/specimens/meta_bar.rs`

## Contract ↔ Svelte

Both props match (`ariaLabel` default `null`, `showSeparators` default `true`). The divergence is undocumented Svelte behavior the contract does not describe, plus a per-child opt-out the contract is silent on.

- [x] FIXED — Svelte calls `setPillContext({ size: "md", typography: "inherit" })` (`MetaBar.svelte:17-20`); now documented in contract §4 (pill typography context injection).
- [x] FIXED — Separator mechanism is per-child opt-in via `[data-separator="true"]`, not blanket "between adjacent items"; pill suppression (`:has(.poodle-pill)`) suppresses the dot + leading padding (`MetaBar.svelte:45-68`). Contract §4 now spells out the `data-separator` opt-out + pill suppression (gap collapse + label hide), and §6 semantics adds the `data-separator` attribute note.
- [x] FIXED — Separator visuals (`0.25rem` dot, 72%-mix `--poodle-color-text-secondary`, `1rem`/`0.75rem` leading padding) now in a new contract §7 Token Usage section with root/separator-dot/pill-suppression/responsive tables.
- [x] FIXED — Root `gap: 0.5rem` + `line-height: 1.4` (`MetaBar.svelte:38`) now in contract §7 root table.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] FIXED — dot radius now `resolve_radius(theme, "radius.pill")` (was `rounded(px(999.0))`). Token-sourced.
- [x] FIXED — the 72% separator mix is now a named `SEPARATOR_DOT_MIX` const with a contract §7 citation (was inline `* 0.72`). No token carries the 72% factor (it is a contract literal) — noted token gap, see Notes.
- [x] FIXED — separator is now per-child opt-in. `MetaBar` stores `(child, separator)` pairs; the dot draws only when `idx > 0 && show_separators && separator`. New `with_child_sep` / `with_children_sep` builders + `MetaItem::separator()` carry the `data-separator` intent. The specimen opts the `Pill` out via `with_child_sep(pill, false)` (Svelte `:has(.poodle-pill)`).
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but not emitted.
- accepted: dot uses `rem_to_px(0.25)` (derived from rem, fine) and gap resolves from `space.inline.sm` token — token-correct.

## Jetstream gap (vs Svelte + contract)

- [x] FIXED — dot radius now `resolve_radius(theme, "radius.pill")` (was `rounded(999.0)`). Token-sourced.
- [x] FIXED — 72% mix now the named `SEPARATOR_DOT_MIX` const (was inline `* 0.72`). Same contract-literal token gap as GPUI.
- [x] FIXED — separator is now per-child opt-in via new `js_meta_bar_sep(spec, theme, Vec<(JsEl, bool)>)`. `js_meta_bar` delegates with all-`true` flags (back-compat). Dot draws only when `idx > 0 && show_separators && separator`. Specimen demonstrates pill suppression (`separator=false`). Covered by `per_child_opt_out_suppresses_its_dot` probe test.
- accepted: no ARIA channel for `aria_label`.
- accepted: gap resolves from `space.inline.sm` token, dot size via `rem_to_px(0.25)` — token/rem-derived.

### Probe tests (Jetstream, `meta_bar.rs`)

- `renders_items_with_separator_dots_between_them` — labels (uppercased) + values render; dot bg = text-secondary @ 72%.
- `show_separators_false_suppresses_dots` — no dot bg when separators off.
- `per_child_opt_out_suppresses_its_dot` — `js_meta_bar_sep` with `false` flags draws no dots.
- `first_child_never_gets_a_leading_dot` — single item, no leading dot.

## Specimen parity

- Svelte covers: Header metadata (separators on, with `Code`+copy, `Pill`, plain `MetaItem`s), No separators, Inherited typography (inline inside running copy). (`MetaBarSpecimen.svelte`)
- GPUI covers: Header metadata (now with `Pill` separator-suppression via `with_child_sep(pill, false)`), No separators. Inherited-typography lives in the MetaItem specimen (`render_meta_item`). (`meta_bar.rs` `render`)
- Jetstream covers: With separators (default), **Rich children (Code + Pill suppression)** (new), Without separators, **Inherited typography** (new, inline in copy), Single item. (`jetstream/.../specimens/meta_bar.rs`)

## Notes

- `consv=fixed`: contract documents the per-`MetaItem` `data-separator` opt-out, pill suppression, pill-context typography injection, and the full separator/root token table (§4 + §7).
- Per-child separator intent now rides on the builder (`with_child_sep` / `js_meta_bar_sep` take `(child, separator)` pairs) rather than `MetaBarSpec`. `MetaItem` carries its own `separator` (new `MetaItemSpec.separator` field, default `true`); the caller forwards it to the bar. Pill suppression is expressed by passing `separator=false` for pill children (the Rust ports cannot introspect child subtrees for `.poodle-pill` the way Svelte's `:has()` does, so the opt-out is explicit at the call site — noted delta).
- **Token gap**: no semantic token carries the `72%` separator-dot mix factor — it is a contract literal (`color-mix(... 72%, transparent)`). Both ports name it `SEPARATOR_DOT_MIX` rather than inlining a magic number. Add a `color.text.separatorDot` (or opacity) token if this factor should be themeable.
- Neither Rust port injects pill typography context (no equivalent of `setPillContext`); pills inside the bar will not pick up MetaBar's inherit sizing — unchanged, structural.
