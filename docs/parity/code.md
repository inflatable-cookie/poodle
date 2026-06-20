<!-- parity consv=gap gpui=8 jetstream=9 specimen=gap -->
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

- Svelte adds `inlineVariant?: "default" | "plain"` (default `"default"`) → `data-inline-variant="plain"` drops inline padding/radius/background (lines 175-179). Not in contract §3. **Fix: add to contract props + states.**
- Svelte adds `typography?: "body" | "inline"` (default `"body"`) → `data-typography="inline"` sets font-size `1em × adjustmentRatio` and `line-height: inherit` (lines 181-184). Not in contract §3. **Fix: add to contract.**
- Inline copy button: Svelte renders an adjacent compact copy button (`.code__copy--inline`, `1.25rem` square, `0.75rem` icon, lines 90-108/246-254) wrapped in `.code--inline-wrap`. Contract §3 mentions "when inline, renders a compact adjacent copy button" but §2 anatomy + §8 token tables omit the inline wrapper and the `--inline` copy sizes. **Fix: add inline-wrap anatomy + inline copy token rows.**
- Block border: Svelte uses plain `var(--poodle-color-border-subtle)` (line 189); contract §8 says `color-mix(border-subtle 42%, transparent)`. **Fix: reconcile — Svelte authoritative, drop the 42% mix.**
- Block background: contract §8 root says `color-mix(panel 92%, elevated)`. Svelte puts no background on the block root; `.code__pre` uses `color-mix(canvas 92%, black)` (line 263). **Fix: contract is wrong on both location and colors — document pre background, not root.**
- Toolbar background: Svelte `color-mix(elevated 60%, panel)` (line 199); contract §8 toolbar table has no background row. **Fix: add toolbar background.**
- Toolbar border-bottom: Svelte plain `var(--poodle-color-border-subtle)` (line 200); contract says `color-mix(border-subtle 32%, transparent)`. **Fix: reconcile to plain token.**
- Source line-height: Svelte `1.4` (line 270); contract §8 source table says `1.4` but §11 checklist says `1.625`. **Fix: correct checklist to 1.4.**
- `.code__toolbar-actions`: Svelte adds `margin-left: auto` (line 214); contract §8 omits it. Minor. **Fix: note it.**

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

- The `consv=gap` driver is twofold: undocumented Svelte surface (`inlineVariant`, `typography`, inline copy button) and stale contract color-mix values (block border 42%, root background panel-92, toolbar border 32%) that Svelte does not implement. All belong reconciled into the contract per "Svelte is parity authority".
- Jetstream `code` is the weakest implementation in this batch — it is effectively a bordered text label and is missing toolbar, inline, line numbers, highlight, copy, and max-height. It should be rebuilt against the contract before its specimens can be considered real.
- GPUI copy button is render-only (no clipboard, no copied state) — Tier-1 parity item §11 unmet on both Rust targets.
