# g15 — v0.2.0 Release Baseline

Status: active — `g15.001`–`g15.010`, `g15.014`–`g15.017`,
`g15.019`–`g15.026`, `g15.028`–`g15.032`, and `g15.034`–`g15.041` complete;
`g15.042` and `g15.044` are in flight; `g15.033` and `g15.048` are ready;
the remaining release path is recompiled through `g15.050`
Posture: release-first; no new parity architecture
Opened: 2026-08-16
Governing refs: `../g14/022-generation-closeout.md`,
`../g14/conformance-estate.md`, `../../contracts/001-working-rules.md`,
`../../logs/2026-08/16-g14-022-generation-closeout.md`

## Generation Goal

Ship Poodle v0.2.0 on an honest, complete Svelte roster. The release
denominator is **every public Svelte component export** — frozen from
`packages/svelte/components/src/index.ts` and the package `exports` map, not
from a representative subset. React stays tightly paired through shared CSS
and framework-free web behaviour; the measured shared Rust and GPUI gaps close
before certification; Jetstream remains program-deferred.

This generation is release-first, not architecture-first. Release-first means
the runway closes the measured implementation, specimen, and evidence gaps
before certification; it does not make certification an early exit. Two parity
architectures were tried and rejected (g13's Rust IR, g14's executable
conformance). g15 does not design a third one. It inventories, closes the
honest gaps, and ships.

## Release Denominator and Active-Cohort Closure

The full Svelte roster is the v0.2.0 release denominator. The generation still
completes its active-cohort implementation and evidence cards before release
certification. Experimental package labels remain honest, Jetstream remains
program-deferred, and no runtime borrows another runtime's pass.

## Why It Matters

Longhorn and most projects under `~/Dev/projects` depend on Poodle. v0.2.0
cannot wait for another speculative cross-runtime architecture. The release
baseline must record what is actually certified per component, what remains
unproved, and what each runtime's evidence is — then ship.

## Sequence

`001` and `002` are executed; the inventory froze the denominator and the
first evidence tranche closed 29 paired gaps. Later cards are listed in
dependency order and are not dispatched until the orchestrator reviews and
advances them. `g15.014` was an urgent prerequisite remediation executed out
of numeric order; `g15.013` remains the final operator gate after the compiled
release children through `g15.050`.

## Runway

Measured from `g15.001`'s frozen roster (`release-baseline-roster.md`) and gap
register (`release-gap-register.md`). Dependency order only; orchestration
and status advance are the orchestrator's.

1. [001 — Release-baseline roster inventory](001-release-baseline-roster-inventory.md) — complete
2. [002 — Svelte focused evidence: foundation display & shell](002-svelte-focused-evidence-display-shell.md) — complete; 29 paired evidence gaps closed
3. [003 — Svelte focused evidence: forms, inputs & overlays](003-svelte-focused-evidence-forms-inputs-overlays.md) — complete; 26 Svelte and 25 React evidence gaps closed
4. [004 — Svelte focused evidence: composites & media](004-svelte-focused-evidence-composites-media.md) — complete; 35 paired evidence gaps closed
5. [005 — Svelte focused evidence: workstation & agent](005-svelte-focused-evidence-workstation-agent.md) — complete; final 24 Svelte and 23 React evidence gaps closed
6. [006 — React mirror implementation & gallery closure](006-react-mirror-closure.md) — complete; React implementation/gallery are 175/0 and focused evidence is 152/23
7. [007 — Licence family native completion](007-licence-family-native-completion.md) — complete; PR #32 closed the Licence native family and prerequisites
8. [008 — Model-connection family native completion](008-model-connection-family-native-completion.md) — complete; PR #33 closed the model-connection native family
9. [009 — Update, settings, Radio & context-provider native closure](009-update-settings-radio-native-closure.md) — complete; PR #34 closed the scoped native surfaces and declared UiPresentationProvider's remaining cascade gap
10. [010 — Display, workstation & agent GPUI specimens](010-display-workstation-agent-gpui-specimens.md) — complete; PR #35 closed all 18 measured GPUI specimen gaps
11. [011 — Human-centred specimen catalogue audit](011-specimen-catalogue-audit.md) — partial; screening baseline and three approved pilots delivered
12. [015 — Specimen caption integrity](015-specimen-caption-integrity.md) — complete; PR #37 restored 52 captions and closed the Svelte-preview gate hole
13. [016 — Specimen idiom convergence](016-specimen-idiom-convergence.md) — complete; PR #38 converged all 29 paired routes
14. [017 — Web specimen axis placement](017-specimen-axis-placement.md) — complete; PR #39 plus accepted Dialog follow-up
15. [018 — Overloaded Examples curation](018-overloaded-examples-curation.md) — complete parent; exact children `020`–`025` landed
16. [019 — GPUI specimen structure](019-gpui-specimen-structure.md) — complete; PR #40 landed the 74-axis/6-caption scope and returned two honest axis-domain gaps
17. [034 — Component-specific specimen axis domains](034-component-specific-specimen-axis-domains.md) — complete; PR #41 closed EmptyState/Icon domains and hardened axis evidence
18. [020–025 — Overloaded Examples family children](020-curate-model-connection-licence.md) — complete; exact 53-page partition landed through PR #49
19. [026 — Headless native specimen probe](026-native-specimen-probe.md) — complete; PR #50 proved 174/174 native routes and all admitted axis panes headlessly
20. [027 — Screen-clear human review](027-screen-clear-human-review.md) — non-dispatchable parent for exact children `028`–`033`
21. [028–033 — Screen-clear family review children](028-review-foundation-controls-entry.md) — exact 56-page partition; `028`–`032` complete, `033` ready
22. [038 — SegmentedControl native option parity](038-segmented-control-native-option-parity.md) — complete; PR #52 landed the clean option-type migration and stable native focus identity
23. [039 — DateTimeZonePicker nested-layer pointer commit](039-date-time-zone-picker-nested-layer.md) — complete; PR #54 closed the paired-web pointer blocker and unblocked `g15.030`
24. [040 — ResizeHandle native keyboard and value semantics](040-resize-handle-native-semantics.md) — complete; PR #56 closed stable native focus identity, keyboard, and numeric-range semantics
25. [041 — Popover interactive trigger semantics](041-popover-interactive-trigger-semantics.md) — complete; PR #59 landed the clean state-aware trigger migration and closed `g15.032`
26. [042 — Stepper native interaction parity](042-stepper-native-interaction-parity.md) — **in flight** on PR #60; closes inert GPUI selection/re-run controls
27. [043 — UiPresentationProvider native cascade](043-ui-presentation-provider-native-cascade.md) — planned; orchestrator architecture decision required before dispatch
28. [012 — Primitive-first visual conformance lane](012-visual-conformance-lane.md) — non-dispatchable parent recompiled into `044`–`047`
29. [044 — GPUI offscreen capture feasibility](044-gpui-offscreen-capture-feasibility.md) — **in flight** on PR #61; decisive no-focus pixel proof
30. [045 — GPUI offscreen capture adoption](045-gpui-offscreen-capture-adoption.md) — planned after an operator-reviewed `044` go verdict
31. [046 — Primitive visual fixture inventory](046-primitive-visual-fixture-inventory.md) — planned after `011` and `045`
32. [047 — Primitive visual comparison](047-primitive-visual-comparison.md) — planned after `046`; closes parent `012`
33. [048 — Packed roster reachability](048-packed-roster-reachability.md) — **ready**; late web API migrations are complete
34. [049 — Release automation truthfulness](049-release-automation-truthfulness.md) — blocked on explicit workflow-edit approval
35. [050 — v0.2.0 release candidate](050-v020-release-candidate.md) — planned after every implementation, specimen, conformance, packaging, and automation gate
36. [014 — Release-gate remediation: security advisory prerequisite](014-release-gate-remediation.md) — complete; PR #31 cleared the `bun audit` nanoid advisory
37. [013 — v0.2.0 release certification](013-v020-release-certification.md) — final operator tag/publication gate after accepted `050`
38. [035 — Solid tone surfaces](035-solid-status-surfaces.md) — complete; PR #44 landed shared solid treatments, with Pill's duplicate axis immediately superseded by `036`
39. [036 — Pill appearance semantics](036-pill-appearance-semantics.md) — complete; PR #45 removed Pill's temporary `fill`, added tint appearance, and made existing solid appearance truthful
40. [037 — AgentTranscript native scroll and jump parity](037-agent-transcript-native-scroll-parity.md) — complete in PR #48; real GPUI viewport unblocked `024`

Supporting evidence: [release-baseline-roster.md](release-baseline-roster.md),
[release-gap-register.md](release-gap-register.md)

## Carry-Forward Envelope (recorded, not implemented)

These enter g15 with their g14 dispositions and are not dispatched as g14
work:

- Approved Licence web references (`g14.015`/`g14.016`); native completion
  recompiled from `g14.017`'s component requirements
- Approved model-connection web references (`g14.018`/`g14.019`); native
  completion recompiled from `g14.020`'s component requirements
- Human-centred specimen catalogue audit (`g14.026`), carried forward with its
  rubric and bounded shared specimen-plan boundary intact
- Primitive-first visual conformance, which may reuse the retained headless
  and native capture foundation (`conformance-estate.md`); the seam is
  recorded, not built
- Native completion of any component the inventory finds incomplete

## Dispatch Rule

Follows the [worker dispatch ledger](../dispatch.md) contract: the orchestrator
dispatches one whole card to a fresh thread/worktree when its dependencies are
met. Workers do not write `dispatch.md` or change roadmap status.

## Current Task And Parallel Lanes

`g15.041` and `g15.032` are complete. `g15.033` is the final serial
screen-clear review and is ready.

Two independent cards are already in flight:

- `g15.042` wires the already-rendered Stepper selection/re-run controls in
  GPUI on PR #60.
- `g15.044` is the capture-platform feasibility proof. It changes no production
  package and decides whether a real no-focus GPUI pixel path exists on PR #61.

`g15.048` is also ready now that the late web public-surface migrations have
settled; dispatch it only when a worker slot is free and its writable scope
does not overlap another release-tooling lane.

Do not dispatch `g15.043` until the orchestrator fixes the native presentation-
context architecture. Do not dispatch `g15.045` without `g15.044`'s `go`
verdict, or `g15.046` before the human-centred audit completes. `g15.049`
requires explicit operator approval because it edits workflows. Release
mutation remains behind `g15.013`.
