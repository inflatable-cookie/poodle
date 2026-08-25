# g15.061 — v0.2.2 release certification

Status: **complete — `v0.2.2` published from the accepted exact candidate**
Depends on: `g15.060`
Governing refs: `060-v022-release-candidate.md`,
`../../contracts/001-working-rules.md`

## Outcome

Review the exact `v0.2.2` recovery candidate, run the one explicitly approved
non-activating GPUI window diagnostic, then perform and certify the human-owned
tag and publication operation from the operator-authorised exact SHA.

This card is never dispatched to a worker.

## Accepted Candidate

- Candidate: `d5607def24c6833913df1b5dcfa06372fcd5dd81`
- Evidence receipt: `2202f0942d56e3aebc9963e18262f367867e081f`
- Integration merge: PR 74 at
  `6ea561be8c45ec7fbdbab4ebeaba4f31284e2596`
- Review record: [PR 74 acceptance](https://github.com/inflatable-cookie/poodle/pull/74#issuecomment-5394209096)
- Release tag: `v0.2.2` at the accepted candidate
- Publication run: [GitHub Actions `32756610293`](https://github.com/inflatable-cookie/poodle/actions/runs/32756610293)

The operator accepted the candidate and separately authorised the diagnostic,
tag, workflow dispatch, and publication. No release workflow was edited.

## Acceptance

- [x] The candidate receipt pins one clean SHA with green headless release
      evidence and expected artifact digests.
- [x] The operator-reviewed window diagnostic captures all retained Button
      fixtures without changing the foreground application.
- [x] The operator explicitly authorises tag `v0.2.2` and publication from
      the reviewed SHA.
- [x] The release workflow publishes core and Svelte 0.2.2, retains React as
      source-only, and uploads the expected artifacts.
- [x] Registry metadata, clean consumer installation, and the immutable Git
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

No tag, workflow dispatch, publication, or registry mutation occurred during
the diagnostic. The later release action used that accepted evidence.

## Release Result

Lightweight tag `v0.2.2` points to exact candidate
`d5607def24c6833913df1b5dcfa06372fcd5dd81` locally and remotely. The
operator-authorised release workflow completed successfully in 30 minutes at
run `32756610293`, with the same head SHA. Release gates, manifest/tag version
agreement, tarball verification, trusted npm publication, and artifact upload
all passed.

The public registry serves `@inflatable-cookie/poodle-core@0.2.2` and
`@inflatable-cookie/poodle-svelte@0.2.2` as `latest`. React remains source-only
and has no npm package. The retained `packed-tarballs` artifact is 523,094
bytes with digest
`sha256:0b83427da8fac0ac068f53bd47759be2716edfd5afe9a7419caa6b555ab96740`.
The configured workflow does not create a GitHub Release.

A new temporary consumer installed exact core and Svelte 0.2.2 with Svelte
5.38.6 from the public registry. Both package roots and representative icon,
token CSS, theme CSS, and Svelte types subpaths resolved successfully.
