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

- **P1-1 focus-visible** → add `.focus(\|s\|…)` override mirroring `.hover`/`.active`; `FocusState`
  + a focus-ring draw path already exist. (Small.)
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
  `bg_gradient_radial` (center + radius as **fractions of the rect**) exist and are used today
  (color_picker hue strip, toast tone-tint, progress). **Caveat (engine):** the radial path in
  `render_bridge.rs` packs only the *last* stop's color over the element's base bg — `stops[0]`
  is ignored, so a multi-stop radial (e.g. the media-thumbnail accent-at-top-left frame wash)
  can't render faithfully yet. That frame upgrade is deferred pending an engine fix + visual
  verification, NOT a missing-primitive gap.
- **Per-side borders + colors** — `border_t_/b_/l_/r_` + `border_color_{top,bottom,left,right}`.
  (table row borders, accordion, field-set legend, status-bar chrome — no need for all-sides hacks.)
- **Custom `BoxShadow`** — a `BoxShadow` type and `style.shadow` field exist (slider/tooltip set
  it directly); see P1-2 — only the *builder ergonomics + token-driven elevation + inset/multi-layer*
  are missing, not shadow itself.
- **Percent in layout** — taffy `LengthPercentage` is used internally; but the public `w(f32)` /
  `flex_basis(f32)` builders take **px**, so percent *width on a div* is still not reachable (P3-6).
- **Interaction events** — `on_click`, `on_drag`, `on_mouse_down`, `on_pointer_enter/leave`,
  `on_scroll`, `focusable` exist; widgets `text_input`, `slider`, `progress`, `rich_text`, `image`.

**Action item (Poodle-side, no engine work):** audit impls still flat-approximating gradients /
multi-side borders and upgrade them. Tracked separately from the engine asks below.

## P1 — high-leverage visual channels (concrete `JsEl` builder adds)

| # | Capability | Status | Unblocks (≈ docs) | Current workaround | Suggested API |
|---|---|---|---|---|---|
| 1 | **focus-visible style state** | missing (`.focusable()` sets tabindex but there is no `.focus(\|s\|…)` style state; only `.hover`/`.active`) | focus rings on text-input, button, icon-button, checkbox, radio-group, switch, select, slider, segmented-control, tabs, fields, popover/menu triggers, resize-handle, scroll-shell, link, … (**34**) | none — focus ring simply not drawn | `.focus(\|s\| s.border_color(..).shadow(..))` mirroring `hover`/`active`; engine paints it when the element holds keyboard focus (`FocusState` already exists in the runtime) |
| 2 | **token-driven / custom / inset / multi-layer box-shadow** | **single-layer LANDED** (`JsEl::shadow(offset_x,offset_y,blur,spread,color)` exists; Poodle resolves `ELEVATION_*` ShadowValue tokens via `theme_ext::elevation_*`); still missing **inset** + **stacked layers** | elevation fidelity on surface, card, popover, menu, dialog, drawer, toast-stack/host, all dropdown overlays; **inset** selection rings on segmented-control, tabs, tri-state-switch, list-card sash (**55**) | overlays/modals/raised surfaces now draw the **token-accurate single-layer** `elevation.surface/overlay/dialog` (preset→token swap done, matches GPUI); multi-layer stacks + inset rings still collapse to one layer (inset rings drawn as a 1px border) | `.shadow(BoxShadow{…,inset})` + `.shadow_layers(vec![..])` for the multi-layer/inset residual; the structured-token `.elevation(token)` equivalent is now wired in Poodle's `theme_ext` |
| 3 | **font-family channel** | missing (`font_family` 0 refs) | code, code-input (monospace cells), kbd chips (menu / command-palette / text-input shortcut / data-table), audio/video time labels (tabular/mono) (**12**) | default sans for everything; mono lost | `.font_family(family)` accepting the `typography.{label,body,code}.family` token (or a `FontFamily` enum Sans/Mono) |
| 4 | **letter-spacing / tracking** | missing | eyebrow (0.12em), badges (0.04/0.03em), table headers, section titles, kbd (resets tracking) (**19**) | dropped (visual-only, no functional loss) | `.letter_spacing(em: f32)` |
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

1. **P1-1 focus-visible** and **P1-2 box-shadow builder/elevation** — the two highest-impact
   visual gaps (34 + 55 docs), both concrete `JsEl` builder + render additions with existing
   substrate (`FocusState`, `BoxShadow`). These alone close the bulk of the "looks unfinished"
   deltas.
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
