# g16.007 — v0.2.2 release certification

Status: **operator gate — exact-candidate window diagnostic accepted; waits
for explicit release approval**
Depends on: `g16.006`
Governing refs: `006-v022-release-candidate.md`,
`../../contracts/001-working-rules.md`

## Outcome

Review the exact `v0.2.2` recovery candidate, run the one explicitly approved
non-activating GPUI window diagnostic, then perform the human-owned tag and
publication operation only after the operator authorises that exact SHA.

This card is never dispatched to a worker.

## Accepted Candidate

- Candidate: `d5607def24c6833913df1b5dcfa06372fcd5dd81`
- Evidence receipt: `2202f0942d56e3aebc9963e18262f367867e081f`
- Integration merge: PR 74 at
  `6ea561be8c45ec7fbdbab4ebeaba4f31284e2596`
- Review record: [PR 74 acceptance](https://github.com/inflatable-cookie/poodle/pull/74#issuecomment-5394209096)

The candidate and headless release evidence are accepted. This does not
authorise the windowed diagnostic, tag, workflow dispatch, publication, or
registry mutation.

## Acceptance

- [x] The candidate receipt pins one clean SHA with green headless release
      evidence and expected artifact digests.
- [x] The operator-reviewed window diagnostic captures all retained Button
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

## Diagnostic Result

The operator authorised the non-activating window diagnostic on 2026-08-24.
It ran from a detached checkout of exact candidate `d5607def` and produced 54
verified captures across the retained 18-fixture Button inventory, with zero
repeat mismatches. All 18 GPUI receipts named
`macos-window-server-nonactivating`, proved T3 Code remained foreground, and
recorded no foreground change.

All 18 Svelte-to-React comparisons passed every channel exactly. The GPUI
comparisons passed dimensions, geometry, and pixels; 16 reported only the
already-contracted `gpui-omits-box-shadow` role finding. The diagnostic exits
non-zero on those classified findings by design, so the result is recorded as
reviewed evidence rather than a green release gate. The visual contact sheet
showed no additional discrepancy.

No tag, workflow dispatch, publication, or registry mutation occurred. The
remaining card work requires a new explicit operator instruction naming the
`v0.2.2` release action.
