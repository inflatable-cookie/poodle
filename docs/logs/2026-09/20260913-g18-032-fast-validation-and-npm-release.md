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

The one optimized complete headless board ran under its fifteen-minute cap and
executed 68 owned units with 5 transitive repeats removed. **54 units passed
and the board stopped at `probe:gpui-specimens`**, which exceeded its
explicit five-minute ceiling and was killed with its owned process group:

```
▶ [55/68] probe:gpui-specimens (bound 5m0s)
✘ probe:gpui-specimens killed after 5m0s (child bound)
board failed: 55/68 units in 10m34s
```

This is the spec 071 stop condition: one leaf alone exceeds five minutes and
fixing it needs product and native-probe behavior changes, which are outside
g18.032's boundaries. A focused run showed the probe's own tests still
"running for over 60 seconds" after compilation, so it is the pre-existing
runaway the baseline recorded (2h37m at ~100% CPU), not a validation-graph
defect.

The web lane passed in the same run: `test:web-pack-install` (1m4s),
`test:web-scope`, `gate:clean`, `ci:web` and every focused release law are
green. The slowest non-blocked units were `test:components` (1m42s),
`test:web-pack-install` (1m4s) and `test:web-scope` (59.0s).

## Hosted candidate drill

The one authorized non-publishing candidate drill ran on the exact pushed head
`96b959428704fd478eb5a66b24d6b72f1826adc1`:

- run `34754769895`, `workflow_dispatch`, mode `candidate`;
- job `npm web release (candidate)`, started 11:34:08Z, completed
  11:34:32Z (**24s**), conclusion `failure`;
- steps 1–10 passed: checkout, `origin/main` fetch, explicit-mode guard, Bun,
  Node, the reviewed npm CLI, and `bun install`;
- step 11 `npm web certificate` failed with **exit 127** because the rewritten
  workflow had dropped the reviewed `inflatable-cookie/setup-effigy` action,
  so `effigy` was not on `PATH`. Later steps were skipped, so no archive set
  or hashes were produced.

The dropped action is repaired and now lawed: `release.yml` must install the
pinned Effigy action (with the pinned version) before the certificate, and the
checker has a planted negative for removing it. The pinned `0.11.0` Effigy is
sufficient for every selector this lane uses; the bounded runner is repo-local.

A re-drill is required to observe the certificate step itself. On this
infrastructure head the generic admission is expected to reject the range
because the root version does not move (`0.4.0` -> `0.4.0`); a certifying drill
belongs to the next version candidate, not to this PR.

### Board-order correction

The first two attempts failed earlier, both on validation-graph defects this
task introduced and then fixed:

1. `test:core-build` failed because the git-plant `scope.test.ts` suite shared
   Bun's default five-second per-test timeout with parallel core-build files;
   it is now its own unit with an explicit thirty-second timeout.
2. `test:web-pack-install` failed because ordinary scope correctly rejected a
   changed `.github/workflows/release.yml`. The narrow `0.4.0` wrapper-repair
   admission is now backed by a structural release-automation admission: a
   range touching only release-automation surfaces is admitted while the
   workflow keeps the one npm certificate entry, Linux runner, ten-minute
   ceiling and run-ID identity protocol, and the checker keeps the spec 071
   invariants.
