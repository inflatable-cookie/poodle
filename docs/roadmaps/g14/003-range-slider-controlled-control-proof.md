# g14.003 — RangeSlider Controlled-control Proof

Status: planned
Depends on: `g14.002`

## Outcome

Prove the kernel handles a controlled, two-part value component without
hard-coded runtime knowledge.

## Scope

- Move RangeSlider portable interface and specimen structure into the shared
  authority/cases.
- Exercise lower/upper values, crossing policy, clamping, keyboard steps,
  pointer drag, disabled state, orientation, accessible names, and event order.
- Observe both identified thumbs, values, roles, focus, token roles, and
  geometry in all active runtimes.
- Replace the thin three-case slider vector where the shared component cases
  cover the same claim; preserve the native role fix.

## Acceptance

- Thumb identity and two-value semantics are exact across runtimes.
- Controlled update and callback order pass through real adapters.
- Existing hand-written fixture copies and redundant vectors are removed.
- A planted crossing, thumb-order, handler, or role divergence fails.
- Cost and new vocabulary delta stay bounded and generic.

## Stop Conditions

- The case model needs RangeSlider-specific nodes or actions.
- A backend passes from `poodle-node` construction without executing its
  interaction path.

## Validation

Run all RangeSlider cases, narrow web/native tests, conformance gates,
`docs:check`, and `git diff --check`.
