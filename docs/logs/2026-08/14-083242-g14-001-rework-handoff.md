# g14.001 Rework Handoff

## What This Thread Was Doing

Implementing `g14.001` as the first executable proof of the redesigned
cross-runtime conformance system. The delivery lives on
`t3code/g14-conformance-kernel-button-proof` in PR #10.

The first pass established useful directions around shared Button cases,
specimen projection, a generated Rust declaration, and planted-failure
legibility. Its foundations do not yet satisfy the revised card.

## Why It Matters

`g14.001` must prove that one authored component contract can bind interface
shape, specimen structure, semantic output, interaction, and token use across
the active runtimes. If Button needs handwritten duplicate types, runtime-
specific case vocabularies, or simulated native behavior, the mechanism will
not scale to the remaining component library.

This is the gate for `g14.002`. Do not treat the current implementation as
close enough or build later cards on top of it.

## Current State

The orchestrator review is recorded in:

- `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/14-g14-001-delivery-review.md`

The authoritative revised card is:

- `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`

The current PR is review-blocked. It has seven foundational problems:

1. The TypeScript interface helper widens literals, while portable Button
   props and events are still handwritten duplicates rather than projections.
2. Case regions and fixtures use open string maps. Parts, states, events,
   axes, and cases are not closed over or validated against the contract.
3. The shared native observer contains Button-specific parts, icon names, and
   tree logic.
4. Native token-role assertions are always vacuous, and GPUI focus is not
   observed. Web evidence can therefore conceal missing native evidence.
5. The GPUI runner discards the rendered element and invokes the source node's
   activation callback directly, bypassing the GPUI listener/input path.
6. Jetstream is required and reported passing despite its deferred status in
   the active Svelte, React, and GPUI cohort.
7. The mechanism adds about 5,145 lines while replacing about 844, so the cost
   trigger in spec 066 requires simplification and an explicit reassessment.

The previous batch log is also stale. It must describe the final mechanism and
its real validation, not preserve claims from the rejected pass.

## Boundaries

- Rework PR #10 in place. Remove or replace failed mechanism; do not stack
  compatibility patches around it.
- Keep Svelte, React, and GPUI as the required cohort.
- Keep Jetstream deferred. It must not be required for setup, validation, or
  completion, and must not be reported as passing this card. Neutral opt-in
  code is acceptable only if it adds no coupling or maintenance burden.
- Do not start or open `g14.002`.
- Do not change roadmap or dispatch status. The orchestrator owns acceptance.
- Do not make Poodle depend on Longhorn or mix the licence-component work into
  this branch.
- Preserve unrelated worktree changes.

Read these authorities in full before editing:

- `/Users/tom/Dev/projects/poodle/AGENTS.md`
- `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`
- `/Users/tom/Dev/projects/poodle/docs/architecture/009-cross-runtime-component-conformance.md`
- `/Users/tom/Dev/projects/poodle/docs/specs/066-executable-component-conformance.md`
- `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
- `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/14-g14-001-delivery-review.md`

The absolute paths point at the orchestrator checkout and are authoritative if
the worktree's copies are stale. Do not overwrite those files from the 001
worktree.

## Important Context

The target is not identical rendering technology. It is one closed authored
contract whose projections make drift hard to express and whose observations
prove the real runtime path.

Useful work from the first pass may be retained where it survives that test:

- shared portable cases and specimen projection;
- generated Rust declaration replacing the old handwritten `ButtonSpec`;
- readable planted failures that identify runtime, case, and violated rule.

Inference is not evidence. A runner may not call source callbacks to claim a
renderer's interaction works. A passing web assertion may not compensate for
an unobserved native assertion. Unsupported evidence must fail or be excluded
by an explicit capability contract; it must not silently become vacuous.

Use Effigy as the command surface. Run `effigy graph` where implementation
ownership or changed-file impact is unclear, `effigy test --plan` before the
final test batch, and the card's focused selectors for validation. Do not add
Jetstream setup as a prerequisite.

## Suggested Next Move

First audit the current diff against all seven blockers and write a short
rework plan in the PR batch log. Then simplify the kernel from the contract
outward:

1. make the authored declarations closed and projection-safe;
2. validate cases and specimens against those declarations;
3. make observers generic and evidence-bearing;
4. exercise the actual Svelte, React, and GPUI render/input paths;
5. remove Jetstream from the required path;
6. subtract superseded machinery and reassess the line-cost trigger.

Complete one coherent rework rather than sending a series of narrow patches.

## Completion Protocol

Before handing back:

1. Meet every acceptance item and validation command in the revised `g14.001`
   card.
2. Include planted failures that prove interface, structure, behavior, token,
   and focus/interaction drift are detected in the runtime that owns them.
3. Update the existing implementation/batch log with the final architecture,
   exact validation results, deletions, retained compatibility surface, and
   post-rework cost calculation.
4. Run `git diff --check` and the focused Effigy validation batch.
5. Push the amended PR #10.
6. Stop for orchestrator review. Do not mark the card accepted or begin 002.
