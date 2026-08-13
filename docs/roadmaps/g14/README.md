# g14 — Executable Component Conformance

Status: active
Opened: 2026-08-14
Governing refs: `../../architecture/009-cross-runtime-component-conformance.md`,
`../../specs/066-executable-component-conformance.md`,
`../../contracts/001-working-rules.md`

## Goal

Make portable drift unable to remain silent across Svelte, React, GPUI, and
Jetstream. Define component interface and specimen structure once, execute the
same cases in all runtimes, compare normalized observable output, and block
completion while a required runtime is missing or inert.

## Fixed Decisions

- Svelte is the reference implementation; contracts define shipped meaning.
- Svelte + React share `poodle-core` and CSS.
- GPUI + Jetstream share `poodle-render` and `poodle-node`.
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
makes the workflow mandatory for new components. `015` deletes superseded
experiments and gates. `016` closes the generation.

## Runway

1. [001 — Conformance kernel and Button proof](001-conformance-kernel-and-button-proof.md) — ready
2. [002 — Primitive substrate certification](002-primitive-substrate-certification.md)
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
15. [015 — Experimental cleanup and gate consolidation](015-experimental-cleanup-and-gate-consolidation.md)
16. [016 — Generation closeout](016-generation-closeout.md)

## Dispatch Rule

Roadmap files are worker handoffs. The orchestrator dispatches one whole file
to a fresh thread/worktree when its dependencies are met. Workers do not write
`dispatch.md` or change roadmap status. The orchestrator reviews the PR,
records evidence, merges, then opens the next file.

No batch-card layer. No parallel work on the conformance kernel before the
prior profile settles its vocabulary.

## First Task

`g14.001` is ready. It must prove one authored Button interface and case can
drive all four implementations, observations, tests, and specimen structure
without becoming a behaviour compiler or additive mirror.
