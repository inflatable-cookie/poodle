# g11.001 Svelte Modernization And Consumer Rollout

Status: active
Owner: Poodle core
Depends on: g10 closeout
Updated: 2026-05-15

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

Current execution state:

- `Select`
  - Poodle component, docs, and specimen moved to the callback-first Svelte 5 surface
  - consumer rollout completed across `underlay`, `underlay-reference`,
    `contact-patch`, `songsprout`, `loophole/composer`, and `acowtancy/dairy`
- `TextInput`
  - Poodle component, docs, guide examples, and specimen moved to bindable
    value plus callback props (`onValueChange`, `onValidationChange`,
    `onSubmit`, `onCancel`, `onClear`, `onKeyDown`, `onFocus`, `onBlur`)
  - Poodle-side consumers updated
  - non-`dairy` consumer rollout completed across `underlay`,
    `underlay-reference`, `contact-patch`, `songsprout`, and
    `loophole/composer`
  - `acowtancy/dairy` remains the open `TextInput` batch because it still has
    the largest concentration of app-owned wrappers and Nightfire/editor forms

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

## Remaining Dispatcher Audit

Snapshot taken after the current callback-first sweep.

### Highest Spread Remaining Seams

No high-spread public dispatcher seams remain in the audited consumer set.

### Internal Or Low-Consumer Seams

No meaningful internal dispatcher seams remain from the audited set. The last
low-consumer cleanup batch covered:

- `ActionDiscoveryPanel`
- `SelectionSummary`
- `CollapseToggle`

### Completed Since Snapshot

- `OrderBy`: dispatcher removed; public seam is now callback-only via `onChange`
- `ListContainer`: dispatcher removed; public seam is now callback-only via `onPageChange`
- `BulkActionBar`: dispatcher removed; public seam is now callback-only via `onAction`, `onClear`, and `onSelectAll`
- `EditableList`: dispatcher removed; public seam is now callback-only via `onReorder`, `onAdd`, `onRemove`, `onChange`, `onSubmit`, and `onCancel`
- `FileUpload`: dispatcher removed; public seam is now callback-only via `onChange`, `onUpload`, `onError`, and `onRemove`
- `DataTable`: dispatcher removed; public seam is now callback-only via `onSortChange`, `onRowToggle`, `onToggleAll`, `onRowAction`, `onRowActionSelect`, `onColumnVisibilityChange`, `onExportCsv`, `onRowClick`, `onFilterChange`, `onPageChange`, and `onLimitChange`
- `Button` and `IconButton`: dispatcher removed; public seams are now callback-only via `onClick`, `onFocus`, `onBlur`, and `onPressedChange`
- `MarkdownEditor`: dispatcher removed; public seam is now callback-only via `onValueChange` while preserving `bind:value`
- `NavCard`: dispatcher removed; public seam is now callback-only via `onClick`
- `ListCard`: dispatcher removed; public seams are now callback-only via `onClick` and `onSelectedChange`
- `ResizeHandle`: dispatcher removed; public seams are now callback-only via `onResizeStart`, `onResizeMove`, `onResizeEnd`, and `onResizeStep`
- `SplitView`: dispatcher removed; public seams are now callback-only via `onRatioChange`, `onPrimaryCollapsedChange`, and `onSecondaryCollapsedChange`
- `EmbedInput`: dispatcher removed; public seams are now callback-only via `onParse` and `onValueChange`
- `ScrollShell`: dispatcher removed; public seam is now callback-only via `onScroll`
- `BlockEditor`: dispatcher removed; public seam is now callback-only via `onChange`
- `ColorPicker`: dispatcher removed; public seams are now callback-only via `onChange` and `onOpenChange`
- `ActionDiscoveryPanel`: dispatcher removed; public seams are now callback-only via `onItemSelect` and `onActiveChange`
- `SelectionSummary`: dispatcher removed; public seams are now callback-only via `onRemove` and `onClear`
- `CollapseToggle`: dispatcher removed; public seam is now callback-only via `onToggle`
- `ListContainer` and `FilterToolbar`: moved to runes-based internals and snippet-first composition
- `ListCard`: moved to runes-based internals and snippet-first composition, with Underlay `EntityListCard` migrated to the new snippet surface
- `DataTable`: moved to runes-based internals and snippet-first composition, with `cell`, `expandedRow`, and `empty` migrated to snippet props across the active consumer set
- `PickerShell` and `RelationPicker`: moved to runes-based internals and snippet-first composition, with `PickerShell` replacing toolbar/selection/state/footer slots and `RelationPicker` consuming the new shell surface directly
- `DockRegion` and `SplitView`: moved to runes-based internals and snippet-first composition, with `DockRegion` replacing `panel`/body slots and `SplitView` replacing primary/secondary slots across Aura, Finch, and the preview surface
- `EditableList` and `LogList`: moved to runes-based internals, with `EditableList` keeping its bindable item model on a cleaner ownership boundary and `LogList` moving its internal filters to bindable state without prop mutation footguns
- `Calendar`, `DatePicker`, `DateRangePicker`, `DateTimePicker`, `DateTimeRangePicker`, and `DateTimeZonePicker`: moved to runes-based internals and cleaned up seeded default-state ownership across the full picker cluster
- `ColorPicker`: moved to runes-based internals and cleaned up value/open ownership without changing visual behavior
- `BlockEditor`: moved to runes-based internals and snippet-first composition, replacing the old `block` / `type-picker` / `add-picker` slot surface across the specimen and Underlay Nightfire wrapper
- `CardRadioGroup`: moved to runes-based internals and snippet-first composition, replacing the old `card` slot surface in the active Dairy consumer

### Risk Notes

- `DataTable` has moderate file count but broad root spread and many event
  seams, so it is a real migration tranche, not a cleanup footnote.
- `NavCard` and `ListCard` were simpler mechanically, but both were highly
  visible layout surfaces and needed the same parity caution as `TextInput`.

## Next Batch Order

The public dispatcher audit is effectively closed. Next waves should focus on:

1. non-event modernization
   - remaining legacy slot/snippet and controlled/uncontrolled cleanups
   - specimen/doc language cleanup where stale event terminology still survives
2. residue audit
   - internal wrappers that still lag modern callback/style parity
   - docs/specimens that still describe event-era behavior

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

## Wave 1 Inventory Baseline

Scope:

- `Select`
- `TextInput`

### Root Coverage

Measured as files with actual component usage, not just package installation.

| Root | TextInput files | Select files | Notes |
|------|-----------------|--------------|-------|
| `underlay` | 9 | 4 | Core auth flows, `EntityList`, Nightfire media/type select |
| `underlay-reference` | 15 | 9 | `acme-admin`, `acme-front`, `acme-ui` all in play |
| `contact-patch` | 5 | 4 | Mostly `cp-admin`; account/media/user flows |
| `compli-me` | 9 | 6 | `admin` plus `front` auth pages |
| `acowtancy` | 69 | 29 | Biggest hotspot by far: `dairy` and `froyo` forms, selectors, Nightfire editors |
| `songsprout` | 10 | 1 | `greenhouse` and `bloom`; mostly auth and media edit |
| `loophole/composer` | 6 | 6 | `composer-admin` editing/detail flows |
| `finch/app-electron` | 0 | 0 | Direct Poodle user, but not of these two components in current scan |
| `soundcheck` | 1 | 0 | One direct `TextInput` usage |
| `loophole/aura` | 1 | 0 | One direct `TextInput` usage |

### Hotspots

- `acowtancy/dairy`
  - dominant heavy consumer for both components
  - forms, filters, relation selectors, Nightfire question editors
- `underlay`
  - canonical wrapper and pattern layer for many downstream auth/list flows
- `underlay-reference/acme-admin`, `contact-patch/cp-admin`,
  `compli-me/admin`, `loophole/composer/composer-admin`
  - repeated admin-account/media/detail usage patterns

### Usage Pattern Notes

`Select` current downstream patterns:

- mixed canonical and compatibility inputs:
  - `options={...}`
  - `items={...}` still present in real consumers and is common in older admin
    flows
- mixed event surfaces:
  - `bind:value`
  - `onValueChange`
  - legacy callback prop `onchange={(value) => ...}` still appears in live
    consumers
- grouped/custom select usage exists inside Underlay-owned surfaces such as
  `NightfireTypeSelect`

`TextInput` current downstream patterns:

- common controlled forms use `bind:value`
- event-driven controlled forms also use `onValueChange`
- multiline usage via `rows={...}` is live in admin editing flows
- search usage via `type="search"` is live in relation selectors and filters
- direct desktop usage exists but is light: `soundcheck` and `loophole/aura`

### Wave 1 Modernization Implications

- `Select` must be treated as the harder seam first, because it still has live
  compatibility alias usage (`items`) and multiple event styles in downstream
  apps
- `TextInput` has wider usage volume, but the public shape looks more stable;
  most risk is around controlled/value patterns, multiline/search behavior, and
  validation ergonomics
- Underlay must lead the rollout for both components because its auth/pattern
  surfaces feed multiple app families
- `acowtancy/dairy` should be treated as the downstream proving ground after
  Underlay because it carries the highest real usage density

## Wave 1 Target Posture

This is the current target direction for implementation work. Treat it as the
 modernization brief unless new evidence from the consumer sweep forces a better
 answer.

### `Select`

Target outcome:

- one canonical option input shape: `options`
- no long-term parallel compatibility props for:
  - `items`
  - `groups`
  - `loadItems`
  - `loadGroups`
- modern callback-first change surface instead of treating
  `createEventDispatcher` events and `onchange` callback prop as co-equal
- modern composition surface for trigger, option, and empty rendering instead
  of extending legacy slot dependence further

Implication:

- existing downstream `items=` and `onchange=` usage should be migrated, not
  preserved as the end state

Current execution status:

- Poodle-side `Select` rewrite started
- component contract moved to Svelte 5 runes, bindable `value`,
  callback-first state change hooks, and snippet-based custom rendering
- legacy `items`, `groups`, `loadItems`, `loadGroups`, and `onchange`
  removed from the Poodle component surface
- in-repo `Select` consumers, specimen, and component docs updated
- downstream consumer migration still pending; `underlay` is the next boundary

### `TextInput`

Target outcome:

- keep the useful semantic breadth: text, search, slug, multiline, validation
- preserve a single clear controlled/uncontrolled model
- move to callback-first value, submit, cancel, clear, and validation hooks
  instead of dispatcher-first public semantics
- modernize leading/trailing composition without extending legacy slot posture
- keep multiline and search behavior explicit and well-scoped; avoid carrying
  ambiguous “auto magic” further unless it materially improves the API

Implication:

- `TextInput` is likely a deeper internal modernization with a smaller public
  contract rewrite than `Select`
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

## Post-Dispatcher Audit

Snapshot taken after the dispatcher cleanup, ownership cleanup, and docs/specimen
residue pass.

### Current Shape

- no `createEventDispatcher` files remain in `packages/svelte/components/src`
- legacy internal structure is still widespread:
- `0/124` component files still use `export let`
- `0/124` component files still use `$:` reactive labels
- `0/124` component files still expose slot-heavy composition surfaces

This is now a different tranche from the public-event rollout. The remaining
work is about internal modernization and composition shape, not event cleanup.

### Highest Legacy-Structure Hotspots

No legacy-structure hotspots remain in `packages/svelte/components/src`.

### Next Tranche

If `g11` continues, the next worthwhile batch is not another broad repo sweep.
It is a focused implementation tranche on the highest-spread core composites
and shells that still define downstream composition style:

1. recently completed in this lane:
   - `UiPresentationProvider`
   - `FieldSet`
   - `MetaBar`
   - `MetaItem`
   - `ListCardCounter`
   - `Region`
   - `Eyebrow`
   - `IconProvider`
   - `Separator`
   - `Spacer`
   - `Field`
2. already completed in this lane:
   - `Card`
   - `DetailItem`
   - `DetailShell`
   - `AppHeader`
   - `MediaPreview`
   - `StatusBar`
   - `Callout`
   - `CommandPalette`
   - `Calendar`
   - `DatePicker`
   - `DateRangePicker`
   - `DateTimePicker`
   - `DateTimeRangePicker`
   - `Collapsible`
   - `ActionDiscoveryPanel`
   - `Pagination`
   - `OrderBy`
   - `MediaThumbnail`
   - `Skeleton`
   - `MetricTile`
   - `PaginationSummary`
   - `NavCard`
   - `Toolbar`
   - `ScrollShell`
   - `UiPresentationProvider`
   - `FieldSet`
   - `MetaBar`
   - `MediaPicker`
   - `MediaBrowsePanel`
   - `EmptyState`
   - `DetailSection`
   - `FormLayout`
   - `EditableLabel`
   - `Progress`
   - `VideoPlayer`
   - `PasswordRequirements`
   - `MarkdownEditor`
   - `AudioPlayer`
   - `FormActions`
   - `TimeAgo`
   - `Meter`
   - `Code`
   - `Table`
   - `Breadcrumbs`
   - `FileUpload`
   - `EmbedInput`
   - `MediaUploadStatusPanel`
   - `Pill`
   - `StatusIndicator`
   - `Icon`
   - `Stack`
   - `Box`
   - `DebugDialog`
   - `PageLoading`
   - `Spinner`
   - `ResizeHandle`
   - `ToastHost`
   - `ListGrid`
   - `ToastStack`
   - `Grid`
   - `Surface`
   - `EmbedPreview`
   - `CollapseToggle`
   - `SelectionSummary`
   - `BulkActionBar`
   - `AudioPlayer`
   - `DateTimeZonePicker`
   - `ColorPicker`
   - `BlockEditor`
   - `CardRadioGroup`

Target for that tranche:

- move from `export let` / `$:` internals to runes-based state where it makes
  the component simpler
- replace named-slot-first public APIs with snippet-first surfaces where the
  composition contract should now be modernized
- preserve visual parity and size/density behavior as hard constraints

Do not reopen low-value syntax churn on small primitives first. The remaining
value is concentrated in the components that shape downstream composition
patterns.
