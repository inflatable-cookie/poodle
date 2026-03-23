# Pug GPUI Preview App — Visual Quality Pass (Handover)

## Context
You're continuing a systematic visual quality pass of the Pug design system's GPUI preview app. The goal is to make the GPUI preview **pixel-equivalent to the Svelte preview** — matching visual appearance, interactivity, and state management across all ~85 specimen pages.

## Project Structure
- **Repo root**: `/Users/betterthanclay/Dev/projects/pug`
- **GPUI preview app**: `/packages/gpui/preview/` — run with `cargo run` from that directory
- **Svelte preview app**: `/packages/svelte/preview/` — run with `npm run dev` (or similar)
- **GPUI components**: `/packages/gpui/components/src/primitives/` and `.../composites/`
- **Svelte components**: `/packages/svelte/primitives/src/` and `/packages/svelte/composites/src/`
- **Svelte specimens**: `/packages/svelte/preview/src/specimens/`
- **GPUI specimens**: `/packages/gpui/preview/src/specimens/`
- **Shared contracts/specs**: `/packages/contracts/primitives/src/` and `.../composites/src/`
- **Compile from**: `cd /packages/gpui/preview && cargo check`

## Architecture
- Components use a **spec pattern**: shared Rust specs (e.g., `ButtonSpec`) define data/tokens, GPUI components resolve tokens via `GpuiThemeProvider` at render time
- Specimens have two signatures: `render(theme: &GpuiThemeProvider)` (static) or `render(state: &AppState, cx: &mut Context<PreviewRoot>)` (interactive)
- Interactive state uses `cx.listener(|this, val: &T, _w, cx| { ... })` — note `&T` reference parameter
- `SpecimenState` tracks toggles (`is_on`/`toggle`), text values, and selections

## Key Patterns & Gotchas
- **GPUI has no CSS text-transform** — uppercase must be done in Rust (`to_uppercase()`)
- **GPUI has no CSS letter-spacing** — known limitation for Eyebrow component
- **GPUI has no CSS gradients** — solid colors approximate where Svelte uses gradients
- **GPUI focus ring**: Svelte uses `outline` with offset; GPUI approximates with `border_color` + `shadow` ring (spread_radius 2px, 28% opacity)
- **Interactive divs need `.id("name")` to use `.on_click()`**
- **Button active state**: Do NOT use `.mt()` — it shifts all content below
- **Absolute positioning**: `.absolute().inset_0()` is relative to nearest positioned ancestor, not viewport
- **Callback types**: `&str` ✓, `&ClickEvent` ✓, `&bool` ✓, `&f64` ✓ | `usize` by value ✗, `bool` by value ✗
- **Color mixing**: `color_mix(a, b, ratio)` blends ratio% of `a` with (1-ratio)% of `b`
- **Theme helpers**: `resolve_color()`, `resolve_px()`, `resolve_radius()`, `resolve_opacity()`, `color_mix()`, `color_mix_black()` in `theme_ext`

## What's Been Fixed (This Session)
All committed and pushed to main:

1. **Accordion** — rewritten to card-style items with bordered containers, tinted bg, title+description in trigger
2. **AlertDialog** — fixed black backdrop, constrained dialog width 420px, inline centered layout
3. **BulkActionBar** — wired click handlers via Rc, added element IDs, hover states
4. **Button** — removed `.mt(px(1.0))` active state that caused layout shift
5. **Calendar** — added `w(px(288.0))` width constraint
6. **Tabs (Pill variant)** — reduced padding/gap to match Svelte (3px container, 2px gap, height - 8px tabs)
7. **Dialog** — title 16px (was 18), width 544px, gap 6px, actions use stack-lg margin
8. **Switch** — token-derived track/thumb sizes, Svelte-matching shadow
9. **Callout** — circular icon bg container, 13px content text, styled dismiss button
10. **Tooltip** — 11px font, 256px max-width, 6px symmetric padding
11. **Collapsible** — 16px title, correct bg mix (surface 88% + text-primary), 42% border opacity
12. **Popover** — 224px min-width
13. **StatusIndicator** — 999px border-radius (was 4.5px!), shadow ring
14. **Slider** — 6px track (was 4px), elevated thumb bg, border-default thumb border
15. **Meter** — success color fill (was accent), track bg with text-primary
16. **RadioGroup** — token-derived indicator size
17. **Table** — header fill uses surface 91% + text-primary
18. **Card** — shadow ring for selected (was border_2), 52% footer divider
19. **Surface** — inset shadow ring on non-elevated surfaces
20. **TextInput** — focus ring shadow + validation state borders
21. **Checkbox** — focus ring shadow
22. **NumberEntry** — Icon stepper buttons (was text symbols), validation, focus ring
23. **Pill** — auto-uppercase badge labels
24. **DataTable** — 120px action column (was 80px), 12px cell padding

## What Still Needs Work
- **Every specimen page needs visual comparison** against Svelte — we've been fixing components but haven't done page-by-page visual verification
- **Focus ring pattern** needs applying to remaining controls (Select, Combobox, Switch, etc.)
- **Drawer** border styling (edge-only vs all-around)
- **StatusIndicator** pending pulse animation
- **Rating** hover state (should be bg highlight, not just color change)
- **FormLayout** uses flexbox where Svelte uses CSS Grid
- **SearchField** delegates entirely to TextInput — needs proper search icon + clear button
- User's exact words: **"We're still miles off done"** and **"there should be no 'mock up' components in the preview, everything needs to work and render just like the Svelte version"**

## Approach
The plan is to go through each specimen page in both the GPUI app and the Svelte app, compare visually, and fix discrepancies one page at a time. The Svelte app can be previewed in a browser. The GPUI app should have screenshot functionality built in.
