# g16.097 — v0.3.0 Release Certification

Status: ready — coordinator-executed; never dispatched to a worker
Type: release mutation — explicit operator authorization recorded
Opened: 2026-09-04
Depends on: merged `g16.054` (PR #165, merge `9e38e7971`); its evidence log `../../logs/2026-09/20260902-g16-054-v030-release-candidate.md`
Governing refs: `054-historycenter-v030-release-candidate.md`, `../g15/061-v022-release-certification.md` (precedent), `../../specs/022-packaging-versioning-and-release-channel-rules.md`, `../../release-notes/0.3.0.md`, `.github/workflows/release.yml`, `../../../AGENTS.md`
Operator decision: 2026-09-04 — "Authorize now": certify and publish the 0.3.0 candidate (tag the certified SHA, dispatch `release.yml`, prove npm `latest` is 0.3.0), then route Loophole adoption separately
Dispatch manifest: `../dispatch.md`

## Outcome

Publish `@inflatable-cookie/poodle-core@0.3.0` and
`@inflatable-cookie/poodle-svelte@0.3.0` from the exact certified candidate,
record the immutable receipts, and hand Loophole adoption to its owner. React
stays source-only. Rust stays source/tag distribution; the tag is its
distribution point.

## Accepted Candidate (verified 2026-09-04)

- Candidate commit: `9b451c48d2fc8ea990313fcd441056169a40f26c`
  (`release: prepare v0.3.0 candidate`, 2026-09-02), an ancestor of
  `origin/main`, merged through PR #165.
- Lockstep at that commit: core, Svelte, React manifests and every
  `packages/**/Cargo.toml` read `0.3.0`; `main` reads the same.
- Tag state: no `v0.3.0` tag exists locally or on `origin`. `v0.2.3` was
  never tagged; its three 2026-08-30 release runs failed or were cancelled and
  published nothing.
- Registry state: npm `latest` is `0.2.2` for both packages.
- Trusted publishing is already configured; the `v0.2.2` run `32756610293`
  published through this same workflow without a long-lived token.

## Ordered Actions (coordinator, from a clean `main` checkout)

1. Re-verify the facts above: ancestor check, lockstep at the candidate,
   tag absence local and remote, npm `latest`. Stop on any change.
2. Create the lightweight tag at the exact candidate and push it:
   `git tag v0.3.0 9b451c48d2fc8ea990313fcd441056169a40f26c` and
   `git push origin v0.3.0`. Never tag a different SHA.
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

- [ ] Lightweight `v0.3.0` points at `9b451c48d` locally and remotely.
- [ ] Dry-run and publish runs both completed green on that ref.
- [ ] npm serves core and Svelte `0.3.0` as `latest`; React has no npm
      package; no GitHub Release is created (the workflow does not make one).
- [ ] Fresh-consumer install of exact `0.3.0` resolves both roots and
      compiled subpaths.
- [ ] Changelog, release notes, and README front doors say `0.3.0` is
      published; `0.2.3` stays described as prepared but unpublished.
- [ ] Execution log records URLs, digests, and transcripts.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Tag names the certified SHA | tag any other commit | `git rev-parse v0.3.0` equals the candidate |
| Dry run precedes publish | publish first | two run URLs in order, dry-run earlier |
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
succeeded without operator instruction.

## Continuation

Loophole adoption (pin `apps/desktop` to exact `0.3.0`, map deletion
failures onto the five HistoryCenter rejection codes) is Loophole-owned
planning and starts only after step 5 is proven. It is recorded in
`../../triage/20260902-000956-history-portfolio-holds.md`.
