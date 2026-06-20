<!-- parity consv=ok gpui=4 jetstream=2 specimen=gap -->
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

- [ ] No `spacing` support — `TextSpec` (`packages/contracts/components/src/text.rs:69-78`) has no `spacing` field and the builder never renders the compact grid; add `spacing` to spec + a `--poodle-space-stack-sm`-resolved `gap` branch in `text.rs:39-46`.
- [ ] `clamp` only sets `overflow_hidden` (`text.rs:44`) and ignores the 1/2/3 line count — no line-clamp limit applied; resolve the clamp value into a max-height/line cap so `clamp=1|2|3` differ.
- [ ] No `as`/element variants — builder always emits `div()` (`text.rs:39`); `TextSpec.element` (`text.rs:72`) is never read, so `as="p"|"span"|"div"` collapse to one node. Honor `spec.element`.
- [ ] Specimen-side token violation: hardcoded `gap(px(10.0))` at `packages/gpui/preview/src/specimens/text.rs:10` — resolve from a stack-spacing token, not raw `10.0`.
- accepted: no ARIA (gpui has no accessibility API) — matches contract §4 (Text adds none anyway).
- note: `font_size_rem()`/`line_height()` return f32 literals in `TextSpec` (`text.rs:128-141`); these are spec-owned contract constants (§3), and the builder resolves them via `rem_to_px` (`text.rs:40`) rather than hardcoding in the component — acceptable, not a literal-in-component violation.

## Jetstream gap (vs Svelte + contract)

Component is entirely absent — dominant gap.

- [x] DONE: created `packages/jetstream/components/src/text.rs` with `js_text(spec, theme)`, registered `pub mod text;` in lib.rs.
- [x] DONE: resolves color from `TextSpec::color_token()` (all six tones) + `font_size_rem()`/`line_height()` via `rem_to_px`, weights mapped 400/500/600/700. Probe-tested.
- [ ] Implement `spacing="compact"` (stack-sm gap grid) and `clamp` 1/2/3 line limiting — and add `spacing` to `TextSpec` first (missing from spec). `clamp` currently degrades to overflow-hidden (same as GPUI).
- [ ] Add `packages/jetstream/preview/src/specimens/text.rs` covering tones, sizes, leading, weight, inline, clamp; none exists.
- accepted: no ARIA channel — matches contract §4.

## Specimen parity

- Svelte covers: Tones (all 6: default/secondary/muted/success/danger/warning), Sizes (xs/sm/md), relaxed leading, weights (medium/semibold/bold), inline `as="span"`, clamp=2 (`TextSpecimen.svelte:7-37`).
- GPUI covers: 4 tones (default/secondary/success/danger), xs size, relaxed+semibold combo (`gpui/.../specimens/text.rs:11-33`). — missing: **muted** + **warning** tones, **sm** size, **medium**/**bold** weights, **inline** (`as="span"`), **clamp** demo.
- Jetstream covers: nothing — **no specimen exists**. — missing: all states.

## Notes

- consv=ok: contract and Svelte are fully aligned for Text — no contract edits needed.
- Spec gap underlies both Rust targets: `TextSpec` omits the `spacing` prop (`packages/contracts/components/src/text.rs:69-78`) that contract §2 and `Text.svelte:21` require, so neither GPUI nor Jetstream can implement compact spacing until the spec adds it.
- Jetstream is the biggest gap by far: zero component + zero specimen vs a complete, contract-faithful Svelte reference.
- GPUI `clamp` is render-degenerate (overflow-hidden only); it does not visually clamp to N lines the way Svelte's `-webkit-line-clamp` does.
