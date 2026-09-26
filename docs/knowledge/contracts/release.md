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

Every release is operator-approved: releases and workflow dispatch are release
mutations (see [AGENTS.md](../../../AGENTS.md)). A failed release run is a
process failure, not a discovery: stop and return to planning.

## Steps (npm/web)

1. **Candidate PR.** On a branch, bump root, core, Svelte and React to the same
   target version, update internal web dependency requirements and `bun.lock`,
   add the `CHANGELOG.md` entry and one `docs/release-notes/<version>.md`, and
   list it in `docs/release-notes/README.md`. Nothing else may change except
   generated stamps and evidence that release policy admits.
2. **Local proof.** Run `effigy release:web-certificate` once on the stable
   candidate. It admits the changed range (`release:web-admission`), then
   builds, packs and source-free-installs one archive set
   (`release:web-archive`). Do not stack `qa`, `ci:web` or a tag dry run on top.
3. **Merge.** The candidate PR needs green required PR CI and review, then
   merges with operator approval.
4. **Candidate run.** Dispatch `.github/workflows/release.yml` with
   `mode=candidate` on the exact merged candidate commit. It re-runs the npm
   certificate and uploads the two tarballs plus an identity manifest (source
   commit, version, SHA-256 values). Hard ceiling: ten minutes.
5. **Tag.** Create `vX.Y.Z` on that same commit.
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
  version. It retries reads only; it never repeats `npm publish`.
- A fresh, source-free consumer installs the published versions from the
  registry and resolves the public entry points (types, SSR and browser).
- `npm view @inflatable-cookie/poodle-svelte dist-tags` shows the new `latest`.

## Roll back

- Nothing published: fix forward on a new candidate. Retract the tag only when
  nothing was published from it.
- Published: npm versions are immutable. Ship a fixed patch release through the
  same steps; deprecate the bad version with `npm deprecate` if consumers must
  be warned. Never re-publish or move a tag.
