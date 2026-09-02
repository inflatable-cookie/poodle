# Roadmaps

Status: reference
Updated: 2026-09-01

Roadmaps record Poodle's executable milestone work. `g15` completed the full
v0.2.x release programme: the roster and parity baseline, failed v0.2.0
publication, v0.2.1 recovery, crates.io-GPUI correction, v0.2.2 publication,
and adoption by all 16 authoritative consumers. `g16` uses a current
active-cohort parity evidence ledger to select bounded semantic and mounted
behavior repairs without inventing another conformance authority.

## Current State

- [Generation index](generation-index.md) is the canonical status summary.
- `g16.036` is complete in PR #127. The 2026-09-01 triage decisions are
  consolidated in the [canonical continuation map](g16/component-continuation-runway.md):
  `g16.045`–`g16.049` are active; `g16.053` is complete in PR #150;
  `g16.050`, `g16.051`,
  and `g16.052` keep explicit serial, ownership, or external gates. `g16.054`
  stays blocked on an accepted, separately promoted compiled-JS/declarations
  prerequisite whose mechanics remain unplanned; its security-audit dependency
  is complete.
  Citations, nested menus, the lab-backed visual tranche, public IconMorph,
  release and adoption mutations, GPUI accessibility, Jetstream, and the
  separate holistic promotion batch remain gated or held.
  Independent `g16.055` is complete in PR #151 and remains outside the
  post-triage continuation map.
- `g16.001` is complete and operator-reviewed in PR #75. It repairs stale
  parity reporting and produces one component-level evidence ledger. `g16.002`
  closed — partial outcome: mounted GPUI behaviour for Checkbox, Switch, and
  SegmentedControl. PR #77 closed `g16.003` RadioGroup native identity and
  mounted evidence. `g16.004` closed ToggleGroup resulting-selection, single-mode
  roving focus, and instance-scoped native identity. `g16.005` closed Slider
  axis, keyboard, callback, and mounted parity in PR #79; ledger 34 → 35
  mounted, 140 → 139 missing. `g16.006` closed Tabs drag, keyboard, and mounted
  parity in PR #80; ledger 35 → 36 mounted, 139 → 138 missing. PR #81 closed
  `g16.007` core TextInput controlled editing and one honest mounted claim;
  ledger 36 → 37 mounted, 138 → 137 missing. `g16.008` then repaired generic
  native text routing — Enter is submission, Tab is real focus traversal, and
  transient text state follows the node that paints the value — deliberately
  moving no evidence cell. `g16.009` closed DurationInput's single-source Rust
  value and one named mounted GPUI behaviour cell (37 → 38 mounted, 137 → 136
  missing). `g16.010` closed Breadcrumbs' reversed Rust callback routing and
  one named mounted GPUI behaviour cell (38 → 39 mounted, 136 → 135 missing)
  plus the Breadcrumbs known-delta cell (`not-applicable` → `present`;
  114 → 115 present, 61 → 60 not-applicable). `g16.011` closed IconButton
  command, toggle, tooltip projection, and mounted GPUI evidence (39 → 40
  mounted, 135 → 134 missing). Known-delta totals stay 115 / 60. PR #86
  closed `g16.012` Collapsible disclosure and mounted parity, moving the ledger
  to 41 mounted / 133 missing. PR #87 closed the operator-approved `g16.013`
  clean migration of TriStateSwitch from legacy `CheckState` to
  `TriStateValue`, repaired native radio behavior and identity, and moved the
  ledger to 42 mounted / 132 missing. PR #88 closed `g16.014` Accordion result
  selection, disclosure semantics, identity, and mounted parity, moving the
  ledger to 43 mounted / 131 missing. PR #90 closed `g16.015` CollapseToggle
  native label, expanded state, focus, disabled behavior, and standalone
  mounted proof; ledger 44 mounted / 130 missing. PR #91 closed `g16.016`
  Pagination loading suppression and mounted navigation/limit proof; ledger
  45 mounted / 129 missing. `g16.017` closed Rating's approved nullable /
  fractional Rust migration and one mounted GPUI cell; ledger 46 mounted /
  128 missing. `g16.018` converged Select's semantic machine and interfaces
  without moving the ledger. `g16.019` closed native Select search, overlay
  pointer targeting, and one mounted GPUI cell; ledger 47 mounted /
  127 missing. Known-delta totals stay 115 / 60. No broader
  conformance programme is implied. PR #95 closed `g16.020`: its 175-row
  continuation register now separates 100 closed, 69 evidence-only, 1
  decision-blocked, and 5 programme-owned components after promoted-lane
  reconciliation. Architecture 011/spec 069 are compiled as
  `g16.021`–`g16.028`; the paired semantic kernel in `021` landed with its
  shared `dragDrop` vector corpus in PR #96.
  TimeInput's native editing decision closed and merged as `g16.029` (ledger
  48 mounted / 126 missing). NumberInput's committed-number/raw-draft and
  clean callback migration merged as `g16.030` in PR #98 (ledger 49 mounted /
  125 missing; known-delta axis 116 present / 59 not-applicable).
  A bounded Fader/Knob/XYPad audit then found real paired-machine, web gesture
  lifetime, native mounting, and accessibility-projection defects. `g16.031`
  closed paired semantics and the Svelte/React gesture and entry lifecycles
  behind one hand-authored `audioControls` vector corpus, moving no ledger
  cell; `g16.032` mounts the three controls through one bounded
  continuous-value Node event (ledger 52 mounted / 122 missing), merged in PR
  #100. This work remains separate from payload drag-and-drop. `g16.022`
  landed the web custom-surface substrate over that kernel in PR #101;
  `g16.023` merged in PR #104 with ordered logical keyboard targets preserving
  windowed reorder. `g16.024` merged in PR #107 with Tree nested intent,
  demand-driven auto-scroll, live drop revalidation, and semantic focus
  ownership on the shared web substrate. `g16.025` projected that kernel through
  renderer-neutral `poodle-node` registrations, shared `poodle-render`
  builders, and a public GPUI `DragDropController`, replacing the
  backend-global payload session so two providers own two sessions; its
  crates.io GPUI 0.2.2 capability matrix is immutable — mouse, keyboard, and
  in-window capture certified; pen, touch, and device-originated cancel remain
  unsupported debt no mouse fixture may flip. It merged in PR #108 after four
  review rounds; no ledger cell moved. PR #113 then merged `g16.026`: Tabs now
  moves with its DockRegion host-bridge consumer through split source/window
  authority and the window-owned GPUI pump. PR #115 then merged `g16.027` after
  three Northstar repair rounds: inbound files and native file drag-out, paired
  in both languages and wired through the web and GPUI controllers, with opaque
  receipts, exact per-installation inbound ownership, validation before
  eligibility, host-owned retention, and no committed export terminal because
  a native drag ending never proves a destination took the file. `g16.028`
  then closed the programme: the last three web HTML-drag owners migrated,
  EditableList/OrderBy/BlockEditor gained their native reorder result paths,
  four GPUI mounted cells moved (52 → 56 mounted, 122 → 118 missing), and the
  absence claim became executable as `effigy drift:drag-inventory`.
  `g16.033` then implemented HistoryCenter's Poodle-owned rejection surface
  and the packed v3 `HistoryEntry` proof: five distinct refusal meanings across
  TypeScript, Rust, both web shells and mounted GPUI, and an installed-tarball
  typecheck on both public Svelte import paths. It moves no ledger cell and
  claims no publication. Keyboard vertical geometry remains design-deferred;
  package publication and Loophole adoption remain separate authorized work.
  Longhorn's `AlreadyAtTarget` wire code is complete in its owning repository.
  The later Tabs/DockRegion drag migrations now have an
  approved clean public break: old DOM-shaped helpers disappear only after
  their mounted replacements pass, with no compatibility layer. Other
  component-continuation decisions remain separate; EditableLabel's accepted
  editing contract is now ready as `g16.045`.
- `g14` tested executable component conformance across Svelte, React, and
  GPUI. `g14.008` rejected the mechanism after its cost and coverage audit;
  `g14.021` preserved the useful fixes and removed the failed authority;
  `g14.022` completed the closeout. The generation is complete.
- `g15` was the release-first runway. The 175-component Svelte and React
  implementation/evidence rosters, measured native declaration/specimen
  baseline, specimen curation and review, native specimen probe, packed roster,
  the first primitive fixture inventory, and truthful release automation are
  complete. PR #68 closed the exact Button comparison; PR #69 then closed its
  measured native focus-ring defect and Stepper keyboard-entry gap. PR #66
  closed `g15.049`; PR #67 closed the GPUI/Zed dependency-licence policy gap
  without admitting GPL code. Those lanes and the `0.2.0` candidate completed;
  its workflow then failed before publication. Card `054` produced green
  replacement candidate `3d914261`; completed gate `013` tagged and published
  it as `v0.2.1` in run `32658293188`. The broken Git tag was later retracted
  after v0.2.2 replaced its fork-sourced GPUI graph; npm retains 0.2.1 for
  install stability while `latest` is 0.2.2. React remains source-only. The
  corrected v0.2.2 candidate then restored crates.io GPUI and all 16
  authoritative consumers moved to the public boundary. The operator removed
  Loophole Legacy, so its cancelled card is historical evidence rather than a
  release obligation. The generation is complete through `g15.079`.
  Jetstream backend admission remains deferred.
- `g13` is complete. Its Rust-authored component IR pilot recorded **revise**,
  then retired and unwound component generation. It remains evidence for g14.
- The first g14 machine-pinning/scene runway was reset after five merged
  batches. Its history is archived in
  [the false-start record](archive/2026-08-14-g14-machine-pinning-false-start.md).
- `g09`–`g12` are complete. Earlier generations remain historical evidence.
- Release automation is tracked separately from roadmap implementation work.

## Generations

- `g01` — foundations, tokens, contracts, primitives, and first Underlay bridge
- `g02` — advanced composites, documentation depth, cleanup, release baseline
- `g03` — hardening, migration, parity automation, downstream adoption
- `g04` — component parity, specialist families, media, editing surfaces
- `g05` — GPUI foundation and cross-runtime parity baseline
- `g06` — shared renderer contracts, typed tokens, layout, events
- `g07` — GPUI rendering build-out and adapter expansion
- `g08` — GPUI production quality and compliance
- `g09` — native package consolidation and semantic sizing/density
- `g10` — Jetstream feasibility and GPUI production hardening
- `g11` — Svelte modernization and shared web machinery
- `g12` — React parity, verification depth, native hardening, audio family
- `g13` — retired Rust-authored component/scene IR pilot
- `g14` — rejected executable-conformance pilot; generation complete
- `g15` — completed v0.2.x release and adoption programme; corrected v0.2.2
  published from candidate `d5607def`, with all 16 authoritative consumers
  adopted and React retained as source-only
- `g16` — active evidence-led continuation generation. `001`–`044` are closed
  or research-complete, including Tree authority in PR #127. The canonical
  post-triage runway is compiled as `045`–`054`; six cards are ready and four
  preserve serial, ownership, or external gates. Dependable drag-and-drop remains a
  closed programme governed by architecture 011/spec 069.

## Rules

- Active milestone files live in generation folders such as `g16/`.
- File names use `NNN-slug.md`; roadmap IDs use forms such as `g16.001`.
- A roadmap file is a complete worker handoff. Do not add a batch-card
  layer.
- Backlog items belong in `backlog/`.
- Architecture belongs in `../architecture/`, not here.
- Generation rollover is manual. Close, pause, supersede, or rehome every live
  roadmap before opening the next generation.
- Purge stale strict-planning artifacts from `../specs/` during rollover.

## Reading Roadmaps

Start with the [generation index](generation-index.md), then open the active
generation README and relevant milestone. Prefer current architecture,
contracts, specs, and implementation when historical cards describe an older
package shape.
