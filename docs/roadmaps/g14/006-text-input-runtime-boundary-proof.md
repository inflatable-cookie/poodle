# g14.006 — TextInput Runtime-boundary Proof

Status: planned
Depends on: `g14.005`

## Outcome

Prove text editing where DOM, GPUI, and Jetstream mechanisms legitimately
differ. Expose missing Jetstream behaviour as incomplete rather than an
accepted declaration.

## Scope

- Move TextInput portable interface and specimen structure into shared
  authority; keep web-only attributes and imperative methods explicit.
- Cover controlled value, typing, selection, validation, clear, focus, submit,
  disabled/read-only, adornment regions, and event ordering.
- Exercise composition/IME capability where the runtime supports it and keep
  missing required support red.
- Compare semantic value/state, events, focus, accessibility, token roles,
  adornment structure, and geometry.

## Acceptance

- Real text entry and editing paths execute in all required runtimes.
- Platform-only props do not leak into the portable Rust spec.
- Declaring Jetstream absence documents the gap but does not green completion.
- A planted dropped edit, selection, IME, or event-order error fails.
- No TextInput-specific generic runner logic; full cost recorded.

## Stop Conditions

- Passing requires pretending text entry happened from a synthetic state
  update.
- The portable interface absorbs DOM/native editor objects.

## Validation

Run TextInput cases, IME/editing suites, conformance gates, `docs:check`, and
`git diff --check`.
