# Component Contract Template

Status: active
Updated: 2026-07-10

Use this template for every new Poodle component before implementation begins.

## 1. Purpose

- Component name:
- Layer: `foundation | composites | workstation`
- Summary:
- In scope:
- Out of scope:

## 2. Anatomy

```text
[Root]
  ├── [Part A]
  ├── [Part B]
  └── [Part C]
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes/no | | |
| ... | | | |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| | | | | |

### Naming Rules

- use `camelCase` for multi-word props
- use `is*` booleans such as `isDisabled`, `isLoading`
- use `on*` handler props such as `onClick`, `onOpenChange`
- use `variant` for semantic appearance families
- use `size` only when the component opts into shared control sizes

### Cross-Renderer Naming Convention

The markdown contract doc is the source of truth and uses camelCase. Each
renderer applies its local naming convention without that counting as drift:

| Doc (camelCase) | Rust `poodle-specs` (snake_case) | Notes |
|-----------------|----------------------------------|-------|
| `isDisabled` | `is_disabled` | `is*` booleans keep the prefix |
| `isLoading` | `is_loading` | |
| `isOpen` | `is_open` | |
| `onClick` | — | event handlers live on component builders, not specs |
| `defaultValue` | `default_value` | multi-word props convert segment-by-segment |
| `ariaLabel` | `aria_label` | |
| `backHref` | `back_href` | |

A doc prop `fooBar` and a Rust field `foo_bar` are considered matching by
convention and must NOT be flagged as drift by audits. Only treat a field
as missing when there is no snake_case field whose name matches the doc's
camelCase prop under segment-by-segment conversion.

Some docs document user-facing booleans without the `is` prefix (e.g.
`disabled`, `loading`, `collapsed`) as a brevity convention. In those cases
the Rust field keeps the `is_*` prefix (`is_disabled`) and the two still
match — the prefix stripping is a documented part of the convention.

### Controlled And Uncontrolled

Document whether the component supports:

- controlled inputs such as `value` plus `onValueChange`
- uncontrolled inputs such as `defaultValue`
- command-only mode with no persistent value

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| default | | |
| hover | | |
| focus | | |
| active | | |
| disabled | | |

### Component States

Use a state table when the state shape is small and linear.

Use a state machine diagram when:

- the component has more than one transient interaction mode
- async loading/open/closing behavior exists
- keyboard, pointer, and focus can drive different transitions

### Behavior Machine

Required for every behavioral component; styled-only components state
`Behavior classification: styled-only (no machine)` here instead. This section
is the source of truth for the headless core (`@inflatable-cookie/poodle-core` TS machines
and the Rust mirror). Rules: transitions are pure; side effects appear only in
the Effects table as named intents; every ARIA/data attribute the machine owns
appears in the Part Attribute Output table as a function of machine state and
context. See `docs/specs/062-headless-core-and-dual-layer-strategy.md`.

Behavior classification: `machine-backed | styled-only`

#### Context

Externally ownable or long-lived values the machine reads and writes. Mark
which are controllable (controlled/uncontrolled seam).

| Field | Type | Initial | Controllable | Meaning |
|-------|------|---------|--------------|---------|
| | | | yes/no | |

#### States

| State | Description |
|-------|-------------|
| | |

#### Events

| Event | Payload | Source |
|-------|---------|--------|
| | | user interaction / keyboard / pointer / programmatic / timer / environment |

#### Transitions

One row per (state, event). Guards are pure predicates over context + payload.
Actions mutate context; effects reference the Effects table by name.

| State | Event | Guard | Target | Actions / Effects |
|-------|-------|-------|--------|-------------------|
| | | | | |

#### Effects

Named side-effect intents the machine emits as data; the adapter executes
them. Every effect must state its cleanup rule.

| Effect | What It Does | Cleanup |
|--------|--------------|---------|
| | | |

#### Part Attribute Output

Attribute/handler output per anatomy part, as a function of state and context.
`data-scope` is the component name on every part; `data-part` is the part
name. Renderers without DOM attributes (GPUI, Jetstream) map each row to the
equivalent native accessibility and state exposure.

| Part | Attribute | Value |
|------|-----------|-------|
| | | |

#### Machinery Dependencies

Shared core services this machine consumes (focus trap, roving tabindex,
dismissable layer, anchor positioning, presence, typeahead, id wiring).

## 5. Events

| Event | When It Fires | Payload | Notes |
|-------|---------------|---------|-------|
| | | | |

## 6. Accessibility

### Semantics

- Role:
- Required attributes:
- Optional attributes:
- Labeling rules:

### Keyboard

| Key | Behavior |
|-----|----------|
| | |

### Focus And Announcement

- focus entry:
- focus exit:
- live-region or announcement behavior:
- GPUI-native accessibility mapping notes:

## 7. Layout

### Sizing

- minimum size:
- maximum size:
- overflow behavior:

### Composition

- parent expectations:
- child expectations:
- resizing rules:

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| | | |

Components should reference semantic roles by default. Primitive tokens should
only appear when they are the semantic source for a documented rule.

## 9. Svelte Notes

- expected substrate:
- wrapper strategy:
- implementation-only details:
- known browser-specific deltas:

## 10. GPUI Notes

- expected crate/module surface:
- theme access strategy:
- implementation-only details:
- known GPUI-native deltas:

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] semantic inputs have the same meaning
- [ ] state transitions match
- [ ] event timing and payload meaning match
- [ ] accessibility rules and keyboard behavior match
- [ ] accessible name, role, state, and value exposure match
- [ ] focus order and restoration behavior match when relevant

### Tier 2: Visual Parity

- [ ] token roles match
- [ ] spacing and sizing match within platform limits
- [ ] overall proportions and hierarchy match

### Tier 3: Implementation Freedom

- [ ] implementation-only differences are documented
- [ ] no implementation detail leaks into the public contract

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| none | | | |

## 13. Approval And Adoption Notes

- contract status: `draft | approved | implemented | parity-reviewed`
- approvers:
- downstream adopters:
- future follow-up:

## Next Task

Copy this template into the correct contract layer and fill it out before
starting implementation work for the component.
