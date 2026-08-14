---
title: g14.001 third rework handoff
status: active
owner: Poodle core
updated: 2026-08-14
tags: [coordination, handoff, g14, conformance]
---

## What This Thread Was Doing

Rework PR #10 in place on
`t3code/g14-conformance-kernel-button-proof`. The current head is `6440cc5d`.
This is the third review pass for `g14.001`, the Button proof for Poodle's new
cross-runtime conformance plane.

The latest replacement made real progress: nullable TypeScript/Rust shapes now
agree, normalized observations are compared, icon identity is observed, GPUI
activation traverses a real window/input path, Jetstream is program-deferred,
and branch hygiene is clean. `effigy conformance:complete --component button`
passes 20 cases across Svelte, React, and GPUI.

Do not mistake that green run for acceptance. The orchestrator found six
remaining gaps where the proof can pass while its contract is false or where
the implementation contradicts its governing card.

## Why It Matters

`g14.001` is the kernel for every later profile, from RangeSlider through
HistoryCenter. It must make drift hard to express and cheap to detect. A green
Button harness that invents native roles, skips TypeScript checking, narrows a
public callback, or runs only outside standing gates would institutionalize
false confidence across the remaining component estate.

This card also owns the first cost decision. The mechanism cannot defer its
amortization argument to a later component while claiming `001` complete; the
roadmap explicitly says to stop and reassess here.

## Current State

- Done so far: 20 shared Button cases execute in Svelte, React, and GPUI; the
  packed web consumer passes; Jetstream is deferred; `git diff --check` is
  clean.
- Still open: the six corrective findings below, updated planted-failure
  evidence, accurate cost evidence, and another orchestrator review.
- Active spec lane: `g14.001` under provisional spec 066.
- Canonical refs:
  - `/Users/tom/Dev/projects/poodle/docs/architecture/009-cross-runtime-component-conformance.md`
  - `/Users/tom/Dev/projects/poodle/docs/specs/066-executable-component-conformance.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
  - `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`
  - `/Users/tom/Dev/projects/poodle/docs/contracts/components/button.md`
  - `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/14-g14-001-delivery-review.md`
- Remaining continuation envelope: rework PR #10 only. `g14.002` remains
  blocked.
- Lane budget / pause signal: `strict-paused`; do not advance g14 until the
  orchestrator accepts this proof.
- Review evidence worktree:
  - `/Users/tom/.t3/worktrees/poodle/t3code-36a5816f`
- Branch and PR:
  - `t3code/g14-conformance-kernel-button-proof`
  - `https://github.com/inflatable-cookie/poodle/pull/10`

The current blockers are:

1. **GPUI execution is not standing enforcement.**
   `conformance:test-gpui` was placed in `ci:jetstream`, while `ci:native` does
   not run it and `qa` excludes `ci:jetstream`. `conformance:compare` is not
   called by any standing board. The batch log's claim that GPUI execution is
   in `ci:native` is false. Put the executed GPUI proof and comparison on a
   genuine standing surface. Do not make Jetstream setup a prerequisite. If a
   real-window run cannot coexist with the current headless board contract,
   stop and return the smallest concrete board options rather than hiding it
   in `ci:jetstream` or substituting compile-only evidence.

2. **The shared native observer still invents Button semantics.**
   `packages/render/src/conformance.rs::label_text` and `role_of` branch on
   `NodeKind::Button`. The role fallback means deleting the renderer's
   `a11y.role` still observes `button`, so a real role divergence passes. The
   newly added `Node::intrinsic_text()` accessor is not used. Remove the
   Button-specific fallback and make the observer read generic node vocabulary
   only. Plant removal of the renderer's Button a11y role and prove the owning
   gate fails with runtime, case, step, and field.

3. **The typed corpus does not type-check.**
   `PartExpectation` has no `icon` field, while four Button cases author one.
   `bunx tsc -p packages/core/tsconfig.json --pretty false` exposes the four
   new errors at `button-cases.ts:105`, `:106`, `:107`, and `:149`; the runtime
   proof stays green because Bun strips the types. Correct the type model and
   add a narrow standing conformance type-check that does not depend on clearing
   unrelated core baseline errors. A typed authority with no type gate is not
   evidence.

4. **The portable event projection regresses Button's public web API.**
   React and Svelte changed `onClick` from a framework `MouseEvent` callback to
   payload-free `ButtonPortableEvents["press"]`, then cast it back internally
   while still passing the event. The Button contract promises `MouseEvent`,
   and existing consumers may use it. Preserve the public framework callback
   shape while mechanically binding it to the semantic `press` event name;
   do not use a cast to conceal the mismatch. Also fix and type-test
   `PayloadArgs`: its comment says a multi-field payload becomes one object,
   but its implementation currently produces one union-valued argument.

5. **Cross-runtime geometry uses a forbidden blanket tolerance.**
   `test/conformance/compare.ts` applies `GEOMETRY_TOLERANCE = 1.0` to every
   geometry field and case. Architecture 009 forbids blanket runtime
   tolerances; spec 066 requires named, assertion-local tolerances. Carry the
   authored assertion tolerance into comparison or use another equally closed
   data path. Add a planted proof showing a delta outside a case's own bound
   fails and a permitted delta passes. Do not add another global default.

6. **The cost stop condition remains triggered.**
   The live `effigy conformance:cost` result is 7,514 mechanism lines against
   619 replaced, including a 3,251-line reusable kernel and 3,387 generated
   lines. The batch log still reports 7,451, calls the reusable kernel roughly
   2,000 lines, and postpones amortization evidence to RangeSlider. The revised
   card specifically required copied artifacts to be removed before
   remeasurement, but the branch retains four committed JSON artifacts,
   including GPUI copies of the canonical interface and case fixtures.
   Remove avoidable copies and mechanism first; make the cost command quiet,
   exhaustive, and byte-grounded. If the mechanism still grows faster than
   what it replaces, honor the stop condition: return evidence and concrete
   simplify/accept/reject options without claiming completion or starting a
   second component.

## Boundaries

- Rework PR #10 in place. Do not open a replacement PR or stack another
  conformance mechanism beside this one.
- Stay inside the writable scope of
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`.
- Do not start `g14.002`, RangeSlider, or any other component to manufacture
  amortization evidence.
- Keep Jetstream program-deferred and absent from required setup, execution,
  completion, and standing Poodle QA.
- Do not touch licence components, licence contracts, their specimen pages, or
  the orchestrator checkout's uncommitted licence work.
- Do not change architecture 009, spec 066, roadmap status, or dispatch state
  to make the branch fit. The orchestrator owns acceptance and planning state.
- Preserve Button's existing public web API unless the operator explicitly
  approves a breaking change.
- Follow repo constraints from
  [AGENTS.md](/Users/tom/Dev/projects/poodle/AGENTS.md).

## Important Context

- Planning lineage: architecture 009 fixes the conformance invariants; spec
  066 defines the provisional executable format; roadmap card 001 is the
  current Button proof and gate for the rest of g14.
- Spec-to-canonical relationship: implementation details may change, but the
  branch may not weaken exact semantic comparison, invent observations, use
  blanket tolerances, or count a non-standing selector as enforcement.
- Svelte remains the reference when implementations disagree. The component
  contract remains the public API authority.
- Svelte and React share one web substrate. GPUI uses the shared Rust
  composition substrate. Jetstream will later consume that Rust substrate but
  is not part of the active completion cohort now.
- The current green proof is useful evidence, not a reason to patch around the
  six findings. Prefer deleting copied artifacts, false fallbacks, casts, and
  stale claims over adding compatibility layers.
- The main checkout is intentionally dirty with the paused licence review.
  Work only in the PR worktree and do not use destructive Git commands.
- `effigy doctor` has existing repository scan findings. Separate baseline
  findings from regressions; do not widen this card to clean unrelated debt.

## Suggested Next Move

Start with a short audit of the six findings against current head `6440cc5d`.
Then complete one coherent correction batch in this order:

1. restore the Button web callback contract and close the event/case types;
2. remove native semantic invention and prove the role plant fails;
3. replace the blanket tolerance with assertion-local comparison;
4. put full executed comparison on the correct standing enforcement surface;
5. remove copied artifacts, rerun the exhaustive cost report, and make the
   stop-condition decision honestly;
6. update the existing batch log and PR description so every claim matches the
   final code and command output.

Do not send another completion claim based only on
`conformance:complete`. The handback must explain why the standing gate catches
the same planted failures from an ordinary Poodle worktree without Jetstream.

## Completion Protocol

1. Run `effigy test --plan` before the final validation batch.
2. Run every conformance build/check/type/execution/cost selector, including
   `effigy conformance:complete --component button`.
3. Run the narrow Button suites, `effigy test:web-pack-install`, applicable
   `ci:web`, `ci:rust`, and `ci:native` boards, `effigy docs:check`, and
   `git diff --check`.
4. Record exact branch regressions separately from known baseline failures.
5. Record planted failures for: portable prop/event rename, unknown case
   vocabulary, wrong icon identity, missing native a11y role, inert GPUI
   listener, GPUI registry removal, assertion-local geometry bounds, and an
   orphan generated artifact.
6. Update the existing g14.001 batch log with the real final file inventory,
   generated-copy count, mechanism/replacement totals, standing-gate wiring,
   and unresolved stop-condition judgment. Remove stale claims rather than
   adding caveats beside them.
7. Push the amended PR #10 and stop. Do not mark `g14.001` accepted or begin
   `g14.002`.
8. If cost or standing window enforcement remains unresolved, return two or
   three concrete options with measured trade-offs. That is a valid handback;
   another unsupported completion claim is not.
