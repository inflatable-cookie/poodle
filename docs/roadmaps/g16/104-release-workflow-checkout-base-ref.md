# g16.104 — Release Workflow Checkout Base Ref And Pre-Tag Dry Run

Status: ready — operator approved 2026-09-04: (1) edit `release.yml`
(workflow authority), (2) retract the `v0.3.0` tag at `eab436eef` and re-tag
after this fix merges. Urgent serial predecessor of `g16.097`
Type: workflow automation — release workflow
Opened: 2026-09-04
Depends on: none. Blocks: `g16.097` v0.3.0 release certification
Governing refs: `.github/workflows/release.yml`,
`.github/workflows/ci-web.yml:32-41` (the proven checkout shape from
`g16.096`), `test/package-install/web-preview.ts:126,167`,
`scripts/check-release-automation.ts`, `../../../AGENTS.md`
Evidence: dry run `33908714014` at tag `v0.3.0` (`eab436eef`) failed in
`Release gates` at `test:web-pack-install`: `git merge-base eab436eef
origin/main` → `fatal: Not a valid object name origin/main`. `actions/checkout`
at a tag ref fetches only that ref, so the ordinary scope classifier has no
base. Locally the same gate passed because `origin/main` exists.
Dispatch manifest: `../dispatch.md`

## Goal

Make `effigy release gates` pass on the release runner at a tag ref, and
make the release workflow provable green on the exact candidate commit
*before* any tag exists, so a failed release run is a protocol violation
rather than a discovery.

## Protocol Failure This Repairs

Two `v0.3.0` dry runs (`33874116177`, `33908714014`) failed after the tag
was pushed. Both defects lived in the tagged tree's own workflow or board
shape, and neither could be seen by the local `effigy release gates` because
the local checkout had `origin/main` and a built `dist/`. The operator's
rule (2026-09-04): a release run that fails is a process failure; CI must be
green on the candidate before the release process starts. Today the
workflow refuses non-tag refs, so that rule cannot be followed. This card
makes it followable.

## Fixed Boundary

- Edit `release.yml` only: on the `actions/checkout` step set
  `fetch-depth: 0`, and add the same explicit
  `git fetch --no-tags origin main:refs/remotes/origin/main` step that
  `ci-web.yml` uses, with the same header comment explaining why. No other
  change to the workflow: triggers, runner, gates, publish, and artifact
  steps stay byte-identical.
- Allow a dry run on a non-tag ref: change the "Require a versioned release
  tag" step to fail only when `inputs.dry-run` is false, and make the
  "Versions agree with the tag" step run only on tag refs (on a branch dry
  run, verify instead that all manifests agree with each other). The Publish
  step keeps its existing tag-and-not-dry-run condition, so a branch ref can
  never publish. Header comment states the two-step protocol: prove the
  candidate green with `gh workflow run release.yml --ref <candidate-sha
  or main> -f dry-run=true`, then tag, then publish.
- Keep `scripts/check-release-automation.ts` green; extend its assertions to
  cover the new rule "publish requires a tag ref and dry-run=false"; never
  relax the publish guard.
- Do not touch `web-preview.ts`. Append one PAPERCUTS entry: the classifier
  should fail with "origin/main is not available" instead of a raw exit 128,
  and should say which workflow to fix. (Second sighting; first was PR #201.)
- No tag, no dispatch, no publish in this lane. `g16.097` owns those after
  this merges.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Base ref exists at a tag ref | dispatch `release.yml` with `dry-run=true` against the new tag | `Release gates` passes `test:web-pack-install` |
| Candidate is provable before tagging | dispatch `dry-run=true` against `main` at the candidate SHA | full run green with Publish skipped |
| Branch ref can never publish | dispatch `dry-run=false` against `main` | fails at "Require a versioned release tag" |
| Nothing else moved | diff `release.yml` | only the checkout `with:` block and one fetch step |
| Checker agrees | `effigy check:release-automation` | pass |
| Ordinary PR board unaffected | this card's own PR | `ci-web` behaves per the g16.096 exception rules |

## Validation

`effigy check:release-automation`, `effigy docs:check`, `git diff --check
origin/main...HEAD`. The dry run on the re-created tag is the executable
proof and belongs to `g16.097` step 3.

## Owned Paths

`.github/workflows/release.yml`, `scripts/check-release-automation.ts` (only
if an assertion must extend), execution log under `docs/logs/2026-09/`, root
`PAPERCUTS.md` (append only).

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, `docs/roadmaps/dispatch.md`.

## Stop Conditions

Stop if the fix needs anything beyond the checkout block and one fetch step,
or if `check-release-automation` cannot accept the shape without relaxing a
release assertion. Escalation owner: operator, via Chatterbox.

## Continuation

`g16.097` re-runs from step 0 (retract `v0.3.0` at `eab436eef`, nothing was
published), records the post-`104` `main` tip as the candidate, and follows
the new order: green branch dry run on the candidate SHA, then tag, then
tag dry run, then publish.
