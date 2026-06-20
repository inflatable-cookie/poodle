<!-- parity consv=ok gpui=3 jetstream=2 specimen=gap -->
# Parity: Separator

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/separator.md`
- Svelte (authoritative): `packages/svelte/components/src/Separator.svelte`
- GPUI: `packages/gpui/components/src/primitives/separator.rs`
- Jetstream: `packages/jetstream/components/src/separator.rs`
- Spec: `packages/contracts/components/src/separator.rs` (`SeparatorSpec`)
- Specimens: svelte `packages/svelte/preview/src/specimens/SeparatorSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/separator.rs` · jetstream `packages/jetstream/preview/src/specimens/separator.rs`

## Contract ↔ Svelte

Svelte matches the contract exactly. All three props (`orientation` default `"horizontal"`, `decorative` default `true`, `tone` default `"subtle"`), the single-div anatomy, both data attributes, and the conditional ARIA (`aria-hidden` when decorative / `role="separator"` + `aria-orientation` when semantic) are present and correct. Token target `--poodle-separator-color` with the 72% color-mix subtle and full `border-default` default tone all match contract §8.

- No divergences. **consv=ok.**

## GPUI gap (vs Svelte + contract)

- [ ] Hardcoded stroke width `px(1.0)` (float) at `separator.rs:74,77` — `SeparatorSpec` exposes `resolved_stroke_width()` (→ `BORDER_WIDTH_DEFAULT`); resolve that token to px instead of the literal `1.0`.
- [ ] Hardcoded subtle-tone alpha `raw_color.a * 0.72` at `separator.rs:61` — the `0.72` magic number duplicates Svelte's color-mix; pull from a token (or a shared subtle-mix constant) rather than a raw float literal in component code.
- [ ] `decorative` field unused — the prop is forwarded via the `decorative()` builder (`separator.rs:42`) but never read in `into_element`; no semantic vs decorative distinction is made. Contract §6 requires it; GPUI accessibility is accepted-absent, but the field being dead should at least drive a non-focusable/structural marker if any AX channel ever lands.
- accepted: no ARIA (gpui has no accessibility API) — decorative/semantic role cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded stroke width `rem_to_px(0.0625)` at `separator.rs:43` — `SeparatorSpec::resolved_stroke_width()` exists (→ `BORDER_WIDTH_DEFAULT`); resolve the token instead of the literal `0.0625` rem.
- [ ] `decorative` field unused — never read in `js_separator`; no semantic vs decorative handling. Accepted that interaction/AX lives outside the component, but the field is dead.
- accepted: no ARIA channel for decorative/semantic role (no accessibility API).

## Specimen parity

- Svelte covers: Horizontal (default subtle), Vertical, Decorative. (Note: Svelte does not show the `default` tone explicitly — both tones share the divider, default tone untested in Svelte specimen.)
- GPUI covers: Horizontal (subtle only), Vertical (two dividers), Decorative. — missing: **`default` tone** group (only subtle rendered).
- Jetstream covers: Horizontal (Subtle + **Default tone**), Vertical. — missing: **Decorative** group. (Jetstream is the only target exercising `default` tone.)

## Notes

- `resolved_stroke_width()` on the spec is the intended token path for the 1px rule; both Rust targets bypass it. Single shared fix pattern across both.
- `decorative` is a no-op in all three runtimes for visual output (it only matters for AX), so its dead state in Rust is low-severity but worth wiring if an AX channel appears.
- Specimen coverage is fragmented: no single target shows all of {subtle, default, decorative, vertical}. Jetstream shows default tone; Svelte/GPUI show decorative. Worth unifying so every target demonstrates the same set.
