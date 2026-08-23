# g15.050 — v0.2.0 Release Candidate

Status: **ready — all implementation, specimen, conformance, packaging,
automation, dependency-policy, and adopter children are complete**
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

## Acceptance Envelope

- [ ] Version `0.2.0` agrees across every release-bearing manifest and lockfile.
- [ ] All release blockers and parent cards are closed with evidence; no
      declared absence is counted as parity.
- [ ] `effigy qa`, packed-roster proof, docs checks, licence/security audits,
      and the repaired read-only release automation are green at one clean SHA.
- [ ] Release notes state exactly what publishes and what remains experimental
      or deferred.
- [ ] Candidate artifacts can be reproduced from the pinned SHA.
- [ ] No tag, GitHub release, npm publish, or registry mutation occurs.

## Stop Conditions

- Any earlier card, release-gap row, or supported gate is open/red.
- The candidate claim exceeds landed conformance evidence.
- A version or packed artifact differs from the pinned commit.

## Continuation

After review, `g15.013` is the explicit operator gate for tag and publication.
