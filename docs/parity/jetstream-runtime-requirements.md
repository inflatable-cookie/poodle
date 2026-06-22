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

## What the runtime already has (corrects stale parity notes)

Several older parity notes claim gaps that no longer exist — impls can use these **today**, no
engine work:

- **Gradients** — `bg_gradient_linear`, `bg_gradient_radial` exist. (progress fill, media-frame
  radial, toast tone-gradient can use these instead of flat approximations.)
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
| 2 | **token-driven / custom / inset / multi-layer box-shadow** | partial (`BoxShadow` + `style.shadow` exist; public builder is only `shadow_sm/md/lg` presets — no inset, no per-token offsets, no stacked layers) | elevation fidelity on surface, card, popover, menu, dialog, drawer, toast-stack/host, all dropdown overlays; **inset** selection rings on segmented-control, tabs, tri-state-switch, list-card sash (**55**) | `shadow_md()` preset stands in for `elevation.overlay/dialog`; inset rings drawn as a 1px border | `.shadow(BoxShadow{offset,blur,spread,color,inset})` + `.shadow_layers(vec![..])`; ideally a `.elevation(token)` that resolves the structured `ELEVATION_*` ShadowValue (GPUI already does this) |
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
