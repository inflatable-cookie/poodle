# g16.094 — Ordinary Cargo Scope Classification Repair

Status: complete — merged in PR #200 as `f7ae38d9f7e644de6d39de43363dd77bbf75f842`
Type: installed-smoke routing repair
Opened: 2026-09-04
Depends on: merged `g16.061`; reproduced ordinary-mode rejection on PR #199
Governing refs: `059-installed-web-distribution-certification.md`, `061-installed-web-smoke-certification-routing.md`, `092-native-consumer-tinyvec-build.md`
Handoff: `../../handoffs/20260904-001500-g16-094-ordinary-cargo-scope-classification.md`
Proof commits: `9b14cf5ed` (plants), `3f9ef94d5` (classifier)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/200
Execution log: `../../logs/2026-09/20260904-g16-094-ordinary-cargo-scope-classification.md`

## Goal

Let ordinary installed-package smoke accept repository-owned dependency-only
Cargo manifest and lock changes while still rejecting actual package-version,
publication, registry, release, and workflow mutations before build or pack.
Strict and `g16.054-candidate` certification semantics remain unchanged.

## Fixed Boundary

- Reproduce unset/ordinary mode rejecting PR #199 solely because changed paths
  include `Cargo.toml` and `Cargo.lock`.
- In ordinary mode only, classify Cargo release-version surfaces from parsed
  content/diff context: package or workspace package `version` mutations remain
  forbidden. Dependency requirements, features, and lock resolution are not
  package release versions merely because their files or inline tables contain
  the word `version`.
- Publication, registry/source replacement, patch/replace, workflow, release,
  package-manager package version, and other existing forbidden surfaces must
  still fail before build/pack.
- Do not change strict or candidate allowlists, receipt production, source-child
  laws, release automation, component code, public APIs, or Nucleus evidence.
- Commit biting ordinary-mode counterexamples before the bounded router repair.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Dependency repair is ordinary | add a dependency feature and refresh Cargo.lock | ordinary route reaches installed smoke |
| Package version stays forbidden | mutate `[package] version` | reject before build/pack |
| Inline dependency version is not release version | change `{ version, features }` under dependencies | classify by owning section, not word search |
| Publication remains forbidden | add `publish`, `registry`, `source`, `[patch]`, or `[replace]` | reject before build/pack |
| Certification is unchanged | run strict/candidate falsifications | exact prior outcomes remain |

## Validation

Run focused router tests and plants through the production scope function,
`effigy test:web-pack-install`, `effigy ci:web`, `effigy docs:check`, and `git
diff --check origin/main...HEAD`. Never run release, workflow, windowed, or
native-visual selectors.

## Validation Result

- Focused plants: `effigy test:core-build` — 51 pass / 0 fail.
- `effigy test:web-pack-install`: 11 files / 22 tests; `mode: ordinary`; no
  receipt hash. Ordinary Cargo version/publication plants reject before
  build/pack; dependency/feature/lock plants are in `scope.test.ts`.
- `effigy ci:web`: pass.
- `effigy docs:check`: pass.
- `git diff --check origin/main...HEAD`: pass.

## Continuation

Rebase PR #199 and require its full `ci:web` to pass before merging g16.092.
No certification re-stamp is required.
