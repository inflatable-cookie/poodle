# Poodle Core

`@inflatable-cookie/poodle-core` is Poodle's framework-free web package. It
contains shared component behavior, generated design tokens, component CSS,
and icon infrastructure used by the Svelte and React packages.

`0.2.1` is a preview-channel npm release. Breaking changes may still ship in
`0.x` minor releases; no `stable` channel exists yet, so pin an exact version
and read the [release notes](https://github.com/inflatable-cookie/poodle/blob/main/docs/release-notes/0.2.1.md) before
upgrading.

```sh
bun add @inflatable-cookie/poodle-core@0.2.1
# or: npm install @inflatable-cookie/poodle-core@0.2.1
```

`0.1.0` was the first registry version. The `0.2.0` workflow stopped before
publication; `0.2.1` is its replacement on the preview channel.

## Public Surfaces

| Import | Purpose |
| --- | --- |
| package root | State machines, prop getters, and shared interaction helpers |
| `/styles/*` | Shared component CSS |
| `/tokens` | Token values, paths, theme metadata, and helpers |
| `/tokens/runtime` | `applyThemeAttributes()` and mode types |
| `/tokens/styles.css` | Base token custom properties |
| `/tokens/themes.css` | All generated theme selectors |
| `/tokens/theme-<name>.css` | One generated theme |
| `/tokens/density-<mode>.css` | One density mode |
| `/tokens/control-size-<size>.css` | One control-size mode |
| `/icons` | Icon types and Poodle's scoped default Lucide set |

## Web Setup

Import base tokens and the modes your application can select:

```ts
import "@inflatable-cookie/poodle-core/tokens/styles.css";
import "@inflatable-cookie/poodle-core/tokens/themes.css";
import "@inflatable-cookie/poodle-core/tokens/density-default.css";
import "@inflatable-cookie/poodle-core/tokens/control-size-md.css";
```

```ts
import { applyThemeAttributes } from "@inflatable-cookie/poodle-core/tokens/runtime";

applyThemeAttributes(document.documentElement, {
  theme: "eclipse",
  density: "default",
  controlSize: "md",
});
```

Framework components import their own shared component styles. Application
code normally does not import `/styles/*` directly.

## Icons

Poodle includes only the Lucide icons required by its components. Generate an
application-owned set for additional names:

```sh
bun x poodle-icons --names icons.json --out src/icons.generated.ts
```

The generated module contains only the requested icon nodes. Merge it into the
Svelte or React `IconProvider`; do not bundle Lucide's full catalogue at
runtime.

## Development

Token artifacts originate in `packages/tokens/schema/`. Default icon names
originate in `src/icons/default-icons.json`; `effigy icons:build` generates the
web modules and shared Rust SVG assets from pinned Lucide data. Regenerate
tokens with `effigy tokens:build` and validate documentation with
`effigy docs:check`.

See the [token architecture](https://github.com/inflatable-cookie/poodle/blob/main/docs/architecture/002-token-system-and-package-layout.md)
and [repository README](https://github.com/inflatable-cookie/poodle/blob/main/README.md) for the full system.
