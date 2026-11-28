# 002 Token System and Package Layout

Status: active
Updated: 2026-11-25
Depends on: [001 Poodle System Shape](001-poodle-system-shape.md)

## Purpose

Poodle uses one token source to produce equivalent web and native artifacts.
This document defines where token meaning lives, what the build emits, and
which packages consumers use.

## Canonical Source

`packages/tokens/schema/` owns the W3C DTCG token source:

```text
schema/
  primitives/          raw color, dimension, typography, radius, motion, etc.
  semantic/            purpose-based component roles
  modes/
    themes/            named colour roles
    style/             named rendering styles
    density/           compact, default, and comfortable spacing
    control-size/      xs, sm, md, lg, and xl control dimensions
  style-treatments/    token overrides for each rendering treatment value
  metadata/            aliases, deprecations, and mode metadata
  manifest.json        schema and output metadata
```

Primitives express raw scales. Semantic tokens express intent such as canvas
background, primary text, control height, or focus ring. Components consume
semantic tokens so a theme can change values without changing component code.

Metadata supports migration and tooling; it is not a second source of token
meaning.

## Build Flow

The TypeScript build under `packages/tokens/scripts/` validates the source,
resolves aliases, and emits deterministic artifacts:

```text
W3C DTCG JSON
      |
      +-- CSS custom properties and mode stylesheets
      +-- TypeScript token objects, paths, themes, and metadata
      +-- Rust token modules and theme definitions
```

Generated CSS and TypeScript outputs live under `packages/tokens/artifacts/`
and are copied into their consumer packages where required; the Rust family is
emitted directly into the `poodle-tokens` crate so the crate is
self-contained. Do not edit generated artifacts by
hand. Run:

```sh
effigy tokens:build
```

## Modes

Poodle treats colour theme, rendering style, density, and control size as
independent inherited axes.

| Axis | Values |
| --- | --- |
| Theme | `iceberg`, `eclipse`, `graphite`, `midnight`, `nord`, `rose`, `forest`, `solarized`, `hornet`, `cobalt`, `clay`, `meadow` |
| Rendering style | `default` |
| Density | `compact`, `default`, `comfortable` |
| Control size | `xs`, `sm`, `md`, `lg`, `xl` |

On the web, set data attributes on any ancestor:

```html
<section data-theme="graphite" data-style="default" data-density="compact" data-control-size="md">
  <!-- Descendants inherit the selected modes. -->
</section>
```

Applications can scope modes to different subtrees. A theme supplies colour
roles. A style supplies shape, border, elevation, type, and motion values and
refers to colour through semantic roles. The token build rejects overlapping
theme and style writes and theme selectors in shared component CSS.

A style mode declares a `treatment` group alongside any direct semantic token
overrides. The build expands the choices from `schema/style-treatments/` before
applying direct overrides.

| Switch | Current value | Applies to |
| --- | --- | --- |
| `fill` | `filled` | Neutral control fills at rest, on hover and on press |
| `surface` | `tonal` | Container and floating surface separation |
| `selection` | `neutral` | Selected items and indicators |
| `typography` | `scale` | Size, leading, weight, tracking, case, and family and neutral text roles |

The base token stylesheet applies the default with or without
`data-style="default"`. To add a style, define its choices in
`schema/style-treatments/` and compose them in `schema/modes/style/`.
The shared stylesheets read the resulting tokens.

Selected items take a neutral fill and stronger text. Edges that read
`style.selection.itemBorder` resolve to transparent. The default sets
`style.selection.indicator` to `color.text.primary` for indicators such as
Rating's filled glyphs and AgentQuestion's progress dots.

The type scale covers PageHeader, SidebarNav, Dialog, DataTable, Button, and
TextInput. Buttons and inputs take `controlLabel` and `inputText` for their
resolved control size (`xs`–`xl`). SplitButton uses `compactControlLabel`
for its smaller labels, and BulkActionBar uses `controlSummary` for its
summary size variants. The scale also supplies `segmentLabel` for segmented
controls, `glyph` metrics for text glyphs, `avatarInitials`, `codeBlock`,
`eyebrow`, `pillLabel`, `statusLabel`, `valueReadout`, and unitless `body`
leading/bold roles for Foundations components, plus `prose` for document
editors and block sliders, `displayControl` for large numeric and code
inputs, `controlHint`, `chipLabel`, and `microLabel` for auxiliary labels
and chips, `emphasis` for bold weights, and `leading` for unitless
line-height ratios. Overlays, menus, navigation, and shells add
`disclosureTitle`, `disclosureDescription`, and `disclosurePanel`;
`appHeaderTitle` and `appHeaderSubtitle`; `breadcrumbLabel`; `menuItem`,
`menubarItem`, and `menuMeta`; `navigationLabel`; `toastTitle` and
`toastBody`; `compactBadge`; and the `glyph.sm` and `glyph.lg` sizes. Media
and agent surfaces add `mediaEyebrow`, `mediaTitle`, `mediaBody`,
`mediaTime`, and `thumbnailTitle`; the `glyph.media` size; `agentText`,
`messageText`, `composerText`, and `questionPrompt`; and `pickerTrigger`.
Density controls spacing.

The style supplies system sans and mono for the legacy per-role family tokens.
The generated CSS gives those aliases zero-specificity defaults at the
document root and each style scope. The six components read a shared override
(`--poodle-font-sans` or `--poodle-font-mono`) ahead of their per-role token.
A low-priority layer lets legacy-only CSS readers pick up a shared override in
a subtree while preserving explicit per-role declarations on the same element.
The shared inputs are unset until an app supplies them.

Themes define `color.fill.{rest,hover,active}` for filled controls and
`color.select.{accent,neutral}` for selected items. Neutral fills use the
theme's primary-text ink at a fixed alpha over the current layer. Light
themes use 0.045, 0.07, 0.10 and 0.10 for those four neutral fills. Dark
themes use 0.055, 0.085, 0.12 and 0.13. Themes also supply accent roles for
semantic emphasis. Solid Tabs keep their accent and inverse text tokens.

Tonal goes from canvas to `color.background.panel` for a container and from
panel to `color.background.elevated` for one that floats. Both use the
existing colour steps. Containers read the switch as
`--poodle-style-surface-fill` (the inline step) and
`--poodle-style-surface-floatingFill` (the elevated step), with
`--poodle-style-surface-border` and `--poodle-style-surface-shadow` for the
other two edges; each component reads its recipe hook first, then the role.
Card, Dialog, Popover, Tooltip, Drawer, HoverCard, CommandPalette,
ToastStack, PickerShell's popover and modal variants, Surface, Callout and
EmptyState all resolve through that chain, so a style reaches them without
per-component CSS. Popover, Tooltip, Drawer, HoverCard, CommandPalette, the
toast body, Surface's elevated tone and PickerShell's floating variants fall
back from `-floatingFill` to `-fill`; Surface, Callout, EmptyState and the
inline PickerShell read `-fill`. Run `effigy tokens:contrast` to check the
new fills
with their paired text on the content layers. The report shows the existing
steps below 0.04 OKLCH lightness so themes can address small steps in a
later palette revision.

Nested style modes start from the base values for style-owned tokens, and
nested themes resolve style aliases against their own colour roles. Style
data references the roles. The themes own the colour values.

Native theme providers support theme, density, and control size. Generated
Rust style definitions carry treatment enums and token references for future
native support.

The inherited `--poodle-contrast` custom property adjusts neutral background
and border separation without redefining a theme. The default is `0.5`; `1`
uses the theme's authored neutral ramp. Native providers apply the equivalent
calculation.

## Web Packages

### `@inflatable-cookie/poodle-core`

The core package publishes the framework-free web surface:

- `/tokens` — generated token values, paths, themes, styles, and metadata
- `/tokens/styles.css` — base tokens
- `/tokens/css/poodle-styles.css` — all rendering style modes
- `/tokens/runtime` — `applyThemeAttributes()` and mode types
- `/styles/*` — shared component styles
- `/icons` — icon types and Poodle's scoped default Lucide set
- the `poodle-icons` command — application-owned icon module generation
- the root export — component state machines and prop helpers

Applications import base tokens plus the theme, density, and control-size
stylesheets they use:

```ts
import "@inflatable-cookie/poodle-core/tokens/styles.css";
import "@inflatable-cookie/poodle-core/tokens/themes.css";
import "@inflatable-cookie/poodle-core/tokens/density-default.css";
import "@inflatable-cookie/poodle-core/tokens/control-size-md.css";
```

Individual mode stylesheets remain available. `/tokens/style-default.css`
scopes the default to `data-style="default"`, and
`/tokens/css/poodle-styles.css` includes all explicit style scopes.

### Framework packages

`@inflatable-cookie/poodle-svelte` and
`@inflatable-cookie/poodle-react` contain runtime shells. They depend on core
behavior and use core styles; neither owns a parallel token system.

The packages are currently private pre-1.0 previews. Until registry publication,
consumer repositories use workspace or file dependencies.

## Icons

Lucide is Poodle's default icon adapter. Core includes only the icons required
by Poodle components and previews, not Lucide's full catalogue. One pinned
manifest generates both core `IconNodes` and the SVG assets consumed by the
Rust renderers. `effigy audit:icons` rejects name or geometry drift between
those outputs. Applications declare any additional names in their own JSON
file and generate a scoped module:

```json
["calendar", "folder-open", "wand-sparkles"]
```

```sh
bun x poodle-icons --names icons.json --out src/icons.generated.ts
```

The application merges that generated set with Poodle's defaults in its icon
provider. This keeps icon choice under application control and avoids shipping
an unused catalogue to every consumer.

## Rust Packages

The token build feeds `poodle-tokens`, which exposes typed theme, rendering style,
density, control-size, and semantic token data. Native code consumes tokens at
compile time.

The relevant native package flow is:

```text
poodle-tokens + poodle-specs
             |
       poodle-render
             |
        poodle-node
             |
        GPUI backend
```

`poodle-gpui` provides the runtime theme provider that resolves the semantic
token roles. `poodle-render` tests resolve these roles from `poodle-tokens`
through a local fixture.

## Package Ownership

```text
packages/tokens/            canonical schema; generated CSS and TS artifacts
packages/core/              public framework-free web surface
packages/svelte/components/ Svelte component package
packages/react/components/  React component package
packages/contracts/tokens/  generated Rust token consumer crate
packages/render/            shared Rust component renderer
packages/gpui/              GPUI adapter and node backend
```

Downstream consumers may translate Poodle tokens into host-owned names, in
their own repositories. Those aliases do not become token authorities, and
application code should not need to know which design system produced the host
values.

## Change Rules

- Change token meaning in the DTCG schema, then regenerate all targets.
- Use semantic tokens in components; reserve primitives for token definitions.
- Keep web component CSS in `poodle-core` so Svelte and React do not drift.
- React shells that render a `poodle-*` class must import the core stylesheet
  that defines it; `effigy drift:react-stylesheets` is the gate.
- Keep runtime-specific conversion in adapters and backends.
- Record aliases and deprecations when renaming public token paths.
- Validate generated outputs before committing a schema change.
  `effigy audit:tokens` (`--check`) compares every committed file under the
  token artifact roots, including files the generator no longer emits.
  Write mode unlinks those orphans under both roots; the core/Svelte mirror
  copies only files the generator still emits, so an artifact-root orphan
  cannot be re-homed into the mirror.
