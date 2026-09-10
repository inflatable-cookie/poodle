# Installed web distribution smoke and certification

`effigy test:web-pack-install` is the permanent installed-package gate for the
compiled core, Svelte, and private React web distributions.

With `POODLE_WEB_PACK_INSTALL_SCOPE_MODE` unset, the selector is ordinary
installed-package smoke. It still clones the exact committed proof point into a
disposable checkout, builds and packs each package twice, and installs the
archives into a consumer with no workspace or source aliases. It checks archive
members, export targets, build receipts, CSS/parser edges, browser and SSR
lanes, the Svelte `5.56.8` floor, the visible `5.38.6` below-floor failure,
declarations under Bundler and NodeNext, and the frozen 176-name roster.
Ordinary source-only and empty `origin/main` ranges are valid. Workflow,
release, package-manager version, and registry/publish ranges fail before
build/pack. Ordinary Cargo classification is content-aware: `[package]` and
`[workspace.package]` version mutations stay forbidden, as do publication,
registry, source, `[patch]`, and `[replace]` content. Dependency requirements,
features, and lock resolution are not version surfaces. Ordinary JS
package-manifest classification is content-aware the same way: dependency and
export-map changes are accepted only while package version, package name, the
`private` publication flag, and the publication/registry transport fields
(`publishConfig`, top-level `registry`) remain byte-identical in effect.
Version or name mutations stay version surfaces; publication or transport
mutations stay registry surfaces. Unparsable, added, or deleted manifests
fail closed as version surfaces. Ordinary changelog maintenance is the single
content-aware release-path exception: the range must contain only
`CHANGELOG.md` and one dated Northstar execution log, both changelog revisions
must parse, Unreleased must remain empty, and versions, dates, reference links,
and normalized entry payloads must be identical. Ambiguous shapes, semantic
mutations, missing or extra logs, and mixed ranges remain forbidden release
surfaces. Ordinary runs emit no certification receipt or receipt hash.

Exact certification requires `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=strict` and
keeps the g16.059 writable allowlist, non-empty range, receipt bytes, and
receipt hash. Candidate certification remains
`POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g16.054-candidate`. Unknown modes reject.
Changed filenames never promote a run into certification.

The certification receipt has schema `poodle-installed-web-distribution`. Its
`sourceCommit`, package archive/build-receipt hashes, `artifactSetId`, and
`rosterNamesSha256` are deterministic. The receipt is evidence for an explicit
certification proof point, not a package file and not a release signal. This
gate does not publish, tag, or run windowed selectors.
