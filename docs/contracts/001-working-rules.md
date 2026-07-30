# 001 - Working Rules

Status: active
Owner: Poodle core
Depends on: `docs/architecture/product-guardrails.md`

## Contract

- Treat `docs/roadmaps/`, `docs/specs/`, and `docs/logs/` as the execution
  authority chain for active Poodle work.
- Use `docs/specs/` as the strict planning and execution-control layer when the
  roadmap alone is not enough to keep the next owner honest.
- In a strict lane, a bare `continue` should resolve through the previous
  closeout's `Next Task`, which should point at the current ready card or an
  explicit planning gate.
- If there is no ready card, the lane is in planning. Do not improvise from a
  dirty worktree or the most recent chat summary.
- When multiple plausible next seams exist inside `g10`, freeze the active
  posture first, then choose the next owner deliberately.
- Keep currentness surfaces aligned so completed cards do not remain advertised
  as ready.

## Generation Rollover Rule

Treat roadmap generations as substantial sequencing eras, not tiny buckets. In
a long-running repo, expect roughly 20 to 40 roadmap files in one generation
before rollover is even worth discussing.

Treat rollover as full closeout:

- every roadmap in the old generation must be explicitly closed, paused,
  superseded, or moved to backlog
- the roadmap front doors must reflect that closed state before the next
  generation opens
- stale strict-planning artifacts from the closing generation must be archived
  or removed from the active `docs/specs/` tree

If those closeout conditions are not satisfied, repair the current generation
instead of opening a new one.

## Typography Inherit Rule

Use `typography="inherit"` for inline text-like primitives when parent copy
should own the local text scale.

Two modes are allowed:

- text-only inherit: for primitives without shell geometry, inherit font metrics
  directly from the parent
- proportional inherit: for primitives with visible shell geometry, convert the
  component's size preset from token `rem` values into equivalent `em` values
  so text, padding, gaps, and other shell metrics stay proportional

Runtime note:

- CSS runtimes should implement this literally with inherited font metrics and
  `em`-relative shell geometry
- non-CSS runtimes may approximate proportional inherit with equivalent
  ratio-preserving metrics from a 1rem baseline until parent-relative inline
  layout exists; that limitation must stay documented on the runtime side

Do not overload `size` with an `"inherit"` option for this behavior. `size`
continues to mean the component's own semantic size preset.

## Svelte Surface Modernization Rule

Treat the current Svelte component layer as compatibility-first, not as the
target shape for new work.

Rules:

- new or substantially reshaped Svelte components should prefer Svelte 5
  runes-based internals over `export let` plus `$:` compatibility mode
- new public composition surfaces should prefer callback props and snippets over
  introducing new `createEventDispatcher` and legacy slot APIs
- do not add new compatibility alias props like parallel `items` / `options`
  inputs unless there is a specific downstream migration need documented first
- when a legacy component is touched substantially, remove old compatibility
  baggage before adding more surface area if that can be done without breaking
  current consumers

Operational note:

- use `effigy svelte:surface-audit` to keep the legacy surface visible during
  modernization work
- the audit is a report, not a gate; the goal is to stop drift first, then
  reduce the backlog deliberately

## Current Posture

Poodle is strict-paused after completing `g12.016`. No implementation card is
ready. The next owner must be chosen and promoted deliberately.

## Next Task

Use `docs/roadmaps/g12/README.md` as the planning gate.
