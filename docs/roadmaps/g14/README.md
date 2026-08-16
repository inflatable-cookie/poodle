# g14 — Executable Component Conformance

Status: active — cleanup delivered, pending orchestrator review
Posture: migration
Opened: 2026-08-14
Verdict: `g14.008` **rejected** the executable conformance mechanism
Governing refs: `008-pilot-verdict.md`,
`../../contracts/001-working-rules.md`

## Generation Result

g14 tested whether one portable interface, executable case corpus, normalized
observation, and completion gate could prevent drift across Svelte, React, and
Rust through GPUI. The goal remains important. This mechanism failed its own
admission rules.

The corrected audit measured 22,746 source LOC against 472 LOC replaced.
HistoryCenter was absent from the comparator; registering it exposed 1,205
cross-runtime differences. Component registration remained manual, two of six
web pairs did not consume the interface authority, primitive certification
claims contradicted the generated report, and GPUI-named evidence entered the
shared Rust layer.

The verdict is reject, not revise. `009`–`014` are retired. `021` removed the
pilot plane, preserving component fixes, focused regressions, and the headless
GPUI test platform. No replacement parity architecture is being designed inside
this generation.

## Retained Learnings

- Svelte and React can share core behaviour and CSS. Rust backends can share
  `poodle-render` and `poodle-node`. That two-substrate shape remains sound.
- Cross-runtime tests found real interaction, focus, accessibility, overlay,
  input, and composite defects. Preserve those claims in smaller owner-local
  tests.
- GPUI validation must stay headless. No local `*-windowed` selector is
  permitted without explicit operator approval.
- Curated specimens are documentation, not exhaustive test matrices. The
  generated corpus projection made Button, RangeSlider, and Tabs less useful
  and was reverted.
- Jetstream remains deferred. Its eventual admission is backend work, not a
  free consequence of shared Rust nodes.
- Approved Licence and model-connection web references remain valid. Their
  native/runtime completion needs a new plan after cleanup.

## Sequence

`001`–`007` and `023` ran the six-profile pilot and repaired headless GPUI
execution. `008` recorded reject. `021` retained the evidence, restored the
hand-written authorities, and removed the rejected plane
(`../../logs/2026-08/16-g14-021-experimental-cleanup-and-gate-consolidation.md`).
`022` closes the generation.

The independent Licence (`015`–`017`), model-connection (`018`–`020`), batched
meter (`024`), catalogue taxonomy (`025`), and specimen audit (`026`) lanes
remain separately tracked. `017`, `020`, and `026` wait for `021` so they do
not build on rejected authority or projection wiring.

## Runway

1. [001 — Conformance kernel and Button proof](001-conformance-kernel-and-button-proof.md) — complete; pilot evidence
2. [002 — Primitive substrate certification](002-primitive-substrate-certification.md) — complete; pilot evidence
3. [003 — RangeSlider controlled-control proof](003-range-slider-controlled-control-proof.md) — complete; pilot evidence
4. [004 — Tabs collection and navigation proof](004-tabs-collection-navigation-proof.md) — complete; pilot evidence
5. [023 — Headless GPUI conformance execution](023-headless-gpui-conformance-execution.md) — complete; infrastructure retained
6. [005 — Popover overlay and focus proof](005-popover-overlay-focus-proof.md) — complete; pilot evidence
7. [006 — TextInput runtime-boundary proof](006-text-input-runtime-boundary-proof.md) — complete; pilot evidence
8. [007 — HistoryCenter composite proof](007-history-center-composite-proof.md) — complete; pilot evidence
9. [008 — Pilot verdict](008-pilot-verdict.md) — complete; **reject**
10. [009 — Foundation and display rollout](009-foundation-display-rollout.md) — retired
11. [010 — Controls and forms rollout](010-controls-forms-rollout.md) — retired
12. [011 — Collections and navigation rollout](011-collections-navigation-rollout.md) — retired
13. [012 — Overlays and input rollout](012-overlays-input-rollout.md) — retired
14. [013 — Composite and workstation rollout](013-composite-workstation-rollout.md) — retired
15. [014 — Completion gate and component factory](014-completion-gate-and-component-factory.md) — retired
16. [015 — Licence web reference](015-licence-web-reference.md) — complete
17. [016 — Licence reference review](016-licence-reference-review.md) — complete; reference approved
18. [017 — Licence active-runtime completion](017-licence-active-runtime-completion.md) — blocked; rewrite after `021`
19. [018 — Model connection web reference](018-model-connection-web-reference.md) — complete
20. [019 — Model connection reference review](019-model-connection-reference-review.md) — complete; reference approved
21. [020 — Model connection active-runtime completion](020-model-connection-active-runtime-completion.md) — blocked; rewrite after `021`
22. [021 — Rejected pilot cleanup and evidence retention](021-experimental-cleanup-and-gate-consolidation.md) — delivered; pending orchestrator review
23. [022 — Generation closeout](022-generation-closeout.md) — **next**, once `021` merges
24. [024 — Batched AudioMeter web surface](024-batched-audio-meter-web-surface.md) — complete
25. [025 — Preview catalogue taxonomy and generated navigation](025-preview-catalogue-taxonomy-and-generated-navigation.md) — complete
26. [026 — Human-centred specimen catalogue audit](026-human-centred-specimen-catalogue-audit.md) — blocked pending `021`

## Dispatch Rule

Roadmap files are worker handoffs. The orchestrator dispatches one whole file
to a fresh thread/worktree when its dependencies are met. Workers do not write
`dispatch.md` or change roadmap status. The orchestrator reviews the PR,
records evidence, merges, then opens the next file.

`g14.021` is delivered and awaiting review. Nothing else dispatches until it
merges: `017`, `020`, `022`, and `026` all depend on knowing what authority and
test infrastructure survived, which is now recorded in `conformance-estate.md`
and the g14.021 log.

## Current Task

Review and merge `g14.021`, then dispatch `g14.022`. Two decisions carry
forward to it: whether `.github/workflows/ci-conformance.yml` is renamed or
deleted (a workflow edit needs explicit operator approval), and what execution
method replaces the rejected pipeline for `017` and `020`.

Local validation stays headless. Never run a `*-windowed` conformance
selector.
