# Poodle Svelte Components

Unified Svelte component library for Poodle. All components in a single
`@inflatable-cookie/poodle-svelte` package.

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

## Next Task

Use this package surface while following the direct-consumer and shell-oriented
onboarding lanes, keeping workflow orchestration host-owned and treating these
components as reusable shells rather than app templates.
