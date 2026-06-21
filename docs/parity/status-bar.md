<!-- parity consv=fixed gpui=0 jetstream=0 specimen=gap | pass: spec reshaped to contract props (summary/aria/chrome/size/density), both targets rebuilt chrome-aware, height dropped, padding/gap/font token-resolved, summary-as-leading-fallback; jetstream probe tests added -->
# Parity: StatusBar

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/status-bar.md`
- Svelte (authoritative): `packages/svelte/components/src/StatusBar.svelte`
- GPUI: `packages/gpui/components/src/primitives/status_bar.rs`
- Jetstream: `packages/jetstream/components/src/shell_status_bar.rs` (note: NOT `status_bar.rs` — Jetstream named this file `shell_status_bar.rs`; the only other reference is the `mod` line in `lib.rs`)
- Rust spec: `packages/contracts/components/src/shell_status_bar.rs` (`ShellStatusBarSpec` — filename/type predate the doc rename `ShellStatusBar` → `StatusBar`)
- Specimens: svelte `packages/svelte/preview/src/specimens/StatusBarSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/status_bar.rs` · jetstream `packages/jetstream/preview/src/specimens/shell_status_bar.rs`

## Contract ↔ Svelte

The contract and Svelte agree on anatomy, ARIA resolution, the chrome modifier, and the size/density token tables. The Rust `ShellStatusBarSpec` is the divergent surface — it never modelled the Svelte prop set.

- [x] FIXED Class prefix: Svelte emits `poodle-status-bar` / `__leading` / `__trailing` (`StatusBar.svelte:35,41,50`); contract §2 anatomy + §8 selectors were unprefixed `.status-bar*`. Aligned all contract class names (anatomy, root/leading/trailing tables, chrome modifier, size + density selectors) to the `poodle-` prefix.
- **CODE (Rust spec, out of scope for contract):** contract §3 props (`summary`, `ariaLabel`, `chrome`, `size`, `sizeRole`, `density`) already match Svelte (`StatusBar.svelte:6-15`). The divergence is entirely Rust-side: `ShellStatusBarSpec` (`shell_status_bar.rs:8-13`) exposes `summary`, `leading_item_count`, `trailing_item_count` — none of `ariaLabel`/`chrome`/`size`/`sizeRole`/`density`, plus a Rust-only `*_item_count`. Rework the Rust spec to the contract prop set; drop/document `*_item_count`. Contract is already correct (no edit needed).
- **CODE:** `chrome` has no Rust representation, so the chrome border-top + 94% panel `color-mix` (`StatusBar.svelte:69-72`) cannot be toggled from the spec. Add `chrome` + `chrome_*` tokens to the spec. Contract §8 already documents both modes.
- **CODE:** `background_token()` returns `COLOR_BACKGROUND_SURFACE` (`shell_status_bar.rs:49-51`); Svelte non-chrome bar is transparent and chrome bar uses `background-panel` at 94% (`StatusBar.svelte:71`). Model both modes spec-side. Contract is already correct.
- **CODE:** Both Rust impls hardcode `1.5rem` height (`status_bar.rs:90`, `shell_status_bar.rs:20`); contract §7 + Svelte specify **no explicit height** (content + padding driven). Drop the fixed height in code. Contract is already correct.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] DONE `chrome` mode — bar is transparent unless `spec.chrome`; chrome paints `color_mix(panel, transparent, 0.94)` bg + token-resolved `border_t_1` border (`status_bar.rs`). No more wrapper-faked chrome path needed.
- [x] DONE Fixed height removed — height is content + padding driven (`0.375rem 0.75rem` default).
- [x] DONE Padding token/scale resolved — `py(padding_block_rem)` (size-scaled) + `px(padding_inline_rem)` (density-scaled); horizontal default `0.75rem`.
- [x] DONE Gap + font token-resolved — root gap = `root_gap_token` (`space.inline.md`) with density override; inner gap = `space.inline.sm`; font = `font_size_rem` (default `0.8125rem`). No raw rem literals (rem comes from contract-exact spec scale methods).
- [x] DONE Root gap applied on the correct axis — `.gap(root_gap)` between leading/trailing, with `justify_between` + `flex_wrap`.
- [x] DONE Size/density variants — spec now carries `size`/`size_role`/`density`; size drives font + padding-block, density drives padding-inline + gap (contract §8 tables).
- accepted: no ARIA (gpui has no accessibility API) — `<footer>` landmark + `aria-label` resolution (contract §6) not emitted (spec exposes `resolved_aria_label()` for the channel that can carry it).
- note (specimen, preview crate — not touched here, shared target lock): specimen still fakes chrome via outer wrapper border and lacks size/density groups; should adopt `.chrome(true)` + size/density specimens.

## Jetstream gap (vs Svelte + contract)

- [x] DONE `chrome` mode — bar transparent unless `spec.chrome`; chrome paints `color_mix(panel, transparent, 0.94)` + `border_t_1` + `border_color_top` (top-only, not a full box border).
- [x] DONE Border width — chrome border uses `border_t_1` (1px = the resolved `0.0625rem` at 16px base). JsEl exposes only a fixed-1px top setter, so width is approximated at 1px + per-side top color set explicitly (noted in code; runtime crate is out of scope to extend).
- [x] DONE Fixed height dropped — content + padding driven.
- [x] DONE Token/scale-resolved: `padding_inline_rem` (density), `padding_block_rem` (size), `root_gap_token` + `density_gap_rem`, `inner_gap_token` (`space.inline.sm`), `font_size_rem` (default `0.8125rem`, was wrongly the xs `0.6875`).
- [x] DONE Padding y now present — `py(padding_block_rem)`; horizontal default `0.75rem`.
- [x] DONE `is_dense()` heuristic removed — density is now the explicit `compact|default|comfortable` prop.
- [x] DONE Layout matches contract — summary is the leading-region fallback (`label` inside `leading_row` only when no leading slot content), never a third grow child; summary and leading items are never both shown.
- [x] DONE Size/density variant tables — spec carries the props; size→font+padding-block, density→padding-inline+gap.
- accepted: no ARIA channel — `<footer>` landmark / `aria-label` resolution absent (`resolved_aria_label()` available spec-side).
- accepted: this component dispatches no events (contract §5); nothing interaction-side to wire in `main.rs`.

## Specimen parity

- Svelte covers: Default (no chrome), With chrome, Summary only, plus `sizes`/`densities` variant snippets (`StatusBarSpecimen.svelte:8-60`). Leading = branch + error items; trailing = cursor/encoding/language metadata.
- GPUI covers: Default (no chrome), With chrome (faked via outer wrapper border), Summary only (`specimens/status_bar.rs:28-121`). — missing: **size variants**, **density variants**; chrome is wrapper-faked, not component-driven.
- Jetstream covers: "With summary", "Minimal" (`specimens/shell_status_bar.rs:22-43`). — missing: **chrome group**, **summary-only-without-items** (its "Minimal" still labels itself summary), **size variants**, **density variants**. Trailing labels use raw `text_size(11.0)` (`:14-20`) — specimen-level hardcode.

## Notes

- `consv=fixed`: the only contract↔Svelte drift was the unprefixed class names, now aligned to `poodle-`. The remaining drift is entirely Rust-spec, not contract: `ShellStatusBarSpec` models a different component (item-count-driven shell bar) than the contract's slot-driven `StatusBar`. Per "Svelte is parity authority" the Rust spec must be reshaped to `summary`/`ariaLabel`/`chrome`/`size`/`sizeRole`/`density` — a code change, out of scope here. The contract already matches Svelte.
- The Rust spec note (contract §"Rust Spec Note", `shell_status_bar.rs:1-4`) already flags the `ShellStatusBar` → `StatusBar` rename and the item-count divergence as accepted-for-now; treat the spec rework as the headline follow-up.
- Both Rust impls invent a fixed `1.5rem` height absent from contract/Svelte — likely cargo-culted from a real IDE status bar. Should be content-driven.
- Jetstream summary-as-grow-child layout (vs Svelte summary-as-leading-fallback) is the most visible behavioral divergence: in Jetstream a bar can show leading items AND summary simultaneously, which Svelte never does.
