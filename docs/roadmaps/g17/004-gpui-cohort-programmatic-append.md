# 004 — GPUI cohort programmatic append replay

Status: complete
Owner: Poodle core
Created: 2026-09-08
Depends on: `g17.003`
Governing refs: `003-background-safe-nonactivation-proof.md`,
`../../evidence/nucleus/README.md`

## Outcome

The GPUI cohort host consumes the closed `programmatic_append` scenario action
and appends the declared AgentTranscript item exactly once before after-actions
capture.

## Shipped result

- Closed parser variant and host-owned transcript state.
- Shared item validation/mapping for renderer and replay.
- Wrong component, malformed item, unknown kind/action/field, and partial replay
  fail before publication.
- Existing pointer/key replay and initial state remain unchanged.

## Evidence

PR #231 merged as `8bd95d3a2cdf8c86edacb450cc33a0a4d02b9983` after
independent review of head `acfb0bdef1e17c5a21cd4fdc705c6f123ec8b955`.
The corrected Lab bundle has distinct initial/after-actions hashes on both web
hosts and completed all 174 captures.

## Limits and continuation

No public component API or comparison policy changed. The result is consumed by
ready task `g17.001`.
