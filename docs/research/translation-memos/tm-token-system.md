# Translation Memo: Poodle Token System

Status: draft
Created: 2026-03-11
Updated: 2026-03-11
Target: g01.002, g01.003

## Summary

Poodle will adopt the W3C Design Tokens Community Group (DTCG) format as the canonical token specification. Tokens will be organized in a three-layer hierarchy (primitives → semantic → component) and emitted to CSS/TypeScript for Svelte and Rust code for GPUI using Style Dictionary 4.0 with custom transforms.

---

## Sources

| Source | Link | Relevant Findings |
|--------|------|-------------------|
| W3C DTCG Spec | https://www.designtokens.org/tr/2025.10/ | Stable spec, industry adoption |
| Style Dictionary | https://styledictionary.com/ | Multi-platform emission tooling |
| hub-gpui | [../source-hubs/hub-gpui.md](../source-hubs/hub-gpui.md) | GPUI Theme-based token consumption |
| tk-design-token-systems | [../value-tracks/tk-design-token-systems.md](../value-tracks/tk-design-token-systems.md) | Token taxonomy research |

---

## Decisions

### 1. Canonical Format: W3C DTCG

**Decision:** Poodle's source of truth for tokens will be W3C DTCG format (version 2025.10 or later).

**Rationale:**
- Industry standard with broad tool support (Figma, Tokens Studio, Style Dictionary)
- Enables interoperability with design tools
- Stable specification (first stable release October 2025)
- Adopted by major design systems (Adobe, Salesforce, IBM, GitHub, Shopify)

**Implications:**
- Design tools can export directly to Poodle's format
- Token files are JSON with `$type`, `$value`, `$description` properties
- Token aliases use `{path.to.token}` syntax

### 2. Token Taxonomy: Three-Layer Hierarchy

**Decision:** Tokens will be organized in three layers: primitives, semantic, and modes.

**Rationale:**
- Industry best practice (Atlassian, Salesforce, Carbon all use similar models)
- Provides clear abstraction levels
- Semantic tokens enable theming without component changes

**Structure:**
```
tokens/
├── primitives/
│   ├── color.json          # blue-50, blue-100, ..., neutral-900
│   ├── dimension.json      # space-1, space-2, ..., size-px
│   ├── typography.json     # font-sans, font-mono, text-xs, text-sm
│   └── ...
├── semantic/
│   ├── color.json          # background-primary, text-secondary, border-default
│   ├── space.json          # padding-sm, gap-md
│   └── ...
└── modes/
    ├── light.json          # Theme values for light mode
    ├── dark.json           # Theme values for dark mode
    └── density.json        # Compact/comfortable spacing
```

**Implications:**
- Component contracts reference semantic tokens only
- Primitives are for documentation and custom overrides
- Modes are overlays that swap semantic token values

### 3. Naming Conventions

**Decision:** Token paths use dot notation; platform outputs transform to idiomatic naming.

| Layer | Token Path | CSS Output | TS Output | Rust Output |
|-------|------------|------------|-----------|-------------|
| Primitive | `primitives.color.blue.500` | `--poodle-blue-500` | `primitives.color.blue[500]` | `poodle::primitives::color::BLUE_500` |
| Semantic | `semantic.color.background.primary` | `--poodle-color-background-primary` | `tokens.color.background.primary` | `poodle::semantic::color::BACKGROUND_PRIMARY` |

**Rationale:**
- Consistent source identifiers across platforms
- Platform conventions honored in outputs
- Rust uses SCREAMING_SNAKE_CASE for constants

**Implications:**
- Style Dictionary transforms handle name conversion
- Token paths are stable API (breaking change to rename)

### 4. Emission Tool: Style Dictionary 4.0

**Decision:** Use Style Dictionary 4.0 for token emission with custom Rust transform.

**Rationale:**
- First-class DTCG support in v4.0
- Proven multi-platform emission (CSS, SCSS, JS, TS, Swift, Android)
- Custom transform support for Rust
- Active maintenance and community

**Alternative Considered:**
- **Cobalt**: Lighter weight, but less mature Rust ecosystem
- **Custom tool**: Higher maintenance burden, not justified at this stage

**Implications:**
- Need to develop custom Rust format/transform
- SD configuration in `sd.config.js`
- Build step required before token consumption

### 5. GPUI Token Consumption: Rust Code Generation

**Decision:** Emit Rust code (structs/constants) for GPUI, not runtime token loading.

**Rationale:**
- GPUI uses compile-time Theme structs (from hub-gpui research)
- Rust's type system benefits from compile-time tokens
- Runtime token loading is possible but not idiomatic in GPUI

**Emitted Rust Structure:**
```rust
// Generated from tokens - do not edit manually
pub mod semantic {
    pub mod color {
        use gpui::Hsla;
        
        pub const BACKGROUND_PRIMARY: Hsla = Hsla::new(1.0, 0.0, 1.0, 1.0);
        pub const BACKGROUND_SECONDARY: Hsla = Hsla::new(0.97, 0.0, 0.97, 1.0);
        // ...
    }
    
    pub mod spacing {
        pub const SM: f32 = 8.0;
        pub const MD: f32 = 16.0;
        // ...
    }
}

pub struct Theme {
    pub background: BackgroundColors,
    pub text: TextColors,
    // ...
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: BackgroundColors {
                primary: semantic::color::BACKGROUND_PRIMARY,
                // ...
            },
            // ...
        }
    }
}
```

**Implications:**
- GPUI implementation imports generated code
- Theme variants (light/dark) are separate generated structs
- Token changes require recompilation

### 6. Svelte Token Consumption: CSS Variables + TypeScript

**Decision:** Emit CSS custom properties and TypeScript types for Svelte.

**Emitted Files:**
```css
/* tokens.css */
:root {
  --poodle-color-background-primary: #ffffff;
  --poodle-color-background-secondary: #f5f5f5;
  --poodle-color-text-primary: #1a1a1a;
  /* ... */
}

[data-theme="dark"] {
  --poodle-color-background-primary: #1a1a1a;
  --poodle-color-background-secondary: #2a2a2a;
  --poodle-color-text-primary: #ffffff;
  /* ... */
}
```

```typescript
// tokens.ts
export const tokens = {
  color: {
    background: {
      primary: 'var(--poodle-color-background-primary)',
      secondary: 'var(--poodle-color-background-secondary)',
    },
    text: {
      primary: 'var(--poodle-color-text-primary)',
    },
  },
} as const;

export type TokenPath = 
  | 'color.background.primary'
  | 'color.background.secondary'
  | 'color.text.primary';
```

**Implications:**
- Svelte components import CSS and TS tokens
- Theme switching via `data-theme` attribute
- Type-safe token paths

### 7. Theme & Mode Strategy

**Decision:** Support light/dark themes and density modes (compact/default/comfortable).

**DTCG Structure:**
```json
{
  "semantic": {
    "color": {
      "background": {
        "primary": {
          "$type": "color",
          "$value": "{modes.light.color.background.primary}",
          "$extensions": {
            "mode": {
              "light": "{primitives.color.white}",
              "dark": "{primitives.color.gray.900}"
            }
          }
        }
      }
    }
  }
}
```

**Emission Strategy:**
- CSS: Separate variable sets with attribute selectors
- Rust: Separate Theme structs or mode-specific constants

**Implications:**
- Apps can switch themes at runtime
- GPUI theme swap requires re-render (GPUI handles this)
- Density affects spacing/dimension tokens

---

## Action Items

- [ ] Create `tokens/` directory structure in repo
- [ ] Set up Style Dictionary 4.0 configuration
- [ ] Develop custom Rust format for SD
- [ ] Define initial primitive token set (color, spacing, typography)
- [ ] Define semantic token set for first components
- [ ] Add token build step to CI
- [ ] Update architecture/001-poodle-system-shape.md with token rules
- [ ] Document token contribution guidelines

---

## Related

- Source hub: [hub-gpui](../source-hubs/hub-gpui.md)
- Value track: [tk-design-token-systems](../value-tracks/tk-design-token-systems.md)
- Milestone: [g01.002](../../roadmaps/g01/002-token-system-and-artifact-emission.md)
- Milestone: [g01.003](../../roadmaps/g01/003-token-artifact-emission-themes-and-density-modes.md)
