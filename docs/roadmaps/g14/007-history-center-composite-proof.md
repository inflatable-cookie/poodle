# g14.007 — HistoryCenter Composite Proof

Status: planned
Depends on: `g14.006`

## Outcome

Prove the same pipeline scales to a host-coordinated interactive composite.
Deliver working shared Rust composition and GPUI interaction rather than a
specimen-only registration.

## Scope

- Move HistoryCenter portable interface and specimen structure into shared
  authority without importing Longhorn or product state.
- Use structured fixture data and named host commands for selection, checkout,
  rename, open/close, and branch navigation.
- Cover empty, linear, multiple-fork, nested-fork, depth-cap, current-entry,
  loading/disabled, and narrow-layout cases.
- Observe identified rows/branches, hierarchy, selection, focus/navigation,
  command payload/order, token roles, accessibility, scroll, and geometry.
- Build missing native composition and backend interaction on shared
  `poodle-render` primitives.

## Acceptance

- All fixture entries appear exactly once with stable semantic identity.
- Svelte, React, and GPUI execute the same command scenarios.
- Native components are functional, accessible, and specimen-visible.
- A planted hierarchy, payload, focus, scroll, or inert-handler error fails.
- Composite needs extensions, not a second conformance architecture.
- Full cost and reusable vocabulary delta are recorded.

## Stop Conditions

- Cases need Longhorn types, persistence, routing, or product callbacks.
- Generic runners need HistoryCenter tree or command logic.
- Native parity requires GPUI-only composition instead of shared
  `poodle-render` composition.

## Validation

Run HistoryCenter core/component/native suites and all conformance gates,
`docs:check`, and `git diff --check`.
