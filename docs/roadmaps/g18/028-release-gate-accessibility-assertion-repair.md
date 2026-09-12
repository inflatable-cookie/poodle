# 028 — Release-gate accessibility assertion repair

Status: changes requested — PR #260 must repin the invalidated Nucleus cohort
Owner: Poodle Rust render contracts
Created: 2026-09-12
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/003-native-accessibility.md`,
`006-v040-web-editor-release-and-desktop-unblock.md`
Depends on: none; current main already contains accepted g18.026 and g18.027

## Outcome

Repair two stale `poodle-render` test assertions that still expect
pre-accessibility semantics. Prove the current intentional accessible labels
without changing product behavior, then reopen retained g18.006's release gate.

## Ready-State Rubric

- [x] Both failures reproduce deterministically on clean main
  `bb8999b68ae2be3366d93413abdd781e2adec1ea`.
- [x] The implementation behavior is traced to accepted accessibility changes:
  button labels from NP-4 and per-option SegmentedControl labels from g16.119.
- [x] The stale assertions and expected replacements are exact.
- [x] Scope is limited to test expectations and release-gate proof.
- [x] Tom selected a separate repair task on 2026-09-12 rather than widening
  retained g18.006.

## Decisions

- `context::tests::the_provider_adds_no_wrapper_node_layout_or_accessibility_entry`
  must expect the provided button's own accessible label `Some("Save")`. The
  test continues to prove that the provider adds no wrapper semantics.
- `segmented_control::tests::icon_only_without_an_icon_keeps_the_visible_label`
  must expect `Some("Grid")`. A segment that falls back to a visible text label
  carries that same label into accessibility output.
- Do not remove, suppress or special-case the current labels to satisfy stale
  tests. No public or runtime behavior changes in this task.
- Any edit under `packages/render` invalidates the source commit bound into the
  29 Nucleus M1/A1 receipts. Tom initially preferred one later repin in
  g18.006, then ruled on 2026-09-12 that PR #260 must not merge with a red web
  check. This task therefore regenerates the complete cohort against its exact
  repaired head. g18.006 performs its own later final repin after changing the
  release-bearing Cargo manifest.
- This repair does not run the full local release board. Its broad proof is the
  required exact-head `web` and `rust` PR checks. Retained g18.006 owns one
  local `effigy release status --check-gates` run after the complete `0.4.0`
  candidate is stable.

## Dispatch manifest

- **State:** ready; independent release-blocking repair; serial before retained
  g18.006 resumes
- **Completion:** one independently reviewed repair PR merged with focused
  tests, the full `poodle-render` library tests, direct generated-evidence
  checks and required exact-head `web`/`rust` PR checks green
- **Owned mutable paths:** `packages/render/src/context.rs`,
  `packages/render/src/segmented_control.rs`; the complete generated Nucleus
  receipt, manifest, parity-ledger and GPUI-census cohort required by the exact
  repaired source identity; focused execution evidence
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, g18.006 task/workspace, versions, locks,
  changelog, release notes, workflows and Desktop
- **Worker:** general Rust test repair; semantics and expected edits are fixed
- **Excluded:** production behavior, accessibility contract changes, candidate
  preparation, local full-board/release certification, tag/publish, workflow
  edits, Desktop mutation and unrelated test cleanup
- **Escalation:** Chatterbox if either expected label contradicts current
  contract or any other release gate fails

## Work

1. Reproduce the two named failures on the exact current main lineage.
2. Confirm the current label projection against contracts and accepted source
   history before editing.
3. Replace only the two stale expectations with `Some("Save")` and
   `Some("Grid")`, preserving each test's original negative assertion.
4. Regenerate and repin the complete Nucleus receipt, manifest, parity-ledger
   and GPUI-census cohort against the exact repaired source identity. Refuse a
   partial cohort or mixed source commits.
5. Run the two focused tests and `cargo test -p poodle-render --lib`. Validate
   the generated cohort with `effigy test:nucleus-parity-receipts`,
   `effigy check:parity-evidence-ledger`, `effigy check:gpui-census`,
   `effigy docs:lint`, and `git diff --check`.
6. Commit and push the stable head. Require the ordinary exact-head GitHub
   `web` and `rust` checks. Do not also run local `effigy qa`, `effigy ci:web`,
   `effigy docs:check`, or `effigy release status --check-gates`; those repeat
   broad work and the final release identity does not exist yet.
7. Update PR #260 and return the new exact head for independent re-review.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Provider remains semantics-neutral | test passes by deleting the button's own label | assertion sees `Some("Save")` while existing wrapper/layout negatives remain |
| Visible segment label remains accessible | icon-only fallback paints `Grid` but accessibility is unnamed | assertion sees `Some("Grid")` |
| Product repair is assertion-only | renderer logic changes to manufacture a passing result | non-generated product diff contains only the two expectations |
| Evidence binds the repaired head | receipts retain `d0554d84`, mix source commits or only suppress validation | all 29 receipts plus manifest, ledger and census regenerate coherently; direct checkers and exact-head PR lanes pass |

## Stop conditions

- Stop if current contracts do not require either label.
- Stop if a production-code change is needed.
- Stop and report any direct evidence checker or required PR lane still red
  after the complete cohort repin; do not widen into another product or release
  repair.
- Stop before candidate, version, tag, publication, workflow or Desktop work.

## Evidence

Planning diagnosis from retained g18.006 at clean main
`bb8999b68ae2be3366d93413abdd781e2adec1ea` reproduced exactly two failures in
`cargo test -p poodle-render --lib`. The same assertions predate intentional
label projection from NP-4 commit `51b820a5d` and g16.119 commit `cceb6646a`.

PR #260 changes only the two assertions at
`390095f6f3653846af0d95d16d35a91c944389a0`. Worker evidence records 2 focused
tests and all 647 `poodle-render` library tests green. Release status advances
to the parity-ledger/GPUI-census refusal because the 29 receipts bind source
commit `d0554d844598725782fb5c6cc16190fdae9b83ca`. PR #260 may not merge with
that red web check; the complete cohort is now part of this repair.

## Next task

After green exact-head PR checks, independent re-review and merge, resume the
existing g18.006 Queue task, worker and workspace on current main. It performs
the later final repin after its release-bearing changes, then runs the full
local release gate once against the stable candidate. Do not replace its task
or candidate lane.
