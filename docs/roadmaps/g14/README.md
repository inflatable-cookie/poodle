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
the primitive substrate that later components depend on. `003` and `004`
prove controlled and collection profiles. `023` then replaces the unsafe
foreground GPUI runner with headless GPUI execution before `005`–`007` resume
the increasing component profiles. `008` records an adopt/revise/reject
verdict.

Rollout cards `009`–`013` stay blocked until `008` records **adopt**. `014`
makes the workflow mandatory for new components.

`015` landed the independent Svelte/React reference tranche for the new
licence surface. `016` completed its operator review and bounded refinement.
Native and shared-case completion stays blocked under `017` until `008`
records **adopt**.

The operator-approved model-connection suite follows the same staged pattern.
`018` delivered its bounded Svelte/React reference and `019` approved the live
specimens after one bounded refinement. `020` stays blocked on **adopt** for
shared cases and GPUI. Cleanup moves to `021`; `022` closes the generation.

`024` is an independent web-performance lane for high-count AudioMeter
rendering. It may run beside the conformance sequence because it changes no
portable component meaning, generated conformance surface, Rust, GPUI, or
Jetstream. It ships Canvas2D first and leaves WebGL2 behind a measured
follow-up decision.

`025` is an independent preview-information-architecture lane. It replaces
the overloaded component `tag` with one generated catalogue taxonomy, then
uses that authority to group the active Svelte, React, and GPUI previews.
Jetstream receives generated metadata only; its shell remains deferred.

Jetstream is outside the g14 completion cohort. Its paired build and evidence
remain opt-in and visibly deferred; a later backend-admission runway must run
the adopted cases before promotion.

## Runway

1. [001 — Conformance kernel and Button proof](001-conformance-kernel-and-button-proof.md) — complete; replacement proof accepted
2. [002 — Primitive substrate certification](002-primitive-substrate-certification.md) — complete; accepted in PR #11
3. [003 — RangeSlider controlled-control proof](003-range-slider-controlled-control-proof.md) — complete; accepted in PR #13
4. [004 — Tabs collection and navigation proof](004-tabs-collection-navigation-proof.md) — complete; accepted in PR #14
5. [023 — Headless GPUI conformance execution](023-headless-gpui-conformance-execution.md) — next
6. [005 — Popover overlay and focus proof](005-popover-overlay-focus-proof.md)
7. [006 — TextInput runtime-boundary proof](006-text-input-runtime-boundary-proof.md)
8. [007 — HistoryCenter composite proof](007-history-center-composite-proof.md)
9. [008 — Pilot verdict](008-pilot-verdict.md)
10. [009 — Foundation and display rollout](009-foundation-display-rollout.md)
11. [010 — Controls and forms rollout](010-controls-forms-rollout.md)
12. [011 — Collections and navigation rollout](011-collections-navigation-rollout.md)
13. [012 — Overlays and input rollout](012-overlays-input-rollout.md)
14. [013 — Composite and workstation rollout](013-composite-workstation-rollout.md)
15. [014 — Completion gate and component factory](014-completion-gate-and-component-factory.md)
16. [015 — Licence web reference](015-licence-web-reference.md) — landed
17. [016 — Licence reference review](016-licence-reference-review.md) — complete; reference approved
18. [017 — Licence active-runtime completion](017-licence-active-runtime-completion.md)
19. [018 — Model connection web reference](018-model-connection-web-reference.md) — complete; accepted in PR #12
20. [019 — Model connection reference review](019-model-connection-reference-review.md) — complete; reference approved
21. [020 — Model connection active-runtime completion](020-model-connection-active-runtime-completion.md)
22. [021 — Experimental cleanup and gate consolidation](021-experimental-cleanup-and-gate-consolidation.md)
23. [022 — Generation closeout](022-generation-closeout.md)
24. [024 — Batched AudioMeter web surface](024-batched-audio-meter-web-surface.md) — ready; independent web-performance lane
25. [025 — Preview catalogue taxonomy and generated navigation](025-preview-catalogue-taxonomy-and-generated-navigation.md) — ready; independent preview-IA lane

## Dispatch Rule

Roadmap files are worker handoffs. The orchestrator dispatches one whole file
to a fresh thread/worktree when its dependencies are met. Workers do not write
`dispatch.md` or change roadmap status. The orchestrator reviews the PR,
records evidence, merges, then opens the next file.

No batch-card layer. No parallel work on the conformance kernel before the
prior profile settles its vocabulary. Bounded web-reference lanes may run in
parallel only when named here. `018` used that exception and is now complete.
`019` stayed in the operator/orchestrator thread and is now complete. `024`
may run independently under its explicit no-conformance/no-native boundary.
`025` may also run independently. It owns preview catalogue metadata and
navigation only; it must rebase concurrent registry additions before review.

## First Task

`g14.023` is next. It replaces the foreground GPUI conformance runner with
headless GPUI execution before `005` resumes the component profiles. Jetstream
stays outside the active cohort. Its local validation remains headless.

The model-connection web reference is approved. Native/conformance completion
stays open under `020` until the pilot records **adopt**.

`g14.024` is ready for a separate worktree now. Give the worker the complete
roadmap file; its Canvas2D, browser, allocation, and performance evidence must
return as one PR for orchestrator review.

`g14.025` is ready for a separate worktree now. Give the worker the complete
roadmap file. It must return the complete classification, generator, active
preview navigation, drift gate, and headless validation as one PR.
