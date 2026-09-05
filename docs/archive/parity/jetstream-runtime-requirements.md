<!-- jetstream-runtime-requirements -->
# Jetstream Runtime Requirements (to close accepted UI deltas)

The Poodle Jetstream component layer (`packages/jetstream/components`, rendering via
`jetstream_runtime::ui_element::JsEl`) reached full contract/specimen parity with a set of
**accepted runtime deltas** — places where a component had to approximate or omit a contract
detail because the *runtime* (`JsEl` / the `jetstream-renderer` engine, sibling repo
`~/Dev/projects/jetstream/crates/jetstream-runtime`) has no channel for it.

This doc enumerates the runtime capabilities needed to close those deltas, grounded in the
**actual** current `JsEl` surface. It is the requirements/ask **from** the UI layer — the
engine work itself lives in the sibling repo and is out of scope here. Component impact counts
are "# of `docs/parity/*.md` files referencing the gap".

## Architecture verdict (2026-06-22): sound — most asks are wire-ups, not new capability

Audited the runtime pipeline (`render_immediate` → `materialize` → Taffy `layout` → draw
commands; `UiStyle`, `FocusState`, `animation`/`tick_transitions`, `draw.rs`). **The model is
correct for Poodle and richer than the parity notes implied.** Pure render-from-spec (host owns
semantic state; runtime retains transient state — hover/pressed/focus/scroll/transitions) matches
the stateless-component contract. **No architectural rework needed.**

The friction was the thin `JsEl` builder (`ui_element.rs`) **under-exposing a capable model**:
`UiStyle` already has `shadow`, `z_index`, `transform`+`transform_origin`, `transitions`,
`background_gradient`, `position`+insets; `draw.rs` honors shadow/transform/focused/hovered/pressed;
`tick_transitions(dt)` runs every frame. So most P1/P2 items below are **builder wire-ups + draw-
honor checks**, not new subsystems:

- **P1-1 focus-visible** → **DONE 2026-06-22.** A focused `.focusable()` node now renders the
  contract's outset focus ring — `draw.rs` emits a `box-shadow` (offset 0, blur 0, spread = 2px
  focus width) in `theme.focus_color`, sitting outside the element without disturbing its border
  (was a border-recolor). **Offscreen-verified** (focused field shows a blue ring outside its
  border, unfocused none). 56/56 interactive `js_*` mark `.focusable()` (slider fixed). The host
  sets focus via its event loop (preview keyboard/click); that's the only remaining wiring and it
  already exists for the interactive specimens.
- **P1-2 custom/inset/multi-layer shadow + elevation** → **LANDED (single-layer):** the runtime now
  has `JsEl::shadow(offset_x, offset_y, blur, spread, color)` feeding the same `style.shadow` the draw
  bridge rasterizes (render path proven). Poodle's Jetstream overlays now resolve **token-accurate
  elevation** from the structured `ELEVATION_*` ShadowValue tokens (`theme_ext::with_elevation` +
  `elevation_surface/overlay/dialog`), matching GPUI's mapping (single layer, spread 0) — the
  one-size `shadow_md/sm/lg` presets are gone from all elevated surfaces (popovers/menus/dropdowns →
  overlay, modals/drawers → dialog, raised cards/surfaces → surface). **Residual:** the contract's
  **multi-layer + inset DONE 2026-06-22** (offscreen-verified): `JsEl::shadow_layers(Vec<BoxShadow>)`
  emits a quad per layer (outset stacks — popover/card depth); `BoxShadow.inset` draws inside the
  element (render_bridge emits inset quads after the element at its own rect; the shader paints
  inward from the edge — blur 0 = a hard inner ring of width `spread`, blur > 0 = a soft inner
  shadow). The runtime channels are complete. **Component wiring (2026-06-22):** `list-card`
  highlighted draws its contract inset accent ring (`inset 0 0 0 1px accent@12%`) via `shadow_layers`
  — offscreen-verified. Contract re-check: the other "ring" candidates aren't spread-based offset-0
  inset rings — `tabs`' `inset 0 0 0 2px` is the drag **drop-target** state (needs a `TabsSpec` field,
  not wired); `tri-state-switch`'s is an offset highlight + a real accent border; `segmented-control`'s
  is an offset highlight. So list-card is the only true spread inset ring. **Residual:** single-layer
  elevation → contract multi-layer stacks where the contract CSS specifies them (needs per-layer
  values / multi-layer tokens).
- **P2-6 animation** → `transitions`/`AnimatableProperty`/`tick_transitions` are live; expose a
  `.transition(…)` builder. (Builder only — the engine already ticks it.)
- **P3-9 rotate / P3-11 z-index** → honored by `UiStyle`+draw; add builders. (Builder only.)

**Genuinely missing from the model (real additions):** font-family (P1-3), letter-spacing (P1-4),
border-style dashed/dotted (P1-5) — no `UiStyle` field. Plus the radial-gradient render bug below.

**One real (non-blocking) cost:** `render_immediate` does `tree.clear()` + full `materialize` +
full `layout()` each call in a continuous frame loop — no dirty-tracking / diff / layout memo.
Correct, but wasteful for static app screens at 60fps. An optimization (skip re-materialize/relayout
when the tree is unchanged) for later **if** profiling warrants — not a rework, not blocking.

## What the runtime already has (corrects stale parity notes)

Several older parity notes claim gaps that no longer exist — impls can use these **today**, no
engine work:

- **Gradients** — `bg_gradient_linear` (angle: 0=top→bottom, 90=left→right; stops 0..1) and
  `bg_gradient_radial` (center + radius as **fractions of the rect**) exist and work
  (color_picker saturation overlays + hue strip, toast tone-tint, progress fill). **Fixed
  2026-06-22** (offscreen-verified): `stops[0]` was ignored (gradient started from the element
  bg, radial discarded it); the radial blend was distorted (shader aliased center-Y as stop0);
  and gradient-only elements (no solid bg) rendered nothing. All three resolved — `color_a =
  stops[0]`, radial uses `t = clamp(dist/radius)`, and the fill quad emits whenever a gradient is
  present. The media-thumbnail accent-wash radial is now renderable. **>2-stop gradients** — the
  GPU layer still packs only first+last; handled at the **component level by tessellation** (render
  an N-stop gradient as N-1 adjacent 2-stop segments, flex-grown to the stop spacing, clipped by
  the parent `overflow_hidden`). The color-picker rainbow hue strip uses this — offscreen-verified
  full red→…→red rainbow (was rendering solid red). A native N-stop GPU path is unnecessary for
  current usage; revisit only if a non-axis-aligned multi-stop case appears.
- **Per-side borders + colors** — `border_t_/b_/l_/r_` + `border_color_{top,bottom,left,right}`.
  (table row borders, accordion, field-set legend, status-bar chrome — no need for all-sides hacks.)
- **Custom `BoxShadow`** — a `BoxShadow` type and `style.shadow` field exist (slider/tooltip set
  it directly); see P1-2 — only the *builder ergonomics + token-driven elevation + inset/multi-layer*
  are missing, not shadow itself.
- **Percent in layout** — taffy `LengthPercentage` is used internally; but the public `w(f32)` /
  `flex_basis(f32)` builders take **px**, so percent *width on a div* is still not reachable (P3-6).
- **Interaction events** — `on_click`, `on_drag`, `on_mouse_down`, `on_pointer_enter/leave`,
  `on_scroll`, `focusable` exist; widgets `text_input`, `slider`, `progress`, `rich_text`, `image`.
- **Focus rings** — render automatically (see P1-1): focused `.focusable()` node draws the
  `theme.focus_color` ring. No engine work to get a focus indicator.
- **`.hover()` / `.active()` — DONE 2026-06-22** (was the real stub; offscreen-verified).
  The `JsStyleOverride` now lives on `NodeStyle`; `UiTree` carries hovered/pressed node indices
  (`GameUi::set_pointer_state(x,y,down)` hit-tests + stores them, non-destructively); and
  `collect_draw_commands` merges each node's override into its draw command's effective
  bg/border/text-color/opacity, propagating to the hovered/pressed node **and its ancestors**
  (CSS `:hover`/`:active`). Host wiring is one call per frame before collect (the preview does
  this after `process_input`). **29 components already called `.hover()`/`.active()` (40 + 6 calls,
  previously no-ops) and now render** — menu/sidebar/list/card/select/button/etc. The retained
  tree is never mutated, so the effect clears the instant the pointer leaves; no rebuild needed.

### Pixel verification is now available (new)

`packages/jetstream/preview/src/bin/snap.rs` renders a `JsEl` scene to a PNG on a **headless wgpu
device** (no window), reusing the preview's `capture_screenshot` path. Quads-only (bg / border /
**shadow** / focus ring; no glyph pass). Visual deltas in the runtime can now be confirmed by
inspecting a PNG — this already corrected two wrong entries in this doc (focus rings "not drawn",
and the shadow render path). **Verify visually before claiming a render gap.**

**Action item (Poodle-side, no engine work):** audit impls still flat-approximating gradients /
multi-side borders and upgrade them. Tracked separately from the engine asks below.

## P1 — high-leverage visual channels (concrete `JsEl` builder adds)

| # | Capability | Status | Unblocks (≈ docs) | Current workaround | Suggested API |
|---|---|---|---|---|---|
| 1 | **focus-visible style state** | **DONE** (outset `box-shadow` ring, spread = 2px focus width, in `theme.focus_color`; visually confirmed) | focus rings on text-input, button, icon-button, checkbox, radio-group, switch, select, slider, segmented-control, tabs, fields, popover/menu triggers, resize-handle, scroll-shell, link, … (**34**) | renders the contract outset ring on focus; 56/56 components mark `.focusable()` | done — host sets focus via its event loop (already wired for interactive specimens) |
| 2 | **token-driven / custom / inset / multi-layer box-shadow** | **DONE** — `JsEl::shadow()`, token elevation, `shadow_layers()` (stacks), `BoxShadow.inset` (rings + soft inner + **offset highlights**) all landed + offscreen-verified | elevation on overlays/surfaces; inset rings (list-card); treatment stacks (button = inset highlight + drop) | every shadow channel renders | **Finding:** elevation tokens are single-layer by design (`--poodle-elevation-overlay: 0 .75rem 2rem …`) — no token rework; my elevation wiring already matches Svelte. Multi-layer is component *treatment* shadows (button highlight+drop, wired). Residual: apply treatment shadows to remaining components opportunistically. |
| 3 | **font-family channel** | **DONE 2026-06-22** (offscreen-verified) | code, code-input, kbd chips (menu/context-menu/menubar/command-palette/action-discovery), audio/video time labels, duration-input, markdown-editor, slug text-input, metric-tile, color-picker hex (**~15 wired**) | — | `JsEl::font_family(FontFamily::{Sans,Mono})` landed (sans default; cosmic-text Family). 15 components now request Mono where the contract specifies code-family; sans already resolved to a system sans. Residual: a *specific* embedded sans (Inter) for exact parity vs the system fallback — not blocking. |
| 4 | **letter-spacing / tracking** | **DONE 2026-06-22** (offscreen-verified) | eyebrow (0.12/0.04em), badge, table header, field-set legend, meta-item, order-by, nav-card, sidebar-nav (0.18em), code lang, page-header, button (0.01em), calendar, pill, … (**~20 wired**) | — | `JsEl::letter_spacing_em(f32)` landed (em, scales with font size); contract-exact values applied per component; eyebrow pulls `spec.letter_spacing_em()`. |
| 5 | **dashed / dotted border-style** | **DONE 2026-06-22** (offscreen-verified) | region, file-upload + dock-region drop-zones, list-card not-live, empty-state (**5 wired**) | — | `JsEl::border_style(BorderStyle::{Solid,Dashed,Dotted})` landed; shader dash-masks the border by perimeter position (packed into the unused `border_params.y` lane). Dotted *underlines* (time-ago/text-link) skipped — no underline element to attach a bottom-border to (text-decoration, not a border). |

## P2 — interaction & motion (larger / architectural)

| # | Capability | Status | Unblocks (≈ docs) | Notes |
|---|---|---|---|---|
| 6 | **animation / motion ticker** | missing in the component render path (some widget-internal rotation exists) | spinner rotation, skeleton shimmer sweep, indeterminate progress bar, toast enter/exit, collapse/accordion height, tri-state thumb slide (**27**) | needs a runtime per-frame ticker feeding the immediate-mode render; components render the static end-state today. Likely the single biggest perceived-quality gap after focus rings. |
| 7 | **component-level interaction callbacks** | by-design gap (Jetstream renders from spec; all click/keyboard/drag wiring lives in the preview `main.rs` event loop, routed via `parse_action(token_key)` hit-test ids) | every interactive component (select/menu/dialog open-close, tab activation, drag reorder, rename commit, toggle, auto-dismiss) | this is an **architecture choice**, not a missing primitive: the `js_*` fns are pure. The ask is a standard component→host event channel (the hit-test ids already exist) so consumers don't hand-wire each in `main.rs`. Medium; design decision. |

## P3 — accepted / niche / web-parity-only

| # | Capability | Status | Unblocks | Disposition |
|---|---|---|---|---|
| 8 | **accessibility (role / aria-*) channel** | missing (`aria`/`role` 0 refs; renderer has no a11y tree) | every interactive component (**138** docs mention it) | **platform-wide accepted gap**, parallels GPUI's no-a11y limit. Large standalone effort (needs an accessibility tree + platform AX bridge). Track as its own epic, not a per-component fix. Specs already carry `aria_label`/`role`/`described_by` for when a channel lands. |
| 9 | **transform: rotate** | missing (`rotate` 0 refs) | chevron rotation (disclosure), list-card sash `rotate(-45deg)` | low — workaround is icon-swap (chevron-down/right) + corner block; visually adequate. |
| 10 | **filter: grayscale** | **DONE 2026-06-22** (`NodeStyle.grayscale` + `JsEl::grayscale()`; shader desaturates toward luminance) | list-card "not-live" wired to `grayscale(1)`; offscreen-verified | done (hover-restore is a hover-state refinement — no grayscale lane on JsStyleOverride) |
| 11 | **z-index channel** | missing (`z_index` 0 refs; layering via `overlay`/`anchor_to` render order) | toast-host stacking order | low — render-order works; only matters for cross-overlay z control. |
| 12 | **text-transform: uppercase** | missing | eyebrow, badges, table headers | low — already handled in-component via `.to_uppercase()`; only costs original-casing for copy/AX. |
| 13 | **percent width on div** | missing (`w`/`flex_basis` take px f32) | range-slider between-fill, card-toggle/field-set column widths | low — worked around with fixed-px flex segments / chunked rows. A `w_pct(f32)`/`flex_basis_pct(f32)` would simplify. |
| 14 | **container queries / viewport units** | missing | responsive collapse: form-actions danger overflow, dialog `max-height: 80vh`, toast narrow-breakpoint, tabs overflow→menu | medium-complex — needs a measure→relayout pass. Web-parity-only; desktop targets render the desktop arm. |

## Suggested order

1. ~~P1-1 focus-visible~~ and ~~P1-2 box-shadow builder/elevation~~ — **both largely DONE.**
   Focus rings render the contract outset ring (P1-1 DONE); custom shadow + token elevation
   landed (P1-2, incl. multi-layer + inset). The two biggest "looks unfinished" deltas are closed.
   All shadow channels render; residual is component wiring only (selection-ring borders → inset
   rings where contracts specify them).
2. **P1-3/4/5** (font-family, letter-spacing, dashed/dotted border) — **all DONE**. The full P1
   set is closed.
3. **P2-6 animation** — large but high perceived value.
4. **P3-8 accessibility** — its own epic, coordinate platform-wide.
5. The rest (P3-9..14) are niche/accepted; do opportunistically.

## Verification once a capability lands

For each delivered runtime feature, the Poodle side: (a) replace the workaround in the affected
`js_*` components, (b) add/update `render_probe` assertions where observable, (c) rebuild
`jetstream/preview`, (d) drop the "accepted delta" note from the affected `docs/parity/*.md`.
Components and the specific approximation per gap are greppable: `rg -l '<keyword>' docs/parity/`.
