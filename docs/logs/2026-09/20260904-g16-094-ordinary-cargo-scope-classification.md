# g16.094 — Ordinary Cargo Scope Classification Repair

Status: complete — awaiting orchestrator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/094-ordinary-cargo-scope-classification.md`
Handoff: `docs/handoffs/20260904-001500-g16-094-ordinary-cargo-scope-classification.md`
Governing refs: `docs/architecture/014-compiled-web-package-distribution.md`,
`docs/roadmaps/g16/059-installed-web-distribution-certification.md`,
`docs/roadmaps/g16/061-installed-web-smoke-certification-routing.md`,
PR #199 exact-head review
Branch: `fix/g16-094-ordinary-cargo-scope`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-094-ordinary-cargo-scope`
Base: `origin/main` at `d7cd34aa14e8b240fd259a697994e98444deab7a`

## Outcome

Ordinary installed-smoke classifies Cargo release-version surfaces from parsed
manifest sections, not from path membership. `[package]` and
`[workspace.package]` version mutations still fail before build/pack. So do
`publish`, `registry`, `source`, `[patch]`, and `[replace]`. Dependency
requirements, features, and lock resolution are not version surfaces.
`package.json` version, workflow, release, and registry paths are unchanged.
Strict still path-rejects Cargo. Candidate allowlists, receipts, source-child
law, and Cargo honesty plants are unchanged.

No component, Nucleus evidence, workflow, release, public API, or sibling
change. No receipt re-stamp. No tag, publish, workflow dispatch, windowed, or
native-visual selector.

## Reproduction

PR #199's exact-head review reproduced unset/ordinary
`test:web-pack-install` rejecting only because the range included Cargo paths:

```text
certification scope rejected forbidden version surface: packages/gpui/node-backend/Cargo.lock,
version surface: packages/gpui/node-backend/Cargo.toml,
version surface: packages/gpui/preview/Cargo.lock
    at assertInstalledScope (test/package-install/scope.ts:572)
```

The production plants committed first reproduced the same path-only label:

```text
version surface: packages/gpui/node-backend/Cargo.lock, version surface: packages/gpui/node-backend/Cargo.toml
version surface: packages/gpui/node-backend/Cargo.toml
```

Publication plants were also rejected as `version surface` rather than
registry/publication content.

## Review oracle

| Invariant | Plant | Result |
| --- | --- | --- |
| Dependency repair is ordinary | feature + lock refresh on `packages/gpui/node-backend/Cargo.toml` + `.lock` | ordinary proof; no receipt |
| Package version stays forbidden | `[package] version` `0.2.3` → `0.3.0` | `version surface: packages/gpui/node-backend/Cargo.toml` |
| Workspace package version stays forbidden | root `[workspace.package] version` | `version surface: Cargo.toml` |
| Inline dependency version is not release version | `{ version, features }` under `[dependencies]` | ordinary proof |
| Lock resolution is not a version surface | third-party `tinyvec` lock bump only | ordinary proof |
| Publication remains forbidden | `publish = true`, `registry`, inline `source`, `[patch]`, `[replace]` | `registry surface: packages/gpui/node-backend/Cargo.toml` |
| Certification is unchanged | strict dependency-only edit; existing candidate publish/registry/retarget/evidence-head plants | strict still `version surface` by path; candidate messages unchanged |

## Validation

- Focused plants: `effigy test:core-build` — 51 pass / 0 fail, including 21
  production-guard plants in `test/package-install/scope.test.ts`.
- `effigy test:web-pack-install` on `4633d576ae9a9f9303d0b52c5ec35a62f108e859`:
  11 files / 22 tests; stdout `mode: ordinary`; no `receiptSha256`; new
  ordinary Cargo version/publish/registry/source/patch/replace plants failed
  as required; existing workflow/version/release/registry and candidate plants
  kept prior outcomes.
- `effigy ci:web`: pass. Inner pack-install again printed `mode: ordinary`
  with no receipt hash.
- `effigy docs:check`: pass.
- `git diff --check origin/main...HEAD`: pass.

`FORCE_COLOR=1` colorizes packed `tsc` diagnostics and breaks the HistoryEntry
exact-matcher. Selectors ran with `FORCE_COLOR` unset; the friction is in
`PAPERCUTS.md`. First `test:core-build` on the cold worktree missed
`packages/core/dist` outputs; rerun passed.

No `release prepare/execute/simulate`, tag, publish, workflow dispatch,
windowed selector, or native-visual selector was run.

## Limits

- This worker has not merged, rebased PR #199, tagged, published, dispatched a
  workflow, or edited Nucleus evidence. Rebase and `ci:web` of PR #199 stay
  with the orchestrator after merge.
- Strict full-pack receipt emission was not rerun on this branch: the range
  includes routing docs outside the g16.059 allowlist, so strict correctly
  rejects it. Strict path/allowlist plants used the production guard.

## Continuation

Orchestrator exact-head review remains. After merge, rebase PR #199 onto
current main and require its full `ci:web` before merging g16.092. No
certification re-stamp is required.
