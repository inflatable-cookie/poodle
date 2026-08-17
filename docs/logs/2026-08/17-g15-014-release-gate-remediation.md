# g15.014 — Release-Gate Remediation (nanoid advisory)

Status: complete — single dependency-surface change, two batches (PR pending)
Date: 2026-08-17
Card: `docs/roadmaps/g15/014-release-gate-remediation.md`
Governing refs: `docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`, `docs/specs/022-packaging-versioning-and-release-channel-rules.md`

## Batches

- **Batch A — locate and plan:** confirmed the exact dependency path
  `workspace:@inflatable-cookie/poodle-react-preview → vite@8.2.1 →
  postcss@8.5.26 → nanoid@3.3.17`, with the root `overrides.nanoid ^3.3.16`
  pinning the vulnerable resolution. The fix is a lockfile-only bump: nanoid
  3.3.18 satisfies both the root override range and postcss's `^3.3.17`
  requirement, so no manifest dependency or toolchain change is needed. A bare
  `bun update nanoid` was rejected because it added nanoid as a root direct
  dependency without moving off 3.3.17 — unrelated churn that did not close the
  advisory.
- **Batch B — remediate and verify:** raised the root `overrides.nanoid` to
  `^3.3.18` (version-pinned config, in card scope) and regenerated the
  lockfile, resolving nanoid 3.3.17 → 3.3.18. `bun audit` now exits clean and
  every non-audit lane stays green.

## Change Footprint

`package.json` (`overrides.nanoid` `^3.3.16` → `^3.3.18`) and `bun.lock`
(matching override entry plus the nanoid resolution entry, integrity hash
updated). Two files, six changed lines total. No dependency was added, removed,
or bumped beyond nanoid; vite 8.2.1, postcss 8.5.26, and all other surfaces are
unchanged.

## Validation

| Command | Result |
| --- | --- |
| `effigy audit:security` (baseline) | failed: nanoid < 3.3.18, high, `workspace:@inflatable-cookie/poodle-react-preview › vite` |
| `effigy audit:security` (after) | pass: no vulnerabilities found |
| `effigy react:build` | pass |
| `effigy test:components` | pass (288 files, 2340 tests) |
| `effigy docs:check` | pass (build clean; gate-tree-guard clean once the change is committed) |
| `effigy qa` | pass |
| `git diff --check origin/main...HEAD` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran. Roadmap status, card, generation-index, and dispatch ledger were
not modified by the worker.

## Open Items

- The advisory is closed on the dependency surface; `g15.013` certification can
  proceed once the remaining parallel lane (PR #29 focused web evidence) lands.
- No new execution friction surfaced in this lane; `PAPERCUTS.md` unchanged.
