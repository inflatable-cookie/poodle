# 002 Component Contract Template And Parity Rules

Status: active
Updated: 2026-03-11
Depends on: `001-token-source-and-artifact-contract.md`

## Purpose

Freeze the normative rules for Poodle component contracts so every new component
is specified against one repeatable template before Svelte or GPUI
implementation begins.

## Contract Source Of Truth

Poodle component contracts live under `docs/contracts/`.

Each component must belong to exactly one of these contract layers:

- `docs/contracts/components/`
- `docs/contracts/components/`
- `docs/contracts/workstation/`

The template source of truth lives at:

- `docs/contracts/template/component-contract-template.md`

## Required Contract Sections

Every contract must contain these sections in order:

1. purpose
2. anatomy
3. props and inputs
4. states
5. events
6. accessibility
7. layout
8. token usage
9. Svelte notes
10. GPUI notes
11. parity checklist
12. known deltas
13. approval and adoption notes

No component may skip these sections by replacing them with implementation
notes or loose examples.

## Naming Rules

Canonical public contracts must follow these naming rules:

- multi-word props use `camelCase`
- booleans use `is*`, `has*`, or another explicit state prefix
- event callbacks use `on*`
- appearance families use `variant`
- shared control-size participation uses `size`

Framework-local prop shapes may adapt internally, but those adaptations must
not replace the canonical contract language.

## Token Usage Rule

Contracts must describe token usage against anatomy parts, not against vague
screenshots or general styling descriptions.

Contracts should reference semantic tokens by default.

Primitive tokens may appear only when:

- they are the documented semantic source,
- or a primitive is intentionally exposed as part of a lower-level layout rule.

## Accessibility Rule

Every contract must define:

- semantic role expectations
- required and optional accessibility attributes
- keyboard behavior
- focus entry/exit behavior
- announcement behavior when dynamic content is involved

Accessibility cannot be deferred to implementation notes alone.

## State Documentation Rule

Use a state table when the interaction model is shallow.

Use a state machine or equivalent transition diagram when:

- multiple transient modes exist,
- async or delayed states exist,
- or keyboard, pointer, and focus transitions are materially different.

## Parity Model

### Tier 1: Strict Parity

Must match across Svelte and GPUI:

- semantic input meaning
- state transitions
- event timing and payload meaning
- accessibility and keyboard rules
- value/control semantics where relevant

### Tier 2: Visual Parity

Should match across Svelte and GPUI:

- token-role usage
- spacing and sizing proportions
- typography hierarchy
- visual emphasis hierarchy

### Tier 3: Implementation Freedom

May differ across Svelte and GPUI:

- internal state implementation
- rendering internals
- CSS versus GPUI styling mechanics
- platform-native focus or animation details

These differences remain allowed only if they do not change Tier 1 or Tier 2
meaning.

## Documented Delta Rule

Any intentional cross-framework difference must be recorded in the `Known
Deltas` section with:

- the delta itself
- why it is allowed
- approval status
- follow-up if the delta should be revisited

Undocumented deltas are treated as defects until reviewed.

## Parity Evidence Rule

A component should not claim parity maturity from contract existence alone.

At minimum:

- `implemented` requires a completed contract plus concrete implementation
  intent or realization
- `parity-reviewed` requires traceable evidence that both runtimes were checked
  against the contract and that any deltas were explicitly recorded

The exact evidence format may vary by milestone, but the evidence must exist
and be discoverable.

## Approval Workflow

Before implementation begins:

1. create or update the contract document
2. fill all required sections
3. record token usage and accessibility requirements
4. mark contract status as `draft`
5. review and promote to `approved`

After first implementation:

1. mark status as `implemented`
2. record any discovered deltas
3. complete the parity checklist
4. promote to `parity-reviewed` when both runtimes have been checked

## Seed Evidence

The first seed contracts proving this template are:

- `docs/contracts/components/button.md`
- `docs/contracts/workstation/panel-surface.md`

## Next Task

Use this spec together with `008-parity-evidence-documented-delta-and-downstream-extension-rules.md`
when `g02` starts turning the documented surface into deeper implementations
and real parity review work.
