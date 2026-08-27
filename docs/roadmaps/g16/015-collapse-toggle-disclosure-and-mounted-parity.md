# g16.015 — CollapseToggle Disclosure And Mounted Parity

Status: complete
Opened: 2026-08-27
Completed: 2026-08-27
Depends on: merged `g16.014` / PR #88 and the resolved selection in
`../../triage/20260827-195632-post-g16-014-native-lane-decision.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/collapse-toggle.md`,
`parity-evidence-ledger.md`

## Goal

- Make native CollapseToggle expose the same label, expanded state, focus,
  disabled behavior, next-state callback, and directional result as Svelte and
  React.
- Prove the repaired standalone control through real headless GPUI pointer and
  keyboard dispatch plus host-owned rebuilds.
- Move exactly CollapseToggle's GPUI mounted-behaviour cell from `missing` to
  `mounted`: 43 → 44 mounted and 131 → 130 missing. Keep known-delta totals at
  115 present / 60 not-applicable.

## Current Evidence

- Svelte and React agree: expanded means `aria-expanded=true` and label
  `Collapse`; collapsed means `aria-expanded=false` and label `Expand`; an
  explicit `ariaLabel` overrides both.
- Both web runtimes report `!collapsed` without owning state. The host supplies
  the next prop value. Disabled controls report nothing.
- `CollapseToggleSpec` already expresses the same collapsed value, direction,
  disabled state, effective label, size/density, icon mapping, and focus-ring
  token methods.
- The shared renderer hardcodes `Toggle section` for the default label, never
  sets `Node.a11y.expanded`, marks disabled controls focusable, and declares no
  enabled tab index or structured focus ring.
- The renderer's callback and icon-direction math already carry the correct
  next state. Preserve them and prove them rather than introducing a machine.
- The GPUI specimen already rebuilds four host-owned direction examples. Its
  curated Examples, Sizes, and Densities surfaces do not need expansion.
- `dock_region_tab_and_collapse_rebuild_the_host_spec_through_mounted_input`
  drives DockRegion's separate internal collapse button. It is not standalone
  CollapseToggle evidence and does not move this ledger cell.

## Fixed Contract

### Semantics and focus

- The node root remains a Button and uses
  `CollapseToggleSpec::effective_aria_label()` for default and explicit labels.
- Project `expanded = !is_collapsed` on every rendered state.
- Enabled controls are ordinary sequential tab stops, declare the contracted
  structured focus ring using the spec's colour/width tokens and `0.0625rem`
  offset, and keep pointer cursor plus hover treatment.
- Disabled controls set the backend disabled channel, have no activation
  handler, no focus handle, no tab stop, and no focus ring. Preserve the
  contract-owned disabled opacity and default cursor.
- Do not add region ownership, `controls`, content ids, or focus restoration.
  CollapseToggle controls an adjacent host-owned region but does not identify
  or render it.

### Transition and direction

- Activation remains stateless: every accepted pointer, Enter, or Space action
  reports exactly `!is_collapsed`; the host rebuilds the spec.
- Preserve `effective_icon_name()` and prove every direction maps to its
  opposite when collapsed.
- Repeated activation without a host rebuild may report the same next value,
  matching both web runtimes. Do not add hidden renderer state or a disclosure
  machine.
- Keep the existing public Svelte, React, Rust spec, renderer function, and GPUI
  compatibility APIs. No breaking migration or alias is needed.

## Execution Plan

- [x] **Batch 1 — focused renderer contract.** Correct default/custom labels,
      expanded state, enabled focus/tab/ring declaration, and disabled focus /
      activation suppression. Add focused spec/renderer tests for all four
      directions, both collapsed states, labels, callback results, and disabled
      behavior.
- [x] **Batch 2 — mounted standalone behavior.** Add one readable named
      headless GPUI regression through the production renderer and node backend.
      Drive pointer, Enter, and Space activation with host rebuilds; prove
      state, label, icon, callback, focus, and disabled inertia.
- [x] **Batch 3 — evidence and closeout.** Preserve the curated GPUI specimen,
      regenerate only CollapseToggle's ledger cell, close the card/decision/log
      and front doors, and run the required headless validation board.

## Specimen And Mounted Proof

- Preserve the existing Directions group and dedicated Sizes/Densities axes.
  Do not add an exhaustive matrix or test copy to the human-facing specimen.
- The mounted regression may assign one stable fixture id solely so the driver
  can target the production node. It must not claim a public identity contract
  or add a new component id API.
- The named mounted regression proves:
  - expanded state announces `Collapse`, exposes expanded true, and paints the
    authored direction;
  - pointer activation reports collapsed true once, the host rebuilds, the
    label becomes `Expand`, expanded becomes false, and the chevron flips;
  - Enter and Space travel through the same production activation path and host
    rebuild as pointer input;
  - an explicit label survives both states;
  - enabled focus is reachable through the real backend and the node declares
    the contracted structured focus ring;
  - disabled controls expose disabled state, cannot receive sequential focus,
    and emit nothing through pointer or keyboard input; and
  - all four direction pairs are covered by focused renderer/spec tests.
- Direct callback invocation, spec inspection alone, the DockRegion regression,
  or fixture-only state mutation does not satisfy mounted behavior proof.

## Explicit Non-Claims

- This card does not change public Svelte or React props, implementations, or
  controlled-state behavior.
- It does not change the Rust spec shape, add a state machine, add region ids or
  `controls`, or change generic node/backend vocabulary.
- It does not rewrite DockRegion or SplitView collapse affordances. Compile
  fallout is allowed only when mechanical and required by the focused renderer
  repair; semantic composite work stops the card.
- It does not repair radius/token visual deltas, add GPUI visual comparison, or
  claim broad native assistive-technology proof.
- It does not change Select, EditableLabel, NumberInput, Rating, Dialog, Menu,
  or other component families.
- It does not admit Jetstream or touch releases, versions, workflows,
  downstream repositories, publication, or sibling repositories.

## Acceptance Criteria

- [x] Native default and explicit accessible labels match the web authority in
      both collapsed states.
- [x] Native expanded state is always the inverse of collapsed state.
- [x] Enabled controls are Button tab stops with a structured focus ring using
      the contract tokens and offset; disabled controls have no focus or ring.
- [x] Pointer, Enter, and Space report only the next collapsed boolean and the
      mounted host rebuilds the spec; no hidden renderer state is introduced.
- [x] Disabled controls expose the disabled channel and emit nothing.
- [x] Every direction paints its authored chevron when expanded and the exact
      opposite when collapsed.
- [x] Focused Svelte and React tests remain green without web implementation
      changes.
- [x] The curated GPUI specimen stays human-centred and keeps its current axes.
- [x] One named mounted regression proves the standalone production path.
- [x] The generated ledger changes only CollapseToggle to 44 mounted / 130
      missing; known-delta totals stay 115 / 60 and visual/accessibility cells
      remain unchanged.
- [x] One August log records the behavior repair, evidence, validation,
      non-claims, and next checkpoint.

## Writable Scope

- `packages/render/src/collapse_toggle.rs` and focused tests
- CollapseToggle-only code under `packages/gpui/preview/src/specimens/` and the
  smallest compatibility change in `packages/gpui/preview/src/node_compat.rs`
  only if mounted/specimen plumbing requires it
- the smallest CollapseToggle mounted regression change in
  `packages/gpui/preview/tests/headless_regressions.rs`
- focused CollapseToggle spec tests in
  `packages/contracts/components/src/collapse_toggle.rs` only when needed to
  lock existing direction/label helpers; do not change the public spec shape
- `scripts/parity-evidence-ledger.ts`, its focused test, and generated
  `parity-evidence-ledger.md` for the one mounted cell
- this card, its source decision, one August log, g16/front-door status, and
  `PAPERCUTS.md` only for new execution friction

Do not edit web component implementations, shared TypeScript machinery,
generic node/backend APIs, other component contracts or semantics, theme/token
definitions, visual fixtures, accessibility reports, package versions,
workflows, releases, downstream repositories, or sibling repositories.

## Validation

Use Effigy selectors discovered in the worker worktree. At minimum:

- focused `poodle-specs` and `poodle-render` CollapseToggle tests;
- focused Svelte and React CollapseToggle tests to prove the web authority stayed
  unchanged;
- the named mounted CollapseToggle regression;
- `effigy regressions:native` and `effigy probe:gpui-specimens`;
- relevant handler/event/role and contract/spec drift selectors;
- `effigy test:parity-evidence-ledger` and
  `effigy check:parity-evidence-ledger`;
- `effigy ci:rust`, `effigy ci:native`, and `effigy ci:web`;
- `effigy docs:check`;
- one final `effigy qa` after the coherent batch; and
- `git diff --check origin/main...HEAD`.

Everything stays headless. Never run `*-windowed`, native visual, Jetstream
preview/QA, release, tag, publication, or workflow-mutation selectors.

## Stop Conditions

- The paired web runtimes or detailed contract disagree on labels, expanded
  state, callback payload, directional mapping, focus, or disabled behavior.
- Correct behavior requires a public API break, compatibility surface, hidden
  renderer state, generic node/backend change, or host-region ownership.
- Mounted proof cannot drive the production renderer/backend through real
  pointer and keyboard dispatch plus host rebuild.
- SplitView or DockRegion requires semantic redesign rather than compiling
  unchanged against the focused repair.
- The ledger generator changes another row/evidence column or validation
  requires windowed execution, workflow/release mutation, Jetstream admission,
  or another component family.

## Continuation

Return the renderer semantic diff, focused test names, mounted regression name,
host-rebuild proof, regenerated ledger totals, validation, and execution log to
the orchestrator. Do not compile or implement `g16.016`. After operator merge,
the orchestrator returns to the measured 44 mounted / 130 missing ledger and
chooses the next bounded parity lane.

