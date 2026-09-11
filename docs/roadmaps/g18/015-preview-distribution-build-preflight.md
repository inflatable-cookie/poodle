# 015 — Preview distribution build preflight

Status: complete — merged as `0cf6073eb2067c4fc4127ec5c318a0f682c5859f` (PR #243) on 2026-09-11
Owner: Poodle developer tooling
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../../tasks/effigy.tasks.toml`,
`../../../scripts/web-distribution/core-build.ts`,
`../../../scripts/web-distribution/svelte-build.ts`,
`../../../scripts/web-distribution/react-build.ts`
Depends on: none

## Outcome

Make `effigy svelte:preview` and `effigy react:preview` fail-safe against stale
ignored package distributions. Each public preview selector must rebuild core
and its framework package successfully before Vite begins listening, so a fresh
or newly merged checkout cannot load component code against an older
`packages/*/dist` export surface.

This is startup correctness. Do not turn it into release automation or a broad
development-process redesign.

## Ready-State Rubric

- [x] PR #242 added `installCodeEditorFocusEntry` to core source and both
  component engines.
- [x] The running Svelte preview loaded its component distribution before core
  `dist` had rebuilt and threw a missing named-export error.
- [x] The current preview selectors call only their Vite `*:run` tasks; package
  builders already exist as `core:build`, `svelte:package`, and
  `react:package`.
- [x] The current served core distribution exports the helper after a later
  build, proving ordering rather than public-source omission.
- [x] The operator approved the permanent repair.

## Decisions

- `svelte:preview` runs the full Svelte package build before `svelte:run`;
  `react:preview` runs the full React package build before `react:run`.
  Framework package tasks already compose core first and remain the authority.
- A failed preflight prevents Vite startup. Do not start against the prior
  ignored output and print a warning.
- Bind the ordering and outcome, not merely task-manifest text. A regression
  must fail when source exports a symbol consumed by the framework package but
  stale core `dist` lacks it.
- Test both missing and stale distributions from an isolated fixture or safely
  preserved disposable copies. Never delete or overwrite the operator’s live
  ignored distributions as a test setup.
- Prove the served module graph after startup, including one named export that
  exists only after the preflight build. Use an ephemeral port and terminate
  every test-owned Vite process.
- Continuous package watch after startup is outside this bounded repair. Record
  it as a follow-up only if current preview development cannot observe ordinary
  source edits through the established build workflow.
- Keep raw `svelte:run` and `react:run` as low-level commands if other tooling
  needs them; the documented public preview selectors own safe startup.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch in parallel with g18.013;
  serial before held g18.011 and retained g18.006; g18.009 remains held
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** preview/package task composition in
  `tasks/effigy.tasks.toml`; focused task-runner/distribution startup fixtures,
  scripts and tests; preview operator documentation if needed; one g18.015 log
- **Reserved closeout surfaces:** component/runtime implementation and public
  API; package versions/manifests/lockfile unless a test fixture requires its
  own isolated manifest; g18 README/index/dispatch; g18.006/g18.009 and
  g18.011–g18.014 task/workspace/PR state; workflows; release/tag/publication;
  Desktop; native/GPUI/Jetstream
- **Worker:** developer-tooling worker comfortable with Effigy task graphs,
  package distributions, Vite startup, subprocess cleanup and isolated tests
- **Excluded:** continuous watch architecture; workflow changes; release
  automation; package API changes; dev-server feature work; port policy changes
- **Escalation:** Chatterbox for a required workflow/release mutation, inability
  to preserve live ignored output, need to replace the public selectors, or a
  broader watcher/process-manager redesign

## Work

1. Reproduce the failure with an isolated stale core distribution missing a
   named export consumed by a newly built framework editor entry. Show that the
   current preview selector can start Vite and serve the mismatched graph.
2. Compose `svelte:preview` through `svelte:package` before `svelte:run`, and
   `react:preview` through `react:package` before `react:run`, using existing
   Effigy task semantics rather than a shell background wrapper.
3. Prove missing/stale output is rebuilt before the listener becomes ready and
   build failure leaves no server or owned child process.
4. Start each public selector on an ephemeral port, request the served module
   graph, and verify the planted named export imports without a browser syntax
   error. Stop all test-owned processes and prove port/process cleanup.
5. Confirm low-level run selectors remain explicit and document that safe
   operator startup uses the public preview selectors.
6. Run focused task/distribution tests, both preview builds, Effigy docs QA,
   and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Core is fresh before framework build | component dist imports a source export absent from core dist | planted stale-export case is rebuilt before framework packaging |
| Framework dist is fresh before Vite | Vite starts with old `editor.js` despite fresh source | served module contains the newly planted framework import |
| Both previews are protected | only Svelte composes package preflight | equivalent task graph and live startup proof for Svelte and React |
| Failure is fail-closed | build fails but Vite still listens using prior output | forced builder failure, no listener, no owned child process |
| Test does not damage live output | regression test deletes the operator’s working `dist` | isolated/disposable paths and before/after live-output identity proof |
| Proof reaches the server | tests only parse TOML and never import served modules | HTTP/browser module import from each public selector |
| Startup does not leak | ephemeral Vite process remains after test | process/port cleanup assertion on pass and planted failure |
| Public route is clear | docs encourage raw `*:run` and bypass preflight | selector inventory/operator text names `*:preview` as safe entry |
| Scope stays bounded | fix becomes a new watcher or release pipeline | diff limited to startup composition and focused proof |
| Sweep remains gated | g18.011 begins while previews can serve stale packages | merged repair before Queue hold release |

## Stop conditions

- Stop if safe proof requires mutating the operator’s live ignored
  distributions or killing an unowned preview process.
- Stop if the repair requires `.github/workflows/`, package version, release,
  or public component API changes.
- Stop before implementing continuous-watch architecture without a separate
  operator decision.

## Evidence

On 2026-09-11 the Svelte Vite server started at 09:16:12 while
`packages/core/dist/index.js` was still stale. The distribution rebuilt at
09:17:43 and then served `installCodeEditorFocusEntry` from the same `@fs` URL.
`tasks/effigy.tasks.toml` defined `svelte:preview` and `react:preview` as direct
aliases of their run tasks even though safe package builders already existed.

## Next task

After this task, g18.013, g18.014 and parallel g18.016/g18.017 merge, Chatterbox
releases held g18.011 for the full three-surface acceptance sweep. Keep g18.006
blocked and g18.009 held.
