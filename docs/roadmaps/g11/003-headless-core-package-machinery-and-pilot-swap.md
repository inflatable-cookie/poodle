# g11.003 Headless Core Package, Shared Machinery, And Pilot Swap

Status: complete (2026-07-10)
Owner: Poodle core
Depends on: `g11.002`
Updated: 2026-07-10

## Purpose

Create the framework-free core package, build the shared machinery, implement
the three pilot machines, and swap the pilot Svelte components
(`Tabs`, `Popover`, `Checkbox`) onto them **without changing their public
surfaces**. This is the proof milestone for the whole program: core extraction
with zero consumer churn.

## Scope

1. Package scaffold: `packages/core` → `@inflatable-cookie/poodle-headless` (confirm name here;
   record in spec `062`). Plain TS, no framework deps. Unit-testable with
   `bun test` or `vitest` — machines are pure, so tests are cheap; write them.
2. Shared machinery, extracted/ported from
   `packages/svelte/components/src/internal` and friends rather than written
   fresh where possible:
   - focus: trap, roving tabindex, restore-on-close
   - dismissable-layer stack (nested overlays, escape/outside-click routing)
   - anchor positioning wrapping Floating UI (`@floating-ui/dom`) — new
     dependency, decide and record
   - presence (mount/unmount animation states)
   - typeahead, id/aria wiring helpers
3. Pilot machines per the `g11.002` specs: tabs, popover, checkbox, each with
   prop getters emitting ARIA + `data-scope`/`data-part`/`data-state`.
4. Svelte adapter: minimal glue (`useMachine`-equivalent for Svelte 5 runes)
   inside `@inflatable-cookie/poodle-svelte`; core added as a dependency of the components
   package.
5. Swap `Tabs.svelte`, `Popover.svelte` (or nearest existing overlay if no
   standalone Popover), and `Checkbox.svelte` internals onto core. Public
   props, callbacks, snippets, and rendered semantics unchanged.

## Consumer Compatibility (hard gate)

Interface-invariant per spec `062`. Proof:

- typecheck matrix across the g11.001 validated consumer roots (`underlay`,
  `acme-admin`, `compli-me/admin`, `cp-admin`, `composer-admin`,
  `greenhouse`), plus grep-driven spot checks in any root using the pilot
  components
- Svelte preview/demo app exercises all contract states for the three pilots
- visual parity: pilots render identically before/after (screenshot or
  side-by-side preview check)

If invariance proves impossible for a pilot, stop, record why here, and route
the interface change through the g11.001 wave process instead of widening
this milestone silently.

## Exit Criteria

- core package exists with tested machinery + three tested machines
- three pilot components run on core with unchanged public surfaces
- consumer validation matrix passed and recorded here
- name/dependency decisions recorded in spec `062`

## Validation

- core unit tests
- `effigy svelte:surface-audit`, targeted `effigy svelte:build`
- consumer typecheck matrix above
- `effigy docs:lint`

## Completion Notes (2026-07-10)

Shipped:

- `packages/core` → `@inflatable-cookie/poodle-headless`: pure transition functions + part
  attribute getters, no interpreter (decision recorded in spec `062`).
  Machinery: `nav` (roving index), `dom/focus`, `dom/dismiss`
  (dismissable-layer stack with pure `resolveDismiss`), `dom/id`.
- Machines: checkbox, popover, tabs (+ tooltip sub-machine). 47 `bun:test`
  tests, typecheck clean.
- Swapped `Checkbox.svelte`, `Popover.svelte`, `Tabs.svelte` onto core.
  Checkbox/Popover use part-attr spreads (adds `data-scope`/`data-part`/
  `data-state`, additive); Tabs keeps explicit markup and existing id format,
  machine drives selection/focus/keyboard/reorder. `internal.ts` focus/nav
  helpers now re-export from core.
- Kept adapter-side by design: Tabs URL-history sync, overflow measurement,
  drag DOM plumbing (final reorder routes through the machine), tooltip
  timers; Popover CSS anchoring (Floating UI deferred to `g11.004` wave 2).
- Known-improvement shipped: nested overlays now dismiss innermost-first via
  the shared layer stack (single-overlay behavior unchanged).

Validation:

- runtime-verified in the preview browser: checkbox toggle/readOnly-revert/
  mixed-indeterminate; popover open/close/escape/outside-dismiss with focus
  restore on every path; tabs click select, arrow roving with disabled skip
  and wrap, Home/End, Alt+Arrow reorder with focus following
- regression caught and fixed during verification: keyboard reorder/focus
  used machine `focusIndex` where old code used the keydown's tab index;
  machine events now carry `fromIndex` (contract + tests updated)
- consumer typecheck matrix: `underlay` (effigy check:types), `acme-admin`,
  `compli-me/admin`, `cp-admin`, `composer-admin` all clean;
  `songsprout/greenhouse` fails only on 2 pre-existing errors in
  `songsprout/stem` media types (no Poodle imports involved — external
  blocker, g11.001 class)
- docs lint: only the pre-existing tree.md / Radio findings

Gotcha recorded in spec `062`: bun snapshots `file:` deps — the internal
core dep must stay `workspace:*` or dev picks up stale copies.

## Next Task

`g11.004` — sweep the remaining behavioral components onto core in waves.
