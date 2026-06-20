<!-- parity consv=fixed gpui=3 jetstream=3 specimen=gap -->
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

- [ ] Hardcoded radius literal `rounded(px(999.0))` at `meta_bar.rs:72` — pill-radius for the dot should resolve from a token, not a raw `999.0`.
- [ ] Hardcoded opacity literal `separator_color.a * 0.72` at `meta_bar.rs:73-74` — the 72% separator mix is a magic number; move to a token or named constant. Svelte uses `color-mix(... 72%, transparent)`; the value should be sourced, not inlined.
- [ ] Separator is positional (`idx > 0`), not per-child opt-in. GPUI draws a dot before every non-first child regardless of child kind, so the Svelte pill-suppression (`:has(.poodle-pill)`) and `data-separator` opt-out are absent (`meta_bar.rs:70-79`). Dots render before pills, unlike Svelte.
- accepted: no ARIA (gpui has no accessibility API) — `aria_label` stored on spec but not emitted.
- accepted: dot uses `rem_to_px(0.25)` (derived from rem, fine) and gap resolves from `space.inline.sm` token (`meta_bar.rs:67`) — token-correct; only `999.0`/`0.72` are literals.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded radius literal `rounded(999.0)` at `meta_bar.rs:45` — same pill-radius literal as GPUI; resolve from a token.
- [ ] Hardcoded opacity literal `separator_color.a * 0.72` at `meta_bar.rs:29` — same 72% magic number as GPUI; source it.
- [ ] Separator is positional (`idx > 0`), not per-child opt-in (`meta_bar.rs:39-48`) — no pill suppression, no `data-separator` opt-out; dots render before every non-first child including pills.
- accepted: no ARIA channel for `aria_label`.
- accepted: gap resolves from `space.inline.sm` token (`meta_bar.rs:26`), dot size via `rem_to_px(0.25)` — token/rem-derived, fine.

## Specimen parity

- Svelte covers: Header metadata (separators on, with `Code`+copy, `Pill`, plain `MetaItem`s), No separators, Inherited typography (inline inside running copy). (`MetaBarSpecimen.svelte`)
- GPUI covers: Header metadata, No separators — **missing: Inherited-typography group**, and no specimen exercises a `Pill` sitting after a `MetaItem` to show separator-suppression (pill gets a dot here, unlike Svelte). (`meta_bar.rs` `render`)
- Jetstream covers: With separators (default), Without separators, Single item — **missing: rich `Code`/`Pill` children** (only text items), **missing: Inherited-typography group**. (`jetstream/.../specimens/meta_bar.rs`)

## Notes

- `consv=fixed`: contract now documents the per-`MetaItem` `data-separator` opt-out, pill suppression, pill-context typography injection, and the full separator/root token table (§4 + §7). Both Rust ports still copy the simpler "dot before every non-first child" model, so they remain a gap against Svelte for any `Pill` child — tracked in the Jetstream/GPUI sections, not contract↔Svelte.
- `MetaBarSpec` has no field to carry per-child separator intent, so even fixing the Rust ports needs the child elements (not just the bar) to signal opt-out — a structural gap, not a one-line fix.
- Neither Rust port injects pill typography context (no equivalent of `setPillContext`); pills inside the bar will not pick up MetaBar's inherit sizing.
