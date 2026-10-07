# Release

Poodle has two distribution paths under one lockstep version and tag. The
rules behind this procedure are in
[spec 071](../specs/071-fast-validation-and-npm-release-pipeline.md) (pipeline),
[spec 022](../specs/022-packaging-versioning-and-release-channel-rules.md)
(versioning and channels) and
[spec 044](../specs/044-deprecation-change-control-and-release-channel-operations.md)
(change control).

- **npm/web train** — root release version, `@inflatable-cookie/poodle-core`
  and `@inflatable-cookie/poodle-svelte`, published to npm. The private React
  package follows the same version but is never published. The publication set
  is `packages/release-manifest.json`.
- **Native train** — the Poodle Cargo crates stay `publish = false`; nothing
  goes to crates.io. Applications depend on one coordinated Poodle git tag,
  using the same tag protocol as the other inflatable-cookie Rust libraries
  (for example Longhorn). This follows operator ruling 2026-10-06 (decision
  08d56e2a). The npm and native packages move together in one candidate and
  one `vX.Y.Z` tag.

Versions stay `0.x`. Breaking changes may ship in a minor release and must be
called out in the changelog and release note.

Shared JavaScript library dependencies are declared as ranges, never exact
pins (operator ruling 2026-09-27), both by consumers and between libraries,
so a patch never forces a release elsewhere. Svelte and React require
`@inflatable-cookie/poodle-core` within the current minor, for example
`>=0.4.4 <0.5`, while the four web versions still move in lockstep. Native
Poodle path dependencies carry the shared exact crate version, and native
consumers pin that set by tag. Third-party runtime dependencies use ranges
unless a stated reason keeps one exact. Release admission requires these shapes.

`CHANGELOG.md` and `docs/release-notes/` are release surfaces. Ordinary PR CI
(`test:web-pack-install` scope) rejects any change to them, so feature and
dependency PRs describe their user-visible changes in the PR description, and
the release candidate PR writes them into the changelog. The one exception is
a changelog-only maintenance PR that changes nothing else.

`.github/workflows/` is a release surface too, but the Bun runtime isn't
pinned there. Every workflow installs Bun with
`bun-version-file: "package.json"`, so `packageManager` is the single pin, and
upgrading Bun means changing `packageManager` (plus `bun.lock`). Ordinary PR
scope admits a workflow change only when its sole changed lines are
setup-bun's `bun-version`/`bun-version-file` inputs; any other workflow edit
needs operator approval.

The root `package.json` is a release surface for its version and its
published shape, but not for development tooling: an ordinary PR may change
root `devDependencies` alone, since they ship in no published package
(operator ruling 2026-09-29).

Every release is operator-approved: releases and workflow dispatch are release
mutations (see [AGENTS.md](../../../AGENTS.md)). A failed release run is a
process failure, not a discovery: stop and return to planning.

## Steps (npm/web)

1. **Candidate PR.** On a branch, run `effigy release:bump X.Y.Z`. The target
   must be a greater pre-1.0 version than the current root version. This single
   selector updates root, core, Svelte and React manifests, current-minor core
   ranges, every Rust crate version and internal Poodle path pin, the Bun
   workspace lock slice, and the two tracked GPUI `Cargo.lock` files. It
   refuses a non-increasing version and does not refresh third-party lock
   resolutions. Add the `CHANGELOG.md` entry and one
   `docs/release-notes/<version>.md`, and list it in
   `docs/release-notes/README.md`. Nothing else may change except generated
   stamps and evidence that release policy admits. In a second
   commit, bind `docs/evidence/releases/v<version>-candidate.json`
   (`poodle.web-candidate-evidence.v1`): the freeze commit as
   `source_commit`, the base, the payload commits since the previous tag, the
   publication set, `native_train_changed: true`, the operator acceptance,
   and focused evidence for both distribution paths. The `0.4.2` candidate
   (`a28c99f44`, `d2438aef7`) is the web evidence reference.
2. **Local proof.** Run `effigy release:web-certificate` once on the stable
   candidate. It admits the changed range (`release:web-admission`), then
   builds, packs and source-free-installs one archive set
   (`release:web-archive`). Do not stack `qa`, `ci:web` or a tag dry run on top.
3. **Merge.** The candidate PR needs green required PR CI and review, then
   merges with operator approval.
4. **Candidate run.** Push `release/X.Y.Z` pointing at the candidate PR's head
   (the evidence commit), then dispatch `.github/workflows/release.yml` with
   `mode=candidate` on that branch. It re-runs the npm certificate and uploads
   the two tarballs plus an identity manifest (source commit, version, SHA-256
   values). Hard ceiling: ten minutes. Admission derives its base itself: the
   candidate's merge-base with `origin/main` before the merge, or, once the
   candidate has merged, the commit just before its own release-input and
   evidence commits (`deriveWebCandidateBase` in
   `test/package-install/web-candidate.ts`). So the run works before or after
   the merge; it never compares the release with itself.
5. **Tag.** Create an annotated `vX.Y.Z` tag on that same candidate commit.
6. **Publish run.** Dispatch `release.yml` with `mode=publish` on the tag and
   `candidate-run-id` set to the candidate run. It downloads that run's
   archives, verifies tag, commit, version, package names and hashes, then
   publishes those exact tarballs with npm trusted publishing. It never
   rebuilds.

## Steps (native crates)

The Rust crates are not published to crates.io. Their release is the same
immutable `vX.Y.Z` tag used by the npm release. All Cargo manifests and the
two committed GPUI locks are aligned to the current web version, `0.4.11`;
the existing `v0.4.11` tag is not moved or recreated. The first coordinated
release after that baseline must use a strictly greater version, such as
`0.4.12`.

1. **Freeze.** Use `effigy release:bump X.Y.Z` as part of the candidate's
   frozen release-input commit. The generic web-candidate admission requires
   every JS manifest, Cargo manifest, and tracked GPUI lock to carry the same
   source-to-target transition in that one commit. It admits only the version
   fields, internal dependency pins, and local Poodle lock entries; an
   unrelated Rust or lockfile change is rejected. Do not add a crates.io
   publication step or `publish = true`.
2. **Certify.** Run the existing web certificate and the native gates against
   the same frozen candidate: `effigy release:web-certificate`,
   `effigy ci:rust`, `effigy check:gpui`, `effigy regressions:native`,
   `effigy test:gpui-census`, `effigy check:gpui-census`, and
   `effigy release:gpui-consumer`. The consumer selector creates a disposable
   local bare repository, tags the exact candidate commit, and compiles a
   fresh consumer using `poodle-gpui-node-backend` plus the same GPUI crate
   identity. It does not create a tag in the working repository.
3. **Accessibility evidence.** Run `effigy test:gpui-ax` on a macOS host with
   Accessibility permission for its AXUIElement probe executable. If the
   candidate worker lacks that trust, the operator runs this gate during
   milestone QA on the exact frozen candidate commit. Record the commit and
   result in the candidate evidence; do not substitute a different head.
4. **Review and tag.** Require green PR CI, candidate evidence for every web
   and native gate, and operator approval. After merge, create the annotated
   `vX.Y.Z` tag on the certified candidate commit used by the npm candidate
   run. The existing workflow continues to certify and publish only the npm
   archives; the immutable tag is the Rust release. Never move or reuse a tag.

### Crates and consumer address

Every Cargo package remains `publish = false`. The tag-addressable public
contract crates are `poodle-tokens`, `poodle-events`, `poodle-headless`,
`poodle-layout`, `poodle-style`, `poodle-markdown`, `poodle-ir`,
`poodle-specs`, `poodle-adapter`, and `poodle-node`, plus `poodle-render`,
`poodle-gpui`, and `poodle-gpui-node-backend`. `poodle-jetstream` retains its
public-intent metadata for Jetstream consumers; it is not required by GPUI
applications. The internal-only packages are `poodle-codegen`,
`poodle-gpui-preview`, and `poodle-jetstream-preview`.

An application pins public packages from one release tag. For example:

```toml
[dependencies]
poodle-node = { git = "https://github.com/inflatable-cookie/poodle", tag = "vX.Y.Z", package = "poodle-node" }
poodle-gpui-node-backend = { git = "https://github.com/inflatable-cookie/poodle", tag = "vX.Y.Z", package = "poodle-gpui-node-backend" }
gpui = { package = "gpui-unofficial", version = "=1.22.0" }
```

The GPUI backend pins `gpui-unofficial =1.22.0`; the application must use that
same exact release so both sides share GPUI's Rust type identity. The native
packages are pre-1.0; breaking changes follow the repository's 0.x version
policy and ship only under a new coordinated tag.

If the tagged workflow wrapper itself fails before publication, a reviewed
wrapper-only repair may dispatch publish from `main` with `release-tag` set to
the existing tag. The same candidate run still supplies the archives; the tag
never moves.

## Verify

- The publish run's bounded registry check sees both packages at the new
  version. It retries reads only, never `npm publish`: up to 36 attempts, 10 s
  apart, each with `npm view --prefer-online`. npm can take minutes to expose a
  trusted publish, and without revalidation the first miss stayed cached
  (`0.4.2`, `0.4.4` and `0.4.5` went red after a successful publish). If the
  check still fails after "Publish the certified archives" succeeded, confirm
  with `npm view <package>@<version> version`. Never rerun publish for a
  version npm has accepted.
- A fresh, source-free consumer installs the published versions from the
  registry and resolves the public entry points (types, SSR and browser).
- `npm view @inflatable-cookie/poodle-svelte dist-tags` shows the new `latest`.

## Roll back

- Nothing published: fix forward on a new candidate. Retract the tag only when
  nothing was published from it.
- Published: npm versions are immutable. Ship a fixed patch release through the
  same steps; deprecate the bad version with `npm deprecate` if consumers must
  be warned. Never re-publish or move a tag.
