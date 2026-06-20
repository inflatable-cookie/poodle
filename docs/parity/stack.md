<!-- parity consv=gap gpui=3 jetstream=6 specimen=gap -->
# Parity: Stack

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/stack.md`
- Svelte (authoritative): `packages/svelte/components/src/Stack.svelte`
- GPUI: `packages/gpui/components/src/primitives/stack.rs`
- Jetstream: `packages/jetstream/components/src/stack.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/StackSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/stack.rs` · jetstream `packages/jetstream/preview/src/specimens/stack.rs`
- Spec struct: `packages/contracts/components/src/stack.rs` (`StackSpec`); token maps `packages/contracts/components/src/types.rs:51-105` (`PaddingScale`)

## Contract ↔ Svelte

Svelte carries sizing props and default behaviors the contract does not document. Svelte is authoritative — update the contract.

- Svelte adds `width?: string | null` and `height?: string | null` (defaults `null`) → composed into inline `width`/`height` (`Stack.svelte:37-38,56-57`). Not in contract §3. **Fix: add to contract props.**
- Svelte adds `minWidth?: string | null` and `minHeight?: string | null` (defaults `null`) → inline `min-width`/`min-height` overrides (`Stack.svelte:39-40,58-59`). Contract §8 only specifies static `min-width:0`/`min-height:0`. **Fix: add the overridable props to contract.**
- Svelte adds `overflow?: OverflowMode` (default `"visible"`, values `visible|hidden|clip`) → inline `overflow` (`Stack.svelte:41,60`). Not in contract. **Fix: add `overflow` prop + `OverflowMode` to §3 shared types.**
- Svelte adds `class?: string` passthrough merged into `poodle-stack ${className}` (`Stack.svelte:28,64`). Not in contract anatomy. **Fix: note class passthrough in contract.**
- `align` default: contract §3 says `"stretch"`; Svelte default is `undefined` resolving to direction-dependent `direction === "column" ? "stretch" : "center"` (`Stack.svelte:48`). Contract is wrong for `direction="row"`. **Fix: document direction-aware align default.**
- `justify` default: contract §3 leaves it blank (`—`); Svelte default is `"start"` (`Stack.svelte:18`) and always emits `justify-content` (`Stack.svelte:53`). **Fix: set contract `justify` default to `"start"`.**
- `LayoutJustify` value: contract §3/§8 spell the wrap value `"space-between"`; Svelte type + helper use `"between"` (`types.ts:36`, `internal.ts:46`). **Fix: change contract `LayoutJustify` union to `start|end|center|between`.**
- `gap`/`padding` token map (contract §8 + SpaceScale map): contract maps `md → space-panel-y` and `lg → space-panel-x`. Svelte `scaleToSpace` agrees (`internal.ts:14-25`), but the Rust `StackSpec.resolved_gap` resolves column gap via `PaddingScale::stack_gap` → `space-stack-{sm,md,lg}` and row gap via `inline_gap` → `space-inline-{sm,md,lg}` (`types.rs:88-104`), and padding via `layout_inset` → `space-inline-*`/`space-stack-*` (`types.rs:52-68`). Svelte uses neither the stack/inline split nor direction-aware gap. **Three-way token divergence — Svelte (`panel-y`/`panel-x`) vs Rust (`stack`/`inline`). Reconcile in contract §8; pick Svelte tokens as authority.**
- `wrap` default: contract `false`, Svelte `false` (`Stack.svelte:24`) — matches.
- `asRole`/`ariaLabel`: contract default `null`; Svelte `null`, emitted as `role`/`aria-label` only when set (`Stack.svelte:64`) — matches.

## GPUI gap (vs Svelte + contract)

Component code is token-clean — zero hardcoded px/color literals in `stack.rs` (all spacing via `resolve_px(theme, token)`).

- [ ] No `overflow` support — `OverflowMode` prop absent from `StackSpec` and builder; Svelte sets `overflow` inline (`Stack.svelte:60`).
- [ ] No `width`/`height`/`minWidth`/`minHeight` sizing builders — Svelte exposes all four (`Stack.svelte:37-40`); `Stack` (`stack.rs:42-74`) forwards only direction/gap/align/justify/wrap/padding/role/aria.
- [ ] Direction-aware align default not applied — Svelte defaults row to `center` (`Stack.svelte:48`); GPUI honors `StackSpec` default `Alignment::Stretch` for all directions (`stack.rs:113-124`, spec default `stack.rs:38`). Resolve align default by direction.
- accepted: no ARIA (gpui has no accessibility API) — `role`/`aria_label` stored in spec but not emitted.
- accepted: row-vs-column gap token split (`stack_gap`/`inline_gap`) is the shared Rust spec behavior, not a GPUI-local bug; tracked in Contract↔Svelte token reconciliation.

## Jetstream gap (vs Svelte + contract)

Component code is token-clean — zero hardcoded px/color literals in `stack.rs` (gap/padding via `resolve_px`).

- [ ] No `direction` support — `js_stack` hardcodes `.flex_col()` (`stack.rs:13`); ignores `spec.direction`, so `direction="row"` never renders horizontally. Svelte switches on direction (`Stack.svelte:51`).
- [ ] No `justify` support — `spec.justify` never read; Svelte always emits `justify-content` (`Stack.svelte:53`). Add justify mapping.
- [ ] No `wrap` support — `spec.wrap` never read; Svelte emits `flex-wrap` (`Stack.svelte:55`). Add wrap.
- [ ] No `overflow`/`width`/`height`/`minWidth`/`minHeight` sizing — none plumbed through `js_stack` (`stack.rs:9-40`); Svelte exposes all (`Stack.svelte:37-41`).
- [ ] Direction-aware align default not applied — defaults to spec `Alignment::Stretch` for all cases (`stack.rs:19-24`); Svelte rows default to `center`.
- [ ] No `role`/`aria_label` channel — spec carries them; `js_stack` drops both.
- accepted: interaction n/a — Stack is a non-interactive layout primitive (no event loop wiring needed).

## Specimen parity

- Svelte covers: Column (default md), Column large-gap center-aligned, Row, Row justify=between, Row wrapping (`StackSpecimen.svelte`).
- GPUI covers: Column default, Column large-gap center, Row, Row justify=between, Row wrapping (`gpui/.../stack.rs`) — full parity with Svelte. Specimen scaffolding uses raw `px(8.0)`/`px(24.0)` for the eyebrow layout (`stack.rs:30,42` etc.), but that is harness chrome, not the Stack under test.
- Jetstream covers: Default gap, Small gap, Center aligned (`jetstream/.../stack.rs:19-29`) — missing: **Row direction**, **Row justify=between**, **Row wrapping** groups. Cannot demonstrate them until `js_stack` supports direction/justify/wrap. Specimen item helper also hardcodes `px(8.0)`/`py(4.0)`/`rounded(4.0)` and `text_size(11.0)` (`stack.rs:16-17,33`) — harness chrome, but worth noting it does not exercise the token path.

## Notes

- Both Rust **component** files are clean: no `.h(<n>)`, `.w(<n>)`, `text_size(<n>)`, `px(<float>)`, `hsla(`/`rgb(` literals. All Stack spacing resolves from tokens. Hardcoded literals appear only in **specimen harness** code (eyebrow/label chrome), which is out of scope for the component token contract.
- Biggest structural gap is Jetstream: `js_stack` is column-only and ignores direction, justify, wrap, and sizing — roughly half the Svelte surface. GPUI is near-complete (sizing + overflow + align-default the only holes).
- The `consv=gap` driver splits into two buckets: (1) undocumented Svelte sizing surface (`width`/`height`/`minWidth`/`minHeight`/`overflow`/`class`), and (2) a genuine three-way token-map disagreement on gap/padding (Svelte `panel-*` vs Rust `stack-*`/`inline-*`). The token map needs a single authority — pick Svelte per "Svelte is parity authority" and realign `PaddingScale::stack_gap`/`inline_gap`/`layout_inset` or the contract §8 table.
