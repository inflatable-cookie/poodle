# g15.006 — React mirror implementation & gallery closure

Status: complete — all three batches landed (PR pending)
Date: 2026-08-16
Card: `docs/roadmaps/g15/006-react-mirror-closure.md`
Governing refs: `docs/roadmaps/g15/release-baseline-roster.md`,
`docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`

## Batches

The card's three named batches were executed in order, each with a narrow test
round at the end. The roster and register React rows were updated once all
three batches were green.

- **Batch A — implementations and exports:** React `AgentPlan` and
  `AgentPlanRecord` as idiomatic thin shells over the shared web substrate,
  matching the Svelte semantics and the contract prop tables. Both are exported
  from `packages/react/components/src/index.ts`; `AgentChatStatus` gained the
  `"questioning"` / `"reviewing-plan"` statuses its implementation already
  handled (see "Bounded Fixes").
- **Batch B — gallery specimens:** `AgentMessageSpecimen.tsx`,
  `AgentPlanSpecimen.tsx`, `AgentPlanRecordSpecimen.tsx`,
  `ChangedFilesSpecimen.tsx`, `ToolCallSpecimen.tsx`,
  `ToolCallGroupSpecimen.tsx` in the React gallery, structure and copy agreeing
  with the Svelte pages. The React registry's svelte-only and embedded-only
  exclusions were retired: every one of the 175 canonical components now has a
  React implementation, export, and gallery specimen.
- **Batch C — residual focused evidence:** `AgentPlan.test.tsx`,
  `IconProvider.test.tsx` (covers both Icon and IconProvider),
  `Tree.test.tsx`, and `SplitView.test.tsx`, each paired with its existing
  Svelte evidence. The remaining 23 focused React gaps (AgentMessage,
  AgentPlanRecord, ChangedFiles, ToolCall, ToolCallGroup plus the final
  workstation/agent tranche) stay owned by `g15.005`.

## Bounded Fixes (contract-first)

The focused evidence surfaced four measured React divergences. Each was fixed
against the Svelte reference and the contract; none changed the shared
`poodle-core` substrate or any Svelte surface.

- **React `AgentMessage` list-item anatomy** — single-paragraph list items
  rendered their inlines inside a redundant `<p class="poodle-agent-message__paragraph">`.
  Svelte flattens those items (marker sits on the text line; the redundant `p`
  pushes content below the marker). The React list branch now renders item
  inlines directly for single-paragraph items. Surfaced by the AgentPlan
  parity case (list markdown), not by AgentMessage's own fixture.
- **React `SplitView` collapse toggles** — with both panes collapsed the React
  toggle conditions hid both expand toggles, stranding the pair with no pill
  way back, against `split-view.md` §Toggle reveal ("a collapse pair is never
  unrecoverable"). Conditions now match Svelte's
  (`!isSecondaryCollapsed || isPrimaryCollapsed` and mirror).
- **React `SplitView` hidden panes** — `primaryHidden` / `secondaryHidden`
  were missing entirely, so a hidden pane was indistinguishable from a
  collapsed one. Added the props and the `isPrimaryGone` / `isSecondaryGone`
  composite (hidden panes take zero space with no toggle and no collapsed data
  attribute), mirroring Svelte.
- **React `AgentChatStatus` type** — narrowed to `"idle" | "busy"` while the
  implementation already handled `"questioning"` and `"reviewing-plan"`; the
  stale type made the plan region unreachable by types. Widened to match
  Svelte's `"idle" | "busy" | "questioning" | "reviewing-plan"` (additive — see
  Public-Package Record).

## Observations (no change made)

- The React `SplitView` also lacks the contract's `divider` prop and the
  `--poodle-split-seam` root anchoring for the toggle pill. Those are
  contract-parity deltas outside this card's evidence scope; recorded in
  `PAPERCUTS.md` for a follow-up card.
- `bunx tsc -p packages/react/preview/tsconfig.json` reports pre-existing
  strict errors in `LicenceActivation`, `ModelConnectionCard`,
  `ModelConnectionPicker`, and `TextInput`. None are touched by this card and
  none are new; they are known board health, matching the roster's
  `effigy doctor` note.
- Retiring the React registry exclusions raised the agent-tools family count
  in the React catalogue from 9 to 11; `catalogue-nav.test.tsx` was updated to
  match.

## Public-Package Record (spec 022)

- **Package:** `@inflatable-cookie/poodle-react` (`packages/react/components`).
  `@inflatable-cookie/poodle-core` and `@inflatable-cookie/poodle-svelte` are
  unchanged.
- **Public entry point:** the root export map (`src/index.ts`, reachable
  through the package `.` entry and the shipped `src`). Adds two named
  exports — `AgentPlan`, `AgentPlanRecord` — and widens the public
  `AgentChatStatus` union to `"idle" | "busy" | "questioning" | "reviewing-plan"`
  (the two statuses the implementation already handled). `AgentPlanStatus`,
  `AgentPlanSettledStatus`, and `AgentPlanDecision` are now re-exported from
  the public types as well.
- **Classification:** additive. New exports; the `AgentChatStatus` change only
  widens a union, so every previously valid value and call remains valid. No
  existing export changed signature or behavior.
- **Downstream re-check:** the roster's downstream-use scan found no canonical
  consumer importing `poodle-react`, so there is no re-check to issue today. A
  downstream adopting the plan region can now import both components and pass
  the widened statuses. The packed-tarball proof was extended to cover the new
  exports: `test:web-pack-install` now mounts `AgentPlan` and `AgentPlanRecord`
  from the packed `poodle-react` tarball, and the React mounted-proof list grew
  11 → 13 (recorded in `release-gap-register.md`'s Package-Install Surface).

## Validation

| Command | Result |
| --- | --- |
| Batch A narrow round (`vitest run` on the four new focused test files) | pass (13 tests) |
| `effigy react:build` | pass |
| `effigy docs:react-specimen-drift` | pass (175 specimens registered) |
| `effigy test:web-pack-install` | pass (5 files, 11 tests; AgentPlan pair mounts from the packed tarball) |
| `effigy test:components` | pass (288 files, 2340 tests) |
| `git diff --check` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Register and Roster Updates

- `release-baseline-roster.md`: React implementation + export 173 → 175,
  React gallery 169 → 175, focused React test 147 present/28 missing →
  152 present/23 missing. The AgentMessage, AgentPlan, AgentPlanRecord,
  ChangedFiles, ToolCall, and ToolCallGroup gallery cells flip to complete;
  the AgentPlan and AgentPlanRecord implementation cells flip to complete; the
  AgentPlan, Icon, IconProvider, Tree, and SplitView focused React test cells
  name their case files. Count-method text updated to the new totals.
- `release-gap-register.md`: the three `g15.006` React mirror rows (missing
  implementation, missing gallery specimen, residual focused gaps) are closed
  and removed; the two Svelte-tranche rows (`g15.002`–`g15.005`,
  `g15.005`) remain. No status line was changed.
- `docs/roadmaps/g15/006-…` card, `README.md`, and `dispatch.md` were not
  modified by the worker.

## Change Footprint

`packages/react/components/src/` (`AgentPlan.tsx`, `AgentPlanRecord.tsx` new;
`AgentMessage.tsx`, `SplitView.tsx`, `types.ts`, `index.ts` modified),
`packages/react/components/test/` (4 new focused test files),
`packages/react/preview/src/gallery/specimens/` (6 new specimens),
`specimen-map.ts` and `registry.ts` (registered the six; retired the two
exclusion sets), `catalogue-nav.test.tsx` (family count 9 → 11),
`test/package-install/` (packed React proof extended with the AgentPlan pair),
the contracts' stale Known Delta rows reconciled (`agent-message.md`,
`agent-plan.md`, `agent-plan-record.md`, `split-view.md`; see review repair
below), the two focused-evidence docs, this log, and `PAPERCUTS.md`. The
`poodle-react` package export map is additive (see Public-Package Record). No
Svelte component, contract prose beyond the delta reconciliation, specimen,
workflow, or downstream repository changed.

## Review Repair (PR #28, blockers)

- **Contracts reconciled to the tree.** `agent-message.md`'s Known Delta no
  longer claims the single-paragraph list-item unwrap is Svelte-only — the web
  targets share it and only the natives keep block-wrapped items.
  `agent-plan.md` (§12, Known Delta, approval notes) and
  `agent-plan-record.md` (Known Delta) no longer defer the React variant;
  `split-view.md` records that the web targets share the zero-footprint default
  and both-collapsed recovery, with React's remaining `divider` opt-in / root
  seam anchoring gap and the natives' still-divergent rows kept as follow-up
  debt. `agent-plan-record.md`'s `onToggle` note now states the framework-neutral
  controlled/uncontrolled rule (host drives state when `expanded` is supplied;
  component-owned otherwise) instead of contradicting the prop table.
- **Public-package record added.** See "Public-Package Record (spec 022)":
  package, entry point, additive classification, and downstream re-check, plus
  the extended packed-tarball proof (`effigy test:web-pack-install`).
- **Gap-register summary narrowed.** The `g15.006` closure sentence no longer
  claims "all React rows evidence-present"; it states the explicit 152/23
  focused-React posture.