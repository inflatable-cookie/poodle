# g14 Baseline Manifest

Status: frozen numbers + measured inventories
Milestone: `g14.002`
Owner: Poodle core (measurement by batch `003`)
Branch: `thread/g14-003-frozen-baseline-and-inventories`
SHA: `847a8652` (main at measurement)
Measured: 2026-08-13
Governing refs: `docs/roadmaps/g14/002-frozen-baseline-and-inventories.md`,
`docs/roadmaps/g13/native-registration-gap.md`,
`docs/roadmaps/g13/batch-cards/047-machine-shape-consolidation.md`

Numbers, not claims. Every count names the command that produced it.
Nothing here was fixed. `g13-b053` unwind is already on this SHA; machine
sources were not in that diff.

## Hole class the pinning stack must catch

**Behaviour divergence between the two machine implementations on events
the shared vector never fires.**

`docs:machine-shape-drift` treats a machine as pinned when both harnesses
execute *a* vector (21/21 today). It does not compare what the two
implementations *do*. Re-counted from `packages/contracts/headless/vectors/machines.json`
on this SHA:

- `slider` — 3 cases; zero mentions of thumb, pair, crossing, or bipolar.
  The two-thumb surface lives in both `packages/core/src/slider.ts` and
  `packages/contracts/headless/src/slider.rs` with no shared case covering
  it. Presence of a `slider` key is not coverage (b045/b047 R4a, still true
  after 053).
- `menu` — 3 cases. `ESCAPE` / `CLOSE` / `OUTSIDE_INTERACT` occur in
  *other* machine blocks in the same file, not in `menu`. Both runtimes
  implement those events independently.

No existing gate compares TS vs Rust transition output. That is `g14.005`.
Docs-silent implementation drift (Svelte vs React handler composition with
every prop gate green) is the same class on the web pair — `g14.005` /
specimen evidence (`g14.009`), not the 13 surface gates.

## 1. Native registration gap

Re-ran the refresh script from `native-registration-gap.md` (verbatim) on
2026-08-13 against `847a8652`.

Command:

```sh
bun - <<'JS'
import { readdirSync, readFileSync } from "node:fs";
const web = readdirSync("packages/svelte/components/src")
  .filter((f) => f.endsWith(".svelte")).map((f) => f.slice(0, -7));
const reg = (p) => new Set([...readFileSync(p, "utf8")
  .matchAll(/display_name:\s*"([^"]+)"/g)].map((m) => m[1]));
for (const [name, path] of [
  ["gpui", "packages/gpui/preview/src/component_registry.rs"],
  ["jetstream", "packages/jetstream/preview/src/component_registry.rs"],
]) {
  const have = reg(path);
  const missing = web.filter((c) => !have.has(c)).sort();
  console.log(`${name}: ${missing.length} missing — ${missing.join(", ")}`);
}
JS
```

| | 2026-08-12 (gap doc) | 2026-08-13 (this SHA) |
|---|---|---|
| Svelte `*.svelte` files | (not restated) | **168** |
| GPUI `display_name` entries | | **155** |
| Jetstream `display_name` entries | | **151** |
| GPUI missing | 14 | **16** |
| Jetstream missing | 15 | **17** |
| Union | 16 | **18** |

Delta vs the 2026-08-12 table: `UpdateCenter` and `UpdateStatus` joined the
union (both natives). Everything else in that table is still missing.
Owner of closure: `g14.008`.

| Component | GPUI | Jetstream |
|-----------|------|-----------|
| `AgentMessage` | — | — |
| `AgentPlan` | — | — |
| `AgentPlanRecord` | — | — |
| `AgentQuestionRecord` | — | — |
| `AgentSubagent` | — | — |
| `ChangedFiles` | — | — |
| `HistoryCenter` | — | — |
| `MenuSurface` | — | — |
| `Radio` | — | yes |
| `RemediationBanner` | — | — |
| `SettingsShell` | — | — |
| `StateTile` | — | — |
| `ThemeSelect` | yes | — |
| `ToolCall` | — | — |
| `ToolCallGroup` | — | — |
| `UpdateCenter` | — | — |
| `UpdateStatus` | — | — |
| `ValidationSummary` | yes | — |

## 2. Machine inventory (re-derived, not copied)

Commands (2026-08-13, this SHA):

- TS modules: `ls packages/core/src/*.ts` excluding `index.ts` / `machine.ts` → **41**
- Rust modules: `ls packages/contracts/headless/src/*.rs` excluding `lib.rs` → **23**
- Canonical TS: module text contains `TransitionResult` → **7**: `edit`, `history-center`, `hover`, `menu`, `modal`, `popover`, `tabs`
- Canonical Rust: `pub fn *transition` plus State+Context+Event+Effect → **6**: `audio`, `hover`, `menu`, `modal`, `popover`, `slider`
- Canonical in both: **4** — `hover`, `menu`, `modal`, `popover`
- Both-runtime pairs (name-mapped): **21**
- `effigy docs:machine-shape-drift` → exit 0: `checked 21 pinned machines, 41 TS modules, 23 Rust modules`; one baseline `rs:text_input`

TS-only (20): `agent-chat-input`, `code-input`, `dock-external-drag`, `edit`, `embed-input`, `file-upload`, `filter-builder`, `history-center`, `input`, `markdown-blocks`, `model-picker`, `position`, `rating`, `relation-picker`, `resize`, `select`, `tabs-reorder`, `toast`, `token`, `update`

Rust-only (2): `audio`, `text_input` (baselined: editing model, not a behaviour machine)

### Pinning

Both harnesses run all 21. Floor of unpinned duplicated machines: **0**.
(`g13-b047` pinned the last seven; still true.)

`machines.json` case counts (this SHA):

| Machine | Cases |
|---|---|
| checkbox | 5 |
| popover | 4 |
| modal | 5 |
| hover | 6 |
| singleSelect | 4 |
| slider | 3 |
| menu | 3 |
| disclosure | 2 |
| toggleGroup | 4 |
| tabs | 5 |
| switch | 5 |

`domain.json` case counts: date 34, color 50, pagination 13, tree 24, duration 15, nav 9.

### Depth (whole-surface vs thin)

Re-read the vectors; do not trust the 047 log as authority.

- **Thin:** `slider` (3 cases, no two-thumb vocabulary) — still the clearest
  "listed in `machines.json` ≠ covered" case.
- **Happy-path / partial:** `menu` (3; no menu-block `ESCAPE`/`CLOSE`/`OUTSIDE_INTERACT`),
  `disclosure` (2). Popover/modal *do* fire `ESCAPE`/`CLOSE` in their own blocks.
- **Presence is not coverage** remains the measured fact the pinning stack
  has to close (`g14.006`).

### What 053 changed here

Nothing in the machine modules, vectors, or harnesses. 053 rehomed capability
*declarations* to `packages/contracts/headless/capabilities/capabilities.json`
(36 gated rows). That is the capability inventory's home, not a machine-shape
change. RangeSlider native thumbs are hard-coded two again (IR wiring
removed); observable pair semantics unchanged; slider vector still thin.

## 3. Drift-gate inventory

The thirteen named in `g13-b052` / `g13.019`, plus `docs:capability-drift`
(landed `g13-b051`, after that list). Selector names from
`tasks/effigy.tasks.toml` on this SHA.

`docs:contract-role-drift` is **not a defined selector**. The gate is
`drift:roles` (`packages/svelte/preview/scripts/contract-role-drift.ts`).
Finding, not a rename.

| # | Selector | Script | Covers | Cannot see |
|---|---|---|---|---|
| 1 | `docs:contract-drift` | `contract-prop-drift.ts` | Contract §3 props ↔ Svelte `Props` (both directions) | Callbacks (`on*`), snippets, React, Rust, behaviour |
| 2 | `docs:value-domain-drift` | `contract-value-domain-drift.ts` | Enumerated prop value sets: contract ↔ TS ↔ Rust | Report-only (exits 0 unless `VALUE_DOMAIN_ENFORCE=1`); not in `docs:check` |
| 3 | `docs:callback-drift` | `contract-callback-drift.ts` | Function-typed `on*` ↔ contract Events/Callbacks | Non-callback `on*` (e.g. `onColor`); React |
| 4 | `docs:spec-drift` | `contract-spec-drift.ts` | Contract props ↔ `poodle-specs` fields | Events (specs are data); web implementation |
| 5 | `drift:roles` | `contract-role-drift.ts` | Contract ARIA roles ↔ Jetstream a11y census (set membership) | Placement; GPUI; needs sibling Jetstream + `cargo run --bin a11y` |
| 6 | `drift:events` | `inert-component-drift.ts` | Contract that declares events must accept ≥1 handler in `poodle-render` | Name-by-name event↔handler mapping |
| 7 | `drift:handlers` | `dead-handler-drift.ts` | Declared `*Handlers` field is read somewhere | Whether the read is the right place |
| 8 | `drift:recipes` | `check-recipe-only-surface.ts` | Retired Treatment tokens/classes stay gone | Current recipe-hook correctness |
| 9 | `drift:adapter-manifests` | `adapter-manifest-drift.ts` | Direct-adapter manifests ↔ `RenderComponent` impls | `poodle-render` registration; preview registries |
| 10 | `docs:machine-shape-drift` | `machine-shape-drift.ts` | Both-runtime machines pinned; transition modules follow g11.002 | Vector depth; TS↔Rust behavioural equality |
| 11 | `docs:focus-ring-drift` | `focus-ring-drift.ts` | Radius / absent treatment / stacked UA outline on owned focusables | Runtime-drawn rings not in the stylesheet |
| 12 | `docs:container-query-drift` | `container-query-drift.ts` | Self-referential `@container` rules | Queries that fire against the wrong ancestor but are not self-targeted |
| 13 | `docs:react-specimen-drift` | `react-specimen-drift.ts` | Claimed standalone specimens exist in React `specimen-map.ts` | Specimen *content* parity; natives; svelte-only / embedded-only exclusions |
| 14 | `docs:capability-drift` | `capability-drift.ts` | 36 declared provision rows ↔ runtime traces (both directions) | Capabilities never declared; `PROBES` vocabulary completeness |

### Known-red (before-state for `g14-b002`)

Ran `bun packages/svelte/preview/scripts/contract-role-drift.ts` on
2026-08-13 in the **main checkout** (same SHA). Exit **1**.

```
checked 132 contract role requirements across 148 specimens
1 contract role(s) never projected, across 1 components:
  range-slider                 slider
```

Worktree run of the same script exited 1 on a Cargo lockfile collision
(sibling `poodle` path-dep; PAPERCUTS) before producing a census. Native
gates are specified to run in the main checkout.

## 4. Parity evidence state

| Evidence | Count | Command / source | 2026-08-13 |
|---|---|---|---|
| Visual axis-tier slugs | 15 | `test/visual/config.ts` `AXIS_TIER_SLUGS` | button, icon-button, text-input, select, checkbox, switch, tabs, pill, list-card, data-table, menu, dialog, toolbar, card, slider |
| Visual SKIPPED (non-deterministic) | 4 | same file `SKIPPED` | page-loading, video-player, audio-player, media-thumbnail |
| Axe `COMPONENT_PROPS` keys | 88 | `test/fixtures/component-props.ts` | web a11y sweep coverage, not four-runtime |
| `docs:react-specimen-drift` | 161 registered | `bun packages/svelte/preview/scripts/react-specimen-drift.ts` | exit 0 on this SHA (ci:web) |
| `docs:capability-drift` | 36 rows | same | exit 0 |
| Native visual baselines | local-only / stale | PAPERCUTS 2026-08-13 | GPUI classified stale; Jetstream snap overwrites in place — not re-measured as pixels |

Components with a Svelte specimen file but **no** React standalone specimen
(declared, not silent — `packages/react/preview/src/gallery/registry.ts`):

- svelte-only catalogue: `AgentPlan`, `AgentPlanRecord`
- embedded-only (hasSpecimen forced false on React): `AgentMessage`, `ChangedFiles`, `ToolCall`, `ToolCallGroup`

Natives: presence in the preview is §1 (registry), not specimen-file name.

## 5. Specimen inventory (before-state for `g14.003`)

Hand-written surfaces. `wc -l` over the glob, 2026-08-13.

| Runtime | Glob | Files | LOC |
|---|---|---|---|
| Svelte | `packages/svelte/preview/src/specimens/*Specimen.svelte` | 165 | 15,944 |
| React | `packages/react/preview/src/gallery/specimens/*Specimen.tsx` | 160 | 14,425 |
| GPUI | `packages/gpui/preview/src/specimens/*.rs` excl. `mod.rs` | 145 | 32,113 |
| Jetstream | `packages/jetstream/preview/src/specimens/*.rs` excl. `mod.rs` | 150 | 23,913 |

Classification heuristic (Svelte/React): `interactive` if `$state` / `useState` /
`bind:value` / `clickLog`; `static` otherwise; `mixed` if both that and a
sizes/section matrix. Re-run by reading those files.

| Runtime | static | interactive | mixed |
|---|---|---|---|
| Svelte | 76 | 12 | 77 |
| React | 74 | 20 | 66 |
| GPUI | 136 | 2 | 7 |
| Jetstream | 150 | 0 | 0 |

Jetstream specimens are display-only under this heuristic (no `click_count` /
`cx.listener` hits). That is a classification of the files, not a claim that
the runtime cannot interact.

Svelte interactive-only (no sizes matrix): AlertDialog, BlockEditor,
ConfirmAction, Dialog, Drawer, ErrorBoundary, FormDialog, ListContainer,
LogList, NavCard, PageLoading, SettingsShell.

**HistoryCentre-class harnesses** (host state, not a prop matrix) — named for
`g14.003`: `HistoryCenterSpecimen.svelte` (362, mixed),
`SettingsShellSpecimen.svelte` (177, interactive),
`AgentChatInputSpecimen.svelte` (237, mixed), `BlockEditorSpecimen.svelte`
(183, interactive), `FilterBuilderSpecimen.svelte` (149, mixed),
`MarkdownEditorSpecimen.svelte` (82, mixed).

Native specimen **filenames** are not a 1:1 slug join (GPUI uses
`card_specimen.rs`, `bx.rs`, `dock_split.rs`, `action_discovery.rs`, …). Do
not treat a failed name join as a missing specimen. Registry `display_name`
(§1) is the presence authority; filename drift is a g14.003 fixture-identity
problem.

## 6. Suggested owners (findings, not patches)

| Finding | Owner |
|---|---|
| `range-slider` never projects `slider` | `g14-b002` (already dispatched) |
| Union 16→18 (`UpdateCenter`, `UpdateStatus`) | `g14.008` |
| Slider/menu vector thinness | `g14.006` |
| TS↔Rust behaviour on unfired events | `g14.005` |
| Four specimen copies + filename aliases | `g14.003` |
| `docs:contract-role-drift` selector missing | orchestrator (card wording); gate is `drift:roles` |
