# g18.006 closeout — v0.4.0 candidate merged (integration checkout, 2026-09-13)

## Merged outcome

- PR #265 (`g18.006: prepare the v0.4.0 web editor release candidate`)
  merged as `567fe01c33e7e80514cff6f7d14c516a9a9a788b`, now `origin/main`.
- Candidate head `9d18a21bdf344d4b659e5a56ab6b6b4b80d6891a`: freeze
  `a797ce413` (lockstep `0.4.0` inputs), repin `d8cb83ccc` (Nucleus +
  GPUI census evidence), repair `9d18a21bd` (deterministic certification
  head on `pull_request` CI).
- Accepted review: round-2 independent review (comment `5649713789`),
  verdict `ready_to_merge` at the exact head; round-1 `changes_required`
  closed by the repair commit. Exact-head CI `rust` + `web` green;
  mergeable `MERGEABLE`/`CLEAN`; non-draft throughout.

## Truthful validation

- Closeout reran no implementation or broad validation suites. The
  certificate is the worker/reviewer evidence plus plugin-owned
  exact-head merge verification: local `main` verified clean at
  `567fe01c`, matching `origin/main`, before this batch.
- Boundaries held: no tag, no publication, no registry mutation, no
  workflow edit or dispatch, no Desktop change, no gate bypass.

## Deferred

- No task-specific failure is deferred. One non-blocking observation
  from review: the guard self-amendment cap moved 150 → 500 added lines
  (≈264 actual growth); growth beyond 500 needs a fresh admission.
- g18.009 owns the hosted branch dry run, tag, tag dry run and
  publication from the unchanged merged candidate. Desktop resumes
  retained g02.058 and PR #215 only after public registry proof.
