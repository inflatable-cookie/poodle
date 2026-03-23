# 001 Flint System Shape

Status: active
Updated: 2026-03-11

## Overview

Flint should be structured around one contract surface and multiple
implementation/runtime surfaces.

## Research Inputs

This architecture baseline should stay aligned with:

- `docs/research/translation-memos/tm-token-system.md`
- `docs/research/translation-memos/tm-contract-template.md`
- `docs/research/translation-memos/tm-svelte-substrate.md`
- `docs/research/source-hubs/hub-gpui.md`
- `docs/research/source-hubs/hub-bits.md`

At minimum, the repo should grow into these ownership layers:

1. token source of truth
2. component contracts and docs
3. Svelte implementation
4. GPUI implementation
5. downstream bridges and artifact emitters
6. future implementation targets

## Package Boundary Model

The exact folder names can change, but the ownership split should remain:

- `tokens/`
  semantic design tokens, scales, aliases, translatable themes, and artifact
  generation
- `contracts/`
  component definitions, props, states, events, layout rules, accessibility
  rules, and parity notes
- `svelte/`
  Svelte implementations, using Bits where appropriate
- `gpui/`
  GPUI implementations against the same contracts
- `implementations/`
  optional future runtime targets such as React or other desktop UI kits when
  they become real requirements
- `bridges/underlay/`
  Underlay-facing token and component adaptation, owned so Underlay apps stay
  Flint-agnostic
- `docs/`
  Northstar vision, architecture, roadmap, logs, research, and specs

## Token System Specification

### Canonical Format: W3C DTCG

Flint adopts the W3C Design Tokens Community Group (DTCG) format (version 2025.10)
as the canonical token specification.

**Rationale:**
- Industry standard with broad tool support (Figma, Tokens Studio, Style Dictionary)
- Enables interoperability with design tools
- Stable specification (first stable release October 2025)
- Future-proof and implementation-neutral

**Token File Structure:**
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
    ├── light.json          # Light theme value mappings
    ├── dark.json           # Dark theme value mappings
    └── density.json        # Compact/comfortable spacing variants
```

### Three-Layer Taxonomy

**Layer 1: Primitives** - Raw values
- Color palette (blue-50 through blue-900)
- Spacing scale (space-1 through space-16)
- Typography scale (text-xs through text-4xl)
- Border radius, shadows, motion timing

**Layer 2: Semantic Aliases** - Purpose-based
- `color-background-primary`, `color-text-secondary`
- `space-component-padding`, `gap-md`
- `typography-body`, `typography-heading`

**Layer 3: Modes** - Theme/density variants
- Light, dark, high-contrast themes
- Compact, default, comfortable density

### Token Types (DTCG)

| Type | Use Case | Example |
|------|----------|---------|
| `color` | Colors | `#007bff`, OKLCH values |
| `dimension` | Spacing, sizing | `16px`, `1rem` |
| `fontFamily` | Typography | `"Inter", sans-serif` |
| `fontWeight` | Typography | `400`, `bold` |
| `duration` | Animation | `200ms` |
| `cubicBezier` | Easing | `[0.4, 0, 0.2, 1]` |
| `border` | Composite borders | `{width, style, color}` |
| `shadow` | Box shadows | `{x, y, blur, spread, color}` |
| `typography` | Composite type | `{family, size, weight, lineHeight}` |

### Artifact Emission: Style Dictionary 4.0

Flint uses **Style Dictionary 4.0** for multi-platform token emission.

**Platforms:**
| Platform | Output | Format |
|----------|--------|--------|
| CSS | CSS custom properties | `css/variables` |
| TypeScript | TS object + types | `javascript/es6` + declarations |
| Rust (GPUI) | Rust structs/constants | Custom format |

**Naming Conventions:**
| Layer | Token Path | CSS | TypeScript | Rust |
|-------|------------|-----|------------|------|
| Primitive | `primitives.color.blue.500` | `--flint-blue-500` | `primitives.color.blue[500]` | `flint::primitives::color::BLUE_500` |
| Semantic | `semantic.color.background.primary` | `--flint-color-background-primary` | `tokens.color.background.primary` | `flint::semantic::color::BACKGROUND_PRIMARY` |

**Emission Strategy:**
- CSS: CSS custom properties with `[data-theme]` selectors
- TypeScript: Object exports with CSS variable references
- Rust: Generated structs and constants (compile-time)

### GPUI Token Consumption

GPUI consumes tokens via **generated Rust code**, not runtime loading.

```rust
// Generated from tokens - do not edit manually
pub mod semantic {
    pub mod color {
        use gpui::Hsla;
        pub const BACKGROUND_PRIMARY: Hsla = Hsla::new(1.0, 0.0, 1.0, 1.0);
        // ...
    }
}

pub struct Theme {
    pub background: BackgroundColors,
    pub text: TextColors,
    // ...
}
```

**Rationale:**
- GPUI uses compile-time Theme structs
- Rust's type system benefits from compile-time tokens
- Runtime token loading is possible but not idiomatic

## Component Contract Specification

### Contract-First Development

All Flint components must have an approved contract document before
implementation begins.

**Workflow:**
1. Write contract document (following template)
2. Review and approve
3. Implement in Svelte
4. Implement in GPUI
5. Verify against parity checklist
6. Document any discovered deltas

### Contract Template (12 Sections)

Every component contract must include:

| # | Section | Contents |
|---|---------|----------|
| 1 | **Purpose** | What the component does |
| 2 | **Anatomy** | Structural parts with diagram |
| 3 | **Props/Inputs** | Configuration options with types |
| 4 | **States** | Visual states and component states |
| 5 | **Events** | Callbacks and when they fire |
| 6 | **Accessibility** | ARIA roles, keyboard behavior |
| 7 | **Layout** | Sizing, spacing, positioning rules |
| 8 | **Token Usage** | Which tokens apply to which parts |
| 9 | **Svelte Notes** | Implementation specifics |
| 10 | **GPUI Notes** | Implementation specifics |
| 11 | **Parity Checklist** | Verification list |
| 12 | **Known Deltas** | Intentional differences (if any) |

### Three-Tier Parity Model

**Tier 1: Strict Parity (Must Match)**
- Semantic behavior
- State transitions
- Event timing
- Accessibility (ARIA roles, keyboard behavior)
- Form integration

**Tier 2: Visual Parity (Should Match)**
- Colors (from tokens)
- Spacing (from tokens)
- Typography (from tokens)
- Overall proportions

**Tier 3: Implementation Freedom (Can Differ)**
- Internal state management
- Event handling details
- Rendering approach
- Animation mechanism
- CSS vs GPUI styling internals

### Props API Design

**Naming:**
- camelCase for multi-word props
- Boolean props: `isDisabled`, `isLoading` (not `disabled`, `loading`)
- Event handlers: `onClick`, `onFocus` (not `handleClick`)

**State Handling:**
- Support both controlled and uncontrolled patterns
- Controlled: `value` + `onChange`
- Uncontrolled: `defaultValue`

**Variants:**
- `variant` for visual style (primary, secondary, ghost)
- `size` for control size (sm, md, lg)
- Use enums with specific values

## Layer Model

Flint should explicitly separate three component layers.

### Layer 1 - Foundation

Portable, low-level building blocks:

- color, spacing, radius, typography, elevation, motion, density, icon, and
  focus tokens
- box/stack/inline/grid/surface/separator/scroll-shell layout primitives
- icon, button, text input, text area, search field, editable label
- checkbox, radio, switch, tri-state switch, segmented control
- slider, range slider, stepper, progress, loading, skeleton
- badge, pill, callout, status, tag, avatar-like identity chips
- tooltip, menu, context menu, popover, dialog, drawer, banner, toast
- tab strip, tab item, reorder affordances

### Layer 2 - Reusable Composites

Reusable but higher-order application components:

- field wrappers, validation states, form rows, form groups
- cards, page headers, detail items, detail sections, breadcrumbs
- list/grid shells, empty states, filter bars, pagination, stat grids
- data table, row actions, column filters, bulk actions
- media-preview widgets such as thumbnail, audio preview, video preview, embed
  shells
- selection pickers, relation pickers, command palette, search results
- shell-level notifications and inline remediation surfaces

### Layer 3 - Workstation Shell

Reusable desktop/pro-tool shells that are still general enough for multiple
GPUI apps:

- app header and project header patterns
- split-pane and dock-region layout
- panel surface, panel header, panel tabs, surface tabs
- panel drag, reorder, transfer, and collapse affordances
- inspector shell, browser shell, history shell, utility sidebars
- workspace window and multi-surface navigation

This layer is in scope for Flint because Loophole is not the only likely consumer
of workstation-style desktop UI.

## Extension Boundary

App-specific component systems should build above Flint.

Examples that should stay outside Flint core:

- Loophole transport bars
- arrangement timelines
- clip editors
- mixer strips tied to DAW semantics
- automation lanes
- plugin/device-chain editors

Flint should instead provide the tokens, shells, controls, tabs, surfaces, and
interaction rules those systems depend on.

## Underlay Integration Rule

Underlay should not become a thin public mirror of Flint. The intended shape is:

- Flint emits semantic tokens and reusable internal components
- Flint theme definitions should translate into both CSS/browser and Rust/GPUI
  forms from the same source model
- Underlay maps those tokens into Underlay-owned CSS/runtime systems
- Underlay wraps or reuses Flint components where it helps, while preserving
  Underlay-facing app APIs
- Underlay apps continue to think in Underlay terms

## Svelte Rule

Svelte implementations use **Bits UI** as an implementation substrate for
headless primitives (accessibility, state management, event handling).

**Bits UI Integration:**
- Bits provides 40+ headless primitives (Dialog, DropdownMenu, etc.)
- Flint components wrap Bits primitives with token-based styling
- Bits is an implementation detail, not a public dependency
- Flint owns the public component contract

**Requirements:**
- Flint docs define the public contract
- Flint tokens define the styling vocabulary
- Flint theme definitions remain canonical even when realized as CSS custom
  properties or browser runtime helpers
- Flint decides what parity means
- swapping Bits or bypassing Bits later must remain possible
- Svelte-first convenience must not redefine the canonical contract in a way
  that blocks future web implementations such as React

**Wrapper Pattern:**
```svelte
<!-- Flint wraps Bits -->
<script>
  import { Button as BitsButton } from "bits-ui";
  import { getTokenClasses } from "$lib/tokens";
  let { variant = "default", ...rest } = $props();
</script>

<BitsButton.Root class={getTokenClasses("button", { variant })} {...rest} />
```

## GPUI Rule

GPUI components should implement the same semantic contract, but they do not
need to mimic browser internals. Parity should be evaluated on:

- public inputs
- states
- interactions
- focus behavior
- layout expectations
- token application
- shared theme realization from the same semantic source
- documented deviations

**GPUI Patterns:**
- Components implement `IntoElement` or `Render` trait
- State via `Model<T>` with `observe`/`notify` or `subscribe`/`emit`
- Styling via Tailwind-like API: `div().flex().bg(cx.theme().background)`
- Theme access via `cx.theme()` which uses Flint-generated token structs

GPUI-first convenience must not redefine the canonical contract in a way that
blocks future desktop-oriented implementations.

## Extensible Implementation Rule

Flint should optimize for Svelte and GPUI now, but the contract and token model
must remain implementation-extensible.

That means:

- the canonical component contract should describe semantic inputs, states,
  events, layout rules, and token usage rather than framework internals
- Svelte and GPUI are the first required implementations, not the only
  permissible ones
- later additions such as React or another desktop UI kit should be able to
  attach as new implementation packages without changing token meaning
- any framework-specific behavior that cannot generalize must be documented as a
  runtime-specific realization, not promoted into the canonical contract

## Theme Translation Rule

Flint themes should be authored once at the semantic layer and translated into
runtime-specific artifacts rather than hand-maintained per framework.

That means:

- a Loophole theme should not need one separate canonical CSS definition and
  one separate canonical GPUI definition
- the same semantic theme should compile into browser/Svelte-friendly artifacts
  and GPUI/Rust-friendly artifacts
- format differences are allowed; semantic divergence is not
- downstream apps may override or extend themes, but they should do so from the
  same semantic theme model rather than by forking meaning per runtime

## Full Component Suite Map

The current planning surface for the potential Flint suite is:

### Tokens

- semantic color roles
- typography scale and roles
- spacing and sizing scales
- radius, border, shadow, and elevation roles
- motion and timing roles
- density and control-size variants
- icon sizing and stroke roles
- z-order and overlay roles
- focus, selected, disabled, error, warning, success, info states
- shared theme definitions translatable across CSS and Rust consumers
- implementation-neutral token and theme semantics that can support future
  targets beyond Svelte and GPUI

### Primitives

- layout primitives
- surfaces and separators
- button family
- text-entry family
- choice and selection controls
- sliders and value controls
- tabs
- overlays
- feedback and status primitives
- loading primitives

### Product And Application Composites

- forms and validation shells
- headers, cards, and detail displays
- filters and search shells
- tables, lists, grids, and pagination
- selection and relation pickers
- media preview and embed shells
- notification and remediation composites

### Workstation Composites

- panel system
- dock and split layout
- app/workspace shell
- command palette
- inspector/browser/history shell patterns
- multi-surface navigation

## Documentation Rule

Each component must have a contract document following the 12-section template:

1. purpose
2. anatomy
3. inputs/props
4. states
5. events
6. layout rules
7. token usage
8. accessibility/focus rules
9. Svelte implementation notes
10. GPUI implementation notes
11. parity checklist
12. known deltas

This minimum outline should be treated as mandatory and tightened in `g01.004`
rather than being left to component-by-component author preference.

## Implementation Gate

For milestone areas that depend on external systems or non-trivial tradeoffs:

- research should land in `docs/research/`
- synthesis should land in translation memos when a real decision is needed
- architecture and specs should absorb the decision before implementation starts

This keeps the main planning surface from drifting away from the active
research program.

## Research References

This architecture is informed by research documented in:

- [docs/research/source-hubs/hub-gpui.md](../research/source-hubs/hub-gpui.md) - GPUI capabilities
- [docs/research/source-hubs/hub-bits.md](../research/source-hubs/hub-bits.md) - Bits UI primitives
- [docs/research/value-tracks/tk-design-token-systems.md](../research/value-tracks/tk-design-token-systems.md) - Token taxonomy patterns
- [docs/research/value-tracks/tk-cross-framework-contracts.md](../research/value-tracks/tk-cross-framework-contracts.md) - Contract patterns
- [docs/research/translation-memos/tm-token-system.md](../research/translation-memos/tm-token-system.md) - Token system decisions
- [docs/research/translation-memos/tm-contract-template.md](../research/translation-memos/tm-contract-template.md) - Contract template decisions
- [docs/research/translation-memos/tm-svelte-substrate.md](../research/translation-memos/tm-svelte-substrate.md) - Svelte substrate decisions

## Next Task

Begin g01.002 token schema implementation using the DTCG format and three-layer
taxonomy defined in this architecture.
