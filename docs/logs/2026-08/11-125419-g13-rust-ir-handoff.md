---
title: g13 Rust-authored cross-runtime IR handoff
status: active
owner: Poodle core
updated: 2026-08-11
tags: [coordination, handoff, g13, IR, workers]
---

## What This Thread Was Doing

This thread closed the fragmented g12 planning state, opened g13, and shaped
the proposed answer to Poodle's four-runtime parity problem: author a
constrained component and specimen-scene IR in Rust, generate typed web inputs
and evidence, and consume the same definitions through the native Rust path.

It added the provisional IR spec and a sixteen-card runway. Cards `001`–`008`
form a bounded pilot; `009`–`016` remain locked unless `g13.008` records an
explicit adopt verdict. The thread then introduced worker orchestration,
dispatched evidence work, and kept schema implementation paused while the
authority inventory and contract corpus were produced.

Two workers finished and pushed their commits. Their output has not been
reviewed or merged. A third worker stalled twice with no edits and its card is
dead pending recompilation. This handoff exists because the T3 Code harness
continued to show stale background work after every known session and agent
had been stopped or reaped.

## Why It Matters

Poodle currently maintains component structure, rendering, registries,
specimens, and preview-shell behavior independently across Svelte, React,
GPUI, and Jetstream. The resulting drift is visible in theme controls,
size/density axes, interaction behavior, accessibility, and the maturity of
the native previews.

The g13 direction aims to centralize facts that are genuinely declarative
without weakening runtime-native behavior. Rust was chosen as the authored
source because exhaustive types, explicit ownership, stable serialization,
compile-time validation, and direct native consumption fit Poodle's desired
core. TypeScript is generated for the web. Focus, IME, portals, measurement,
pointer capture, accessibility projection, and framework lifecycle stay in
runtime adapters.

The pilot is deliberately evidence-first. It must prove that one source can
preserve Button, RangeSlider, TextInput, preview-shell, and specimen-axis
behavior in all four runtimes before Poodle commits to a compiler migration.

## Current State

- Main is clean at `7eec3d66` and matches `origin/main` before this handoff
  closeout commit.
- The active roadmap is g13. `g13.001` remains in progress.
- The active spec lane is provisional spec 063. It governs the pilot but does
  not supersede stable architecture or component contracts yet.
- `g13-b001` finished on `thread/g13-001-authority-inventory` at pushed commit
  `251cc858`. It contains an authority inventory, named documentation baseline
  repairs, generated evidence updates, and a batch log. Review and merge are
  pending.
- `g13-b005` finished on `thread/g13-pilot-expressiveness-corpus` at pushed
  commit `2f8dc5db`. It contains a 129-requirement pilot expressiveness corpus,
  a batch log, and papercut evidence. Review and merge are pending.
- `g13-b004` is dead. The `cursor-grok-4.5-medium` provider stalled twice and
  produced no worktree edits or commit. Recompile or split the card and use a
  different model before redispatch.
- `g13-b002` is planned but blocked on reviewed and merged `g13-b001`.
  `g13-b003` follows `g13-b001` and `g13-b002` and owns the crate-placement
  ruling plus executable g13.002 handoff.
- Current main does not pass `effigy docs:lint`: it is missing the Keyboard,
  ModMatrixGrid, and WaveformDisplay contract/preview inventory entries, and
  its shared Svelte export/preview counts are stale. These are the inherited
  failures repaired by worker commit `251cc858`, not a new handoff regression.
- No IR crate, package, schema, compiler, or generated runtime source has been
  created. That pause is intentional under `IR-12`.
- No OMP worker process, persistent exec session, or collaboration monitor is
  known to remain live. OMP sessions `27571` and `22910` were explicitly
  reaped. The stale T3 Code background indicator is a harness/UI state, not
  evidence of ongoing repo work.
- The worker-monitor experiment failed operationally: a continuous Luna agent
  consumed excessive context and still did not surface completion. The worker
  playbook now requires a non-LLM event-driven watcher for GPT-5.6 Sol and
  permits a cheap model only for bounded one-shot anomaly interpretation.

Canonical refs for execution:

- `/Users/tom/Dev/projects/poodle/AGENTS.md`
- `/Users/tom/Dev/projects/poodle/docs/architecture/001-poodle-system-shape.md`
- `/Users/tom/Dev/projects/poodle/docs/architecture/006-headless-core-and-machine-model.md`
- `/Users/tom/Dev/projects/poodle/docs/architecture/007-appearance-recipe-contract.md`
- `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`
- `/Users/tom/Dev/projects/poodle/docs/contracts/components/button.md`
- `/Users/tom/Dev/projects/poodle/docs/contracts/components/range-slider.md`
- `/Users/tom/Dev/projects/poodle/docs/contracts/components/text-input.md`
- `/Users/tom/Dev/projects/poodle/docs/specs/062-headless-core-and-dual-layer-strategy.md`
- `/Users/tom/Dev/projects/poodle/docs/specs/063-rust-authored-component-and-scene-ir.md`
- `/Users/tom/Dev/projects/poodle/docs/roadmaps/g13/README.md`
- `/Users/tom/Dev/projects/poodle/docs/roadmaps/g13/001-authority-inventory-and-fixture-baseline.md`
- `/Users/tom/Dev/projects/poodle/docs/roadmaps/g13/batch-cards/README.md`
- `/Users/tom/Dev/projects/poodle/docs/roadmaps/dispatch.md`
- `/Users/tom/Dev/docs/worker-orchestration-playbook.md`

Worker artifacts to review:

- `/Users/tom/Dev/projects/poodle-wt/g13-001-authority-inventory`
- `/Users/tom/Dev/projects/poodle-wt/g13-001-authority-inventory/docs/roadmaps/g13/authority-inventory.md`
- `/Users/tom/Dev/projects/poodle-wt/g13-001-authority-inventory/docs/logs/2026-08/11-g13-001-authority-inventory.md`
- `/Users/tom/Dev/projects/poodle-wt/g13-pilot-expressiveness-corpus`
- `/Users/tom/Dev/projects/poodle-wt/g13-pilot-expressiveness-corpus/docs/roadmaps/g13/pilot-expressiveness-corpus.md`
- `/Users/tom/Dev/projects/poodle-wt/g13-pilot-expressiveness-corpus/docs/logs/2026-08/11-g13-pilot-expressiveness-corpus.md`

## Boundaries

- Stay inside the g13.001 evidence and placement runway until its three
  batches close.
- Do not create an IR crate, schema, macro, compiler, code generator, or new
  package before the authority inventory is reviewed and `g13-b003` records
  the maintainer's placement ruling.
- Do not treat arbitrary Rust functions as cross-runtime input. The shared
  boundary is typed, serializable data plus a bounded expression vocabulary.
- Do not make `poodle-node` the web authoring model. The IR must preserve DOM,
  CSS/recipe, form, slot, lifecycle, and accessibility semantics above the
  resolved native node layer.
- Do not move focus, IME, portals, measurement, pointer capture, input,
  hit-testing, or accessibility into drawing. Default drawing consumes only a
  serializable VisualState.
- Do not begin broad family migration before the `g13.008` adopt gate.
- Do not trust worker output merely because it is committed. Review source
  claims, scope, generated changes, and validation evidence before merge.
- Preserve unrelated user changes and use isolated worktrees for dispatched
  workers. The orchestrator alone changes roadmap and dispatch state.
- Follow `/Users/tom/Dev/projects/poodle/AGENTS.md`, the repo-local Effigy
  skill, Northstar currentness rules, and
  `/Users/tom/Dev/docs/worker-orchestration-playbook.md`.

## Important Context

- Planning lineage: g12 was consolidated and closed; g13 is a clean generation
  built around the Rust-authored IR pilot. Spec 063 is provisional. Stable
  architecture and component contracts remain authoritative until promotion
  at `g13.008`.
- Spec 062's rejection of Mitosis-style arbitrary framework behavior remains
  valid. Spec 063 changes only the decision for constrained declarative data,
  conformance vectors, and named capabilities.
- The authority worker counted 170 contracts, 108 direct Jetstream
  `RenderComponent<Spec>` implementations, and 157 `js_*` compatibility shims
  plus `nel.rs`. It reports that Jetstream's registry claims generator
  derivation but no generator exists. Verify these claims during review.
- The authority worker repaired three missing component-contract index
  entries, three parity coverage entries, AgentSubagent usage docs, generated
  Svelte/React evidence, and shared audit counts from 186 to 200. Its recorded
  validation passed `effigy svelte:surface-audit`, `effigy docs:lint`,
  `effigy docs:check`, and `git diff --check`.
- The corpus worker produced 129 requirements: `CROSS` 21, `BTN` 29, `RNG`
  29, `TXT` 32, `SHELL` 10, and `NEG` 8. It classified them across shared
  declarative definition, generated target artifact, adapter capability,
  conformance vector, and explicit runtime extension without recommending a
  schema.
- Corpus unknowns requiring maintainer judgment are: whether embedded
  RangeSlider's `aria-orientation` is contractually scoped, and whether Rust
  Button's `Danger`/`Success` enum variants are intentional beyond the
  contract union.
- Corpus evidence gaps include missing RangeSlider/TextInput conformance
  vectors, no executed native assistive-technology traces, missing native
  vertical RangeSlider and GPUI per-thumb focus, absent Jetstream TextInput
  typing/key events, contract-silent Button density values, and Rust Button
  fields not covering truncate/fit/maxWidth.
- The authority branch started from `535fcf22`, before later orchestration
  commits. A raw `git diff main..thread/g13-001-authority-inventory` therefore
  shows apparent deletions of newer roadmap files. Review the scoped commit
  with `git show 251cc858`; preserve both main and worker `PAPERCUTS.md`
  entries when resolving the likely conflict.
- The corpus branch started from `c0e1eb01`. Its scoped commit is
  `2f8dc5db`; its `PAPERCUTS.md` change also needs deliberate conflict review.
- Known pre-existing repo-health failures are `health.task.execute`, the
  god-file scan, and a stale graph warning. `effigy doctor` has also mutated
  unrelated generated Rust files in past runs, recorded in `PAPERCUTS.md`.
- User preference: use the worker orchestration system for heavy execution,
  but keep architecture, review, sequencing, and judgment with the primary
  agent. Parallelize independent ready work. Do not use an always-running LLM
  monitor.

## Suggested Next Move

Review and merge worker output in dependency order. Start with commit
`251cc858`, not by dispatching more implementation work.

1. Fetch and confirm both remote branches and commits exist.
2. Read both worker deliverables and logs in full.
3. Inspect `git show --stat` and `git show --check` for `251cc858`. Validate
   its counts and generated-artifact claims against current main.
4. Merge or cherry-pick `251cc858`, resolving only real conflicts and
   preserving newer main roadmap/dispatch state plus both papercut entries.
5. Run `effigy svelte:surface-audit`, `effigy docs:lint`,
   `effigy docs:check`, and `git diff --check` as the consolidated review
   gate. Record the merge evidence and update `g13-b001` to merged.
6. Review `2f8dc5db` against the three canonical component contracts. Merge it
   only after confirming the corpus contains no schema recommendation and its
   unknowns are reported rather than silently resolved. Run the relevant docs
   gate and mark `g13-b005` merged.
7. Update the g13.001 execution plan, batch-card index, dispatch ledger, and a
   merge/review log in the same closeout commit.
8. Promote `g13-b002` to ready and dispatch it through the worker playbook.
   Its job is fixture and quantitative-baseline freeze, not schema design.
9. Recompile `g13-b004` into smaller research cards and choose a different
   model only when there is useful independent capacity. Do not block b002 on
   the dead prior-art attempt unless the recompiled dependency says so.

Do not begin g13.002 schema implementation in the re-entry turn.

## Completion Protocol

1. Verify worker ancestry after merge with
   `git merge-base --is-ancestor 251cc858 main` and
   `git merge-base --is-ancestor 2f8dc5db main`.
2. Set `g13-b001` and `g13-b005` to merged in their cards, the batch index, and
   the dispatch ledger only after review and validation pass.
3. Update `g13.001` checkboxes and currentness text to match the actual merged
   state. Keep `g13.001` in progress until b002 and b003 close.
4. Add a dated review/merge log with commands, exit states, resolved
   conflicts, accepted findings, and rejected or deferred worker claims.
5. Remove the two worker worktrees only after their commits are reachable from
   main and no uncommitted files remain. Branch deletion is optional and must
   not precede the ancestry check.
6. Preserve the corpus unknowns and evidence gaps as explicit inputs to b002
   and b003. Do not settle them implicitly in an IR representation.
7. Keep the lane pause explicit: no IR implementation package before the
   placement ruling; no rollout before the `g13.008` adopt verdict.
8. Leave one next task: execute `g13-b002` after the reviewed b001 merge.
