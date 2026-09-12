# g18.028 — Release-gate accessibility assertion repair

Status: complete — merged as `018b9214cb2eb88cb9651d9ec8a17808e53410cd` (PR #260) on 2026-09-12 after exact-head independent re-review
Date: 2026-09-12
Branch: `ns-a6d2eb0a-9989-48ea-88cb-15e52219553d`
Card: `docs/roadmaps/g18/028-release-gate-accessibility-assertion-repair.md`
Handoff: `docs/handoffs/20260912-172129-g18-028-release-gate-accessibility-assertion-repair.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/003-native-accessibility.md`,
`docs/roadmaps/g18/006-v040-web-editor-release-and-desktop-unblock.md`
Base: pushed `main` at `3b1e91e82fe25dc3838a5dbc2612c5d488902da1`
(stage-aware validation policy, post-g18.027)

## Merge and review

- PR #260 merged `018b9214cb2eb88cb9651d9ec8a17808e53410cd` on 2026-09-12 with
  parents `3b1e91e82fe25dc3838a5dbc2612c5d488902da1` (main) and
  `3eb8e6c98c9f95a4e2289660df66e571381a42cd` (reviewed repin head): the merge
  matches the reviewed head exactly, no refresh was needed.
- Independent re-review (issue comment `5647932569`, `ready_to_merge`)
  verified the repin round at the exact head: non-generated product diff is
  exactly the two expectation lines, all 123 receipt diffs change only
  `source_commit`/`run_id` (no observation drift), zero `d0554d84` refs
  remain, direct evidence checkers green. Two earlier `ready_to_merge`
  reviews covered the superseded assertion-only head `390095f6f` and were not
  reused.
- Merge-time checks were green (exact-head `rust`, `web`); the branch diff
  stayed inside the owned assertion plus generated-evidence paths with no
  production, contract, version, lock, changelog, release-note, workflow,
  Desktop, candidate or tag mutation.

## Outcome

The two stale `poodle-render` expectations now assert the accepted
accessible-label semantics: the provided button carries its own
`Some("Save")` (NP-4 `51b820a5d`, provider stays semantics-neutral) and the
visible-text segment fallback projects `Some("Grid")` (g16.119 `cceb6646a`),
with all negative wrapper/layout assertions preserved. The complete Nucleus
cohort (58 M1/A1 + 65 mounted receipts, manifest `resolution.source_commit`,
GPUI execution record, census pair) binds the single repaired source identity
`390095f6f3653846af0d95d16d35a91c944389a0`. The Rust library gate is green;
retained g18.006 resumes on current main.

## Execution

- Reproduced both failures deterministically, traced each label to its
  accepted contract lineage, replaced only the two stale expectations.
- Mid-lane operator ruling (main `3894ef547`) moved the Nucleus cohort repin
  into this task after the `390095f6f` head tripped the receipt source pin in
  the web lane; the worker regenerated the full cohort, refused partial/mixed
  commits, and returned the new exact head for fresh independent re-review.
- Canonical policy (main `3b1e91e82`) made validation stage-aware: this
  repair's broad proof is the exact-head `web`/`rust` PR lanes, not a local
  full-board run. A redundant nine-minute local release-gate probe was stopped
  at Tom's request and not restarted.
- Stopped at the repair boundary: no production, contract, version, lock,
  changelog, release-note, workflow, Desktop, candidate, tag or publication
  change; no g18.006 state mutation.

## Validation

- Both focused tests pass; `cargo test -p poodle-render --lib`: 647 passed,
  0 failed.
- `effigy test:nucleus-parity-receipts`: 17 pass / 0 fail;
  `effigy check:parity-evidence-ledger`: 176 rows validated;
  `effigy check:gpui-census`: artifacts match generator.
- `effigy docs:lint` green; `git diff --check` clean.
- Exact-head GitHub `web` and `rust` checks green at merge.

## Closeout (integration checkout, 2026-09-12)

- Verified local `main` clean at `018b9214cb2eb88cb9651d9ec8a17808e53410cd`,
  matching `origin/main`; merge parents confirm the reviewed head merged
  without refresh.
- Deferred, not repaired here: retained g18.006 performs its own later final
  repin after its release-bearing `0.4.0` manifest change, then runs the one
  full local release gate against the stable candidate.

## Next task

Retained g18.006 resumes its Queue task, worker and workspace on current
main now that the Rust release gate is reopened. It owns the final repin,
the full local release gate, and all candidate/version/tag/publication work.
Do not replace its task or candidate lane.
