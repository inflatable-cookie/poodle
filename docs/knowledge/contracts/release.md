# Release

Poodle has two release trains. The rules behind this procedure are in
[spec 071](../specs/071-fast-validation-and-npm-release-pipeline.md) (pipeline),
[spec 022](../specs/022-packaging-versioning-and-release-channel-rules.md)
(versioning and channels) and
[spec 044](../specs/044-deprecation-change-control-and-release-channel-operations.md)
(change control).

- **npm/web train** — root release version, `@inflatable-cookie/poodle-core`
  and `@inflatable-cookie/poodle-svelte`, published to npm. The private React
  package follows the same version but is never published. The publication set
  is `packages/release-manifest.json`.
- **Native train** — Cargo crates, distributed from source and tags. It has no
  release procedure or tag authority yet; an npm candidate never bumps Cargo
  manifests, locks, GPUI receipts or native evidence.

Versions stay `0.x`. Breaking changes may ship in a minor release and must be
called out in the changelog and release note.

Shared libraries are declared as ranges, never exact pins (operator ruling
2026-09-27), both by consumers and between libraries, so a patch never forces
a release elsewhere. Svelte and React require `@inflatable-cookie/poodle-core`
within the current minor, for example `>=0.4.4 <0.5`, while the four web
versions still move in lockstep. Third-party runtime dependencies use ranges
unless a stated reason keeps one exact. Release admission requires that
range shape.

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

1. **Candidate PR.** On a branch, bump root, core, Svelte and React to the same
   target version, retarget the current-minor core ranges, refresh `bun.lock`
   workspace versions and intra-repo ranges with `effigy lock:workspaces-refresh`
   (Bun 1.4.2 leaves that slice stale through `bun install`, `--force` and
   `--lockfile-only`; `--frozen-lockfile` still passes),
   add the `CHANGELOG.md` entry and one `docs/release-notes/<version>.md`, and
   list it in `docs/release-notes/README.md`. Nothing else may change except
   generated stamps and evidence that release policy admits. In a second
   commit, bind `docs/evidence/releases/v<version>-candidate.json`
   (`poodle.web-candidate-evidence.v1`): the freeze commit as
   `source_commit`, the base, the payload commits since the previous tag, the
   publication set, `native_train_changed`, the operator acceptance and the
   focused evidence. The `0.4.2` candidate (`a28c99f44`, `d2438aef7`) is the
   reference.
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
