# g12.002 React Infra And Conversion Playbook

Status: complete (2026-07-13)
Owner: Poodle core
Depends on: `g12.001` (shared styles)

## What Shipped

- **Specimen harness** (`packages/react/preview/src/harness.tsx`):
  hash-routed gallery using the Svelte preview's `#components/<kebab-slug>`
  scheme, so the same Playwright probes drive both frameworks. Specimen
  modules self-register on import; no route table to maintain.
- Pilot specimens (button, checkbox, tabs) moved onto the harness.
  Verified: route filtering, shared-css styling, and the tabs machine
  (arrow navigation skipping a disabled item) all green via headless
  Playwright on port 4180.
- `@types/react` added to `@inflatable-cookie/poodle-react` so the components package
  typechecks standalone.

## Conversion Playbook (per component)

1. Read the contract (`docs/contracts/components/<component>.md`) — anatomy,
   props, states, ARIA. The Svelte component is the visual/behavior
   reference; the contract is the spec.
2. `import "@inflatable-cookie/poodle-styles/<kebab>.css";` — never write CSS. Class names,
   data-attributes, and recipe hooks come for free and must match the
   Svelte markup exactly (that's what the stylesheet addresses).
3. Props interface mirrors the Svelte `Props` shape. Svelte→React idiom
   map: `Snippet` → `ReactNode`; `$bindable` value+`onXChange` →
   controlled/uncontrolled pair (`value?` + `defaultValue?` + `onChange?`);
   `$derived` → plain expressions or `useMemo`; `$effect` → `useEffect`;
   `bind:this` → `useRef`.
4. Behavior comes from `@inflatable-cookie/poodle-headless` — the same machine transition the
   Svelte shell calls, with effects executed against `useState` setters.
   No behavior re-implementation; if a machine is missing something, that's
   a core bug, not a shell workaround.
5. Register a specimen mirroring the Svelte preview's states for that
   component; keep the slug identical.
6. Verify with the shared Playwright probe (styling present, machine
   interactions, recipe hook override) before moving on.

## Export Discipline

`@inflatable-cookie/poodle-react`'s `index.ts` re-exports each component + its props type.
Naming and prop shapes track the Svelte package; deviations are bugs
(interface-invariance rule, g11.001).
