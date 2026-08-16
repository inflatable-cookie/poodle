# g14 Conformance Estate

Status: retained-estate ledger — pilot rejected by `g14.008`, removed by `g14.021`
Updated: 2026-08-16
Owner: Poodle orchestrator

## What this is

The g14 executable conformance mechanism is rejected and gone. This page is
the short ledger of what survived it and where each thing lives now.

Authoritative documents:

- verdict and rule-by-rule evidence: `008-pilot-verdict.md`
- defect ledger, deleted inventory, retained selectors:
  `../../logs/2026-08/16-g14-021-experimental-cleanup-and-gate-consolidation.md`
- rejected design records, kept for traceability:
  `../../architecture/009-cross-runtime-component-conformance.md`,
  `../../specs/066-executable-component-conformance.md`
- the pilot's own delivery logs, `g14.001`–`g14.007`, unchanged

A provisional `keep` in an old log is not standing authority. Nothing below
claims cross-runtime parity.

## Retained estate

| Surface | Where it lives |
| --- | --- |
| Component, renderer, node, focus, input, overlay, and accessibility fixes | shipped; unchanged by the cleanup |
| Headless GPUI test platform — in-memory, no window, no focus theft | `packages/gpui/preview/src/headless_driver.rs` |
| Native regressions only a mounted window can prove | `packages/gpui/preview/tests/headless_regressions.rs`, via `effigy regressions:native` |
| Every product/backend defect the pilot caught | named owners in the g14.021 log's defect ledger |
| Hand-written Rust declarations | `poodle_specs::{ButtonSpec, PopoverSpec, TextInputSpec, HistoryCenterSpec, TabsSpec, RangeSliderSpec}` |
| Curated Svelte, React, and GPUI specimens | unchanged; audited under `g14.026` |
| Native visual compare/refresh and `--control-size` | `effigy test:native-visual` |

## Removed estate

Portable interface modules, typed case corpora, generated interface/case/roster
JSON, generated Rust declarations, the conformance codegen parser and Rust
target, Svelte/React corpus hosts and adapters, GPUI corpus adapters and
fixture support, the normalized observation comparator and its manual component
registries, the primitive capability roster/probes/report, the corpus
projection, the cost script, the planted-failure suite, and every
`conformance:*` selector except the workflow-compatible `ci:conformance` alias.

Full path inventory is in the g14.021 log.

## Standing gates

No gate claims component completion across runtimes. What exists:

| Claim | Gate |
| --- | --- |
| Svelte props match contracts | `docs:contract-drift` |
| value domains align | `docs:value-domain-drift` (report-only by default) |
| callbacks match contracts | `docs:callback-drift` |
| Rust specs match contracts | `docs:spec-drift` |
| native roles appear | `drift:roles` |
| native has a handler | `drift:events` / `drift:handlers` |
| machine names have vectors | `docs:machine-shape-drift` |
| React specimen registered | `docs:react-specimen-drift` |
| capability declarations trace to source | `docs:capability-drift` |
| visual baselines exist | `test:native-visual`, web snapshot tools |
| component behaviour, per runtime | `check:svelte`, `react:build`, the component test boards, `cargo test -p poodle-render`, `regressions:native` |

Each still covers one projection. That is the honest state: the hole
`g14.001`'s problem baseline described is open, and closing it needs a
different architecture than the one that was tried.

`ci:conformance` is a legacy alias for `regressions:native`, kept because
`.github/workflows/ci-conformance.yml` calls that name. `g14.022` rules on
whether renaming it is worth the operator approval a workflow edit needs.

## Staged Licence Intake

Web delivery is not completion.

| Component | Web reference | Review | Native completion |
| --- | --- | --- | --- |
| `LicenceStatus` | g14.015 landed | g14.016 | g14.017, replanned after g14.021 |
| `LicenceActivation` | g14.015 landed | g14.016 | g14.017, replanned after g14.021 |
| `LicenceSeats` | g14.015 landed | g14.016 | g14.017, replanned after g14.021 |

`LicenceCentre` is an explicit non-goal. The comparison between Poodle's
structural field mirrors and Longhorn's generated field maps stays
Longhorn-owned. Jetstream is program-deferred.

## Staged Model-Connection Intake

Same posture. Poodle owns presentation and interaction only; Nucleus and
Swallowtail remain external authorities.

| Component | Web reference | Review | Native completion |
| --- | --- | --- | --- |
| `ModelConnectionPicker` | g14.018 | g14.019 | g14.020, replanned after g14.021 |
| `ModelConnectionSetup` | g14.018 | g14.019 | g14.020, replanned after g14.021 |
| `ModelConnectionCard` | g14.018 | g14.019 | g14.020, replanned after g14.021 |
| `ModelCatalogueEditor` | g14.018 | g14.019 | g14.020, replanned after g14.021 |

`ModelPicker` stays the per-thread model/options control. No provider
registry, route fallback, credential authority, provider schema, or
model-default policy enters this intake.

## Cleanup Rule

Every active claim ends with one canonical gate. A legacy gate may stay while
coverage migrates, but it needs an owner and a retirement condition. The g14
pilot is the standing example of what happens when a mechanism accumulates
component-specific registries in five places: one of them silently omitted a
corpus and the completion board went green anyway.
