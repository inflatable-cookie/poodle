# Post-g16.011 Native Lane Decision

Status: resolved — promoted into merged `g16.012`
Captured: 2026-08-27
Resolved: 2026-08-27
Source: orchestrator evidence checkpoint after PR #85

## Finding

The ledger is at 40 mounted / 134 missing. The next lane should close a real
foundation-control defect through production dispatch without starting one of
the unresolved breaking editing migrations.

### Collapsible — selected

Collapsible is the smallest high-use disclosure primitive with a complete web
authority and a bounded native repair:

- `CollapsibleSpec::current_open()` correctly resolves controlled `open` or
  `default_open`, but the renderer announces `spec.open.unwrap_or(false)`.
  A default-open native disclosure therefore paints open while reporting
  closed.
- expanded state and region role are placed on the outer shell. The contract
  puts button role, accessible name, expanded state, and content ownership on
  the trigger, with `Region` semantics on the mounted content.
- the trigger is focusable even when disabled, has no structured focus ring or
  explicit sequential tab position, and always carries a pointer cursor.
- the GPUI compatibility wrapper's public-looking `with_id` method is a no-op,
  so multiple mounted instances cannot preserve authored trigger identity
  across host rebuilds.
- the interactive specimen already reports host-owned open state, but its
  "Default open" example uses controlled `open` rather than proving the
  `defaultOpen` seed named by the contract.
- no named mounted GPUI regression proves pointer, Enter, Space, host rebuild,
  default-open behavior, focus traversal, or disabled inertia.

All required node/backend channels already exist: button and region roles,
expanded/controls/labelled-by state, activation, tab position, runtime identity,
and `FocusRing`. The card must reuse them. A new backend capability is a stop.

### Deferred candidates

- **TriStateSwitch:** high-value and close to the existing RadioGroup pattern,
  but its Rust spec still stores legacy `CheckState`, defaults to `excluded`
  while the contract defaults to `default`, and keeps an undocumented `label`
  field. Correct closure is a breaking pre-1.0 spec migration requiring an
  explicit operator decision.
- **NumberInput:** its committed number/raw-draft decision remains open in
  `20260826-213343-number-input-native-value-model.md`.
- **EditableLabel:** committed value, live draft, commit payload, focus
  restoration, and select-on-focus still need a dedicated model decision.
- **Accordion:** the renderer reports the activated item while the contract
  reports the resulting selection and the spec retains overlapping selection
  fields. It should follow a separate semantic/API card, not be folded into
  Collapsible.
- **Display-only components:** a mounted test would not prove useful behavior.
- **Visual and accessibility programmes:** remain distinct evidence lanes.

## Decision

Promote Collapsible into `g16.012`. Keep `CollapsibleSpec` and the existing
`collapsible(...)` renderer signature non-breaking. Make the compatibility
wrapper's existing instance id real; an additive identity-aware renderer path
is allowed if needed, while a breaking renderer/spec migration is not.

Move only Collapsible's GPUI mounted-behaviour cell from `missing` to `mounted`:
40 → 41 mounted and 134 → 133 missing. Known-delta totals stay 115 present / 60
not-applicable. Native assistive-technology and visual evidence remain
unpromoted.
