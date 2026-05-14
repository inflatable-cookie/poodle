# g11.001 Svelte Modernization And Consumer Rollout

Status: active
Owner: Poodle core
Depends on: g10 closeout
Updated: 2026-05-14

## Purpose

Systematically modernize Poodle's Svelte component layer and migrate all known
consumers one component wave at a time.

This is not a syntax churn project. The point is to remove or contain legacy
public seams that hamstring consumers:

- compatibility alias props that extend old shapes instead of converging on one
  canonical input shape
- `createEventDispatcher`-centric public APIs where callback props are the
  better long-term surface
- legacy slot-only composition where snippets or cleaner controlled seams are
  the better modern contract
- compatibility-mode internals that make complex components harder to maintain
  than the equivalent Svelte 5 runes-based implementation

## Why A New Generation

This is a new sequencing era, not an incidental follow-up to GPUI hardening.
The work spans:

- nearly the entire `@poodle/svelte` public surface
- `underlay` wrapper and editor ownership boundaries
- six Underlay-root consumer app families
- direct desktop consumers outside the Underlay rollout path

That is broad enough to justify a new generation rather than smearing the work
across leftover `g10` files.

## Modernization Principles

1. Do not rewrite the whole package at once.
2. Work component-by-component in bounded waves.
3. Each wave must update Poodle first, then every known consumer of that
   component.
4. Prefer full modernization to compatibility-preserving half measures.
5. Prefer removing legacy surface area over adding more compatibility layers.
6. Temporary migration shims are the exception, not the default. Use them only
   when a specific rollout would otherwise stall, and remove them in the same
   wave or the immediately following one.
7. Validate the full consumer wave before moving to the next component set.

## Modernization Posture

This generation is not trying to minimize disruption inside Poodle. It is
using coordinated consumer rollouts to make deeper modernization safe.

Default stance:

- choose the most up-to-date, ideal public and internal implementation that
  Poodle actually wants to live with
- do not preserve outdated props, event shapes, or composition seams just
  because they already exist
- do not stop at syntax cleanup if the public API is still shaped by legacy
  compatibility decisions
- use the consumer migration work to absorb the disruption deliberately instead
  of leaving the old surface in place forever

## Consumer Matrix

### Primary Integration Surface

- `underlay`
  - owns wrapper normalization and the highest-value adapter seams
  - must be updated first for any component that Underlay re-exports or
    structurally depends on

### Underlay-Root Consumer Families

- `underlay-reference`
  - likely child packages: `acme-admin`, `acme-front`, `acme-ui`
- `contact-patch`
  - likely child packages: `cp-admin`, `cp-front`
- `compli-me`
  - likely child packages: `admin`, `front`
- `acowtancy`
  - known child packages with Poodle deps: `dairy`, `cream`, `froyo`
- `songsprout`
  - known child packages with Poodle deps: `greenhouse`, `bloom`
- `loophole/composer`
  - known child packages: `composer-admin`, `composer-front`

Treat each root as a rollout boundary. If a component wave affects one child
package in that root, inspect the rest of the root for the same component
before calling the wave complete.

### Direct Desktop Consumers

- `finch/app-electron`
- `soundcheck`
- `loophole/aura`

These do not sit behind Underlay wrappers in the same way and must be checked
explicitly on every affected wave.

## Execution Model

Each wave follows this sequence:

1. Inventory
   - identify the exact Poodle components in scope
   - map every known usage across `underlay`, the six Underlay-root consumer
     families, and direct desktop apps
2. Contract decision
   - define the canonical modern public shape
   - start from the ideal current-state API and implementation, not the lowest
     disruption path
   - decide what compatibility shims are temporary, what is removed now, and
     what must stay until a later wave
3. Poodle implementation
   - modernize the component internals and public surface
   - update docs, specimens, and component usage guidance
4. Underlay migration
   - update Underlay wrappers, editor surfaces, or direct imports first
   - keep Underlay-owned seams Underlay-owned; do not leak Poodle internals
5. Consumer migration
   - update all affected child packages in each root
   - include direct desktop consumers after the Underlay-root sweep
6. Validation
   - run narrow Poodle checks
   - run Underlay checks
   - run target app checks only where the wave actually touched usage
7. Closeout
   - record completed consumers and remaining exceptions in the roadmap file
   - only then open the next component wave

## Wave Order

### Wave 0 — Program Infrastructure

- consumer inventory report per component
- migration note template
- repeatable scan commands for Poodle and downstream repos
- rule update so new work does not add fresh legacy surface

Status: started by audit groundwork, not complete until the first component wave
has a tracked consumer matrix.

### Wave 1 — Input And Choice Primitives

Priority components:

- `TextInput`
- `Select`
- `Checkbox`
- `Switch`
- `RadioGroup`
- `ToggleGroup`
- `SegmentedControl`

Reason:

- these have the highest chance of callback-prop, event, validation, and
  controlled/uncontrolled friction
- they are used widely across Underlay-root admin apps and direct forms

### Wave 2 — Overlay And Menu Primitives

Priority components:

- `Dialog`
- `Drawer`
- `Popover`
- `Tooltip`
- `Menu`
- `Menubar`
- `HoverCard`

Reason:

- these carry the highest legacy slot and dispatcher density
- they often define composition patterns that downstream apps copy elsewhere

### Wave 3 — Buttons, Actions, And Navigation Chrome

Priority components:

- `Button`
- `IconButton`
- `Tabs`
- `NavigationMenu`
- `Pagination`
- `OrderBy`

Reason:

- high usage density
- old event and slot posture easily bleeds into consumer ergonomics

### Wave 4 — Editor And Workflow Composites

Priority components:

- `BlockEditor`
- `MarkdownEditor`
- `RelationPicker`
- `MediaBrowsePanel`
- `FormDialog`

Reason:

- these are the most complex Svelte surfaces and the most likely to punish
  consumers if modernized inconsistently
- Underlay and content-heavy apps depend on them structurally

### Wave 5 — Long Tail Sweep

- remaining primitives and composites with low consumer count
- remove obsolete compatibility aliases that survived earlier waves only for
  rollout safety

## Consumer Update Rules

- Underlay first for any component it wraps, shapes, or normalizes
- do not update one app in a root and leave sibling packages stale without an
  explicit note
- when a root still uses `@poodle/svelte-primitives` /
  `@poodle/svelte-composites`, treat import-shape cleanup as part of the same
  modernization conversation if the affected component is touched there
- if a direct desktop app uses a component without Underlay in front of it,
  update that app in the same wave rather than deferring indefinitely

## Validation Baseline

### Poodle

- `git diff --check`
- `effigy svelte:surface-audit`
- targeted `effigy svelte:build` when preview or docs changed

### Underlay

- `effigy tasks`
- `effigy validate`
- targeted `bun x svelte-check --tsconfig ./ts/tsconfig.json` only when needed
  outside an Effigy-covered path

### Consumer Roots

Prefer repo-owned Effigy tasks where available. Use the narrowest relevant
checks in the touched child packages only.

## Success Criteria

A component wave is done only when:

- the Poodle component has a clearly documented canonical modern surface
- legacy baggage removed in that wave is actually gone from the component
- Underlay is updated where relevant
- every known consumer usage in scope has been updated or explicitly parked
- validation passed in the touched repos

## Risk Controls

- avoid mass mechanical rewrites with no consumer migration
- avoid “compat forever” aliases that preserve two public shapes indefinitely
- avoid converting internals to runes while leaving the confusing public API
  untouched; public seam cleanup comes first
- avoid cross-root drift by treating each app family root as the unit of
  rollout evidence
- avoid compromise designs chosen mainly to protect outdated downstream usage;
  update the downstream usage instead

## Initial Inventory Notes

Known highest-priority consumer surfaces from current local scan:

- Underlay Nightfire/editor surfaces
- admin and account forms across `acme-admin`, `cp-admin`, `compli-me/admin`,
  `dairy`, `greenhouse`, and `composer-admin`
- direct desktop UI in `finch/app-electron`, `soundcheck`, and `loophole/aura`

Known import-shape diversity that may need cleanup during rollout:

- `@poodle/svelte`
- `@poodle/svelte-primitives`
- `@poodle/svelte-composites`

## Next Task

Start Wave 1 with `Select` and `TextInput`.

Before code changes:

1. produce a concrete consumer usage inventory for those two components across
   `underlay`, the six Underlay-root app families, and the direct desktop apps
2. decide the canonical modern public shape and which legacy props or events
   will be removed, retained temporarily, or bridged
3. only then implement the Poodle change and migrate consumers in the same wave
