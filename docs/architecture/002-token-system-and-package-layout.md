# 002 Token System And Package Layout

Status: active
Updated: 2026-03-11
Depends on: `001-poodle-system-shape.md`

## Purpose

Turn the token roadmap from an abstract design task into a concrete repository
layout and implementation plan, including a translatable theme system that can
feed both browser/Svelte and GPUI consumers from one source.

This document specifies the concrete package structure and build toolchain for
the token system architecture defined in `001-poodle-system-shape.md`.

## Research Inputs

This note should stay aligned with:

- `docs/research/translation-memos/tm-token-system.md`
- `docs/research/source-hubs/hub-gpui.md`
- `docs/research/value-tracks/tk-design-token-systems.md`

## Working Repository Shape

Poodle should add the token system through a small number of explicit top-level
workspaces rather than through ad hoc framework folders.

Recommended first layout:

```text
poodle/
  docs/
  packages/
    tokens/
      schema/           # DTCG format source files
      scripts/          # Bun + TypeScript token build scripts
      artifacts/        # Generated outputs (CSS, TS, Rust)
    svelte/
      tokens/           # Imports from packages/tokens/artifacts/
      primitives/
      composites/
      workstation/
    gpui/
      tokens/           # Imports from packages/tokens/artifacts/rust/
      primitives/
    implementations/    # Reserved for future targets
      react/
      desktop/
    bridges/
      underlay/
```

This is still a planning shape, not a commitment to the final build toolchain.
The important part is ownership clarity:

- `packages/tokens/` owns source-of-truth token definitions and emitted
  artifacts
- `packages/svelte/` owns Svelte consumption of emitted tokens
- `packages/gpui/` owns Rust/GPUI consumption of emitted tokens
- `packages/implementations/` is reserved for future implementation targets if
  and when they become real
- `packages/bridges/underlay/` owns Underlay-specific mapping and ingestion

## Token Package Boundary

The first implementation tranche should split `packages/tokens/` into three
responsibility areas:

### `packages/tokens/schema/`

Owns canonical token source files in **W3C DTCG format**.

Planned files:

```text
packages/tokens/schema/
  primitives/
    color.json          # blue-50, blue-100, ..., neutral-900
    dimension.json      # space-1, space-2, ..., size-px
    typography.json     # font-sans, font-mono, text-xs, text-sm
    radius.json
    border.json
    elevation.json
    motion.json
    icon.json
  semantic/
    color.json          # background-primary, text-secondary, border-default
    space.json          # padding-sm, gap-md
    typography.json     # body, heading, caption roles
  modes/
    themes/
      light.json        # Light theme semantic mappings
      dark.json         # Dark theme semantic mappings
      loophole-studio.json
    density/
      compact.json
      comfortable.json
    control-size/
      sm.json
      md.json
      lg.json
  metadata/
    aliases.json
    deprecations.json
    themes.json
  manifest.json         # DTCG group definitions and build metadata
```

**DTCG Format Example:**
```json
{
  "background-primary": {
    "$type": "color",
    "$value": "{primitives.color.white}",
    "$description": "Primary background color"
  },
  "text-primary": {
    "$type": "color", 
    "$value": "{primitives.color.gray.900}",
    "$description": "Primary text color"
  }
}
```

The important split is:

- `primitives/` owns raw scales
- `semantic/` owns role-based aliases consumed by components
- `modes/` owns theme, density, and control-size overlays
- `metadata/` owns aliases and lifecycle data without becoming a second
  semantic source

### `packages/tokens/scripts/`

Owns token compilation and emission on the JavaScript side using **Bun and
TypeScript**, with Style Dictionary-compatible planning retained where useful.

Planned files:

```text
packages/tokens/scripts/
  build-tokens.ts           # Main build entrypoint
  validate-tokens.ts        # Schema validation
  sd.config.ts              # Style Dictionary configuration
  transforms/               # Custom transforms
    name-transforms.ts      # Path to naming conventions
    rust-transform.ts       # Rust-specific value transforms
  formats/                  # Custom formats
    rust-format.ts          # Rust struct/constant generation
  actions/                  # Post-build actions
    copy-to-gpui.ts         # Copy Rust artifacts to GPUI package
```

The likely first implementation path is a single Bun-executed TypeScript
entrypoint that drives CSS and TypeScript output plus a Rust-facing emission
path for GPUI consumers. If Style Dictionary remains the chosen library, it
should still be configured from TypeScript rather than plain JavaScript.

**Style Dictionary Configuration:**
```ts
// sd.config.ts
export default {
  source: ['schema/**/*.json'],
  platforms: {
    css: {
      transformGroup: 'css',
      buildPath: '../artifacts/css/',
      files: [{
        destination: 'poodle-tokens.css',
        format: 'css/variables'
      }]
    },
    ts: {
      transformGroup: 'js',
      buildPath: '../artifacts/ts/',
      files: [{
        destination: 'index.ts',
        format: 'javascript/es6'
      }]
    },
    rust: {
      transforms: ['name/pathToPascalCase', 'rust/color'],
      buildPath: '../artifacts/rust/',
      files: [{
        destination: 'mod.rs',
        format: 'rust/module'
      }]
    }
  }
};
```

### `packages/tokens/artifacts/`

Owns generated consumer-facing artifacts. **These files are generated - do not edit manually.**

Planned files:

```text
packages/tokens/artifacts/
  css/
    poodle-tokens.css              # Base tokens as CSS custom properties
    poodle-theme-iceberg.css       # Iceberg theme overrides
    poodle-theme-eclipse.css       # Eclipse theme overrides
    poodle-density-compact.css     # Compact density overrides
    poodle-density-comfortable.css # Comfortable density overrides
  ts/
    index.ts                    # Token object exports
    themes.ts                   # Theme object exports
    metadata.ts                 # Source path and lifecycle metadata
  rust/
    mod.rs                      # Module exports
    primitives/                 # Primitive constants
    semantic/                   # Semantic constants
    themes.rs                   # Theme structs
    density.rs                  # Density helpers
    metadata.rs                 # Source path and lifecycle metadata
```

**CSS Output Example:**
```css
/* poodle-tokens.css */
:root {
  --poodle-color-blue-500: #3b82f6;
  --poodle-color-background-primary: var(--poodle-color-white);
  --poodle-color-text-primary: var(--poodle-color-gray-900);
}

/* poodle-theme-eclipse.css */
[data-theme="eclipse"] {
  --poodle-color-background-primary: var(--poodle-color-gray-900);
  --poodle-color-text-primary: var(--poodle-color-white);
}
```

**Rust Output Example:**
```rust
// semantic/color.rs
pub const BACKGROUND_PRIMARY: Hsla = Hsla::new(1.0, 0.0, 1.0, 1.0);
pub const TEXT_PRIMARY: Hsla = Hsla::new(0.1, 0.0, 0.1, 1.0);

// themes.rs
pub struct Theme {
    pub background: BackgroundColors,
    pub text: TextColors,
}

pub struct BackgroundColors {
    pub primary: Hsla,
    pub secondary: Hsla,
}
```

## Shared Styles Package (`packages/styles`)

Added g12.001 (2026-07-13). All component stylesheets live in
`@poodle/styles` — plain global CSS, one kebab-case file per component,
unique `poodle-*` classes, data-attribute states, recipe hooks. Both web
frameworks import from it (`import "@poodle/styles/button.css"`), so React
parity never duplicates styling and the Svelte implementation remains the
single visual proof reference. Svelte components carry no `<style>` blocks.

## Contrast Axis (Neutral Ramp Knob)

Added 2026-07-13. The CSS artifacts emit neutral background and border
tokens as contrast-scalable values controlled by one inherited custom
property:

```css
--poodle-contrast: 0.5; /* 0.4 = flat … 0.5 = library default … 1 = full theme ramp … 1.6 = accentuated */
```

The library default is **0.5** (softened ramp) — the raw theme literals
correspond to `--poodle-contrast: 1`.

- **Opaque neutrals** (`color.background.canvas|surface|panel|elevated`,
  opaque borders): emitted via relative color syntax, scaling oklch
  lightness distance from the theme's canvas anchor —
  `oklch(from <literal> calc(anchor + (l - anchor) * k) c h)`. Each theme
  block emits `--poodle-contrast-anchor-l` (build-time oklch L of its
  canvas), so the canvas stays fixed and the elevation ramp compresses or
  expands around it. Overriding the anchor variable switches the pivot.
- **Translucent neutrals** (dark/loophole border whites): alpha multiplies
  by `max(0.4, k)` — the floor keeps borders from vanishing at low k.
- **Untouched**: accent, status, text, and overlay tokens.

At k = 1 everything computes pixel-identical to the literal theme values
(Playwright-verified); unset, the 0.5 default applies. The knob is a plain inherited custom
property: set it app-wide, per-view, or animate it. The Rust/TS artifacts
keep literal values; the GPUI and Jetstream theme providers apply the same
math numerically (`poodle_headless::color::apply_neutral_contrast`, OKLab
constants matching browser relative-color rendering, conformance-tested
against browser-computed values) with a `contrast` field defaulting to 0.5
and a `with_contrast` builder. The GPUI preview exposes a Contrast toggle.

## Svelte Consumption Layout

The Svelte side should not define parallel token meaning. It should import
artifacts or generated data from `packages/tokens/`.

Planned first files:

```text
packages/svelte/tokens/
  index.ts                    # Re-exports from tokens/artifacts/ts/
  runtime.ts                  # Runtime theme switching utilities
  css.ts                      # CSS variable references
```

**Example Svelte Token Consumption:**
```typescript
// packages/svelte/tokens/index.ts
export { tokens, type TokenPath } from '../../tokens/artifacts/ts';
export { themes, type Theme } from '../../tokens/artifacts/ts/themes';
```

Theme handling on the Svelte side should therefore look like "apply translated
theme artifacts from the canonical theme source", not "define the theme again
in CSS by hand".

The same rule should keep Svelte package APIs from becoming the canonical shape
for all future web implementations.

## GPUI Consumption Layout

The GPUI side should receive generated Rust modules rather than hand-maintained
duplicate values.

Planned first files:

```text
packages/gpui/tokens/
  src/
    lib.rs                      # Re-exports from tokens/artifacts/rust/
    theme_ext.rs                # GPUI-specific Theme extensions
    bridge.rs                   # GPUI Theme integration
```

**GPUI Theme Integration:**
```rust
// packages/gpui/tokens/src/theme_ext.rs
use poodle_tokens::semantic::color;
use gpui::Theme;

impl Theme {
    pub fn poodle_light() -> Self {
        Self {
            background: color::BACKGROUND_PRIMARY,
            text: color::TEXT_PRIMARY,
            // ...
        }
    }
}
```

The generated Rust artifacts from `packages/tokens/artifacts/rust/` can either
be copied into or included by this package, but ownership should remain with
the token generator path.

Theme handling on the GPUI side should therefore look like "consume Rust-facing
theme artifacts generated from the canonical theme source", not "recreate the
theme manually in Rust".

The same rule should keep GPUI package APIs from becoming the canonical shape
for all future desktop-oriented implementations.

The intended GPUI realization is generated Rust modules and theme helpers that
GPUI packages assemble into idiomatic `Theme` usage without becoming a second
canonical token source.

## Build Integration

### NPM Scripts (packages/tokens/)

```json
{
  "scripts": {
    "build": "style-dictionary build",
    "build:css": "style-dictionary build --platform css",
    "build:ts": "style-dictionary build --platform ts", 
    "build:rust": "style-dictionary build --platform rust",
    "watch": "style-dictionary build --watch",
    "clean": "rm -rf artifacts/"
  }
}
```

### CI Integration

Token build should run in CI to verify:
- Schema validity (DTCG format)
- No missing token references
- All platforms emit successfully
- Generated files are up to date

### Pre-commit Hook

```bash
# Verify tokens build successfully before commit
npm run --prefix packages/tokens build
git add packages/tokens/artifacts/
```

## Future Implementation Slot

Poodle should not build React or other additional targets now, but the package
layout should leave room for them.

If a future target becomes real, it should land under a dedicated package such
as:

```text
packages/implementations/react/
packages/implementations/tauri-ui/
packages/implementations/egui/
```

Those packages should consume the same token artifacts and component contracts
rather than creating a second canonical contract path.

## Underlay Bridge Layout

Underlay integration should live behind a bridge layer that maps Poodle semantics
into Underlay-owned conventions.

Planned first files:

```text
packages/bridges/underlay/
  css/
    poodle-to-underlay.css     # CSS variable mapping
  ts/
    token-map.ts            # Token name mapping
    theme-map.ts            # Theme value mapping
    component-wrappers.ts   # Underlay-facing component wrappers
```

This keeps the public Underlay API stable while still allowing Underlay to
consume Poodle tokens internally.

It also means an Underlay consumer should be able to ingest a Poodle-authored
theme through Underlay-owned mapping without needing a separate hand-authored
theme definition for the same brand or app.

## First Implementation Batch

The first token implementation batch should only create:

- [ ] the `packages/tokens/` shape with DTCG schema files
- [ ] Style Dictionary 4.0 configuration and build scripts
- [ ] schema files for the core token families (color, spacing, typography)
- [ ] schema files for the first shared theme definitions (light, dark)
- [ ] CSS, TypeScript, and Rust artifact outputs for initial slice
- [ ] custom Rust format for Style Dictionary
- [ ] Svelte and GPUI token-consumer stubs

It should not yet attempt:

- the full primitive catalogue
- full theme branding support
- downstream bridge completion
- density mode variants (can be added later)

## Rule

No framework-specific package should introduce canonical token values directly.
Canonical values must originate in the token schema layer and flow outward.

The same rule applies to themes: framework-specific packages may realize a
theme, but they may not become the source of truth for the theme.

The same rule also applies to component semantics: implementation packages may
realize a contract, but they may not redefine the canonical contract to fit one
runtime only.

## Next Implementation Bias

The first implementation tranche only needs to prove:

- one DTCG-authored token schema can emit CSS, TypeScript, and Rust artifacts
- one named semantic theme can translate into browser and GPUI outputs from the
  same source
- Svelte can consume emitted artifacts without redefining token meaning
- GPUI can consume emitted artifacts without redefining token meaning

## References

- [001-poodle-system-shape.md](./001-poodle-system-shape.md) - Token system specification
- [../research/translation-memos/tm-token-system.md](../research/translation-memos/tm-token-system.md) - Token system research decisions
- [W3C DTCG Specification](https://www.designtokens.org/tr/2025.10/) - Canonical token format
- [Style Dictionary](https://styledictionary.com/) - Multi-platform emission tool

## Next Task

1. Set up `packages/tokens/` directory structure
2. Install Style Dictionary 4.0
3. Create initial DTCG schema files (primitives/color.json, semantic/color.json)
4. Implement custom Rust format
5. Configure build scripts
6. Generate first artifact outputs
7. Update g01.002 and g01.003 milestone checklists
