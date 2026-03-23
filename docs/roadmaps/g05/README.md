# g05 GPUI Foundation, Spec Crates, Cross-Runtime Parity Baseline, And Demo Alignment

Status: completed
Updated: 2026-03-14

## Context

`g05` established the GPUI-native foundation for Flint: contract audit, theme
runtime, spec crates for primitives, composites, and workstation layers,
accessibility proof, cross-runtime parity evidence, and demo-app alignment. This
work landed the renderer-agnostic spec types and token consumption layer that
GPUI (and future rendering targets) build on.

The original g05 scope included GPUI parity for the expanded g04 component
surface, demo-app parity, downstream adoption proof, and docs promotion. That
work has been deferred to g06, which will redesign the shared Rust contract
layer to support multiple rendering targets (GPUI, Jetstream, and future
backends) before building out renderer-specific implementations.

## Starting State

- `g04` Svelte component surface is complete (Underlay parity, ~25 new
  components, significant feature extensions)
- GPUI had no implementation presence in Flint prior to this generation

## Exit State

- GPUI contract audit, theme runtime, and native preview app baseline are
  established
- GPUI spec crates cover structural, action, text-entry, selection, overlay,
  disclosure, and navigation primitives
- GPUI composite spec crates cover form, data, browse, detail, picker, media,
  and workstation layers
- native accessibility proof and cross-runtime parity evidence are explicit
- shared demo-app contract and section model are defined
- Svelte demo-app rebuild and coverage upgrade are complete
- spec crates provide renderer-agnostic types (enums, builder-pattern structs,
  token accessor methods) that carry no GPUI rendering dependency

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | GPUI contract audit, parity priority matrix, and implementation order | g03.014 | Foundation | Completed |
| 002 | GPUI theme runtime, token application, and native preview app baseline | 001 | Foundation | Completed |
| 003 | GPUI layout, surface, scrolling, and structural primitives | 001, 002 | Core build | Completed |
| 004 | GPUI action, text-entry, and field primitives | 001-003 | Core build | Completed |
| 005 | GPUI selection, value, feedback, and date or time primitives | 001-004 | Core build | Completed |
| 006 | GPUI overlay, disclosure, navigation, and menu primitives | 001-005 | Core build | Completed |
| 007 | GPUI form, validation, and remediation composite parity | 004-006 | Depth | Completed |
| 008 | GPUI data, browse, detail, picker, and media composite parity | 003-007 | Depth | Completed |
| 009 | GPUI workstation shell, command discovery, and layout orchestration parity | 003, 006, 008 | Workstation | Completed |
| 010 | GPUI native accessibility, focus, keyboard, and assistive-technology proof | 003-009 | Hardening | Completed |
| 011 | Cross-runtime parity report, intentional delta register, and acceptance-harness expansion | 007-010 | Hardening | Completed |
| 012 | Shared demo-app audit, gap register, and target-shape freeze | 009-011 | Alignment | Completed |
| 013 | Cross-runtime demo-app contract, section model, and parity checklist | 012 | Alignment | Completed |
| 014 | Svelte demo-app rebuild, component adoption, and coverage upgrade | 012, 013 | Alignment | Completed |

## Dependency Shape

```text
g03.014
  -> 001 Contract Audit
      -> 002 Theme Runtime
          -> 003 Structural Primitives
              -> 004 Action / Text-Entry Primitives
                  -> 005 Selection / Value / Feedback Primitives
                      -> 006 Overlay / Disclosure / Navigation Primitives
                          -> 007 Form Composites
                              -> 008 Data / Browse Composites
                                  -> 009 Workstation Shell
                                      -> 010 Accessibility Proof
                                          -> 011 Parity Report
                                              -> 012 Demo Audit
                                                  -> 013 Demo Contract
                                                      -> 014 Svelte Demo Rebuild
```

## Deferred Work

The following work was originally scoped for g05 but has been deferred to g06
to allow a clean redesign of the shared Rust contract layer with multi-renderer
support (GPUI, Jetstream, and future backends):

- GPUI parity for g04-added components (dialog, file-upload, temporal, code,
  card, navigation, list-interaction, media, editing, operational)
- GPUI feature-extension parity for existing component depth
- expanded cross-runtime parity report
- GPUI demo-app parity implementation
- GPUI downstream reference-app proof
- published docs platform and evaluator onboarding
