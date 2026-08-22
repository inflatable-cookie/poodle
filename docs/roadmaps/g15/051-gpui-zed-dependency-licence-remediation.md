# g15.051 — GPUI/Zed Dependency Licence Remediation

Status: **ready — operator accepted the fixed licence/source direction on
2026-08-22**
Depends on: `g15.045`, `g15.049`
Unblocks: `g15.050`
Governing refs: `../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../contracts/001-working-rules.md`, `release-gap-register.md`, repository
`AGENTS.md`

## Problem

The truthful release board rejects the GPUI dependency graph. The pinned Zed
revision brings GPL-3.0-or-later `zlog`, `ztracing`, and `ztracing_macro` into
the normal graph through `gpui` and `sum_tree`. The same audit also rejects the
permissive `bzip2-1.0.6` licence because Poodle has not recorded it, while the
source and security gates reject every Git dependency because none has an
approved immutable-source policy.

Adding GPL exceptions would make the command green while leaving Poodle's MIT
native distribution claim unresolved. The graph must change instead.

## Goal

Adopt one exact, minimal Zed fork that removes GPL tracing crates from
Poodle's resolved GPUI graph, admit the permissive bzip2 licence with its
notice, and pin every reviewed Git source without weakening fail-closed audits.

## Fixed Decisions

- Create or reuse `inflatable-cookie/zed` as a GitHub fork of
  `zed-industries/zed`. Base the Poodle patch on exact upstream commit
  `1ea16c1ab9dd6d36649e002dc60995634da04daf` and pin Poodle to the resulting
  immutable fork commit.
- Keep the fork patch narrow:
  - remove the normal `ztracing` dependency from `crates/gpui` and
    `crates/sum_tree`;
  - replace their `ztracing::instrument` imports/annotations with standard
    `tracing::instrument`;
  - do not change GPUI behaviour, APIs, rendering, features, or unrelated Zed
    crates;
  - leave test-only `zlog` outside the patch when it does not enter Poodle's
    resolved graph; remove it only if the fork's focused checks require that
    equally narrow cleanup.
- Do not relicense Zed code and do not open an upstream PR or modify the
  upstream issue. The operator authorised the Poodle-owned fork, not upstream
  coordination.
- Admit `bzip2-1.0.6` in the global permissive licence allowlist. Preserve its
  required copyright, conditions, and disclaimer in Poodle's third-party
  notice evidence.
- Do not add GPL licence exceptions, package exceptions, or broad source
  bypasses.
- Approve only these reviewed Git repositories in `deny.toml`, with every
  manifest use pinned by full `rev` and every lockfile source checked against
  its expected commit:
  - `https://github.com/inflatable-cookie/zed`;
  - `https://github.com/zed-industries/font-kit`;
  - `https://github.com/zed-industries/scap`;
  - `https://github.com/zed-industries/wasm_thread`;
  - `https://github.com/proptest-rs/proptest`.
- Extend the repository-security audit to recognise only the exact approved
  manifest and lockfile URL/revision pairs. Changing a URL or revision must
  fail until reviewed.

## Scope Envelope

- Create the fork and one minimal patch commit under the authorised
  `inflatable-cookie` GitHub owner. Record upstream base, fork branch, patch
  commit, and diff summary in the execution log.
- Point the GPUI node backend and preview manifests at the exact fork revision;
  update their lockfiles mechanically.
- Update licence, Git-source, notice, and repository-security policy as fixed
  above.
- Prove the node-backend and preview normal graphs contain no `zlog`,
  `ztracing`, `ztracing_macro`, or other GPL dependency.
- Run supported headless GPUI and release-audit checks. Preserve the existing
  offscreen capture result and Rust 1.95 floor.

## Writable Scope

External fork:

- `crates/gpui/Cargo.toml`
- `crates/gpui/src/svg_renderer.rs`
- `crates/sum_tree/Cargo.toml`
- `crates/sum_tree/src/cursor.rs`
- `crates/sum_tree/src/sum_tree.rs`
- fork lockfile only when the focused check changes it mechanically

Poodle:

- `deny.toml`
- `packages/gpui/node-backend/{Cargo.toml,Cargo.lock}`
- `packages/gpui/preview/{Cargo.toml,Cargo.lock}`
- `scripts/audit-repository-security.ts`
- `scripts/audit-license-compliance.ts`
- `THIRD_PARTY_NOTICES.md` and a native notice surface if packaging evidence
  shows the root notice is not carried with the distributed artifact
- focused audit tests or fixtures required to make the exact source/revision
  rule executable
- one August `g15.051` execution log
- `PAPERCUTS.md` for newly found execution friction

No component, contract, public API, visual baseline, specimen, theme, package
version, release note, workflow, tag, publication, Jetstream integration, or
unrelated dependency update belongs here.

## Acceptance Envelope

- [ ] `inflatable-cookie/zed` contains one reviewable patch based on exact
      upstream `1ea16c1ab9dd6d36649e002dc60995634da04daf`; the log records both
      immutable commits and the bounded diff.
- [ ] Poodle's GPUI node-backend and preview manifests pin the exact fork
      commit, and their lockfiles resolve the same commit.
- [ ] The resolved normal graph for both manifests contains no `zlog`,
      `ztracing`, `ztracing_macro`, GPL-3.0-or-later, or other unapproved
      licence.
- [ ] `bzip2-1.0.6` is allowed as a permissive licence and its redistribution
      terms are retained; no GPL exception exists.
- [ ] Every Git source is from the five fixed repositories and every direct
      dependency plus lockfile entry uses its reviewed full revision. Unknown
      or mutable sources still fail.
- [ ] GPUI adapter, node-backend, preview, headless regressions, and offscreen
      capture remain green on Rust 1.95 with no window or focus acquisition.
- [ ] Licence, source, notice, repository-security, and read-only release gates
      pass without a bypass. No release mutation occurs.

## Validation

- focused fork check for `gpui` and `sum_tree`
- exact `cargo metadata` / inverse-tree proof for node-backend and preview
- `cargo deny --manifest-path packages/gpui/preview/Cargo.toml check licenses sources`
- `effigy audit:licenses`
- `effigy audit:security`
- `effigy ci:native`
- `effigy smoke:gpui-offscreen-capture`
- `effigy docs:check`
- `effigy release gates`
- `git diff --check origin/main...HEAD`

Run the full release gate once after the focused batch is green. Never run a
`*-windowed`, native-visual, paired-Jetstream, release-prepare, release-execute,
tag, or publish path.

## Stop Conditions

- The worker cannot create or push the fork under `inflatable-cookie`.
- The patch needs behaviour, API, renderer, feature, or unrelated upstream
  changes.
- A GPL crate remains in either resolved normal graph.
- A new Git repository, moving ref, licence exception, or distribution-policy
  decision is required.
- Notice compliance cannot be made explicit from the authoritative upstream
  licence text.
- Any requested validation requires a window, focus acquisition, release
  mutation, tag, publication, or workflow edit.

## Continuation

After review and merge, close the dependency-licence row in the release gap
register. `g15.050` remains blocked on `g15.043`, `g15.047`, and every other
open release gap; this card does not authorise candidate or publication work.
