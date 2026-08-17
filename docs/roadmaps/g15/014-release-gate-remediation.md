# g15.014 — Release-Gate Remediation: Security Advisory Prerequisite

Status: **complete** — merged in PR #31 (`7a93a7c8`); `g15.013` unblocked
Depends on: `g15.001` (recorded the open advisory)
Governing refs: `release-gap-register.md`, `release-baseline-roster.md`,
`../../contracts/001-working-rules.md`

## Outcome

Close the single open release gate: the high-severity `nanoid` advisory
(GHSA-2v37-7h3g-55p8, nanoid < 3.3.18) reached through the React preview's
Vite dependency, which fails `bun audit` inside `effigy qa`. Effigy's release
contract does not allow a red gate to be waived, so this remediation is a
prerequisite for `g15.013` certification. It is a dependency-surface fix only
and touches no component behaviour.

## Scope

- resolve the advisory by upgrading the affected dependency chain or
  replacing it, so `bun audit` exits clean
- the dependency chain is the React preview's Vite stack — no component,
  contract, specimen, or public API change is in scope

## Execution Plan

- [x] **Batch A — locate and plan:** identify the exact dependency path
      (`@inflatable-cookie/poodle-react-preview → vite → … → nanoid`), check
      whether the fix is a bump, an override, or a replacement, and record
      the plan and affected lockfile surfaces.
- [x] **Batch B — remediate and verify:** apply the dependency change,
      regenerate the lockfile, and run `effigy qa` to confirm `bun audit`
      passes while every other lane stays green.

## Goals

- [x] `bun audit` passes inside `effigy qa`.
- [x] The remediation is the smallest dependency-surface change that closes
      the advisory; no unrelated dependency churn.
- [x] All non-audit lanes remain green after the change.

## Acceptance

- [x] `effigy qa` passes fully green, including `bun audit`.
- [x] The change is confined to dependency manifests/lockfiles (and any
      version-pinned config the upgrade requires).
- [x] `effigy react:build`, `effigy test:components`, and `effigy docs:check`
      pass.

## Stop Conditions

- The fix changes component, contract, specimen, or public API behaviour.
- The advisory is suppressed or bypassed instead of remediated.
- The dependency change expands into unrelated upgrades without a new card.

## Writable Scope

- dependency manifests, lockfiles, and version-pinned config
- one August batch log under `docs/logs/2026-08/`
- `PAPERCUTS.md` for newly discovered execution friction

## Validation

- `effigy qa` (headless local release board — `bun audit` lane must pass)
- `effigy react:build`, `effigy test:components`, `effigy docs:check`
- `git diff --check`

Never run a `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or
any Jetstream selector.
