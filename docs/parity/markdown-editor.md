<!-- parity consv=fixed gpui=11 jetstream=8 specimen=gap -->
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

- [ ] Hardcoded tool-button dims — `.w(px(28.0)).h(px(28.0))` at `markdown_editor.rs:111-112`. Contract §8 tool size varies by size (`1.5/1.75/2/2.25/2.5rem`); resolve from a size-scaled token, drop literal `28.0`.
- [ ] Hardcoded toolbar vertical padding `.py(px(6.0))` at `markdown_editor.rs:193`; contract toolbar-Y is density-driven (`0.25/0.375/0.5rem`). Resolve from density token.
- [ ] Hardcoded tool/mode gap `.gap(px(2.0))` at `markdown_editor.rs:197` and `218`; contract tool gap density-driven (`0.0625/0.125/0.1875rem`). Resolve from density token.
- [ ] Hardcoded separator dims — `.w(px(1.0)).h(px(16.0))` at `markdown_editor.rs:150`; also the separator itself is not in contract anatomy (§2) — remove the invented separator or justify it.
- [ ] Split divider hardcoded `.w(px(1.0))` at `markdown_editor.rs:298`; contract uses `border-right: 0.0625rem` on the textarea. Use a border, resolved width.
- [ ] Pane padding uses `panel_space_x/y_rem(density)` (`markdown_editor.rs:81-82`) not the contract pane-X/pane-Y density table (`0.625/0.75/0.875rem`). Resolve from the dedicated pane density values.
- [ ] `min_height` parse only strips `"px"` (`markdown_editor.rs:128-133`) and falls back to `200.0`; contract `minHeight` default is `"12rem"` and is a CSS length. Parse rem, default to `12rem`-equivalent.
- [ ] Edit pane is a non-editable `div` rendering `display.to_string()` with ad-hoc key-event char appending (`markdown_editor.rs:248-288`) — not a real textarea; no selection, no toolbar insertion, no monospace font-family from `typography.code.family`. Accepted as runtime limit BUT font-family + placeholder color tokens are still missing.
- [ ] Tool buttons missing `aria-label`/`title` and are not disabled in preview mode — contract §6 requires per-action aria-label + native disabled in preview; GPUI `toolbar_btn` (`markdown_editor.rs:108-122`) emits neither label nor disabled state.
- [ ] Toolbar formatting buttons are inert — no `on_click` wired to insert markdown syntax (contract §5 / Tier-1 "toolbar actions produce correct markdown"). Only mode buttons have handlers.
- [ ] Preview pane renders raw markdown text, not rendered HTML elements (h1/h2/code/blockquote styling from contract §8 "Preview Rendered Elements" all absent). Accepted as Tier-3 rendering freedom only if a renderer is plugged; currently shows source text — note as visual gap.
- accepted: no ARIA (gpui has no accessibility API).

## Jetstream gap (vs Svelte + contract)

- [ ] Tool buttons use ASCII glyph labels `["B","I","H","#","<>","\u{201C}","\u{2022}"]` (`markdown_editor.rs:71`) instead of the contract icons (bold/italic/heading/link/code/quote/list). Render `Icon` primitives, not letters. Order also differs from contract (heading before link/code, here `#` then `<>`).
- [ ] Hardcoded toolbar gap `rem_to_px(0.5)` at `markdown_editor.rs:62`; matches contract `0.5rem` literal but should resolve from token. Low-priority literal.
- [ ] Hardcoded tool font-size `rem_to_px(0.75)` (`markdown_editor.rs:79`), textarea `rem_to_px(0.8125)` (`lines 157/163`), preview `rem_to_px(0.875)` (`lines 186/192`) — contract values but hardcoded literals; resolve from typography tokens.
- [ ] Toolbar uses full `.border(1.0)` (`markdown_editor.rs:67`) — contract is `border-bottom` only. Apply bottom border only.
- [ ] Toolbar bg via `tint(elevated, 0.72)` (`markdown_editor.rs:39`) — `tint` is a manual color-mix helper; contract is `color-mix(...elevated 72%, transparent)`. Verify tint matches the transparent-mix semantics, not a solid lighten.
- [ ] Disabled opacity hardcoded `.opacity(0.48)` at `markdown_editor.rs:54`; contract uses `state.opacity.disabled` token. Resolve from token, not literal `0.48`.
- [ ] Tool buttons disabled-opacity hardcoded `.opacity(0.4)` at `markdown_editor.rs:83`; contract tool `:disabled` opacity `0.4` — still a literal, resolve a token.
- [ ] No textarea/edit interactivity, no markdown insertion, no rendered-HTML preview (preview shows source text). Accepted: interaction lives in preview `main.rs` event loop — but note `main.rs` has no markdown-editor wiring, so mode switching/typing is not exercisable.
- accepted: ARIA channel absent; real text editing belongs to preview event loop.

## Specimen parity

- Svelte covers: Split view (pre-filled), Edit mode (empty + placeholder), Preview mode (disabled tools), Disabled (`MarkdownEditorSpecimen.svelte`-equivalent — note: the actual Svelte specimen file should be confirmed; registry-driven).
- GPUI covers: Interactive (split, stateful mode + value, char counter), Edit only, Preview only, Disabled — broadest coverage; mode switching is wired and interactive. — missing: nothing major vs contract specimen set.
- Jetstream covers: Edit mode, Split mode, Empty placeholder — missing: **Preview mode** group, **Disabled** group; tool buttons show letters not icons so visual parity with toolbar is broken; hardcoded `text_size(11.0)` group labels.

## Notes

- GPUI/Jetstream both render the preview pane as plain source text rather than parsed HTML — this is the single biggest visual divergence. The contract §8 "Preview Rendered Elements" table (h1–hr styling) is entirely unrealized in both Rust targets. Markdown→HTML rendering is Tier-3 freedom, but showing unparsed source is arguably "worse than no preview". Flag for product decision.
- GPUI is the only target with a working interactive specimen (stateful mode + value via AppState). Jetstream has no editor wiring in `main.rs`.
- Spec `MarkdownEditorSpec` has `render_html_label: Option<String>` as a stand-in for the `renderHtml` callback — acceptable since callbacks can't cross the spec boundary, but neither Rust target reads it.
