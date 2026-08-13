# 065 Scene Authoring And Specimen Fixtures

Status: active
Updated: 2026-08-13
Owner: Poodle core
Depends on: `../architecture/006-headless-core-and-machine-model.md`,
`../architecture/001-poodle-system-shape.md`

## Purpose

Make the scene the one fixture authority. Specimen pages are authored once
and rendered by all four runtimes, so implementation differences are
diagnosable instead of confounded by fixture differences. This is the one
surface the g13 pilot proved replaces duplication instead of adding to it:
four hand-written preview shells became one Rust source, proven across all
four runtimes (`../roadmaps/g13/pilot-verdict-evidence.md` §5, kept by the
g13.020 verdict — `../roadmaps/g13/020-consolidate-and-reassess.md`).

## Scope

**In scope — scene authoring and specimen fixtures:**

- scene definitions: component references and typed prop bindings
- layout nodes, text, groups, loops, conditions, and named slots
- local fixture state and semantic event wiring
- theme, size, density, orientation, and contrast axes
- interaction scenarios and stable capture identifiers
- declared runtime capability requirements

Authoring happens once (`poodle-ir` and `poodle-codegen` carry the
scene-only surface); every runtime renders the same scene. Specimens
migrate onto the scene system in measured tranches, static tier before
interactive (`../roadmaps/g14/003-scene-authored-specimen-migration.md`).

**Out of scope, permanently:**

- any evaluator, or compiled behaviour — fixtures bind literals and declared
  axes, nothing executes
- scenes as an application framework — routing, persistence, data fetching,
  authorization, product state, arbitrary host callbacks, and DAW-specific
  models stay outside
- new framework targets
- component-surface codegen (retired with spec 063's component half)

## Fixture Authority

One scene definition updates all four previews in one build. A change to a
fixture cannot touch one runtime and leave the others on stale fixtures.

- The scene is the source of the specimen page; a runtime's shell renders
  it, and does not carry a second copy of fixture content.
- Scene output is deterministic, versioned, and drift-checked without
  worktree mutation.
- Interaction scenarios carry stable capture identifiers so specimen
  evidence gates (`../specs/064-cross-runtime-machine-pinning.md` mechanism
  5) stay pinned across runs.

## Boundaries

- Implementation differences surface in the rendering, not in the fixture.
  A runtime that renders a scene differently is a finding for the parity
  pairs, not a reason to fork the scene.
- Scenes describe composition and evidence, never product behaviour.
- No application framework grows from the scene system.

## Promotion

Promoted from spec 063's scene half (`063`'s component half is retired;
the retirement is recorded there). Normative here; the migration sequence
lives in the roadmap.
