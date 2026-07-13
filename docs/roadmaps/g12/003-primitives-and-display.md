# g12.003 React Batch: Primitives And Display

Status: complete (2026-07-13)
Owner: Poodle core
Depends on: `g12.002`

## What Shipped

19 components + shared infra, per the playbook:

- Infra: `presentation.tsx` (UiPresentationProvider context + size/density
  resolvers mirroring the Svelte tables), `icon-registry.ts` (alias table +
  lucide resolution — alias list must be edited alongside the Svelte copy),
  `internal.ts` (layout style tables), `types.ts` (shared prop types,
  Snippet→ReactNode).
- Display: Text, Eyebrow, Code (copy-to-clipboard, line numbers,
  highlights), Icon (lucide nodes), Pill (+PillContext), Avatar, Skeleton
  (all five presets), Spinner (ring/grid), Separator, Meter, Progress,
  StatusIndicator, MetaItem, TextLink.
- Layout: Box, Stack, Spacer, Grid, Region.

Deferred to the overlay batch (Tooltip dependency): TimeAgo,
ListCardCounter.

## Verification

Headless Playwright against the React preview: 17 styling probes green
(shared `@poodle/styles` addressing) + lucide icon nodes render. Preview
builds; `@poodle/react` typechecks.
