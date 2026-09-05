<!-- parity consv=ok gpui=1 jetstream=0 specimen=ok | jet-specimen: real 3-layer js_scroll_shell — vertical (12 overflowing rows) + horizontal (10 columns); mirrors gpui's 2 groups; both previews build clean -->
<!-- pass: both targets rebuilt to the 3-layer Root→Viewport→Content anatomy (contract §2). Content wrapper added; horizontal/both gets a non-shrinking row (max-content analogue). GPUI: Root clips (overflow_hidden + radius), Viewport scrolls per-axis + padding + focus ring. Jetstream same minus focus. Keyboard scroll remains open (needs stateful ScrollHandle host). gpui build + jetstream probe (4) green. -->
# Parity: ScrollShell

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/scroll-shell.md`
- Svelte (authoritative): `packages/svelte/components/src/ScrollShell.svelte`
- GPUI: `packages/gpui/components/src/primitives/scroll_shell.rs`
- Jetstream: `packages/jetstream/components/src/scroll_shell.rs`
- Spec: `packages/contracts/components/src/scroll_shell.rs`
- Specimens: svelte `packages/svelte/preview/src/specimens/ScrollShellSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/scroll_shell.rs` · jetstream `packages/jetstream/preview/src/specimens/scroll_shell.rs`

## Contract ↔ Svelte

Svelte matches the contract: `direction`/`padding`/`asRole`/`label`/`focusable`/`onScroll` props with matching defaults; three-layer root→viewport→content DOM; `overflowForDirection` axis mapping; `overscroll-behavior: contain`; `.content--h` min-width max-content for horizontal/both; focus ring with `0.125rem` offset; focusable adds `tabindex=0`, defaults role `region`, label `Scrollable content`. No divergence.

- Rust `ScrollShellSpec` omits `onScroll` (callback) and models `asRole` as `role: Option<SurfaceRole>` rather than the contract's `"region" | "group"`. These are spec-shape nuances (callbacks live outside the spec; SurfaceRole superset). Not a contract↔Svelte divergence — flagged under targets where behavior is missing.

## GPUI gap (vs Svelte + contract)

GPUI does real scrolling: `overflow_y_scroll`/`overflow_x_scroll`/`overflow_scroll` per direction, `min_h_0`/`min_w_0` to allow shrink, focus ring on focusable, padding + radius from tokens. Now built as the 3-layer anatomy.

- [x] DONE: **Content layer + horizontal max-content** — rebuilt to Root→Viewport→Content (contract §2). Root clips (`overflow_hidden` + `radius.surface`), Viewport owns per-axis overflow + padding + focus ring, Content wraps children; for horizontal/both the Content is a non-shrinking row (`flex_row().flex_shrink_0()`) — gpui's `min-width: max-content` analogue. Vertical Content is `flex_col().w_full()`.
- [ ] **No keyboard scrolling** — contract §6 requires Arrow/Page/Home/End when the viewport is focused; `.focusable()` adds the focus ring only. gpui's `track_scroll`/`ScrollHandle` (offset/set_offset) makes this feasible, but the handle must persist across frames — the stateless `IntoElement` shell recreates it each render and loses offset. Needs a stateful host (Entity owning the ScrollHandle) before key handlers can be wired faithfully. Left open.
- accepted: **`overscroll-behavior: contain`** — gpui exposes no overscroll-contain API; scroll chaining can't be prevented. Contract §12 marks this an allowed "where possible" delta.
- accepted: no ARIA (gpui has no accessibility API) — region role / aria-label not emitted, though spec stores them.
- accepted: scrollbar visuals platform-native (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] DONE: `js_scroll_shell` wires `spec.direction` (axis + scroll), `overflow_scroll` (per-axis), `resolved_padding()` (token-resolved inset, probe-verified to offset children), and `radius.surface`.
- [x] DONE: rebuilt to the 3-layer anatomy (Root→Viewport→Content). Content wrapper added; horizontal/both gets a non-shrinking row (`flex_row().flex_shrink_0()`) as the `max-content` analogue, vertical is `flex_col().w_full()`. Probe-tested: `three_layer_anatomy` (child depth ≥ 3), `horizontal_lays_out_in_a_row`.
- accepted: focus ring / focusable / keyboard scroll live in the preview `main.rs` event loop; no ARIA channel; scrollbar visuals platform-native.

## Specimen parity

- Svelte covers: **Vertical scroll** (12 surfaces, 10rem box), **Horizontal scroll** (10 columns, nowrap). Both contract §13 groups.
- GPUI covers: Vertical scroll (12 items), Horizontal scroll (10 columns, `flex_shrink_0` + `whitespace_nowrap`). Full parity with Svelte.
- Jetstream covers: **only "Default"** — 3 text lines in a 120px box (`specimens/scroll_shell.rs:22-25`). — missing: **Vertical scroll** (overflowing content), **Horizontal scroll** entirely. The single specimen does not even demonstrate overflow since the stub clips rather than scrolls.

## Notes

- Jetstream is the least-complete target here: the component is a clipping `div`, not a scroll shell. Until the runtime exposes a scroll viewport, a faithful specimen can't demonstrate the "overflowed" state — per CLAUDE.md, the current 3-line specimen overstates coverage and should be reduced or marked pending until real scrolling lands.
- GPUI specimen `surface_row`/`column_item` helpers hardcode px (`px(24.0)`, `px(3.0)`, `px(8.0)`) — specimen chrome, not component code.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/scroll_shell.rs:30`) — specimen chrome.
