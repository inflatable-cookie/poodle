# g16.006 — v0.2.2 release certification

Status: **blocked — waits for an accepted `g16.005` candidate and explicit
operator approval**
Depends on: `g16.005`
Governing refs: `005-gpui-cratesio-recovery-candidate.md`,
`../../contracts/001-working-rules.md`

## Outcome

Review the exact `v0.2.2` recovery candidate, run the one explicitly approved
non-activating GPUI window diagnostic, then perform the human-owned tag and
publication operation only after the operator authorises that exact SHA.

This card is never dispatched to a worker.

## Acceptance

- [ ] The candidate receipt pins one clean SHA with green headless release
      evidence and expected artifact digests.
- [ ] The operator-reviewed window diagnostic captures all retained Button
      fixtures without changing the foreground application.
- [ ] The operator explicitly authorises tag `v0.2.2` and publication from
      the reviewed SHA.
- [ ] The release workflow publishes core and Svelte 0.2.2, retains React as
      source-only, and uploads the expected artifacts.
- [ ] Registry metadata, clean consumer installation, and the immutable Git
      tag are verified after the run.

## Stop Conditions

- Do not move or reuse `v0.2.0` or `v0.2.1`.
- Do not tag a different SHA from the reviewed candidate.
- Do not waive a red gate, focus-taking diagnostic, dependency-source defect,
  or package-install failure.
- Do not mutate a release workflow without separate explicit operator
  approval.

