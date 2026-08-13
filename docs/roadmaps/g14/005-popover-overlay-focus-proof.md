# g14.005 — Popover Overlay And Focus Proof

Status: planned
Depends on: `g14.004`

## Outcome

Prove overlay composition, placement, dismissal, focus transfer, and
runtime-owned layer mechanisms can differ while their observable result stays
contract-bound.

## Scope

- Move Popover interface/specimen structure into shared authority.
- Cover trigger/content regions, controlled open state, placement, Escape,
  outside interaction, request-close/event order, initial focus, and restore.
- Observe layer relationship, role/name, focus path, placement result, token
  roles, and dismissal effects through each real runtime.
- Reconcile existing popover machine-interface/vector experiments with the
  component cases.

## Acceptance

- Equivalent focus and dismissal results pass despite runtime mechanism.
- Placement geometry uses named bounded tolerances only.
- Inert outside/Escape handling and broken focus restore fail.
- Generic schema/runners gain no Popover-specific behaviour.
- Redundant machine/specimen surfaces are removed and costed.

## Stop Conditions

- A tolerance masks missing content, focus, dismissal, or placement.
- The shared case needs portal or backend implementation details.

## Validation

Run Popover cases and overlay/focus suites, conformance gates, `docs:check`,
and `git diff --check`.
