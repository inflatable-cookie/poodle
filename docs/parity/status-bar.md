<!-- parity consv=gap gpui=6 jetstream=8 specimen=gap -->
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

- Class prefix: Svelte emits `poodle-status-bar` / `poodle-status-bar__leading` / `poodle-status-bar__trailing` (`StatusBar.svelte:35,41,50`); contract §2 anatomy uses unprefixed `.status-bar*`. Cosmetic, but **fix: align contract class names to the `poodle-` prefix** for traceability.
- Spec mismatch (drives `consv=gap`): contract §3 props are `summary`, `ariaLabel`, `chrome`, `size`, `sizeRole`, `density` (Svelte `StatusBar.svelte:6-15`). `ShellStatusBarSpec` (`shell_status_bar.rs:8-13`) instead exposes `summary`, `leading_item_count`, `trailing_item_count` — none of `ariaLabel`/`chrome`/`size`/`sizeRole`/`density` exist, and `*_item_count` is a Rust-only invention with no contract or Svelte counterpart. **Fix: rework the Rust spec to the contract prop set; drop or document `*_item_count`.**
- `chrome` prop has no Rust representation, so the chrome border-top + 94% panel `color-mix` (`StatusBar.svelte:69-72`) cannot be toggled from the spec. **Fix: add `chrome` to the spec + a `chrome_*` token surface.**
- `background_token()` returns `COLOR_BACKGROUND_SURFACE` (`shell_status_bar.rs:49-51`), but Svelte's non-chrome bar has **no** background (transparent, blends into container) and the chrome bar uses `background-panel` at 94% (`StatusBar.svelte:71`). The spec's single surface token matches neither mode. **Fix: model both modes; chrome → panel `color-mix`, default → transparent.**
- Both Rust impls hardcode `1.5rem` height (`status_bar.rs:90`, `shell_status_bar.rs:20`). Contract §7 and Svelte specify **no explicit height** — height is content + padding driven (`padding: 0.375rem 0.75rem`). The fixed height is a Rust-only invention. Noted per-impl below.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] No `chrome` mode — bar always draws bg + border-top (`status_bar.rs:81-94`); cannot render the no-chrome transparent variant. Specimen fakes chrome with an outer wrapper border (`specimens/status_bar.rs:75-79`) instead of the component's own `border-top`.
- [ ] Hardcoded height `.h(px(rem_to_px(1.5)))` at `status_bar.rs:90` — contract has no fixed height; remove and let padding (`0.375rem 0.75rem`) drive it.
- [ ] Padding wrong: only `.px(space.inline.sm)` (`status_bar.rs:92`), no vertical padding. Contract default is `0.375rem 0.75rem`; horizontal should be `0.75rem`, not `space.inline.sm`. Resolve padding-block + padding-inline from tokens.
- [ ] Raw rem literals not token-resolved: gap `px(rem_to_px(0.375))` (`status_bar.rs:97,123`) and summary `text_size(px(rem_to_px(0.8125)))` (`status_bar.rs:107`). Contract gap = `space.inline.md` (root) / `space.inline.sm` (inner); font-size `0.8125rem` should resolve from a token, not a rem literal.
- [ ] Root gap wrong axis: contract root gap is `space-inline-md` between leading/trailing (`StatusBar.svelte:62`); GPUI uses `justify_between` + per-section `0.375rem` and never applies the md root gap.
- [ ] No size/density variants — spec carries neither, so the size font/padding-block table and density padding-inline/gap table (contract §8) are unreachable.
- accepted: no ARIA (gpui has no accessibility API) — `<footer>` landmark + `aria-label` resolution (contract §6) not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] No `chrome` mode — bar always draws bg + `border(1.0)` border (`shell_status_bar.rs:25-30`); cannot render the no-chrome transparent variant, and border is a full box border, not contract's `border-top` only.
- [ ] Hardcoded border width `.border(1.0)` at `shell_status_bar.rs:27` — raw float literal; contract border is `0.0625rem` (chrome only). Resolve from a token.
- [ ] Hardcoded height `let bar_height = rem_to_px(1.5)` at `shell_status_bar.rs:20` then `.h(bar_height)` (`:28`) — contract has no fixed height; drop it.
- [ ] Raw rem literals, none token-resolved: `pad_x = rem_to_px(0.5)` (`:21`), `gap = rem_to_px(0.375|0.5)` (`:22`), `font_size = rem_to_px(0.6875)` (`:23`). Contract padding `0.375rem 0.75rem`, root gap `space-inline-md`, inner gap `space-inline-sm`, font-size `0.8125rem` — all must resolve from tokens. Font size `0.6875rem` is the **xs** size value, wrong for the default bar.
- [ ] Padding x-only via `pad_x` (`shell_status_bar.rs:29`); no vertical padding. Contract default `0.375rem 0.75rem`; horizontal should be `0.75rem`, not `0.5rem`.
- [ ] Gap heuristic `is_dense()` (`shell_status_bar.rs:22`, spec `:45-47` = item count > 3) is invented — contract density is an explicit `compact|default|comfortable` prop, not derived from item count.
- [ ] Layout deviates from contract: summary is rendered as a third grow child between leading and trailing (`shell_status_bar.rs:40-50`) with a spacer fallback. Contract/Svelte put summary **inside** the leading region as a fallback when no leading snippet exists (`StatusBar.svelte:41-47`); summary and leading items are never both shown.
- [ ] No size/density variant tables (spec lacks the props).
- accepted: no ARIA channel — `<footer>` landmark / `aria-label` resolution absent.
- accepted: this component dispatches no events (contract §5); nothing interaction-side to wire in `main.rs`.

## Specimen parity

- Svelte covers: Default (no chrome), With chrome, Summary only, plus `sizes`/`densities` variant snippets (`StatusBarSpecimen.svelte:8-60`). Leading = branch + error items; trailing = cursor/encoding/language metadata.
- GPUI covers: Default (no chrome), With chrome (faked via outer wrapper border), Summary only (`specimens/status_bar.rs:28-121`). — missing: **size variants**, **density variants**; chrome is wrapper-faked, not component-driven.
- Jetstream covers: "With summary", "Minimal" (`specimens/shell_status_bar.rs:22-43`). — missing: **chrome group**, **summary-only-without-items** (its "Minimal" still labels itself summary), **size variants**, **density variants**. Trailing labels use raw `text_size(11.0)` (`:14-20`) — specimen-level hardcode.

## Notes

- `consv=gap` driver is the Rust spec, not Svelte: `ShellStatusBarSpec` models a different component (item-count-driven shell bar) than the contract's slot-driven `StatusBar`. Per "Svelte is parity authority" the Rust spec must be reshaped to `summary`/`ariaLabel`/`chrome`/`size`/`sizeRole`/`density`, not the contract bent to match the spec.
- The Rust spec note (contract §"Rust Spec Note", `shell_status_bar.rs:1-4`) already flags the `ShellStatusBar` → `StatusBar` rename and the item-count divergence as accepted-for-now; treat the spec rework as the headline follow-up.
- Both Rust impls invent a fixed `1.5rem` height absent from contract/Svelte — likely cargo-culted from a real IDE status bar. Should be content-driven.
- Jetstream summary-as-grow-child layout (vs Svelte summary-as-leading-fallback) is the most visible behavioral divergence: in Jetstream a bar can show leading items AND summary simultaneously, which Svelte never does.
