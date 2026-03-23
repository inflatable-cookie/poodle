# 009 Form Shell, Validation, And Action Row Rules

Status: active
Updated: 2026-03-11
Depends on: `003-accessibility-and-assistive-technology-baseline.md`

## Purpose

Freeze the baseline composition and validation rules for form fields, field
wrappers, and action rows so richer form work in `g02` builds on one shared
posture rather than disconnected input demos.

## Field Wrapper Rule

Every addressable form control must have one canonical naming and message
surface.

That means:

- the field wrapper owns visible label, help text, and validation copy
- the child control owns value, focus, and editing behavior
- the child control must receive the wrapper's relationship ids or
  GPUI-equivalent metadata

Flint should not duplicate label and error wiring inside each individual form
control implementation.

## Validation State Rule

The baseline validation-state vocabulary is:

- `none`
- `invalid`
- `valid`
- `pending`

These meanings are Tier 1 parity semantics.

Visual treatment may vary by runtime.
State meaning may not.

## Message Precedence Rule

When both pending and invalid information exist, invalid wins for the visible
and assistive relationship surface.

Pending state may still influence action rows or background activity summaries,
but it must not replace an active invalid message on the field.

## Async Validation Rule

Pending validation is part of the form contract, not a follow-up enhancement.

At minimum:

- the field can expose pending state
- pending copy can be surfaced textually
- parent forms can coordinate pending status in the action row or nearby status
  region
- GPUI must provide equivalent announcement/event signaling for material
  pending-state changes

This baseline does not yet require a full validation engine.
It does require the components to carry pending semantics cleanly.

## Action Row Rule

Forms must use a stable action row rather than scattering buttons after the
last field.

The action row must define:

- logical action ordering
- wrapping behavior on narrow layouts
- where status or save-state copy may appear
- how primary and secondary actions align

The action row itself remains structurally neutral unless a later contract
requires stronger semantics.

## Disabled And Busy Coordination Rule

When a form becomes disabled or materially busy:

- field controls must expose disabled or busy semantics consistently
- destructive or state-changing actions must not remain spuriously available
- status text must not be icon-only

The exact policy of whether a busy form is fully disabled is parent-owned.
The baseline requires the policy to be explicit and coherent.

## Accessibility Rule

The form baseline must preserve:

- visible labels
- real control-to-description relationships
- real control-to-error relationships
- explicit pending/error announcement surfaces
- logical tab order through fields and actions

Svelte should use native form, label, and input behavior first.
GPUI must intentionally recreate the same meaning through native accessibility
nodes and events.

## Seed Evidence

The first concrete evidence for this spec is:

- `docs/contracts/foundation/field.md`
- `docs/contracts/foundation/form-actions.md`
- `docs/contracts/foundation/text-input.md`
- `docs/contracts/foundation/search-field.md`
- `packages/svelte/primitives/src/Field.svelte`
- `packages/svelte/primitives/src/TextInput.svelte`
- `packages/svelte/primitives/src/SearchField.svelte`
- `packages/svelte/primitives/src/FormActions.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this baseline into `g02.002` and later milestones so table filters,
pickers, dialogs, and settings forms all reuse one field and action-row
posture.
