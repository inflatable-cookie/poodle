---
title: g13 batch 044 — two preview-shell defects b035 measured
status: complete
milestone: side-quest (preview shell, outside the g13 IR lane)
owner: Poodle core
updated: 2026-08-13
tags: [log, g13, preview, shell, svelte, react, papercut, SHELL-08]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/044-preview-shell-defects.md` on branch
`thread/g13-044-preview-shell-defects`: fixed the two preview-shell defects
b035 measured live and recorded in `PAPERCUTS.md` (2026-08-12). Both were
reproduced in a browser before any change, fixed at the diagnosed lines, and
re-verified in a browser after. No other file touched; both `PAPERCUTS.md`
entries marked resolved in place. No sub-agents; sources read directly.

## Measured before-state

| Defect | Shell | Before | Reference (React) |
|---|---|---|---|
| R1 URL persistence | Svelte | clicking density `default` / size `lg` updates pills to `eclipse / default / lg` but `location.href` stays `http://localhost:4174/` (no query string, no console error) | `?theme=eclipse&density=default&controlSize=lg#components` after the same clicks |
| R2 grid vs header | Svelte / React | query `date` → header "6 components", grid renders 165 (Svelte) / 163 (React) cards | — (same defect in both) |

## Fixes (only the card's writable paths)

- `packages/svelte/preview/src/App.svelte` — R1: `let hasMounted = false`
  → `let hasMounted = $state(false)`. Chosen over dropping the guard: the
  guard orders the write after `onMount`'s `syncCurrentLocation()`, so an
  incoming query param (e.g. `?theme=forest`) is read before the effect
  writes; without it the effect would run with defaults first and clobber
  the incoming URL. `$state` mirrors React's `setMounted(true)` inside the
  mount effect exactly. No loop: the effect writes only to
  `window.history.replaceState`, never to reactive state.
- `packages/svelte/preview/src/pages/CatalogueLanding.svelte` and
  `packages/react/preview/src/gallery/CatalogueLanding.tsx` — R2: grid
  groups now derive from the `components` prop. Both build a slug lookup
  from the prop and filter each `componentsByTag()` group by it, dropping
  groups left empty (same rule `componentsByTag()` itself applies and the
  sidebar uses). Count line and grid agree because the prop is the same
  filtered set.

## Validation (all measured in a browser; both previews)

- R1 Svelte: fresh load of `/?theme=forest` settles at
  `?theme=forest&density=compact&controlSize=sm#components` (param
  preserved, write-on-mount matches React); clicking density `default` +
  size `lg` gives `?theme=forest&density=default&controlSize=lg#components`
  — same shape as the React reference.
- R2 both: query `date` → header "6 components" and exactly 6 grid cards,
  one section (`Inputs`), zero cards outside the grid.
- No console errors (pageerror + console.error captured on reload) in
  either preview.

Step-6 gates: `effigy check:svelte`, `effigy test:components`,
`effigy docs:lint`, `effigy ci:web`, `git diff --check` — all exit 0
(see card).

## Acceptance criteria

- [x] Svelte's URL persistence matches React's, shown in a browser.
- [x] Grid count equals header count with a query active, both previews.
- [x] Both papercut entries marked resolved.
- [x] All step-6 commands exit 0; no baseline refreshed.

## Not done

Per batch card: no merge (branch pushed only), nothing outside the writable
paths, no generated artifact touched, no component/specimen/core file, no
`PAPERCUTS.md` neighbour reflowed.
