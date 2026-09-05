<!-- parity consv=ok gpui=1 jetstream=1 specimen=ok -->
# Parity: ThemeSelect

> Status line above is machine-read. `consv` = contract↔Svelte (`ok`/`fixed`/`gap`);
> `gpui`/`jetstream` = open-todo counts; `specimen` = `ok`/`gap`.

## Sources

- Contract: `docs/contracts/components/theme-select.md`
- Svelte (authoritative): `packages/svelte/components/src/ThemeSelect.svelte` (+ controller `packages/svelte/components/src/theme-controller.ts`)
- React: `packages/react/components/src/ThemeSelect.tsx` (+ `packages/react/components/src/theme-controller.tsx`)
- GPUI: `packages/gpui/components/src/primitives/theme_select.rs`
- Jetstream: `packages/jetstream/components/src/theme_select.rs`
- Spec: `packages/contracts/components/src/theme_select.rs` (`ThemeSelectSpec` + `ThemeOption` + `ThemeSwatch`)
- Data source: `packages/svelte/tokens/src/theme-options.ts` (`themeOptions()`, framework-neutral)
- Specimens: svelte `packages/svelte/preview/src/specimens/ThemeSelectSpecimen.svelte` · react `packages/react/preview/src/gallery/specimens/ThemeSelectSpecimen.tsx` · gpui `packages/gpui/preview/src/specimens/theme_select_specimen.rs` · jetstream `packages/jetstream/preview/src/specimens/theme_select.rs`

## Contract ↔ Svelte

`consv=ok`. New component; contract authored alongside the Svelte implementation.

- [x] Swatch-tile popover; trigger shows the current theme's swatch + label + chevron.
- [x] Selection resolution: controlled `value` / theme controller / uncontrolled; `onChange` fires in every mode. Verified headless.
- [x] Modular controller (`createThemeController` / `getThemeController`) reads `themeOptions()`, applies `data-theme` via `applyThemeAttributes`, persists to localStorage. Component works standalone without it.
- [x] Six agreed states + size/density; `columns`, `showLabel`, unknown-value fallback.

React is interface-invariant (same props/types; own local type copy + `ThemeControllerProvider`/`useThemeController`). Both web targets verified headless (open, 9 tiles, select updates value + re-themes, zero console errors). The preview app header now uses ThemeSelect (replacing the 3-tab toggle), offering all registered themes app-wide.

## GPUI gap (vs Svelte + contract)

- [x] Full render from `ThemeSelectSpec`: trigger swatch + label + chevron, popover grid of swatch tiles (canvas + surface card + accent dot + text bar), selected accent ring + check, size table. Swatch hex via `parse_hex_color`; chrome from spec token methods. Build-verified (`cargo check`).
- [ ] Live event-loop wiring only (open/select): the render is a faithful function of the spec; a host drives clicks. Shared render-only posture. Combinator of the theme (application) is host-owned — native apps switch their own `GpuiThemeProvider`.
- accepted: no ARIA (GPUI has no accessibility API).
- accepted: swatch colors are sRGB→linear approximations of the literal hex.

## Jetstream gap (vs Svelte + contract)

- [x] Same anatomy from `JsEl` (`js_theme_select`): trigger + swatch + popover grid; hex→linear `Vec4` via `hex_to_rgb255`; chrome from spec tokens. `#[cfg(test)] mod tests` render_probe: trigger shows the current label; open grid lists all theme labels. Build-verified (`cargo check`; probe tests share the pre-existing `metrics_c.rs` test-build limitation).
- [ ] Live event-loop wiring only — shared render-only posture.
- accepted: no ARIA; sRGB→linear swatch approximation.

## Specimen parity

- Svelte covers: standalone live-value picker, compact (no label), four columns, disabled, size ladder, density variants.
- React covers: the same set one-to-one.
- GPUI covers: open picker, disabled, sizes, densities.
- Jetstream covers: open picker, disabled, sizes, densities.

## Notes

- **New component (2026-07-15).** Motivation: the preview header's 3 theme tabs
  didn't scale to a growing theme set. Added 6 themes (midnight, nord, rose,
  forest, solarized, high-contrast → 9 total), an aggregate `poodle-themes.css`
  (one import loads every theme layer), and `themeOptions()` (framework-neutral
  swatch catalogue derived from the token metadata). The component renders a
  supplied catalogue and reports selection, so it works with or without the
  controller.
- Swatch colors come free from `themes[name].overrides` (resolved against base
  semantics for non-overridden paths, e.g. light's accent).
- Contrast axis: CSS-only; Rust artifacts stay literal.
- 3 `poodle-specs` unit tests (current option, selection, unknown-value fallback).
