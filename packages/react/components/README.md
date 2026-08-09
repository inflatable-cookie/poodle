# Poodle React Components

`@inflatable-cookie/poodle-react` — the full Poodle component library for React. All 132
components in a single package, as hand-written TSX shells over the shared
`@inflatable-cookie/poodle-core` state machines. Same contracts, tokens, recipes, and CSS as
`@inflatable-cookie/poodle-svelte`; the Svelte implementation stays the visual proof reference.

## Public Surface

Everything is exported from the package root — components and types:

```ts
import { Button, Dialog, DataTable } from "@inflatable-cookie/poodle-react";
import type { ControlSize, ControlDensity, SelectOption } from "@inflatable-cookie/poodle-react";
```

Peer dependencies: `react` and `react-dom` `>=18` (built and verified on 19).

## Setup

Three things wire a consuming app up.

**1. Load the token CSS** once, at the app entry — semantic tokens plus the
theme / density / control-size layers you use (published by
`@inflatable-cookie/poodle-core/tokens`, the shared token package):

```ts
import "@inflatable-cookie/poodle-core/tokens/styles.css";        // semantic token definitions
import "@inflatable-cookie/poodle-core/tokens/theme-eclipse.css"; // + theme-iceberg / theme-graphite
import "@inflatable-cookie/poodle-core/tokens/density-default.css";
import "@inflatable-cookie/poodle-core/tokens/control-size-md.css";
```

Component CSS ships with the components — each imports its own
`@inflatable-cookie/poodle-core/styles/<name>.css`, so your bundler pulls it in automatically when you
import the component. You never import component stylesheets by hand.

**2. Set the active theme / density / size** on a root element with
`applyThemeAttributes` from `@inflatable-cookie/poodle-core/tokens` (it stamps the
`data-theme` / `data-density` / `data-control-size` attributes the token CSS
keys off):

```ts
import { applyThemeAttributes } from "@inflatable-cookie/poodle-core/tokens";
applyThemeAttributes(document.documentElement, {
  theme: "eclipse",
  density: "default",
  controlSize: "md",
});
```

**3. Wrap the tree** in `UiPresentationProvider` (app-wide density + size
defaults). Add `IconProvider` for application-owned string icon names. Poodle's
scoped default Lucide set covers only component-owned chrome:

```tsx
import { createIconSet } from "@inflatable-cookie/poodle-core/icons";
import { UiPresentationProvider, IconProvider } from "@inflatable-cookie/poodle-react";
import { rocket, "trash-2" as trash2 } from "lucide-static/icon-nodes.json";

const icons = createIconSet({ rocket, "trash-2": trash2 });

<UiPresentationProvider density="default" sizeScale="md">
  <IconProvider icons={icons}>
    <App />
  </IconProvider>
</UiPresentationProvider>;
```

## Coming from `@inflatable-cookie/poodle-svelte`

The two packages are interface-invariant — same component names, same prop
names, same behavior (they run the same `@inflatable-cookie/poodle-core` machines). Only the
framework idioms differ:

| Svelte | React |
| --- | --- |
| `bind:value={x}` | controlled `value={x}` + `onValueChange`, or uncontrolled `defaultValue` |
| Event props (`onValueChange`, `onCheckedChange`, …) | identical names |
| `{#snippet leading()}…{/snippet}` slot | `leading={<Icon … />}` — a `ReactNode` prop |
| Parameterized snippet `{#snippet item(row)}` | render-prop `item={(row) => …}` |
| `class` / `on:click` | `className` / `onClick` |

Prop tables for every component are in the preview's usage docs (below) and in
`component-docs.ts`, authored canonically in the Svelte preview and re-exported
live — so React never drifts from canon.

## Preview & Reports

`packages/react/preview` is a browsable gallery mirroring the Svelte preview
one-to-one (same `#components/<slug>` routes for side-by-side diffing), with
theme / density / size / contrast controls, a Tokens inspector, and per-component
usage docs.

```sh
bun run --cwd packages/react/preview dev       # gallery on :4180
bun run --cwd packages/react/preview reports   # docs + parity + accessibility artifacts
```

## Stability

- Public entry point is the package root; import components and types from it.
- Release channel is **preview / experimental** — no published version yet;
  consumers link it by `file:` in the monorepo.
- `@inflatable-cookie/poodle-svelte` remains canon. React tracks it: shared contracts, shared
  `@inflatable-cookie/poodle-core` machines, shared `@inflatable-cookie/poodle-core/styles` CSS and tokens, and
  live-re-exported docs / parity data.
- Row rendering, async data policy, ranking, and workflow orchestration stay
  host-owned semantics, same as the Svelte package.
