# g14 Conformance Estate

Status: active baseline
Measured: 2026-08-14
Owner: Poodle orchestrator

## Problem Baseline

Poodle has no component-level gate that proves the same portable interface,
fixture, behaviour, semantic output, and specimen structure across the active
Svelte, React, and GPUI cohort while preserving a renderer-neutral Rust
boundary.

The previous frozen baseline measured:

- 168 Svelte component files
- 16 GPUI and 17 Jetstream registration gaps; 18-component union
- 21 name-mapped TS/Rust vector pairs, but only four canonical transition
  machines in both languages
- approximately 86k lines of hand-written specimen code across four runtimes
- 14 drift gates, each covering one projection rather than component
  completion

Those figures are point-in-time evidence, not a completion denominator. The
g14.001 executable roster is now established from source: the Button corpus
(`packages/core/src/conformance/button-cases.ts`) enumerates 19 cases across
four runtimes, executed by `effigy conformance:complete --component button`.

## Current Gates And Holes

| Claim | Current evidence | Silent hole |
| --- | --- | --- |
| Svelte props match contracts | `docs:contract-drift` | React, Rust, composition regions, portable methods |
| value domains align | `docs:value-domain-drift` | report-only by default; currently 19 disagreements and 10 unresolved types under enforcement |
| callbacks match contracts | `docs:callback-drift` | React and native event shape/timing |
| Rust specs match contracts | `docs:spec-drift` | behaviour, defaults interpreted by renderer, web implementation |
| native roles appear | `drift:roles` | placement, state, accessible name, GPUI evidence |
| native has a handler | `drift:events` / `drift:handlers` | correct handler, payload, timing, actual backend action |
| machine names have vectors | `docs:machine-shape-drift` | vector depth and equal output; currently fails after generated imports |
| React specimen registered | `docs:react-specimen-drift` | specimen content and GPUI |
| capability declaration has a trace | `docs:capability-drift` | undeclared capability vocabulary and actual parity |
| visual baselines exist | web and native snapshot tools | shared fixture identity; stale GPUI captures; Jetstream overwrite workflow |
| **Button completes the conformance kernel** | `conformance:complete --component button` (19 cases × 4 runtimes) | profile pilots 2–6; native token-role projection |

`docs:check` currently stays green while the machine-shape selector is red.
`check:svelte` currently has three `AppHeaderCenterHarness.svelte` Snippet
identity errors. These failures predate the redesigned runway; g14.001
recorded them as baseline (they are untouched by this card).

## Conformance Kernel Status (g14.001)

One behaviour source per runtime pair, pinned by execution (spec 066,
architecture 009). Delivered for Button:

- **Portable interface module** — `packages/core/src/conformance/button.ts`
  (TS authority; Svelte and React import the inferred portable types).
- **Typed case corpus** — `packages/core/src/conformance/button-cases.ts`,
  serialized to neutral JSON (`conformance:build`), consumed by the Rust
  pipeline.
- **Generated Rust declaration** — `packages/contracts/components/src/generated/button.rs`
  replaces the hand-written `ButtonSpec` struct/default/builders; the token
  recipes live in the extension module beside it.
- **Runtime harnesses** — web runners (Svelte + React, real DOM + real
  events), Jetstream runner (real `GameUi` pointer/keyboard dispatch, no
  window), GPUI runner (real backend conversion + node-level dispatch).
- **Observation** — `component-observation.v1` emitted per runtime; exact
  fields compared with local tolerances; per-assertion verdicts
  (`pass`/`fail`/`vacuous`) with a no-vacuous-only coverage rule.
- **Specimens** — all four Button specimen pages are corpus projections;
  hand-written specimen fixtures deleted.
- **Completion** — `effigy conformance:complete --component button` fails on
  missing registration, stale authority, or any divergence (planted-failure
  proofs recorded in the g14.001 batch log).

### Native gaps surfaced by the corpus (recorded debt, not completion)

- `fit`, `truncate`, `max_width` are declared portable but `poodle-render`
  does not consume them (web-only behaviour today). Not covered by required
  Button cases; tracked for `g14.014`.
- Native token-role projection is absent (no `data-*` analogue on nodes);
  token roles are asserted on the web pair, native channels are observed and
  recorded.
- GPUI focus/focus-visible is not observable headlessly (no window); the
  focused-state assertions are covered by the web and Jetstream runtimes.
- Keyboard activation on web: happy-dom implements no browser default
  actions, so the harness performs the browser default (keydown then click);
  Jetstream proves the real keyboard confirm path.

## Experimental Surface Disposition

No experimental surface is architecture merely because it merged.

| Surface | Provisional disposition | Deciding milestone |
| --- | --- | --- |
| RangeSlider native slider role | keep | done |
| Rust component/scene IR shell artifacts | retire or isolate from active component path | g14.001 |
| five generated display specimens | adapt fixture content into cases or retire | g14.001 / g14.009 |
| Rust display specimen component definitions | retire; must not duplicate portable interface authority | g14.001 |
| generated machine interfaces | adapt only if the kernel replaces declarations and fixes its standing gate | g14.001 |
| machine vectors | adapt as case inputs where they prove component behaviour | profile pilots |
| capability registry | adapt as debt/evidence, never completion | g14.001 |
| prop/callback/spec drift scripts | consolidate behind component completion | g14.014 |
| native registration and snapshot tooling | repair and feed completion evidence | g14.002 / g14.014 |
| stale specs 063–065 and old roadmap | archived/retired | done |
| **g14.001 conformance kernel (interface, corpus, harnesses, observers)** | keep — the pilot proof; profile pilots 2–6 reuse it | g14.010 |
| **hand-written ButtonSpec declaration surface** | replaced by `generated/button.rs` + extension module | done |
| **hand-written Button specimen fixtures (4 runtimes)** | replaced by corpus projections | done |
| **generated specimen scenes (specimen-ts/rust targets)** | still the shell/nav surface; Button no longer depends on them | g14.009 |

## Staged Licence Intake

The licence surface enters through a bounded web-reference tranche while the
conformance kernel is still under proof. Web delivery is not completion.

| Component | Web reference | Review | Native/shared cases | Completion state |
| --- | --- | --- | --- | --- |
| `LicenceStatus` | g14.015 landed | g14.016 | g14.017 after adopt | incomplete |
| `LicenceActivation` | g14.015 landed | g14.016 | g14.017 after adopt | incomplete |
| `LicenceSeats` | g14.015 landed | g14.016 | g14.017 after adopt | incomplete |

`LicenceCentre` is an explicit non-goal. The downstream comparison between
Poodle's structural field mirrors and Longhorn's generated field maps remains
Longhorn-owned; g14.015 exports the Poodle side but cannot claim that gate.
Jetstream is program-deferred rather than a per-component known delta.

## Cleanup Rule

Every active claim ends with one canonical gate. A legacy gate may stay while
coverage migrates, but it needs an owner and retirement condition. Generated
<<<<<<< HEAD
artifacts stay out of hand-edited source roots where possible (the poodle-specs
`generated/` module is generated and gated byte-exact). Known generated-source
and god-file health findings are owned by g14.018 rather than normalized as
permanent warnings.
