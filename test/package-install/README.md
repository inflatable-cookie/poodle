# Installed web distribution smoke, admission and archive certification

`effigy test:web-pack-install` is the permanent installed-package gate for the
compiled core, Svelte, and private React web distributions. The npm release
lane (spec 071) is separate: it classifies a changed range and certifies one
archive set, then publishes those exact bytes.

## Ordinary smoke

With `POODLE_WEB_PACK_INSTALL_SCOPE_MODE` unset, the selector is ordinary
installed-package smoke. It clones the exact committed proof point into a
disposable checkout, builds and packs each package twice, and installs the
archives into a consumer with no workspace or source aliases. It checks archive
members, export targets, build receipts, CSS/parser edges, browser and SSR
lanes, the Svelte `5.56.8` floor, the visible `5.38.6` below-floor failure,
declarations under Bundler and NodeNext, and the frozen 176-name roster.
Ordinary source-only and empty `origin/main` ranges are valid. Workflow,
release, package-manager version, and registry/publish ranges fail before
build/pack. Ordinary Cargo classification is content-aware: `[package]` and
`[workspace.package]` version mutations stay forbidden, as do publication,
registry, source, `[patch]`, and `[replace]` content. Ordinary JS
package-manifest classification is content-aware the same way. Ordinary
changelog maintenance is the single content-aware release-path exception.
Ordinary runs emit no certification receipt or receipt hash.

## npm web-candidate admission (generic)

`effigy release:web-admission` runs `test/package-install/web-admission.ts`,
which applies the version-independent law in `web-candidate.ts`:

- the target is a greater pre-1.0 semantic version of the base;
- root, core, Svelte and the private paired React manifest move in exact
  lockstep;
- internal web dependency requirements and `bun.lock` resolve at the target;
- the changelog and one matching `docs/release-notes/<target>.md` describe it;
- exactly one commit changes the derived release-input set, and every later
  change is a generated/evidence/execution-record surface bound to it;
- every changed path is a release input or a declared evidence family.

Nothing in this law names a generation, task, source or target version. A
future `0.4.1` or `0.5.0` candidate is admitted by the same code. Partial,
stale, source, workflow, registry, native and lockstep failures name the
specific violated rule and fail closed before any build.

`effigy test:web-candidate` runs the focused laws, including synthetic
`0.4.1`/`0.5.0` admission and the fail-closed range negatives.

## Archive certificate

`effigy release:web-archive` is certificate mode
(`POODLE_WEB_PACK_INSTALL_ARCHIVE_CERTIFICATE=1`). It builds and packs exactly
once, installs those same archives into the source-free consumer, and writes
the candidate identity manifest named by `packages/release-manifest.json`
(`poodle-npm-candidate.json`) plus the tarballs into
`POODLE_WEB_PACK_INSTALL_ARCHIVE_OUT`. The manifest records the source commit,
the lockstep version, each package path from the release authority, and each
tarball's SHA-256.

`effigy release:web-certificate` runs admission, then the archive certificate.
It is the one Effigy entry the hosted candidate mode invokes and the root
`[release.gates.headless]` authority. Aggregate `qa` is never release
authority.

`bun scripts/verify-npm-candidate.ts` verifies a downloaded archive set
before any npm mutation: schema, tag version, source commit, the exact package
set from the release authority, each tarball's bytes against the manifest, and
the packed `package/package.json` identity. Publish mode consumes the candidate
run's bytes by ID and never rebuilds.

## Historical certification modes

`POODLE_WEB_PACK_INSTALL_SCOPE_MODE=strict` keeps the g16.059 writable
allowlist, non-empty range, receipt bytes, and receipt hash. The historical
`g16.054-candidate` and `g18.006-candidate` policies are preserved unchanged
as evidence of the `0.3.0` and `0.4.0` releases; active release policy does not
name them. Unknown modes reject. Changed filenames never promote a run into
certification.

The certification receipt has schema `poodle-installed-web-distribution`. Its
`sourceCommit`, package archive/build-receipt hashes, `artifactSetId`, and
`rosterNamesSha256` are deterministic. The receipt is evidence for an explicit
certification proof point, not a package file and not a release signal. This
gate does not publish, tag, or run windowed selectors.
