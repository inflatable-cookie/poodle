# Poodle Core

`@inflatable-cookie/poodle-core` is Poodle's framework-free web package. It
contains shared component behavior, generated design tokens, component CSS,
and icon infrastructure used by the Svelte and React packages.

`0.2.3` is a preview-channel npm release. Breaking changes may still ship in
`0.x` minor releases; no `stable` channel exists yet, so pin an exact version
and read the [release notes](https://github.com/inflatable-cookie/poodle/blob/main/docs/release-notes/0.2.3.md) before
upgrading.

```sh
bun add @inflatable-cookie/poodle-core@0.2.3
# or: npm install @inflatable-cookie/poodle-core@0.2.3
```

`0.1.0` was the first registry version. The `0.2.0` workflow stopped before
publication and `0.2.1` replaced it. `0.2.2` corrected Poodle's public Rust
GPUI dependency identity. `0.2.3` is a lockstep patch; this package carries
no product API change in it.

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

## Custom-surface drag and drop

Same-document pointer, touch, and keyboard drag is a core controller plus
idiomatic Svelte/React bindings. It is not a portable component and does not
add a ledger row.

```ts
import { createDragDropController } from "@inflatable-cookie/poodle-core";

const controller = createDragDropController();
const disconnect = controller.connect(root);
controller.registerSource(sourceEl, {
  sourceId: "clip-1",
  subject: { kind: "clip", id: "clip-1" },
  allowedOperations: ["move"],
  label: "Intro clip",
});
controller.registerTarget(listEl, {
  targetId: "timeline",
  acceptedKinds: ["clip"],
  label: "Timeline",
  resolvePosition: () => "inside",
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: (intent) => ({ status: "committed" }),
});
controller.registerKeyboardTarget({
  targetId: "clip-2",
  acceptedKinds: ["clip"],
  label: "Verse clip",
  order: 1,
  resolvePosition: (input) => (input.direction === "previous" ? "before" : "after"),
  canDrop: (intent) => ({ accepted: true, intent }),
  onDrop: () => ({ status: "committed" }),
});
```

Framework packages export `DragDropProvider`. Svelte uses `useDragDrop()`
actions plus a logical keyboard-target helper; React uses `useDragSource` /
`useDropTarget` / `useKeyboardDropTarget`. See spec 069 and architecture 011.

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
