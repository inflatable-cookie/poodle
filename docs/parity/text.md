<!-- parity consv=ok gpui=0 jetstream=1 specimen=gap | pass: TextSpec gained `spacing` (compact stack-sm gap); both targets render it; gpui specimen detoned (stack-sm gap token, +muted/warning/sm/weights/inline/clamp); clamp+element stay accepted runtime limits; jet probe tests cover size/tone/weight/spacing -->
# Parity: Text

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/text.md`
- Svelte (authoritative): `packages/svelte/components/src/Text.svelte`
- GPUI: `packages/gpui/components/src/primitives/text.rs`
- Jetstream: _missing_ — no `packages/jetstream/components/src/text.rs`; `lib.rs` has no `text` module (only `text_input`, line 89)
- Specimens: svelte `packages/svelte/preview/src/specimens/TextSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/text.rs` · jetstream _none_

## Contract ↔ Svelte

Props, tones, sizes, weights, leading, spacing, clamp, and ARIA posture all
match between contract §2–§4 and `Text.svelte`. No divergence.

- `as` / `tone` / `size` / `weight` / `leading` / `spacing` / `clamp` unions and
  defaults in `Text.svelte:4-24` exactly match contract §2.
- Tone→token map (`Text.svelte:101-116`) matches contract §3: secondary+muted →
  `--poodle-color-text-secondary`; success/danger/warning → `--poodle-color-status-*`.
- Sizes (`0.75`/`0.8125`/`0.875rem`), weights (`500`/`600`/`700`), leading
  (`1.5`/`1.6`), `spacing="compact"` grid with `--poodle-space-stack-sm`, and
  `clamp` 1/2/3 box-clamp all match contract §3 (`Text.svelte:49-99`).
- ARIA: Svelte adds no roles, per contract §4. Element choice via `as` only.
- consv=ok.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] DONE: `spacing` support — added `TextSpacing` enum + `spacing` field + `spacing_gap_token()` to `TextSpec`; GPUI builder resolves `space.stack.sm` and applies `flex().flex_col().gap()` when compact.
- [ ] accepted (runtime limit): `clamp` sets `overflow_hidden` only and ignores the 1/2/3 line count — GPUI/gpui has no `-webkit-line-clamp` analogue, so the N-line cap degrades to clipping. Matches Jetstream; not closable without a line-clamp API.
- [ ] accepted (N/A): `as`/element variants — GPUI has no DOM/semantics layer, so `p`/`span`/`div` have no rendering difference (Svelte's only delta is the HTML tag). Builder collapses to one node by design, same as Jetstream. `spec.element` is carried for portability.
- [x] DONE: specimen token violation fixed — `gap(px(10.0))` → `resolve_px(theme, "space.stack.sm")`; specimen also gained muted+warning tones, sm size, medium/bold weights, inline span, compact-spacing, and clamp demos.
- accepted: no ARIA (gpui has no accessibility API) — matches contract §4 (Text adds none anyway).
- note: `font_size_rem()`/`line_height()` return f32 literals in `TextSpec` (`text.rs:128-141`); these are spec-owned contract constants (§3), and the builder resolves them via `rem_to_px` (`text.rs:40`) rather than hardcoding in the component — acceptable, not a literal-in-component violation.

## Jetstream gap (vs Svelte + contract)

Component is entirely absent — dominant gap.

- [x] DONE: created `packages/jetstream/components/src/text.rs` with `js_text(spec, theme)`, registered `pub mod text;` in lib.rs.
- [x] DONE: resolves color from `TextSpec::color_token()` (all six tones) + `font_size_rem()`/`line_height()` via `rem_to_px`, weights mapped 400/500/600/700. Probe-tested.
- [x] DONE: `spacing="compact"` — `js_text` wraps the label in a `flex_col().gap()` carrying the resolved `space.stack.sm` when compact (`TextSpec` now has the `spacing` field). `clamp` still degrades to overflow-hidden (no JsEl line-clamp — accepted runtime limit, same as GPUI).
- [ ] Add `packages/jetstream/preview/src/specimens/text.rs` (+ register in `specimens/mod.rs`) covering tones, sizes, leading, weight, inline, clamp, compact spacing; none exists. (Preview can't be build-verified in this environment.)
- accepted: no ARIA channel — matches contract §4.
- note: probe tests added in `text.rs` cover content→label, danger tone→status color, size→font px, weight→css value, and compact→gapped-column wrap.

## Specimen parity

- Svelte covers: Tones (all 6: default/secondary/muted/success/danger/warning), Sizes (xs/sm/md), relaxed leading, weights (medium/semibold/bold), inline `as="span"`, clamp=2 (`TextSpecimen.svelte:7-37`).
- GPUI covers: 4 tones (default/secondary/success/danger), xs size, relaxed+semibold combo (`gpui/.../specimens/text.rs:11-33`). — missing: **muted** + **warning** tones, **sm** size, **medium**/**bold** weights, **inline** (`as="span"`), **clamp** demo.
- Jetstream covers: nothing — **no specimen exists**. — missing: all states.

## Notes

- consv=ok: contract and Svelte are fully aligned for Text — no contract edits needed.
- Spec gap underlies both Rust targets: `TextSpec` omits the `spacing` prop (`packages/contracts/components/src/text.rs:69-78`) that contract §2 and `Text.svelte:21` require, so neither GPUI nor Jetstream can implement compact spacing until the spec adds it.
- Jetstream is the biggest gap by far: zero component + zero specimen vs a complete, contract-faithful Svelte reference.
- GPUI `clamp` is render-degenerate (overflow-hidden only); it does not visually clamp to N lines the way Svelte's `-webkit-line-clamp` does.
