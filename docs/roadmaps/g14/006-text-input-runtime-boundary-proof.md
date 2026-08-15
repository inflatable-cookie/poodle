# g14.006 — TextInput Runtime-boundary Proof

Status: complete — accepted in PR #20
Depends on: `g14.005` — accepted in PR #18

## Outcome

Prove text editing where DOM and GPUI mechanisms legitimately differ. Keep the
portable action, capability, and observation boundary neutral for later
Jetstream admission.

## Scope

- Move TextInput portable interface and executable cases into shared
  authority; keep web-only attributes and imperative methods explicit.
  Preserve the curated catalogue specimen.
- Cover controlled value, typing, selection, validation, clear, focus, submit,
  disabled/read-only, adornment regions, and event ordering.
- Exercise composition/IME capability where the runtime supports it and keep
  missing required support red.
- Compare semantic value/state, events, focus, accessibility, token roles,
  adornment structure, and geometry.

## Acceptance

- Real text entry and editing paths execute in all active runtimes.
- Platform-only props do not leak into the portable Rust spec.
- Jetstream remains program-deferred and cannot appear as passing or
  `not-applicable` in active-cohort evidence.
- A planted dropped edit, selection, IME, or event-order error fails.
- No TextInput-specific generic runner logic; full cost recorded.

## Stop Conditions

- Passing requires pretending text entry happened from a synthetic state
  update.
- The portable interface absorbs DOM/native editor objects.

## Validation

Run TextInput cases, IME/editing suites, `effigy ci:conformance` (headless),
`docs:check`, and `git diff --check`. No foreground conformance selector is
permitted.
