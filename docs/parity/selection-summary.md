<!-- parity consv=fixed gpui=0 jetstream=0 specimen=ok -->
<!-- specimen=ok: both Rust specimens backfilled — multiple-selected (count via chip row), empty state ("No selection"), single item, truncated (max 3 → "+3 more"), full 5-step size ladder, density (compact/default/comfortable); clear control wired everywhere (GPUI also wires on_remove/on_clear). Jetstream parity drift fixed: dropped `meta`-laden items for plain {id,label} matching Svelte; truncation aligned to max 3. Real js_selection_summary/SelectionSummary builders + tokens only. Both targets build clean. -->
<!-- pass 41: both Rust targets built out. Chip/overflow radius now resolve from `radius.control`, border-width from `border.width.default` (no px literals). Overflow badge gets its own per-size font-size (+ GPUI line-height) via spec `overflow_font_rem`/`overflow_line_height_rem`. Item `meta` no longer rendered (anatomy is ChipLabel+RemoveIcon per contract §2). Clear link now renders unconditionally whenever populated (default "Clear", overridable via clear_action), matching Svelte. Additive spec methods: radius_token/border_width_token/chip_font_rem/chip_min_height_rem/overflow_font_rem/overflow_line_height_rem. Jetstream render_probe tests cover placeholder, chips+clear, overflow count, custom label, token-resolved chip bg (5/5 pass). GPUI builds. No count string in any target (chips row IS the summary — Svelte authority). Remove/clear interaction is preview-loop owned. -->
# Parity: SelectionSummary

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/selection-summary.md`
- Svelte (authoritative): `packages/svelte/components/src/SelectionSummary.svelte`
- GPUI (composite): `packages/gpui/components/src/composites/selection_summary.rs`
- Jetstream: `packages/jetstream/components/src/selection_summary.rs`
- Spec: `packages/contracts/components/src/selection_summary.rs` (+ `SelectionSummaryItem`/`RemediationAction` in `packages/contracts/components/src/composite_types.rs:122,564`)
- Specimens: svelte `packages/svelte/preview/src/specimens/SelectionSummarySpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/selection_summary_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/selection_summary.rs`

## Contract ↔ Svelte

Svelte matches the contract on the public surface (props, anatomy, empty/populated/truncated states, ARIA). The divergences are between the **Rust spec** and Svelte+contract, and one contract gap.

> **2026-07-15 — additive split-chip extension (for FilterBuilder).** Svelte + React `SelectionSummary` gained an optional `onActivate` prop: when set, each chip splits into a separate activation `<button>` (the label) and a remove IconButton — two independent, non-nested controls (`aria-label="Edit {label}"` / `"Remove {label}"`). When `onActivate` is null the whole chip stays the remove button (unchanged default). Backward-compatible; the existing RelationPicker consumer is unaffected. **Rust targets unchanged** — the split affordance is web-only by design: the only consumer (`FilterBuilder`) renders its own clause pills in GPUI/Jetstream rather than routing them through `SelectionSummary`, so no Rust `SelectionSummary` change is required. Accepted web-first delta.

- **Rust spec adds an item `meta` field** absent from Svelte. Svelte `SelectionItem` is `{ id, label }` (lines 7–10) and contract §2/§3 anatomy is `ChipLabel + RemoveIcon` only. `SelectionSummaryItem` (`composite_types.rs:564`) and both Rust components render a second `meta` label inside each chip (gpui `selection_summary.rs:204-211`, jetstream `selection_summary.rs:93-99`). **Rust-side (code) fix — out of scope for the contract pass.** Contract is correct: anatomy is `ChipLabel + RemoveIcon` only, matching Svelte. Default to removing `meta` from Rust.
- **Rust models clear as a `RemediationAction` struct** (`id`, `label`, `variant`, `is_disabled`); Svelte uses a fixed `TextLink` labelled "Clear" driven by the `onClear` callback (lines 59–61). Contract §2 names the part `ClearLink` (a `TextLink`) and §5 specifies an `onClear` callback with no payload. The struct's `variant`/`is_disabled` have no Svelte/contract counterpart. **Rust-side (code) fix — out of scope.** Contract is correct (callback model, `ClearLink` is a `TextLink`); the spec models more surface than the contract authorizes.
- **Clear link is unconditional in Svelte, optional in Rust.** Svelte always renders the clear `TextLink` whenever `items.length > 0` (line 59). Rust renders it only when `clear_action`/`on_clear` is set (gpui `selection_summary.rs:254`, jetstream `selection_summary.rs:125`). Contract §4 (populated state) and §2 anatomy say the clear link appears whenever items exist. **Rust-side (code) fix — out of scope.** Contract already states clear appears whenever populated, matching Svelte.
- [x] FIXED: contract §8 now documents the `chips-min-height` per-size values that Svelte sets (`Skeleton`-equivalent at SelectionSummary lines 173,185,196,208,219). Added a `Chips min-height` row to each size variant table: xs `0.875rem`, sm `1rem`, md `1.25rem`, lg `1.5rem`, xl `1.75rem`.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [x] Chip radius resolves from `radius.control` via `resolve_radius(theme, spec.radius_token())` — no `px(12.0)` literal.
- [x] Overflow-badge radius resolves from the same `radius.control` token — no `px(12.0)` literal.
- [x] Overflow badge has its own font-size (`overflow_font_rem`) and line-height (`overflow_line_height_rem`, applied as a `relative` ratio), distinct from `chip_font`.
- [x] Item `meta` no longer rendered — anatomy is ChipLabel + RemoveIcon only (contract §2).
- [x] Clear link renders unconditionally whenever populated (default "Clear", `clear_action.label` overrides) — matches Svelte; gated only by the empty early-return.
- accepted: no ARIA (gpui has no accessibility API) — no `aria-label="Current selection"` on root, no per-chip `aria-label="Remove {label}"`.
- accepted: chip bg/overflow bg/border are computed via alpha-lerp (`selection_summary.rs:145-156`) rather than `color-mix`; resolves from real tokens, matches Svelte intent.

## Jetstream gap (vs Svelte + contract)

- [x] Chip radius resolves from `radius.control` via `resolve_radius(theme, spec.radius_token())` — no `rem_to_px(0.5)` literal.
- [x] Border-width resolves from `border.width.default` via `resolve_px(theme, spec.border_width_token())` on both chip and overflow — no `.border(1.0)` literal.
- [x] Overflow badge uses its own per-size font-size (`overflow_font_rem`), distinct from `chip_font`. (Line-height is a CSS box concern with no direct JsEl analogue; font-size carries the distinction — noted.)
- [x] Item `meta` no longer rendered — anatomy is ChipLabel + RemoveIcon only (contract §2).
- [x] Clear link renders unconditionally whenever populated (default "Clear", `clear_action.label` overrides) — matches Svelte.
- accepted: clear/remove interaction is preview-loop owned — chips/clear are display-only `button`/`label` elements with no JsEl click handler; remove/clear dispatch must live in the preview event loop (`main.rs`). Wiring flagged as preview-side, not component-side.
- accepted: no ARIA channel (no root `aria-label`, no per-chip remove label).

## Specimen parity

- Svelte covers: Multiple items selected (with remove + clear wired), Single item, Truncated (max 3 visible), plus size + density variant grids (`SelectionSummarySpecimen.svelte`).
- GPUI covers: Multiple items, Single item, Sizes, Truncated (max 3), plus size + density variant grids via `specimen_layout`. — missing: **no clear-action group** (every spec built with `SelectionSummarySpec::new`, none calls `with_clear_action`), and **no remove/clear handler wired** (`on_remove`/`on_clear` never set), so the interactive remove/clear that Svelte demonstrates is absent.
- Jetstream covers: Multiple selections (with clear), Single selection, Truncated (max 2, with clear), Sizes, Densities. — missing: parity drift in data — uses `meta`-laden items ("Approval still"/"Image" etc.) that Svelte/contract don't have, and the single-item case carries `meta` ("Primary"). Singular-vs-plural count text is moot (no count text in any target). Truncation threshold differs (jet max=2 vs svelte max=3) — cosmetic.

## Notes

- No target renders a count string ("N selected") — neither Svelte, contract, nor Rust. There is no count text / pluralization surface to audit; the "summary" is the chip row itself.
- `consv=fixed`: the only contract-side fix (missing `chips-min-height` per-size table) is applied. The remaining three Contract↔Svelte bullets are Rust-spec/code divergences (`item.meta`, `RemediationAction.variant`/`is_disabled`, clear-link gating) where the contract already matches authoritative Svelte — those are code fixes out of scope for this contract-reconciliation pass. Per "Svelte is parity authority", reconcile Rust down to the Svelte surface.
- `clear_font` in GPUI uses root `font_size` (`selection_summary.rs:131`); Svelte uses a dedicated `--poodle-selection-summary-clear-font-size` that happens to equal root size per breakpoint — equivalent in value, divergent in token source. Low priority.
- Jetstream clear uses root `font_size` (`selection_summary.rs:130`) — same equivalence note as GPUI.
- ARIA absence is accepted once per target (gpui/jetstream have no accessibility API). Contract §6 ARIA (`role`/`aria-label`/`aria-hidden`) is satisfied only by Svelte.
