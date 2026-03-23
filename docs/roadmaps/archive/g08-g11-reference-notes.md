# g08–g11 Reference Notes (Archived)

These notes were extracted from the original g08–g11 roadmap generations before
they were consolidated. The generations contained inflated completion claims but
also useful architectural decisions and constraint documentation preserved here.

---

## Jetstream Adapter Architecture

### Adapter Pipeline

The `flint-jetstream` adapter depends on 8 contract crates: `flint-tokens`,
`flint-layout`, `flint-events`, `flint-style`, `flint-adapter`, `flint-primitives`,
`flint-composites`, `flint-workstation`. The adapter owns the mapping between Flint
spec identity and Jetstream `UiNodeId` handles.

Core type mapping:
- `WidgetKind` enum: `Panel`, `Label`, `Button`, `Slider`, `ProgressBar`,
  `Image`, `List`, `TextInput`
- `JetstreamNodeHandle` carries: `node_id`, `spec_type`, `widget_kind`
- `JetstreamAdapter` stores the theme provider

### Token Bridge

`JetstreamThemeProvider` implements the `ThemeProvider` trait:

- `resolve_color`: hex parsing (`#RRGGBB`, `#RRGGBBAA`), `rgba()` parsing, or
  semantic constant matching against 19 typed constants
- `resolve_space`: rem→px conversion (×16), px passthrough, or plain number
- `resolve_radius` and `resolve_border_width`: delegate to `resolve_space`
- `resolve_opacity`: direct float parsing
- Scale factor via `with_scale_factor()` builder

The 19 semantic color constants: `COLOR_ACCENT_BASE`, `COLOR_ACCENT_HOVER`,
`COLOR_ACCENT_FOCUS_RING`, `COLOR_BACKGROUND_CANVAS/SURFACE/PANEL/OVERLAY/ELEVATED`,
`COLOR_TEXT_PRIMARY/SECONDARY/INVERSE`, `COLOR_BORDER_SUBTLE/DEFAULT/STRONG`,
`COLOR_STATUS_SUCCESS/WARNING/DANGER`, `COLOR_ICON_PRIMARY/MUTED`.

### Style Mapping

`JetstreamSizing` enum: `Fixed(f32)`, `Grow`, `Fit` — maps from `LayoutSizing`.
Special case: `Constrained` sizing approximated as `Fixed` at the midpoint.

---

## Jetstream Rendering Constraints

- **Layout**: Flexbox-like only. No CSS Grid. Grid specs emulated with nested
  row/column Panel nodes.
- **Text**: Single-style runs, LTR, Latin/common scripts. No rich text, no
  complex shaping, no IME.
- **Colors**: Solid colors only in the adapter. Game_ui layer supports linear
  gradients, but adapter uses solid approximations.
- **Images**: GPU texture handles. No SVG rendering.
- **Shadows**: One box shadow per `UiNode`. No stacked or inset shadows.
- **Transforms**: None. No rotation, scale, or skew.
- **Scrolling**: Vertical scroll with clipping. No momentum or snap.
- **Input**: Keyboard, mouse, gamepad. No touch. No OS clipboard. No IME.
- **No ARIA**: Not applicable in game engine context. Gamepad navigation is
  Jetstream-unique (D-pad/stick → focus, A → confirm, B → cancel).

---

## Delta Register: Jetstream

Parity classification pattern:

**Structural delta (all components)**: Retained-mode UiTree rendering differs
from DOM-based layout in font rendering, anti-aliasing, sub-pixel behavior.

**Visual deltas**: Surface, Banner, CallOut, Tooltip render with flat solid
fills instead of gradients. Dialog, Drawer, HoverCard, Popover render with
only the primary shadow.

**Behavioral deltas**:
- TextInput, TextArea, SearchField, PinInput, EditableLabel — visual-only,
  no OS clipboard or multi-cursor
- FileUpload — renders drop-zone visual only, cannot trigger native file dialogs
- Date pickers — calendar grids static, popup interaction not wired

---

## Parity Tier Vocabulary

- **Strict parity**: Visual and behavioral match between runtimes
- **Visual parity**: Visual match with minor behavioral differences
- **Native adaptation**: Intentional deviation justified by platform constraint

---

## GPUI-Specific Notes

- GPUI uses `gpui::Svg` via alpha mask — `stroke="white"` required for
  visibility (black = zero luminance = invisible)
- `FlintIcon` wraps SVG rendering with text_color application
- CSS `opacity` vs color alpha: element-level `.opacity()` works like CSS
  opacity; reducing individual color alpha makes dark-on-dark invisible
- `color_mix(a, b, ratio)` helper for hover/active blending
- Font rendering: static weight TTF files (Regular 400, Medium 500,
  SemiBold 600, Bold 700) registered in GPUI's font system
- `TabStripSpec` → `FlintTabStrip` has no separate Svelte counterpart

---

## Workstation Substrate Boundary Decisions (g11)

**NOTE**: Workstation components are being consolidated into primitives and
composites. These boundary decisions may be partially deprecated but the
Flint-vs-downstream distinction remains useful.

**Belongs in Flint**:
- Window host model, surface identity, surface-to-window ownership
- Region grammar and layout snapshots
- Strip rails (all four edges, icon-first and mixed-content modes)
- Resize handles, split dividers, collapse affordances
- Dock active-panel emphasis
- Panel variant system (utility/standard/focused)
- Hosted-surface container with bounded states

**Must stay downstream / out of Flint**:
- DAW semantics (transport, timeline, mixer, automation, etc.)
- Window management policy (which surfaces open, default positions)
- Panel assignment policy (which panels go where, pinning rules)
- Strip item semantics (which icons, what they activate)
- Plugin lifecycle
- Command routing and keyboard shortcuts
- Project file paths, save/load behavior

---

## Performance Target

Adapter overhead target for Jetstream: under 1ms for 100-node UI trees.
Not yet measured; flagged as future profiling work.

---

## What Actually Shipped (Honest Record)

### g08 (Jetstream Rendering Build-Out)
- Theme bridge (`theme.rs`) — solid, 381 lines, full ThemeProvider impl
- Layout mapper (`style_map.rs`) — solid, 510 lines
- 8 real component implementations (button, accordion, checkbox, switch,
  badge, progress, separator, status_indicator)
- ~100 adapter stubs returning type strings with no component-specific rendering
- 20 real component tests, ~150 stub assertion tests
- Demo scene structure (4 screens, shallow renders)

### g09 (GPUI Component Build-Out)
- 98 component files — all real rendering code, all Partial quality
- Colors mostly token-resolved, dimensions universally hardcoded
- 79 specimen files in preview app
- Working preview app with navigation, theme switching, display controls
- 199 tests (data/logic, not visual)
- Zero focus rings, minimal ARIA

### g10 (Jetstream Preview App)
- 8 specimen files matching the 8 real components
- Preview app shell with section navigation
- Demo and Tokens sections are "coming soon" placeholders

### g11 (Workstation Contracts)
- 18 workstation contract files — comprehensive
- Svelte workstation components built then consolidated into composites
- GPUI and Jetstream implementations deferred
