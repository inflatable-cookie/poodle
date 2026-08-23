# g15.050 — v0.2.0 Release Candidate

Status: **complete — PR #72 landed candidate `4428ad10` with evidence-only
receipt `42f46a9b`; merge commit `c60abf6b`**
Depends on: `g15.011`, `g15.012`, `g15.042`, `g15.043`, `g15.048`, `g15.049`,
`g15.051`, `g15.052`, `g15.053`
Unblocks: `g15.013` operator certification gate
Governing refs: `release-baseline-roster.md`, `release-gap-register.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`013-v020-release-certification.md`

## Goal

Prepare one exact, reviewable v0.2.0 candidate without tagging or publishing:
lockstep versions, honest release notes, clean packed artifacts, full headless
QA, and an operator-readable certification receipt pinned to one commit.

## Fixed Decisions

- Version the three public-intent TypeScript packages
  (`poodle-core`, `poodle-svelte`, `poodle-react`) and every Rust crate below
  `packages/` at `0.2.0`. Update intra-repository dependency requirements and
  lockfiles with them. Private/internal TypeScript tooling manifests are not
  release-bearing and keep their current versions.
- The v0.2.0 publication set remains core + Svelte. React is packed and
  certified as experimental but is not published. Rust crates are certified
  as source/tag distribution. Jetstream stays deferred and must not be
  described as active-cohort parity.
- Use two commits for honest exact-SHA evidence:
  1. a **candidate commit** containing every version, lockfile, release-note,
     changelog, metadata, guide, and candidate-facing documentation change;
  2. an **evidence-only commit** adding the August `g15.050` execution log and
     receipt, which names the candidate commit, commands, artifact filenames,
     sizes, and SHA-256 checksums.
- Run every candidate gate from a clean checkout of the candidate commit. If
  any candidate-bearing file changes afterward, create a new candidate commit,
  rerun the complete board, and replace the receipt. Editing only the receipt
  does not repin the candidate.
- Keep tarballs outside the tracked tree. The committed receipt proves how to
  reproduce them; it is not an artifact store.
- `effigy release gates` is read-only and allowed. `effigy release prepare`,
  `effigy release execute`, workflow dispatch, tag creation/push, GitHub
  release creation, `npm publish`, and registry mutation are forbidden.

## Scope Envelope

- Set every release-bearing TypeScript package and Rust crate to `0.2.0` in
  lockstep; update lockfiles mechanically.
- Write `docs/release-notes/0.2.0.md` and the Unreleased changelog disposition.
  Name breaking/additive/behavioral changes, migration actions, the certified
  Svelte roster, React/Rust/GPUI posture, visual-evidence boundary, and deferred
  Jetstream status.
- Reconcile package metadata, guides, release manifest/operations, roadmap
  fronts, roster, and gap register with the candidate claim.
- Produce clean packed tarballs and a dry-run release receipt from the exact
  candidate commit.
- Run the complete supported headless board. Native/windowed or publish actions
  remain separate operator gates.

## Writable Scope

- the three public-intent TypeScript package manifests and relevant Bun
  lockfile entries;
- every Rust crate manifest below `packages/`, intra-repository version
  requirements, and Cargo lockfiles;
- `CHANGELOG.md`, `docs/release-notes/0.2.0.md`, package/readme/operator guides,
  release manifest/operations metadata, and release-facing front doors whose
  current wording would otherwise contradict the candidate;
- `docs/logs/2026-08/20260823-g15-050-*.md` for the evidence-only receipt;
- `PAPERCUTS.md` for newly found execution friction.

Do not edit component contracts or implementations, specimens, visual
baselines, workflow files, Effigy task definitions, dependency policy, or
Jetstream admission state. A required change to one of those surfaces means
the candidate is not ready: stop and report it.

## Acceptance Envelope

- [ ] Version `0.2.0` agrees across every release-bearing manifest and lockfile.
- [ ] All release blockers and parent cards are closed with evidence; no
      declared absence is counted as parity.
- [ ] `effigy qa`, packed-roster proof, docs checks, licence/security audits,
      and the repaired read-only release automation are green at one clean SHA.
- [ ] Release notes state exactly what publishes and what remains experimental
      or deferred.
- [ ] Candidate artifacts can be reproduced from the pinned SHA.
- [ ] The committed receipt pins the candidate commit rather than its later
      evidence-only commit, and no candidate-bearing file changed after the
      recorded gates ran.
- [ ] No tag, GitHub release, npm publish, or registry mutation occurs.

## Required Validation

From the clean candidate commit:

- exact version/lockfile checks across the fixed manifest denominator;
- `effigy test:web-pack-install` and clean local packs for core, Svelte, and
  experimental React; record filenames, byte sizes, and SHA-256 checksums;
- `effigy check:release-automation`;
- `effigy audit:licenses` and `effigy audit:security`;
- `effigy qa`;
- `effigy release gates` and proof that its one `headless` gate ran;
- `effigy docs:check`;
- `git diff --check` before the candidate commit and
  `git diff --check origin/main...HEAD` before PR handoff.

Use only supported headless selectors. Do not run `*-windowed`, native-visual,
Jetstream preview/QA, or any release mutation.

## Stop Conditions

- Any earlier card, release-gap row, or supported gate is open/red.
- The candidate claim exceeds landed conformance evidence.
- A version or packed artifact differs from the pinned commit.

## Continuation

After review, `g15.013` is the explicit operator gate for tag and publication.
