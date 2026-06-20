<!-- parity consv=ok gpui=4 jetstream=3 specimen=gap -->
# Parity: ResizeHandle

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/resize-handle.md`
- Svelte (authoritative): `packages/svelte/components/src/ResizeHandle.svelte`
- GPUI: `packages/gpui/components/src/primitives/resize_handle.rs`
- Jetstream: `packages/jetstream/components/src/resize_handle.rs`
- Spec: `packages/contracts/components/src/resize_handle.rs` · `Orientation` `packages/contracts/components/src/types.rs:183`
- Specimens: svelte `packages/svelte/preview/src/specimens/ResizeHandleSpecimen.svelte` · gpui `packages/gpui/preview/src/specimens/resize_handle.rs` · jetstream `packages/jetstream/preview/src/specimens/resize_handle.rs`

## Contract ↔ Svelte

Svelte matches the contract precisely: `orientation`/`disabled`/`ariaLabel`/`ariaValueNow`/`ariaValueMin`/`ariaValueMax` props with matching defaults; `role="separator"` + `aria-orientation`; window-level mousemove/mouseup drag; keyboard 8px step + Home/End ±9999; idle line 82% border-default; hover/dragging accent-base; focus ring. `aria-label` defaults to `"Resize"`. No divergence.

- Orientation convention (authoritative): `orientation="horizontal"` → **vertical line**, `width:0.5rem; height:100%`, `col-resize`, drag left/right. `orientation="vertical"` → **horizontal line**, `width:100%; height:0.5rem`, `row-resize`, drag up/down. GPUI follows this; **Jetstream inverts it** (see below).

## GPUI gap (vs Svelte + contract)

GPUI matches orientation, sizing, idle 82% line, focus ring, disabled opacity, cursors. Tokens resolved via spec methods.

- [ ] **Hover recolors the whole hit target, not the line** — `container.hover(|s| s.bg(accent_hover))` with `hover_color.opacity(0.3)` (`resize_handle.rs:139-140`); contract §8 hover changes only the *line* `background` to `accent-base`. Apply accent to the line, not a translucent fill on the container.
- [ ] **No keyboard resize** — contract §6/§5 require ArrowLeft/Right/Up/Down (±8) and Home/End (±9999) via `onResizeStep`; the component is `.focusable()` but binds no `on_key_down` and exposes no step callback. Render-only.
- [ ] **No drag callbacks** — `onResizeStart`/`onResizeMove`/`onResizeEnd` equivalents are not wired (no `on_drag`/mouse handlers); the handle cannot request resizes. Wire GPUI drag model.
- [ ] `rounded(px(999.0))` literal (`resize_handle.rs:116,122`) — pill sentinel matching `999rem`; acceptable but note it's a raw float rather than a token (no radius token exists for "full pill"; consider adding one).
- accepted: no ARIA (gpui has no accessibility API) — separator role / aria-value* not emitted, though spec stores them.

## Jetstream gap (vs Svelte + contract)

- [x] **Orientation inverted vs contract/Svelte/GPUI** — FIXED: the two match arms were swapped; Horizontal now renders the vertical line (`w(0.5rem) height:100%`), Vertical the horizontal line.
- [x] **Idle line not at 82% opacity** — FIXED: `handle_color` now `tint(border, 0.82)` matching the contract `color-mix(border-default 82%, transparent)`.
- [ ] **No hover state** — contract/Svelte change the line to `accent-base` on hover/drag; `js_resize_handle` has no hover branch. (Interaction would live in preview event loop, but no hover-color path exists.)
- [ ] **No keyboard / drag callbacks** — no `onResizeStep`/start/move/end wiring; component is purely visual.
- [ ] **No focus ring** — contract §8 focus-visible outline (`accent.focusRing`); GPUI applies it, Jetstream does not.
- accepted: interaction (drag, keyboard) lives in preview `main.rs` event loop, not the component; no ARIA channel.

## Specimen parity

- Svelte covers: Horizontal split (vertical handle), Vertical split (horizontal handle), **Disabled horizontal**, **Disabled vertical** — all four contract §12 groups, in real left/right + top/bottom pane layouts.
- GPUI covers: Horizontal split, Vertical split, Disabled horizontal, Disabled vertical — full parity with Svelte (matching pane layouts).
- Jetstream covers: Horizontal, Vertical (bare, no pane context). — missing: **both Disabled groups**; and because orientation is inverted, the two it does show render the wrong axis.

## Notes

- The orientation inversion is the highest-priority Jetstream fix: it makes the two non-disabled specimens render the opposite axis from Svelte/GPUI, so the integration test currently "passes" while showing wrong output.
- `rem_to_px(0.125)` / `rem_to_px(0.5)` literals in both Rust targets are token-derived rem constants (the contract specifies exact rem values, no token indirection), so they are acceptable per the contract's "exact values" table — not hardcoded-px violations.
- Specimen `group()`/`pane()` helpers hardcode `text_size(11.0)` (jetstream) and `text_sm()`/`px(6.0)` radius (gpui) — specimen chrome.
