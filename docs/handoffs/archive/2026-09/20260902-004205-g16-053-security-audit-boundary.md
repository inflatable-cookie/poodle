---
title: g16.053 security audit boundary worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
review_authority: orchestrator
merge_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260902-004205-g16-053-security-audit-boundary.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g16, Papercuts, security]
---

## What This Thread Was Doing

Poodle's repository security audit currently mistakes `sk-` inside an ordinary
hyphenated English compound for an OpenAI token. This handoff dispatches only
`g16.053`: repair that one repository-owned matcher boundary and add biting
positive/negative tests without weakening the audit or excluding source.

This is one bounded implementation lane. No transcript or second prompt is
part of the authority chain.

## Why It Matters

The false positive keeps `audit:security`, `qa`, and the later release-candidate
gate red even though no credential exists. The smallest correct repair restores
a trustworthy fail-closed audit. It clears only one prerequisite; it does not
authorize candidate, tag, workflow, publication, or Loophole work.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Planning base commit:** `7f59ae42f4917c675968819eb23a5e41dc90013c`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `7f59ae42f4917c675968819eb23a5e41dc90013c` before this handoff was drafted
- **Planning checkout:** clean before these uncommitted handoff drafts
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Planning artifacts included at the base:** merged PR #148, `g16.053`, the
  canonical continuation runway, accepted release sequence, and the open root
  `PAPERCUTS.md` entry
- **Worker branch:** `fix/g16-053-security-audit-boundary`
- **Worker worktree:** `/Users/tom/.t3/worktrees/poodle/g16-053-security-audit-boundary`
- **Worktree creation command:** fallback only:
  `git worktree add /Users/tom/.t3/worktrees/poodle/g16-053-security-audit-boundary -b fix/g16-053-security-audit-boundary origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required
- **Required sibling worktree links:** none
- **Required workspace label:** exactly `Papercuts` before launch
- **Active spec lane:**
  `docs/roadmaps/g16/component-continuation-runway.md`
- **Roadmap milestone:** `docs/roadmaps/g16/README.md`
- **Ready cards, in order:**
  `docs/roadmaps/g16/053-repository-security-audit-boundary-repair.md`
- **Allowed runway:** `g16.053` only
- **Remaining card budget:** one card
- **Dispatch topology:** independent ready frontier beside `g16.045`–`g16.049`;
  `g16.055` is a separate already-dispatched lane
- **Parallel safety check:** this lane owns only the repository-security matcher,
  its focused test boundary, the exact root papercut entry, and its card/log.
  It does not share intended mutable source with other ready lanes.
- **Surfaces this lane owns:** `scripts/audit-repository-security.ts`; the
  smallest testable matcher-policy seam if extraction is required;
  `scripts/audit-repository-security.test.ts`; focused local fixtures if needed;
  the exact security-audit false-positive entry in `PAPERCUTS.md`;
  `docs/roadmaps/g16/053-repository-security-audit-boundary-repair.md`; one
  `g16.053` execution log only if repository convention needs it
- **Integration ownership:** the orchestrator owns `docs/roadmaps/g16/README.md`,
  `docs/roadmaps/generation-index.md`, continuation-runway/register front doors,
  release sequencing, `g16.054` and its compiled-JS/declarations prerequisite,
  review, and merge
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/contracts/001-working-rules.md`;
  `PAPERCUTS.md`; `scripts/audit-repository-security.ts`;
  `scripts/audit-repository-security.test.ts`;
  `docs/triage/20260901-230400-history-release-adoption-decision.md`
- **Review oracle:** `g16.053` `## Review Oracle`
- **Model capability profile:** `mechanical` non-frontier implementation worker;
  one exact matcher boundary and focused regression set need careful but routine
  repository work, not frontier escalation
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no workflow, release/version/package,
  unrelated scanner, consumer, sibling-repository, windowed/native-visual, tag,
  registry, or publication mutation; do not suppress files or remove evidence
  prose
- **Required validation:** focused matcher tests; `effigy audit:security`;
  `effigy docs:check`; the narrow relevant QA selector discovered through
  Effigy; and `git diff --check origin/main...HEAD`
- **PR base/head:** current pushed `main` at dispatch / worker branch head
  pending
- **PR URL:** pending
- **Review state:** awaiting worker implementation and PR, then exact-head
  orchestrator review
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

Please keep this run inside the named runway:

- **In scope:** reproduce the known English-compound false positive; make the
  OpenAI-token matcher require a real left token boundary through the smallest
  equivalent rule; add focused positive and negative tests for `sk-`,
  `sk-proj-`, whitespace, quote, `=`, `:`, near misses, and embedded English
  compounds; prove the same repository denominator and every other secret class
  remain unchanged; close only the exact papercut entry when the proof passes.
- **Out of scope:** another scanner or secret class; path exclusions;
  suppressions; deleted/rewritten evidence prose; broad security refactoring;
  package, version, lock, generated, release-note, workflow, tag, registry,
  publication, Loophole/Longhorn, consumer, windowed, native-visual, or
  `g16.054` work.
- **Outcome shape:** smallest complete repository-owned bug fix with a biting
  regression. A real key shape at an allowed boundary must still be detected;
  a matching test that only restates the repaired regex is insufficient unless
  it exercises the production matcher path.
- Do not invent a new security policy, change the denominator, or weaken the
  fail-closed audit.
- This handoff represents one worker lane, and sibling lanes may be running
  concurrently. Write only inside **Surfaces this lane owns**. Leave global
  closeout, release, and front-door surfaces to **Integration ownership**. Stop
  on shared mutable scope rather than resolving it yourself.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge belongs to the orchestrator after its accepted
  review/check gate.

## Important Context

- **Planning lineage:** repeated g16 validation found `audit:security` matching
  the substring beginning `sk-` inside `mask-plus-translated-highlight`. The
  accepted HistoryCenter release sequence made the fix a separate Papercuts
  prerequisite before candidate freeze. PR #148 promoted it as `g16.053`.
- **Why this card is ready:** the failure, expected boundary, owner, scope,
  positive and negative cases, denial of file exclusions, validation, and
  release separation are all fixed by the card.
- **Decisions and preferences:** a left word boundary is the recorded plausible
  fix, but the production behavior—not that spelling—is authoritative. Preserve
  all real-key fixtures and other secret classes. Test through a reusable policy
  seam only when needed; do not broaden the audit rewrite.
- **Open tensions:** tracked test source must not itself trip the production
  repository audit. Construct fixtures safely while still proving the exact
  production matcher behavior. If a correct fix needs denominator or policy
  changes, stop.
- **Report after:** reproduction plus biting focused tests, then after the full
  card validation and diff-scope proof.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, the canonical continuation runway, `g16.053`, the exact root
papercut, working rules, and the production audit plus its tests. Reproduce the
false positive first, then make the smallest testable production-path repair.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare its generated path/branch with the planned
   fallback or create another worktree merely because they differ.
3. If current context is `main`, dirty, unregistered, or unusable, inspect the
   named worktree. If unusable, read `.agents.local.env`, require
   `AGENTS_WORKTREE_CONTAINER_DIR`, and ask the operator when absent. Create a
   unique worktree/branch there from pushed `origin/main`. Never use `/tmp`,
   `TMPDIR`, or a guessed path; never clean, reset, stash over, or discard dirty
   state. Report a launcher-supplied dirty or `main` worktree instead.
4. From the selected worktree, record this handoff's repository-relative path.
   Run `GIT_SSH_COMMAND="ssh -o ConnectTimeout=10 -o BatchMode=yes" git fetch
   origin`. Confirm `HEAD` equals `origin/main`, confirm
   `git merge-base --is-ancestor 7f59ae42f4917c675968819eb23a5e41dc90013c HEAD`,
   and confirm this relative path exists in `HEAD`. Load it with
   `git show HEAD:docs/handoffs/20260902-004205-g16-053-security-audit-boundary.md`.
   If the absolute dispatch file differs from that tracked blob, stop.
5. Required sibling links are `none`.
6. Confirm the active Paseo workspace carries exactly the `Papercuts` label
   before implementation starts. If it does not, stop and report it.
7. Read the active milestone, assigned card, `AGENTS.md`, and canonical refs.
8. Use Effigy only where it fits the job. Run cheap orientation checks and
   record what actually ran.

### While you work

- Reproduce the current production false positive before changing the matcher.
- Keep the fix local to the OpenAI matcher and a focused production-path test
  seam. Do not alter scan enumeration, other secret classes, or audit output
  merely to make the test easy.
- After the coherent matcher/test batch, report changed files, validation run,
  what remains, risks, and blockers.
- Stop if the repair needs exclusions, policy weakening, workflow/release
  changes, another scanner, or new unrelated audit work.

### When the assigned runway is complete

1. Run the required final validation exactly as listed in **Current State**.
2. Falsify every `g16.053` oracle row. At minimum plant and restore the prior
   unanchored matcher, a quoted `sk-proj-` case, the English compound, and an
   attempted path exclusion. Prove each test fails for its named reason.
3. Update the exact root papercut entry, card, and optional execution log with
   actual evidence. Do not edit the g16 README, generation index, continuation
   front doors, release packet, or `g16.054`.
4. Push the worker branch. If a sibling lane merged first, rebase onto current
   `main`, rerun the required validation, and report the new exact head.
5. Open one PR against current pushed `main`. The planning base above is not a
   self-referential hash for the handoff commit.
6. Link the card, papercut, production matcher/tests, validation, falsification
   evidence, diff-scope proof, and unresolved items.
7. Report the PR URL and exact head. Do not merge, start candidate work, or run
   a release command.

### Review and merge path

The orchestrator reviews the current PR head against the card, production-path
regressions, unchanged denominator/secret classes, diff, and checks. Shared-
identity review is posted as the canonical PR comment when formal self-approval
is unavailable. Requested changes stay on this branch. Blocking classes are
`execution-miss`, `oracle-gap`, `planning-change`, `validation-gap`, and
`integration-drift`. Requested changes: none. The orchestrator alone merges an
accepted, current, mergeable head after required checks.

- **Closeout refs:**
  `docs/roadmaps/g16/053-repository-security-audit-boundary-repair.md`, the
  exact `PAPERCUTS.md` entry, and one optional `docs/logs/2026-09/` g16.053 log;
  global runway/front-door and release-prerequisite status remain
  orchestrator-owned after merge.

### Handoff closeout

Before calling the runway complete, leave the card, papercut, tests, and next-
task state honest. An accepted `g16.053` merge clears only the security-audit
gate; `g16.054` remains blocked on its separately promoted compiled-JS and
declarations prerequisite. If blocked, record the blocker and stop.
