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

- **P1-1 focus-visible** → **ALREADY WORKS (corrected 2026-06-22 by offscreen render).** `draw.rs`
  swaps `border_color → theme.focus_color` (the `border_focus` token) at ≥2px whenever a `.focusable()`
  node holds `FocusState` focus, and rasterizes it. **Visually confirmed** via the new offscreen snap
  harness (focused field shows the blue ring, unfocused doesn't). 55/56 interactive `js_*` already
  mark `.focusable()` (slider was the lone miss — now fixed). **Residual is fidelity + state, not
  absence:** the ring is a 2px border-recolor, not the contract's outset `box-shadow 0 0 0 3px focus.ring`
  with offset; and it only appears once something sets focus (Jetstream's host/preview event loop —
  the accepted interaction-loop bucket). An exact outset ring would need a focus-state style merge
  (`focus_style.shadow`) — a nicety, since a token-colored ring already renders.
- **P1-2 custom/inset/multi-layer shadow + elevation** → **LANDED (single-layer):** the runtime now
  has `JsEl::shadow(offset_x, offset_y, blur, spread, color)` feeding the same `style.shadow` the draw
  bridge rasterizes (render path proven). Poodle's Jetstream overlays now resolve **token-accurate
  elevation** from the structured `ELEVATION_*` ShadowValue tokens (`theme_ext::with_elevation` +
  `elevation_surface/overlay/dialog`), matching GPUI's mapping (single layer, spread 0) — the
  one-size `shadow_md/sm/lg` presets are gone from all elevated surfaces (popovers/menus/dropdowns →
  overlay, modals/drawers → dialog, raised cards/surfaces → surface). **Residual:** the contract's
  multi-layer stacks (inset highlight + stacked drops on popover/card) and inset selection rings still
  collapse to the single token layer — needs `inset` + `.shadow_layers(vec![..])` on the builder.
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
| 1 | **focus-visible style state** | **PRESENT — corrected** (the engine already draws a `theme.focus_color` ring at ≥2px for the focused `.focusable()` node; visually confirmed) | focus rings on text-input, button, icon-button, checkbox, radio-group, switch, select, slider, segmented-control, tabs, fields, popover/menu triggers, resize-handle, scroll-shell, link, … (**34** — but most are NOT a gap; the ring renders) | ring renders on focus; 55/56 components mark `.focusable()` (slider fixed) | **residual only:** outset `box-shadow 0 0 0 3px` fidelity (vs the 2px border-recolor) via a `focus_style.shadow` merge; + host focus-state mgmt (interaction-loop bucket). Not a missing primitive. |
| 2 | **token-driven / custom / inset / multi-layer box-shadow** | **single-layer LANDED** (`JsEl::shadow(offset_x,offset_y,blur,spread,color)` exists; Poodle resolves `ELEVATION_*` ShadowValue tokens via `theme_ext::elevation_*`); still missing **inset** + **stacked layers** | elevation fidelity on surface, card, popover, menu, dialog, drawer, toast-stack/host, all dropdown overlays; **inset** selection rings on segmented-control, tabs, tri-state-switch, list-card sash (**55**) | overlays/modals/raised surfaces now draw the **token-accurate single-layer** `elevation.surface/overlay/dialog` (preset→token swap done, matches GPUI); multi-layer stacks + inset rings still collapse to one layer (inset rings drawn as a 1px border) | `.shadow(BoxShadow{…,inset})` + `.shadow_layers(vec![..])` for the multi-layer/inset residual; the structured-token `.elevation(token)` equivalent is now wired in Poodle's `theme_ext` |
| 3 | **font-family channel** | **DONE 2026-06-22** (offscreen-verified) | code, code-input, kbd chips (menu/context-menu/menubar/command-palette/action-discovery), audio/video time labels, duration-input, markdown-editor, slug text-input, metric-tile, color-picker hex (**~15 wired**) | — | `JsEl::font_family(FontFamily::{Sans,Mono})` landed (sans default; cosmic-text Family). 15 components now request Mono where the contract specifies code-family; sans already resolved to a system sans. Residual: a *specific* embedded sans (Inter) for exact parity vs the system fallback — not blocking. |
| 4 | **letter-spacing / tracking** | **DONE 2026-06-22** (offscreen-verified) | eyebrow (0.12/0.04em), badge, table header, field-set legend, meta-item, order-by, nav-card, sidebar-nav (0.18em), code lang, page-header, button (0.01em), calendar, pill, … (**~20 wired**) | — | `JsEl::letter_spacing_em(f32)` landed (em, scales with font size); contract-exact values applied per component; eyebrow pulls `spec.letter_spacing_em()`. |
| 5 | **dashed / dotted border-style** | missing (`dashed`/`BorderStyle` 0 refs — border builders set width only) | region (dashed frame), file-upload + dock-region drop-zones, list-card not-live, empty-state, time-ago + text-link dotted underline (**6**) | rendered as solid border (right width/color, wrong style) | `.border_style(BorderStyle::{Solid,Dashed,Dotted})` |

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
| 10 | **filter: grayscale** | missing | list-card "not-live" desaturation | niche — workaround is opacity dim. |
| 11 | **z-index channel** | missing (`z_index` 0 refs; layering via `overlay`/`anchor_to` render order) | toast-host stacking order | low — render-order works; only matters for cross-overlay z control. |
| 12 | **text-transform: uppercase** | missing | eyebrow, badges, table headers | low — already handled in-component via `.to_uppercase()`; only costs original-casing for copy/AX. |
| 13 | **percent width on div** | missing (`w`/`flex_basis` take px f32) | range-slider between-fill, card-toggle/field-set column widths | low — worked around with fixed-px flex segments / chunked rows. A `w_pct(f32)`/`flex_basis_pct(f32)` would simplify. |
| 14 | **container queries / viewport units** | missing | responsive collapse: form-actions danger overflow, dialog `max-height: 80vh`, toast narrow-breakpoint, tabs overflow→menu | medium-complex — needs a measure→relayout pass. Web-parity-only; desktop targets render the desktop arm. |

## Suggested order

1. ~~P1-1 focus-visible~~ and ~~P1-2 box-shadow builder/elevation~~ — **both largely DONE.**
   Focus rings already render (P1-1 corrected); custom shadow + token elevation landed (P1-2).
   The two biggest "looks unfinished" deltas are mostly closed. Residual: outset focus-ring
   fidelity + multi-layer/inset shadow — both niceties, deferrable.
2. **P1-3/4/5** (font-family, letter-spacing, dashed) — small, independent, typography/border
   fidelity.
3. **P2-6 animation** — large but high perceived value.
4. **P3-8 accessibility** — its own epic, coordinate platform-wide.
5. The rest (P3-9..14) are niche/accepted; do opportunistically.

## Verification once a capability lands

For each delivered runtime feature, the Poodle side: (a) replace the workaround in the affected
`js_*` components, (b) add/update `render_probe` assertions where observable, (c) rebuild
`jetstream/preview`, (d) drop the "accepted delta" note from the affected `docs/parity/*.md`.
Components and the specific approximation per gap are greppable: `rg -l '<keyword>' docs/parity/`.
