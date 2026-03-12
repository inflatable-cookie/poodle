# 029 Advanced Primitive Promotion And Substrate Mapping

Status: active
Updated: 2026-03-12
Depends on: `005-product-composite-composition-and-information-architecture-rules.md`, `028-primitive-baseline-and-bits-aligned-surface.md`

## Purpose

The documented foundation baseline is now implemented in Svelte, but the wider
headless substrate still exposes additional generalized primitives. This spec
defines how those families should be promoted into Pug instead of either
stopping too early or copying the substrate catalogue uncritically.

## Promotion Rule

Wider substrate primitives should become first-class Pug contracts only when
all of the following are true:

- the surface is generalized enough for downstream apps beyond one product
- the semantics fit cleanly into Pug's foundation layer rather than an existing
  composite or workstation layer
- the public API can remain Pug-owned rather than mirroring substrate naming
- parity expectations with GPUI can be stated concretely

## Non-Promotion Rule

Not every substrate primitive should become a Pug foundation primitive.

In particular:

- command-launcher semantics already belong primarily to workstation contracts
- table-heavy semantics already live in composite contracts such as
  `DataTable`
- pagination posture currently lives at the browse and table composite layer,
  not as a standalone low-level primitive

## Current Expansion Tranche

The first post-baseline advanced primitive tranche promotes these families into
Pug foundation:

- `Toggle`
- `ToggleGroup`
- `Toolbar`
- `Meter`
- `PinInput`
- `Combobox`

These are generalized utility or advanced-input surfaces that still fit cleanly
inside the foundation layer without reopening workstation or data-table
ownership.

## Ownership Mapping Rule

The wider substrate catalogue should currently map into Pug like this:

### Promote To Foundation

- toggle-style action controls
- toolbar grouping semantics
- compact advanced-input controls such as PIN or fixed-code entry
- combobox-style selection plus query controls
- meter-style bounded status displays

### Keep In Composite Or Workstation Layers

- command launchers and discovery surfaces
- table-rich data presentation
- pagination flows tied to browse-state ownership
- specialized workstation tabs or shell-level command infrastructure

### Defer Until Contracts Are Explicit

- none in the currently selected families

## Follow-On Expansion Tranche

The next generalized tranche promotes the date-selection family into
foundation:

- `Calendar`
- `RangeCalendar`
- `DatePicker`
- `DateRangePicker`

These remain date-only foundation surfaces. Time-aware or scheduling semantics
are still intentionally outside the current baseline.

## Current Time Aware Tranche

The next follow-on tranche promotes the smallest time-aware family into
foundation:

- `TimeField`
- `DateTimePicker`

These remain local-value surfaces only. Timezone-aware or scheduling semantics
are still outside the current baseline.

## Current Utility Tranche

The next utility tranche promotes:

- `HoverCard`
- `Rating`

These are foundation-safe utility surfaces that do not themselves claim
routing, shell, or command-bar ownership.

## Current Navigation Tranche

The next navigation tranche promotes:

- `NavigationMenu`
- `Menubar`

These are now explicit foundation surfaces, while routing-aware navigation and
native OS menu integration remain outside the current baseline.

## Current Low-Level Data Tranche

The next data tranche promotes:

- `Table`
- `Pagination`

These are low-level structured data primitives. Richer data-table interaction
and command-discovery semantics remain outside foundation.

## Current Disclosure Tranche

The next disclosure tranche promotes:

- `Collapsible`
- `Accordion`

These are generalized web and docs disclosure surfaces that fit cleanly inside
foundation without leaking product IA or shell orchestration semantics.

## Implementation Rule

Expansion tranches should stay meaningful and family-based.

Good batches:

- utility and advanced-input controls together
- date-selection surfaces together
- utility overlays together when they are not already covered

Bad batches:

- adding one advanced primitive only because a demo needs it
- mirroring substrate names without a Pug ownership decision
- splitting semantically linked families across many tiny implementation passes

## Current Risk

The wider advanced primitive tranche is currently implemented as Svelte-native
wrappers rather than true Bits-backed wrappers. That is acceptable for surface
definition and initial docs alignment, but it should not be confused with the
final substrate posture.

## Evidence

- `docs/research/source-hubs/hub-bits.md`
- `docs/research/translation-memos/tm-svelte-substrate.md`
- `docs/contracts/foundation/README.md`
- `packages/svelte/primitives/README.md`

## Next Task

Use this mapping while deciding whether the next tranche should cover
deeper integration hardening, parity discipline, or a deliberate revisit of
any still-ambiguous ownership boundaries.
