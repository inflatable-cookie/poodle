# 029 — v0.4.0 candidate-scope admission

Status: ready
Owner: Poodle installed web distribution certification
Created: 2026-09-12
Governing refs: `../../contracts/001-working-rules.md`,
`../../specs/070-compiled-web-distribution-contract.md`,
`006-v040-web-editor-release-and-desktop-unblock.md`,
`009-v040-release-certification-and-desktop-unblock.md`,
`../../../test/package-install/scope.ts`
Depends on: none; current main contains accepted g18.028 and the retained
g18.006 candidate has made no unique commit

## Outcome

Add one closed admission policy for the Poodle `0.4.0` candidate so the normal
PR `web` lane and local release gate can certify its release-bearing diff
without a workflow exception or ambient operator-only environment variable.
Merge the policy before retained g18.006 prepares any candidate input.

## Ready-State Rubric

- [x] Clean pre-promotion product baseline
  `ee5bdefce276558abb37c76486bc244680d812b4` reproduces the missing admission.
- [x] Ordinary `test:web-pack-install` rejects a planted `0.4.0` package bump
  plus changelog entry before build or pack.
- [x] The only explicit candidate mode is the historical
  `g16.054-candidate`, fixed to `0.3.0` and its old path set.
- [x] Retained g18.006 is clean at current main with no PR or unique commit.
- [x] Tom selected a separate reviewed precursor on 2026-09-12 and moved the
  hosted branch dry run to g18.009 after candidate merge.

## Decisions

- Preserve strict certification and the historical `g16.054-candidate`
  contract. Do not rename it, alias it, or weaken its `0.3.0` proof.
- Add a distinct closed `g18.006-candidate` policy for the exact
  `0.3.0` → `0.4.0` release. Keep policy data together: mode, source/target
  versions, writable release inputs, generated/evidence paths and content
  validators. Do not scatter another version literal across unrelated branches.
- The normal PR lane has no candidate environment variable. When ordinary
  certification sees release-bearing changes, it may select the g18.006 policy
  only when the complete base-to-head range satisfies that closed policy.
  Partial, wrong-version or unrelated release changes remain rejected.
- A candidate needs a frozen release-input commit followed by generated
  evidence whose `source_commit` names that frozen identity. Permit later
  commits only when every later change is evidence/log-only and the final
  package and release inputs remain byte-identical to the frozen candidate.
  Reject hidden or later candidate-input drift.
- The admitted candidate paths are only the g18.006 release inputs and exact
  evidence families already authorized by its dispatch manifest: lockstep JS
  and Cargo manifests/requirements, locks, changelog, `0.4.0` release notes,
  generated version stamps/package evidence, the complete Nucleus receipt,
  ledger and GPUI-census cohort, and the fixed g18.006 execution record. Do not
  admit component source, workflows, release/publish transport, registry
  configuration, Desktop, or arbitrary documentation.
- g18.006 owns the stable local gate and ordinary exact-head PR checks. The
  hosted release-workflow branch dry run occurs after its candidate merges and
  belongs to g18.009. No workflow edit is authorized.

## Dispatch manifest

- **State:** ready; independent release-infrastructure precursor; serial before
  retained g18.006 resumes
- **Completion:** one independently reviewed PR merged with focused scope laws,
  ordinary candidate recognition, strict/historical-mode preservation and
  exact-head `web`/`rust` checks green
- **Owned mutable paths:** `test/package-install/scope.ts`,
  `test/package-install/scope.test.ts`,
  `test/package-install/web-preview.ts`, `test/package-install/README.md`, and
  one focused execution log
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, g18.006/g18.009 cards and handoffs, retained g18.006 task,
  versions, locks, changelog, release notes, generated candidate evidence,
  workflows, tags, registries and Desktop
- **Worker:** complex certification-policy implementation; must preserve
  fail-closed path/content checks and prove the ordinary CI entry
- **Excluded:** candidate preparation, any `0.4.0` manifest or changelog edit,
  Nucleus regeneration, workflow edit/dispatch, tag, publication, Desktop
  mutation and g18.006/g18.009 execution
- **Escalation:** Chatterbox if safe automatic recognition requires a workflow
  change, a broader writable surface, or cannot bind evidence to one frozen
  candidate identity

## Work

1. Reproduce the ordinary-mode rejection with a committed-range fixture; do
   not mutate real release inputs.
2. Refactor candidate policy selection only as far as needed to keep the
   historical g16.054 rule intact and express one closed g18.006 policy without
   duplicated version/path branching.
3. Make ordinary certification recognize the g18.006 candidate only from the
   complete validated diff. Keep ordinary maintenance, strict certification
   and unknown-mode refusal unchanged.
4. Validate the frozen candidate-input identity and evidence-only suffix. The
   final release inputs must match the frozen commit byte-for-byte.
5. Add production-path laws for accepted `0.4.0`, partial/wrong versions,
   arbitrary source, workflow/publish/registry surfaces, React admission,
   Cargo retargeting, hidden/later input drift, false ordinary promotion and
   unchanged g16.054 behavior.
6. Run the focused scope and web-pack-install suites, package-install docs
   check, `git diff --check`, then use required exact-head PR checks as the
   broad proof. Do not run the release gate; g18.006 owns it once.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Ordinary CI admits only the exact candidate | any version bump selects candidate policy | complete `0.3.0` → `0.4.0` fixture passes; partial and wrong-version plants fail before build/pack |
| Candidate identity is frozen once | evidence commit silently changes a manifest | later-input-drift plant fails; final input bytes equal the frozen commit |
| Release transport stays protected | workflow, publish script or registry config rides the allowlist | production guard rejects all three planted surfaces |
| Historical proof remains immutable | g16.054 semantics are repurposed for 0.4.0 | existing g16.054 laws remain green and still require 0.3.0 |
| React and Rust publication stay bounded | candidate makes React public or adds Cargo registry metadata | manifest/Cargo content plants fail closed |
| Precursor is not a candidate | this PR edits versions, locks, changelog or receipts | diff contains only owned harness/docs paths |

## Stop conditions

- Stop if ordinary recognition cannot remain closed and content-validated.
- Stop if a workflow edit or environment-only CI exception appears necessary.
- Stop if the required path set exceeds g18.006's existing release/evidence
  ownership or admits component implementation.
- Stop before candidate preparation, release gate, tag, publication or Desktop.

## Evidence

Retained g18.006 proved on the clean pre-promotion product baseline that
`effigy release status --check-gates`
reaches `test:web-pack-install`, where ordinary certification rejects
`CHANGELOG.md` and package version surfaces. Explicit
`g16.054-candidate` accepts only `0.3.0` and is not selected by PR CI. The
release workflow supplies no scope environment. g18.028 is complete, and the
retained g18.006 branch/workspace is clean at main with no candidate mutation.

## Next task

After this PR merges and closes, resume Queue task
`17ac3fee-de90-4b32-9672-1134770bb086` in its retained workspace and rebase it
onto the admitted policy. g18.006 prepares and merges the stable candidate;
dependency-queued g18.009 then performs the hosted branch dry run, tag, tag dry
run and publication sequence.
