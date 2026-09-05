# g16.108 — Docs Spine Compaction

Status: complete — awaiting re-review after operator decision execution (rev 3)
Date: 2026-09-05
Card: `docs/roadmaps/g16/108-docs-spine-compaction.md` (amended on main at
`954a0252`, item 6 "Historical-prefix gates", manifest revision 14)
Dispatch: `docs/roadmaps/dispatch.md` revision 7 text; coordinator manifest
revision 14 per operator
Base: `954a025222aeb2d7f126ed7f620c2948de58efb9` (`origin/main`, promoted —
the commit that last touched `dispatch.md`; branch rebased onto it)
Worker PR: open from this branch (see closeout)
Handoff: `docs/handoffs/20260905-...-g16-108-docs-spine-compaction.md`

## Outcome

All five fixed boundaries landed in one PR. 175 closed-lane handoffs are
archived by filename month; the 140-file parity audit corpus moved to
`docs/archive/parity/` behind a pointer README; 56 specs that no current
surface references moved to `docs/specs/archive/` with one-line index
entries; the consumer guides no longer teach removed APIs (Tabs, ButtonTone,
Popover trigger state, StatusIndicator, RadioGroup, SegmentedControl, Stack,
ToastHost, IconButton, HistoryCenter v3); and `docs:snippet-check` now
compiles every self-contained guide snippet against the shipped
poodle-svelte surface and is wired into `docs:check`. Nothing open was
archived; the front doors (`roadmaps/README.md`, `generation-index.md`,
`g16/README.md`, `dispatch.md`) were not touched.

## Boundary 1 — Handoffs

184 files under `docs/handoffs/` at the base snapshot. Lane state per the
generation records: g15.001–079 complete (056 superseded, 078 cancelled) and
g16.001–105 merged except the open frontier. 175 closed-lane handoffs moved
with `git mv` (this head then adds the g16.108 handoff itself: 10 top-level
files — 9 retained briefs plus this lane's brief — and 185 total):

- `docs/handoffs/archive/2026-08/` — 96 files (g15 programme and the
  canonical g16.001–027 era, by filename month)
- `docs/handoffs/archive/2026-09/` — 79 files (g16.028–105 era)

**Kept in place (9)** — lane state open, held, or indeterminate; listed so
the reviewer can confirm nothing open was archived:

| File | Reason |
| --- | --- |
| `20260827-181210-papercuts-wave2-history-center.md` | lane close undeterminable (no card/log receipt) |
| `20260831-141135-poodle-orchestrator.md` | active orchestrator thread brief |
| `20260831-180208-paseo-project-settings.md` | no Poodle card owns this thread |
| `20260901-111247-poodle-design-intelligence-research.md` | feeds open g16.052 |
| `20260901-121256-design-guidance-pilot-planning.md` | feeds open g16.052 |
| `20260901-230407-conformance-lab-architecture-planning.md` | feeds held VL-1 lab gate |
| `20260901-230408-visual-comparison-tranche-planning.md` | gated direction, no card |
| `20260901-230409-jetstream-readiness-review.md` | Jetstream admission held |
| `20260904-083000-fresh-poodle-orchestrator-continuation.md` | current orchestrator continuation brief |

No handoff exists for open lanes g16.051/052/097/106/107 (none were ever
created), so there was nothing open-lane-owned to preserve beyond the nine
above.

`docs/README.md` gains the retention rule: a handoff is archived when its
lane closes, into `handoffs/archive/YYYY-MM/` by the month in the filename.
No link-checked file (root READMEs, `docs/README.md`, guides, package
READMEs) links into `docs/handoffs/`, so the move broke nothing.

## Boundary 2 — Parity

140 files renamed at 100% from `docs/parity/` to `docs/archive/parity/`
(139 component audits plus `TEMPLATE.md`). `docs/parity/README.md` was
replaced by a short pointer naming the archive and the current authority
sources, and the archive README records the move and re-anchors its links one
level deeper (review-corrected: the diff is 140 R100 renames plus the README
pointer rewrite and the archive README, not 141 renames).

Parity-edit instructions are gone from the g16 cards that carried them:

- `001` line 64: historical-audit guard now names `docs/archive/parity/`.
- `010`: delivery bullet and writable-scope entry no longer list
  `docs/parity/breadcrumbs.md` as an update target.
- `034`: owned-path bullet repointed at the archived skeleton audit as a
  historical record.
- `035`: governing ref and owned-path entry now point at the archived
  markdown-editor audit, explicitly not an edit target.

Oracle check `grep docs/parity/` in `docs/roadmaps/g16/*.md` now matches only
card 108's own description of this move. Historical logs were left alone
(point-in-time records; not an owned path).

## Boundary 3 — Specs

Reference rule applied exactly as the card states: a spec is referenced when
its filename or a path to it appears in `docs/architecture`,
`docs/contracts`, `docs/roadmaps/g16`, or `docs/roadmaps/generation-index.md`,
or one of those sources names it as "spec NNN". Under that rule **56** specs
are unreferenced, not the audit's 28. The divergence is explained by the
audit's method: g16 card numbers 001–061 collide one-for-one with spec
numbers, and every g16-card mention of those numbers in the README/runway
documents would have counted as a reference — the strict corpus scan cannot
reproduce 28. The card's instruction ("Compute the set") was followed with a
reproducible method; every archived spec is provably uncited by filename,
path, or "spec NNN" wording in the four sources (evidence table available on
request from the log author). 14 specs stay: 001, 008, 015, 022, 025, 026,
044, 062, 063, 066, 067, 068, 069, 070.

- 56 files moved to `docs/specs/archive/`; `docs/specs/archive/index.md`
  carries one line per archived spec (name — title).
- Seven kept specs whose `Depends on:` headers name moved specs now cite
  `archive/NNN-*.md`.
- Spec 001 header: `Status: draft` → `Status: active` (Updated 2026-09-05).
- `docs/specs/README.md` rewritten: "current cross-cutting references" now
  lists exactly the 14 kept specs with one-line descriptors, and the README
  points at the archive index. Oracle 5 holds: no moved spec is listed as
  active anywhere in the README diff.
- docs:check follow-ups: the retired-Treatment drift gate
  (`scripts/check-recipe-only-surface.ts`) exempts `docs/archive/` now that
  the parity corpus lives there, and the historical manifests
  (`packages/{ecosystem-acceptance,reference-apps,g03-closeout,
  shared-demo-app-audit}.json`) repoint their `docs/specs/NNN-*` evidence
  citations at the archive. The regenerated react `component-docs.json`
  artifact was committed with the HistoryCenter v3 docs rewrite.

## Boundary 4 — Guides

- `svelte-developer-guide.md`: Tabs snippet uses `variant="card"` and the
  variant list reads `"card" | "pill" | "block"`; `ButtonTone` has all four
  members; the Popover trigger teaches `triggerIsInteractive` with
  `PopoverTriggerState` applied to a real Button (the removed pre-state
  zero-argument Button trigger is gone, with a note on when the default
  trigger mode applies).
- The snippet check then forced further truth repairs in the same class:
  Stack `direction="row"`, StatusIndicator `status=` (not the removed
  `tone=`), RadioGroup `disabled`/`name`/`ariaLabel` (removed `id` and
  `isDisabled`), SegmentedControl `ariaLabel` (removed `id`), `lang="ts"`
  restored on seven fences that used TS syntax in plain `<script>` blocks,
  an `ariaLabel` added to a recipe IconButton that requires it, an untyped
  `MenuItem[]` literal annotated, and the shell recipe's Card `padding`
  props and store-less `<ToastHost />` corrected to the current surface.
- `packages/svelte/preview/src/component-docs.ts`: the whole
  `"history-center"` block (props table + usage) is rewritten from the v1/v3
  hybrid (`entries`/`branches`/`onSelectEntry`/`onCheckout`/
  `onLoadMore*`/`branchCount`) to the v3 `pages`/`continuations`/`run` host-op
  surface with `HistoryPathPage` data and `onNavigateEntry`.
- `docs/contracts/components/README.md`: duplicate `token-input.md` index
  line removed (one remains).

## Boundary 5 — Snippet check

`packages/svelte/preview/scripts/docs-snippet-check.ts` extracts every
fenced `svelte` block from `docs/guides/*.md` into a throwaway consumer
project at `packages/svelte/preview/.snippet-check` (deleted on success)
that depends on the poodle packages by `file:` link, exactly like
`packages/svelte/install-smoke`, and runs `svelte-check --threshold error`
against the shipped surface.

- Self-contained fences compile verbatim; markup-only fragments get
  synthesized poodle imports and implicit-any context stubs; fragments that
  import app modules (`$lib`, sibling packages), use application-owned
  components, or property-access earlier-fence state are skipped with
  printed reasons (32 of 92 fences today — app-context by construction).
- Selector `docs:snippet-check` added to `tasks/effigy.tasks.toml` and
  wired into `docs:check` directly after `docs:lint` (docs:check already
  built the package dist before that point). The one task-catalogue change
  is on its own lines; g16.107's edits on other lines rebase cleanly.
- Planted-prop proof: reintroducing `variant="underline"` in the Tabs fence
  fails with `Type '"underline"' is not assignable to type '"card" |
  "pill" | "block"'` (snippet `svelte-developer-guide-29.svelte`); reverted,
  the check is green.

## Revision 2 — Reviewer blockers (exact-head `eeb8fa2f`) and responses

1. **Stack API**: guide still taught `direction="vertical|horizontal"` at
   :729/:735 → now `"column"`/`"row"`.
2. **Stale/broken archive and spec links** (three groups + same-class):
   `docs/archive/parity/README.md` pointer depth fixed (`../../parity/…`);
   archived specs 064/065 links to kept 066 → `../066-…`; kept spec 008 body
   cites of archived 002/007 → `docs/specs/archive/…`; plus archived specs'
   `../architecture`/`../roadmaps` depth, bare mentions of kept specs
   (`../` prefix), and archive-internal root-relative cites of archived
   specs. Re-audit: 0 missing link targets inside `docs/specs/**`.
3. **`scripts/check-recipe-only-surface.ts` reverted** to `origin/main`
   (unauthorized scope expansion removed). Consequence, verified:
   `drift:recipes` now fails `docs:check` on exactly the 13 archived parity
   lines citing `--poodle-treatment-*`. The 4-line `docs/archive/`
   historical-prefix extension is semantically required for the card's
   `docs:check`-green oracle after the sanctioned relocation; it is
   **pending explicit planning/manifest authorization** and is not applied.
4. **Coordination records refreshed**: base identified as the stale
   dispatch snapshot; open cards per `origin/main` `da8c9c37` (g16.097
   closed; g16.109/110 ready; held 051/052); handoff counts clarified
   (184 base → 175 archived + 9 retained + 1 new = 185).
5. **Parity wording** corrected (see Boundary 2).

Same-class stale `docs/specs/…` mentions remain in `docs/research/`
value-track records (019, 038) and historical logs; both are out of owned
scope and not link-checked — left as point-in-time records.

## Revision 3 — Operator decision execution (2026-09-05, manifest revision 14)

Operator decision (card amendment `954a0252`, item 6): the drift gate's
hardcoded historical prefixes are in scope for this lane; add
`docs/archive/` beside `docs/logs`, `docs/parity`, `docs/roadmaps`; plant a
test proving an active-path reference under `docs/guides/` still fails; audit
any other gate whose only objection is a hardcoded prefix; never edit
archived content to satisfy a gate; do not hold the relocation.

Executed on top of a rebase onto promoted main `954a0252` (11 commits
replayed cleanly):

- **Gate change** — `scripts/check-recipe-only-surface.ts` gains
  `docs/archive/` in `HISTORICAL_PREFIXES` (the only code change; 6 lines).
  `drift:recipes` in `tasks/effigy.tasks.toml` is now a composition that runs
  the gate scan and its planted tests, so the proof rides `docs:check` and
  `ci-web`.
- **Planted test** — `scripts/check-recipe-only-surface.test.ts` (new): runs
  the gate hermeticly in a throwaway mini-repo. Test 1 plants a
  retired-token line under `docs/guides/` plus the identical wording under
  `docs/archive/parity/` — the gate fails listing only the guides file
  (active paths still bite; archived content is never edited for a gate).
  Test 2 proves archive-only content is green. Forbidden literals are
  assembled at runtime so the gate never trips on its own test source.
- **Other-gate audit** — gates with hardcoded doc-prefix exclusions were
  enumerated by grepping every `.ts`/`.rs`/`.toml`/`.json` under
  `scripts/`, `packages/`, `test/`, `tasks/` for `docs/logs`, `docs/parity`,
  `docs/roadmaps`. Only `drift:recipes`
  (`scripts/check-recipe-only-surface.ts`) is a repo-wide content scanner
  whose historical list needed the follow. Inspected and clean:
  `lint-docs.ts` validators scan contracts/guides/catalog surfaces and JSON
  manifests, not `docs/archive/`; `audit-repository-security.ts` is
  path-independent over tracked files (identical content pre/post move);
  `parity-evidence-ledger.ts`/`nucleus-parity-receipts.ts` read fixed paths.
  No other gate required an analogous update.
- **Rebase fallout** — `g16.106` merged on main (PR #211) with a top-level
  handoff; its lane is now closed, so that brief was archived to
  `docs/handoffs/archive/2026-09/` under the retention rule. Counts:
  185 pre-existing files (184 at the original base plus g16.106's brief),
  176 archived, 9 retained top-level briefs, plus this lane's handoff = 10
  top-level, 186 total.
- **Authorization recorded** in the card (item 6) and this log; archived
  parity content untouched; relocation stands.

## Validation

- `effigy docs:snippet-check` — green: 60 snippets from 18 guides compile;
  32 app-context fragments skipped and listed.
- `effigy docs:check` — green (run on the PR head; gate:clean confirms no
  residue).
- `bunx svelte-check --tsconfig ./tsconfig.json --threshold error` in
  `packages/svelte/preview` — 0 errors (validates the component-docs.ts
  rewrite and the new script).
- `git diff --check origin/main...HEAD` — clean.
- Oracle: nothing open archived (9-file keep list above); `docs:check` link
  checks green after all three moves; zero parity-edit instructions remain
  in g16 cards; snippet check green with the planted stale prop failing;
  specs README lists no archived spec as active.

## Files Changed

Directories moved: `docs/handoffs/archive/{2026-08,2026-09}` (175 files),
`docs/archive/parity/` (141 files), `docs/specs/archive/` (56 files +
`index.md`). Edited: `docs/README.md`, `docs/parity/README.md`,
`docs/archive/parity/README.md`, `docs/specs/README.md`,
`docs/specs/001-token-source-and-artifact-contract.md`, seven kept specs'
dependency headers, `docs/roadmaps/g16/{001,010,034,035}-*.md`,
`docs/guides/{svelte-developer-guide,002-action-pattern-recipes,
003-list-and-filter-recipes,014-admin-app-shell-recipes}.md`,
`docs/contracts/components/README.md`,
`packages/svelte/preview/src/component-docs.ts`,
`packages/svelte/preview/scripts/docs-snippet-check.ts` (new),
`tasks/effigy.tasks.toml`, this log, and the worker handoff.
