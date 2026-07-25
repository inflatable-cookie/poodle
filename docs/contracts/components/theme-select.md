# ThemeSelect

Status: detailed contract
Updated: 2026-07-15

## 1. Purpose

- Component name: `ThemeSelect`
- Layer: `composites`
- Summary: a purpose-built theme picker — a trigger showing the current theme's
  swatch that opens a popover grid of theme swatch tiles (a mini preview of each
  theme's colors). Scales to many themes with visual flair a plain `Select` lacks.
- In scope: swatch-tile popover, controlled/uncontrolled/controller-driven
  selection, size/density scaling, an optional modular theme controller that
  serves the available Poodle themes and applies the selection
- Out of scope: authoring themes (that is token work), density/control-size
  selection (separate controls), theme persistence policy beyond the controller's
  optional localStorage

The component is decoupled from the controller: it renders a supplied theme
catalogue and reports selection, so it works in any context (a form, a settings
page) without the controller. The controller is a convenience that wires it to
the full Poodle theme set + DOM application.

## 2. Anatomy

```text
[Root .theme-select] <div role="group">  (position: relative, carries data-size/density/disabled/open)
  ├── [Trigger .theme-select__trigger] <button aria-haspopup="dialog" aria-expanded aria-controls>
  │   ├── [Swatch .theme-select__swatch] <span aria-hidden>  (current theme mini preview)
  │   │   ├── [Surface .theme-select__swatch-surface]  (card on the canvas)
  │   │   ├── [Accent .theme-select__swatch-accent]  (dot)
  │   │   └── [Text .theme-select__swatch-text]  (bar)
  │   ├── [Label .theme-select__label] <span>  (current theme name; hidden when showLabel=false)
  │   └── [Chevron .theme-select__chevron] <span aria-hidden>
  └── [Surface .theme-select__surface] <div role="dialog"> (rendered inline when open)
      └── [Grid .theme-select__grid] <div role="listbox">  (columns via --poodle-theme-select-columns)
          └── [Tile .theme-select__tile] <button role="option" aria-selected data-selected>  (repeated)
              ├── [Swatch .theme-select__swatch--tile]  (larger mini preview; a check overlay when selected)
              └── [Tile Label .theme-select__tile-label]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | relative wrapper; `role="group"`, `data-size`/`data-density`/`data-disabled`/`data-open` | — |
| Trigger | yes | opens the popover; shows the current swatch + label + chevron | `--poodle-size-control-height`, `--poodle-radius-control`, `--poodle-color-background-surface`, `--poodle-color-border-default` |
| Swatch | yes | mini preview using the theme's literal swatch colors (canvas/surface/accent/text); border from `border` swatch | (inline per-theme colors) |
| Label | no | current theme name; hidden when `showLabel=false` | `--poodle-color-text-primary` |
| Chevron | yes | popover indicator (`▾`) | `--poodle-color-text-secondary` |
| Surface | yes | anchored `role="dialog"` popover | `--poodle-overlay-z-menu`, `--poodle-radius-surface`, `--poodle-color-background-elevated`, `--poodle-elevation-overlay` |
| Grid | yes | `role="listbox"` swatch-tile grid; `columns` sets the column count | — |
| Tile | no | one selectable theme; `role="option"`, selected gets an accent ring + check | `--poodle-color-accent-base` (selected) |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `themes` | `ThemeOption[]` | controller's list | no | theme catalogue. Falls back to the theme controller's list, then empty. Use `themeOptions()` from `@poodle/svelte-tokens` for the full Poodle set |
| `value` | `string \| undefined` | controller / uncontrolled | no | controlled current theme value. Omit to use the controller (if present) or internal state |
| `columns` | `number` | `3` | no | swatch-tile grid columns |
| `showLabel` | `boolean` | `true` | no | show the current theme name in the trigger |
| `ariaLabel` | `string` | `"Theme"` | no | accessible name for the group, trigger, and listbox |
| `disabled` | `boolean` | `false` | no | disables the control |
| `size` | `ControlSize \| null` | `null` | no | explicit size override |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role |
| `density` | `ControlDensity \| null` | `null` | no | explicit density override |
| `onChange` | `((value: string) => void) \| null` | `null` | no | fired when a theme is selected |

### Shared Types

```typescript
type ThemeSwatch = { canvas: string; surface: string; accent: string; text: string; border: string };
type ThemeOption = { value: string; label: string; description?: string; swatch: ThemeSwatch };
```

`themeOptions()` (from `@poodle/svelte-tokens`) returns every registered Poodle
theme as a `ThemeOption`, with swatch colors resolved from the theme's token
overrides (falling back to the base semantic value).

### Controlled / Uncontrolled / Controller

Resolution order for the current value and selection sink:

1. **Controlled** — `value` provided → the component reflects it; `onChange` reports selection.
2. **Controller** — a theme controller in context (no `value`) → the component reads its current theme and calls `setTheme` on select.
3. **Uncontrolled** — neither → internal state.

`onChange` always fires on selection, in every mode.

## 4. Modular Theme Controller

An optional controller serves the available themes, holds the current selection,
applies it to the DOM (`data-theme` via `applyThemeAttributes`), and optionally
persists it (localStorage). It is framework-specific:

- **Svelte**: `createThemeController(config?)` (call in a root component's script;
  publishes context) + `getThemeController()`. `ThemeControllerConfig`:
  `{ themes?, initial?, target?, persistKey? }` (`persistKey` defaults to
  `"poodle-theme"`, `null` disables persistence; `target` defaults to
  `document.documentElement`).
- **React**: `<ThemeControllerProvider …>` + `useThemeController()`, same config.

`ThemeSelect` auto-consumes the controller when present. The controller reads
`themeOptions()` by default, so it serves the full Poodle theme set with zero
per-app wiring. Consuming apps load the theme CSS layers once — import
`@poodle/svelte-tokens/themes.css` (every theme in one aggregate).

## 5. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | — | trigger shows current swatch + label + chevron |
| open | click trigger | popover grid of swatch tiles; the current theme's tile has an accent ring + check; focus moves to it |
| disabled | `disabled=true` | reduced opacity; non-interactive |
| unknown value | `value` not in `themes` | trigger label falls back to "Theme"; no swatch |

Escape and outside interaction dismiss the popover (dismissable-layer). Behavior
classification: styled-only (no machine) — popover open/dismiss on the shared
dismissable-layer stack; selection is plain state.

## 6. Accessibility

- Root `role="group"`, `aria-label` from `ariaLabel`
- Trigger: `aria-haspopup="dialog"`, `aria-expanded`, `aria-controls`, accessible
  name `"{ariaLabel}: {current label}"`
- Popover `role="dialog"` (`tabindex="-1"`); grid `role="listbox"`; tiles
  `role="option"` with `aria-selected`
- Swatches are `aria-hidden` (decorative); the tile's text label names the option
- On open, focus moves to the selected tile (or first); Escape / outside-click
  dismiss

## 7. Layout

- Trigger: `inline-flex`, size-stepped `min-height` from control-height
- Swatch: fixed mini rect (trigger `1.25rem`; tile `2.75×2rem`), literal per-theme
  colors, `overflow: hidden`, positioned surface/accent/text children
- Surface: portalled and viewport-positioned (`002-anchored-overlays.md`),
  requesting `bottom-start` with an `8px` offset
- Grid: `repeat(columns, …)`, `max-height` with vertical scroll for many themes

## 8. Token Usage

Chrome (trigger, surface, tile selection) resolves from semantic tokens; swatch
colors are the theme's literal values (the whole point of a preview). See the
Anatomy token-target column.

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`–`"xl"` |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` |
| `data-disabled` | Root | `"true"` / `"false"` |
| `data-open` | Root | `"true"` when open |
| `data-selected` | Tile | `"true"` for the current theme |

## 9. Svelte Notes

- owns open state + anchored popover; `registerDismissLayer` from `@poodle/headless`
- size/density via `getUiPresentation` + `resolveSemanticControlSize`
- controller via `getThemeController()` context; falls back to props
- swatch colors applied inline (`style="background:…"`) — per-theme literals

## 10. GPUI Notes

- expected crate/module surface: `poodle_gpui::primitives::theme_select`
- render is a faithful function of `ThemeSelectSpec` (themes + value + open):
  trigger swatch + label + chevron, and a popover grid of tiles when open. Swatch
  hex parsed via `parse_hex_color`; chrome from tokens. The controller is web-only
  (native apps switch their own `GpuiThemeProvider`); render-only build-verified
- no ARIA API — accessible-name intent documented only

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] `themes` catalogue accepted; each renders a swatch tile
- [ ] current value: controlled / controller / uncontrolled resolution
- [ ] selecting a tile fires `onChange` (and updates value / controller)
- [ ] selected tile gets an accent ring + check
- [ ] `columns` sets the grid column count
- [ ] `showLabel=false` hides the trigger label
- [ ] disabled suppresses interaction
- [ ] unknown value → "Theme" trigger label

### Tier 2: Visual Parity

- [ ] trigger dimensions/border/radius/background match the control treatment
- [ ] swatch mini-preview layout (canvas + surface card + accent dot + text bar)
- [ ] surface anchoring/radius/elevation
- [ ] all five sizes

### Tier 3: Implementation Freedom

- [ ] popover/animation/portal behavior is platform-owned
- [ ] the controller is web-only; native theme application is host-owned

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| Controller is web-only (Svelte/React) | applies `data-theme`; native apps switch their own ThemeProvider | accepted | — |
| GPUI/Jetstream preview doesn't drive live selection | shared render-only posture — render is a faithful function of the spec; host wires interaction | accepted | — |
| Native swatch colors sRGB→linear approximated | build-verified native color pipeline | accepted | — |

## 13. Approval And Adoption Notes

- contract status: `implemented`
- downstream adopters: the Poodle preview app header; consuming apps offering theme choice
- future follow-up: theme grouping / search when the catalogue grows large

## 14. Specimen Definitions

Standalone (explicit `themeOptions()` + local value): live-value picker, compact
trigger (no label), four columns, disabled, size ladder, density variants. The
preview app header uses the controlled integration (all registered themes).
