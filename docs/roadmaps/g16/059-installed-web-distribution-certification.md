# g16.059 — Installed Web Distribution Certification

Status: complete — merged in PR #163
Type: implementation — installed package oracle and promotion receipt
Opened: 2026-09-02
Depends on: accepted `g16.058`
Governing refs: `../../architecture/014-compiled-web-package-distribution.md`,
`056-web-distribution-contract.md`, `057-core-build-substrate.md`,
`058-shell-distributions.md`

Proof commit: `b37f404737062f9603c097dee0ee8b8916595d1f`
Worker PR: https://github.com/inflatable-cookie/poodle/pull/163
Merge: `536c9f7431d1095d48a91ce8d77613932f0e04ad`

## Goal

Make `test:web-pack-install` the sole permanent certification harness for the
compiled web boundary. Produce one accepted, exact-main receipt covering clean
builds, archives, browser and SSR consumption, declarations, CSS/parser
isolation, version-floor negatives, deterministic hashes, and the canonical
public roster. This is the prerequisite that can unblock `g16.054`.

## Fixed Boundary

- Certification runs from a clean temporary checkout of one exact commit.
- Core, Svelte, and private React are built and packed from clean staging;
  installed consumers use archive `file:` references and concrete peers.
- No workspace link, TypeScript path, source alias, sibling import, hand-edited
  tarball, preview app, suppressed diagnostic, or raw-source fallback counts.
- The harness owns both installed browser mount and server render. It also owns
  archive/export inspection, floor negative, declarations, CSS/marked graphs,
  repeated build/pack hashes, receipt membership, notices, and roster identity.
- This card does not change versions, edit release notes/history, tag, publish,
  dispatch workflows, mutate registries, or write sibling repositories.
- The declaration-tools manifest carries the repository-required exact `MIT`
  license metadata; dependencies and package behavior are unchanged.
- Merged-main baseline is red from a clean checkout in two linked places:
  Bun's shared module cache can make the nested TypeScript 6.0.3 declaration
  check observe root TypeScript 7, and docs/export audits run before shell
  `dist/` exists. This card owns the minimal resolver and selector-order repair;
  it must not weaken the 6.0.3 pin or permit stale artifacts.

## Ordered Work

1. Replace the source-oriented installed-pack baseline with clean staged builds
   of core, Svelte, and private React at one exact commit.
2. Inspect every export and wildcard target. Require compiled JavaScript and
   declarations; reject `.svelte`, non-declaration `.ts`/`.tsx`, maps, `src`,
   workspace metadata, source aliases, sibling paths, and missing targets.
3. Install archives into a fresh no-workspace consumer. Mount root/direct
   Button/Select and `./markdown` through browser conditions; render them via
   `svelte/server` under Node/default resolution; prove direct client output
   fails SSR.
4. Prove Svelte `5.56.8` browser/SSR operation and retain a visible below-floor
   negative such as `5.38.6`.
5. Compile root/direct/markdown/React declarations under Bundler and NodeNext.
   Retain HistoryEntry `continuationCount` positives, unsuppressed
   `branchCount` failures, Tree reorder positives, and four expected failures.
6. Assert exact sideEffects arrays, core style/token exports, focused CSS
   graphs, ordinary parser absence, and clear missing-`marked` behavior.
7. Build twice and pack twice. Compare every output hash, archive SHA-256,
   actual dotfile receipt membership, provenance, notices, artifact-set ID,
   exact source commit, and one derived 176-name roster denominator.
8. Record the accepted installed-distribution receipt without changing the
   certified package tree.

## Acceptance

- All installed browser, Node SSR, worker-like/default resolution, declaration,
  negative, CSS/parser, archive-content, and deterministic checks pass from the
  exact commit.
- Client output demonstrably cannot serve SSR. The declared Svelte floor works
  in both modes and the below-floor leg remains an expected visible failure.
- Archives contain only the contracted `dist` boundary and documents, including
  `package/dist/.poodle-build.json`; all exports resolve inside the archive.
- Two builds and packs are identical. Receipt provenance is stable and names
  the canonical roster denominator, artifact set, archive hashes, and commit.
- React remains private/validation-only. No release mutation occurs.

## Evidence

- The outer `effigy test:web-pack-install` selector ran from a disposable
  detached checkout of the exact proof commit. It performed two clean builds,
  two packs, archive-boundary/export/declaration/CSS/parser/notice checks,
  installed browser and Node SSR probes, default-resolution and direct-client
  SSR rejection, Svelte `5.56.8` floor success, the visible `5.38.6`
  below-floor failure, and the retained HistoryEntry/Slider/Tree proofs.
- Result: 11 test files and 22 tests passed. The receipt and all seven
  falsification receipts are recorded in the execution log.
- The scope proof derived 20 changed paths from required base
  `721b17791946a6a7dcf254de0775f353d47090eb`; all passed the writable
  allowlist, and a real committed workflow mutation was rejected by the same
  guard.
- Deterministic receipt identity: `5cb4b6d2f848e426de868fd56501b1d2c02c368f6ece7f2fb32692394c9f733c`.
- Receipt source commit: `b37f404737062f9603c097dee0ee8b8916595d1f`.
- Roster: 176 names; `f497bfa0a47e1627a1ee7076016ac5566d83584d458b3f3693b688885a02a84a`.
- Artifact set: `2d2fac98557cd50cc4e01d265405771401049060b88776cfe6d4627523903577`.

## Limits

- This worker has not merged, released, dispatched a workflow, run a windowed
  selector, or dispatched `g16.054`. The single authorized worker PR is
  [#163](https://github.com/inflatable-cookie/poodle/pull/163).
- The receipt certifies the committed proof point above. Subsequent
  documentation-only closeout commits must not be substituted for that receipt
  source commit without rerunning the installed oracle.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Installed archives, not source, are tested | consumer resolves workspace core | realpath/export check fails |
| Browser and SSR both work | only browser fixture passes | server oracle blocks receipt |
| Floor is truthful | 5.56.8 server render fails | floor leg blocks acceptance |
| Negative type proofs bite | `branchCount` compiles | unsuppressed fixture fails harness |
| Receipt matches immutable inputs | evidence edit changes package tree | certified tree hash changes |
| Roster is canonical | fixture remains 175 | denominator gate fails |
| Green certification is not a release | committed workflow/release/version/registry path appears | derived scope guard fails |

## Writable Scope

`test:web-pack-install` and its permanent fixtures/helpers, installed-content
and condition probes, deterministic receipt evidence, package-install docs,
this card, one execution log, and new papercuts. Only minimal package/build
repairs required to make the accepted architecture truthful may enter after
explicit review; do not redesign prior cards. Do not edit versions, changelog,
release notes/history, workflows, tags, registries, sibling repositories,
component behavior, or React publication state.

## Validation

Run the full upgraded `test:web-pack-install` oracle from a clean temporary
checkout, repeated build/pack/hash checks, relevant web build and declaration
selectors, CSS/parser and notices audits, roster drift checks, `effigy
docs:check`, one final headless `effigy qa`, and `git diff --check
origin/main...HEAD`. No release, workflow-dispatch, windowed, or native-visual
selector is authorized.

## Validation Result

- `effigy test:shell-build` from a cold nested declaration-tools cache: 31
  pass / 0 fail, 1,084 expects; the selector installed and validated
  TypeScript `6.0.3` before its parallel test workers started.
- `effigy test:web-pack-install`: 11 files / 22 tests passed from a clean
  detached checkout of the exact proof commit.
- Scope falsification: a real temporary two-commit repository changed
  `.github/workflows/release.yml`; the production guard rejected it.
- `effigy docs:check`: pass.
- `effigy qa`: pass. The full headless board passed release automation, web,
  Rust/headless, GPUI consumer, license, security, and advisory checks; license
  compliance reported 9 package manifests, 17 Cargo manifests, and 4 notice
  surfaces clean, with no vulnerabilities.
- `git diff --check origin/main...HEAD`: pass.

## Stop Conditions

Stop on any raw source/map/workspace resolution; missing export or declaration;
wrong client/server selection; client SSR success; Svelte floor failure;
suppressed or non-biting negative; CSS/parser/dependency drift; nonidentical
build or archive; nondeterministic/missing receipt; roster disagreement;
candidate-bearing mainline change during certification; compatibility shim;
or workflow/tag/registry/sibling mutation.

## Continuation

After accepted merge, the orchestrator may update `g16.054` to depend on the
completed exact-main receipt and dispatch its immutable `0.3.0` candidate lane.
`g16.054` must rerun the installed oracle against the candidate and remains the
sole owner of versions, release-history honesty, release notes, candidate
receipt, tags, registry checks, and later release authority. Completion here is
not a release and grants no publication authority.
