# 028 Primitive Baseline And Bits-Aligned Surface

Status: active
Updated: 2026-03-12
Depends on: `005-product-composite-composition-and-information-architecture-rules.md`, `021-public-package-api-stability-and-parity-debt-baseline.md`

## Purpose

Define the true primitive baseline for Poodle instead of inferring it from a mix
of contracts, preview examples, and compound components. This baseline should
be broad enough to cover the standard headless app surface most downstream
products need while keeping Poodle's ownership boundaries intact.

## Baseline Rule

Poodle's primitive baseline is the union of:

- Poodle-owned structural primitives such as layout, surface, spacing, and scroll
  shells that Bits does not provide
- Bits-aligned interaction primitives that represent the standard headless
  control surface most apps need

This means the primitive baseline is larger than the currently shipped Svelte
package and should not be inferred from what happened to land first during the
forms and preview tranches.

## Ownership Rule

Bits remains an implementation substrate, not the public contract.

Poodle still owns:

- component names and entry points
- semantic prop naming
- token-driven appearance
- parity expectations with GPUI
- documentation, examples, and adoption guidance

Bits may accelerate:

- accessibility mechanics
- focus management
- keyboard behavior
- headless state machines
- overlay wiring

## Standard Primitive Families

The generalized primitive baseline should explicitly include these families:

### 1. Structural

- `Box`
- `Stack`
- `Inline`
- `Grid`
- `Spacer`
- `Surface`
- `Separator`
- `ScrollShell`

### 2. Action And Text Entry

- `Button`
- `IconButton`
- `TextInput`
- `TextArea`
- `SearchField`
- `Field`
- `FormActions`
- `EditableLabel`
- `NumberEntry`

### 3. Selection And Value

- `Checkbox`
- `Switch`
- `TriStateSwitch`
- `RadioGroup`
- `SegmentedControl`
- `Select`
- `Slider`
- `RangeSlider`

### 4. Feedback And Status

- `Progress`
- `Spinner`
- `Skeleton`
- `Badge`
- `Pill`
- `Callout`
- `Banner`
- `StatusIndicator`

### 5. Overlay, Navigation, And Menus

- `Tabs`
- `TabStrip`
- `Menu`
- `ContextMenu`
- `Tooltip`
- `Popover`
- `Dialog`
- `Drawer`

### 6. Disclosure

- `Collapsible`
- `Accordion`

## Bits-Aligned Coverage Rule

When Bits already provides a generalized headless primitive family, Poodle should
normally expose a corresponding contract-backed primitive unless one of the
following is true:

- the family is intentionally out of scope for Poodle's product direction
- the family belongs at a higher composite or workstation layer instead
- parity requirements are not yet explicit enough to support a stable wrapper

Silence is not enough. Missing surface area should be treated as explicit debt,
not as an implicit decision.

## Current Svelte Packaging Baseline

After the current tranche, `@poodle/svelte-primitives` covers the full documented
foundation contract set:

- structural primitives: `Box`, `Stack`, `Inline`, `Grid`, `Spacer`,
  `Surface`, `Separator`, `ScrollShell`
- action primitives: `Button`, `IconButton`
- text entry primitives: `Field`, `TextInput`, `TextArea`, `SearchField`,
  `FormActions`, `EditableLabel`, `NumberEntry`
- selection and value primitives: `Checkbox`, `Switch`, `TriStateSwitch`,
  `RadioGroup`, `SegmentedControl`, `Select`, `Slider`, `RangeSlider`
- feedback primitives: `Progress`, `Skeleton`, `Badge`, `Pill`, `Callout`,
  `Banner`, `StatusIndicator`, `Spinner`
- disclosure primitives: `Collapsible`, `Accordion`

The package now covers the current primitive baseline defined by the documented
foundation contracts.

The remaining gap is broader than that baseline: Bits exposes additional
generalized primitives that Poodle has not yet promoted into its own contract
catalogue, including advanced inputs, date or calendar controls, table and
pagination helpers, and toolbar or command-oriented primitives.

## Packaging Rule

Primitive package growth should happen in meaningful family tranches.

Good batches:

- layout plus surface primitives together
- value controls together
- overlay and navigation primitives together
- advanced input or date families together when their contracts are ready

Bad batches:

- adding one lonely primitive because a preview route needed it
- shipping ad hoc wrappers that bypass the contract vocabulary
- treating composites as the de facto public baseline while primitives remain
  missing

## Downstream Guidance

Downstream apps should assume the primitive layer is the base reusable surface.
Compound components should compose it, not substitute for it.

When a downstream app needs a generalized control family already covered by
Bits, the expected direction is:

1. define or confirm the Poodle contract
2. implement the Poodle primitive wrapper
3. compose it upward into composites or workstation shells

Not:

1. build more product-level composites
2. let the primitive gap remain implicit
3. backfill the primitive later only if it becomes painful

## Evidence

- `docs/contracts/components/README.md`
- `docs/research/source-hubs/hub-bits.md`
- `docs/research/translation-memos/tm-svelte-substrate.md`
- `packages/svelte/primitives/README.md`
- `packages/svelte/primitives/src/index.ts`

## Next Task

Decide whether to widen the Poodle primitive contract catalogue beyond the current
foundation baseline so more of the wider Bits surface becomes explicit rather
than remaining implementation-only potential.
