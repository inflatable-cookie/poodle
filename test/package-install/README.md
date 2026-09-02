# Installed web distribution certification

`effigy test:web-pack-install` is the permanent installed-package gate for the
compiled core, Svelte, and private React web distributions.

The selector clones the exact committed proof point into a disposable checkout,
builds and packs each package twice, and installs the archives into a consumer
with no workspace or source aliases. It checks archive members, export targets,
build receipts, CSS/parser edges, browser and SSR lanes, the Svelte `5.56.8`
floor, the visible `5.38.6` below-floor failure, declarations under Bundler and
NodeNext, and the frozen 176-name roster.

The emitted receipt has schema `poodle-installed-web-distribution`. Its
`sourceCommit`, package archive/build-receipt hashes, `artifactSetId`, and
`rosterNamesSha256` are deterministic. The receipt is evidence for the
committed proof point, not a package file and not a release signal. This gate
does not publish, tag, or run windowed selectors.
