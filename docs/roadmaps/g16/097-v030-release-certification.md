# g16.097 — v0.3.0 Release Certification

Status: complete — published 2026-09-05 from certified `v0.3.0`; coordinator-
executed and never dispatched to a worker. The first tag was retracted, and
the pre-tag re-certification at `b4158a1b` exposed the stale release tarball verifier
Type: release mutation — explicit operator authorization recorded
Opened: 2026-09-04
Depends on: merged `g16.054` (PR #165, merge `9e38e7971`) and its evidence log `../../logs/2026-09/20260902-g16-054-v030-release-candidate.md`; merged `g16.098`; merged `g16.103` release tarball verifier repair — serial, because the tag must contain the repaired release workflow
Governing refs: `054-historycenter-v030-release-candidate.md`, `../g15/061-v022-release-certification.md` (precedent), `../../specs/022-packaging-versioning-and-release-channel-rules.md`, `../../release-notes/0.3.0.md`, `.github/workflows/release.yml`, `../../../AGENTS.md`
Operator decision: 2026-09-04 — "Authorize now": certify and publish the 0.3.0 candidate (tag the certified SHA, dispatch `release.yml`, prove npm `latest` is 0.3.0), then route Loophole adoption separately
Dispatch manifest: `../dispatch.md`

## Outcome

Publish `@inflatable-cookie/poodle-core@0.3.0` and
`@inflatable-cookie/poodle-svelte@0.3.0` from the exact certified candidate,
record the immutable receipts, and hand Loophole adoption to its owner. React
stays source-only. Rust stays source/tag distribution; the tag is its
distribution point.

## First Attempt (2026-09-04, failed before publish)

- Tag `v0.3.0` was pushed at `9b451c48d2fc8ea990313fcd441056169a40f26c`
  before the dry run. Dry run `33874116177` failed in `effigy release gates`
  on three `react-preview` suites that cannot resolve
  `@inflatable-cookie/poodle-react` on a cold runner (the defect `g16.098`
  repairs). No publish step ran; npm `latest` is still `0.2.2`.
- The failing behaviour is in the tagged tree, so the tag cannot certify.
  The operator chose retraction over a `0.3.1` skip: nothing was published
  and the tag is minutes old. Precedent: the broken `v0.2.1` tag was
  retracted after `v0.2.2` replaced it.

## Second Attempt (2026-09-04, failed before publish)

- After `g16.103` merged, tag `v0.3.0` was re-created at `eab436eef` and
  dry run `33908714014` failed in `Release gates`: `git merge-base
  eab436eef origin/main` → `fatal: Not a valid object name origin/main`.
  The release checkout fetches only the tag ref, the same class of defect
  `g16.096` fixed for the PR board. No publish ran. `g16.104` repairs the
  workflow and adds the pre-tag branch dry run so this cannot recur.

## Third Attempt (2026-09-04, retracted before publish)

- The operator-authorized candidate tag `v0.3.0` was pushed at
  `eab436eefc1a65d0e0cde518a113a51c5d4d7f4e` for dry run `33908714014`.
  It failed in `Release gates` because the GitHub release checkout did not
  have `origin/main`; no pack or publish step ran and npm remained at
  `latest` `0.2.2`.
- The tag was retracted locally and remotely on 2026-09-04 after the operator
  confirmed the second retraction. `git tag -d v0.3.0` deleted the local tag,
  `git push origin :refs/tags/v0.3.0` deleted the remote tag, and a follow-up
  `git ls-remote --tags origin refs/tags/v0.3.0` was empty.
- g16.104 is the promoted repair lane. No new tag will be created until its
  merged workflow fix and the pre-tag branch dry run are green.

## Candidate (re-certification)

- Candidate commit: the `main` tip immediately after `g16.103` merges,
  recorded by the coordinator in this card before any tag. Lockstep must
  still read `0.3.0` everywhere (it does on current `main`).
- Trusted publishing is configured; the `v0.2.2` run `32756610293` published
  through this same workflow.

## Ordered Actions (coordinator, from a clean `main` checkout)

0. Retract the failed tag: `git push origin :refs/tags/v0.3.0` and
   `git tag -d v0.3.0`. Verify `git ls-remote --tags origin v0.3.0` is empty.
   Record the retracted SHA and the failed run URL in the execution log.
1. After `g16.103` merges, record the candidate SHA (`main` tip) here.
   From a clean detached checkout of that SHA run
   `bun install --frozen-lockfile`, prove `lucide-static` resolves inside that
   checkout at `1.31.0`, then run `effigy release gates`
   (the full `qa` board) and the pack/hash steps `g16.054` used; record
   tarball names, sizes, and SHA-256. Verify lockstep `0.3.0`, tag absence
   local and remote, and npm `latest` `0.2.2`. Stop on any red gate.
1b. Prove the workflow on the candidate before any tag (protocol rule
   2026-09-04; enabled by `g16.104`): `gh workflow run release.yml --ref
   main -f dry-run=true` with `main` at the candidate SHA. Wait for green.
   Record the run URL. A red run here is a stop, and no tag is created.
2. Create the lightweight tag at that exact candidate and push it:
   `git tag v0.3.0 <candidate-sha>` and `git push origin v0.3.0`. Never tag a
   different SHA, and never tag before steps 1 and 1b are green.
3. Dry run first: `gh workflow run release.yml --ref v0.3.0 -f dry-run=true`.
   Wait for completion. Require every step green, including release gates,
   version-tag agreement, and pack verification. Record the run URL.
4. Publish: `gh workflow run release.yml --ref v0.3.0 -f dry-run=false`.
   Wait for completion. Record the run URL, duration, and the
   `packed-tarballs` artifact size and digest.
5. Prove the registry: `npm view @inflatable-cookie/poodle-core dist-tags
   versions --json` and the same for `poodle-svelte` must show `latest`
   `0.3.0`. In a fresh temporary Vite + Svelte 5 project, install exact
   `0.3.0` of both from the public registry and resolve the package roots and
   one compiled subpath each. Record the transcript.
6. Write the execution log under `docs/logs/2026-09/` with every URL, hash,
   and transcript; update this card's Status and Release Result; update
   `CHANGELOG.md` and `docs/release-notes/0.3.0.md` headers from "candidate —
   not published" to the published date; update the README status paragraphs
   that describe `0.2.2` as latest. Commit on `main` and push.
7. Send Chatterbox one administrative notice naming the tag, run URLs, and
   npm proof. Chatterbox routes Loophole adoption to its owner; the
   coordinator does not write Loophole.

## Acceptance

- [ ] The failed `v0.3.0` tag is gone locally and remotely; the log records
      its SHA and run.
- [ ] Lightweight `v0.3.0` points at the re-certified candidate locally and
      remotely, and `effigy release gates` passed on it from a clean
      detached checkout before tagging.
- [ ] Dry-run and publish runs both completed green on that ref.
- [ ] npm serves core and Svelte `0.3.0` as `latest`; React has no npm
      package; no GitHub Release is created (the workflow does not make one).
- [ ] Fresh-consumer install of exact `0.3.0` resolves both roots and
      compiled subpaths.
- [ ] Changelog, release notes, and README front doors say `0.3.0` is
      published; `0.2.3` stays described as prepared but unpublished.
- [ ] Execution log records URLs, digests, and transcripts.

## Second Attempt (2026-09-04, stopped before tag)

- Candidate `b4158a1b68db9292c17be1d8c219f0fc26512a0b` passed frozen
  bootstrap, checkout-local dependency provenance, the full headless release
  gate, ordinary installed-package certification, and local core/Svelte/React
  pack proof. The tree remained clean.
- `release.yml` still required generated source members under `package/src/**`.
  The compiled package contract intentionally publishes those artifacts under
  `package/dist/**` and excludes source. `g16.103` owns the authorized workflow
  repair. No tag, workflow dispatch, or publish occurred.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Tag names the certified SHA | tag any other commit | `git rev-parse v0.3.0` equals the recorded candidate |
| Gates precede the tag | tag before a green local `release gates` | log shows the gate transcript timestamp before the tag push |
| Workflow precedes the tag | tag before a green branch dry run of `release.yml` on the candidate SHA | log shows the branch dry-run URL and timestamp before the tag push |
| Dry run precedes publish | publish first | two run URLs in order, dry-run earlier |
| Retraction is complete | old tag still on origin | `git ls-remote --tags origin v0.3.0` shows only the new SHA |
| Registry truth | `latest` still `0.2.2` after the run | `npm view` transcript with `0.3.0` |
| Docs say what npm says | changelog still says "not published" | header diff in the closeout commit |
| No workflow edit | `release.yml` diff in this lane | `git diff --stat` on the closeout commit shows no `.github/` change |

## Stop Conditions

Stop and send Chatterbox an escalation capsule when: any re-verification fact
differs; the dry run is red for any reason (do not repair code in this lane);
the publish step fails closed (trusted publisher or allowance are
operator-owned); `npm view` disagrees with the run's success; or anything
would require editing `.github/workflows/`, moving an existing tag, or
touching a sibling repository. Never rerun a publish that partially
succeeded without operator instruction. Never move a tag that has been
published; retraction is allowed only because nothing was published from
the first attempt.

## Continuation

Loophole adoption (pin `apps/desktop` to exact `0.3.0`, map deletion
failures onto the five HistoryCenter rejection codes) is Loophole-owned
planning and starts only after step 5 is proven. It is recorded in
`../../triage/20260902-000956-history-portfolio-holds.md`.

## Final Certification — 2026-09-05

- Candidate: `85609d941a208ff2f854e9f7c0e457089cc77d0e`.
- Local `effigy release gates` passed from a clean detached checkout after a
  temporary local reachability branch was created and removed.
- Branch dry run passed: https://github.com/inflatable-cookie/poodle/actions/runs/33930305831
- Tag dry run passed: https://github.com/inflatable-cookie/poodle/actions/runs/33934223827
- Publish passed: https://github.com/inflatable-cookie/poodle/actions/runs/33952493234
- `v0.3.0` resolves remotely to the candidate SHA.
- Registry proof: core and Svelte both report `latest: 0.3.0`.
- Fresh npm consumer installed exact core/Svelte `0.3.0` with Svelte `5.56.8`;
  compiled subpaths `@inflatable-cookie/poodle-core/icons` and
  `@inflatable-cookie/poodle-svelte/types` resolved successfully.
- `packed-tarballs` artifact: 930019 bytes; downloaded archive SHA-256
  `dc04659cd7a716f58a6a59f448c97ec5959855828d35de89e0e33acd0c4fea49`.
