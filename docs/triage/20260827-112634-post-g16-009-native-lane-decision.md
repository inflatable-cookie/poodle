# Post-g16.009 Native Lane Decision

Status: resolved — closed by `g16.010`; see
`../logs/2026-08/20260827-g16-010-breadcrumbs-callback-semantics-and-mounted-parity.md`
Captured: 2026-08-27
Source: orchestrator evidence checkpoint after PR #83

## Finding

The next mounted-evidence lane should close an observable contract defect. It
must not promote display-only components merely because their ledger cells are
empty.

### Breadcrumbs — selected

Breadcrumbs has one bounded callback seam with a clear web authority:

- Svelte and React render a non-current item with `href` as an anchor;
- a non-current, linkless item is a button and `onNavigate` receives its
  authored `value`;
- current items and the synthetic ellipsis are inert.

The shared Rust renderer currently does the opposite. It attaches
`on_navigate` only when an item has `href` and sends the URL. Linkless items
cannot navigate at all. The GPUI compatibility wrapper exposes no
`on_navigate` builder, so the specimen cannot demonstrate the callback.
`docs/contracts/components/breadcrumbs.md` also retains a stale Rust note that
describes the incorrect URL-driven behavior despite its strict-parity
checklist naming the web rule.

This is a contract implementation defect, not a new design decision. Repair
the shared renderer, make linkless crumbs native button-like focus stops with
the standard focus ring and accessible name, wire the GPUI specimen, and prove
pointer plus keyboard activation through the mounted backend. Native `href`
navigation stays outside this lane because the node/backend boundary has no
URL-routing channel.

### Deferred candidates

- **IconButton:** high leverage, but honest closure includes
  `onPressedChange` ownership and the built-in tooltip lifecycle. That needs a
  separate API/overlay decision.
- **EditableLabel:** Rust still conflates committed value and draft and cannot
  produce the contract's `{ value, previousValue }` commit payload. Focus
  restoration and select-on-focus also need a dedicated design.
- **NumberInput:** its typed value plus raw-draft decision remains open in
  `20260826-213343-number-input-native-value-model.md`.
- **TimeInput:** GPUI is currently an unconstrained text field; min, max, step,
  and segment editing need a real native time-entry design.
- **Pill:** display-only. Adding a mounted behavior test would prove no new
  behavior and inflate the ledger.

## Decision

Promote Breadcrumbs into `g16.010`. Move exactly its GPUI mounted-behavior
cell from `missing` to `mounted`, taking the ledger from 38 to 39 mounted and
136 to 135 missing. Do not add native URL routing, a generic link abstraction,
or broader accessibility/visual claims.
