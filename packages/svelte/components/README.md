# Poodle Svelte Components

Unified Svelte component library for Poodle. All components in a single
`@inflatable-cookie/poodle-svelte` package.

`0.2.3` is a preview-channel npm release. Breaking changes may still ship in
`0.x` minor releases; no `stable` channel exists yet, so pin an exact version
and read the [release notes](https://github.com/inflatable-cookie/poodle/blob/main/docs/release-notes/0.2.3.md) before
upgrading. `0.1.0` was the first registry version. The `0.2.0` workflow stopped
before publication and `0.2.1` replaced it. `0.2.2` corrected Poodle's public
Rust GPUI dependency identity. `0.2.3` adds triggerless `ContextMenu`
composition.

## Setup

Install the package and its `@inflatable-cookie/poodle-core` peer, which
carries tokens, styles, behavior, and icons:

```sh
bun add @inflatable-cookie/poodle-svelte@0.2.3 @inflatable-cookie/poodle-core@0.2.3
# or: npm install @inflatable-cookie/poodle-svelte@0.2.3 @inflatable-cookie/poodle-core@0.2.3
```

```json
{
  "dependencies": {
    "@inflatable-cookie/poodle-core": "0.2.3",
    "@inflatable-cookie/poodle-svelte": "0.2.3"
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

Custom-surface drag and drop is a substrate, not a catalogue component:

```ts
import { DragDropProvider, useDragDrop } from "@inflatable-cookie/poodle-svelte";
```

`useDragDrop` returns a snapshot store plus `dragSource` / `dropTarget`
actions and a `keyboardDropTarget` helper for element-free ordered keyboard
targets. The provider owns one overlay and one polite live region.

`DragDropProvider` also accepts `crossWindowTargetBridge` and
`inboundFileBridge` for this document. External files reach ordinary drop
targets as receipts; a drag-out source declares `fileExportBridge` and shows
its own state through the snapshot. Poodle never receives a path or a `File` —
see the core package README.

## Stability Notes

- public entry points are the package root and `./types`
- row rendering, async data policy, ranking, and workflow orchestration remain
  host-owned semantics even when the components own shell posture
- virtualization strategy, embed runtimes, and richer asset playback stay out
  of the public package contract for now
- GPUI and Jetstream parity for this family is documented in contracts

See the [Svelte developer guide](https://github.com/inflatable-cookie/poodle/blob/main/docs/guides/svelte-developer-guide.md)
for themes, icon generation, component conventions, and application recipes.
