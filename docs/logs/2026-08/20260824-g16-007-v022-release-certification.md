# g16.007 — v0.2.2 Release Certification

Date: 2026-08-24
Card: `../../roadmaps/g16/007-v022-release-certification.md`
Candidate: `d5607def24c6833913df1b5dcfa06372fcd5dd81`
Candidate merge: PR 74 at
`6ea561be8c45ec7fbdbab4ebeaba4f31284e2596`

## Authority

The operator explicitly authorised the non-activating window diagnostic after
the candidate was accepted. This did not authorise tag creation, workflow
dispatch, publication, registry mutation, or release-workflow editing.

After reviewing the diagnostic, the operator separately authorised tag
`v0.2.2` and publication from exact candidate `d5607def`. The release workflow
was not edited.

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

## Release

Lightweight tag `v0.2.2` was created and pushed at
`d5607def24c6833913df1b5dcfa06372fcd5dd81`. Remote tag verification returned
the same SHA. Existing tags were not moved, deleted, or reused.

[GitHub Actions run `32756610293`](https://github.com/inflatable-cookie/poodle/actions/runs/32756610293)
completed successfully in 30 minutes with the exact candidate as its head.

| Workflow step | Result |
| --- | --- |
| complete headless Effigy release gate | pass |
| manifest/tag version agreement | pass |
| tarball pack and content verification | pass |
| npm trusted publication | pass |
| packed artifact upload | pass |

The run retains `packed-tarballs` (523,094 bytes), digest
`sha256:0b83427da8fac0ac068f53bd47759be2716edfd5afe9a7419caa6b555ab96740`.
The configured workflow does not create a GitHub Release, and none exists for
`v0.2.2`.

## Registry And Consumer Proof

| Package | Published | npm `latest` |
| --- | --- | --- |
| `@inflatable-cookie/poodle-core` | `0.2.2` | `0.2.2` |
| `@inflatable-cookie/poodle-svelte` | `0.2.2` | `0.2.2` |
| `@inflatable-cookie/poodle-react` | no | absent; source-only by policy |

Svelte retains peer range `>=5.38.6 <6`. A new temporary npm project installed
exact core and Svelte 0.2.2 plus Svelte 5.38.6 from the public registry. Runtime
imports succeeded for 398 core exports, 111 icon exports, 13 token exports,
and 207 Svelte exports. Representative token CSS, theme CSS, and Svelte types
subpaths resolved from the installed packages.

## Closeout

`g16.007` is complete. Poodle release recovery is closed; the next move is a
fresh Longhorn v0.2.2 adoption card, followed by the bounded Underlay and
Soundcheck Library 0.2.2 updates.
