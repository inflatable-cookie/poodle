# 003 — Background-safe non-activation proof

Status: complete
Owner: Poodle core
Created: 2026-09-08
Depends on: none
Governing refs: `../../contracts/001-working-rules.md`,
`../../evidence/nucleus/README.md`

## Outcome

Native capture proves that its own process never becomes foreground while the
operator remains free to use other applications.

## Shipped result

- Foreground receipts record process identity and PID.
- Unrelated operator transitions remain admissible and auditable.
- Capturer baseline/self-frontmost, unreadable samples, failed reads, too few
  samples, focus, key-window, and application-active evidence fail closed.
- Every capture mode inherits the shared transport proof.

## Evidence

PR #230 merged as `583aa173935dd66ea0d8bd17196115f5b211a01b` after
independent review of head `62be0575a51919697f78dbeaf9789339d83ae2f6`.
Poodle Lab adopted the contract and completed a 174-capture background-safe run.

## Limits and continuation

This task changed evidence causality, not component pixels or semantics. macOS
still requires an unlocked window server and Screen Recording permission; the
operator does not need to remain hands-off.
