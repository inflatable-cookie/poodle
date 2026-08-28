# g16.019 — Select Mounted Overlay Parity

Status: planned — blocked on merged `g16.018`
Opened: 2026-08-28
Depends on: completed and merged
`018-select-semantic-machine-and-interface-convergence.md`
Governing refs: `../../contracts/001-working-rules.md`,
`../../architecture/001-poodle-system-shape.md`,
`../../contracts/components/select.md`, `parity-evidence-ledger.md`

## Goal

Complete Select's production GPUI behavior on the semantic substrate from
`g16.018`, then move exactly Select from missing to mounted: 46 → 47 mounted and
128 → 127 missing.

## Planned Scope

- replace the static native search row with a real host-controlled text-editing
  path using the existing text-input substrate;
- drive open/query/highlight/value results through production pointer and
  keyboard input with host rebuilds;
- keep combobox focus on the trigger/search editor while highlight moves;
- implement Arrow Up/Down, Home/End, Enter, Escape, Tab, clear, freeform commit,
  disabled options, groups, and focus return from the landed transition model;
- repair deferred-overlay pointer targeting at the smallest reusable backend
  seam if the landed evidence still reproduces the Pagination test workaround;
- remove Select-specific test-only focus-ring/id stamps made obsolete by the
  production implementation;
- keep the GPUI specimen curated and interactive; and
- add one readable named mounted regression, then update only Select's ledger
  cell and closeout surfaces.

## Entry Gate

Do not mark this card ready until `g16.018` is merged and the orchestrator has
verified:

- the exact landed state/event/effect/result API;
- Svelte/React callback timing and paired vectors;
- the Rust handler/spec construction surface used by composed Select callers;
- whether editable search can compose existing Node text input without a new
  generic capability; and
- whether deferred-overlay pointer misses still reproduce on the current
  backend after the interface migration.

## Fixed Boundaries

- No new public web props or another Select semantic redesign.
- No closure of composed components that happen to use Select.
- No broad overlay rewrite, visual-comparison or accessibility programme.
- No Jetstream admission, windowed validation, release/version/workflow work,
  downstream changes, or sibling-repository work.

## Acceptance Envelope

- [ ] Real editable query, pointer selection, keyboard navigation/dismissal,
      clear, and freeform commit rebuild host-owned Select state.
- [ ] Disabled control/options are inert; groups and stable instance identity
      remain coherent.
- [ ] Focus/highlight behavior matches the detailed contract within GPUI's
      documented accessibility boundary.
- [ ] One named mounted regression removes the Pagination-era test workaround
      and proves two Select instances do not collide.
- [ ] Ledger changes only Select to 47 mounted / 127 missing; known-delta,
      visual, and broad accessibility totals do not move.
- [ ] Headless native/web/docs/QA gates pass and the execution log states exact
      non-claims.

## Continuation

Blocked. After `g16.018` merges, the orchestrator must recompile this card
against the landed API, add exact writable scope, validation, stop conditions,
and worker handoff, then mark it ready. A worker must not execute this planned
outline directly.

