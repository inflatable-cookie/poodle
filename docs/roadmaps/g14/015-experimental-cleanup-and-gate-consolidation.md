# g14.015 — Experimental Cleanup And Gate Consolidation

Status: blocked pending `g14.014`
Depends on: `g14.014`

## Outcome

Remove superseded experiments, stale methodologies, duplicate authorities,
orphaned generated artifacts, and gates whose claim is now owned elsewhere.

## Scope

- Execute every remaining disposition in `conformance-estate.md`.
- Remove retired component/scene IR, machine-interface, vector, capability,
  specimen, snapshot, and audit surfaces that no longer own a distinct claim.
- Move required generated output out of hand-edited source roots where
  practical; close `scan.generated-in-src` findings.
- Split or simplify conformance god files; close owned `scan.god-files`
  findings.
- Keep historical logs and Git evidence; active docs describe only the live
  system.

## Acceptance

- One authority and one canonical gate per portable claim.
- No active task selector points at retired machinery.
- `effigy doctor`, docs/CI boards, and orphan checks are clean or every
  remaining exception has a non-g14 owner and reason.
- Deleted material and recovery path are recorded.
