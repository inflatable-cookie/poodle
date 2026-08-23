# 022 Packaging Versioning And Release Channel Rules

Status: active
Updated: 2026-08-22
Depends on: `021-public-package-api-stability-and-parity-debt-baseline.md`

## Purpose

Freeze the packaging and release posture for the current Poodle package set so the
repo can distinguish public-intent packages from internal-only packages before
downstream adoption starts.

## Package Classification Rule

Every package must fall into one of these release classes:

- `source-of-truth`: internal packages that define or generate canonical data
- `runtime-package`: packages intended to be consumed by downstream runtimes
- `bridge`: internal adaptation layers for downstream-owned systems
- `tooling`: internal docs, preview, or validation tooling

The canonical classification record is `packages/release-manifest.json`.
Operational change-control rules now live in `packages/release-operations.json`.

## Channel Rule

Only two release channels exist in the current baseline:

- `preview`: public-intent packages that may eventually be published but are
  still explicitly pre-release
- `internal`: packages that must not be treated as downstream dependencies

No package may imply a stable channel until a later generation explicitly adds
one.

## Versioning Rule

The current baseline remains pre-1.0:

- all packages remain on `0.x`
- breaking changes may happen in minor releases while the system is still
  pre-1.0
- breaking changes must still be called out in release notes, roadmap/log
  summaries, and any downstream handoff material

## Release Metadata Rule

Each release-bearing package manifest should declare:

- package name
- version
- description
- explicit release intent metadata
- explicit package exports or crate boundaries

This metadata may live in:

- `package.json` for JS/TS packages
- `Cargo.toml` metadata for Rust crates
- `packages/release-manifest.json` for repo-wide classification

## Current Package Baseline

The current release posture is:

### Preview Channel Public-Intent Packages

- `@inflatable-cookie/poodle-core`
- `@inflatable-cookie/poodle-svelte`
- `@inflatable-cookie/poodle-react`
- Rust contracts: `poodle-adapter`, `poodle-events`, `poodle-headless`,
  `poodle-ir`, `poodle-layout`, `poodle-markdown`, `poodle-node`,
  `poodle-specs`, `poodle-style`, and `poodle-tokens`
- Rust renderers: `poodle-render`, `poodle-gpui`,
  `poodle-gpui-node-backend`, and `poodle-jetstream`

### Internal Packages

- `@inflatable-cookie/poodle-tokens`
- `@inflatable-cookie/poodle-svelte-preview`
- `@inflatable-cookie/poodle-react-preview`
- `@inflatable-cookie/poodle-install-smoke`
- `poodle-gpui-preview`
- `poodle-jetstream-preview`

## Consumption Rule

Downstream repos should only plan around preview-channel packages.

They should not depend on:

- internal source-of-truth token build packages
- bridge packages
- preview/docs tooling packages

## Release Note Rule

Each release-capable tranche should document:

- which packages changed
- whether the change affects public-intent entry points
- whether the change is additive, behavioral, or breaking
- what downstream evaluators should re-check

## Native Dependency Licence And Source Rule

Public-intent native packages may ship under Poodle's permissive release
posture only when their resolved normal dependency graph is compatible with
that distribution claim.
Poodle therefore applies these rules to every GPUI release graph:

- GPL-3.0-or-later dependencies are not admitted. Do not make a release gate
  green by adding a crate exception for `zlog`, `ztracing`,
  `ztracing_macro`, or another strong-copyleft dependency.
- `bzip2-1.0.6` is an accepted permissive licence. Preserve its copyright,
  licence conditions, and disclaimer in the distributed notice surface.
- Remote Git sources are denied by default. An approved source must use a
  reviewed repository URL and an immutable full commit revision. Branches,
  tags, moving refs, and unreviewed repositories remain forbidden.
- `cargo-deny` keeps `unknown-git = "deny"` and
  `required-git-spec = "rev"`. Repository security checks independently pin
  the expected URL and revision in manifests and lockfiles.
- A release candidate must prove that the normal dependency graph for every
  public-intent GPUI package contains no GPL dependency and that the licence,
  source, notice, and repository-security audits pass.

The current Zed revision pulls its GPL tracing crates into GPUI's normal graph
through `gpui` and `sum_tree`. Until upstream resolves
[Zed issue #55470](https://github.com/zed-industries/zed/issues/55470), Poodle
uses a narrow `inflatable-cookie/zed` fork based on exact upstream commit
`1ea16c1ab9dd6d36649e002dc60995634da04daf`. The fork may only replace the
normal `ztracing` use in `gpui` and `sum_tree` with the standard permissive
`tracing` instrumentation already present in the workspace. Poodle pins the
resulting fork commit exactly and records both base and patch revisions.
Unrelated fork divergence requires a separate decision.

This is Poodle's distribution policy, not legal advice. Admitting a
strong-copyleft dependency or changing the native distribution claim requires
an explicit operator decision before implementation.

## Seed Evidence

- `packages/release-manifest.json`
- `packages/release-operations.json`
- `packages/core/package.json`
- `packages/svelte/components/package.json`
- `packages/react/components/package.json`
- `packages/svelte/preview/package.json`
- `packages/contracts/*/Cargo.toml`
- `packages/render/Cargo.toml`
- `packages/gpui/*/Cargo.toml`
- `packages/jetstream/*/Cargo.toml`
- [SPDX `bzip2-1.0.6`](https://spdx.org/licenses/bzip2-1.0.6.html)
- [GNU GPL FAQ on linking and combined programs](https://www.gnu.org/licenses/gpl-faq.en.html)
- [Zed issue #55470](https://github.com/zed-industries/zed/issues/55470)
