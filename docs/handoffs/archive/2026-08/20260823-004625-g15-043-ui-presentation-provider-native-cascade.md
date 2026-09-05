---
title: g15.043 UiPresentationProvider native cascade worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Poodle orchestrator
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260823-004625-g15-043-ui-presentation-provider-native-cascade.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g15, rust, presentation, context, gpui]
---

## What This Thread Is Doing

Execute `g15.043` only. Replace the native no-op presentation provider with
one real construction-time cascade across the complete shared Rust component
surface.

This is an operator-approved atomic pre-v1 API migration. Add one explicit
`poodle-render` context, preserve omitted semantic size/density inputs, carry
the context through shared composition, make scoped host children build inside
their context, prove real GPUI output, and add a standing drift guard.

This file and the ready card are the complete worker prompt. Do not depend on
conversation history.

## Why It Matters

Web descendants inherit size and density through framework context and CSS.
Native descendants currently resolve concrete values before a provider sees
their Node tree, so the GPUI provider is only a preview passthrough. That is the
last implementation blocker before the v0.2.0 candidate.

The earlier shortcut—seeding concrete specs in a particular builder order—was
rejected. It can silently erase an explicit `md` or `default` reset. The
accepted design keeps omission in the type system and carries one explicit
borrowed context during construction.

## Current State

- **Repository:** `/Users/tom/Dev/projects/poodle`
- **Planning branch:** `main`
- **Accepted architecture base:**
  `288746f357334b828ff30e949e066f45998f9b5f`
- **Pushed base:** local `HEAD` and `origin/main` matched that SHA before this
  handoff commit; the planning checkout was clean.
- **Posture:** `strict-ready`.
- **Worker branch:** `t3code/g15-043-ui-presentation-provider-native-cascade`.
- **Worker worktree:** use the clean registered non-`main` worktree supplied
  by the launcher. Do not create a second worktree.
- **Ready card:**
  `docs/roadmaps/g15/043-ui-presentation-provider-native-cascade.md`.
- **Architecture authority:**
  `docs/architecture/010-native-presentation-construction-context.md`.
- **Allowed runway:** `g15.043` only.
- **Remaining budget:** one atomic Rust migration, one August execution log,
  one PR, then stop.
- **Dispatch topology:** serial. `g15.050` and `g15.013` remain blocked. No
  overlapping worker lane is authorised.
- **Canonical refs:** `AGENTS.md`, `.agents/skills/effigy/SKILL.md`,
  architecture 001 and 010, working rules 001, the UiPresentationProvider
  contract, GPUI developer guide, `g15.043`, and the release gap register.
- **Runtime restriction:** headless only. Never run a `*-windowed` selector,
  `test:native-visual`, GPUI preview window, Jetstream QA/preview selector,
  workflow, release mutation, tag, or publication command.
- **PR base/head:** `main` <- worker branch.
- **Merge authority:** absent. Push a PR and stop.

## Fixed Design

Implement architecture 010 without a compatibility twin.

1. `poodle_render::RenderContext` carries a borrowed token-only
   `ThemeProvider` plus effective size-scale and density defaults.
2. Every public shared component renderer accepts `&RenderContext`, not bare
   `&dyn ThemeProvider`.
3. Native semantic `ControlSize` and `ControlDensity` component inputs become
   optional. `None` inherits; `Some` always wins.
4. Size-role mapping happens after explicit-or-inherited base-size selection.
5. A provider creates a nested context, invokes an immediate child builder,
   and returns that child unchanged.
6. A composite that creates a presentation scope around host content must
   build that content inside the scope. Do not pass an already-built Node
   across the boundary.
7. `ThemeProvider`, `poodle-node`, GPUI, and Jetstream do not implement or
   carry inheritance state.

Root defaults are `md` / `default`. Nested providers replace both defaults
only for their closure. Explicit `md` / `default` inside an
`xl` / `comfortable` scope must remain explicit.

## Measured Starting Surface

The orchestrator measured:

- 125 component spec files with semantic size or density;
- 107 concrete `ControlSize` and 117 concrete `ControlDensity` surfaces;
- seven files already retaining optional presentation inputs;
- 103 render modules reading those inputs;
- 168 render modules accepting `ThemeProvider` directly;
- 113 render modules constructing descendant specs; and
- 14 web composites creating an internal provider boundary.

Recompute the final counts. Explain every denominator change; do not hard-code
the starting counts into the guard.

## Work In Five Batches

### 1. Context and laws

- Add the borrowed root/scoped context and explicit theme accessor.
- Add base-size, semantic-size, and density resolution helpers.
- Prove root, outer, nested, sibling restoration, explicit `md`, and explicit
  `default` behavior before migrating the roster.

### 2. Spec explicitness

- Migrate semantic `ControlSize` / `ControlDensity` fields to `Option` across
  the native component roster.
- Defaults become `None`; existing builders store `Some`.
- Exclude component-specific domains such as `AvatarSize`, `IconSize`, meter
  dimensions, and numeric sizes.
- Refactor spec helper methods that currently consume concrete fields so they
  receive or use resolved values. Never resolve omission with a root default
  before the context sees it.

### 3. Renderer and composition migration

- Move every public component renderer and current Rust caller to the context
  API in one branch.
- Resolve a component once and propagate effective values into descendant
  specs.
- Pass the same context through ordinary composition.
- Audit architecture 010's fourteen named internal-provider owners. Use a
  bounded immediate child-builder closure where host content crosses a new
  scope; otherwise record exact source evidence that no child crosses it.
- Update GPUI preview facades, specimens, examples, and headless tests.
- Make only compile-required mechanical Jetstream call-site changes. Do not
  create a sibling link, run Jetstream QA, or claim parity.

### 4. Provider and standing guard

- Implement the real shared provider and delete the GPUI preview no-op facade.
- Rebuild its GPUI specimen from actual inherited output: root, outer, nested,
  explicit reset.
- Prove Button, TextInput, a descendant-building composite, and a scoped host
  slot.
- Prove the returned child has no provider wrapper semantics or layout.
- Add one narrow source-audit selector. It must catch planted regressions for a
  concrete semantic field, a public renderer taking bare `ThemeProvider`, and
  a preview passthrough/manual-equivalent provider. Restore every plant.
- Wire the selector into the existing headless native board, not a workflow.

### 5. Docs and evidence

- Update the provider contract and GPUI guide with construction, nesting,
  explicit reset, and migration examples.
- Write one `20260823-g15-043` execution log with final counts, the fourteen-
  component audit, API changes, proof cases, validation, and any compile-only
  Jetstream edits.
- Reconcile the card and release-gap row only. Do not advance the release
  candidate or certification cards.

Report after each meaningful batch. This migration is large enough that silent
multi-hour work is not acceptable, but do not split it into micro-commits.

## Hard Boundaries

Do not add or retain:

- an old renderer signature, alias, adapter, or silent fallback;
- default-value comparison as an explicitness heuristic;
- global, thread-local, task-local, or backend presentation state;
- provider metadata or unresolved presentation values in `poodle-node`;
- GPUI-specific inheritance logic;
- a universal component, scene, slot, or cross-language representation;
- post-built Node scaling/mutation;
- a second native component implementation.

Do not edit Svelte or React component/CSS implementation, public web props,
tokens, themes, unrelated specimens, node painting/layout vocabulary, visual
comparator policy/assets, package versions, workflows, release notes,
Longhorn, tags, publication, or unrelated triage work.

Stop if a contract/web/native size-density surface disagrees and the correct
behavior is not already decided. Stop if an internal provider needs a general
scene abstraction rather than an immediate builder. Stop if the source guard
cannot avoid false claims about component-specific size domains. Stop if
evidence requires a visible window or Jetstream setup. Report exact types and
options instead of weakening the architecture or shipping a partial roster.

## Important Source Context

- `packages/render/src/presentation.rs` already owns the shared semantic size
  and density metric laws. Reuse it; do not create parallel tables.
- `packages/contracts/components/src/ui_presentation_provider.rs` already owns
  the provider's two values and size-role table.
- `packages/gpui/preview/src/providers.rs` and its provider specimen currently
  admit the no-op/manual-equivalent state. They must stop doing so.
- `ButtonSpec` and a small handful of recent specs already demonstrate
  optional presentation inputs. They are examples, not an excuse to migrate
  only that cohort.
- Shared composite renderers frequently construct child specs and some accept
  prebuilt Node slots. Distinguish ordinary composition from a real nested
  presentation boundary.
- The paired web internal-provider list is fixed in architecture 010. Audit
  every row; do not infer completion from a representative sample.
- Jetstream is program-deferred. Compile adaptation is allowed only because a
  shared public Rust signature changed.

## Validation

Use `effigy tasks` to confirm selectors. Run focused tests while migrating,
then one broad headless board after the atomic tree compiles:

- focused `poodle-specs` and `poodle-render` context/provider/composition tests;
- focused GPUI specimen probe and mounted headless regressions;
- the new presentation audit plus its three restored plant proofs;
- `effigy check:gpui`;
- `effigy ci:rust`;
- `effigy ci:native`;
- `effigy docs:check`;
- `git diff --check origin/main...HEAD`.

Do not run large suites after each edit. Do not run any windowed, native-visual,
Jetstream, workflow, or release path.

## Worker Protocol

### Before editing

1. Run only:
   - `git rev-parse --show-toplevel`
   - `git branch --show-current`
   - `git status --porcelain`
   - `git worktree list --porcelain`
2. Require a clean registered non-`main` worktree. Never clean, reset, stash,
   or discard another checkout.
3. Fetch `origin`; require `HEAD == origin/main`; confirm the accepted base is
   an ancestor and this handoff exists in `HEAD`.
4. Read `AGENTS.md`, the Effigy skill, architecture 010, the ready card,
   provider contract, GPUI guide, and the narrow source files above.
5. Run `effigy tasks` and the smallest clean starting tests. Do not begin with
   the full QA board.

### While working

- Keep one compilable architectural direction in the branch.
- Commit meaningful batches, but do not merge partial architecture to `main`.
- Add focused laws before mechanical roster migration.
- Use compiler failures to drive call-site migration; do not hide them behind
  generic adapters.
- Record final inventories and the fourteen-boundary audit as evidence.
- If a stop condition fires, push the evidence and report the blocker.

### Completion

1. Run the validation board and record exact results.
2. Write the execution log and reconcile only authorised status surfaces.
3. Inspect the full diff for forbidden web, node, release, and workflow scope.
4. Run `git diff --check origin/main...HEAD`.
5. Commit, push, and open one PR against `main`.
6. Link architecture 010, `g15.043`, the execution log, final counts, internal-
   provider audit, planted guard proofs, and validation in the PR body.
7. Report the PR URL and stop. Never merge.

## Review And Merge

The orchestrator independently reviews explicitness, every public renderer
signature, context flow, the fourteen internal boundaries, real provider
evidence, the standing guard, API migration, scope, and headless validation.
Only the orchestrator merges after review and any operator-facing specimen
check required by the resulting UI.

After merge, the orchestrator closes `g15.043` and compiles `g15.050`. The
worker must not start release-candidate or certification work.
