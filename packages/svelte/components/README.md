# Poodle Svelte Components

Unified Svelte component library for Poodle. All components in a single
`@inflatable-cookie/poodle-svelte` package.

`0.2.0` is a preview-channel npm release. Breaking changes may still ship in
`0.x` minor releases; no `stable` channel exists yet, so pin an exact version
and read the [release notes](https://github.com/inflatable-cookie/poodle/blob/main/docs/release-notes/0.2.0.md) before
upgrading. `0.1.0` was the first registry version; `0.2.0` supersedes it on
the preview channel.

## Setup

Install the package and its `@inflatable-cookie/poodle-core` peer, which
carries tokens, styles, behavior, and icons:

```sh
bun add @inflatable-cookie/poodle-svelte@0.2.0 @inflatable-cookie/poodle-core@0.2.0
# or: npm install @inflatable-cookie/poodle-svelte@0.2.0 @inflatable-cookie/poodle-core@0.2.0
```

```json
{
  "dependencies": {
    "@inflatable-cookie/poodle-core": "0.2.0",
    "@inflatable-cookie/poodle-svelte": "0.2.0"
  }
}
```

Svelte 5 (`>=5.38.6 <6`) is a peer dependency.

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

See the [Svelte developer guide](https://github.com/inflatable-cookie/poodle/blob/main/docs/guides/svelte-developer-guide.md)
for themes, icon generation, component conventions, and application recipes.
