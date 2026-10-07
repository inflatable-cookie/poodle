# 022 Packaging Versioning And Release Channel Rules

Status: active
Updated: 2026-10-07
Depends on: `archive/021-public-package-api-stability-and-parity-debt-baseline.md`

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
- breaking changes must still be called out in release notes, the changelog,
  and any downstream migration material

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

The private root package version is repository release metadata and moves in
lockstep with the web release versions. Tom ruled on 2026-09-12 that it must
stay current even though it is unpublished, because Effigy reads it as the
repository version and a stale value is misleading.

### Preview Channel Public-Intent Packages

- `@inflatable-cookie/poodle-core`
- `@inflatable-cookie/poodle-svelte`
- Rust contracts: `poodle-adapter`, `poodle-events`, `poodle-headless`,
  `poodle-ir`, `poodle-layout`, `poodle-markdown`, `poodle-node`,
  `poodle-specs`, `poodle-style`, and `poodle-tokens`
- Rust renderers: `poodle-render`, `poodle-gpui`,
  `poodle-gpui-node-backend`, and `poodle-jetstream`

The web public release set is core and Svelte only. See
[compiled web package distribution](../architecture/014-compiled-web-package-distribution.md)
and [compiled web distribution contract](070-compiled-web-distribution-contract.md).

### Compiled private validation package

- `@inflatable-cookie/poodle-react` is compiled and certified to the same
  `dist` boundary as the public web packages, but stays private and unpublished
  until a named consumer is admitted. `packages/release-manifest.json` still
  records React as preview `publicIntent`; that metadata mutation belongs to
  the release-candidate lane after installed certification, not to package
  publication.

### Internal Packages

- `@inflatable-cookie/poodle-tokens`
- `@inflatable-cookie/poodle-svelte-preview`
- `@inflatable-cookie/poodle-react-preview`
- `@inflatable-cookie/poodle-install-smoke`
- `poodle-gpui-preview`
- `poodle-jetstream-preview`

## Consumption Rule

Downstream repos should only plan around preview-channel public-intent
packages. The web public set is core and Svelte.

They should not depend on:

- `@inflatable-cookie/poodle-react` until a named consumer is admitted
- internal source-of-truth token build packages
- bridge packages
- preview/docs tooling packages

## Release Note Rule

Each release-capable tranche should document:

- which packages changed
- whether the change affects public-intent entry points
- whether the change is additive, behavioral, or breaking
- what downstream evaluators should re-check

## Lockstep Release Rule

The npm packages and native Cargo crates share one pre-1.0 version and one
immutable `vX.Y.Z` tag. A candidate moves the root, core, Svelte, private React
validation package and every Rust crate together; internal npm ranges and Rust
path pins, `bun.lock`'s workspace slice and the committed GPUI lock entries
move with them. Core and Svelte are the only npm publications. Cargo crates
remain `publish = false` and are consumed from the Poodle git tag; nothing is
published to crates.io. See
[`release.md`](../contracts/release.md) for the candidate gates and tag steps,
and [`071-fast-validation-and-npm-release-pipeline.md`](071-fast-validation-and-npm-release-pipeline.md)
for the npm workflow boundary.

## Native Dependency Licence And Source Rule

Public-intent native packages may ship under Poodle's permissive release
posture only when their resolved normal dependency graph is compatible with
that distribution claim.
Poodle therefore applies these rules to every GPUI release graph:

- GPL-3.0-or-later dependencies are not admitted. Do not make a release gate
  green by adding a crate exception for `zlog`, `ztracing`,
  `ztracing_macro`, or another strong-copyleft dependency.
- A notice-bearing licence is carried in the distributed notice surface for
  exactly as long as its crate is in the resolved graph. The notice surface
  describes the current graph; it is not an append-only archive, and a notice
  for a crate that has left is a false claim about what Poodle distributes.
  `audit:licenses` enforces both directions against the lockfiles: a missing
  notice fails, and so does a retained claim no lockfile resolves.
- GPUI 1.22.0 resolves `libbz2-rs-sys` under `bzip2-1.0.6`. That existing
  operator-approved licence is allowed in `deny.toml`, with notices at the
  repository root and beside the public-intent GPUI node backend.
- Remote Git sources are denied by default. An approved source must use a
  reviewed repository URL and an immutable full commit revision. Branches,
  tags, moving refs, and unreviewed repositories remain forbidden.
- Public-intent Poodle packages must consume GPUI from its crates.io release.
  A preview, capture harness, test helper, or other internal tool may not
  choose a fork or Git source for the GPUI types exposed to consumers.
- `cargo-deny` keeps `unknown-git = "deny"` and
  `required-git-spec = "rev"`. Repository security checks independently pin
  the expected URL and revision in manifests and lockfiles.
- A release candidate must prove that the normal dependency graph for every
  public-intent GPUI package contains no GPL dependency and that the licence,
  source, notice, and repository-security audits pass.

`v0.2.1` temporarily used a narrow `inflatable-cookie/zed` fork to remove GPL
tracing from a newer unpublished GPUI revision needed by the offscreen capture
tool. Longhorn adoption proved that this changed the public GPUI crate identity
and made Poodle's GPUI types incompatible with a consumer's crates.io GPUI
types. That source shape is rejected for public packages and is removed by
`g15.059`.

GPUI 1.22.0 has no true offscreen pixel-readback API. Poodle therefore keeps
its default native evidence on GPUI's in-memory test platform and treats real
pixel capture as an explicit, non-activating window-server diagnostic. The
diagnostic stays outside default QA, CI, and release gates. A future
true-offscreen route must arrive through a crates.io GPUI release or a new
explicit public-dependency decision; it cannot silently reintroduce a fork.

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
- [GNU GPL FAQ on linking and combined programs](https://www.gnu.org/licenses/gpl-faq.en.html)
- [Zed issue #55470](https://github.com/zed-industries/zed/issues/55470)
- `docs/knowledge/research/gpui-cratesio-nonactivating-capture.md`
