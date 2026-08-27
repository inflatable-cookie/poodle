# g16.010 — Breadcrumbs Callback Semantics And Mounted Parity

Date: 2026-08-27
Status: complete — PR #84
Branch: `t3code/update-breadcrumb-component`
Card: `docs/roadmaps/g16/010-breadcrumbs-callback-semantics-and-mounted-parity.md`
Source triage: `docs/triage/20260827-112634-post-g16-009-native-lane-decision.md`

## Outcome

Shared Rust Breadcrumbs had reversed the web contract: it attached
`on_navigate` to `href` crumbs and sent the URL, while Svelte and React
activate linkless crumbs and send the authored `value`. The renderer now
matches the web rule. A crumb is callback-interactive only when it is
non-current, linkless, not the ellipsis, and the host supplied `on_navigate`.
Activation calls that handler once with `BreadcrumbItem.value`.

The generated ledger moves two Breadcrumbs cells and no other row. GPUI
mounted behaviour: `missing` → `mounted` (38 → 39 mounted, 136 → 135
missing). Known deltas: `not-applicable` → `present` (114 → 115 present,
61 → 60 not-applicable), because native `href` inertness is a recorded
runtime delta. GPUI accessibility stays `manual`. GPUI visual stays
`missing`. Jetstream stays deferred.

## Repair

- Linkless text, icon-plus-label, and icon-only crumbs are each one button
  target: `NodeRole::Button`, sequential focus, pointer cursor, accessible
  name from `label`, standard focus ring.
- The icon stays decorative. Icon-only presentation keeps the hidden semantic
  label on the same target.
- `href`, current, and ellipsis crumbs do not invoke the callback and do not
  become button focus stops.
- Native `href` crumbs remain inert. The node/backend boundary has no
  URL-routing channel. That is a runtime delta, not a reason to send the URL
  through `on_navigate`.
- The GPUI wrapper exposes `on_navigate`. The Basic specimen shows a compact
  "Navigated to" readout. Host state is the existing `specimens.text` map.

The contract's stale Jetstream note described the reversed URL-driven
behavior. GPUI notes name the callback rule and the `href` delta. A real
`Known Deltas` section records native `href` inertness so the ledger cell
matches the contract.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#breadcrumbs_callback_navigation_through_mounted_pointer_and_keyboard`
proves, through production hit testing, focus, and key dispatch:

- pointer activation of a linkless text crumb emits `home` once
- Enter and Space on a focused icon-only crumb emit `projects` and keep the
  authored accessible name
- current, ellipsis, and `href` crumbs expose no callback activation and no
  sequential focus handle
- Tab from a flanking control visits Home, then Projects, then the control
  after, skipping the inert crumbs

Fixture-local ids are stamped after render. No public instance-id prop was
added. Direct handler calls are used only in focused renderer tests.

## Explicit non-claims

- no native URL routing, generic Link node, router integration, or browser
  anchor behavior
- no Svelte/React public behavior change
- no truncation, icon, separator, current-item, size/density, or wrapping
  redesign
- no GPUI accessibility or visual promotion
- no Jetstream admission
- no IconButton, EditableLabel, NumberInput, TimeInput, Pill, or Breadcrumbs
  consumers
- no other component's ledger row

## Validation

Focused `poodle-render` Breadcrumbs tests (10), Svelte and React Breadcrumbs
tests (20, unchanged), named mounted Breadcrumbs regression.
`effigy regressions:native` (85), `effigy probe:gpui-specimens` (8),
`effigy drift:handlers`, `effigy drift:events`,
`effigy test:parity-evidence-ledger` (5), `effigy check:parity-evidence-ledger`
(175 rows), `effigy ci:native`, `effigy ci:web`, `effigy docs:check`,
`effigy qa`, and `git diff --check origin/main...HEAD`.

`effigy doctor` is already red on the planning base (generated-in-src,
god-files, stale-suppressions). That baseline is unchanged and was not
absorbed.

## Remaining gaps

- native `href` navigation stays a runtime delta until the node/backend owns
  a real link channel
- native accessibility, visual comparison, and Jetstream admission are
  unchanged and unclaimed
- the next evidence lane is an orchestrator checkpoint against 39 mounted /
  135 missing
