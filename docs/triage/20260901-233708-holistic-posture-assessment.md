# Holistic Posture Assessment — Advisory To The Orchestrator

Status: open — advisory input; nothing here is execution authority
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator (promotion); authored by an independent
Fable advisory thread at the operator's request
Source: front doors, roadmaps, triage, papercuts, sibling consumer repos, npm,
and five delegated read-only code audits (native pair, validation gates, web
pair, docs/planning, packaging/consumers). Audit reports were kept outside the
repo; every claim below was spot-verified against the tree before inclusion.
Promotion route: see the final section

## Thesis

Poodle's value centre and cost centre have separated.

- **Value centre:** `@inflatable-cookie/poodle-svelte` + `poodle-core`.
  Nineteen app manifests across 16 sibling repos consume it, every one pinned
  to npm `0.2.2`, most importing 20–80 distinct components.
- **Cost centre:** the native Rust pair. No shipping product depends on
  `poodle-render`, `poodle-gpui`, or the node backend. Longhorn's
  `crates/longhorn-poodle` pulls only `poodle-specs` (data types) by git tag and
  is used only by Longhorn `prototypes/`. Jetstream's `jetstream-poodle` is a
  path-linked engine adapter, not a product. Finch's GPUI app is archived.
  Loophole desktop is Tauri + Svelte; Loophole's own vision says
  "Tauri-first / GPUI-second … GPUI backend later".
- **Effort split since 2026-08-01 (lines changed):** gpui 213k, render 103k,
  jetstream 100k, codegen 82k, contracts 56k versus svelte 93k, react 64k,
  core 54k. Native ≈ 2.2× web. Most g16 cards exist to move one or two
  "GPUI mounted" ledger cells; 119 remain at roughly one cell per card.
- **React:** 176 shells, zero consumers, unpublished, and prop drift against
  Svelte in 32 components is not gated.

This is not "stop native". Loophole's GPUI-second posture is a real bet. The
advice is to pace the native pair to a named consumer milestone and to put the
things that the 16 live consumers feel today at the front of the runway.

## Verified Facts

| Fact | Evidence |
| --- | --- |
| npm `latest` is `0.2.2`; no `0.2.3` on npm; no `v0.2.3` git tag | `npm view` both packages; `git tag` |
| `CHANGELOG.md` and `docs/release-notes/README.md` record `0.2.3` as released 2026-08-30; all package.json/Cargo.toml say `0.2.3` | `CHANGELOG.md:14`, `docs/release-notes/README.md:23` |
| `effigy qa` is red on `main` | `audit:security` OpenAI regex (`scripts/audit-repository-security.ts:23`) has no left boundary; now matches 5 files (PAPERCUTS recorded 1) |
| All five GitHub workflows are `workflow_dispatch` only; no push/PR trigger exists; `effigy qa` runs only inside a manual `release.yml` dispatch | `.github/workflows/*.yml` `on:` blocks |
| `effigy qa` on `main`: 2:47 wall-clock, single red child `audit:security`; ledger "mounted" is a static name map (`parity-evidence-ledger.ts:72,444,512`) | validation audit run 22:29Z |
| `effigy doctor` reports 3 error classes: 30 generated-in-src, 67 god-files (12 critical), 22 stale suppressions (19 errors) | `.effigy/reports/doctor/` |
| Published web packages are raw source: 0 `.js`, 0 `.d.ts`; `files: ["src"]`, exports point at `src/index.ts` | `packages/svelte/components/package.json`, packed tarballs |
| Neither web package declares `sideEffects`; 172 Svelte components import their own CSS as a side effect | grep; fresh Vite app with Button + Select builds a 684 kB CSS bundle containing list-card and tabs styles |
| `marked` is a hard dependency of core, svelte, and react; used by 2 of 176 components | package.json files; `AgentMessage.svelte`, `MarkdownEditor.svelte` |
| `poodle-core` root exports 751 symbols; ~45% have no consumer in either shell; test fixtures (`MODEL_*_FIXTURES`) and clip/motion internals are public | `packages/core/src/index.ts` |
| Thin-shell claim does not hold for the large composites: DataTable 632/604 lines Svelte/React with 0 core; Button 264/185 with 0 core; Select, Tabs, Tree, MarkdownEditor duplicate loader, overflow, rename, and cursor logic per shell | line counts; paired `file:line` ranges in the web audit |
| No drift gate compares React props to Svelte or to contracts; 32 components diverge (React-only `defaultValue`s, missing `formenctype`/`formmethod`, `ToastStack.onDismiss` arity, attribute casing) | `packages/svelte/preview/scripts/contract-prop-drift.ts` parses Svelte only |
| `packages/core/src/file-upload.ts:102` uses `document`/`Image` outside `dom/` | architecture 006 rule |
| Consumer-reported Poodle defects live only in consumer `PAPERCUTS.md` files; Poodle has zero GitHub issues | Figmatic: Tree treeitem has no accessible name, `Select variant="ghost"` ignored in native mode, core `licence.ts` type drift; Loophole: Keyboard vertical geometry, DockRegion `canAcceptPanel` gating |
| AGENTS.md hard rule and product-guardrails say Underlay must wrap Poodle behind adapters; architecture 001 §Host says Underlay imports Poodle directly (settled by g12.022). Underlay templates import 64 components; its 5 apps declare Poodle directly | `AGENTS.md:10`, `docs/architecture/001-poodle-system-shape.md:206` |
| All 17 Rust crates declare `publish = false`; GPUI guide tells consumers to use path deps | Cargo manifests; `docs/guides/gpui-developer-guide.md` |
| Docs volume: roadmaps 446 files, logs 341, handoffs 125 (121 uncited by any canonical doc; no retention policy), parity 141 (marked historical yet 4 g16 cards instruct editing it), specs 70 (28 unreferenced; rollover purge never ran) | counts; `docs/parity/README.md` vs `docs/roadmaps/g16/010-*.md:13` |
| `docs/roadmaps/g16/README.md` is 3,873 words of PR narrative; `docs/roadmaps/README.md` 1,685; same history repeated in logs and generation index | word counts |
| Guides teach retired APIs: `Tabs variant="underline"` (`svelte-developer-guide.md:903,908`), `ButtonTone` with 2 of 4 members (`:1226`), pre-state Popover trigger snippet (`:820`), HistoryCenter v1 props in `component-docs.ts:5135` | grep |
| Contract/spec drift the gates miss: `slider.rs:91` returns 0 for `max <= min` while contract and core widen to `min + 1`; `checkbox.rs:14` `description_id` vs contract `describedBy`; contracts index lists `token-input.md` twice | grep |
| 112 remote branches, 98 already merged into main and not deleted | `git branch -r --merged` |
| Paseo profile "Gemini Flash Worker" resolves to `cursor/grok-4.6` | `list_profiles` |

## Findings By Lane

### A. Release truth (blocker)

The repository claims a release that does not exist. Every consumer is on
`0.2.2`; the guide tells them to install `0.2.2`; the changelog says `0.2.3`
shipped. The in-flight PR #144 correctly moves the HistoryEntry break to
`0.3.0`, but it inherits the ghost `0.2.3` baseline. Decide `0.2.3`'s fate
(publish it, or fold its ContextMenu change into `0.3.0` and rewrite the
changelog entry as unreleased) before any release lane runs. Bind future
changelog headers to a publication receipt so this cannot recur.

### B. Green main (blocker, cheap)

`effigy qa` cannot pass on `main`. The fix is a `\b` on one regex and has been
in PAPERCUTS since g16.028 closed. A red release board silently trains every
worker to treat qa failure as noise. Fix it in the next mechanical batch and
add a scheduled or post-merge `effigy qa` on `main` so a red main is a paged
event, not a papercut.

### C. Consumer packaging (high, small cost, large payoff)

Three one-batch changes improve every one of the 16 consumers:

1. Add `"sideEffects": ["**/*.css"]` to core and svelte package.json so barrel
   imports stop bundling all 172 component stylesheets.
2. Move `marked` behind the two components that use it (peer/optional or a
   subpath export) so a Button consumer does not ship a markdown parser.
3. Decide whether to emit `dist/` (svelte-package + `.d.ts`). Today the
   package is only consumable by Svelte 5 + Vite/SvelteKit toolchains. That
   may be an acceptable pre-1.0 boundary if stated in README; it is not
   stated.

### D. Web pair architecture (high)

The "thin shell over core" contract is true for primitives (Checkbox 47%
shell) and false for composites (DataTable, Button, Select, Tabs, Tree,
MarkdownEditor, HistoryCenter). Every composite fix is written twice and
React quietly drifts because nothing gates it. Options:

- Gate first: extend `contract-prop-drift.ts` to parse React `Props`
  interfaces and fail on Svelte↔React divergence. Cheap, immediate.
- Then extract the pure parts (CSV export, selection math, tab overflow,
  async option loader, rename state) into core one composite per card.
- Or decide React's fate. With no consumer and no publication, React costs a
  full second implementation of 176 components. If it stays, it needs a
  consumer and a gate; if it goes, that is an explicit operator decision the
  vision must record.

### E. Native pair (pending)

The native-pair audit (poodle-headless/poodle-render/node-backend boundary,
god files, dead IR crates, `too_many_arguments` clusters, test reality of
"mounted" cells, Jetstream cost) is still running. This section will be
updated in a follow-up commit on this branch.

### F. Validation gates (high)

Measured on `main` at 22:29Z: `effigy qa` takes 2:47 wall-clock, 3,498 web
tests and 167 native regressions pass, and the only red child is
`audit:security`. The lattice is large and mostly fast; its problems are
shape, not cost.

- **No automated board exists.** Every workflow under `.github/workflows/`
  is `workflow_dispatch` only, by a documented decision after macOS lanes
  exhausted the Actions allowance. `effigy qa` is the release gate but runs
  only when a human dispatches `release.yml`. At 28 merged PRs in one day,
  "main is green" is whatever the last worker said it was. A cheap headless
  push/PR board (ci:web + ci:rust, Linux, no macOS) restores a shared signal
  for a few minutes of runner time per merge.
- **"Mounted" is a name map, not execution.** `parity-evidence-ledger.ts:72`
  is a component→test-name table; a cell is `mounted` when the map has an
  entry and the string appears anywhere in the cited file (`:512-529`). The
  generator never runs cargo or a browser. Fine as an expected-test manifest;
  it should not be read as proof, and the g16 runway reads it as proof.
- **Red or unenforced checks outside the board.** `docs:machine-shape-drift`
  exits 1 with 20 findings and is in no board. `docs:value-domain-drift` has
  20 findings but is report-only unless `VALUE_DOMAIN_ENFORCE=1`. `drift:roles`
  fails because it shells into the Jetstream preview, which no longer
  compiles. `lint-docs.ts:3093` adds only contract-only prop errors, so
  Svelte-only drift is green in the composed docs gate while red standalone.
- **Coverage narrower than claimed.** `test:contracts` omits
  `packages/contracts/node`. There is no `check:react` typecheck in any board.
  Visual capture skips nine nondeterministic components.
- **Validation dirties the checkout.** `test:web-pack-install` leaves two
  tarballs in the tree (found in the orchestrator's `main` checkout after the
  audit; removed). Doctor then reads them as invalid UTF-8 and reports scan
  errors. `gate-tree-guard.ts:27` keys its snapshot on one global
  `/tmp/poodle-gate-tree-guard.json`, so concurrent worktrees can consume each
  other's state.
- **Doctor is mis-tuned.** `quality/effigy.scan.toml:72-87` scans
  generated-in-src but excludes only token/icon roots, not the committed
  catalogue/specimen roots that `tasks/effigy.tasks.toml:14-25` calls
  intentional inputs. Every `#[allow(` is scored high. A permanently red
  doctor has the same effect as a red qa: nobody reads it.
- **Denominator in seven places.** `specimen_probe.rs:41` hard-codes 175;
  the census test, ledger script, `parity.ts:754`, two JSON reports, and the
  demo audit repeat it. One `public-surface.json` manifest with per-runtime
  flags would derive all of them.
- **Flake root causes are concrete.** `window_capture.rs:820-836` builds a
  temp dir from pid + body length so parallel empty-manifest tests collide;
  the smoke wrapper drops stderr. `specimen_probe.rs:295-346` asserts a
  120 s wall clock while four shards run concurrently behind one global
  registry lock, so any neighbouring Vitest run trips it.

### G. Docs and planning posture (medium, compounding)

The spine records process faithfully and serves readers poorly:

- Roadmap README and generation index are PR journals. A contributor needs
  several thousand words to learn "what is ready now". Reduce both to a status
  table and leave narrative to logs.
- Handoffs and parity have no retention policy; 121 handoffs and 141 parity
  files are dead weight that agents still read and sometimes edit. Archive
  them and add one sentence of policy to `docs/README.md`.
- 28 specs are unreferenced; spec 001 is still `draft`. Run the rollover purge
  the working rules already mandate.
- Guides teach removed APIs. Add a guide-snippet compile check (extract fenced
  Svelte blocks, `svelte-check` them against the package) so this class cannot
  recur.
- The Underlay hard rule in `AGENTS.md` contradicts architecture 001. Either
  rule is defensible; the always-loaded one must match the settled one.

### H. Process observations

- There is no inbound defect channel. Consumers record Poodle bugs in their
  own PAPERCUTS and they never reach this runway. A recurring "consumer
  papercut sweep" lane (grep 16 sibling PAPERCUTS files for `poodle`) would
  have surfaced the Figmatic Tree accessibility name defect and the Select
  ghost-variant defect weeks ago. Both are ledger-invisible: the ledger counts
  runtimes, not users.
- The ledger drives card selection, and the ledger measures a runtime nobody
  ships. Add a second selection input: consumer impact (components × consumers
  × reported defects). Nucleus imports 29 components, Acowtancy 79; a Select
  regression hurts 14 repos, a GPUI mounted cell hurts nobody today.
- 98 merged branches remain on the remote. Enable delete-on-merge.
- Doctor is red on every run, so its signal is ignored. Either tune the scan
  config for checked-in generated catalogues or act on the god-file list;
  a permanently red doctor is the same problem as a red qa.

## Recommendations, Ranked

1. **Release truth and green main** — one mechanical batch: regex boundary
   plus a prose fixture, `0.2.3` disposition, changelog rewrite, pack-install
   output to a temp dir, worktree-keyed gate state, delete merged branches.
   No design decisions, no contract changes. Do this before PR #144 lands.
   Then a Linux-only push/PR board running `ci` so main has a shared signal
   (needs operator approval for workflow edits).
2. **Consumer packaging trio** — `sideEffects`, `marked` isolation, README
   statement of the toolchain boundary (or a `dist/` build if the operator
   wants a broader boundary). Ships in `0.3.0` alongside the HistoryEntry break
   so consumers absorb one migration.
3. **React gate or React decision** — extend prop drift to React now; ask the
   operator whether React continues without a consumer.
4. **Consumer papercut sweep lane** — recurring, cheap, and it feeds the runway
   with defects real users hit. Promote the two Figmatic defects and the
   Loophole Keyboard geometry as the first cards.
5. **Docs compaction** — roadmap status tables, archive handoffs/parity, spec
   purge, guide snippet check, Underlay rule reconciliation. Docs-only, can run
   on a cheap model in parallel with everything else.
6. **Ledger honesty** — rename the mounted map to an expected-test manifest
   and have the ledger consume test execution output; admit or delete
   `machine-shape-drift`; ratchet `value-domain-drift`; add `poodle-node` to
   `test:contracts` and a React typecheck to `ci:web`; tune doctor excludes.
7. **Native pacing decision** — ask the operator to name the first GPUI
   consumer milestone (Loophole GPUI-second, or a Longhorn shell). Until one is
   named, cap native work at "keep it compiling and honest" and stop selecting
   cards by ledger cell count alone. Revisit after the pending native and
   validation audits report.

Items 1, 2, 5, and 6 are mechanical and suit cheap models. Items 3, 4, and 7
need the operator or the orchestrator's judgment.

## Questions Only The Operator Can Answer

- Is `0.2.3` published under a different mechanism, or is it a ghost? If a
  ghost, fold it into `0.3.0`?
- Does React stay without a consumer? If yes, who is its first consumer?
- Is "Svelte 5 + Vite/SvelteKit only" the intended consumer boundary for the
  web packages pre-1.0?
- Which product ships GPUI Poodle first, and roughly when? That answer sets
  the native pair's budget.
- Which Underlay rule is current: adapter-wrapped (AGENTS.md) or direct import
  (architecture 001)?

## Promotion Route

1. Orchestrator reads this note and the pending native/validation updates.
2. Recommendations 1, 2, and 6 become mechanical cards; no operator
   question is needed except the workflow-edit approval.
3. Recommendations 3 and 7 go to the operator as decisions; record answers in
   vision/architecture, then compile cards.
4. Recommendations 4 and 5 become recurring cheap-model lanes.
5. Remove this note when each item is promoted or rejected; carry any
   unresolved operator question into its own note.
