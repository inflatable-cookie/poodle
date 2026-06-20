<!-- parity consv=fixed gpui=8 jetstream=9 specimen=gap -->
# Parity: Code

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/code.md`
- Svelte (authoritative): `packages/svelte/components/src/Code.svelte`
- GPUI: `packages/gpui/components/src/primitives/code.rs`
- Jetstream: `packages/jetstream/components/src/code.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/CodeSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/code.rs` · jetstream `packages/jetstream/preview/src/specimens/code.rs`

## Contract ↔ Svelte

Svelte has props and visual values the contract does not document, and several contract color-mix values disagree with Svelte. Svelte is authoritative — update the contract.

- [x] FIXED `inlineVariant?: "default" | "plain"` added to contract §3 props, §4 States (inline plain), and §8 (`[data-inline-variant="plain"]` token table dropping padding/radius/background).
- [x] FIXED `typography?: "body" | "inline"` added to contract §3 props, §4 States (inline typography), and §8 (`[data-typography="inline"]` token table — `1em × adjustmentRatio`, `line-height: inherit`). Inline base font-size also corrected to `0.8125em × adjustmentRatio`.
- [x] FIXED Inline copy button: §2 anatomy now shows the `.code--inline-wrap` span wrapping the `<code>` + `.code__copy--inline` button; §8 adds an "Inline wrap" token table and "Inline copy button" / "Inline copy button SVG icon" rows (`1.25rem` square, `0.75rem` icon).
- [x] FIXED Block border: §8 block table now uses plain `var(--poodle-color-border-subtle)`; dropped the 42% mix.
- [x] FIXED Block background: §8 block root background removed (block root carries no background) and the `.code__pre` table now documents `color-mix(canvas 92%, black)` plus density-adjusted padding.
- [x] FIXED Toolbar background: §8 toolbar table adds `color-mix(elevated 60%, panel)`.
- [x] FIXED Toolbar border-bottom: §8 toolbar table reconciled to plain `var(--poodle-color-border-subtle)`; dropped the 32% mix.
- [x] FIXED Source line-height: §11 Tier-2 checklist corrected from `1.625` to `1.4` (the §8 source table was already `1.4`).
- [x] FIXED `.code__toolbar-actions`: §8 toolbar-actions table adds `margin-left: auto`.
- [x] FIXED (extra) §10 GPUI Notes color-mix mappings rewritten to reference the reconciled values (inline 72%, pre 92% canvas/black, toolbar 60%, line-highlight 12%, plain border tokens) instead of the stale 42%/92%-panel/32% mixes.

## GPUI gap (vs Svelte + contract)

Behavior + visual gaps. `[ ]` open, `[x]` done. Mark accepted runtime limits.

- [ ] Hardcoded rem-float literals throughout: inline `px(rem_to_px(0.375/0.125/0.25))` at `code.rs:109-111`; toolbar `py(px(rem_to_px(0.375)))` `code.rs:150`, language `text_size(px(rem_to_px(0.6875)))` `code.rs:160`, copy `w/h/rounded(px(rem_to_px(1.5/1.5/0.25)))` `code.rs:174-176`, `max_h(px(rem_to_px(20.0)))` `code.rs:201` — resolve all from tokens, not raw rem floats.
- [ ] No copy clipboard behavior or 2s feedback — copy button is a static `div` with hover only (`code.rs:170-189`); no `on_click`, no check-icon swap. Contract §4/§5 require copy + 2s feedback.
- [ ] Copy icon is `clipboard-copy` registry icon (`code.rs:183`); Svelte uses an inline 14px SVG and swaps to a check on copy. No copied state.
- [ ] No **line numbers** rendering parity — GPUI prints `format!("{:>3} ", n)` text (`code.rs:224`) instead of a `2.5rem` right-aligned tabular gutter (contract §8 line-number table). Width/alignment/tabular-nums absent.
- [ ] Highlighted line does not extend `±1rem` (contract §8 + §10) — `row.bg(highlight_bg)` only (`code.rs:219-221`); no negative margin / padding bleed to container edge.
- [ ] Inline mode lacks the `adjustmentRatio` scaling (`code.rs:106-116`); font is `size_font_rem` raw, not `0.8125em × adjustmentRatio`. Also no `inlineVariant`/`typography` support.
- [ ] Block background/border colors diverge: uses `spec.fill_token()`/`spec.border_token()` (`code.rs:98/124`) — verify these map to Svelte's pre `color-mix(canvas 92%, black)` and plain `border-subtle`, not the contract's stale mixes.
- [ ] Line gap uses `space.inline.sm` between number and content (`code.rs:217`); Svelte line-number uses `padding-right: 1rem`. Spacing model differs.
- accepted: no ARIA (gpui has no accessibility API) — block `aria-label`, copy-button label not emitted.

## Jetstream gap (vs Svelte + contract)

- [ ] Hardcoded border-width literal `.border(1.0)` at `code.rs:27` — resolve from a border-width token, not raw `1.0`.
- [ ] **No block toolbar at all** — `js_code` (lines 12-30) renders only a single `label()` with bg/border/padding. No language label, no copy button, no toolbar anatomy (contract §2 block mode).
- [ ] **No inline mode** — `spec.is_inline` never read; always renders the block-ish label. Contract §4 inline state unsupported.
- [ ] **No line numbers** — `spec.show_line_numbers` never read; renders raw multiline string.
- [ ] **No line highlighting** — `spec.highlight_lines` never read.
- [ ] **No copy button / clipboard / 2s feedback** — `spec.is_copyable` never read.
- [ ] **No maxHeight / scroll** — `spec.max_height` never read; no overflow container.
- [ ] Source is a `label()` not a per-line structure; lines are not split into `.code__line` spans, so highlight/number/scroll anatomy is impossible without restructuring.
- [ ] Padding uses `control_space_x_rem` + `panel_space_y_rem` (`code.rs:15-16/29`); contract pre padding is `0.75rem 1rem` (density-adjusted). Verify the x token matches `1rem`, not control spacing.
- accepted: no ARIA channel; interaction (copy click) would live in preview event loop.

## Specimen parity

- Svelte covers: Block with language label, With line numbers and highlight, CSS with max height, Inline code, No copy button, Sizes, Densities (`CodeSpecimen.svelte`).
- GPUI covers: Block w/ language, Line numbers + highlight, CSS max-height, Inline code, No copy button, Sizes, Densities. — missing: nothing in the specimen set, but the underlying copy/feedback behavior is non-functional (static button).
- Jetstream covers: Code block, With language hint (no toolbar rendered), With line numbers (not actually rendered). — missing: real **toolbar/language label**, **highlight**, **inline**, **max-height/CSS**, **no-copy-button** variant, **Sizes**, **Densities**. Most specimens are nominal only because the component lacks the features.

## Notes

- `consv=fixed`: the undocumented Svelte surface (`inlineVariant`, `typography`, inline copy button + wrap) is now in the contract, and the stale color-mix values (block border 42%, root background panel-92, toolbar border 32%) are reconciled to Svelte's plain tokens / pre background. Remaining gpui/jetstream work (jetstream `code` lacks toolbar/inline/line-numbers/highlight/copy/max-height entirely) is code-side.
- Jetstream `code` is the weakest implementation in this batch — it is effectively a bordered text label and is missing toolbar, inline, line numbers, highlight, copy, and max-height. It should be rebuilt against the contract before its specimens can be considered real.
- GPUI copy button is render-only (no clipboard, no copied state) — Tier-1 parity item §11 unmet on both Rust targets.
