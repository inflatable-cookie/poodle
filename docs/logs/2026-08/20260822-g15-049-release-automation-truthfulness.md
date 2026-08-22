# g15.049 Release Automation Truthfulness

Date: 2026-08-22
Card: `../../roadmaps/g15/049-release-automation-truthfulness.md`
Governing refs: `../../roadmaps/g15/release-gap-register.md`,
`../../specs/022-packaging-versioning-and-release-channel-rules.md`,
`../../contracts/001-working-rules.md`, `../../effigy.toml`,
`../../tasks/effigy.tasks.toml`, `../../packages/release-operations.json`
Handoff: `../../handoffs/20260822-090530-g15-049-release-automation-truthfulness.md`
Worker branch: `t3code/release-automation-truthfulness`
PR: [#66](https://github.com/inflatable-cookie/poodle/pull/66)

## Outcome

Release automation now has one configured headless authority. The retained
manual workflows are thin launchers for named Effigy selectors, the stale
conformance workflow and alias are gone, and `effigy release gates` executes
the complete `effigy qa` board instead of succeeding with zero configured
gates.

The release workflow remains a human-dispatched, tag-checked, dry-run-first
publication path. It preserves the core + Svelte publish set, keeps React out
of publication, and uses npm trusted publishing prerequisites without a
long-lived registry token. No package version, lockfile, tag, publication,
release mutation, component, specimen, fixture, native implementation,
Jetstream engine, or unrelated workflow surface changed.

## Changed Surfaces

- `.github/workflows/ci-web.yml` runs `effigy ci:web`.
- `.github/workflows/ci-rust.yml` runs `effigy ci:rust` on explicit Rust
  `1.95`.
- `.github/workflows/ci-native.yml` installs Bun `1.3.14`, selects Rust `1.95`,
  and runs `effigy ci:native`.
- `.github/workflows/ci-visual.yml` maps explicit `smoke`, `axis`, and
  `sweep` tiers to the current named selectors and rejects unknown tiers.
- `.github/workflows/release.yml` fails early off `refs/tags/v*`, installs Rust
  `1.95` and the reviewed `cargo-deny 0.19.4` CLI, runs `effigy release gates`
  as its single release gate, then retains pack inspection and dry-run
  publication.
- `.github/workflows/ci-conformance.yml` was deleted.
- `effigy.toml` now requires Effigy `0.11.0` and configures the `headless`
  release gate with `command = "effigy qa"`.
- `tasks/effigy.tasks.toml` removes `ci:conformance`, puts the static release
  automation check first in `qa`, adds `check:release-automation`, and records
  the workflow runtime bootstrap accurately.
- `scripts/check-release-automation.ts` statically protects workflow pins,
  selector mappings, runtime prerequisites, tag gating, tier rejection,
  release posture, publish scope, and non-vacuous gate configuration.

## Immutable Action and Trusted-Publishing Evidence

Every retained `uses:` reference is a full SHA with a same-line release or
channel comment. The resolved official repository evidence is:

| Action | Reviewed ref |
| --- | --- |
| `actions/checkout` | `v7.0.1` → `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| `actions/setup-node` | `v7.0.0` → `820762786026740c76f36085b0efc47a31fe5020` |
| `actions/cache` | `v6.1.0` → `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` |
| `actions/upload-artifact` | `v7.0.1` → `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` |
| `oven-sh/setup-bun` | `v2.2.0` → `0c5077e51419868618aeaa5fe8019c62421857d6` |
| `inflatable-cookie/setup-effigy` | `v1.0.0` → `987fd556617ea2c3e0ab5cef6b47b250817f50c8` |
| `dtolnay/rust-toolchain` | `Rust 1.95` → `4360b52568e2003a75bf9bc1d59f33a8e3fc893c` |

The action SHAs were checked against their official repositories and the
security posture was checked against [GitHub's secure use reference](https://docs.github.com/en/actions/reference/security/secure-use).
Trusted publishing follows [npm's trusted publishing docs](https://docs.npmjs.com/trusted-publishers/): job-local `id-token: write`,
`contents: read`, no `NPM_TOKEN`, Bun `1.3.14`, Rust `1.95`, Node `22.22.2`,
npm `12.0.2`, and cargo-deny `0.19.4`. The selected npm CLI is above the
documented npm `>=11.5.1` and Node `>=22.14.0` trusted-publishing requirements.

## Validation

| Check | Result |
| --- | --- |
| `bun scripts/check-release-automation.ts` | pass — checked 5 retained workflows, immutable runtimes, tag gating, the Effigy gate, alias removal, and publish set |
| `actionlint .github/workflows/*.yml` | pass |
| `effigy ci:web` | pass — 357 test files, 3,053 tests; packed proof 20/20; exact 175/175 Svelte and 175/175 React roots |
| `effigy test:visual-fixtures` | pass — 43 Bun inventory tests and 15 Rust loader tests; fixture lane preserved from PR #65 |
| `effigy ci:rust` | pass — all declared Rust contract suites |
| `effigy ci:native` | pass — drift checks, 56 headless regressions, 8 specimen probes, native and adapter suites |
| `effigy test:visual-smoke` | fail — one existing unrelated `pill [eclipse-compact-md]` delta at 0.665% across 15 pairs; no visual surface changed |
| `effigy release gates` | fail after executing 1 configured `headless` gate and `effigy qa`; `cargo deny --manifest-path packages/gpui/node-backend/Cargo.toml check licenses` rejects `bzip2-1.0.6` and GPL-3.0-or-later Zed dependencies not explicitly allowed |
| `effigy docs:check` | pass |
| `git diff --check origin/main...HEAD` | pass |

The release-gate run proves the gate is configured and executed; its failure
is an existing dependency-license policy issue outside this batch and remains
tracked by `docs/triage/20260822-104657-gpui-zed-license-policy.md`. No release
workflow was dispatched, no windowed/native-visual/Jetstream selector was run,
and no tag, package, registry, or release mutation occurred.

## Continuation

The release-automation truthfulness surface is ready for review. The
orchestrator should review the known cargo-deny baseline failure alongside the
workflow diff and continue the remaining release runway with `g15.050`.
