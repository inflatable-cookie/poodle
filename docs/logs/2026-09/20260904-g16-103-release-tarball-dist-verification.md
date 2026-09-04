# g16.103 — Release Tarball Dist Verification Repair

Status: complete — awaiting orchestrator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/103-release-tarball-dist-verification.md`
Dispatch: `docs/roadmaps/dispatch.md` revision 7
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/specs/070-compiled-web-distribution-contract.md`,
`docs/specs/022-packaging-versioning-and-release-channel-rules.md`,
`.github/workflows/release.yml`, `scripts/check-release-automation.ts`
Branch: `fix/g16-103-release-tarball-dist-verification`
Worktree: `/Users/tom/.t3/worktrees/poodle/g16-103-release-tarball-dist-verification`
Base: `origin/main` at `c28b2dbf650430c67d1b1aaeaa856ef223638931`

## Outcome

The release pack verifier now checks the compiled package Poodle publishes.
`Pack and verify contents` requires `package/dist/.poodle-build.json` on every
packed tarball, core icon JS/DTS floors, compiled aliases, and token CSS.
Stale `package/src/**` assertions are gone. Licence, README, and manifest
checks stay. Trigger, dry-run, tag gate, publish guard, OIDC, package set,
pinned tooling, and artifact upload are unchanged.

`scripts/check-release-automation.ts` requires those compiled patterns, forbids
`package/src/` in the pack block, and plants both directions in-process:
omitting the receipt assertion fails, restoring a stale source assertion fails.

No tag, workflow dispatch, publish, export, package-content, sibling, windowed,
or native-visual change.

## Review oracle

| Invariant | Plant / probe | Result |
| --- | --- | --- |
| Verifier matches compiled package | restore `package/src/**` in the pack block; list real archives | plant fails with `stale package/src members`; core and Svelte archives have 0 `package/src/` members |
| Runtime and declarations survive | omit icon `.js` or `.d.ts` from a core listing | workflow `grep -c` floors fail (0 vs 50); real archive has 108 / 108 |
| Receipt survives npm packing | omit `package/dist/.poodle-build.json` from the checker and from a listing | checker plant fails; listing plant count is 0; both real archives have the receipt once |
| Token CSS survives | restore a `package/src/` assertion; list compiled CSS | stale plant fails; core archive has 22 token CSS files |
| Release controls stay fixed | `git diff origin/main -- .github/workflows/release.yml` | only Pack-and-verify commentary and assertions change |
| No release mutation | local/remote `v0.3.0` tags; `gh run list --workflow=release.yml` | no `v0.3.0` tag; this lane dispatched no release run |

## Validation

- `effigy check:release-automation`: pass. Plants:
  `plant omit compiled receipt: failed as required`
  `plant restore package/src: failed as required`
- Local `npm pack` of core and Svelte, then the workflow's `grep -c` helper:

  | Archive | Bytes | Entries | Receipt | Icon JS | Icon DTS | Aliases | Token CSS | `package/src/` |
  | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
  | `inflatable-cookie-poodle-core-0.3.0.tgz` | 411237 | 522 | 1 | 108 | 108 | 1 | 22 | 0 |
  | `inflatable-cookie-poodle-svelte-0.3.0.tgz` | 518812 | 653 | 1 | n/a | n/a | n/a | n/a | 0 |

  SHA-256: core
  `90039ff22df7cd8c6105dc80d524b065c06e8d1a4aef8e62a7988e15b22d01a3`;
  Svelte
  `da47724507b69032ee4310ce3e35813463c8b11c796c48369bd1f7302da2712b`.
- Listing plants: omitting receipt / icon JS / icon DTS / token CSS each drops
  the matching `grep -c` to 0.
- `git diff --check origin/main...HEAD`: pass after commit.
- No `release prepare/execute`, tag, publish, or workflow dispatch.

PR-head `ci-web` is expected to fail only on the documented installed-package
scope guard for this authorized `.github/workflows/release.yml` mutation.
`ci-rust` must be green. Post-merge push-to-`main` boards belong to the
coordinator.

## Closeout

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`, and the
`g16.097` candidate-state update.
