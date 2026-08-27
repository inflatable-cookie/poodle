# Post-g16.010 Native Lane Decision

Status: resolved — closed by `g16.011`; see
`../logs/2026-08/20260827-g16-011-icon-button-activation-toggle-and-mounted-parity.md`
Captured: 2026-08-27
Source: orchestrator evidence checkpoint after PR #84

## Finding

The ledger is at 39 mounted / 135 missing. The next lane should close a
high-use primitive defect through production dispatch, not add mounted tests
to inert presentation components.

### IconButton — selected

IconButton is the action primitive reused by navigation, editing, licensing,
model-management, message, and history composites. Its web contract is clear,
but the shared Rust path is incomplete:

- `default_pressed` is stored but ignored by the renderer;
- the renderer accepts only `on_click`, so it cannot emit the contract's
  resulting `onPressedChange` value;
- the GPUI compatibility wrapper always renders with no handler;
- `tooltip` and its `ariaLabel` fallback never reach the existing
  renderer-neutral native-tooltip channel;
- pressed semantics, explicit button role, sequential tab position, and the
  standard focus ring are not projected together; and
- there is no named mounted GPUI regression for pointer, Enter, Space,
  resulting toggle state, or disabled/loading inertia.

The node/backend already owns activation, focus, `Node.tooltip`, toggled
state, and `FocusRing`. The repair therefore needs no new backend vocabulary
and no breaking public spec migration. Add a handler-bundle entry point while
retaining the simple click entry point as the normal composition helper.

### Deferred candidates

- **EditableLabel:** still needs a committed-value versus live-draft model so
  it can emit `{ value, previousValue }` honestly and restore focus after edit.
- **NumberInput:** remains the highest-leverage editing migration, but its
  breaking native committed-value/raw-draft decision is still open in
  `20260826-213343-number-input-native-value-model.md`.
- **TimeInput:** still needs a native segmented-time design for min, max,
  step, and editing behavior.
- **Pill and other display-only components:** a mounted behavior cell would
  not prove new behavior.
- **Visual and accessibility programmes:** remain separate evidence lanes;
  this card must not promote either from one interaction regression.

## Decision

Promote IconButton into `g16.011`. Close command activation, controlled and
seeded toggle reporting, native tooltip text projection, semantic node state,
and one named mounted GPUI regression. Move only IconButton's GPUI mounted
cell from `missing` to `mounted`: 39 → 40 mounted, 135 → 134 missing. Keep
known-delta totals at 115 present / 60 not-applicable.
