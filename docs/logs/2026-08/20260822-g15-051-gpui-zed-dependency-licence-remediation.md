# g15.051 GPUI/Zed Dependency Licence Remediation

Date: 2026-08-22
Card: `../../roadmaps/g15/051-gpui-zed-dependency-licence-remediation.md`
Handoff: `../../handoffs/20260822-114839-g15-051-gpui-zed-dependency-licence-remediation.md`
Worker branch: `t3code/remediate-gpui-zed-dependency-license`
PR: [#67](https://github.com/inflatable-cookie/poodle/pull/67)

## Outcome

The normal GPUI graphs no longer resolve GPL-3.0-or-later `zlog`,
`ztracing`, or `ztracing_macro`. Poodle now consumes one minimal,
immutable Zed fork revision that replaces the normal `ztracing::instrument`
uses with the existing standard `tracing::instrument` API. Test-only
`zlog` remains in the fork but does not enter either Poodle normal graph.

The bzip2-1.0.6 licence is admitted with its redistribution notice. Git source
policy remains fail-closed: only the five reviewed repositories and exact full
revisions are accepted in Cargo manifests and lockfiles.

## External fork evidence

- Fork: https://github.com/inflatable-cookie/zed
- Upstream base: `zed-industries/zed@1ea16c1ab9dd6d36649e002dc60995634da04daf`
- Fork branch: `poodle/g15-051-tracing-licence-remediation`
- Patch commit: [`87d9afbe71ef06ea0634499dc35d104bb29dc020`](https://github.com/inflatable-cookie/zed/commit/87d9afbe71ef06ea0634499dc35d104bb29dc020)
- Patch diff: six files, 5 insertions and 10 deletions. The five named source/manifests are `crates/gpui/Cargo.toml`, `crates/gpui/src/svg_renderer.rs`, `crates/sum_tree/Cargo.toml`, `crates/sum_tree/src/cursor.rs`, and `crates/sum_tree/src/sum_tree.rs`; the sixth file is the mechanical `Cargo.lock` update.
- Fork validation: `cargo check --locked -p gpui -p sum_tree` passed; `cargo test --locked -p sum_tree --lib` passed 10/10.

## Poodle changes

- Pinned `packages/gpui/node-backend` and `packages/gpui/preview` to the
  fork revision and regenerated both lockfiles. The previously ignored
  node-backend lockfile is now tracked as reviewed source evidence.
- Added `bzip2-1.0.6` and the five fixed Git repositories to `deny.toml`;
  retained `unknown-git = "deny"` and `required-git-spec = "rev"`.
- Added the bzip2/libbzip2 notice to the repository root and the public-intent
  native package, with executable notice-marker checks. The terms are from the
  exact [`libbz2-rs-sys` v0.2.5 `COPYING` text](https://github.com/trifectatechfoundation/libbzip2-rs/blob/v0.2.5/COPYING);
  [SPDX](https://spdx.org/licenses/bzip2-1.0.6.html) remains the identifier
  reference.
- Added exact Cargo manifest/lock URL-plus-revision validation and focused
  negative tests for unknown repositories, mutable references, changed
  revisions, and missing notices.
- `cargo deny` reports `no-license-field` warnings for `gpui_util` and
  `gpui_shared_string`. Both crates carry tracked
  `LICENSE-APACHE -> ../../LICENSE-APACHE` symlinks at fork commit `87d9afbe`;
  their Apache-2.0 licence evidence is present even though their manifests omit
  the metadata field.
- Added one small `PAPERCUTS.md` entry for the zsh lowercase `path`
  variable collision found during repository probing.

## Validation

Implementation checks ran on the worker worktree. No windowed, workflow,
release mutation, tag, or publication path was run.

| Check | Result |
| --- | --- |
| Normal dependency graph proof for node-backend and preview | pass; no `zlog`, `ztracing`, `ztracing_macro`, GPL-3.0-or-later package, or unapproved Git source |
| `bun test scripts/audit-repository-security.test.ts` | 5/5 pass |
| `cargo deny --manifest-path packages/gpui/preview/Cargo.toml check licenses sources` | pass |
| `effigy audit:licenses` | pass; 8 package manifests, 17 Cargo manifests, 5 notice surfaces |
| `effigy audit:security` | pass; exact manifest and lockfile source checks clean |
| `effigy ci:native` | pass; node backend 24/24, Poodle render 370/370, GPUI adapter 133/133, Jetstream adapter 162/162, headless regressions 56/56 |
| `effigy smoke:gpui-offscreen-capture` | pass; three repeated captures matched hash `be94eaceb6c310c4e067c012b579c53d2c6d4147fc63160673316538c9997c6d` |
| `effigy docs:check` | pass after the fresh worktree's ignored JS dependencies were bootstrapped with `bun install --frozen-lockfile` |
| `effigy release gates` | pass; configured/executed 1 headless gate, exit 0, 174498ms |
| `git diff --check origin/main...HEAD` | clean after the final log commit |

## Continuation

The orchestrator found that the original notice used generic SPDX bzip2 text
rather than the exact `libbz2-rs 0.2.5` COPYING text. Fix `0954aa1f` restored
all dependency-specific attribution and tightened the executable notice
markers. PR #67 was then accepted and merged as `30e2aae3` with operator
authorization. `g15.051` is complete. The next planning checkpoint is the
exact `g15.047` comparator envelope.
