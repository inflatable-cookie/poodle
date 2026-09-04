# g16.104 — Release Workflow Checkout Base Ref And Pre-Tag Dry Run

Status: complete — awaiting orchestrator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/104-release-workflow-checkout-base-ref.md`
Dispatch: `docs/roadmaps/dispatch.md` revision 7
Governing refs: `.github/workflows/release.yml`,
`.github/workflows/ci-web.yml:32-41`, `scripts/check-release-automation.ts`,
`../../../AGENTS.md`
Branch: `fix/g16-104-release-workflow-checkout-base-ref`
Worktree: `/Users/tom/.t3/worktrees/poodle/g16-104-release-workflow-checkout-base-ref`
Base: `origin/main` at `02bb74ff2e3f0de1580e569e84ce63dd6ee38a59`
Planning base at dispatch: `7f6fd9a1c03ed64ba99771832d28e8648fba379e` (ancestor
of the current tip; the extra commit is `docs(g16.097): record retracted
failed dry run`)
Worker PR: pending

## Outcome

`release.yml` now checks out with `fetch-depth: 0` and fetches
`origin/main` the same way `ci-web.yml` does, so `effigy release gates` can
resolve `git merge-base HEAD origin/main` on a tag ref. A branch dry-run is
allowed: `Require a versioned release tag` fails only when `dry-run` is
false, `Versions agree with the tag` runs only on tag refs, and a branch
dry-run checks that manifests agree with each other. Publish still requires
a versioned tag ref and `dry-run=false`. Header comment records the
two-step protocol: prove the candidate green with
`gh workflow run release.yml --ref <candidate-sha or main> -f dry-run=true`,
then tag, then publish.

`scripts/check-release-automation.ts` requires the checkout fetch, the
dry-run exception on the tag-require step, tag-only version agreement,
branch lockstep, and the unchanged Publish conjunction. Plants: omit the
`origin/main` fetch; drop the dry-run exception from tag-require; run
versions-agree-with-tag on every ref; drop the dry-run guard from Publish;
drop the tag-ref guard from Publish. Each plant fails.

No tag, workflow dispatch, publish, `web-preview.ts` edit, sibling,
windowed, or native-visual change. Second sighting of the classifier's raw
exit 128 is appended to `PAPERCUTS.md`.

## Review oracle

| Invariant | Plant / probe | Result |
| --- | --- | --- |
| Base ref exists at a tag ref | omit `git fetch --no-tags origin main:refs/remotes/origin/main` | plant fails with `release must fetch origin/main`; live dispatch belongs to `g16.097` |
| Candidate is provable before tagging | drop `&& !inputs.dry-run` from tag-require | plant fails with `must fail only when dry-run is false`; live branch dry-run belongs to `g16.097` step 1b |
| Branch ref can never publish | dispatch `dry-run=false` against a branch is still the tag-require failure; Publish plants drop each conjunct | tag-require still fails when dry-run is false on a non-tag; plant publish without dry-run guard fails; plant publish without tag guard fails |
| Versions agree with the tag only on tag refs | remove that step's `if:` | plant fails with `must run only on tag refs` |
| Publish guard unchanged | `git diff origin/main -- .github/workflows/release.yml` around Publish | Publish `if:` remains `startsWith(github.ref, 'refs/tags/v') && !inputs.dry-run` |
| Checker agrees | `effigy check:release-automation` | pass, including the plants above |
| Ordinary PR board unaffected | this card's own PR | expected `ci-web` workflow-scope rejection per the g16.096 exception; `ci-rust` must be green |

## Validation

- `effigy check:release-automation`: pass. Plants:
  `plant omit compiled receipt: failed as required`
  `plant restore package/src: failed as required`
  `plant omit origin/main fetch: failed as required`
  `plant tag-require without dry-run exception: failed as required`
  `plant versions-agree-with-tag on every ref: failed as required`
  `plant publish without dry-run guard: failed as required`
  `plant publish without tag guard: failed as required`
- `effigy docs:check`: pass.
- `git diff --check` on the owned paths: pass.
- No `release prepare/execute`, tag, publish, or workflow dispatch.

PR-head `ci-web` is expected to fail only on the documented installed-package
scope guard for this authorized `.github/workflows/release.yml` mutation.
`ci-rust` must be green. The executable branch dry-run on the candidate SHA
belongs to `g16.097` after this merges.

## Closeout

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.
