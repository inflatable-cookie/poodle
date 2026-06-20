<!-- parity consv=fixed gpui=3 jetstream=3 specimen=gap -->
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

- [x] FIXED Svelte `width`/`height` (defaults `null`) → inline `width`/`height` (`Stack.svelte:37-38,56-57`). Added to contract §3 props + §8 inline table.
- [x] FIXED Svelte `minWidth`/`minHeight` (defaults `null`) → overridable `min-width`/`min-height` (`Stack.svelte:39-40,58-59`). Added to contract §3 props + §8 (base min-* now noted overridable).
- [x] FIXED Svelte `overflow?: OverflowMode` (default `"visible"`) → inline `overflow` (`Stack.svelte:41,60`). Added `overflow` prop + `OverflowMode` shared type + §8 inline row.
- [x] FIXED Svelte `class?: string` passthrough merged into `poodle-stack ${className}` (`Stack.svelte:28,64`). Added `class` prop + §9 note; §9 root class corrected to `poodle-stack`.
- [x] FIXED `align` default — now documented direction-aware (`column` → `stretch`, `row` → `center`, `Stack.svelte:48`) in §3 + §9.
- [x] FIXED `justify` default — set to `"start"` (`Stack.svelte:18`), always emitted (§3 + §8 + §9).
- [x] FIXED `LayoutJustify` union — changed `"space-between"` → `"between"` (`types.ts:36`, `internal.ts:46`) in §3 shared types + §8 inline table.
- [x] CONTRACT-OK `gap`/`padding` token map — contract §8 already maps `md → space-panel-y`, `lg → space-panel-x`, matching Svelte `scaleToSpace` (`internal.ts:14-25`). The `stack`/`inline` split lives only in the Rust `StackSpec` (`types.rs:88-104`); that is a code-side spec realignment, not a contract drift. Contract is correct per Svelte. (See Notes — Rust spec follow-up.)
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

- [x] DONE: `direction` (flex_col/flex_row), `justify` (Start/End/Center/SpaceBetween → justify_start/end/center/between), and `wrap` (flex_wrap) now wired. Probe-tested: row lays children horizontally, column vertically.
- [ ] No `overflow`/`width`/`height`/`minWidth`/`minHeight` sizing — these are **not on `StackSpec`** (the contract sync added them to the contract, but the Rust spec lacks the fields). Add to `StackSpec` first, then both Rust targets can resolve.
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
- `consv=fixed`: the undocumented Svelte sizing surface (`width`/`height`/`minWidth`/`minHeight`/`overflow`/`class`), direction-aware align default, `justify` default, and the `between` rename are all now in the contract. The contract §8 token map already matched Svelte (`panel-*`), so no contract change was needed there.
- **Code-side follow-up (not contract):** the Rust `StackSpec` resolves gap/padding via `PaddingScale::stack_gap`/`inline_gap`/`layout_inset` → `space-stack-*`/`space-inline-*` (`types.rs:52-104`), diverging from Svelte's `panel-y`/`panel-x`. Per "Svelte is parity authority" the Rust spec should be realigned to the `panel-*` tokens. This is a code change, out of scope for the contract reconciliation.
