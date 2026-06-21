<!-- parity consv=fixed gpui=2 jetstream=1 specimen=ok pass=41 -->
# Parity: MarkdownEditor

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/markdown-editor.md`
- Svelte (authoritative): `packages/svelte/components/src/MarkdownEditor.svelte`
- GPUI: `packages/gpui/components/src/composites/markdown_editor.rs`
- Jetstream: `packages/jetstream/components/src/markdown_editor.rs`
- Spec: `packages/contracts/components/src/markdown_editor.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/MarkdownEditorSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/markdown_editor_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/markdown_editor.rs`

## Contract ↔ Svelte

- [x] FIXED **`value` default**: contract §3 now `value: string | undefined` default `undefined` (bindable), with the `value !== undefined` controlled/uncontrolled switch documented in §3; defaulting to `""` would force controlled mode (Svelte `:32,57`).
- **`placeholder` default**: contract §3 says `"Write markdown..."`; Svelte matches (`line 35`). OK.
- [x] FIXED **Mode-X**: contract §8 Size table gains a Mode-X column (`xs 0.375 / sm,md 0.5 / lg 0.625 / xl 0.75 rem`). Corrected the parity note's claim — `--poodle-md-editor-mode-x` is *size*-driven only (not density), and is currently declared-but-unconsumed in Svelte CSS, so it lives in the size table, not the density table (Svelte `:248,259,272,277`).
- **Toolbar separator**: GPUI/Jetstream invent a vertical separator between tool groups; Svelte has none and contract §2 anatomy lists no separator. Not a contract gap — flagged under Rust gaps as an invented part.
- Tool icons: contract §8 mode icons `pencil`, `columns-2`, `eye` and tool icons bold/italic/heading/link/code/quote/list match Svelte `toolbarActions` (`lines 150-158`). OK.
- Preview-empty copy "Nothing to preview" matches across contract §2, Svelte (`line 233`). OK.
- `render_html` / `onValueChange` present in contract §3/§5 and Svelte; Rust specs carry `render_html_label` placeholder only (rendering is Tier-3 freedom). OK.

## GPUI gap (vs Svelte + contract)

- [x] FIXED tool-button dims — now `px(rem_to_px(spec.tool_size_rem()))` (contract §8 size table `1.5/1.75/2/2.25/2.5rem`); literal `28.0` gone.
- [x] FIXED toolbar vertical/horizontal padding — `toolbar_y_rem()`/`toolbar_x_rem()` density values (`0.25/0.375/0.5` × `0.375/0.5/0.625rem`).
- [x] FIXED tool/mode gap — density `tool_gap_rem()` (`0.0625/0.125/0.1875rem`); mode-switcher also gets `mode_x_rem()`/`mode_y_rem()` padding.
- [x] FIXED invented separator removed — not in contract anatomy; tools now render in one container in contract order (bold/italic/heading/link/code/quote/list).
- [x] FIXED split divider — textarea now uses `border_r_1()` + `border-subtle` (contract `border-right: 0.0625rem`), no standalone divider div.
- [x] FIXED pane padding — `pane_pad_rem()` (`0.625/0.75/0.875rem`), not `panel_space_*`.
- [x] FIXED min-height — `spec.min_height_rem()` parses rem/px and defaults to `12rem` (contract), no `200.0` px fallback.
- [x] FIXED placeholder + font-size tokens — placeholder color = `text.tertiary`, preview-empty = `text.tertiary`, textarea `0.8125rem` / preview `0.875rem`. (Monospace font-family: GPUI text on a `div` has no font-family API here — Known Delta.)
- [x] FIXED tool buttons disabled in preview — `tools_disabled()` dims to `0.4` + `Arrow` cursor; hover = accent@12% (was the bogus `color.bg.hover` token which resolves to **black** — real bug fixed).
- [ ] Toolbar formatting buttons remain inert (no `on_click` markdown insertion) — Tier-1 "toolbar actions produce correct markdown" lives in the preview event loop. Note.
- [ ] Preview pane shows source text, not parsed HTML (contract §8 "Preview Rendered Elements") — Tier-3 rendering freedom; a markdown→HTML renderer plugs in at the preview loop. Note.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [x] FIXED tool buttons — now real `Icon` glyphs in contract order (bold/italic/heading/link/code/quote/list), not ASCII letters.
- [x] FIXED tool/textarea/preview font-sizes — `0.75 / 0.8125 / 0.875rem` resolved via `rem_to_px`; toolbar gap `0.5rem` (contract literal).
- [x] FIXED toolbar border — `border_b_1()` (bottom only), was full `.border(1.0)`.
- [x] FIXED toolbar bg — `tint(elevated, 0.72)` is alpha-only (matches `color-mix(elevated 72%, transparent)`), confirmed not a solid lighten.
- [x] FIXED disabled opacity — root uses `resolve_opacity("state.opacity.disabled")`, not literal `0.48`; tool `:disabled` `0.4` from contract.
- [x] FIXED edit pane is now a `text_input` (value + placeholder), preview pane shows source; split textarea gets right border. Three probe-verified panes (edit/preview/split).
- [ ] No real text editing / markdown insertion / rendered-HTML preview — interaction belongs to the preview `main.rs` event loop (still no md wiring there). Note.
- accepted: ARIA channel absent; real text editing belongs to preview event loop.

## Specimen parity

- Svelte covers: Split view (pre-filled), Edit mode (empty + placeholder), Preview mode (disabled tools), Disabled (`MarkdownEditorSpecimen.svelte`-equivalent — note: the actual Svelte specimen file should be confirmed; registry-driven).
- GPUI covers: Interactive (split, stateful mode + value, char counter), Edit only, Preview only, Disabled — broadest coverage; mode switching is wired and interactive. — missing: nothing major vs contract specimen set.
- Jetstream covers: Edit mode, Split mode (with char-count status), **Preview mode** (tools disabled), Empty placeholder, **Disabled** — now full contract specimen set; tool buttons render real icons. Probe tests cover all seven tool icons + three mode icons, placeholder/value text, preview-empty copy, and split dual-pane.

## Notes

- GPUI/Jetstream both render the preview pane as plain source text rather than parsed HTML — this is the single biggest visual divergence. The contract §8 "Preview Rendered Elements" table (h1–hr styling) is entirely unrealized in both Rust targets. Markdown→HTML rendering is Tier-3 freedom, but showing unparsed source is arguably "worse than no preview". Flag for product decision.
- GPUI is the only target with a working interactive specimen (stateful mode + value via AppState). Jetstream has no editor wiring in `main.rs`.
- Spec `MarkdownEditorSpec` has `render_html_label: Option<String>` as a stand-in for the `renderHtml` callback — acceptable since callbacks can't cross the spec boundary, but neither Rust target reads it.
- Pass 41: added additive pure helpers to `MarkdownEditorSpec` so both targets resolve the contract §8 tables from one place: `tool_size_rem`, `mode_x_rem`, `toolbar_x/y_rem`, `tool_gap_rem`, `mode_y_rem`, `pane_pad_rem`, `min_height_rem` (parses rem/px, defaults `12rem`), `effective_size`, `shows_editor/shows_preview/tools_disabled`, `char_count`, plus token methods (`tool_color/tool_hover_color/tool_hover_fill/textarea_color/placeholder_color/preview_empty_color/split_divider/toolbar_border/focus_ring`). Unit-tested in poodle-specs. No token gaps — all values map to existing semantic tokens; the only Known Delta is GPUI lacking a `div` font-family API (monospace textarea) and GPUI/Jetstream lacking inset rendering for the preview HTML tree.
