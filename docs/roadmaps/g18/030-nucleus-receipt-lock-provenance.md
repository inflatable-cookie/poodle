# 030 — Nucleus receipt lock provenance

Status: complete — PR #262 merged as `80609ff3528bc32fc44cd6f1a8dc5dd1ddb68b85` on 2026-09-12 after exact-head independent review
Owner: Poodle native evidence infrastructure
Created: 2026-09-12
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`,
`006-v040-web-editor-release-and-desktop-unblock.md`,
`029-v040-candidate-scope-admission.md`,
`../../../packages/gpui/preview/src/nucleus_receipts.rs`,
`../../../scripts/nucleus-parity-receipts.ts`
Depends on: none; current main contains accepted g18.029 and retained g18.006
has no unique candidate commit

## Outcome

Make the GPUI Nucleus receipt emitter derive its lockfile SHA-256 and selected
package resolution from the actual preview `Cargo.lock`. Remove the
release-version literals that force every lockstep release to edit emitter
source, then repin the current source-bound evidence once for this precursor.

## Ready-State Rubric

- [x] Retained g18.006 is clean at current main with no candidate mutation.
- [x] A real `0.4.0` bump must change the preview lockfile.
- [x] The current emitter hard-codes that lockfile's old hash and four Poodle
  package versions.
- [x] The merged g18.029 policy correctly excludes component/runtime source
  from a release-only candidate.
- [x] Deriving provenance before candidate preparation removes the conflict
  without widening candidate ownership.
- [x] Scope, evidence repin and continuation are closed.

## Decisions

- Keep g18.029's closed candidate admission unchanged. The release candidate
  must not gain arbitrary runtime-source ownership.
- Read the exact `packages/gpui/preview/Cargo.lock` bytes at receipt emission.
  Hash those bytes and derive the canonical five-package resolution in fixed
  order: `gpui`, `poodle-gpui`, `poodle-gpui-preview`, `poodle-node`, and
  `poodle-render`.
- Use the existing `sha2` test dependency and a small fail-closed parser for
  the Cargo lock package blocks. Do not add a dependency or change either
  Cargo manifest/lockfile in this task.
- Reject missing, duplicate, incomplete, unexpected-source, or checksum-less
  registry entries. Do not silently keep a stale fallback.
- The emitter source edit changes the mounted runtime identity. Repin the
  complete current Nucleus M1/A1 cohort, manifest, ledger and affected GPUI
  census evidence once on the final implementation identity.
- g18.006 performs its separate final `0.4.0` repin after the version inputs
  freeze. That later run must require no emitter-source edit.

## Dispatch manifest

- **State:** complete; independent release-infrastructure precursor merged
  before retained g18.006 resumes
- **Completion:** one independently reviewed PR merged with derived provenance,
  planted version/hash laws, complete current evidence repin and exact-head
  `web`/`rust` checks green
- **Owned mutable paths:**
  `packages/gpui/preview/src/nucleus_receipts.rs`; focused tests beside that
  module if needed; `docs/evidence/nucleus/**`; source-bound
  `docs/evidence/gpui/**`; parity ledger/census generated projections; one
  focused execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.006/g18.009 cards and handoffs, retained g18.006 task,
  versions, manifests, locks, changelog, release notes, workflows, tags,
  registries and Desktop
- **Worker:** native evidence worker comfortable with deterministic lock
  parsing, provenance falsification and complete headless receipt repins
- **Excluded:** Cargo manifest or lock edits; installed-package policy changes;
  candidate preparation; workflow changes; release gates; windowed selectors;
  tags; publication; Desktop mutation; unrelated test optimization
- **Escalation:** Chatterbox if deriving the exact current resolution requires a
  new dependency, changes receipt schema/order, or cannot fail closed

## Work

1. Add focused laws around a pure lock-provenance helper before changing the
   emitter. Prove the current lock, a planted `0.4.0` lock, altered bytes,
   missing/duplicate packages and invalid source/checksum cases.
2. Replace `LOCKFILE_SHA256` and the hard-coded Poodle versions with values
   derived from the exact lockfile bytes. Keep receipt schema, field order,
   package order and current output byte shape stable apart from provenance.
3. Prove a planted `0.3.0` to `0.4.0` lock change updates emitted hash and
   resolution without any source rewrite or fallback.
4. Freeze the implementation identity. Run the complete current headless
   Nucleus emission once and repin every source-bound M1/A1 receipt, manifest,
   ledger and affected GPUI census artifact to that one identity.
5. Run focused receipt validation, parity-ledger and census checks, docs lint
   and `git diff --check`. Open one non-draft PR and report immediately; Queue
   owns asynchronous exact-head CI observation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Lock identity is live | receipt keeps the old SHA after one lock byte changes | fixture hash changes and equals SHA-256 of the supplied bytes |
| Release versions are not embedded | planted `0.4.0` Poodle entries still emit `0.3.0` | derived fixed-order resolution reports every planted version |
| Parsing fails closed | duplicate or missing selected package is shadowed or skipped | focused negative laws reject before receipt publication |
| External provenance stays exact | `gpui` loses its registry source/checksum | source/checksum negative laws reject |
| Current evidence is coherent | only some receipts or manifest fields are repinned | all M1/A1 receipts, manifest, ledger and census validate one source/lock identity |
| Candidate scope remains closed | precursor widens g18.029 or edits release inputs | diff contains no scope-policy, manifest, lock, changelog or version change |
| Release no longer needs emitter edits | `0.4.0` preparation still changes this source file | disposable versioned-lock proof emits correct provenance from unchanged source |

## Stop conditions

- Stop if a new dependency or Cargo lock change is required.
- Stop if the parser cannot uniquely identify the canonical five packages.
- Stop if a complete current source-bound repin cannot validate at one commit.
- Stop before candidate preparation, release gates, workflows, tags,
  publication or Desktop.

## Evidence

Retained g18.006 proved the conflict on clean main
`274757dc2b8f4f9800ee87211a8cc9daee16ff67`: its mandatory future preview
`Cargo.lock` bump changes the real lock hash and Poodle versions, while
`nucleus_receipts.rs` hard-codes the old hash and `0.3.0` resolution. The file
is correctly outside the merged release-only candidate allowlist. A complete
28-path candidate plant is admitted; adding the required emitter edit is the
sole rejected path. The retained task has no PR or unique commit.

## Merge and review

PR #262 merged as `80609ff3528bc32fc44cd6f1a8dc5dd1ddb68b85` on 2026-09-12
with parents `a057041942364989b3bbd31428df5c30b029f341` (main carrying this
handoff) and `362013c7bbc64496588749bf052324a2be9f22ea` (reviewed head). The
merge matches the reviewed head exactly; no base refresh was needed.

The accepted independent review is [PR comment
5648889053](https://github.com/inflatable-cookie/poodle/pull/262#issuecomment-5648889053),
bound to the exact reviewed head with `ready_to_merge`. It found no blocking
findings. Exact-head GitHub `rust` and `web` checks were green at the merge
gate. The review noted only a non-blocking PR-description count shorthand;
the committed evidence is complete.

## Closeout

- The integration checkout was verified clean on `main` at the merged commit,
  matching both `origin/main` and the provider's merged PR identity before this
  closeout batch.
- Closeout reran no implementation or broad validation suites. It consumes the
  worker/reviewer evidence recorded above and the plugin's exact-head merge
  gate. No task-specific validation failure is deferred.
- Broader release work remains deliberately deferred: retained g18.006 now
  resumes the `0.4.0` candidate on current main, and g18.009 owns the hosted
  branch dry run, tag and publication sequence. No release, tag, workflow or
  Desktop mutation occurred here.

## Next task

Resume Queue task `17ac3fee-de90-4b32-9672-1134770bb086` in its retained
workspace on current main. Fast-forward it to the accepted merge, prepare the
immutable `0.4.0` candidate, perform the one final version-bound Nucleus repin,
and continue through ordinary review. g18.009 remains dependency-queued behind
that candidate and owns the hosted branch dry run, tag and publication.
