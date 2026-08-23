# g15.054 — v0.2.1 Release Recovery Receipt

Date: 2026-08-23
Verdict: **green replacement candidate; operator gate required**
Candidate: `3d914261c621ef1184d42d7182f7530586c8b267`

## Failed Release Boundary

The operator authorised `v0.2.0` at `7922a3a9`. GitHub Actions run
[`32656225297`](https://github.com/inflatable-cookie/poodle/actions/runs/32656225297)
failed in `Install the reviewed npm trusted-publishing CLI` while
`npm install --global npm@12.0.2` replaced the npm tree executing the command.
The running CLI could no longer load `promise-retry`.

Every later step was skipped: dependency install, release gates, version
agreement, packing, publication, and artifact upload. Registry checks after the
run showed core and Svelte still at `0.1.0`; React remained absent. The
`v0.2.0` tag remains immutable at `7922a3a9` and is not reused.

## Recovery

- The workflow installs npm `12.0.2` under `$RUNNER_TEMP/npm-cli`, verifies the
  explicit binary, and appends its bin directory to `$GITHUB_PATH` for later
  steps. A local isolated-prefix reproduction returned `12.0.2`.
- `check-release-automation` now requires that isolated shape and rejects
  `npm install --global npm@...`.
- All 20 release-bearing manifests, intra-repository requirements, tracked
  lockfiles, and generated version stamps moved to `0.2.1`.
- `0.2.0` notes now record the failed pre-publication tag; `0.2.1` is the
  replacement registry release with the same product payload.

No component, token, contract, renderer behavior, or publish-set boundary
changed. Core and Svelte remain the only npm publication targets; React stays
packed and source-only.

## Validation

All commands ran from the content committed as candidate `3d914261`.

| Command | Result |
| --- | --- |
| isolated npm `12.0.2` prefix install and explicit binary check | pass |
| `bun install --frozen-lockfile` | pass; no changes |
| 20-manifest `0.2.1` agreement and Poodle lock-block check | pass |
| `effigy check:release-automation` | pass |
| `effigy ir:check` | pass |
| `effigy catalogue:check` | pass |
| `effigy docs:check` | pass |
| `effigy qa` | pass |
| `effigy release gates` | pass; 1 configured, 1 executed, headless 220937 ms |
| `git diff --check` | clean |
| local/remote `v0.2.1` collision check | absent |
| npm `0.2.1` collision check for core and Svelte | absent; registry contains only `0.1.0` |

No windowed, native-visual, or Jetstream selector ran locally.

## Packed Artifacts

The publication path used the isolated npm `12.0.2` binary. Repacking with the
machine-default npm `10.9.8` produced identical bytes and digests.

| Artifact | Entries | Bytes | SHA-256 |
| --- | ---: | ---: | --- |
| `inflatable-cookie-poodle-core-0.2.1.tgz` | 393 | 268952 | `3e9d74d60a00e40fb0bbaac3ed0863799d3df2da48a2b808975c4815b454e0da` |
| `inflatable-cookie-poodle-svelte-0.2.1.tgz` | 214 | 253582 | `a13054beb302aee11af1a2b1c1679b512a7c1a0b3e244fb76c804d57271f4fff` |
| `inflatable-cookie-poodle-react-0.2.1.tgz` | 208 | 250072 | `b58a39c220eed9982d6aaf8a33d67531abd7c8095e304836c89b7c3652a0a350` |

Every tarball contains `LICENSE`, `README.md`, and `package.json`. Core also
contains 108 icon modules, its generated alias map, and 22 token stylesheets.
Packed README links are absolute HTTPS links only.

The independent `bun pm pack` path inside `effigy qa` produced:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| core | 262876 | `8f2042f69e4626d821cabb92ca99511d00c6df3caa36b95888f013919ceabc74` |
| Svelte | 253452 | `8f11450f5e140034b5605fc53367a926d3fd006ecd91a0e3eefb15f42f6f8ca3` |
| React | 249857 | `f0c7d80b1cdf0bed463201d272fb7b2a570df701261f97bc3a7594204a9b5b6b` |

The packed-install proof remained 175/175 for both web runtimes, with 9 Svelte
and 13 React components mounted from clean installed roots.

## Continuation

`g15.054` is complete. `g15.013` is the only remaining gate. The operator must
review candidate `3d914261`, confirm npm Trusted Publisher remains configured
for both packages and `release.yml`, then explicitly authorise creation and
publication of `v0.2.1`. Any candidate-bearing change invalidates this receipt
and requires a new patch candidate plus the complete evidence rerun.
