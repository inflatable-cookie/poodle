# 002 Token System and Package Layout

Status: active
Updated: 2026-08-09
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
    themes/            named theme values
    density/           compact, default, and comfortable spacing
    control-size/      xs, sm, md, lg, and xl control dimensions
  metadata/            aliases, deprecations, and theme metadata
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

Generated outputs live under `packages/tokens/artifacts/` and are copied into
their consumer packages where required. Do not edit generated artifacts by
hand. Run:

```sh
effigy tokens:build
```

## Modes

Poodle treats theme, density, and control size as independent inherited axes.

| Axis | Values |
| --- | --- |
| Theme | `iceberg`, `eclipse`, `graphite`, `midnight`, `nord`, `rose`, `forest`, `solarized`, `hornet`, `cobalt`, `clay`, `meadow` |
| Density | `compact`, `default`, `comfortable` |
| Control size | `xs`, `sm`, `md`, `lg`, `xl` |

On the web, set data attributes on any ancestor:

```html
<section data-theme="graphite" data-density="compact" data-control-size="md">
  <!-- Descendants inherit the selected modes. -->
</section>
```

Applications can scope different modes to different subtrees. Native theme
providers expose the same axes through typed builders.

The inherited `--poodle-contrast` custom property adjusts neutral background
and border separation without redefining a theme. The default is `0.5`; `1`
uses the theme's authored neutral ramp. Native providers apply the equivalent
calculation.

## Web Packages

### `@inflatable-cookie/poodle-core`

The core package publishes the framework-free web surface:

- `/tokens` — generated token values, paths, themes, and metadata
- `/tokens/styles.css` — base tokens and theme selectors
- `/tokens/runtime` — `applyThemeAttributes()` and mode types
- `/styles/*` — shared component styles
- `/icons` — icon types and Poodle's scoped default Lucide set
- the `poodle-icons` command — application-owned icon module generation
- the root export — component state machines and prop helpers

Applications import base tokens plus the mode styles they can select:

```ts
import "@inflatable-cookie/poodle-core/tokens/styles.css";
import "@inflatable-cookie/poodle-core/tokens/themes.css";
import "@inflatable-cookie/poodle-core/tokens/density-default.css";
import "@inflatable-cookie/poodle-core/tokens/control-size-md.css";
```

Individual theme, density, and control-size stylesheets remain available as
explicit subpath exports for consumers that need a narrower CSS payload.

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

The token build feeds `poodle-tokens`, which exposes typed theme, density,
control-size, and semantic token data. Native code consumes these values at
compile time.

The relevant native package flow is:

```text
poodle-tokens + poodle-specs
             |
       poodle-render
             |
        poodle-node
        /          \
GPUI backend    Jetstream backend
```

`poodle-gpui` and `poodle-jetstream` provide runtime-specific theme providers
and style mapping. They resolve the same semantic token roles and must not
introduce backend-only theme values.

## Package Ownership

```text
packages/tokens/            canonical schema and generated artifacts
packages/core/              public framework-free web surface
packages/svelte/components/ Svelte component package
packages/react/components/  React component package
packages/contracts/tokens/  generated Rust token consumer
packages/render/            shared Rust component renderer
packages/gpui/              GPUI adapter and node backend
packages/jetstream/         Jetstream adapter
packages/bridges/underlay/  internal host token bridge
```

Downstream bridges may translate Poodle tokens into host-owned names. They do
not become token authorities, and application code should not need to know
which design system produced the host values.

## Change Rules

- Change token meaning in the DTCG schema, then regenerate all targets.
- Use semantic tokens in components; reserve primitives for token definitions.
- Keep web component CSS in `poodle-core` so Svelte and React do not drift.
- Keep runtime-specific conversion in adapters and backends.
- Record aliases and deprecations when renaming public token paths.
- Validate generated outputs before committing a schema change.
