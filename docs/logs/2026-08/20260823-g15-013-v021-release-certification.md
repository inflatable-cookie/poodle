# g15.013 — v0.2.1 Release Certification

Date: 2026-08-23
Verdict: **released; g15 complete**
Candidate: `3d914261c621ef1184d42d7182f7530586c8b267`
Tag: `v0.2.1`

## Release

The operator reviewed the `g15.054` recovery receipt and explicitly authorised
the exact replacement tag and publication mutation. Lightweight tag `v0.2.1`
was created and pushed at the pinned candidate. Immutable `v0.2.0` remains at
`7922a3a951e94b607566563ff2750fe825ad7b0d` and was not moved, removed, reused,
or rerun.

[GitHub Actions run `32658293188`](https://github.com/inflatable-cookie/poodle/actions/runs/32658293188)
completed successfully in 20m11s. It used head SHA
`3d914261c621ef1184d42d7182f7530586c8b267` and tag ref `v0.2.1`.

| Workflow step | Result |
| --- | --- |
| isolated npm `12.0.2` bootstrap | pass |
| dependency installation | pass |
| complete headless Effigy release gate | pass |
| manifest/tag version agreement | pass |
| tarball pack and content verification | pass |
| npm trusted publication | pass |
| packed artifact upload | pass |

The run retains the `packed-tarballs` artifact (522,969 compressed bytes).
The workflow does not create a GitHub Release, and none exists for `v0.2.1`;
the Git tag, successful workflow run, npm registry entries, and retained
artifact are the configured release surfaces.

No local windowed, native-visual, or Jetstream selector ran during release.

## Registry Result

| Package | Published | npm `latest` | Repository metadata |
| --- | --- | --- | --- |
| `@inflatable-cookie/poodle-core` | `0.2.1` | `0.2.1` | `packages/core` |
| `@inflatable-cookie/poodle-svelte` | `0.2.1` | `0.2.1` | `packages/svelte/components` |
| `@inflatable-cookie/poodle-react` | no | absent | source-only by policy |

Core published at `2026-08-23T18:51:06.633Z`; Svelte published at
`2026-08-23T18:51:11.943Z`. The Svelte peer range remains
`svelte >=5.38.6 <6`.

## Consumer Proof

A new temporary npm project installed exact core and Svelte `0.2.1` with
Svelte `5.38.6` from the public registry. npm resolved all three exact versions.
Runtime imports succeeded for the core and Svelte package roots: 398 core
exports, 111 icon exports, 13 token exports, and 207 Svelte exports.
Representative icon, token CSS, theme CSS, and Svelte types subpaths all
resolved from the installed packages.

## Triage Disposition

- `transitions.dev` motion learning stays explicitly open for a later bounded
  research and promotion pass.
- The Longhorn-backed conformance lab stays explicitly open for a post-release
  ownership/process decision; the Button comparator prerequisite is complete.
- Native presentation explicitness is already closed and promoted into
  architecture 010 and `g15.043`.

## Closeout

`g15.013` is complete. Every g15 implementation and release card is complete,
and generation g15 is closed. No new generation is active. The next move is an
operator-led planning checkpoint, not another release mutation.
