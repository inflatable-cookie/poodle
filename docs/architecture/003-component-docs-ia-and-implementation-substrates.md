# 003 Component Docs IA And Implementation Substrates

Status: active
Updated: 2026-03-11
Depends on: `001-poodle-system-shape.md`, `002-token-system-and-package-layout.md`

## Purpose

Freeze the contract-documentation information architecture plus the first
implementation-substrate rules for Svelte and GPUI so Poodle can grow from token
infrastructure into real shared components without leaking runtime-specific
assumptions into the canonical contract.

## Research Inputs

This note should stay aligned with:

- `docs/research/translation-memos/tm-contract-template.md`
- `docs/research/translation-memos/tm-svelte-substrate.md`
- `docs/research/value-tracks/tk-cross-framework-contracts.md`
- `docs/research/source-hubs/hub-bits.md`
- `docs/research/source-hubs/hub-gpui.md`

## Documentation Information Architecture

Poodle now carries three contract layers under `docs/contracts/`:

- `foundation/` for primitives and low-level layout
- `composites/` for reusable application/productivity components
- `workstation/` for reusable shell and panel-system components

The template at `docs/contracts/template/component-contract-template.md` is the
required starting point for every component.

The contract surface exists to define semantics. It is not a docs-site skin or
an implementation changelog.

## Svelte Substrate Policy

### Posture

Poodle's Svelte implementation may use Bits as an internal substrate, but Bits is
not the public contract source of truth.

### What Bits May Own

- accessibility wiring
- keyboard behavior internals
- focus management internals
- overlay plumbing
- compound state management where Bits already provides stable primitives

### What Poodle Must Own

- public prop names and types
- semantic variants and sizes
- token usage and styling rules
- documentation and parity checklists
- delta recording and cross-framework guarantees

### Wrapper Rule

Bits-backed components must be wrapped in Poodle-owned components or part wrappers
before they are public.

App code should not need:

- Bits imports
- Bits types
- Bits naming
- Bits-specific state-machine knowledge

### Styling Rule

Svelte implementations consume generated token artifacts through:

- `packages/svelte/tokens/`
- CSS variable references
- wrapper-owned `class` and `data-*` styling conventions

The browser-side styling system may use utility classes or authored CSS, but
the semantic token source remains Poodle-owned.

### Compound Versus Simple Exports

Use Poodle compound exports for complex structures such as dialogs, menus, tabs,
and selects when part-level control is essential.

Use a simplified single-surface export when:

- the component interaction is shallow,
- and collapsing parts does not hide necessary semantics.

### No-Leak Rule

Svelte implementation choices must not become canonical web-contract
requirements. If a convenience is Svelte-only, document it as implementation
detail rather than as the contract.

## GPUI Substrate Policy

### Posture

Poodle's GPUI implementation is a native GPUI surface, not a browser abstraction
ported into Rust.

### Crate Shape

The first intended GPUI ownership split is:

```text
packages/gpui/
  tokens/         # generated Rust token consumption
  primitives/     # foundation components
  composites/     # reusable app-level composites
  workstation/    # workstation shell components
```

### Token Ingestion Rule

GPUI consumes generated Rust artifacts from `packages/tokens/artifacts/rust/`.

It must not define hand-maintained duplicate canonical values for:

- semantic colors
- spacing and sizing roles
- theme identities
- density and control-size overlays

### Theme Access Rule

GPUI components should read semantic roles through generated theme helpers or a
Poodle-owned theme layer assembled from generated Rust outputs.

The token meaning lives in the schema/artifact layer, not inside GPUI render
code.

### Idiomatic GPUI Rule

GPUI components may remain idiomatic by using:

- native GPUI layout APIs
- GPUI entity/view patterns
- GPUI-native focus and event handling
- GPUI-native rendering decisions

These choices are valid as long as they do not redefine the canonical
component contract.

### Native Delta Rule

Allowed GPUI-native deltas include:

- code-based styling internals instead of CSS
- GPUI-native focus-ring rendering
- GPUI-specific layout implementation details
- immediate/retained-mode hybrid rendering internals

These remain acceptable only when Tier 1 and Tier 2 parity requirements stay
intact and the deltas are documented.

## Shared Parity Expectations

The substrate split exists to preserve one semantic contract across two
different runtime models.

What must match:

- props and input meaning
- states and transition meaning
- event meaning
- accessibility semantics
- token-role usage

What may differ:

- implementation mechanics
- runtime-specific helper APIs
- low-level layout/styling technique

## Accessibility Posture

Accessibility is part of the substrate contract, not only part of web markup.

### Svelte

The Svelte side should lean on semantic HTML and standard browser accessibility
machinery wherever possible.

### GPUI

The GPUI side must intentionally provide:

- native accessible node mappings
- role/name/state/value exposure
- keyboard reachability
- visible focus
- focus restoration
- dynamic announcements where the component contract requires them

GPUI accessibility gaps are not acceptable “native differences” unless the
contract explicitly documents an approved delta and a remediation path.

## First Seed Contracts

The first seeded contract examples proving this IA and substrate posture are:

- `docs/contracts/foundation/box.md`
- `docs/contracts/foundation/stack.md`
- `docs/contracts/foundation/inline.md`
- `docs/contracts/foundation/grid.md`
- `docs/contracts/foundation/spacer.md`
- `docs/contracts/foundation/surface.md`
- `docs/contracts/foundation/separator.md`
- `docs/contracts/foundation/scroll-shell.md`
- `docs/contracts/foundation/button.md`
- `docs/contracts/workstation/panel-surface.md`

## Next Task

Use this architecture note to guide `g01.008` through `g01.010` so the
interactive primitive contracts preserve explicit GPUI accessibility support
alongside runtime-neutral semantics.
