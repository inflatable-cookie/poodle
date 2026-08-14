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

Those figures are point-in-time evidence, not a completion denominator. New
g14.001 establishes the executable roster from source.

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

`docs:check` currently stays green while the machine-shape selector is red.
`check:svelte` currently has three `AppHeaderCenterHarness.svelte` Snippet
identity errors. These failures predate the redesigned runway; g14.001 must
either fix or explicitly route them before claiming a clean baseline.

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
artifacts stay out of hand-edited source roots where possible. Known
generated-source and god-file health findings are owned by g14.018 rather than
normalized as permanent warnings.
