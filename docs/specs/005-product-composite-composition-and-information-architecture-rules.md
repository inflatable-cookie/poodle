# 005 Product Composite Composition And Information Architecture Rules

Status: active
Updated: 2026-03-11
Depends on: `002-component-contract-template-and-parity-rules.md`, `003-accessibility-and-assistive-technology-baseline.md`, `004-overlay-focus-dismissal-and-layering-rules.md`

## Purpose

Freeze the first rules for product-style composites so Pug can support
Underlay-style applications and Loophole-adjacent product surfaces without
smuggling app-specific workflows into the shared contract layer.

## Core Rule

Product composites compose primitives into reusable information architecture.

They do not own:

- app-specific domain models
- data fetching policy
- persistence policy
- command registry wiring
- authorization rules
- or one-off screen logic

If a component needs domain-specific nouns to describe its core semantics, it
is probably not a generic product composite.

## Product Composite Families

The first composite tranche is grouped into:

- information display: `Card`, `PageHeader`, `Breadcrumbs`, `DetailRow`,
  `DetailSection`, `DetailShell`, `EmptyState`
- browse shells: `FilterToolbar`

These families should cover:

- detail and settings-style views
- catalog or browser-style collection views
- empty and light remediation states

They intentionally stop short of:

- domain-specific editors
- workflow wizards
- command palettes
- and workstation shell infrastructure

## Primitive Composition Rule

Composite contracts must name the primitive surfaces they build upon.

That includes primitives such as:

- `Surface`
- `ScrollShell`
- `Button` and `IconButton`
- `SearchField`
- `Select`
- `SegmentedControl`
- `Tabs`
- feedback primitives such as `Callout`, `Banner`, `Progress`, and `Skeleton`

Composite contracts may simplify composition for downstream apps.
They may not redefine primitive semantics.

Examples:

- `FilterToolbar` groups controls but does not redefine text-entry semantics
- `DetailShell` owns scroll/state posture but not field-edit semantics

## Information Architecture Rule

Product composites must preserve navigable information hierarchy.

That means:

- page and section headings remain explicit
- breadcrumb location context remains distinct from tab or shell navigation
- detail rows preserve label/value relationships
- empty-state messaging remains textual and actionable when needed
- browse shells make the relationship between controls, summary, and results
  legible

The contract layer should document these IA relationships directly rather than
leaving them to screen-specific conventions.

## Accessibility Rule

Product composites must keep accessibility explicit even when they mostly
arrange lower-level primitives.

That includes:

- heading hierarchy
- landmark or named-region behavior where relevant
- breadcrumb current-location semantics
- label/value relationships in detail views
- empty-state message availability independent of decoration
- focus continuity when list, grid, or detail shells swap between ready,
  empty, loading, and error states

GPUI must implement equivalent named-region, grouped-content, and state-change
semantics through native accessibility APIs or accessible node structures.

## Shell Neutrality Rule

Product composites may own:

- local headers
- browse controls
- state regions
- body scroll boundaries

They may not silently absorb:

- app shell chrome
- workstation docking/tab-strip semantics
- or global command routing

Those belong in workstation contracts or downstream app code.

## State Ownership Rule

Product composites may expose coarse semantic state such as:

- `ready`
- `empty`
- `loading`
- `error`

They must not hide the source of those states.

Hosts still own:

- when data is loading
- what counts as empty
- what error copy to display
- and what remediation actions are available

## Collection Rule

Browse shells define browse framing, not item semantics.

Therefore:

- row or tile rendering stays host-owned
- selection models stay host-owned unless a future specialized composite owns
  them explicitly
- virtualization strategy stays implementation detail
- shells must not force inappropriate roles such as pretending card grids are
  spreadsheet grids

## GPUI-Specific Rule

GPUI implementations must explicitly preserve:

- named-region hierarchy
- heading relationships
- label/value semantics
- browse-shell state changes
- scroll ownership
- and focus continuity when shell state changes

The absence of HTML landmarks does not relax these requirements.

## Contract Author Checklist

Product composite authors must define:

- which primitives are composed
- what semantic value the composite adds beyond those primitives
- what stays app-owned
- what heading or region structure exists
- how empty/loading/error posture is handled
- how focus continuity behaves when content changes
- GPUI-native accessible-tree expectations

## Seed Evidence

The first contracts that explicitly exercise this baseline are:

- `docs/contracts/composites/card.md`
- `docs/contracts/composites/page-header.md`
- `docs/contracts/composites/breadcrumbs.md`
- `docs/contracts/composites/detail-row.md`
- `docs/contracts/composites/detail-section.md`
- `docs/contracts/composites/detail-shell.md`
- `docs/contracts/composites/filter-toolbar.md`
- `docs/contracts/composites/empty-state.md`

## Next Task

Use this composite baseline while executing `g02.001` and later depth tranches,
keeping the extension boundary clear as the catalogue expands.
