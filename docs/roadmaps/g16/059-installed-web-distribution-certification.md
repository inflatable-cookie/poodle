# g16.059 — Installed Web Distribution Certification

Status: blocked — requires accepted `g16.058`
Type: implementation — installed package oracle and promotion receipt
Opened: 2026-09-02
Depends on: accepted `g16.058`
Governing refs: `../../architecture/014-compiled-web-package-distribution.md`,
`056-web-distribution-contract.md`, `057-core-build-substrate.md`,
`058-shell-distributions.md`

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

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Installed archives, not source, are tested | consumer resolves workspace core | realpath/export check fails |
| Browser and SSR both work | only browser fixture passes | server oracle blocks receipt |
| Floor is truthful | 5.56.8 server render fails | floor leg blocks acceptance |
| Negative type proofs bite | `branchCount` compiles | unsuppressed fixture fails harness |
| Receipt matches immutable inputs | evidence edit changes package tree | certified tree hash changes |
| Roster is canonical | fixture remains 175 | denominator gate fails |
| Green certification is not a release | tag/workflow/npm change appears | scope gate fails |

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
