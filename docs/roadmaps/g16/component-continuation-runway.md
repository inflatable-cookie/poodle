# g16 Component Continuation Runway

Status: canonical post-triage planning map
Compiled: 2026-09-01
Source head: `35afa1e215eefd49a09b890e2b5642d79923f981`
Sources: `component-continuation-register.md` and the accepted 2026-09-01
triage packets

This map replaces the stale post-motion queue with one bounded continuation
frontier. It records promotion, serial edges, external gates, and holds. It
does not change parity evidence, authorize release or sibling-repository
mutation, or create a worker handoff.

## Closed Baseline

- `g16.029`–`g16.035` are complete.
- `g16.036` is complete in PR #127. Its paired-web Tree authority seam keeps
  the existing native synchronous single-row route.
- `g16.037`–`g16.044` are research-complete in PRs #128–#135.
- The accepted decision packets resolve the previous EditableLabel,
  block-slider, toast, shimmer, icon-geometry, design-guidance, release,
  citations, nested-menu, lab, visual-tranche, and Jetstream questions.

## Ready Frontier

These cards have complete boundaries and no unmet serial dependency:

| Card | Lane | Writable boundary | Result |
| --- | --- | --- | --- |
| [`g16.045`](045-editable-label-editing-model-and-mounted-parity.md) | EditableLabel | contract, paired runtime implementation, mounted proof | one committed/draft editing model |
| [`g16.046`](046-block-slider-and-range-slider-appearance.md) | Slider / RangeSlider | additive appearance contract and active-runtime implementation | opt-in horizontal block treatment |
| [`g16.047`](047-toast-same-id-update-convention.md) | ToastHost / ToastStack | existing host update path, timer, announcement, focus, native role | consumer-owned same-id settlement convention |
| [`g16.048`](048-agent-subagent-ownership-and-shimmer-benchmark.md) | AgentSubagent | contract reconciliation, then disposable web benchmark | threshold verdict; no shipped shimmer |
| [`g16.049`](049-icon-geometry-format-and-registry-foundation.md) | icon geometry IG-01/02 | internal architecture, format, vectors, generated registry | validated internal geometry foundation |
| [`g16.052`](052-contributor-design-guidance-pilot.md) | contributor guidance | finite Svelte-only matched pilot | promotion or rejection verdict |
| [`g16.053`](053-repository-security-audit-boundary-repair.md) | repository audit | existing OpenAI-key matcher and focused tests | green, non-waived security audit input |

The lanes are independently dispatchable after this planning PR is accepted.
If implementation discovers shared barrel, registry, contract, or generated
file ownership between cards, serialize the overlapping pair before edits.
Global g16 front-door and closeout edits remain orchestrator-owned.

`g16.048` contains one internal phase gate: reconcile the stale contract with
the live Svelte, React, shared Rust, and GPUI ownership before running the
disposable browser benchmark. A threshold pass can justify a later bounded
implementation card; it cannot ship production shimmer itself.

## Serial Frontier

| Card or continuation | State | Opens when | Fixed boundary |
| --- | --- | --- | --- |
| [`g16.050`](050-icon-geometry-internal-runtime-substrate.md) | blocked | accepted `g16.049` | IG-03–IG-05; internal plan, node/GPUI headless proof, private web shells |
| [`g16.051`](051-icon-geometry-native-visual-admission.md) | blocked | accepted `g16.050` plus an operational, separately authorized lab | IG-06 native visual admission only |
| [`g16.054`](054-historycenter-v030-release-candidate.md) | blocked | accepted `g16.053` | immutable `0.3.0` candidate and certification; no release mutation |
| AgentSubagent implementation | gated | `g16.048` meets every threshold and a new bounded card is accepted | web-only finite sweep; no generic effect API |
| IconMorph public admission | gated | `g16.051` returns an admit verdict | IG-07 requires a later explicit public card |
| HistoryCenter publication and Loophole adoption | gated | accepted `g16.054`, exact receipts, and separate orchestrator authority | tag/publish and sibling adoption stay distinct mutations |

## Programme Gates And Holds

| Programme | State | Missing authority or proof | Next planning event |
| --- | --- | --- | --- |
| nested menu pointer intent | gated | recursive React/Rust/GPUI substrate plus resolved focus-contract contradiction | compile only after both prerequisites are explicit; retain conditional pointer corridor and synchronized close grace |
| agent citations | gated | authored rich semantic content carrier and paragraph-level consumer evidence | keep composition-owned; compile the carrier prerequisite separately |
| dedicated conformance lab | external hold | separate repository authority and operator-approved capture environment | bootstrap outside this Poodle runway |
| six-component / 24-fixture visual tranche | blocked | operational dedicated lab | preserve the accepted fixture set; do not substitute local windowed runs |
| GPUI accessibility | programme hold | explicit programme choice and accepted manual/runtime evidence boundary | no component-local card |
| Jetstream | hold | backend admission remains outside the active cohort | no implementation card |

The nested-menu direction remains conditional pointer intent: immediate sibling
transfer only inside a valid corridor and accepted synchronized close grace.
The citation direction remains composition-gated: paragraph proof first, with
the full semantic content carrier as a separate serial prerequisite.

## Promotion Rule

A continuation becomes ready only when its authority, writable scope,
acceptance, validation, evidence movement, stop conditions, and dependencies
are explicit. A missing ledger cell alone is not a defect. External lab,
release, adoption, and backend work never inherit authority from a planning
card.
