# Poodle Svelte Components

Unified Svelte component library for Poodle. All components in a single
`@inflatable-cookie/poodle-svelte` package.

This package is a pre-1.0 source preview and is not published to a registry.
Use a workspace or file dependency and pair it with
`@inflatable-cookie/poodle-core` for tokens, styles, behavior, and icons.

## Setup

```json
{
  "dependencies": {
    "@inflatable-cookie/poodle-core": "file:../poodle/packages/core",
    "@inflatable-cookie/poodle-svelte": "file:../poodle/packages/svelte/components"
  }
}
```

Import the token stylesheet once at the application entry:

```ts
import "@inflatable-cookie/poodle-core/tokens/styles.css";
import "@inflatable-cookie/poodle-core/tokens/themes.css";
import "@inflatable-cookie/poodle-core/tokens/density-default.css";
import "@inflatable-cookie/poodle-core/tokens/control-size-md.css";
```

Set `data-theme`, `data-density`, and `data-control-size` on an ancestor, then
import components from the package root.

## Public Surface

All components are exported from the package root:

```ts
import { Button, Dialog, DataTable } from "@inflatable-cookie/poodle-svelte";
```

Types are available from the root or the `/types` subpath:

```ts
import type { ControlSize, ControlDensity, SelectOption } from "@inflatable-cookie/poodle-svelte";
```

## Stability Notes

- public entry points are the package root and `./types`
- row rendering, async data policy, ranking, and workflow orchestration remain
  host-owned semantics even when the components own shell posture
- virtualization strategy, embed runtimes, and richer asset playback stay out
  of the public package contract for now
- GPUI and Jetstream parity for this family is documented in contracts

See the [Svelte developer guide](../../../docs/guides/svelte-developer-guide.md)
for themes, icon generation, component conventions, and application recipes.
