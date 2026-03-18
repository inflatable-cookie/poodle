# CLAUDE.md

## What Pug Is

Pug is a **production UI component library** intended to be consumed by real applications (Loophole, Underlay apps, and future projects). It is not a prototype, demo, or learning exercise.

The system has one contract surface (`docs/contracts/`) and multiple implementation targets:
- **Svelte** (`packages/svelte/`) — web apps
- **GPUI** (`packages/gpui/`) — Zed-based desktop
- **Jetstream** (`packages/jetstream/`) — custom game-engine desktop

All implementations must faithfully implement the same component contracts. The contracts define anatomy, props, states, token usage, accessibility, and visual rules. Implementations that deviate from contracts are bugs.

## Quality Standards

### No Mockups, No Fakes

**Never create mockup or placeholder implementations.** Every component in every preview app must be a real, working component that goes through the adapter pipeline:

1. Create the component's `Spec` (e.g., `ButtonSpec`) with proper props
2. Pass it through `adapter.render(&spec, &style, theme)` to resolve tokens
3. Materialize the adapter output into platform nodes via `render_bridge`

A specimen that hand-codes colors, sizes, or layout instead of resolving them from the token system is **worse than having no specimen at all** — it hides incomplete work and makes debugging harder. If a component's adapter implementation is incomplete, leave the specimen unimplemented until the adapter properly resolves the component's tokens.

### Contracts Are The Source of Truth

- Read the contract in `docs/contracts/foundation/` before implementing any component
- The contract defines the anatomy (which parts exist), the token targets (which semantic tokens control each visual property), the props, states, and accessibility requirements
- Implementation must match the contract — not approximate it, not simplify it, not invent alternatives

### Token Resolution Is Mandatory

Components must resolve their visual properties from the semantic token system, not hardcode values:
- Colors come from `theme.resolve_color(spec.some_fill_token())`
- Spacing comes from `theme.resolve_space(spec.some_gap_token())`
- Radii come from `theme.resolve_radius(spec.radius_token())`
- Typography comes from token-defined font sizes, weights, and families

Hardcoded colors like `Vec4::new(0.2, 0.3, 0.4, 1.0)` are never acceptable in component implementations.

## Development Workflow Rules

- Any changes to Svelte components in `packages/svelte/primitives/src/` must be reflected in the corresponding contract file in `docs/contracts/foundation/`. For example, changes to `Button.svelte` should be mirrored in `docs/contracts/foundation/button.md`. Always keep component implementations and their contracts in sync.

- When implementing a Jetstream component:
  1. Read the contract (`docs/contracts/foundation/<component>.md`)
  2. Ensure `pug_primitives` has a complete `Spec` struct matching the contract props
  3. Implement `RenderComponent<Spec>` in the adapter with proper token resolution
  4. Write the specimen using the adapter pattern (see `specimens/button.rs` as the reference)
  5. Add `materialize_*` helpers in `render_bridge.rs` if needed

- The preview apps (`packages/svelte/preview/`, `packages/jetstream/preview/`) exist to **test that components work correctly**. They are integration tests, not showcases for fake UI.

## Architecture Quick Reference

### Adapter Pipeline (Jetstream)

```
ComponentSpec + StyleDescriptor + ThemeProvider
        ↓ adapter.render()
JetstreamNodeHandle { layout: taffy::Style, visuals: JetstreamVisuals }
        ↓ render_bridge::materialize()
UiTree node (Widget + taffy::Style + NodeStyle)
```

### Key Crates

- `pug-tokens` — semantic token definitions, themes (dark, light, loophole-studio)
- `pug-primitives` — component spec structs (ButtonSpec, CheckboxSpec, etc.)
- `pug-adapter` — `RenderComponent` trait, `ThemeProvider` trait
- `pug-jetstream` (adapter) — Jetstream adapter implementing `RenderComponent` for each spec
- `pug-layout` — `LayoutIntent` abstraction shared across platforms
- `pug-style` — `StyleDescriptor` for visual property descriptions

### Layout Mapping

`LayoutIntent` → `pug_jetstream::map_layout()` → `taffy::Style`

Key rules:
- `LayoutSizing::Grow` on both axes → `flex_grow: 1` (fills remaining space)
- `LayoutSizing::Grow` on one axis only → `flex_grow: 0`, relies on `align_self: Stretch`
- `LayoutSizing::Fixed(n)` → explicit size, no grow, no shrink
- `min_size` defaults to `0` (not `auto`) so containers can be constrained by parents
