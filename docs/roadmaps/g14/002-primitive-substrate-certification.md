# g14.002 — Primitive Substrate Certification

Status: planned
Depends on: `g14.001`

## Outcome

Certify the shared shapes that components assume. Prove `poodle-render` emits
renderer-neutral `poodle-node` primitives and intents and GPUI interprets them
with equivalent semantic observations; prove web observations use matching
token and layout roles.

## Scope

- Freeze the primitive roster: layout/sizing/clipping/scroll, surface/border/
  radius/shadow, text/icon, control state, focus/keyboard/pointer,
  accessibility, overlay/dismissal, and text-editing boundary.
- Add primitive conformance cases and observation probes using the g14.001
  kernel. No component-specific workaround.
- Repair the GPUI preview/capture workflow: stale baselines and the
  `--control-size`/`--size` mismatch.
- Emit a capability matrix with executed evidence for the active cohort.
  Missing rows fail; declared absence stays incomplete. Record Jetstream as a
  deferred backend admission, not a capability-by-capability waiver.
- Route existing capability declarations into the matrix or retire them.

## Acceptance

- Both native backends execute each required primitive case.
- Planting a backend-only style, semantic, focus, or event error fails the
  matching observation.
- Snapshot refresh is explicit, reviewable, and cannot overwrite the only
  evidence silently.
- Every later component profile can name certified primitives rather than add
  bespoke backend tolerances.
- Full mechanism cost and retained/replaced tooling are recorded.

## Stop Conditions

- Certification requires a universal web/native render tree.
- A backend cannot expose enough observation to distinguish implemented from
  inert.
- A component workaround is proposed instead of repairing a shared primitive.

## Validation

Run the primitive cases, both native suites, relevant web observation tests,
`docs:check`, and `git diff --check`. Record the certified matrix in the PR.
