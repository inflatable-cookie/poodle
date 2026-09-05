# g16.103 — Release Tarball Dist Verification Repair

Status: complete — merged in PR #208 at `eab436eef`; unblocked `g16.097`
Type: workflow automation — explicit operator approval recorded
Opened: 2026-09-04
Depends on: none
Blocks: `g16.097` v0.3.0 release certification
Governing refs: `../../architecture/014-compiled-web-package-distribution.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`.github/workflows/release.yml`, `scripts/check-release-automation.ts`,
`../../../AGENTS.md`
Operator decision: 2026-09-04 — "Authorize repair": urgently migrate the
release tarball verifier from stale source paths to the canonical compiled
package boundary, then re-certify v0.3.0 from the repaired `main` tip
Dispatch manifest: `../dispatch.md`

## Goal

Make the release workflow verify the package Poodle actually publishes. The
core package moved to a source-free, receipt-backed `dist/**` contract in
`g16.057`, but `release.yml` still checks the pre-migration `package/src/**`
layout. The stale verifier would reject a correct dry run after every local
release gate has passed.

## Failure Evidence

At candidate `b4158a1b68db9292c17be1d8c219f0fc26512a0b`:

- frozen dependency installation resolved `lucide-static` checkout-locally to
  the locked `1.31.0`; manifests and the tracked tree stayed clean;
- `env -u POODLE_WEB_PACK_INSTALL_SCOPE_MODE effigy release gates --json`
  passed with no blockers in 685816 ms;
- ordinary installed-package certification passed 11 files / 22 tests against
  the exact candidate;
- local archives contained `package/dist/**` and package metadata, but no
  `package/src/**`, matching `packages/core/package.json`;
- the core archive was 411238 bytes with SHA-256
  `3d46bc39d6247af10f5052e6060afdb42c07220b1633d16b6029b018e0d58719`;
  Svelte was 518811 bytes with SHA-256
  `3ee935c56dd5a98859173bc22d4109da324707e1cb59019726803d162c5917c5`;
  private React was 291636 bytes with SHA-256
  `bcbebd16db34c54511d95f3dd2815af9ad5cc9ceb8b14e7f4b2f95447a47106e`;
- no local or remote `v0.3.0` tag existed and no release workflow or publish
  mutation ran.

## Fixed Boundary

- Edit `.github/workflows/release.yml` only inside the `Pack and verify
  contents` commentary and assertions. Keep the workflow manual, tag-targeted,
  dry-run by default, and Linux-only. Keep the publish condition, package set,
  OIDC permissions, pinned actions/tooling, and artifact upload unchanged.
- Replace the three stale core `package/src/**` requirements with compiled
  archive requirements:
  - `package/dist/.poodle-build.json` exactly once for every packed package;
  - at least 50 `package/dist/icons/icons/*.js` runtime icon modules;
  - at least 50 matching `package/dist/icons/icons/*.d.ts` declarations;
  - `package/dist/icons/aliases.generated.d.ts` exactly once;
  - at least 20 `package/dist/tokens/generated/css/*.css` token stylesheets.
- Keep licence, README, and manifest membership checks.
- Do not add `src/**` to any package allowlist. Do not change exports, package
  contents, generated outputs, versions, dependencies, or public API.
- Extend `scripts/check-release-automation.ts` so the canonical compiled
  patterns and receipt assertion are required and the stale source patterns
  are forbidden. Plant each direction: removing a required compiled assertion
  fails, and restoring a stale source assertion fails.
- Record implementation and validation in one execution log under
  `docs/logs/2026-09/`. `PAPERCUTS.md` may receive one append-only note if the
  worker finds additional small execution friction.
- No tag creation, workflow dispatch, registry mutation, publish, sibling
  change, native-visual selector, or windowed selector.

## Validation

- `effigy check:release-automation`, including the two planted failures.
- Local core and Svelte `npm pack` listings prove the required archive members
  and reject every stale `package/src/**` expectation.
- `git diff --check origin/main...HEAD`.
- The PR's `ci-rust` check must be green. Its `ci-web` check is expected to fail
  only when the ordinary installed-package scope guard rejects the authorized
  `.github/workflows/release.yml` mutation. That policy failure is accepted for
  this PR only; any earlier or additional failure is blocking.
- After merge, require the automatic `ci-web` and `ci-rust` push-to-`main` runs
  to pass. The green `main` run is the full headless proof because the ordinary
  scope diff is then empty.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Verifier matches compiled package | require any `package/src/**` member | static checker plant fails; real local archive passes |
| Runtime and declarations survive | omit either icon `.js` or `.d.ts` floor | static checker and archive listing fail |
| Receipt survives npm packing | omit `package/dist/.poodle-build.json` | static checker and archive listing fail |
| Token CSS survives | use the old source CSS path | stale-pattern plant fails; compiled archive has at least 20 files |
| Release controls stay fixed | alter trigger, dry-run, publish guard, OIDC, package set, or upload | existing release-automation assertions fail; semantic diff shows no change |
| No release mutation | create a tag or dispatch the workflow | local/remote tag absence and no new release run |

## Owned Paths

`.github/workflows/release.yml` (`Pack and verify contents` block only),
`scripts/check-release-automation.ts`, one execution log under
`docs/logs/2026-09/`, and root `PAPERCUTS.md` append-only.

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`, and the
`g16.097` candidate-state update.

## Stop Conditions

Stop and return to Chatterbox when the repair would change package contents or
exports; weaken a release control; touch another workflow block; require a tag,
dispatch, or publish to validate; expose any source file in the archive; or
produce a PR failure beyond the documented workflow-scope rejection.
Escalation owner: Chatterbox.

## Continuation

After accepted exact-head review and merge, the coordinator records the merge
SHA, requires green push-to-`main` web and Rust boards, and restarts `g16.097`
from a fresh detached checkout of that new `main` tip. The previously certified
`b4158a1b` tree remains evidence, not a taggable candidate.

