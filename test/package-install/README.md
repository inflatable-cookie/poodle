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

Ordinary CI carries no candidate environment variable. When a range still
contains release-bearing changes after the content-aware checks above — a
forbidden version or release surface, or any staged release note — ordinary
scope admits it only when the complete base-to-head diff satisfies the closed
`0.3.0` → `0.4.0` g18.006 policy described below. Partial, wrong-version,
retargeted, transport, lone release-note, unrelated and unbound-evidence
ranges stay ordinary rejections, and a closed admission never emits a
certification receipt.

Exact certification requires `POODLE_WEB_PACK_INSTALL_SCOPE_MODE=strict` and
keeps the g16.059 writable allowlist, non-empty range, receipt bytes, and
receipt hash. The historical
`POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g16.054-candidate` policy is preserved
unchanged: it keeps its `0.2.3` → `0.3.0` proof, its direct one-commit
candidate tree, its writable set and its content rules. The closed
`POODLE_WEB_PACK_INSTALL_SCOPE_MODE=g18.006-candidate` policy admits only the
exact `0.3.0` → `0.4.0` release: the complete lockstep JS and Cargo
manifests, intra-repository requirements, tracked locks, changelog, `0.4.0`
release notes, generated version stamps, the complete Nucleus
receipt/ledger/GPUI-census cohort and the fixed g18.006 execution record. It
requires exactly one frozen release-input commit, permits only an
evidence/log-only suffix, keeps the final release inputs byte-identical to
that frozen commit, binds changed evidence `source_commit` values to it, and
rejects component source, workflows, publish/registry transport, React
admission, Cargo retargeting and arbitrary documentation. Unknown modes
reject. Changed filenames never promote a run into certification.

The certification receipt has schema `poodle-installed-web-distribution`. Its
`sourceCommit`, package archive/build-receipt hashes, `artifactSetId`, and
`rosterNamesSha256` are deterministic. The receipt is evidence for an explicit
certification proof point, not a package file and not a release signal. This
gate does not publish, tag, or run windowed selectors.
