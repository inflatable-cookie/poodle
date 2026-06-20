<!-- parity consv=gap gpui=5 jetstream=6 specimen=gap -->
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

- **Rust spec adds an item `meta` field** absent from Svelte. Svelte `SelectionItem` is `{ id, label }` (lines 7–10) and contract §2/§3 anatomy is `ChipLabel + RemoveIcon` only. `SelectionSummaryItem` (`composite_types.rs:564`) and both Rust components render a second `meta` label inside each chip (gpui `selection_summary.rs:204-211`, jetstream `selection_summary.rs:93-99`). **Fix: drop `meta` from the chip render path in both Rust targets, or — if `meta` is intended — add it to Svelte + contract §2/§3. Svelte is authoritative; default to removing it from Rust.**
- **Rust models clear as a `RemediationAction` struct** (`id`, `label`, `variant`, `is_disabled`); Svelte uses a fixed `TextLink` labelled "Clear" driven by the `onClear` callback (lines 59–61). Contract §2 names the part `ClearLink` (a `TextLink`) and §5 specifies an `onClear` callback with no payload. The struct's `variant`/`is_disabled` have no Svelte/contract counterpart. **Fix: keep the callback model; treat `clear_action.label` as cosmetic only. Note the spec models more surface than the contract authorizes — not a Svelte change.**
- **Clear link is unconditional in Svelte, optional in Rust.** Svelte always renders the clear `TextLink` whenever `items.length > 0` (line 59). Rust renders it only when `clear_action`/`on_clear` is set (gpui `selection_summary.rs:254`, jetstream `selection_summary.rs:125`). Contract §4 (populated state) and §2 anatomy say the clear link appears whenever items exist. **Fix: render clear whenever populated; do not gate on an optional action being supplied.**
- **Contract gap (not Svelte):** contract §8 does not document the `--poodle-selection-summary-chips-min-height` per-size table that Svelte sets (lines 173,185,196,208,219) — only `chip` min-height is tabulated in §8 Size Variants. **Fix: add the chips-container `min-height` per size to contract §8.**

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded chip radius literal `rounded(px(12.0))` at `selection_summary.rs:191` — Svelte/contract use `var(--poodle-radius-control)`; resolve from `radius.control` token, not raw `12.0`.
- [ ] Hardcoded overflow-badge radius literal `rounded(px(12.0))` at `selection_summary.rs:243` — same `radius.control` token, not raw `12.0`.
- [ ] Overflow badge reuses `chip_font` and omits its own font-size + line-height. Svelte gives overflow distinct `--poodle-selection-summary-overflow-font-size` and `--poodle-selection-summary-overflow-line-height` per size (lines 176-225). `selection_summary.rs:247` uses `chip_font` and sets no line-height — resolve overflow font-size/line-height per size.
- [ ] Renders item `meta` inside chips (`selection_summary.rs:204-211`) — not in Svelte/contract anatomy; remove (see Contract↔Svelte).
- [ ] Clear link gated on `clear_action` (`selection_summary.rs:254`); Svelte renders it whenever populated. Render unconditionally when `items` non-empty.
- accepted: no ARIA (gpui has no accessibility API) — no `aria-label="Current selection"` on root, no per-chip `aria-label="Remove {label}"`.
- accepted: chip bg/overflow bg/border are computed via alpha-lerp (`selection_summary.rs:145-156`) rather than `color-mix`; resolves from real tokens, matches Svelte intent.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded chip radius literal `let chip_radius = rem_to_px(0.5)` at `selection_summary.rs:24` — Svelte/contract use `var(--poodle-radius-control)`; resolve from `radius.control` token, not magic `0.5`.
- [ ] Hardcoded border-width literal `.border(1.0)` at `selection_summary.rs:85` (chip) and `:120` (overflow) — contract chip border is `0.0625rem`; resolve a border-width token, not raw `1.0`.
- [ ] Overflow badge reuses `chip_font`, no distinct overflow font-size / line-height (`selection_summary.rs:111-122`) — add per-size overflow font-size + line-height like Svelte.
- [ ] Renders item `meta` inside chips (`selection_summary.rs:93-99`) — not in Svelte/contract anatomy; remove (see Contract↔Svelte).
- [ ] Clear link gated on `spec.clear_action` (`selection_summary.rs:125`); Svelte renders it whenever populated. Render unconditionally when `items` non-empty.
- [ ] Clear/remove are not wired: chips are `button("")` and the clear is `button(...).focusable()` with no click handler (`selection_summary.rs:75,128-132`) — component only renders; remove/clear interaction must live in the preview event loop. Confirm `main.rs` dispatches `onRemove`/`onClear`, or flag wiring as absent.
- accepted: no ARIA channel (no root `aria-label`, no per-chip remove label).

## Specimen parity

- Svelte covers: Multiple items selected (with remove + clear wired), Single item, Truncated (max 3 visible), plus size + density variant grids (`SelectionSummarySpecimen.svelte`).
- GPUI covers: Multiple items, Single item, Sizes, Truncated (max 3), plus size + density variant grids via `specimen_layout`. — missing: **no clear-action group** (every spec built with `SelectionSummarySpec::new`, none calls `with_clear_action`), and **no remove/clear handler wired** (`on_remove`/`on_clear` never set), so the interactive remove/clear that Svelte demonstrates is absent.
- Jetstream covers: Multiple selections (with clear), Single selection, Truncated (max 2, with clear), Sizes, Densities. — missing: parity drift in data — uses `meta`-laden items ("Approval still"/"Image" etc.) that Svelte/contract don't have, and the single-item case carries `meta` ("Primary"). Singular-vs-plural count text is moot (no count text in any target). Truncation threshold differs (jet max=2 vs svelte max=3) — cosmetic.

## Notes

- No target renders a count string ("N selected") — neither Svelte, contract, nor Rust. There is no count text / pluralization surface to audit; the "summary" is the chip row itself.
- `consv=gap` driver: Rust spec carries surface (`item.meta`, `RemediationAction.variant`/`is_disabled`) that authoritative Svelte and the contract do not, and Rust gates the clear link on an optional action where Svelte renders it unconditionally. Per "Svelte is parity authority", reconcile Rust down to the Svelte surface (or, only for `meta`, promote into Svelte+contract if the product genuinely wants per-chip metadata).
- `clear_font` in GPUI uses root `font_size` (`selection_summary.rs:131`); Svelte uses a dedicated `--poodle-selection-summary-clear-font-size` that happens to equal root size per breakpoint — equivalent in value, divergent in token source. Low priority.
- Jetstream clear uses root `font_size` (`selection_summary.rs:130`) — same equivalence note as GPUI.
- ARIA absence is accepted once per target (gpui/jetstream have no accessibility API). Contract §6 ARIA (`role`/`aria-label`/`aria-hidden`) is satisfied only by Svelte.
