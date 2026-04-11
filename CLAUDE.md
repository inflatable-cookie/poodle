# CLAUDE.md

## What Poodle Is

Poodle is a **production UI component library** intended to be consumed by real applications (Loophole, Underlay apps, and future projects). It is not a prototype, demo, or learning exercise.

The system has one contract surface (`docs/contracts/`) and multiple implementation targets:
- **Svelte** (`packages/svelte/`) — web apps
- **GPUI** (`packages/gpui/`) — Zed-based desktop
- **Jetstream** (`packages/jetstream/`) — custom game-engine desktop

All implementations must faithfully implement the same component contracts. The contracts define anatomy, props, states, token usage, accessibility, and visual rules. Implementations that deviate from contracts are bugs.

## Quality Standards

### No Mockups, No Fakes

**Never create mockup or placeholder implementations.** Every component in every preview app must be a real, working component that resolves all visual properties from the token system through the component's Spec.

A specimen that hand-codes colors, sizes, or layout instead of resolving them from the token system is **worse than having no specimen at all** — it hides incomplete work and makes debugging harder. If a component's spec or token resolution is incomplete, leave the specimen unimplemented until the spec properly resolves the component's tokens.

### Contracts Are The Source of Truth — NO EXCEPTIONS

**Before writing any component code, read the contract in `docs/contracts/components/<component>.md` from start to finish.** The contract is the authoritative specification. Every implementation decision must be traceable to a contract requirement. If something is not in the contract, do not invent it. If something IS in the contract, it MUST be implemented.

The contract defines:
- **Anatomy**: Which DOM/element parts exist (root, indicator, label, panel, etc.) — every part must be present
- **Props**: Every prop, its type, and its default value — the Spec struct must match exactly
- **Token targets**: Which semantic token controls each visual property — hardcoding the resolved value is NEVER acceptable
- **States**: hover, active, focus, disabled, loading, etc. — each must be handled
- **Accessibility**: ARIA attributes, roles, keyboard behavior — these are not optional
- **Sizing**: Exact dimensions in rem/px from tokens — do not guess or approximate

**Checklist for every component implementation:**
1. ☐ Read the full contract before writing any code
2. ☐ Every dimension resolves from a token (height, padding, gap, radius, font-size) — ZERO hardcoded px values
3. ☐ Every color resolves from a token via the Spec's token methods
4. ☐ Anatomy matches contract (all parts present, correct nesting)
5. ☐ All props from contract are supported in the Spec
6. ☐ Disabled/loading states reduce opacity via `disabled_opacity_token()`, not a hardcoded value
7. ☐ Focus ring implemented where contract requires it
8. ☐ ARIA attributes applied (role, aria-label, aria-expanded, etc.)

### Size And Density Contract

Size and density are orthogonal axes:
- **Size** controls intrinsic dimensions: height, vertical padding, font-size, icon-size, border-radius. Size changes make a component physically larger or smaller.
- **Density** controls spacing between siblings: horizontal padding, gaps between items, margins between list rows. Density changes make a layout tighter or looser without changing component height.

**Density must never affect vertical padding or component height.** If a density variant overrides `padding-top`, `padding-bottom`, `padding-block`, `min-height`, or `height`, that is a bug — those properties belong to size variants. The only exception is when vertical padding has a distinct compositional meaning for density on a case-by-case basis (e.g. a panel's internal vertical padding between sections), which must be explicitly justified.

### Token Resolution Is Mandatory

Components must resolve their visual properties from the semantic token system, not hardcode values:
- Colors come from `theme.resolve_color(spec.some_fill_token())`
- Spacing comes from `theme.resolve_space(spec.some_gap_token())`
- Radii come from `theme.resolve_radius(spec.radius_token())`
- Typography comes from token-defined font sizes, weights, and families
- Dimensions come from token-defined sizes (control-height, min-width, icon-size)

Hardcoded pixel values like `.h(16.0)` or `.text_size(13.0)` in component code are **always wrong**. The correct form is `.h(resolve_px(theme, spec.control_height_token()))`. If a token doesn't exist for a value specified in the contract, add the token — do not hardcode the value.

### Reference Implementation

The **Svelte implementation** (`packages/svelte/primitives/src/`) is the proof reference for visual correctness. When the contract is ambiguous, refer to the Svelte implementation for clarification. The GPUI implementation may have deviations and should not be used as a reference.

## Development Workflow Rules

- Any changes to Svelte components in `packages/svelte/primitives/src/` must be reflected in the corresponding contract file in `docs/contracts/components/`. For example, changes to `Button.svelte` should be mirrored in `docs/contracts/components/button.md`. Always keep component implementations and their contracts in sync.

- When implementing a Jetstream component:
  1. **Read the full contract** (`docs/contracts/components/<component>.md`) — every section
  2. Cross-reference the **Svelte implementation** for visual reference
  3. Ensure `poodle_specs` Spec struct has every prop and token method the contract requires
  4. Implement `js_<component>()` in `packages/jetstream/components/src/` resolving ALL values from tokens
  5. Verify: zero hardcoded pixel values, all anatomy parts present, ARIA attributes applied
  6. Write the specimen in `packages/jetstream/preview/src/specimens/` showing all contract states

- The preview apps (`packages/svelte/preview/`, `packages/jetstream/preview/`) exist to **test that components work correctly**. They are integration tests, not showcases for fake UI.

## Architecture Quick Reference

### Component Pipeline (Jetstream)

```
ComponentSpec (e.g. ButtonSpec)
    + JetstreamThemeProvider (token resolution)
        ↓ js_button(spec, theme) → JsEl
            (fluent builder: div().h(height).bg(fill).child(...))
        ↓ game_ui.render_immediate(&root_el)
            (materialize JsEl tree → UiTree → Taffy layout → draw commands)
```

### Key Crates

- `poodle-tokens` — semantic token definitions, themes (dark, light, loophole-studio)
- `poodle-specs` — component spec structs (ButtonSpec, CheckboxSpec, etc.)
- `poodle-adapter` — `ThemeProvider` trait
- `poodle-jetstream` (adapter) — `JetstreamThemeProvider` for token resolution
- `poodle-jetstream-components` — Jetstream component implementations (`js_button`, `js_checkbox`, etc.)
- `jetstream-runtime::ui_element` — `JsEl` fluent builder, `div()`, `label()`, `button()`, `list()`

### Layout Mapping

`LayoutIntent` → `poodle_jetstream::map_layout()` → `taffy::Style`

Key rules:
- `LayoutSizing::Grow` on both axes → `flex_grow: 1` (fills remaining space)
- `LayoutSizing::Grow` on one axis only → `flex_grow: 0`, relies on `align_self: Stretch`
- `LayoutSizing::Fixed(n)` → explicit size, no grow, no shrink
- `min_size` defaults to `0` (not `auto`) so containers can be constrained by parents
