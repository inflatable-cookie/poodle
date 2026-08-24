# g16.007 v0.2.2 Release Certification — Window Diagnostic

Date: 2026-08-24
Card: `../../roadmaps/g16/007-v022-release-certification.md`
Candidate: `d5607def24c6833913df1b5dcfa06372fcd5dd81`
Candidate merge: PR 74 at
`6ea561be8c45ec7fbdbab4ebeaba4f31284e2596`

## Authority

The operator explicitly authorised the non-activating window diagnostic after
the candidate was accepted. This did not authorise tag creation, workflow
dispatch, publication, registry mutation, or release-workflow editing.

The diagnostic ran from a clean detached worktree at the exact candidate SHA:

```sh
effigy test:visual-button-comparison-windowed
```

## Result

- Comparator integrity tests: 35 passed, 0 failed.
- Inventory: 18 Button fixtures, 3 runtimes, 54 retained captures.
- Repeat evidence: 0 mismatches across two captures per runtime and fixture.
- Comparisons: 36 total.
- Svelte to React: every dimensions, geometry, roles, and pixels channel
  passed exactly for all 18 fixtures.
- Svelte to GPUI: every dimensions, geometry, and pixels channel passed.
- Classified GPUI findings: 16 instances of the existing
  `gpui-omits-box-shadow` role delta, with no unclassified finding.
- Visual review: no additional discrepancy in the generated contact sheet.

The selector exited 1 because its fixed policy deliberately keeps a classified
known delta blocking. This is diagnostic evidence, not a waived release gate;
the headless release board was already accepted under `g16.006`.

Three Svelte navigations used the runner's bounded recovery path after a
60-second selector timeout. Every retained capture and repeat still verified.
The avoidable delay is recorded in `PAPERCUTS.md`.

## Foreground Proof

All 18 retained GPUI receipts reported:

- transport: `macos-window-server-nonactivating`;
- foreground baseline: `com.t3tools.t3code`;
- foreground verdict: `proved`;
- foreground changes: 0;
- samples per receipt: 22–340.

The GPUI batch therefore did not take focus from T3 Code.

## Disposable Evidence

The generated evidence stayed outside the tracked tree:

- `summary.json` SHA-256:
  `b89c1217f38fbb882d3cbe8b97be4ab1d188f3afab1e988735fdea9c490f5f79`;
- `contact-sheet.html` SHA-256:
  `142effc255372d68dd5f08535ef09802499dbc3030ff75a176b3b7ea6bc652e4`.

No baseline was updated and no diagnostic artifact is release input.

## Continuation

The window diagnostic is accepted. `g16.007` remains at the operator gate.
The next action is a separate explicit instruction to create tag `v0.2.2` and
run the repository's publication path from the accepted exact candidate.
