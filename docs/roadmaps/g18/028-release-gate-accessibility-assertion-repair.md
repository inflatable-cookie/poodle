# 028 — Release-gate accessibility assertion repair

Status: ready
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

## Dispatch manifest

- **State:** ready; independent release-blocking repair; serial before retained
  g18.006 resumes
- **Completion:** one independently reviewed PR merged with both focused tests,
  full `poodle-render` library tests and the release status gate green
- **Owned mutable paths:** `packages/render/src/context.rs`,
  `packages/render/src/segmented_control.rs`; focused execution evidence only
- **Reserved closeout surfaces:** g18 README, generation index, dispatch
  projection, task status/evidence, g18.006 task/workspace, versions, locks,
  changelog, release notes, workflows and Desktop
- **Worker:** general Rust test repair; semantics and expected edits are fixed
- **Excluded:** production behavior, accessibility contract changes, candidate
  preparation, tag/publish, workflow edits, Desktop mutation and unrelated test
  cleanup
- **Escalation:** Chatterbox if either expected label contradicts current
  contract or any other release gate fails

## Work

1. Reproduce the two named failures on the exact current main lineage.
2. Confirm the current label projection against contracts and accepted source
   history before editing.
3. Replace only the two stale expectations with `Some("Save")` and
   `Some("Grid")`, preserving each test's original negative assertion.
4. Run the two focused tests, `cargo test -p poodle-render --lib`, and
   `effigy release status --check-gates`.
5. Open one non-draft PR and return exact-head evidence for independent review.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Provider remains semantics-neutral | test passes by deleting the button's own label | assertion sees `Some("Save")` while existing wrapper/layout negatives remain |
| Visible segment label remains accessible | icon-only fallback paints `Grid` but accessibility is unnamed | assertion sees `Some("Grid")` |
| Repair is test-only | renderer logic changes to manufacture a passing result | exact diff contains assertion/evidence changes only |
| Release gate is genuinely reopened | focused tests pass while another library or release check remains red | full library test plus `effigy release status --check-gates` |

## Stop conditions

- Stop if current contracts do not require either label.
- Stop if a production-code change is needed.
- Stop and report any additional red release gate; do not widen this repair.
- Stop before candidate, version, tag, publication, workflow or Desktop work.

## Evidence

Planning diagnosis from retained g18.006 at clean main
`bb8999b68ae2be3366d93413abdd781e2adec1ea` reproduced exactly two failures in
`cargo test -p poodle-render --lib`. The same assertions predate intentional
label projection from NP-4 commit `51b820a5d` and g16.119 commit `cceb6646a`.

## Next task

After this repair merges, resume the existing g18.006 Queue task, worker and
workspace on current main. Do not replace its task or candidate lane.
