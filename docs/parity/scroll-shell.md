<!-- parity consv=ok gpui=3 jetstream=1 specimen=gap -->
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

GPUI does real scrolling: `overflow_y_scroll`/`overflow_x_scroll`/`overflow_scroll` per direction, `min_h_0`/`min_w_0` to allow shrink, focus ring on focusable, padding + radius from tokens.

- [ ] **No Content layer / horizontal `min-width: max-content`** — contract §2 anatomy requires a Content wrapper; for horizontal/both it must set `min-width: max-content` so content does not collapse (contract §8). GPUI appends children directly to the viewport (`scroll_shell.rs:119-121`); horizontal row relies on child `flex_shrink_0` in the specimen instead.
- [ ] **No keyboard scrolling** — contract §6 requires Arrow/Page/Home/End scrolling when the viewport is focused; `.focusable()` adds focus ring only (`scroll_shell.rs:104-109`), no key handlers. Contract §10 explicitly says GPUI must add this where the platform doesn't.
- [ ] **No `overscroll-behavior: contain` equivalent** — scroll chaining not prevented. Contract §12 marks this an allowed delta "where possible" — note but try.
- accepted: no ARIA (gpui has no accessibility API) — region role / aria-label not emitted, though spec stores them.
- accepted: scrollbar visuals platform-native (contract §12 Known Delta).

## Jetstream gap (vs Svelte + contract)

- [x] DONE: no longer a stub — `js_scroll_shell` now wires `spec.direction` (axis + scroll), `overflow_scroll` (per-axis), `resolved_padding()` (token-resolved inset, probe-verified to offset children), and `radius.surface`.
- [ ] No focus ring / focusable / horizontal content `max-content` — focus/focusable live in the preview event loop; `max-content` for horizontal scroll needs non-shrinking children. Accepted/preview-side.
- accepted: interaction (keyboard scroll) lives in preview `main.rs` event loop; no ARIA channel; scrollbar visuals platform-native.

## Specimen parity

- Svelte covers: **Vertical scroll** (12 surfaces, 10rem box), **Horizontal scroll** (10 columns, nowrap). Both contract §13 groups.
- GPUI covers: Vertical scroll (12 items), Horizontal scroll (10 columns, `flex_shrink_0` + `whitespace_nowrap`). Full parity with Svelte.
- Jetstream covers: **only "Default"** — 3 text lines in a 120px box (`specimens/scroll_shell.rs:22-25`). — missing: **Vertical scroll** (overflowing content), **Horizontal scroll** entirely. The single specimen does not even demonstrate overflow since the stub clips rather than scrolls.

## Notes

- Jetstream is the least-complete target here: the component is a clipping `div`, not a scroll shell. Until the runtime exposes a scroll viewport, a faithful specimen can't demonstrate the "overflowed" state — per CLAUDE.md, the current 3-line specimen overstates coverage and should be reduced or marked pending until real scrolling lands.
- GPUI specimen `surface_row`/`column_item` helpers hardcode px (`px(24.0)`, `px(3.0)`, `px(8.0)`) — specimen chrome, not component code.
- Specimen `group()` helper hardcodes `text_size(11.0)` (jetstream `specimens/scroll_shell.rs:30`) — specimen chrome.
