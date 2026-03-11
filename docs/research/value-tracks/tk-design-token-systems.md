# Value Track: Design Token Systems

Status: complete (findings documented)
Created: 2026-03-11
Updated: 2026-03-11
Priority: critical (blocks g01.002, g01.003)

## Purpose

Research design token taxonomy patterns, standards, and tooling to inform Pug's:
- Canonical token schema (g01.002)
- Artifact emission for CSS/TypeScript/Rust (g01.003)
- Naming conventions across platforms
- Theme and mode handling

---

## Key Findings

### W3C DTCG Format (v2025.10)

**Status:** First stable version released October 2025. Production-ready.

**Core Structure:**
```json
{
  "token-name": {
    "$type": "color",
    "$value": "#007bff",
    "$description": "Primary action color"
  }
}
```

**Token Types Supported:**
| Type | Use Case | Example |
|------|----------|---------|
| `color` | Colors | `#007bff`, `{colorSpace: "srgb", components: [0,0.5,1]}` |
| `dimension` | Spacing, sizing | `16px`, `1rem` |
| `fontFamily` | Typography | `"Inter", sans-serif` |
| `fontWeight` | Typography | `400`, `bold` |
| `duration` | Animation timing | `200ms` |
| `cubicBezier` | Animation easing | `[0.4, 0, 0.2, 1]` |
| `number` | Numeric values | `1.5`, `0.5` |
| `strokeStyle` | Border styles | `solid`, `dashed` |
| `border` | Composite borders | `{width, style, color}` |
| `shadow` | Box shadows | `{x, y, blur, spread, color}` |
| `gradient` | Gradients | Multiple color stops |
| `typography` | Composite typography | `{fontFamily, fontSize, fontWeight, lineHeight}` |
| `transition` | Composite transitions | `{duration, delay, timingFunction}` |

**Aliases/References:**
```json
{
  "color-primary": {
    "$type": "color",
    "$value": "{color.blue.500}"
  }
}
```

**Multi-File/Theming:**
- DTCG supports `$themes` for organizing tokens by theme
- Groups can have `$type` that applies to children
- Enables light/dark, density, brand variants

### Token Taxonomy Patterns

**Three-Layer Model (Industry Standard):**

1. **Primitives/Foundations** - Raw values
   - Color palette (blue-50, blue-100, ... blue-900)
   - Spacing scale (space-1, space-2, ... space-16)
   - Typography scale (text-xs, text-sm, ... text-4xl)

2. **Semantic Aliases** - Purpose-based
   - `color-background-primary`
   - `color-text-secondary`
   - `space-component-padding`

3. **Component Tokens** - Component-specific (optional)
   - `button-background-default`
   - `input-border-error`

**Best Practice:**
- Components should primarily use semantic aliases
- Raw scales for reference and custom overrides only
- Avoid component tokens unless truly necessary

### Style Dictionary 4.0

**Key Features:**
- First-class DTCG format support
- Async API, better error handling
- Token expansion for composite values
- Cross-platform transforms

**Supported Outputs:**
| Format | Use Case |
|--------|----------|
| `css/variables` | CSS custom properties |
| `scss/variables` | Sass variables |
| `javascript/es6` | JS module exports |
| `typescript/es6-declarations` | TS types |
| `ios-swift/class.swift` | Swift class |
| `android/resources` | Android XML |

**Custom Transforms:**
- Can create custom transforms for Rust emission
- Name transforms: `name/pathToCamelCase`, `name/pathToPascalCase`, `name/pathToKebabCase`

### Multi-Platform Emission Strategies

**Web (CSS):**
```css
:root {
  --pug-color-background-primary: #ffffff;
  --pug-color-text-primary: #1a1a1a;
}

[data-theme="dark"] {
  --pug-color-background-primary: #1a1a1a;
  --pug-color-text-primary: #ffffff;
}
```

**TypeScript:**
```typescript
export const tokens = {
  color: {
    background: {
      primary: 'var(--pug-color-background-primary)',
      secondary: 'var(--pug-color-background-secondary)',
    },
    text: {
      primary: 'var(--pug-color-text-primary)',
    },
  },
} as const;

export type Tokens = typeof tokens;
```

**Rust (for GPUI):**
```rust
// Option A: Constants
pub const COLOR_BACKGROUND_PRIMARY: Hsla = Hsla::new(1.0, 0.0, 1.0, 1.0);

// Option B: Theme struct
pub struct Theme {
    pub background: BackgroundColors,
    pub text: TextColors,
}

pub struct BackgroundColors {
    pub primary: Hsla,
    pub secondary: Hsla,
}
```

### Theme & Mode Handling

**CSS Approach:**
- CSS custom properties with attribute selectors
- `[data-theme="dark"]` overrides
- Media query support for `prefers-color-scheme`

**GPUI Approach:**
- Theme struct swapped at runtime
- GPUI handles DPI/scale separately
- No native CSS custom property equivalent

**Recommended Pug Strategy:**
- Define modes in DTCG: `light`, `dark`, `density` (compact/default/comfortable)
- Emit separate CSS files or CSS layers for each theme
- Emit Rust code for each theme variant
- Runtime theme switching handled by consuming app

---

## Recommendations for Pug

### Token Taxonomy

**Pug Token Hierarchy:**
```
tokens/
├── primitives/
│   ├── color/           # Raw color palette
│   ├── dimension/       # Spacing, sizing scales
│   └── typography/      # Font families, sizes
├── semantic/
│   ├── color/           # background, text, border, accent
│   ├── space/           # padding, margin, gap
│   └── typography/      # roles: body, heading, caption
└── modes/
    ├── light.json       # Light theme values
    ├── dark.json        # Dark theme values
    └── density.json     # Density variants
```

### Naming Conventions

| Platform | Token Path | Output Name |
|----------|------------|-------------|
| Source | `semantic.color.background.primary` | - |
| CSS | - | `--pug-color-background-primary` |
| TypeScript | `tokens.color.background.primary` | camelCase |
| Rust | `pug::semantic::color::BACKGROUND_PRIMARY` | SCREAMING_SNAKE_CASE |

### Emission Strategy

**Option 1: Style Dictionary (Recommended)**
- Use Style Dictionary 4.0 with custom Rust transform
- Single source of truth, multiple outputs
- Community support, DTCG compliant

**Option 2: Custom Tooling**
- Build custom emitter for maximum control
- Higher maintenance burden
- Only if Style Dictionary can't meet needs

### Format Choice

**Adopt W3C DTCG as canonical format because:**
1. Industry standard (Adobe, Figma, Sketch, Tokens Studio support)
2. Interoperability with design tools
3. Pug can accept DTCG tokens from external sources
4. Future-proof specification

---

## Related

- Source hub: [hub-gpui](../source-hubs/hub-gpui.md) (GPUI token consumption)
- Translation memo: [tm-token-system](../translation-memos/tm-token-system.md) (pending)
- Milestone: [g01.002](../../roadmaps/g01/002-token-system-and-artifact-emission.md)
- Milestone: [g01.003](../../roadmaps/g01/003-token-artifact-emission-themes-and-density-modes.md)

---

## Next Task

Create translation memo synthesizing token system recommendations for g01.002/003.
