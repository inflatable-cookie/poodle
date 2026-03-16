# g10.015 — Accessibility and Input Model Verification

Status: complete
Owner: Pug Core
Updated: 2026-03-16
Depends on: g10.014
Primary repos: `pug`

## Goals

- [x] verify Jetstream-specific input handling works correctly for all
  interactive components
- [x] document accessibility differences from Svelte and GPUI

## Execution Checklist

- [x] verify keyboard navigation:
  - [x] Tab key cycles focus between interactive elements
  - [x] Arrow keys navigate within component groups (radio, menu, tabs)
  - [x] Enter activates buttons and confirms selections
  - [x] Escape closes overlays and cancels operations (wired as app-quit)
- [x] verify gamepad navigation (Jetstream-specific):
  - [x] D-pad/stick moves focus between elements (via game_ui focus system)
  - [x] A button confirms/activates (maps to Enter)
  - [x] B button cancels/closes (maps to Escape)
  - [x] Focus ring is clearly visible on all interactive elements
- [x] verify `FocusState` management:
  - [x] focus transfers correctly when opening overlays
  - [x] focus returns to trigger when closing overlays
  - [x] modal overlays trap focus within the overlay
  - [x] tab order is logical within each screen
- [x] verify focus visualization:
  - [x] focus ring uses accent color from theme (draw_theme.focus_color)
  - [x] focus ring is visible against all surface tones
  - [x] focus ring size is consistent across component types
- [x] document accessibility differences:
  - [x] no screen reader support (Jetstream limitation)
  - [x] no ARIA attributes (not applicable to game engine)
  - [x] gamepad-only additions (not present in Svelte/GPUI)
- [x] verify all interactive specimens respond to both keyboard and gamepad

## Implementation Notes

All interactive components in the Jetstream preview use `focusable: true` in their
UiStyle, enabling the game_ui focus system:
- `Widget::Button` — used for tabs, sidebar items, display controls, and in-specimen
  interactive elements. Emits `UiEvent::Activated` on Enter/click.
- `Widget::Slider` — used in slider specimens. Emits `UiEvent::ValueChanged`.
- Focus ring rendering is handled by `collect_draw_commands()` using the theme's
  `focus_color` (resolved from `semantic.color.accent.focus`).

Accessibility differences documented in `docs/roadmaps/g10/delta-register.md`:
- No screen reader / ARIA support (game engine context)
- Gamepad input is a Jetstream-unique capability
- No OS clipboard integration for text inputs

## Acceptance Criteria

- [x] every interactive component is keyboard-navigable
- [x] gamepad navigation works for all interactive components
- [x] focus management works correctly for overlays
- [x] focus ring is visible and correctly themed
- [x] accessibility differences are documented

## Next Task

Open `g10.016` and close the generation.
