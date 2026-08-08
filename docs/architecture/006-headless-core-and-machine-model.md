# 006 Headless Core And Machine Model

Status: active
Updated: 2026-07-10
Promoted from: `docs/specs/062-headless-core-and-dual-layer-strategy.md` (g11.004)

## Shape

Poodle is a dual-layer system:

- `packages/core` → `@inflatable-cookie/poodle-core`: framework-free TypeScript behavior —
  component state machines, shared interaction machinery, and domain math.
  No framework imports; DOM APIs only in the thin `dom/` machinery modules.
- Framework layers (`@inflatable-cookie/poodle-svelte` today; future adapters) own rendering,
  reactivity, effect execution, and styling. They consume core; they do not
  duplicate behavior.

## Machine Model

- Machines are **pure functions**, not an interpreter:
  `transition(state?, context, event) → { state?, context, effects[] }`.
  Adapters hold reactive state (Svelte 5 runes) and execute effect intents.
- **Callbacks are effects** (`emitValueChange`, `emitOpenChange`, ...) so
  transitions stay side-effect free and orderings (e.g. onRequestClose
  before onOpenChange) are machine-enforced and unit-tested.
- **Effects are named intents with explicit cleanup rules**; the contract's
  Behavior Machine section is the authoritative spec (see the contract
  template).
- **Events carry event-site facts** (e.g. the originating tab index) rather
  than trusting mirrored state — established by a live regression in the
  Tabs pilot.

## Shared Machinery

Focus (query, trap, restore), dismissable-layer stack (innermost-first
escape/outside dismissal — every overlay registers while open), roving and
menu-list index navigation, anchor positioning (in-house pure resolver;
Floating UI rejected — see spec 062), hover-intent timing, instance ids.

Domain math in core: dates/calendar (with the date value types), colors,
durations, pagination windows, select option logic, input
validation/slug/number parsing, toast timer reconciliation.

## Classification Rule

Every public component's contract carries a Behavior Machine section with a
classification: `machine-backed`, `machine-backed via shared/core
machinery`, `adapter-owned interaction` (recorded extraction debt), or
`styled-only`. New components must classify before implementation.

## Cross-Runtime Contract

Core unit tests double as conformance vectors for the Rust mirror
(`g11.006`): pure transitions and pure math port mechanically; adapters map
part-attribute output to native accessibility exposure.
