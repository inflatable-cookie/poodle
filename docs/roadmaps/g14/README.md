# g14 — Executable Component Conformance

Status: active
Posture: migration
Opened: 2026-08-14
Governing refs: `../../architecture/009-cross-runtime-component-conformance.md`,
`../../specs/066-executable-component-conformance.md`,
`../../contracts/001-working-rules.md`

## Goal

Make portable drift unable to remain silent across Svelte, React, and Rust
through GPUI. Define component interface and specimen structure once, execute
the same cases in the active runtimes, compare normalized observable output,
and block completion while an active runtime is missing or inert. Keep the
Rust case and node boundary renderer-neutral for later Jetstream admission.

## Fixed Decisions

- Svelte is the reference implementation; contracts define shipped meaning.
- Svelte + React share `poodle-core` and CSS.
- GPUI uses shared `poodle-render` and `poodle-node`; Jetstream remains a
  deferred consumer of that same substrate.
- Cross-language behaviour remains hand-implemented in those two substrates.
- Portable interfaces may generate declarations. Behaviour may not.
- Shared cases own fixture data, test actions/assertions, and specimen
  structure. A separate scene component model is not allowed.
- Runtimes normalize their real output into one semantic observation format.
- Missing or declared-absent required capability is incomplete, not parity.
- Every cost report includes schema, generators, generated files, adapters,
  runners, tests, and wiring.

## Sequence

`001` builds the smallest end-to-end kernel through Button. `002` certifies
the primitive substrate that later components depend on. `003`–`007` prove
the same pipeline across increasing component profiles. `008` records an
adopt/revise/reject verdict.

Rollout cards `009`–`013` stay blocked until `008` records **adopt**. `014`
makes the workflow mandatory for new components.

`015` landed the independent Svelte/React reference tranche for the new
licence surface. `016` is an immediate operator review and bounded refinement
checkpoint over those live specimen pages. Native and shared-case completion
stays blocked under `017` until `008` records **adopt**. `018` deletes
superseded experiments and gates. `019` closes the generation.

Jetstream is outside the g14 completion cohort. Its paired build and evidence
remain opt-in and visibly deferred; a later backend-admission runway must run
the adopted cases before promotion.

## Runway

1. [001 — Conformance kernel and Button proof](001-conformance-kernel-and-button-proof.md) — review-blocked; PR #10 replacement required
2. [002 — Primitive substrate certification](002-primitive-substrate-certification.md) — blocked on 001
3. [003 — RangeSlider controlled-control proof](003-range-slider-controlled-control-proof.md)
4. [004 — Tabs collection and navigation proof](004-tabs-collection-navigation-proof.md)
5. [005 — Popover overlay and focus proof](005-popover-overlay-focus-proof.md)
6. [006 — TextInput runtime-boundary proof](006-text-input-runtime-boundary-proof.md)
7. [007 — HistoryCenter composite proof](007-history-center-composite-proof.md)
8. [008 — Pilot verdict](008-pilot-verdict.md)
9. [009 — Foundation and display rollout](009-foundation-display-rollout.md)
10. [010 — Controls and forms rollout](010-controls-forms-rollout.md)
11. [011 — Collections and navigation rollout](011-collections-navigation-rollout.md)
12. [012 — Overlays and input rollout](012-overlays-input-rollout.md)
13. [013 — Composite and workstation rollout](013-composite-workstation-rollout.md)
14. [014 — Completion gate and component factory](014-completion-gate-and-component-factory.md)
15. [015 — Licence web reference](015-licence-web-reference.md) — landed
16. [016 — Licence reference review](016-licence-reference-review.md) — ready operator checkpoint
17. [017 — Licence active-runtime completion](017-licence-active-runtime-completion.md)
18. [018 — Experimental cleanup and gate consolidation](018-experimental-cleanup-and-gate-consolidation.md)
19. [019 — Generation closeout](019-generation-closeout.md)

## Dispatch Rule

Roadmap files are worker handoffs. The orchestrator dispatches one whole file
to a fresh thread/worktree when its dependencies are met. Workers do not write
`dispatch.md` or change roadmap status. The orchestrator reviews the PR,
records evidence, merges, then opens the next file.

No batch-card layer. No parallel work on the conformance kernel before the
prior profile settles its vocabulary. The bounded licence review in `016` is
the only current parallel exception; it cannot edit kernel, Rust, native, or
roadmap surfaces.

## First Task

`g14.001` is the next worker card. PR #10 is retained as failed-pilot evidence,
not merged authority. The replacement pass must prove one authored Button interface
and case can drive Svelte, React, shared Rust composition, and GPUI
observations, tests, and specimen structure without becoming a behaviour
compiler or additive mirror.

`g14.016` is active as an orchestrator/operator checkpoint in this thread.
Review the landed
Svelte and React licence specimens, record feedback, and permit at most one
bounded web-reference refinement pass. Native/conformance completion remains
visibly open under `017`.
