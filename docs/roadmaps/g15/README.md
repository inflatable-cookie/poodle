# g15 — v0.2.x Release and Consumer Adoption

Status: complete — corrected `v0.2.2` published from candidate `d5607def`;
all 16 authoritative consumers adopted the released boundary, React remains
source-only, and Jetstream backend admission remains deferred
Posture: release-first; no new parity architecture
Opened: 2026-08-16
Closed: 2026-08-25
Governing refs: `../g14/022-generation-closeout.md`,
`../g14/conformance-estate.md`, `../../contracts/001-working-rules.md`,
`../../logs/2026-08/16-g14-022-generation-closeout.md`

## Generation Goal

Ship Poodle on an honest, complete Svelte roster, publish the resulting
release, and move the authoritative consumer estate onto that public boundary.
The original `v0.2.0` tag stopped before publication. `v0.2.1` recovered the
release but exposed a fork-sourced GPUI identity in downstream Rust graphs.
`v0.2.2` restored crates.io GPUI, republished the corrected packages, and became
the adopted release. The release denominator is **every public Svelte component
export** — frozen from
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

Longhorn and most projects under `~/Dev/projects` depend on Poodle. The release
could not wait for another speculative cross-runtime architecture, and it was
not complete while consumers still resolved local or incompatible identities.
The generation therefore records what was certified, publishes the corrected
boundary, and proves that boundary in every authoritative consumer.

## Sequence

Cards `001`–`054` built and published the release baseline. Cards `055`–`079`
continue the same programme through the downstream defect discovery, corrected
`v0.2.2` release, and consumer rollout. They were provisionally numbered as
g16 during execution and consolidated here on 2026-08-25 because no new
strategic programme began. Historical branch, handoff, and log filenames keep
their original labels as provenance.

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
39. [054 — v0.2.1 release recovery](054-v021-release-recovery.md) — complete; candidate `3d914261` is green with an exact receipt
40. [013 — v0.2.1 release certification](013-v020-release-certification.md) — complete; run `32658293188` published core and Svelte from exact candidate `3d914261`
41. [035 — Solid tone surfaces](035-solid-status-surfaces.md) — complete; PR #44 landed shared solid treatments, with Pill's duplicate axis immediately superseded by `036`
42. [036 — Pill appearance semantics](036-pill-appearance-semantics.md) — complete; PR #45 removed Pill's temporary `fill`, added tint appearance, and made existing solid appearance truthful
43. [037 — AgentTranscript native scroll and jump parity](037-agent-transcript-native-scroll-parity.md) — complete in PR #48; real GPUI viewport unblocked `024`
44. [052 — Native focus-ring parity](052-native-focus-ring-parity.md) — complete; PR #69 landed one reusable node channel and closed the measured Button ring and Stepper keyboard-entry/focus gaps
45. [055 — Consumer adoption inventory](055-consumer-adoption-inventory.md) — complete; froze 16 authoritative repositories, source policy, exclusions, and dependency order
46. [056 — Longhorn Poodle 0.2.1 adoption](056-longhorn-poodle-v021-adoption.md) — stopped and superseded after exposing the fork-sourced GPUI identity
47. [057 — Underlay Poodle 0.2.1 adoption](057-underlay-poodle-v021-adoption.md) — complete; PR 4 removed the local override before the corrected patch
48. [058 — Soundcheck Library Poodle 0.2.1 adoption](058-soundcheck-library-poodle-v021-adoption.md) — complete; PR 5 established the public package shape before the corrected patch
49. [059 — GPUI crates.io recovery](059-gpui-cratesio-recovery.md) — complete; PR 73 restored one crates.io GPUI identity and retained headless evidence
50. [060 — v0.2.2 release candidate](060-v022-release-candidate.md) — complete; accepted exact candidate `d5607def`
51. [061 — v0.2.2 release certification](061-v022-release-certification.md) — complete; run `32756610293` published core and Svelte 0.2.2 and verified a clean registry consumer
52. [062 — Longhorn Poodle 0.2.2 adoption](062-longhorn-poodle-v022-adoption.md) — complete; PR 9 proved public web packages, the Rust tag, and one crates.io GPUI identity
53. [063 — Underlay Poodle 0.2.2 follow-up](063-underlay-poodle-v022-follow-up.md) — complete; PR 5 moved the foundation to the corrected patch
54. [064 — Soundcheck Library Poodle 0.2.2 follow-up](064-soundcheck-library-poodle-v022-follow-up.md) — complete; PR 6 aligned development and peer ranges
55. [065 — Nucleus Poodle 0.2.2 adoption](065-nucleus-poodle-v022-adoption.md) — complete; PR 1 proved one public Poodle identity
56. [066 — Soundcheck Poodle 0.2.2 adoption](066-soundcheck-poodle-v022-adoption.md) — complete; PR 11 aligned the app, Longhorn adapter, and Soundcheck Library
57. [067 — Underlay Reference Poodle 0.2.2 adoption](067-underlay-reference-poodle-v022-adoption.md) — complete; PR 1 removed Poodle sibling overrides
58. [068 — Acowtancy adoption](068-acowtancy-poodle-v022-adoption.md) — complete; tagged Underlay and public Poodle landed through PR 56
59. [069 — Compli Me adoption](069-compli-me-poodle-v022-adoption.md) — complete; PR 3 preserved the newer Underlay v0.9.4 boundary
60. [070 — Composer adoption](070-composer-poodle-v022-adoption.md) — complete; PR 1 aligned tagged Underlay and public Poodle
61. [071 — Contact Patch adoption](071-contact-patch-poodle-v022-adoption.md) — complete; PR 1 aligned web and Rust dependency sources
62. [072 — Songsprout adoption](072-songsprout-poodle-v022-adoption.md) — complete; PR 1 aligned Bloom, Greenhouse, Stem, and Nursery
63. [073 — Finch adoption](073-finch-poodle-v022-adoption.md) — complete; PR 1 merged the reviewed public-package boundary
64. [074 — Figmatic adoption](074-figmatic-poodle-v022-adoption.md) — complete; PR 17 removed Poodle sibling sources
65. [075 — Bovine Accelerator Desktop adoption](075-bovine-accelerator-desktop-poodle-v022-adoption.md) — complete; PR 25 merged the reviewed boundary
66. [076 — Loophole adoption](076-loophole-poodle-v022-adoption.md) — complete; PR 8 proved one registry Poodle identity
67. [077 — Jetstream adoption](077-jetstream-poodle-v022-adoption.md) — complete; PR 1 moved the web surface while retaining explicit paired Rust paths
68. [078 — Loophole Legacy adoption](078-loophole-legacy-poodle-v022-adoption.md) — cancelled after the operator removed the repository
69. [079 — Underlay Reference tagged Underlay adoption](079-underlay-reference-v092-adoption.md) — complete; PR 2 preserved Poodle 0.2.2 while moving all active Underlay sources to v0.9.4

Supporting evidence: [release-baseline-roster.md](release-baseline-roster.md),
[release-gap-register.md](release-gap-register.md)

## Consumer Adoption Boundary

The 16-repository rollout was complete only when each authoritative consumer
resolved exact npm 0.2.2 or Rust tag `v0.2.2`, carried no active committed
Poodle `file:` override, installed cleanly from its declared sources, and had
its own relevant headless validation plus merged PR. Longhorn-shaped Rust
consumers proved one crates.io GPUI identity. The six Underlay consumer estates
also converged web and Rust on one released Underlay tag with no active sibling
Underlay source; Compli Me and Underlay Reference legitimately advanced from
the v0.9.2 baseline to v0.9.4.

Historical snapshots were never consumers. Loophole Legacy left the estate
when the operator removed its repository. Jetstream's public web packages moved
to 0.2.2 while its explicitly paired local Rust paths remained local under the
deferred backend-integration contract.

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

## Generation Closeout

`g15.012` and its exact children are complete. PR #68 landed `g15.047` after
operator review of all 54 captures. The diagnostic mechanism worked: web is
exact, native geometry and pixels are close, and it exposed a real focus-ring
defect without weakening policy.

`g15.048` is complete in PR #64. Its exact packed-package proof is included in
the accepted `g15.050` candidate.

`g15.049` is complete in PR #66. The retained workflows are pinned Effigy
launchers, branch publication fails closed, and `effigy release gates` now
executes the complete headless board.

`g15.051` is complete in PR #67. It removed GPUI's normal GPL tracing graph and
made exact Git-source policy fail closed. That bounded fork was later rejected
as a public dependency after Longhorn exposed its source-identity cost;
`g15.059` restored crates.io GPUI without reopening component work.

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
npm in place, before gates, packing, or publication. `g15.054` recovered the
release as `v0.2.1` from candidate `3d914261`; run `32658293188` completed the
gate, pack, publication, and artifact upload. Longhorn then proved that the tag
leaked a fork-sourced GPUI identity into downstream Rust graphs. The broken Git
tag was retracted after replacement; npm retains 0.2.1 for install stability.

`g15.059`–`061` produced and published corrected candidate `d5607def` as
`v0.2.2`. Run `32756610293` published core and Svelte to npm `latest`, retained
React as source-only, and passed clean public-registry installation. Cards
`055`–`079` then moved all 16 authoritative consumers to the declared release
and their own repository-owned evidence boundaries. Loophole Legacy `078` was
cancelled because the repository was removed; it is not a completion gap.
Jetstream `077` closed the final active lane. The generation is complete.
