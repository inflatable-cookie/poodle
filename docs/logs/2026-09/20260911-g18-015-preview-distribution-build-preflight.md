# g18.015 — Preview distribution build preflight

Status: merged
Merge: `0cf6073eb2067c4fc4127ec5c318a0f682c5859f` (PR #243) on 2026-09-11
Date: 2026-09-11
Card: `docs/roadmaps/g18/015-preview-distribution-build-preflight.md`
Handoff: `docs/handoffs/20260911-g18-015-preview-distribution-build-preflight.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`tasks/effigy.tasks.toml`,
`scripts/web-distribution/core-build.ts`,
`scripts/web-distribution/svelte-build.ts`,
`scripts/web-distribution/react-build.ts`
Branch: `ns-dac68def-76a0-49af-b175-084548597957`
Base: `origin/main` at `bf02d773b78e7537c6380df92ef296b3435d398e`

## Outcome

`effigy svelte:preview` and `effigy react:preview` rebuild core and the matching
framework package before Vite listens. A failed package build never starts the
dev server. Raw `svelte:run` / `react:run` stay low-level Vite entry points.

## What changed

- `svelte:preview` = `svelte:package` then `svelte:run`. `react:preview` =
  `react:package` then `react:run`. Existing package tasks still compose
  `core:build` first.
- `docs:dev` now uses `svelte:preview`, so the documented docs path gets the
  same preflight.
- Isolated fixture proof in
  `scripts/web-distribution/preview-distribution-preflight.test.ts`: missing
  and stale core dist, planted `installCodeEditorFocusEntry` mismatch, live
  HTTP `@fs` module import, fail-closed planted builder error, process/port
  cleanup, live ignored-dist identity unchanged.
- Operator text names `*:preview` as the safe entry and `*:run` as low-level.

## Review oracle

| Invariant | Plant | Result |
| --- | --- | --- |
| Core is fresh before framework build | isolated core dist missing `installCodeEditorFocusEntry` | public preview rebuilds the export before listen |
| Framework dist is fresh before Vite | planted `g18_015_stale_framework_dist` editor file | served `editor.client.js` / `editor.js` lose the marker and import the export |
| Both previews are protected | Svelte and React selectors | same task graph and live HTTP proof |
| Failure is fail-closed | planted `core-build.ts` throw | non-zero exit, no listener, no owned child |
| Test does not damage live output | before/after fingerprint of worker `packages/*/dist` | identity match |
| Proof reaches the server | fetch `@fs` engine + core + editor, named import of served bytes | mismatch on `*:run`, present after `*:preview` |
| Startup does not leak | ephemeral port + process-group stop | port free and pid dead on pass and planted failure |
| Public route is clear | README, preview README, docs README, `docs:dev` | `*:preview` is the safe entry |

## Validation

- `bun test scripts/web-distribution/preview-distribution-preflight.test.ts`:
  2 pass / 0 fail
- `effigy svelte:package`, `effigy svelte:build`, `effigy react:package`,
  `effigy react:build`: clean
- `effigy docs:lint`: pass
- `git diff --check`: pass

No `release prepare/execute`, tag, publish, workflow edit, windowed selector,
or native-visual selector was run.

## Limits

This worker has not merged, released g18.011, or resumed g18.006/g18.009.
Continuous package watch after startup stays out of scope.

## Closeout

- Merge performed by the plugin as
  `0cf6073eb2067c4fc4127ec5c318a0f682c5859f` on 2026-09-11 (PR #243),
  with parents `cc67412d8` (main) and `54c3a24ab` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `54c3a24abff00df03a23e8a210d7b92ef506a219` by betterthanclay
  ([comment #5632044289](https://github.com/inflatable-cookie/poodle/pull/243#issuecomment-5632044289)).
  No changes required; three non-blocking observations only.
- Reviewed-head validation (reviewer ran at the exact head, tree left
  clean): `bun test scripts/web-distribution/preview-distribution-preflight.test.ts`
  2 pass / 0 fail, 62 expects; `effigy docs:lint` pass;
  `git diff --check` clean; CI `web` pass 7m51s including
  `test:preview-preflight` (2 pass, 65.4s) inside `ci:web`; CI `rust` pass.
- Non-blocking reviewer notes (deferred, no acceptance impact): the
  "fresh before listen" check polls at 150 ms, so it proves the invariant
  for the current sequential composition but would not catch a
  sub-poll-order race if a concurrent builder were introduced;
  `listenersOn` relies on `lsof` and degrades silently if absent
  (present on the ubuntu runner and locally); `ci:web` wall time rises
  roughly 65–80 s against the job's 10-minute timeout.
- Deferred: no release, tag, publish, Desktop, native, watcher, workflow,
  or package-API work starts from this task. `g18.011` stays held until
  g18.013, g18.014, g18.016, and g18.017 merge; `g18.006` stays blocked
  and `g18.009` held until the sweep, all other blocking repairs, and
  operator acceptance complete.

## Continuation

After g18.013, g18.014, g18.016, and g18.017 merge, Chatterbox releases
held g18.011.
