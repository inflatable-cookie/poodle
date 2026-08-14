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
licence surface. `016` completed its operator review and bounded refinement.
Native and shared-case completion stays blocked under `017` until `008`
records **adopt**.

The operator-approved model-connection suite follows the same staged pattern.
`018` delivers its bounded Svelte/React reference, `019` reviews the live
specimens in the orchestrator thread, and `020` stays blocked on **adopt** for
shared cases and GPUI. Cleanup moves to `021`; `022` closes the generation.

Jetstream is outside the g14 completion cohort. Its paired build and evidence
remain opt-in and visibly deferred; a later backend-admission runway must run
the adopted cases before promotion.

## Runway

1. [001 — Conformance kernel and Button proof](001-conformance-kernel-and-button-proof.md) — complete; replacement proof accepted
2. [002 — Primitive substrate certification](002-primitive-substrate-certification.md) — complete; accepted in PR #11
3. [003 — RangeSlider controlled-control proof](003-range-slider-controlled-control-proof.md) — ready
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
16. [016 — Licence reference review](016-licence-reference-review.md) — complete; reference approved
17. [017 — Licence active-runtime completion](017-licence-active-runtime-completion.md)
18. [018 — Model connection web reference](018-model-connection-web-reference.md) — complete; accepted in PR #12
19. [019 — Model connection reference review](019-model-connection-reference-review.md) — ready; operator/orchestrator lane
20. [020 — Model connection active-runtime completion](020-model-connection-active-runtime-completion.md)
21. [021 — Experimental cleanup and gate consolidation](021-experimental-cleanup-and-gate-consolidation.md)
22. [022 — Generation closeout](022-generation-closeout.md)

## Dispatch Rule

Roadmap files are worker handoffs. The orchestrator dispatches one whole file
to a fresh thread/worktree when its dependencies are met. Workers do not write
`dispatch.md` or change roadmap status. The orchestrator reviews the PR,
records evidence, merges, then opens the next file.

No batch-card layer. No parallel work on the conformance kernel before the
prior profile settles its vocabulary. Bounded web-reference lanes may run in
parallel only when named here. `018` used that exception and is now complete.
`019` stays in this operator/orchestrator thread; it is not a worker dispatch.

## First Task

`g14.003` is the next conformance card. It must prove controlled two-part value
semantics through the certified primitive, observer, and driver substrate.
RangeSlider-specific branches in generic machinery are a stop condition.
Jetstream remains outside the active cohort.

`g14.019` is ready in this orchestrator thread. Review the landed Svelte and
React model-connection specimens with the operator, then freeze the accepted
reference. Native/conformance completion stays open under `020`.
