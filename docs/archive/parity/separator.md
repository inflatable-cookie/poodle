<!-- parity consv=ok gpui=0 jetstream=0 specimen=ok | pass: GPUI+Jetstream specimens backfilled — horizontal+vertical, subtle+default tone, decorative+semantic, all real Separator/js_separator; both previews build clean -->
<!-- pass: stroke from border-width-default token on both targets (gpui f32::from(Pixels); jet resolve_px); subtle 72% mix sourced from new SeparatorSpec::subtle_mix_ratio() (no magic float); decorative now read (no AX channel); +4 jet probe tests (orientation/thickness/subtle/default tone) -->
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

- [x] Stroke width now `px(f32::from(resolve_px(theme, spec.resolved_stroke_width())))` — token-resolved (`border.width.default` → 1px), not the `px(1.0)` literal.
- [x] Subtle-tone alpha now `raw_color.a * spec.subtle_mix_ratio()` — the `0.72` lives on the spec (new `SeparatorSpec::subtle_mix_ratio()`, shared by both targets; Default tone → 1.0). No magic float in component code. (Note: no dedicated semantic token exists for the 72% mix; it is a contract-fixed constant carried on the spec.)
- [x] `decorative` now read (`let _is_semantic = !self.spec.decorative;`) so the prop is no longer dead. No visual difference per contract §4; it is the hook for any future AX channel.
- accepted: no ARIA (gpui has no accessibility API) — decorative/semantic role cannot be emitted.

## Jetstream gap (vs Svelte + contract)

- [x] Stroke width now `resolve_px(theme, spec.resolved_stroke_width())` — token-resolved (`border.width.default` → 1px), not the `rem_to_px(0.0625)` literal. Color base is `resolve_color(theme, spec.resolved_color())` tinted by `spec.subtle_mix_ratio()` (subtle 0.72 / default 1.0).
- [x] `decorative` now read (`let _is_semantic = !spec.decorative;`) so the field is no longer dead. No visual effect (contract §4); no AX channel in Jetstream.
- accepted: no ARIA channel for decorative/semantic role (no accessibility API).

## Specimen parity

- Svelte covers: Horizontal (default subtle), Vertical, Decorative. (Note: Svelte does not show the `default` tone explicitly — both tones share the divider, default tone untested in Svelte specimen.)
- GPUI covers: Horizontal (default), Tone emphasis (subtle + default), Vertical, Vertical (default tone), Decorative, Semantic (`decorative=false`) (`gpui/.../separator.rs`, 6 groups). All real `Separator::from_spec`; both tones, both orientations, and both decorative/semantic modes exercised. Specimen coverage `ok`.
- Jetstream covers: Horizontal (default), Tone emphasis (subtle + default), Vertical (subtle + default dividers), Decorative, Semantic (`decorative=false`) (`jetstream/.../separator.rs`, 5 groups). All real `js_separator`. Specimen coverage `ok`.

## Notes

- `resolved_stroke_width()` on the spec is the intended token path for the 1px rule; both Rust targets bypass it. Single shared fix pattern across both.
- `decorative` is a no-op in all three runtimes for visual output (it only matters for AX), so its dead state in Rust is low-severity but worth wiring if an AX channel appears.
- Specimen coverage is fragmented: no single target shows all of {subtle, default, decorative, vertical}. Jetstream shows default tone; Svelte/GPUI show decorative. Worth unifying so every target demonstrates the same set.
