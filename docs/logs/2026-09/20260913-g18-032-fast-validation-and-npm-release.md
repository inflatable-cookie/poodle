# g18.032 fast validation and npm release — execution log

Date: 2026-09-13
Task: [`g18.032`](../../roadmaps/g18/032-fast-validation-and-npm-release.md)
Spec: [`071`](../../specs/071-fast-validation-and-npm-release-pipeline.md)

## Baseline evidence

The complete legacy board was run once under the g18.032 budget and exceeded
its fifteen-minute cap. The operator terminated only the owned process tree.
The named defect was reproduced: `probe:gpui-specimens` had no execution bound
and ran at ~100% CPU, and Effigy's composite nesting printed no live child
output, so the board appeared silent for the whole run.

This is the recorded baseline. The old board was not rerun; the retained
partial run is sufficient evidence for the two defects.

A parser-level inventory of the legacy `qa` graph (no execution) found the
transitive repeats the spec predicted:

- `core:build` executed four times through `ci:web` (`svelte:package`,
  `react:package`, `test:components`, direct);
- `test:web-pack-install` executed twice, once through `ci:web` and once from
  `qa`;
- the legacy aggregate also nested `ci`, `ci:web` and `ci:rust` directly.

## Changes

### Bounded, observable validation runner

`scripts/validation/run-board.ts` expands the declared Effigy task graph and
executes every owned unit exactly once. It:

- emits a `▶ start` line with the unit's bound and a `✔/✘ done` line with
  elapsed time for every child;
- hard-stops an individual child at its declared bound and kills its whole
  owned process group (children are spawned detached);
- hard-stops the board at fifteen minutes and reports the offending unit;
- removes transitive repeats and reports them as reused work;
- writes a `poodle.validation.board.v1` summary naming failures, reused work
  and the slowest leaves.

`quality/validation-bounds.json` declares the board and child ceilings, plus a
smaller three-minute bound for `probe:gpui-specimens`. `qa` now executes
`qa:board` through the runner and keeps its complete assertion inventory.

### Version-independent candidate admission

`test/package-install/web-candidate.ts` derives the source and target versions
from the compared commits and applies one version-independent law. It admits
synthetic `0.4.1` and `0.5.0` candidates and fails closed on partial bumps,
stale requirements, arbitrary source, workflow, registry and native changes,
missing release notes, misbound evidence and a second frozen release-input
commit. Changed-range admission is a separate selector from archive
certification.

### npm publication authority and archive certificate

`packages/release-manifest.json` now carries the npm publication set and the
candidate identity manifest name. The archive certificate builds and packs
exactly once and writes the tarballs plus their identity manifest. Publish mode
verifies tag, commit, version, package set and SHA-256 before any npm mutation
and publishes tarballs, never package directories.

### Hosted protocol

`.github/workflows/release.yml` retains one file with `candidate` and `publish`
modes: Linux, a ten-minute hard job timeout, no Rust/native/aggregate setup or
selector, artifact upload and run-ID download, and tag plus explicit
publish-mode guards. The root Effigy release gate is the npm certificate; the
aggregate board is never release authority.

## Local evidence

- Archive certificate (certificate mode, one build and one pack, source-free
  install): **55.9s** on the development machine, bounded at seven minutes by
  the runner.
  - `@inflatable-cookie/poodle-core@0.4.0`
    `inflatable-cookie-poodle-core-0.4.0.tgz`
    `eb7cbac0a345db9121794c11c543733a1598caac7e0cbae030708b84f04b4810`
  - `@inflatable-cookie/poodle-svelte@0.4.0`
    `inflatable-cookie-poodle-svelte-0.4.0.tgz`
    `0a5afe9e076215b06aaf4dafc4d77168014fd5cf17dc4fa8585e8c9ac98524fc`
  - source commit `b09ecfd604742a93efab8325b585e94b5565d959`
- `bun scripts/verify-npm-candidate.ts` accepted that manifest and rejected a
  byte-tampered tarball before any npm invocation.
- Focused laws: `bun test test/package-install/scope.test.ts` (66 pass),
  `bun test test/package-install/web-candidate.test.ts
  scripts/verify-npm-candidate.test.ts scripts/validation/run-board.test.ts`
  (27 pass), including a planted hang that is named, bounded and killed with
  its descendant process.
- `bun scripts/check-release-automation.ts` passed with eleven planted
  negatives (aggregate gate, Rust setup, macOS runner, missing timeout,
  directory publish, second Effigy entry, tag dry run, missing source-commit
  binding, missing archive upload, React publication authority and frozen
  release policy).

## Final board

Recorded below after the one optimized complete headless board run.
