# Source Hub: GPUI

Status: active (findings documented)
Created: 2026-03-11
Updated: 2026-03-11

## Purpose

Document GPUI's component model, styling system, and capabilities to inform:
- Token emission rules for Rust consumers (g01.002, g01.003)
- GPUI substrate policy (g01.006)
- Parity expectations between Svelte and GPUI implementations
- What GPUI can/cannot do compared to browser-based UI

---

## Source Inventory

### Official Sources

| Source | URL | Type | Last Checked |
|--------|-----|------|--------------|
| GPUI Docs | https://github.com/zed-industries/zed/tree/main/crates/gpui/docs | Official | 2026-03-11 |
| Zed Source | https://github.com/zed-industries/zed | Reference impl | 2026-03-11 |
| GPUI Examples | https://github.com/zed-industries/zed/tree/main/crates/gpui/examples | Code samples | 2026-03-11 |
| DeepWiki GPUI | https://deepwiki.com/zed-industries/zed/2.2-ui-framework-(gpui) | Community docs | 2026-03-11 |

### Community Resources

| Source | URL | Type | Notes |
|--------|-----|------|-------|
| gpui-component (Longbridge) | https://github.com/longbridge/gpui-component | Component library | Stock trading UI components |
| GPUI CE Fork | https://github.com/gpui-ce/gpui-ce/ | Community fork | Community-maintained GPUI |

---

## Key Findings

### Architecture Overview

GPUI is a **hybrid immediate and retained mode, GPU-accelerated UI framework for Rust**.

**Core Concepts:**
1. **Entity** - State management: App owns all state, UI components interact through handles/context
2. **View** - Declarative UI: High-level components implementing `Render` trait
3. **Element** - Imperative UI: Low-level building blocks for custom drawing/layout

**Key Files in Zed Source:**
- `crates/gpui/src/styled.rs` - Styling trait definitions
- `crates/gpui/src/geometry.rs` - Pixels, DevicePixels, ScaledPixels types
- `crates/gpui/src/window.rs` - Window management
- `crates/gpui/src/platform.rs` - Platform abstraction

### Component Model

**Basic Component Structure:**
```rust
// From gpui-component Button example
#[derive(IntoElement)]
pub struct Button {
    pub base: Div,
    id: ElementId,
    icon: Option<Icon>,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    selected: bool,
    variant: ButtonVariant,
    // ... etc
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}
```

**Key Patterns:**
- Components implement `IntoElement` or `Render` trait
- Use `Div` as base container (similar to HTML `<div>`)
- State via `Model<T>` with `observe`/`notify` or `subscribe`/`emit` patterns
- Event handling through callbacks with `ClickEvent`, `WindowContext`

### Styling System

**Tailwind-inspired API:**
```rust
div()
    .flex()
    .gap_2()
    .p_4()
    .bg(cx.theme().background)
    .rounded_md()
    .child("Content")
```

**StyleRefinement:**
```rust
StyleRefinement::default()
    .rounded_xl()
    .py_3()
    .px_4()
    .border_2()
    .border_color(cx.theme().accent)
```

**Theme Access:**
- Via `cx.theme()` in render context
- Returns theme struct with semantic color roles
- GPUI components access tokens through the theme, not direct token references

### Token System Analysis

**How GPUI Handles Design Tokens:**

1. **Theme-Centric**: GPUI uses a `Theme` struct that contains semantic color roles:
   - `cx.theme().background`
   - `cx.theme().accent`
   - `cx.theme().border`
   - etc.

2. **No Direct Token Consumption**: GPUI does not natively consume external JSON token files
   - Tokens are compiled into Rust code (Theme struct)
   - Style values are Rust types (not CSS values)

3. **For Pug's Token Emission:**
   - **Option A**: Generate Rust Theme struct from Pug tokens
   - **Option B**: Generate constants/enums that GPUI theme can reference
   - **Option C**: Runtime token map (less idiomatic for GPUI)

**Recommended Approach for Pug:**
- Emit Rust code defining semantic color structs and constants
- GPUI implementation constructs Theme from these
- Keep raw scale tokens for documentation/reference

### Layout System

**Flexbox-based:**
- `.flex()`, `.flex_col()`, `.flex_row()`
- `.gap_2()`, `.p_4()`, `.m_2()` (Tailwind-like spacing)
- `.items_center()`, `.justify_between()`

**Geometry Types:**
- `Pixels` - Logical, DPI-independent
- `DevicePixels` - Physical screen pixels
- `ScaledPixels` - Custom scaling factor

**Sizing:**
- Automatic handling of DPI/scale factors per window
- `logical_size.scale(scale_factor)` → `DevicePixels`

### Event & Input Handling

**Event Types:**
- `ClickEvent` - Mouse clicks
- `KeyboardEvent` - Key presses
- `FocusEvent` - Focus changes
- `MouseMoveEvent`, etc.

**Actions:**
- Action-based input decoupling (similar to command pattern)
- Actions can be bound to keybindings
- `cx.dispatch_action(action)`

**Focus Management:**
- Built-in focus system
- Tab navigation handled by GPUI
- Can programmatically set focus

### Primitive Components

**Built-in Elements:**
- `div()` - Primary container
- `label()` - Text labels
- `input()` - Text input
- `button()` - Buttons
- `img()` - Images
- `svg()` - SVG graphics
- `canvas()` - Custom drawing

**From gpui-component Library:**
- Button, IconButton, Tab
- Input, TextArea, SearchInput
- Checkbox, Radio, Switch
- Dropdown, Menu, Popover
- Table, List, Resizable
- Notification, Tooltip, Modal
- Dock, Sidebar, ScrollView

### Workstation UI Capabilities

**Window Management:**
- `WindowDecorations::Server` - Platform draws title bar
- `WindowDecorations::Client` - GPUI draws custom title bar (macOS default)
- Custom traffic light positioning on macOS

**Dock/Panel Evidence:**
- gpui-component has `Dock` and `Resizable` components
- Split views via custom implementations
- No built-in workspace persistence (app must implement)

### Platform Deltas: GPUI vs Browser

**GPUI Can Do (that browsers struggle with):**
- Native performance, GPU-accelerated everything
- Custom window decorations
- Direct OS integration
- Fine-grained control over rendering
- Better memory control

**Browsers Can Do (that GPUI lacks):**
- Native form controls with OS integration
- Built-in accessibility tree (GPUI has basics but not as mature)
- CSS full feature set (animations, transforms, etc.)
- Web-native APIs (geolocation, camera, etc.)
- Massive ecosystem of libraries

**Parity Implications:**
- GPUI styling is code-based, not CSS-based
- Focus on semantic behavior match, not implementation match
- Some visual differences expected (GPUI's focus rings vs browser)
- State management patterns differ significantly

---

## Critical Questions Answered

### Token System (for g01.002, g01.003)

**Q: How does GPUI handle design tokens?**
A: Through a `Theme` struct with semantic color roles, not direct token consumption. Pug should emit Rust code defining these themes.

**Q: Can GPUI consume external token definitions?**
A: Not natively. Tokens must be compiled into Rust code. Runtime token loading is possible but not idiomatic.

**Q: How are color roles defined?**
A: As fields on the Theme struct: `background`, `accent`, `border`, etc.

### Component Model (for g01.006)

**Q: What is the idiomatic component structure?**
A: Struct implementing `IntoElement` or `Render`, with `Div` as base, using `ViewContext` for state.

**Q: How does state management work?**
A: `Model<T>` with `observe`/`notify` for simple changes, `subscribe`/`emit` for typed events.

**Q: How are events handled?**
A: Callbacks with event types (`ClickEvent`, etc.) and `WindowContext` for operations.

### Primitive Capabilities

**Q: What built-in input components exist?**
A: Basic elements (div, input, button). Rich components in gpui-component library.

**Q: How are overlays implemented?**
A: Via `Popover`, `Modal` in gpui-component. No native overlay in core GPUI.

---

## Implications for Pug

### Token Emission (g01.002, g01.003)

**Recommended Strategy:**
1. Define canonical Pug token schema (DTCG-based)
2. Emit Rust code (structs/constants) for GPUI
3. GPUI implementation constructs Theme from emitted code
4. Keep TypeScript types and CSS variables for Svelte side

**Naming Conventions:**
- CSS: `--pug-color-background-primary`
- TypeScript: `tokens.color.background.primary`
- Rust: `pug::tokens::color::BACKGROUND_PRIMARY` or theme struct fields

### GPUI Substrate (g01.006)

**Key Policy Decisions:**
1. Pug components use `IntoElement` pattern
2. State via Pug-defined models (not ad hoc)
3. Styling through GPUI's Tailwind-like API
4. Theme integration via emitted Rust code

### Parity Definition

**What Parity Means:**
- Same semantic props/behavior
- Same accessibility guarantees
- Same state transitions
- Different styling implementation (expected)
- Different event handling internals (expected)

**Documented Deltas:**
- GPUI uses native focus rings (not customizable like CSS)
- GPUI animations different from CSS animations
- GPUI layout uses different constraint system than CSS

---

## Related

- Value track: [tk-gpui-idioms](./tk-gpui-idioms.md)
- Translation memo: [tm-token-system](../translation-memos/tm-token-system.md) (pending)
- Milestone: [g01.006](../../roadmaps/g01/006-gpui-substrate-and-rust-token-binding-baseline.md)
- Milestone: [g01.002](../../roadmaps/g01/002-token-system-and-artifact-emission.md)

---

## Next Task

Create translation memo synthesizing token emission strategy for g01.002/003.
