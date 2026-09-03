# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

- 2026-09-03 — RESOLVED 2026-09-03 by g16.076. Two open Selects plus a focused
  trigger used to close both layers: `on_cancel` Closed the focused instance
  and `dismiss_innermost` popped the other. Overlay members skip cancel so
  Escape owns one stack pop; non-searchable trigger blur no longer Close.
  Hit while converting g16.076.

- 2026-09-03 — `HeadlessDriver::pointer_activate_id` on a missing runtime id
  falls through to `pointer_activate_at(0.92)` and can toggle whatever control
  sits there. A vanished Select option after disable-while-open looked like a
  click on the listbox row and reopened or toggled the trigger. Assert the
  bounds are gone, then do not click the missing id. Hit while converting
  g16.076.

- 2026-09-03 — `dispatch_key_raw("enter")` always sends key-up. GPUI synthesizes
  a click from Enter/Space on key-up for the focused element. Select close
  restores the trigger during key-down, so the matching key-up reopens the
  menu. Use `dispatch_key_press("enter")` when the close path restores focus
  in the same keystroke. Hit while converting g16.076.

- 2026-09-02 — `effigy ci:web` `test:components` (`bunx vitest run`) includes
  `react-preview`, which resolves `@inflatable-cookie/poodle-react` from the
  workspace symlink to `packages/react/components` **dist exports**. `react:package`
  runs later in the same board. A worktree without `dist/` fails three
  preview suites (`catalogue-nav`, `g15-031-*`, `g15-033-*`) after every
  aliased suite already passed. Warm the React dist (`effigy react:package`)
  before a first `ci:web` on a cold worktree, or give `react-preview` the same
  src alias `react-components` already has. Hit while closing g16.065.

- 2026-09-02 — RESOLVED 2026-09-02 by g16.061. Unset `effigy test:web-pack-install`
  was still g16.059 strict certification. Empty `origin/main..HEAD` and ordinary
  feature ranges failed before build/pack (`certification scope found no changed
  paths` or the writable allowlist), so `ci:web` could not go green for feature
  PRs. Default is now receipt-free ordinary smoke; `strict` and
  `g16.054-candidate` stay explicit. Hit while reproducing g16.060 merge
  validation.

- 2026-09-02 — RESOLVED 2026-09-02. Lazy AgentMessage in root AgentPlan delayed
  the parser and emptied SSR plan bodies. Operator moved all five markdown
  components to `./markdown` and restored synchronous render. Hit while closing
  g16.058.

- 2026-09-02 — RESOLVED 2026-09-02. `effigy test:shell-build` ran compiler/shim
  falsification under Bun's 5s default; `spawnSync.status` was `null` at
  5008 ms. The oracle now has an explicit 30s compiler timeout and a 60s test
  timeout. Hit while closing g16.058.

- 2026-09-02 — RESOLVED 2026-09-02. Compiled shell archives cannot keep the
  pre-058 pack-install consumer green unchanged. Svelte `5.38.6` cannot run
  `5.56.8` client output (`$.delegated` is not a function). Vite 7.3.1 leaves
  CSS imports in compiled React `node_modules` `.js` for Node
  (`ERR_UNKNOWN_FILE_EXTENSION`). Existing Slider/Tree React type proofs
  required packed `src/types.ts`. The existing consumer is pinned to `5.56.8`,
  uses the disposable-smoke CSS stub, and those proofs read `dist/*.d.ts`. No
  059 receipt or below-floor negative. Hit while closing g16.058.

- 2026-09-02 — RESOLVED 2026-09-02. Node 22 `navigator` is a getter-only
  global. Disposable browser mount uses `defineProperty`; Node
  `--conditions=browser --import css-register.mjs` runs the mount. Hit while
  closing g16.058.

- 2026-09-02 — RESOLVED 2026-09-02. `@sveltejs/package` `emitDts` rejects
  TypeScript 7.0.2 (`"typescript": "^5.0.0 || ^6.0.0"`). Distribution
  declarations now stage through `scripts/web-distribution/declaration-tools`
  pinned to TypeScript `6.0.3` and `@sveltejs/package` 2.5.7. Root repo
  TypeScript stays `^7.0.2`. Hit while closing g16.058.

- 2026-09-02 — Staged-dist path audit treats any `/…` quoted string as a
  workspace path. Svelte SSR HTML contains `"/</span>"`, which is not a path.
  Values with `<` or `>` are skipped. Hit while closing g16.058.

- 2026-09-02 — RESOLVED 2026-09-02. After core exports moved to `dist/`,
  `docs:lint` (and therefore `health`) failed on a fresh checkout until
  `core:build` produced `packages/core/dist`. `health` and `test:components`
  now run `core:build` first. The pack-install directory-membership check
  now treats `src` and `dist` as trees (`package/<dir>/` or exact
  `package/<dir>`) and still treats `LICENSE` as a file. g16.059's new
  receipt/consumer/two-pack work stays out of this harness. Hit while
  closing g16.057.

- 2026-09-02 — `scripts/gate-tree-guard.ts` writes `poodle-gate-tree-guard.json`
  into the shared OS temp dir. Parallel worktrees (`docs:check` / `ci:web`)
  clobber each other's snapshot; `--compare` then fails with "no snapshot found".
  Hit while closing g16.050 next to a sibling lane.

- 2026-09-02 — Same-worktree parallel `effigy ci:web` and `effigy docs:check`
  both take `task:poodle/core:build`. The second fails immediately with
  `lock conflict for task:poodle/core:build`. Run those boards sequentially, or
  make the waiter retry until the holder exits. Hit while closing g16.063.

- 2026-09-02 — Worker handoff `Planning base commit` used a padded short SHA
  (`a809792c62399523830068082454445123754879`) instead of `git rev-parse`
  (`a809792c6cd9873f9716b4954d2b4b803c6b65eb`). The unique prefix is an
  ancestor; the 40-char string is not an object. Record `git rev-parse --verify`
  output. Hit while starting g16.056.

- 2026-09-02 — WebKit refuses some ports as restricted (`4190` is one of
  them: `Not allowed to use restricted network port`). Local browser probes
  should pick a free port outside that list (`4187`–`4188` are already taken).
  Hit while proving g16.046 block hits.

- 2026-09-02 — `rustfmt` on a huge leaf such as
  `packages/gpui/preview/src/node_compat.rs` still rewrites hundreds of
  unrelated `IntoElement` wrappers. Format the edited functions by hand or
  skip rustfmt on that file. Hit while repairing g16.046.

- 2026-09-02 — `rustfmt --edition 2021 packages/gpui/preview/tests/headless_regressions.rs`
  (and `node_compat.rs`) rewrites the whole file. Those modules are already
  far over rustfmt's wrap budget, so a one-line insertion becomes a thousand-line
  churn. Format only the new hunk by hand, or use `cargo fmt` scoped so it
  respects the crate's existing skip/ignore posture. Hit while repairing g16.045.

- 2026-09-02 — React default `stickyTones = ["danger"]` is a new array per
  render. Putting that identity in a `useEffect` dependency list retriggers
  `setState` forever. Keep a module-level default and depend on a joined key.
  Hit while wiring g16.047 config reconcile.

- 2026-09-01 — RESOLVED 2026-09-01. An earlier `ci:web` run OOMed the
  specimen census at the ordinary V8 heap while closing g16.034. The shared
  rAF/cancel cleanup and React smoke cleanup removed the leak: ordinary
  `effigy test:components` and `effigy ci:web` now pass at default Vitest
  parallelism and normal heap (372 files / 3472 tests). No heap override or
  worker serialization is required.

- 2026-09-01 — Adding one public portable catalogue component requires a
  coordinated denominator bump across `specimen_probe.rs` `EXPECTED_ROUTES`,
  `test/parity/specimen-axis-census.test.tsx`, `lint-docs.ts` GPUI/native
  proofs, `packages/gpui/cross-runtime-parity-report.json`,
  `packages/gpui/native-accessibility-proof.json`, the ledger generator's
  required `N/N portable specimen routes` phrase, and
  `packages/shared-demo-app-audit.json` export counts. Missing any one fails
  a later board after `docs:check` already passed. Hit while closing g16.034.

- 2026-09-01 — `rustfmt path/to/lib.rs` formats the whole crate because
  rustfmt follows `mod` declarations. Passing `packages/render/src/lib.rs`
  rewrote dozens of unrelated renderer files. Format only the edited leaf
  modules, never a crate root, unless the whole crate is in scope. Hit while
  implementing g16.034.

- 2026-09-01 — The HistoryCenter usage snippet in
  `packages/svelte/preview/src/component-docs.ts:5145` still teaches the retired
  v2/v1 API: `entries`/`branches` props, `branchCount` on `HistoryEntry`, and
  `onSelectEntry` / `onCheckout` / `onLoadMoreEntries`. None of those exist on
  the v3 surface. It is generated into both previews'
  `artifacts/component-docs.json`, so the public docs are the last place still
  advertising the shape `g16.033` proves the package no longer exports. A
  reader who copies it gets four unknown props and a type error. Not fixed
  here: `component-docs.ts` is outside this card's writable scope, and the
  snippet needs a v3 rewrite rather than a field edit. Found while running the
  v2 `branchCount` absence search for g16.033.

- 2026-09-01 — RESOLVED 2026-09-02 by g16.053. `audit:security` failed on
  `main` on an English word.
  `docs/triage/20260901-080641-post-g16-research-queue.md:153` contains
  "mask-plus-translated-highlight", and the OpenAI matcher
  `/sk-(?:proj-)?[A-Za-z0-9_-]{20,}/` in
  `scripts/audit-repository-security.ts:23` had no left boundary, so it matched
  the `sk-plus-translated-hi…` inside `mask-`. `effigy qa` was therefore red on
  `main` itself. The production matcher now requires a left word boundary
  (`\bsk-(?:proj-)?[A-Za-z0-9_-]{20,}/`) and focused tests exercise that
  production path: real `sk-` / `sk-proj-` shapes at whitespace, quote, `=`,
  and `:` still match; `mask-plus-translated-highlight` and
  `task-backed-...` do not. Denominator stays `git ls-files` with no path
  exclusion. Found while closing g16.028.

- 2026-09-01 — `probe:gpui-specimens` fails on a wall-clock budget
  (`probe shard N exceeded the two-minute test-body budget`) rather than on
  anything it constructed. All four shards reported `42/42 routes constructed`
  and still failed at 162s because a `bunx vitest run` happened to be running
  beside them; alone the same shards finish in ~21s. A budget that measures the
  machine rather than the work sends the reader looking for a construction
  regression that is not there. Either scale it from a warm-up measurement or
  say in the failure that contention is the likely cause. Hit while validating
  g16.028.

- 2026-09-01 — `packages/render` is not `rustfmt`-clean on `main`
  (`action_discovery_panel.rs`, `agent_chat_input.rs`, `drag_drop.rs`, and
  others differ), so `cargo fmt --check` cannot be used to check a change and
  `cargo fmt` would bury it in unrelated churn. AGENTS.md already warns against
  crate-root formatting; a one-off format commit, or a documented exclusion,
  would let workers verify their own diff. Hit while closing g16.028.

- 2026-08-31 — `poodle-gpui-node-backend` converts every child through the
  PUBLIC `to_gpui` entry, so any pre-walk that entry performs restarts at the
  subtree root once per node. `collect_layers` survives that by deduping;
  a new depth pre-walk did not, and silently flattened every nested drop
  target to depth zero. Give `to_gpui` a private recursion entry, or make the
  re-entry explicit in its doc comment. Hit while building g16.025.

- 2026-08-31 — `scripts/parity-evidence-ledger.ts` hard-codes the ledger's
  `Updated:` line, and its reproduction test compares the whole document. Any
  cell move needs the same date edited in two files or the test fails on a
  header line rather than on evidence. Derive it from the newest evidence, or
  exclude the header from the comparison. Hit while closing g16.025.

- 2026-09-01 — second sighting of the `smoke:gpui-window-capture` qa flake
  below, this time on `tests::batch_mode_accepts_no_other_flag`. `ci:native`
  had just run the same selector green, the branch touches nothing under
  `bin/window_capture`, and the selector passed in isolation and on the next qa
  run. Two different assertions now, which points at the harness rather than
  either test. Hit while closing g16.028.

- 2026-08-30 — `smoke:gpui-window-capture` failed once in `effigy qa` on
  `an_empty_or_malformed_batch_manifest_is_rejected`, then passed in isolation
  and on the next qa run. The smoke script only prints the last three cargo
  lines, so the real assertion is lost. Keep the full cargo stderr on failure.
  Hit while repairing g16.024.

- 2026-08-30 — `parity-evidence-ledger` resolves a React source as the first
  existing path among `Name`, `Name.tsx`, … On a case-insensitive volume,
  `src/tree/` matches `Tree` before `Tree.tsx`, so adding an internal helper
  directory with that name rewrites the Tree/React ledger cell. Name internal
  folders away from the public component (`tree-item/`, not `tree/`). Hit
  while closing g16.024.

- 2026-08-30 — Svelte smoke and axe globs are `src/*.svelte`, so every
  top-level `.svelte` file is treated as a public component. An internal
  helper at that path (`EditableListRow`) failed both suites because it
  required `DragDropProvider`. Keep internal Svelte helpers in a
  subdirectory. Hit while closing g16.023.

- 2026-08-30 — A separate PR thread checked the orchestrator-owned primary
  checkout off `main` and onto its worker commit while a handoff was being
  written. The committed worker branch was safe, but validation ran against
  the wrong source until the orchestrator restored `main`. Worker and review
  helpers must never use the primary planning checkout for PR checkout.

- 2026-08-30 — Vite is not hoisted at the repo root, so a fixture
  `vite.config.ts` cannot `import { defineConfig } from "vite"` and
  `import { createServer } from "vite"` fails from `test/`. Use a plain config
  object and spawn `packages/svelte/preview/node_modules/vite/bin/vite.js`.
  Hit while adding the g16.022 drag-drop browser probe.

- 2026-08-29 — `rustfmt` on a touched `packages/gpui/preview/tests/headless_regressions.rs` or specimen file rewrites the whole multi-thousand-line file. Format only the new function, or skip rustfmt on those paths. Hit while landing g16.032 Batch 1; recovered by restoring and re-applying the surgical edit.

- 2026-08-29 — `scripts/gate-tree-guard.ts` keeps its snapshot at a fixed
  `os.tmpdir()` path shared by every worktree, and `--compare` deletes it. Two
  concurrent `effigy ci:web`/`qa` runs in different worktrees therefore fail
  each other with "no snapshot found". Key the snapshot by repository root or
  by a run id. Seen while closing g16.031; the same board passed on a rerun.

- 2026-08-29 — The Svelte package build strips a function parameter's type
  annotation but leaves the optional marker, so `function f(id?: number)` in a
  `.svelte` script ships as invalid JavaScript and breaks every consumer import
  of that component. Nothing in the component suites catches it; only
  `effigy test:web-pack-install` does. Use a default parameter instead, or make
  the strip handle `?`. Found while closing g16.031.

- 2026-08-29 — `scripts/parity-evidence-ledger.ts` cites one focused test per
  web surface and picks the alphabetically first matching file. Adding a
  second focused test file for a component can silently move that citation and
  fail `docs:check` with "ledger cell differs from live evidence". Either cite
  every matching file or sort by a stated rule. Found while closing g16.031;
  worked around by naming the new file after the cited one
  (`AudioControlsLifecycle`).

- 2026-08-29 — `effigy ci:rust` rustfmt-dirties dozens of unrelated
  `packages/render/src/*.rs` files in an otherwise clean worker tree. Revert
  the extra formatting or stop running crate-wide rustfmt from the CI board.
  Found while closing g16.029.

- 2026-08-27 — RESOLVED 2026-08-27. HistoryCenter never re-read props after
  OPEN, so a host that handed back new pages while the popover stayed open
  kept the OPEN-time copy. Already fixed: the Svelte/React adapters dispatch
  `PAGES_CHANGED` on pages identity change (`HistoryCenter.svelte` /
  `HistoryCenter.tsx` pages `$effect` / `useEffect`). Evidence:
  `HistoryCenter.test.ts` "re-requests continuations when the host supplies
  pages containing the open run". Filed from Loophole 2026-08-12.

- 2026-08-27 — RESOLVED 2026-08-27. HistoryCenter stale-level reconcile needed
  an event that never came, so an open fork whose run now sat on the spine
  spun on "Loading…". Already fixed: `PAGES_CHANGED` is inert itself and
  drives `reconcileStaleLevels` once. Evidence: core tests "PAGES_CHANGED is
  inert on its own" and "PAGES_CHANGED drives the stale-level reconcile
  exactly once". Filed from Loophole 2026-08-12.

- 2026-08-27 — RESOLVED 2026-08-27. HistoryCenter's single-fork picker
  disabled its actions menu with the Select, greying out Checkout. Already
  fixed: menu enablement is independent of `row.disabled`; Checkout gates on
  `picked.preferred`. Evidence: "single fork: the Select is disabled but
  Checkout and Rename stay live on their own gates (R1)". Filed from Loophole
  2026-08-12.

- 2026-08-27 — RESOLVED 2026-08-27. Poodle-svelte `types.ts` still exported
  the v2 `HistoryEntry` (`branchCount`). Replaced with a re-export of the
  core `HistoryEntry` / `HistoryEntryPosition` shapes; React `types.ts`
  matched. Evidence: HistoryCenter package-types tests assign
  `continuationCount` on the public type.

- 2026-08-27 — RESOLVED 2026-08-27. Poodle Select ignored `variant="ghost"`
  in native mode: the native root had no `data-variant`, so ghost CSS never
  applied. Native root now stamps `data-variant={variant}`. Evidence:
  Select svelte/react tests "stamps data-variant on the native root". Filed
  from Figmatic 2026-08-14.

- 2026-08-27 — `effigy drift:roles` resolves the deferred Jetstream preview
  and fails in an otherwise valid Poodle worktree when the sibling Jetstream
  checkout is absent. Keep the role census on the active cohort or split the
  deferred backend into an opt-in selector. Found while reviewing `g16.013`;
  the roadmap explicitly keeps Jetstream out of normal validation.

- 2026-08-27 — `rustfmt packages/contracts/components/src/lib.rs` formats
  every child spec module. Format the leaf file (`icon_button.rs`) and keep
  `lib.rs` edits surgical. Hit while landing `g16.011`; recovered by restoring
  the unrelated spec files.

- 2026-08-27 — `rustfmt <file>` without `--edition 2021` treats preview crates
  as 2015 (fails on `async`) and still rewrites wrapping across the whole
  file. Restore the file and re-apply the surgical edit; do not format
  `node_compat.rs` or `headless_regressions.rs` that way. Hit while landing
  `g16.009`.

- 2026-08-26 — `rustfmt packages/render/src/lib.rs` still formats every child
  module. Format the leaf file, not the crate root. Hit while landing
  `g16.003`; recovered by restoring the crate and re-applying the export.

- 2026-08-26 — `cargo fmt --manifest-path <crate>/Cargo.toml -- <file>` still
  formats the whole crate. Format named files with `rustfmt <file>` instead.
  Hit while closing `g16.002`.

- 2026-08-26 — a launcher worktree without `node_modules` lets bun resolve
  `lucide-static` from `~/.bun/install/cache` at 1.34.0 while the lock wants
  1.31.0, so `audit:icons` fails before any web test. Run `bun install` in the
  worktree first. Hit during `g16.002` `ci:web`.

- 2026-08-25 — Bovine Accelerator Desktop PR 26 changed the current source set
  after the Poodle 0.2.2 adoption merged, but its frozen private-candidate
  receipt still names candidate `54bc1634` and Bovine source `9098a98`.
  `prepare:longhorn-private-candidate` now derives candidate `741bd287` and
  stops on receipt drift, so the source-independent proof cannot be recreated
  from current `main` until the receipt is refreshed or candidate proof is
  decoupled from unrelated source movement. Affects g15.075 post-merge
  revalidation; the committed Poodle manifest and lock graph remain correct.

- 2026-08-25 — Underlay Reference's `effigy doctor` cannot provide a clean
  consumer-review baseline: container health lacks `cargo-fmt`, root
  `test.exclude_catalogs` is unsupported, and `acme-docs` references built-in
  `docs` commands that the effective catalog cannot resolve. Repair the
  container toolchain and Effigy manifest/task routing. Affects review
  `g15.079`.

- 2026-08-25 — Finch's `effigy doctor` cannot provide a clean consumer-review
  baseline: `effigy.toml` still uses unsupported `test.exclude_catalogs`, while
  Cargo link-health inspection fails because the committed lock wants a Signal
  refresh under `--locked`. Repair the manifest key and make dependency health
  inspection read-only against the accepted lock. Affects Finch review
  `g15.073`.

- 2026-08-25 — Figmatic's headless `effigy build` rewrites committed Studio
  `dist/`, mutates tracked `node_modules` contents, and refreshes root
  `Cargo.lock`, leaving hundreds of dirty paths after a read-only PR validation
  run. Keep generated/dependency output untracked or make the validation task
  restore/compare it explicitly. Affects Figmatic consumer review `g15.074`.

- 2026-08-25 — Loophole's `effigy test:workspace` refreshes 22 sibling Signal
  package versions in the committed root `Cargo.lock` even when the tested PR
  does not touch Rust dependencies. Isolate sibling lock resolution or add a
  no-write/check mode so headless validation stays read-only. Affects Loophole
  consumer review `g15.076`.

- 2026-08-25 — Underlay tag `v0.9.2` correctly stamps the Rust workspace as
  0.9.2 and pins Poodle Svelte 0.2.2, but its root JavaScript `package.json`
  still declares version `0.9.1`. Git-tag consumers resolve the right commit,
  yet package/lock diagnostics can report a misleading Underlay version. Align
  the JS package version before the next Underlay tag. Found while widening
  Poodle `g15.068`–`072` and `079`.

- 2026-08-24 — Required Northstar orientation through `effigy doctor` now
  fails on three broad repository scans (`generated-in-src`, `god-files`, and
  `stale-suppressions`: 41 error findings total) even though none is connected
  to the active consumer-adoption lane. Baseline or intentionally configure
  these scans so doctor distinguishes new health drift from accepted source
  shape. Affects every worker and orchestrator startup in Poodle.

- 2026-08-24 — The exact-candidate Button visual diagnostic recovered three
  Svelte captures only after separate 60-second `waitForSelector` timeouts,
  despite all retained captures verifying and repeating byte-identically. Add
  an early preview-health probe or shorter bounded navigation timeout so a
  healthy recovery does not add three opaque minutes. Affects
  `test:visual-button-comparison-windowed`; found during `g15.061`.

- 2026-08-24 — An exact-candidate review ran green `effigy qa`, then
  `effigy release gates` failed once in `smoke:gpui-window-capture` after about
  2.6 seconds. The isolated selector passed immediately and a complete release
  gate retry passed 1/1. Effigy's task report retained only exit 1 rather than
  the child stderr, leaving the transient cause opaque. Preserve child stderr
  and investigate repeated headless-board warm-state races. Found while
  reviewing `g15.060`.

- 2026-08-23 — `effigy release simulate` cannot parse Poodle's accepted
  `CHANGELOG.md`: it rejects the prose under both `Unreleased` and `0.2.0` as
  unexpected content. The repository's documented tag-plus-workflow release
  path does not consume this parser, but the generic Effigy release simulation
  and prepare/execute protocol is unusable until the changelog dialect or
  parser is aligned. Found during `g15.013` read-only certification.

- 2026-08-23 — Bumping a workspace package version leaves `bun.lock` stale and
  there is no supported way to refresh only that. `bun install`,
  `bun install --force`, and `bun install --lockfile-only` all leave the
  `workspaces` block's `version` and intra-repo range strings at the old
  values, and `--frozen-lockfile` still passes, so the drift is silent.
  Deleting the lock and reinstalling does pick them up, but it also re-resolves
  every registry range (here: rolldown, oxc, vitest all moved), which is a
  dependency-policy change a release candidate must not make. A
  `bun install --lockfile-only --no-update` or an explicit workspace-metadata
  refresh would remove the hand-edit. Found by g15.050.

- 2026-08-23 — Every release-bearing Rust crate version bump forces a
  regeneration pass: `poodle-codegen`'s version is stamped into all 45
  generated artifacts through `GENERATOR_VERSION`, so `ir:check` and
  `catalogue:check` go red until `ir:build` and `catalogue:build` restamp them.
  Neither check is on the `qa` board, so the drift is only visible if someone
  runs them. Either put the two `*:check` selectors on the board or drop the
  generator version from the header (source path + schema version already
  identify the artifact). Found by g15.050.

- 2026-08-23 — The headless GPUI root began calling the production
  `reset_element_ids` boundary in every render, but the generated element and
  gesture counters were process-global atomics while Rust ran independent
  test apps in parallel. One app could rewind another app's in-progress tree
  walk, making generated identity proofs intermittent. Fixed on g15.052 by
  matching the counters to GPUI's UI-thread model and the thread-local
  registries they key.

- 2026-08-22 — zsh reserves the lowercase `path` variable for `PATH`; a shell
  inventory loop using `path` silently removed `git`, `rg`, and `sort` from the
  command search path. Use a neutral loop variable in repository probes.
  Found on g15.051.

- 2026-08-22 — `effigy doctor` can fail before `tasks.health` starts because
  writing `.effigy/runtime/tasks/active/*.json` hits the machine-wide
  `Too many open files in system` limit. The health command and useful doctor
  findings then share one red result. A bounded retry or an early file-limit
  diagnostic would make the failure actionable. Found while dispatching
  g15.046.

- 2026-08-21 — A fresh git worktree has no `node_modules`, and
  `effigy docs:check` fails deep into the board with `Cannot find module
  '@inflatable-cookie/poodle-core/tokens'` from the Svelte/React preview
  parity scripts rather than up front with a missing-deps hint. A one-line
  preflight (or an `effigy init --check` hint) would save the debugging loop.
  `bun install` in the worktree fixes it. Found on g15.045.

- 2026-08-21 — `poodle-tokens` reaches outside its own crate directory for
  generated code: `src/lib.rs` includes
  `src/../../../tokens/artifacts/rust/mod.rs`. The crate is therefore not
  self-contained, and any disposable copy, vendoring, or `cargo package`-style
  relocation fails with `couldn't read ... No such file or directory` unless
  `packages/tokens/artifacts` is copied alongside it. Every crate in the Rust
  graph inherits the constraint, since they all depend on `poodle-tokens`.
  A build script or an in-crate generated module would remove it. Found on
  g15.044 while vendoring the Rust graph into a throwaway proof workspace.

- 2026-08-21 — `effigy qa` reproducibly reaches `gate-tree-guard --compare`
  with no snapshot even when run alone from a clean committed worktree; every
  preceding component, package, type, and docs step passes. The earlier entry
  treated this as a parallel-worktree race, but it also affects a single broad
  board. Keep snapshot/compare state invocation-local or make the guard one
  atomic step. Found while closing g15.041.

- 2026-08-21 — The contract drift checkers (`contract-prop-drift.ts`,
  `contract-callback-drift.ts`, `contract-value-domain-drift.ts`) parsed only
  `interface Props`; a discriminated-union props shape
  (`type Props = Common & (A | B)`, the LicenceActivation pattern) failed
  prop-drift outright and was silently skipped by the other two. Fixed on
  g15.041 with a shared `unionPropsBody()` fallback — any future union-typed
  props get it for free, but a third props shape will need the same visit.

- 2026-08-21 — No SSR-capable vitest project existed for Svelte: every
  `.svelte` file compiled client-only, so `render` from `svelte/server`
  throws. vite-plugin-svelte ignores `compilerOptions.generate`; the working
  knob is omitting `resolve.conditions: ["browser"]` in the SSR project so the
  plugin server-compiles through vitest's SSR transform. Fixed on g15.041 with
  the `svelte-components-ssr` project (server-render evidence for
  `defaultOpen`/SSR-dependent components now has a home).

- 2026-08-20 — `effigy ci:rust` runs `test:contracts`, which lists seven
  contract crates but not `packages/contracts/node`. `poodle-node` is the
  vocabulary every component depends on, and its own unit tests run under no
  selector — only `cargo test --manifest-path packages/contracts/node/...`
  reaches them. Adding the crate to `test:contracts` is a one-line task edit,
  which needs the task-file owner. Found on g15.040.

- 2026-08-20 — `gh pr merge --merge --delete-branch` successfully merges a PR
  from its worker worktree, then exits non-zero because it tries to check out
  `main`, which is already owned by the planning worktree. Merge first without
  local deletion, verify PR state, then remove the worktree/branches from the
  planning checkout. Affects orchestrator PR closeout.

- 2026-08-20 — `@testing-library/*/fireEvent.click` dispatches only `click`,
  not `mousedown`. Overlay dismiss handlers listen on document `mousedown`,
  so a click-only option test can pass the old broken DateTimeZonePicker
  implementation. Pointer-commit regressions must dispatch `mousedown` on
  the real target, then `click`. Found on g15.039 review.

- 2026-08-20 — `rustfmt --edition 2021 packages/render/src/lib.rs` walks every
  `mod` and reformats the whole `poodle-render` crate; the same happens for
  `packages/gpui/preview/src/main.rs`. Format only the files you edited, never
  a crate-root `lib.rs`/`main.rs`, or restore the unrelated modules before
  commit. Found again on g15.038 review round 3.

- 2026-08-20 — gpui 0.2.2's `Frame::clear` never clears `debug_bounds`, so
  `VisualTestContext::debug_bounds` accumulates selector entries for the life
  of a window: a second page mounted in the same window still "finds" the
  previous page's selectors, with stale bounds. Any probe that discovers
  elements across routes needs a fresh window per route. Found by g15.026.

- 2026-08-20 — The GPUI preview's bin test target did not compile at all:
  `specimens/scene_specimen.rs`'s test module used `use super::*`, which
  chains the parent's `use gpui::*` and resolves `#[test]` to gpui-macros
  0.2.2's `test` proc macro — the known rustc-crashing one — producing a
  SIGBUS inside `librustc_driver` on every `--test` build. No selector ran
  that target (`regressions:native` uses `--test headless_regressions`), so
  the breakage was invisible, and it hid three failing
  `contract_usage_docs::tests` assertions whose expected contract events/slots
  no longer match the docs. Found by g15.026; the glob fix landed with the
  probe, the three stale assertions need an owner.

- 2026-08-19 — `effigy bootstrap:deps` fails in a second worktree of this repo:
  `cargo fetch` for `packages/jetstream/preview` aborts with "package collision
  in the lockfile: packages poodle-layout v0.1.0 (<other worktree>) and
  poodle-layout v0.1.0 (<this worktree>) are different, but only one can be
  written to lockfile unambiguously". The bun half of bootstrap has already run
  by then, so the JS checks work and only the Rust fetch is lost; `cargo build`
  in each crate still resolves. A per-worktree `CARGO_TARGET_DIR`/lockfile
  scope, or fetching Rust crates per manifest directory, would remove the
  collision. Found by g15.020.

- 2026-08-17 — Sweeping the catalogue headlessly wedges the browser on
  `#components/file-upload`: the specimen opens a native file chooser and the
  page never settles, so a Playwright run with no `filechooser` handler and no
  per-page deadline hangs there forever and silently loses everything after it.
  A driver needs `page.on("filechooser", …)`, `page.on("dialog", …)`, and a
  hard per-page deadline that relaunches rather than awaits a poisoned browser.
  Worth a shared preview-probe helper before `g15.012` builds its capture lane
  on the same path. Found by g15.011.

- 2026-08-17 — `bun run --cwd packages/svelte/preview dev` takes whatever port
  is free, so parallel worktrees silently land on each other's neighbours. Worse,
  a stale server bound to `127.0.0.1:<port>` shadows a new one bound to
  `*:<port>`, and the new server reports "ready" on a URL that answers 404. Pass
  `--port <n> --strictPort` when driving a preview from an agent, and check
  `lsof -nP -iTCP -sTCP:LISTEN` before trusting the banner. Found by g15.011.

- 2026-08-17 — `effigy docs:check` dies at `report:parity`'s React preview
  step in a worktree with no `node_modules`:
  `bun run --cwd packages/react/preview parity:report` cannot resolve
  `@inflatable-cookie/poodle-core/tokens`, while the same script under
  `packages/svelte/preview` succeeds. `docs:lint` and `docs:spec-drift`
  already passed. Found by g15.009 closeout.

- 2026-08-17 — The headless GPUI driver's mount box is a fixed 160×60, and its
  content mask clips **hit testing** as well as paint: `bounds_for` happily
  reports an element at y=240, `pointer_activate_id` clicks its centre, and
  nothing fires. Any mounted regression over a component taller than 60px has
  to drive keyboard activation instead, or shrink its fixture until the target
  lands inside the box. A driver option for a larger mount box (or one that
  sizes to its child) would remove the guessing. Found by g15.008 Batch C.

- 2026-08-17 — `poodle_render::collapsible` marks its trigger `focusable` but
  renders no focus patch, so the GPUI backend creates no focus handle for it
  and a composed Collapsible's disclosure is unreachable by keyboard and
  cannot receive a focus request. `model_catalogue_editor` stamps the ring on
  the trigger itself to make the hidden-section focus destination real. Same
  class as the `icon_button` entry below; fix both in the primitives and drop
  the per-composition workarounds. Found by g15.008 review round 1.

- 2026-08-17 — `IconButtonSpec::with_expanded` and `with_controls` never reach
  `Node.a11y`: `poodle_render::icon_button` ignores both, so a composed
  disclosure has no `expanded` and no `controls` relationship unless its caller
  restates them. `history_center`, `changed_files` and now
  `model_connection_card` all set `node.a11y.expanded` themselves after
  building the button. Project the spec fields in `icon_button` and drop the
  three workarounds. Found by g15.008 Batch B.

- 2026-08-17 — The vocabulary has no Escape channel for a plain control:
  `NodeKey` carries arrows/Home/End/Space/F2 and nothing else, so a keyboard
  reorder grab can only be cancelled through `Interaction::on_cancel`, which is
  documented as "cancels the current input edit". `model_catalogue_editor` uses
  it for cancel-grab because there is no alternative. Either add `NodeKey::
  Escape` or widen `on_cancel`'s documented meaning to "cancel the current
  gesture". Found by g15.008 Batch B.

- 2026-08-17 — `contract-spec-drift` only had a *global* web-only prop set, so
  exempting one component's `defaultValue` would have exempted the ~20
  components that legitimately carry `default_value`. g15.008 added a
  slug-scoped `WEB_ONLY_BY_SLUG` beside it for the model-connection family's
  uncontrolled seeds. If more families land with native bindings that keep the
  current value on the host, the two lists should probably become one
  slug-aware structure rather than two.

- 2026-08-17 — The headless GPUI test platform renders a view several times
  per `window.draw`, so an interactive node without a declared `id` does not
  keep a stable element across a click (press and release land on different
  element states and the click is dropped). The production preview renders
  once per platform frame and is unaffected. Mounted regressions in
  `tests/headless_regressions.rs` therefore assign explicit ids to the
  interactive nodes they drive — the pattern every retained regression there
  already used — and the driver's `reset_element_ids` per frame does not by
  itself fix id-less nodes. Found by g15.007 Batch A while proving grouped
  CodeInput and FileUpload browse in a mounted window.

- 2026-08-17 — The same Northstar card and handoff can be dispatched twice
  without warning while its dispatch-ledger entry is already `in-flight`,
  producing competing PRs #30 and #31 for `g15.014`. Add a duplicate-launch
  guard keyed by repository plus card/handoff identity, and surface the active
  worker or PR before starting another run. Affects orchestrator worker
  dispatch.

- 2026-08-16 — React `SplitView` is missing the contract's `divider` prop and
  the `--poodle-split-seam` root anchoring for the toggle pill: the pill is
  positioned against the divider box, which a collapsed or hidden sibling can
  leave degenerate, and `data-divider` never renders. The roster counts React
  SplitView "complete" on impl/export/gallery/test axes, so the deltas are not
  inventory gaps; they are contract-parity deltas for a follow-up card.
  Found by g15.006 while writing the SplitView focused evidence (the
  both-collapsed recoverability and `primaryHidden`/`secondaryHidden` deltas
  were fixed there; these remain).

- 2026-08-16 — two `effigy docs:check` runs in separate worktrees can race on
  `gate-tree-guard` state: one run reached `--compare` after the shared snapshot
  had disappeared and failed with "no snapshot found", while the other passed.
  Namespace guard snapshots by worktree and process, or make snapshot/compare
  one atomic invocation. Found while reviewing g15.003 and g15.004 in parallel.

- 2026-08-16 — cards that say only `git diff --check` can report green after
  all work is committed while committed trailing whitespace remains: the bare
  command checks the empty working-tree diff. Final worker gates should compare
  the integration range, for example `git diff --check origin/main...HEAD`.
  Found on PR #26's committed g15.004 batch log.

- 2026-08-16 — `icon_button` renders no focus patch, and the GPUI backend
  creates a focus handle only for a focusable node that carries one. Every
  IconButton is therefore unfocusable and unreachable by keyboard on native
  unless its caller stamps a ring itself, which `poodle-render::history_center`
  now does for undo, redo and the picker actions. Fix it in `icon_button` and
  drop the per-composition workaround. Found by g14.007, retained by g14.021.

- 2026-08-16 — `packages/core`'s `check` script still reports ~90 pre-existing
  strict errors, and the narrow `conformance:typecheck` selector that cards
  leaned on instead is gone with the rejected pilot (g14.021). New core modules
  now have no type gate short of `check:svelte`. Fix the strict backlog or
  scope the script to a passing config.

- 2026-08-15 — `docs:check`'s recursive Markdown scan enters
  `packages/gpui/preview/target/`; when it runs beside Cargo, a temporary rmeta
  directory can disappear between `readdir` calls and fail docs lint with
  `ENOENT`. Exclude build-output directories from the operator-guide scan or
  tolerate vanished entries. Affects parallel docs + Rust validation.

- 2026-08-14 — Empty catalogue collection vocabularies emit
  `CatalogueCollectionId = never`. Mapping that union in shared preview
  nav fails `effigy check:svelte`, which type-checks preview files via
  the components workspace. Keep a string-keyed lookup until collections
  exist, or emit a non-`never` empty-collection type.

- 2026-08-14 — React preview files under `src/gallery/` need four `../`
  segments to reach `packages/svelte/preview/src`, while `src/main.tsx`
  needs three. Shared catalogue CSS/nav imports copied the shallower
  depth and failed Vite resolve. A workspace alias for the shared
  preview catalogue module would remove the depth trap.

- 2026-08-14 — `packages/core`'s `check` script (`bun x tsc -p ./tsconfig.json`)
  reports ~90 pre-existing strict errors on main (history-center, color, tree,
  test fixtures), so it cannot serve as a green gate for new core modules.
  Cards relied on `conformance:typecheck`/`check:svelte` instead; the former is
  gone (g14.021). Either fix the strict backlog or scope the script to a
  passing config.

- 2026-08-14 — `effigy docs:contract-drift` fails on main for Button's
  Svelte-only `children`, `leading`, and `trailing` props, while
  `effigy docs:check` stays green because the aggregate lint path treats that
  surface differently. Cards that name the standalone selector inherit a red
  unrelated baseline. Align the two prop-normalization paths or declare the
  slot props once in the contract-drift policy.

- 2026-08-14 — g14.004 said only “conformance gates”, so a worktree agent ran
  the foreground `ci:conformance-windowed` board for 15+ minutes and repeatedly
  stole desktop focus. Selector naming and comments did not protect the
  operator. Resolved at the task boundary with a local opt-in guard; planned
  cards now name the headless selector explicitly. Full GPUI execution still
  needs migration onto GPUI's in-memory test platform.
  RESOLVED 2026-08-15 by g14.023: the windowed board, its opt-in guard, and the
  AppKit driver are deleted; `conformance:complete` runs the full cohort on
  GPUI's in-memory test platform in any worktree.

- 2026-08-14 — repeated `conformance:test-gpui-windowed` runs in one desktop
  session can write all three reports and exit the GPUI child while Effigy
  keeps waiting; later runs can also miss RangeSlider scrub/focus AppKit events
  after an earlier full-green run. The selector should time out/reap the child
  and isolate or reset foreground input state between invocations.
  RESOLVED 2026-08-15 by g14.023: the selector is deleted; the headless board
  is deterministic in-memory execution with no platform input state.

- 2026-08-14 — `effigy graph explore ... --json` can leave its process alive
  after emitting the complete envelope, holding `.effigy/graph/refresh.lock` and
  making subsequent `graph index` calls fail after 10 seconds. The graph command
  should release its refresh lock before output completion or exit promptly once
  the one-shot envelope is written.

- 2026-08-14 — fresh t3 worktrees ship without `node_modules`; `effigy
  conformance:test-web` fails on unresolved `@sveltejs/vite-plugin-svelte`
  until `bun install`. Doctor/bootstrap could seed workspace installs for
  worktree checkout, or the conformance task could fail with that command.

- 2026-08-14 — A Svelte 5 prop named `state` collides with the `$state` rune at
  runtime (`store_invalid_shape: state is not a store with a subscribe method`).
  Components that need a public `state` prop must alias it in `$props()`
  (`state: catalogueState = "ready"`). Hit while building
  `ModelCatalogueEditor` / `ModelConnectionPicker`.

- 2026-08-14 — Collapsible's `slide` transition calls `element.animate`, which
  happy-dom does not implement. Opening a Collapsible in component tests throws
  `TypeError: element.animate is not a function`. Work around by not exercising
  open transitions in happy-dom suites, or polyfill `Element.prototype.animate`.

- 2026-08-14 — `contract-prop-drift` finds a contract's props by searching for
  the literal `### Public Props`, and **skips the contract entirely** when it
  finds none (`if (cProps.size === 0) { skipped++; continue }`). A contract that
  titles its table `## 4. Public Props` at heading level 2 therefore opts itself
  out of the gate silently, and the run still reports success. 36 of 170 are
  currently skipped, `update-status` among them. Measured on the licence trio:
  before their headings were restructured the gate checked 131 surfaces, after
  it checked 134 — the three were invisible to it, with no warning either way.
  A "skipped because no props section" list in the output, or a required
  heading in the contract template, would make the hole visible.

- 2026-08-14 — Svelte `TextInput` defines `focus()` but never exports it
  (`TextInput.svelte:379`), while React's exposes it through
  `useImperativeHandle` (`TextInput.tsx:436`) and the working rules say
  imperative escape hatches "are expected in both web runtimes". A composite
  that must move focus into a field has no shared mechanism: `LicenceActivation`
  routes through `getFocusableElements(panel)` in both frameworks instead, which
  works but is focus-by-position rather than focus-by-identity. Adding
  `export function focus()` to the Svelte component would close it.

- 2026-08-13 — codegen write mode sweeps each target's output-root **top
  level** for orphans, so two targets sharing one `generated/` root delete
  each other's artifacts on `ir:build` (g14-b005's first build silently
  removed the shell artifacts). Sibling targets must own a nested root
  (`generated/specimens/`), which the sweep treats as another target's
  territory. The shell and specimen targets are the precedent pair.

- 2026-08-13 — the documented g13-036 symlink workaround for the Jetstream
  sibling path-dep no longer works: a `poodle` symlink next to a real
  worktree makes cargo fail with a lockfile collision ("packages
  poodle-layout … are different, but only one can be written to lockfile
  unambiguously") because the preview's own deps and jetstream-poodle's
  deps land on two textual paths to the same crates. Working recipe
  (g14-b002, log §2): branch clone of poodle at
  `/Users/tom/.t3/worktrees/poodle/poodle` + sources-only copy of jetstream
  at `/Users/tom/.t3/worktrees/poodle/jetstream` + `CARGO_TARGET_DIR` set to
  the main checkout's jetstream `target/` (rsync -a preserves mtimes, so the
  warm cache hits). Any Jetstream-touching card needs this. Supersedes the
  b003 entry that only diagnosed the collision.

- 2026-08-13 — the React preview is dead on this branch: the Svelte
  canonical registry lists `update-center`/`update-status` as standalone
  specimens (the UpdateCenter PR `b433498d` added them) but the React
  gallery has no specimens for them, and `gallery/specimen-map.ts` throws
  at module load ("Missing React specimens"), so `#root` never mounts and
  no React DOM observation is possible. g13-b049 hit this trying to
  re-prove the four-runtime web half live; it fell back to the jsdom
  component test (which shows the renamed attribute) plus the
  artifact-parity test. Either add the two React specimens or drop them
  from the canonical registry's specimen list. Affects every card that
  needs a live React preview on this lineage.

- 2026-08-13 — g13-047's R4a depth inventory (see its log) found the
  conformance vectors for menu (3 cases) and popover (4 cases) cover
  open/action paths only: CLOSE/ESCAPE/OUTSIDE_INTERACT dismissal events
  are unpinned on both machines, and popover's initialFocus strategies are
  untested. The pinning mechanism is carrying roughly a quarter of the
  machine surfaces at full depth (4 of 13 inventoried machines exercise
  their whole surface). Follow-up card: deepen the dismissal-event vectors
  first — that is the surface the dismissable-layer machinery exists for.

- 2026-08-13 — the Jetstream `snap -- specimens` bin overwrites
  `/tmp/poodle-specimens/{slug}.png` in place with no warning, so a
  before/after comparison (g13-046's renamed-vs-restored proof) silently
  destroys the first capture if you do not copy it before re-running.
  Either write `{slug}-{timestamp}.png`, or print the previous file's mtime
  when overwriting. Any card doing a live state proof through the snap
  should copy each output before the next render.

- 2026-08-13 — g13-042 classified the GPUI visual baselines
  (`packages/gpui/preview/baselines/button-eclipse-compact-sm.png` et al.,
  local-only in the main checkout) as **stale**, not moved: the top chrome
  of the current preview matches the baseline 99-100%, while the specimen
  content region below differs systematically (~45% agreement). The
  saturated-color histogram is near-identical (same palette features),
  which fits an older preview layout rather than a render change — and the
  Jetstream counterpart is a true zero against its baseline (deterministic
  renderer, same `poodle-render` crate). No baseline refreshed. The gate's
  own axis flag is also inert: `test/native-visual/capture.ts` passes
  `--control-size`, but the preview parses `--size` — every GPUI gate
  capture since the flag rename ran at the default size. Either fix the
  harness flag or drop the axis claim; then regenerate the baselines.

- 2026-08-13 — the Jetstream `snap -- specimens` scene is a fixed
  900×640, and Button's States row (disabled/loading) sits below the fold:
  only the upper specimen rows are pixel-demonstrable through the snap.
  The `data-loading` rename was therefore invisible in the Jetstream snap
  (the loading treatment is in the clipped row) while the
  `data-has-leading` rename (icons row, mid-page) was visible. A card that
  needs to pixel-prove a bottom-row state in Jetstream either scrolls the
  snap scene or captures per-section.

- 2026-08-13 — RESOLVED in g13-042. b041's review moved the `button-ts`
  artifact from the preview packages to the component packages
  (`packages/{svelte,react}/components/src/generated/button/`) but never
  moved the two path constants in `packages/codegen/tests/button.rs`, so
  `cargo test` for `poodle-codegen` was red on the branch (`No such file
  or directory`) until 042 corrected them. A review that moves where
  generated code lands must update the tests that byte-compare it.

- 2026-08-12 — RESOLVED at review. `g13-b041` had `Button.svelte`/`Button.tsx`
  import the generated definition across a package boundary
  (`../../preview/src/generated/button`), so the packed `poodle-svelte` /
  `poodle-react` tarballs (`files: src`) did not carry it and
  `test:web-pack-install` failed to resolve the import in its consumer. Green
  on `main`, red on the branch — a real regression, not a pre-existing gap.

  **Cause was the card, not the worker.** `041`'s Writable Paths named
  `packages/{svelte,react}/preview/src/**/generated/**` as the artifact
  location, copied from `b035` where the consumer *was* the preview. A
  component's artifact has to live in the package that ships it. Fixed at
  review: the `button-ts` target now emits into
  `packages/{svelte,react}/components/src/generated/button/`, the four imports
  point there, and `test:web-pack-install` is green again.

  **`test:web-pack-install` is not in `ci:web`.** Every gate the card listed
  passed while the packaged build was broken. A card that moves where code is
  emitted must run it; better, it belongs in `ci:web`.

- 2026-08-12 — RESOLVED 2026-08-13: rule deleted; the drag-handle keeps the ring
  b038 gave it. g13-038 found a dead focus rule: `.poodle-order-by__item:focus-visible`
  (order-by.css) rings the item row, but the item div is never focusable — no
  `tabindex`, and the only focusable inside is the drag-handle button. The ring
  never renders; the drag-handle got its own ring this batch. The dead rule
  should be deleted or retargeted to the handle.

- 2026-08-12 — RESOLVED 2026-08-12: g13-038 closed the sweep. The 56-sheet
  stacking is gated by `docs:focus-ring-drift` (absent-treatment + stacked-UA
  checks with baselines); hidden inputs and machine-driven controls now set
  `outline: none`, and the detail-item/field info triggers defer to the nested
  Popover's own ring. Original report follows. g13-037 measured the other half
  of the 2026-08-11 focus-ring entry: 56 component stylesheets draw their own
  focus ring without suppressing the UA `outline: auto` on the same element,
  so Chrome stacks a second 1px ring. Live in checkbox/radio/switch/tri-state/
  segmented-control (the hidden native input draws `outline: auto` — invisible
  on the clipped inputs, so harmless there) and visibly on the
  `detail-item`/`field` info-trigger wrappers, whose own ring lands on the
  icon while the UA ring draws on the focusable wrap. The follow-up card owns
  the sweep; the fix pattern is `outline: none` on the focusable element + the
  component's own ring on its visible surface.

- 2026-08-12 — g13-036: the scene now owns the control labels in all four
  shells, but the web shells still hardcode accessibility labels: Svelte and
  React `DisplayControls` pass `ariaLabel="Neutral contrast"` (contrast),
  `ariaLabel="Search components"` (search) and `ariaLabel="Control size"`
  (size) while the visible label comes from the scene (`Contrast`/`Search`/
  `Size`). The natives are aligned (GPUI's contrast aria label is now the
  scene's `Contrast`; Jetstream's was already), so the web pair carries the
  only remaining second copy of label text — R3's "the scene owns the word"
  is not complete until the web aria labels read from the artifact too.
  Out of scope for 036 (web preview sources are not its writable paths).

- 2026-08-12 — RESOLVED 2026-08-12: the include now covers `test/**` and the
  36 errors are cleared (701 files, 0 errors, up from 449). Verified by
  reintroducing the stale `rootContinuationCount` — the gate catches it now.
  The previews are still unchecked and remain open. Original report follows.
  `check:svelte` type-checks `packages/svelte/components/src`
  only. Its tsconfig `include` is `src/**`, so the 36 files in `test/` are not
  checked, and no task type-checks either preview at all. This is not
  hypothetical: the `rootContinuationCount` → `precedingContinuationCount`
  rename (`2a6d3af9`) left four live sites stale — both HistoryCentre specimens
  and both component test suites — and every gate stayed green. Widening the
  include to `test/**` surfaces 36 pre-existing errors across 11 files
  (`SplitView.svelte.test.ts` 12, `AppHeader.svelte.test.ts` 8,
  `contract-prop-drift.ts` 6 via its test, and others); adding `vite/client`
  and `vitest/globals` types does not reduce them, so they are real. Clear that
  backlog, then widen the include, then decide whether the previews get a gate
  of their own.

- 2026-08-12 — g13-032's picker pencil is an `IconButton`, which forwards no
  data attributes (explicit props only, no rest spread): the pencil's
  `data-part="picker-rename"`/`data-branch` markers had to live on a wrapper
  span, and the rename focus-restore targets the wrapper's inner button. Any
  composite that needs a `data-part`/`data-branch` on an `IconButton` hits
  the same wall; a rest-prop pass on `IconButton` (forward `data-*`/`aria-*`
  to the inner button) would remove the wrapper pattern.

- 2026-08-12 — g13-030's picker puts Poodle's `Select` inside the
  `HistoryCenter` popover, and Select portals its listbox to the document
  body — so Tab from an open listbox is outside `Popover`'s focus trap
  (`trapFocusKeydown` only intercepts keys whose target is inside the
  surface; the portalled menu's Tab bubbles to body instead). The Select's
  own arrows/Enter/Escape work and roving traversal is unaffected; the
  escape is a Select-in-Popover composite concern that predates the
  component-level fix. Either trap on a `contains()` that includes the
  Select's portalled layer, or have Select stop Tab from its open listbox.
  Affects any composite that puts a Select inside a trap (HistoryCenter
  first instance).

- 2026-08-12 — g13-029 hit this: b028's `packages/core/src/history-center.ts`
  does not type-check as committed, and `effigy check:svelte` (a card
  acceptance gate) cannot exit 0 until it is fixed. `HistoryCenterOpenFork.inner`
  is declared `HistoryCenterOpenFork | null` (line 198) but every use site
  treats it as `ReadonlyMap<string, HistoryCenterOpenFork> | null`
  (`withAddedLevel`, `new Map(level.inner)`, `findLevel`, `walkLevels`), plus
  strict-null gaps in the machine (`pick` possibly null in `confirm`, `anchor`/
  `entry` possibly undefined). Type-only — `effigy test:core` passes 482/0 —
  but the declaration contradicts the code. Core has no type gate in effigy
  (`test:core` is vitest, which strips types), so the error ships silently.
  Fix: one declaration line + the strict-null guards; a `tsc --noEmit` gate on
  core would have caught it at b028.

- 2026-08-12 — RESOLVED 2026-08-12 by Longhorn `777de887`. The mechanism in
  this report was wrong: `record_applied` always installs the new entry as its
  parent's preferred continuation, so a run's terminal entry has **no**
  children and its `continuationCount` is 0, not 1. Poodle's implementation
  saturates at 0 and was never affected. The report did surface a real hole
  the other way round — `ForkHistory::from_state` accepted a parent with
  children and no preference, and two Longhorn fixtures were already in that
  state, including an anchor whose 64 alternates were unreachable. Longhorn
  added a two-half guard: one child needs no preference, two or more without
  one is `MissingPreferredChild`. Original report follows.
  R4's `forkCount = continuationCount - 1` makes a **single fork
  off a run's last entry invisible**: the authority defines
  `continuation_count` as all children ("this page's own next entry included…
  a run's last entry is always zero", `ForkEntryRecord` doc), so a last entry
  with one divergent child carries 1 → `forkCount 0` → no disclosure
  affordance, no way to open the fork. Verified in the Longhorn prototype
  (`projection/project.rs`: `continuation_count = child_ids(entry).len()`);
  the v3 disclosure model therefore cannot reach such a fork at all, while the
  authority's own doc line claims the case never occurs. Either the authority
  forbids recording a fork at the preferred chain's leaf (making the doc line
  a true invariant), or v3 needs a head-fork affordance. Affects g13.029
  rendering and the Longhorn thread.

- 2026-08-12 — RESOLVED 2026-08-12 (b027 Part 1, `1331b5e5`: the twelve specs
  now carry `dismiss_on_outside_interact` — default `true`, matching the web —
  and `OPEN_GAPS` is `{}` again; each renderer resolves the field). Original
  report follows. A contract cannot document a prop the matching `*Spec` struct
  deliberately lacks without `effigy docs:lint` failing: the contract ↔
  poodle-specs drift gate (`packages/svelte/preview/scripts/contract-spec-drift.ts`)
  requires every documented Public Prop to exist on the Spec. g13-026 needed
  `dismissOnOutsideInteract` documented on twelve specs (`select`, `menu`,
  `context-menu`, `menubar`, `navigation-menu`, `split-button`, `theme-select`,
  `ref-select`, `model-picker`, `order-by`, `list-card`, `filter-builder`) that
  deliberately model no dismissal (default matches every native platform, so a
  field would be invented data) — carried via `OPEN_GAPS` entries instead,
  which the card's writable paths exclude. Same decision-shaped hole as the
  g13.009 `initialFocus` papercut: either the specs gain the fields or the gate
  needs a sanctioned carve-out for "default-is-platform-standard" props. Decide
  once, then burn down both gaps (delete the `OPEN_GAPS` entries).

- 2026-08-12 — RESOLVED 2026-08-12 (b027 Part 2, `c04a9cdc`: the parser no
  longer reads a comma inside a string literal as a prop boundary — regression
  test on the exact `placeholder = "Select date, time, and zone"` line —
  Snippet-typed props are separated from props, and the gate now enforces the
  reverse direction, exiting non-zero on undocumented props). Original report
  follows. `contract-prop-drift` only checks that documented props are
  implemented, never that implemented props are documented, so an undocumented
  public prop can never fail the gate. The reverse direction exists behind
  `DRIFT_REPORT=1` but never exits non-zero and mixes snippets (`children`,
  `footer`) in with real props. It also has a depth bug: it reports `and` and
  `time` as props of `date-time-zone-picker`, both lifted from inside
  `placeholder = "Select date, time, and zone"` and `defaultValue = { date:
  null, time: null, timeZone: null }`, though the comment at
  `contract-prop-drift.ts:51` says default values and object literals are
  skipped. Fix the parser, separate snippets from props, then enforce.

- 2026-08-11 — RESOLVED 2026-08-12 (`5854634c` regenerated the artifacts,
  `761f81d8` added the `gate:snapshot`/`gate:clean` guard that now fails any
  gate which rewrites a committed artifact). Original report follows.
  `effigy docs:check` fails at HEAD once the parity reports
  regenerate: `packages/svelte/preview/artifacts/parity-report.json` reports
  201 exports / 164 components (`HistoryCenter`, from g13-b020/b021) while
  the committed report and `packages/shared-demo-app-audit.json` say
  200/163, so `docs:lint`'s audit-json consistency check fails. The report
  generators have no `--check` mode (b015 failure mode 8), so the committed
  report drifted silently and the first regeneration after any component
  lands breaks the gate. Either regenerate+commit the report artifacts
  together with component landings, or give the report generators a check
  mode wired into `docs:check`. Affects every gate run after a component
  lands.

- 2026-08-11 — (the `isForkPoint` instance was fixed in b023 review; the
  structural point stands and is why it recurs.)
  `packages/core/src/index.ts` hard-lists every per-module
  export (`export { historyCenterTransition, …, isForkPoint } from
  "./history-center"`), so a card that deletes a core export (g13-023's D1
  dropped `isForkPoint`) cannot remove it: `index.ts` is outside the card's
  writable set. Plain bun/Node ESM validates re-export bindings at link time
  (`export 'missing' not found`, probe-verified), so any consumer importing
  the package index crashes until the follow-up card edits `index.ts`;
  vitest is lenient, which is why the component gate showed only the
  HistoryCenter suites red. Same wall recurs for any future export deletion.
  Either make `index.ts` writable on export-deleting cards, or switch these
  blocks to wildcard re-exports (`export * from "./history-center"`) so
  deletions stop breaking the package index.

- 2026-08-11 — RESOLVED 2026-08-12 (`761f81d8`: added the tsconfig and the
  `*.css` ambient declaration, wired `check:svelte-components` into
  `check:svelte`; coverage went from 1 file to 449, and the two real errors it
  exposed were fixed in AudioSwitch). Original report follows.
  `packages/svelte/components` has no `tsconfig.json` and is never
  type-checked. `effigy check:svelte` runs `svelte-check` against
  `packages/svelte/install-smoke` — one file, 0 errors — so the 164 components
  are unchecked. A `class` prop passed to `Icon`, which accepts no such prop,
  shipped through every gate: it was reported from a consumer repo whose own
  type-check caught it. The prop was silently dropped, so the checkpoint pin
  also rendered unstyled. Pairs with the 2026-08-10 React entry (`react:build`
  transpiles without `tsc`): neither web runtime has a type gate. Standing one
  up will surface a backlog, so it wants its own card.

- 2026-08-11 — `box-sizing: border-box` plus `width: 100%` plus a horizontal
  margin overflows by twice the margin: border-box covers padding and border but
  never margin. Hit on `HistoryCenter`'s inline rename input, which spilled past
  the popover edge. `width: auto` is not the escape for form controls — an
  `<input>` is replaced, so auto resolves to its intrinsic size rather than
  filling. Worth a lint rule or a shared field-inset helper, since the
  combination reads as correct.

- 2026-08-11 — Components that render inside a `Popover` can set a width that
  fights the surface's. `HistoryCenter` set `width: clamp(20rem, 34vw, 26rem)`
  while passing the surface `surfaceMaxWidth: min(26rem, ...)`, so content could
  reach the surface's own maximum and then the surface's horizontal padding
  pushed it past the rounded edge. `MessageCenter` has the same shape
  (`clamp(22rem, 36vw, 28rem)` inside a 30rem surface) and only escapes because
  its numbers happen to leave room for the padding — a token change to
  `space-panel-x` would break it. Consider a rule that a popover-hosted root
  never sets its own width, or a check for it.

- 2026-08-11 — The 85-icon default icon set has no undo, redo, history, or
  pin glyphs (no `corner-up-left`, `rotate-ccw`, `pin`, …), so the new
  `HistoryCenter` titlebar cluster falls back to `arrow-left` / `list` /
  `arrow-right` and checkpoint pins to `git-commit-horizontal`. Tooltips and
  labels carry the semantics, but canonical undo/redo/pin affordances need
  the icon set extended (an icon-crate change, not a component change).

- 2026-08-11 — Consumers are compensating for Poodle API gaps with `:global`
  CSS overrides, and those fail silently when Poodle refactors: a scan found
  `.poodle-tabs*` overrides in 9 repos, including three still targeting the
  pre-rename `[data-variant="text"]` selector. Overrides on `__list`, `__tab`
  and `__panel` (figmatic, soundcheck-library, nucleus, underlay,
  loophole-legacy) are each an unmet prop. Treat the override inventory as an
  API-gap backlog, and note that consumer migrations must audit CSS selectors
  as well as props — a selector break produces no type error and no build
  error, just wrong layout.

- 2026-08-11 — happy-dom evaluates `@media` rules only at stylesheet parse
  time: changing the window width afterwards updates `matchMedia` but not the
  cascade, so `getComputedStyle` keeps the pre-resize rules (verified with a
  standalone happy-dom probe). A responsive-layout test must set the width
  *before* injecting the stylesheet, which is fragile when the CSS arrives via
  a component import (vitest stubs CSS anyway, so tests re-inject it).
  Affects any future media-query behavior test in the component suites.

- 2026-08-11 — `cargo build --manifest-path packages/jetstream/preview/`
  cannot resolve its poodle deps from a g13 worktree: the sibling
  `jetstream-poodle` crate (`~/Dev/projects/poodle-wt/jetstream/crates/
  jetstream-poodle/Cargo.toml`) hard-points `../../../poodle/packages/…` at the
  main checkout, which does not exist alongside the worktrees — so the
  jetstream preview never compiles here (g13-013 hit the same wall and logged
  it). The workaround for a one-off check is temporarily repointing those
  paths at the worktree; it should not need to exist. Consider canonicalizing
  the sibling path or documenting the required checkout layout.

- 2026-08-11 — **Resolved 2026-08-12.** The docs preview's global
  `button:focus-visible, input:focus-visible` outline
  (`packages/svelte/preview/src/app.css`) outranked every component that draws
  its own focus treatment: a bare element selector plus pseudo-class is (0,1,1)
  and beats a component class at (0,1,0). 33 component stylesheets set
  `outline: none` for exactly this reason, so all of them were being overridden
  — TextInput visibly rendered its rounded focus border and the preview's
  square outline at the same time. The first fix narrowed it with
  `:not([class*="poodle-"])`, which made it match nothing: every focusable
  element in the shell is a Poodle component or carries a `poodle-` class
  (measured 0 matches across four pages). Dead, but still dangerous — a shell
  copied into a host app takes the rule with it and it starts matching that
  host's chrome. Now deleted. Chrome that needs a ring gives itself one, keyed
  to its own class and radius.

- 2026-08-11 — Removing an explicit `border` declaration from a `<button>`-based
  component style leaks the UA default `2px outset buttonborder`, and the two
  preview shells resolve that default differently (Svelte preview: black,
  React preview: white — a `color-scheme` difference), silently breaking the
  Svelte↔React visual gate. g13-016 hit exactly this: deleting the
  NavigationMenu trigger's unconditional border without adding `border: 0`
  produced a 0.564% pixel diff on the navigation-menu slug only. Button styles
  that go borderless must declare `border: 0` explicitly (the `.poodle-tabs__tab`
  and Menubar trigger pattern). Affects any future border-removal on a button.

- 2026-08-11 — The visual gate cannot catch a bug in a brand-new specimen:
  with no prior baseline to differ from, a new specimen reports no diff. `b013`
  added `activeOutline` specimens and the sweep showed zero tabs diffs, while
  the outline was in fact drawing a square border around a rounded tab. Only a
  human looking at it found it. Consider failing, or at least reporting, when a
  gate encounters a slug with no baseline rather than silently passing.
  Affects visual-gate trust for new components and new specimens.

- 2026-08-11 — Both the Svelte↔React parity gate
  (`test/parity/component-parity.test.tsx`) and the React smoke sweep
  (`packages/react/components/test/smoke.test.tsx`) enumerate React exports
  with `typeof comp === "function"`, and React `forwardRef` returns an element
  type object, not a function — so every ref-forwarding component silently
  drops out of both gates. MenuSurface and (since b010) TextInput already sit
  outside coverage this way; g13-014's AppHeader joined them. Counts drift
  down with no signal. Accept element-type objects in the filter (a
  `$$typeof` check) so forwardRef components stay gated. Affects Svelte↔React
  parity and smoke coverage.

- 2026-08-11 — `packages/tokens/scripts/build-tokens.ts --check` cannot see a
  stale committed artifact: `writeFile` only writes/compares files the
  generator still emits and never deletes (`build-tokens.ts:233-251`), so a
  token file removed from the generator stays committed forever and the gate
  passes. Only the svelte mirror deletes on regenerate
  (`syncSvelteTokenArtifacts`, `:270-274`); `scripts/build-default-icons.ts`
  has the right pattern (stale set, deleted in write mode / failed in check
  mode, `build-default-icons.ts:176-197`). Port the icons stale-detection to
  `audit:tokens`. Affects token drift gating (found by card 015 research).

- 2026-08-11 — The web visual gate's fixed ports (4174/4180) are squattable by
  any other worktree's dev server: a stale `vite --port 4180 --strictPort`
  left running by a previous session makes the gate's strict-port spawn die
  instantly while `waitForPort` polls the squatter's 404s until the 60s
  timeout, with the failing port hidden because the spawned preview's output
  is ignored. `test/visual/run.ts` should pre-flight `isUp` + a content probe
  (200 + expected root) before spawning, or the gate should pick ephemeral
  ports. Affects every card that needs visual enumeration.

- 2026-08-11 — g13-013's batch-card Known State claimed `render_underline`
  renders "no accessories at all — no icon, no count, no close", but
  `build_tab_label` already renders icon+count in every renderer (only close
  was missing). Card Known State is not always current against the working
  tree; workers must verify rather than trust "verified — build on this".
  Affects card-authoring accuracy and worker trust.

- 2026-08-11 — `effigy docs:lint` requires every Cargo.toml under
  `packages/` to be registered in `packages/release-manifest.json` (reverse
  check in `packages/svelte/preview/scripts/lint-docs.ts`
  `validateReleaseOperations`), so a new contract crate cannot pass the docs
  gate until the orchestrator adds its entry — but card writable-path rules
  exclude the release manifest. g13-011 shipped `poodle-ir` with `docs:lint`
  failing on exactly this error and recorded the conflict in its batch log
  (entry shape already fixed by ruling R1: kind `contract-crate`, channel
  `preview`). Either extend the lint to accept a "registration pending" state
  for new crates, or make the release-manifest writable on crate-creation
  cards. Affects new-crate onboarding.

- 2026-08-11 — `effigy visual:report` (sweep tier) reports 53 failing
  Svelte↔React pairs at HEAD (308 compared; 46 size / 4 capture / 3 pixels,
  spread across both themes — e.g. `tree`, `xy-pad`, `tool-call`, `fader`,
  `agent-plan`) with no allowlist or debt inventory covering them, so report
  mode cannot distinguish pre-existing parity debt from a card's real
  regressions without a manual before/after comparison (g13.008 had to run
  the gate twice and diff the summaries). Track the failing-pair set (committed
  allowlist with reasons, or a parity-debt inventory) so cards can diff
  against it. Affects visual-gate triage.

- 2026-08-11 — A new public web prop cannot be documented in a contract
  without `effigy docs:lint` failing: the contract ↔ poodle-specs drift gate
  (`packages/svelte/preview/scripts/contract-spec-drift.ts`) requires every
  documented Public Prop to exist on the matching `*Spec` struct, so g13.009's
  `initialFocus` (a focus-behaviour semantic, not web plumbing) cannot ship
  without `DialogSpec`/`FormDialogSpec` gaining `initial_focus` — stopped the
  card; see `docs/logs/2026-08/11-g13-009-dialog-initial-focus.md` §5. Either
  the Specs need the field (native renderers own focus per dialog.md §10) or
  the gate needs a sanctioned web-only carve-out; a decision is required
  before the next prop of this kind lands.

- 2026-08-11 — Batch card 006 (button tone parity) lists `split-button.css`
  as "implements danger", but the contract amendment it depends on
  (`282ce489`) added a `--poodle-split-shadow` column to split-button.md §8
  "Tone: danger" that the stylesheet never implemented — primary danger
  resolves `none` against a mandated elevation shadow (stopped the card;
  see `docs/logs/2026-08/11-g13-006-button-tone-parity.md` §5). Card gap
  tables should be re-verified against amended contracts at dispatch time.

- 2026-08-11 — `docs/guides/svelte-developer-guide.md` §Types still defines
  `type ButtonTone = "default" | "danger"` (2 members) while
  `docs/contracts/004-shared-control-types.md` canonically defines all four
  (`default | danger | success | warning`); the same block restates
  `ControlSize`/`ControlDensity`/`SemanticControlSizeRole`/`ButtonVariant`/
  `ValidationState`/`StatusTone` with no link to 004. The guide contradicts the
  canonical contract and misleads docs-based type resolution. Refresh the block
  or replace it with a 004 reference. Affects docs-authority consistency.

- 2026-08-11 — The g13.002 generated-LOC measure globs
  `packages/core/src/tokens/generated/*` and `packages/tokens/artifacts/**`
  return 0 lines without bash `globstar`: the first matches only the
  `css/`/`ts/` subdirectories and `**` degenerates to `*`. Actual counts are
  1,813 lines / 25 files and 2,407 lines / 31 files. Document the recursion or
  use an explicit recursive glob. Affects reproducible measurement.

- 2026-08-11 — `pilot-expressiveness-corpus.md` §8 counts `EXT` as
  Button 5 / RangeSlider 3 / TextInput 2 (10 total), but the row-level
  classification marks only 3/2/1 (6 total). The manifest records both; the
  count table drifted from the rows. Recompute the §8 table from the rows.
  Affects corpus arithmetic.

- 2026-08-11 — Jetstream RangeSlider densities specimen
  (`packages/jetstream/preview/src/specimens/range_slider.rs`) renders the
  standard variant, while range-slider.md §13 / `RNG-25` specify density
  specimens use embedded bipolar (Svelte/React do). Align the Jetstream
  densities group. Affects cross-runtime specimen parity.

- 2026-08-11 — `docs/parity/text-input.md` status line reads `gpui=2
  jetstream=2` but its own gap sections enumerate 8 GPUI / 9 Jetstream open
  `[ ]` bullets. The pass-level count drifted from the bullets (distinct from
  the stale-path lag of `OBS-04`). Refresh the status line. Affects parity-doc
  readability.

- 2026-08-11 — `effigy docs:check` rewrites the committed
  `packages/tokens/artifacts/rust/*` via `report:parity` → `tokens:build`,
  leaving a dirty worktree, and `effigy audit:tokens` fails at HEAD: commit
  `45caae82` rustfmt-formatted the generated Rust artifacts without updating
  `packages/tokens/scripts/build-tokens.ts`, whose emitter writes 4-space
  override arrays. Align the generator with the committed formatting or
  regenerate the artifacts from it. Affects the docs gate and token drift
  audit.

- 2026-08-11 — React ports of Button and TextInput omit contract-listed
  web-native props: `Button.tsx` has no `formenctype`/`formmethod` passthrough
  and `TextInput.tsx` has no `autocorrect`, all three documented in the
  contracts and implemented in Svelte. Add the props to the React components.
  Affects Svelte↔React surface parity.

- 2026-08-11 — A playbook-style detached `nohup omp -p` launch exited
  immediately with an empty log and untouched worktree under the Codex exec
  harness. Document a harness-safe detach or persistent-session recipe.
  Affects worker orchestration from agent threads.

- 2026-08-10 — `effigy doctor` runs a failing health check that reformats
  generated Rust token artifacts and unrelated Rust tests, leaving a dirty
  worktree. Make doctor diagnostics read-only or isolate formatter output.
  Affects routine repo-health checks.

- 2026-08-10 — `effigy react:build` runs Vite transpilation without a TypeScript
  check; direct `tsc` finds pre-existing `AgentChatStatus` errors the build
  reports as green. Add a React consumer type-check selector and CI lane.
  Affects React parity work.

- 2026-08-10 — A combined `git log -G` secret-pattern history scan ran for
  more than 90 seconds without progress or a result. Add a bounded, cached
  tracked-tree and history secret scanner. Affects public-release audits.

- 2026-08-10 — `effigy tasks` advertises a `distribution` built-in, but
  `effigy distribution --help` resolves it as an undefined catalog task and
  the general help omits it. Align discovery and command routing. Affects
  package-publication audits.

- 2026-08-10 — `effigy graph explore` can hang after a successful index refresh
  without progress or timeout output. Add a bounded query timeout and a clear
  fallback diagnostic. Affects agent code-navigation workflows.

- 2026-08-10 — `cargo fmt --manifest-path` follows Poodle's linked Rust
  workspaces and reformats unrelated crates, creating broad release-sweep
  churn. Add an Effigy formatter that accepts an explicit changed-file set.

- 2026-08-09 — Vite JSON named exports omit kebab-case `lucide-static`
  keys, so bundle-safe imports cannot express most icon names. This blocks a
  plain `createIconSet({ ... })` helper over `icon-nodes.json`; add a build-time
  extractor or a per-icon node package surface. Affects web icon adoption.

- 2026-08-06 — `effigy doctor` reports the repo's `isolation` manifest key as
  unsupported, so routine health checks cannot go green on the checked-in
  manifest. Align the manifest schema or update Effigy's accepted config keys.


- 2026-08-12 — RESOLVED 2026-08-13 (g13-044): `hasMounted` is now `$state`,
  so the persistence `$effect` re-runs on mount and on every preview-mode
  change; the Svelte preview writes `?theme=…&density=…&controlSize=…` back
  to the URL exactly like React's `useEffect` (browser-verified, incoming
  params preserved). Original report follows.
  Svelte preview's SHELL-08 URL persistence never fires: in
  `packages/svelte/preview/src/App.svelte`, the `$effect` that writes
  `theme`/`density`/`controlSize` back into the URL is gated on
  `hasMounted`, a plain `let` in a runes-mode component — not reactive, so
  the effect never observes the state it is supposed to re-serialize.
  Measured live (g13-035): clicking density/size updates the top-bar pills
  and `data-theme` but `location.href` keeps the pre-change query string,
  with no console error; the React preview's equivalent `useEffect` does
  persist (`?theme=forest&density=default&controlSize=lg` after clicks), so
  the two web shells drift on SHELL-08. Either make `hasMounted` `$state`
  (or drop the guard and run the write on mount) or route the effect through
  `$effect` with an explicit reactive flag. Affects the shared-preview-shell
  parity story once 036 lands.

- 2026-08-12 — RESOLVED 2026-08-13 (g13-044): both `CatalogueLanding`
  surfaces now derive their grid groups from the `components` prop, so the
  grid card count equals the header count under any query
  (browser-verified: 6/6 with query `date`). Original report follows.
  Both catalogue-landing grids ignore their filtered component
  list: `packages/svelte/preview/src/pages/CatalogueLanding.svelte` and the
  React mirror render `componentsByTag()` (every component) and use the
  `components` prop only for the count line. With a search query active, the
  sidebar filters correctly and the header reads "2 components" while the
  grid still renders ~164 cards (measured live in both previews, g13-035).
  The search axis itself works — the data flows — but the landing grid
  defeats the filter's visible result. Fix: derive `groups` from the
  `components` prop (filter `componentsByTag()` items by the passed set)
  in both files.

- 2026-08-12 — Two findings from the SettingsShell specimen.

  **RESOLVED 2026-08-13. Poodle's Svelte `Button` silently drops `onclick`.**
  Button now composes the DOM-spelled handler alongside its own for the three
  it binds (`onclick`, `onfocus`, `onblur`) rather than overwriting it, with a
  mutation-checked test. Button is the only component with this shape — 1 of
  166 spreads `restProps` *and* binds a native handler — so the fix is
  targeted, not a pattern. Original report follows. Its prop is `onClick`;
  `onclick` falls into `restProps`, which is spread at `Button.svelte:146` —
  *before* the component sets its own `onclick={handleClick}` at `:172`. The
  component's handler wins and the consumer's is discarded with no error. The
  SettingsShell specimen shipped with five dead buttons for exactly this reason
  and looked, from outside, like a broken component. React is unaffected —
  `onClick` is its own convention there. Any `on*` prop the component also binds
  natively has the same shape.

  **`DetailItem`'s responsive rule keys off the wrong box.** It declares
  `container-type: inline-size` on `.poodle-detail-item` and then writes
  `@container (max-width: 26rem)` rules targeting `.poodle-detail-item` itself
  — but an element cannot be matched by its own container query, so the rules
  resolve against the nearest ancestor container instead. Measured inside
  SettingsShell: the item is 240px wide, its ancestor container is a page-wide
  `.poodle-detail-section`, so the 26rem query never fires; the label column
  takes its `11.25rem` max and the value column resolves to **20px**, at which
  `word-break: break-word` breaks values character by character ("Da rk",
  "Co m pa ct"). Any narrow grid cell hits this, not just SettingsShell. The fix
  is a layout decision on a shared component with three layouts and two
  presentations, so it wants its own card rather than an improvised column
  tweak.

- 2026-08-13 — `detail-section.css`'s `[data-separated="true"]::before` (a
  0.125rem separator inset) sits inside the section's own `@container
  (max-width: 28rem)`, so it resolves against the enclosing
  `DetailSectionGroup` rather than the section. Plausibly the box you want for
  a separator inset shared across sibling sections.

  **Measured 2026-08-13, and the guess above was wrong.** A `DetailSection` has
  **no ancestor container at all** in the preview, so the query never fires and
  the narrow-width inset is simply unreachable; only when nested in a
  `DetailSectionGroup` would it answer to the group. Either way it never
  follows the section's own width. Left in place: the effect is a hairline
  inset staying at its default, and fixing it means relocating the
  pseudo-element onto a child, in a component whose other `@container` rules
  correctly target `__body` and `__header`. The baseline entry now records the
  measurement instead of the guess.

- 2026-08-15 — g14.005 (Popover overlay/focus proof) recorded three
  environment/harness notes worth keeping visible: (1) the web runner's
  layer-count observation imports through a relative path
  into the core source, like the hosts' component imports; package subpaths
  and the bare entry resolve against a stale bun package cache in a clean
  checkout, which the initial CI run caught — a local global bun
  install had masked the gap. (2) happy-dom 20.11.2 implements no layout at all (`getBoundingClientRect`
  is always zero), so the web leg of the placement/offset/width claims runs
  against a box-model stub plus an anchor-box stylesheet — the same
  browser-default-simulation posture as the keydown → click harness; the
  corpus asserts only relative gaps, alignment deltas, and width match with
  named tolerances, so no runtime-specific constant leaks in. (3) gpui
  0.2.2's mousedown focuses the hit element, so a pointer press on a
  disabled trigger reports focused on both GPUI and happy-dom; the observers
  treat disabled parts as unfocused on both runtimes (browser semantics),
  which is what keeps the cross-runtime comparison green.

- 2026-08-17 — g15.005 (workstation & agent evidence) found that **no repo
  selector typechecks React test files.** `effigy react:build` builds
  `packages/react/preview` only, and there is no `check:react` counterpart to
  `check:svelte`; `packages/react/components` has no `tsconfig.json`. The
  ToolCallGroup fixtures in this card were missing the required
  `TranscriptToolCall` discriminant on both sides — `check:svelte` caught the
  Svelte copy and nothing caught the React copy. The React fixtures were fixed
  by hand off the Svelte error. A React typecheck gate would have caught both,
  and would also have caught the JSX-attribute `\n` fixture bug in the same
  tranche (a string attribute keeps `\n` literal, so the whole markdown fixture
  collapsed to one line and the assertion measured nothing). Recording the gap
  rather than inventing a gate here: adding one is a board-health change, not
  component evidence.

- 2026-08-17 — g15.005 review found a second unguarded seam: **nothing checks
  that a React component imports its shared stylesheet.** The React
  `UiPresentationProvider` rendered the contract-required
  `.poodle-ui-presentation-provider` wrapper without importing
  `@inflatable-cookie/poodle-core/styles/ui-presentation-provider.css`, which
  is the only definition of the `display: contents` that makes the wrapper
  layout-neutral. Component tests cannot catch this: the vitest DOM loads no
  stylesheets at all (`document.styleSheets.length === 0`, computed `display`
  is `block` even for the Svelte reference, which does import it), so the
  neutrality requirement is only assertable through the class hook and the
  absence of ARIA. A per-component check that every `.poodle-<name>` root's
  module imports the matching `styles/<name>.css` would close this; it is
  board health, not component evidence, so it is recorded rather than built.

- 2026-08-17 — The optimized GPUI preview **test binary** can SIGBUS rustc
  inside `gpui_macros`, including with a fresh `CARGO_TARGET_DIR` and
  incremental compilation disabled. `effigy check:gpui` and the mounted
  regression binary compile cleanly, so this is specific to the oversized
  `poodle-preview` unit-test target. Its bin-unit cases need a smaller test
  target or non-optimized test profile; adding more tests to the binary makes
  the compiler failure easier to hit.

- 2026-08-21 — g15.042 found that **the node backend's generated element-id
  counter is a process global**, so two test threads that render node trees at
  once make id-less controls unclickable. `reset_element_ids` restarts
  `NEXT_ID` once per rendered frame precisely so a node that declares no id
  keeps the same `ElementId` across the frames a real click spans; with a
  second thread resetting the same counter, the id can change between a press
  and its release, and gpui — which keys the pending mouse-down by that id —
  drops the click with no error. The new Stepper route probe passed alone and
  failed inside `probe:gpui-specimens`, whose sweep runs four shards in
  parallel. Worked around with an `RwLock` in `specimen_probe.rs`: shards share
  it, a test that clicks node-backed controls takes it exclusively. A real fix
  would make the counter thread-local, which is where every other backend
  registry (`FOCUS_HANDLES`, `ELEMENT_BOUNDS`) already lives — small, but it is
  backend ownership rather than this card's seam.

- 2026-08-22 — g15.047 rediscovered that **any new multi-capture Playwright
  harness must recycle its page and restart dead previews**: a single page
  degrades after ~15-20 SPA navigations (vite client state accumulates) until
  `waitForSelector` stops seeing markers, and a preview spawned earlier in a
  long batch can die mid-run. The knowledge exists in `test/visual/run.ts`
  (`RECYCLE_AFTER`, `ensureUp`) but is not exported as a helper, so the
  Button-comparison harness (`test/visual/button-comparison/capture-web.ts`)
  re-learned it from a timeout at fixture nine. A small shared
  `captureSession()` helper in `test/visual/` would save the next harness the
  same failure.

- 2026-08-27 — `effigy graph index --json` rebuilt the graph successfully and
  `effigy graph status --json` reported `ready`, but the indexing process stayed
  alive beyond the command's 30-second execution window with no final envelope.
  Make successful index completion terminate promptly or emit a diagnostic for
  the remaining work. Affects graph-assisted orchestrator checkpoints.

- 2026-08-28 — g16.020 worker preflight found that the handoff's declared
  `base_commit` (`69118d83173d3d69b284b5ecf6d7315dc43ae5a8`) is not a Git object;
  the actual ancestor resolves from short `69118d831` to
  `69118d83122e976d256af6033e57d1c8e6b1a9de`. Friction: dispatch validation
  cannot verify the literal handoff value. Impact: workers need a manual
  ancestry check before trusting the base. Fix: validate the full SHA when the
  handoff is generated, or reject it before dispatch. Surface: Northstar
  worker-pr-loop handoff validation.

- 2026-09-02 — g16.048's disposable Playwright runner stalled while launching
  Firefox after Chromium in one multi-engine process, although an isolated
  Firefox launch completed. The bounded workaround is one `--browser=` run per
  engine and receipt consolidation; a future runner should isolate launches or
  apply an engine-start timeout. Surface: disposable browser benchmark runner.
