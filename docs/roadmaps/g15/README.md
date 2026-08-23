# g15 — v0.2.1 Recovery Release Baseline

Status: active — every implementation, specimen, conformance, packaging,
automation, dependency-policy, and adopter card through `g15.053` is complete;
`v0.2.0` failed before publication and the replacement `0.2.1` candidate is
green under `g15.054`, awaiting its exact commit/receipt pin before the
`g15.013` operator gate
Posture: release-first; no new parity architecture
Opened: 2026-08-16
Governing refs: `../g14/022-generation-closeout.md`,
`../g14/conformance-estate.md`, `../../contracts/001-working-rules.md`,
`../../logs/2026-08/16-g14-022-generation-closeout.md`

## Generation Goal

Ship Poodle on an honest, complete Svelte roster. The original `v0.2.0` tag is
retained after a pre-publication workflow failure; `0.2.1` is the recovery
release. The release
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
release children through `g15.051`.

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
11. [011 — Human-centred specimen catalogue audit](011-specimen-catalogue-audit.md) — complete; the full catalogue carries human verdicts
12. [015 — Specimen caption integrity](015-specimen-caption-integrity.md) — complete; PR #37 restored 52 captions and closed the Svelte-preview gate hole
13. [016 — Specimen idiom convergence](016-specimen-idiom-convergence.md) — complete; PR #38 converged all 29 paired routes
14. [017 — Web specimen axis placement](017-specimen-axis-placement.md) — complete; PR #39 plus accepted Dialog follow-up
15. [018 — Overloaded Examples curation](018-overloaded-examples-curation.md) — complete parent; exact children `020`–`025` landed
16. [019 — GPUI specimen structure](019-gpui-specimen-structure.md) — complete; PR #40 landed the 74-axis/6-caption scope and returned two honest axis-domain gaps
17. [034 — Component-specific specimen axis domains](034-component-specific-specimen-axis-domains.md) — complete; PR #41 closed EmptyState/Icon domains and hardened axis evidence
18. [020–025 — Overloaded Examples family children](020-curate-model-connection-licence.md) — complete; exact 53-page partition landed through PR #49
19. [026 — Headless native specimen probe](026-native-specimen-probe.md) — complete; PR #50 proved 174/174 native routes and all admitted axis panes headlessly
20. [027 — Screen-clear human review](027-screen-clear-human-review.md) — complete non-dispatchable parent for exact children `028`–`033`
21. [028–033 — Screen-clear family review children](028-review-foundation-controls-entry.md) — complete exact 56-page partition
22. [038 — SegmentedControl native option parity](038-segmented-control-native-option-parity.md) — complete; PR #52 landed the clean option-type migration and stable native focus identity
23. [039 — DateTimeZonePicker nested-layer pointer commit](039-date-time-zone-picker-nested-layer.md) — complete; PR #54 closed the paired-web pointer blocker and unblocked `g15.030`
24. [040 — ResizeHandle native keyboard and value semantics](040-resize-handle-native-semantics.md) — complete; PR #56 closed stable native focus identity, keyboard, and numeric-range semantics
25. [041 — Popover interactive trigger semantics](041-popover-interactive-trigger-semantics.md) — complete; PR #59 landed the clean state-aware trigger migration and closed `g15.032`
26. [042 — Stepper native interaction parity](042-stepper-native-interaction-parity.md) — complete; PR #60 closed inert GPUI selection/re-run controls
27. [043 — UiPresentationProvider native cascade](043-ui-presentation-provider-native-cascade.md) — complete; PR #70 landed the cascade and PR #71 corrected explicit-size parity found in review
28. [053 — Breadcrumb item icons](053-breadcrumb-item-icons.md) — complete; PR #71 adds icon-plus-label and accessible icon-only items across the active cohort
29. [012 — Primitive-first visual conformance lane](012-visual-conformance-lane.md) — non-dispatchable parent recompiled into `044`–`047`
30. [044 — GPUI offscreen capture feasibility](044-gpui-offscreen-capture-feasibility.md) — complete; PR #61 proved a deterministic no-focus Metal pixel path
31. [045 — GPUI offscreen capture adoption](045-gpui-offscreen-capture-adoption.md) — complete; PR #62 adopted the exact no-focus Metal seam
32. [046 — Primitive visual fixture inventory](046-primitive-visual-fixture-inventory.md) — complete; PR #65 froze the exact 18-case Button inventory and paired validators
33. [047 — Primitive visual comparison](047-primitive-visual-comparison.md) — complete; PR #68 landed the exact 18-fixture comparison after operator review and routed the measured native focus defect to `052`; closes parent `012`
34. [048 — Packed roster reachability](048-packed-roster-reachability.md) — complete; PR #64 proves exact 175/175 packed-root reachability for both web runtimes
35. [049 — Release automation truthfulness](049-release-automation-truthfulness.md) — complete; PR #66 made every retained workflow a pinned Effigy launcher and made the release board non-vacuous
36. [051 — GPUI/Zed dependency licence remediation](051-gpui-zed-dependency-licence-remediation.md) — complete; PR #67 removed GPL tracing from the resolved GPUI graph through a minimal exact-revision fork and hardened licence/source policy
37. [050 — v0.2.0 release candidate](050-v020-release-candidate.md) — complete; registry preflight replaced the PR #72 candidate with `7922a3a9`, recorded by evidence-only receipt `f9b5dcab`
38. [014 — Release-gate remediation: security advisory prerequisite](014-release-gate-remediation.md) — complete; PR #31 cleared the `bun audit` nanoid advisory
39. [054 — v0.2.1 release recovery](054-v021-release-recovery.md) — candidate green; isolated npm bootstrap and lockstep patch are complete, awaiting exact commit/receipt pin
40. [013 — v0.2.1 release certification](013-v020-release-certification.md) — final operator tag/publication gate after accepted `054`
41. [035 — Solid tone surfaces](035-solid-status-surfaces.md) — complete; PR #44 landed shared solid treatments, with Pill's duplicate axis immediately superseded by `036`
42. [036 — Pill appearance semantics](036-pill-appearance-semantics.md) — complete; PR #45 removed Pill's temporary `fill`, added tint appearance, and made existing solid appearance truthful
43. [037 — AgentTranscript native scroll and jump parity](037-agent-transcript-native-scroll-parity.md) — complete in PR #48; real GPUI viewport unblocked `024`
44. [052 — Native focus-ring parity](052-native-focus-ring-parity.md) — complete; PR #69 landed one reusable node channel and closed the measured Button ring and Stepper keyboard-entry/focus gaps

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

`g15.012` and its exact children are complete. PR #68 landed `g15.047` after
operator review of all 54 captures. The diagnostic mechanism worked: web is
exact, native geometry and pixels are close, and it exposed a real focus-ring
defect without weakening policy.

`g15.048` is complete in PR #64. Its exact packed-package proof is included in
the accepted `g15.050` candidate.

`g15.049` is complete in PR #66. The retained workflows are pinned Effigy
launchers, branch publication fails closed, and `effigy release gates` now
executes the complete headless board.

`g15.051` is complete in PR #67. GPUI's normal GPL tracing graph is gone,
the exact Poodle-owned Zed fork is pinned, the permissive bzip2 licence carries
its dependency-specific notice, and exact Git-source policy fails closed.

`g15.052` is complete in PR #69. Its reusable focus-ring channel closed both
the Button defect measured by `g15.047` and the Stepper keyboard-entry/focus
gap retained from `g15.042`, without widening comparator policy.

`g15.043` is complete in PR #70, with the explicit-size semantics corrected
across the native resolver and architecture evidence in PR #71: explicit size
is final; `sizeRole` maps inherited presentation scale only.

`g15.053` is complete in PR #71. Per-item icons and the accessible icon-only
root are accepted across the active cohort.

`g15.050` is complete. The first `g15.013` registry preflight found and corrected a
false first-publication claim, then replaced the candidate with `7922a3a9` and
reran the complete board. Evidence-only receipt `f9b5dcab` records the result.
The operator authorised `v0.2.0`, but run `32656225297` failed while replacing
npm in place, before gates, packing, or publication. The tag stays immutable;
`g15.054` owns the `0.2.1` recovery candidate. Its full headless board and
release wrapper are green; the exact commit and receipt are the remaining
non-mutating closeout before `g15.013` returns as the sole operator gate.
