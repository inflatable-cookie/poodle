# g01.006 GPUI Substrate And Rust Token-Binding Baseline

Status: completed
Owner: Poodle Core
Updated: 2026-03-11
Depends on: g01.002, g01.004
Primary repos: `poodle`

## Research Inputs

- [hub-gpui](../../research/source-hubs/hub-gpui.md)
- [tm-token-system](../../research/translation-memos/tm-token-system.md)

## Context

Poodle's second implementation target is GPUI. It needs the same semantic contract
as Svelte without being forced into browser-shaped abstractions.

## Goals

- [ ] define the GPUI package/subcrate posture
- [ ] define how Rust/GPUI consumers receive token artifacts
- [ ] define parity expectations specific to GPUI-native interaction and layout
- [ ] define how GPUI components expose contract-aligned interfaces
- [ ] ensure the GPUI substrate does not become the canonical shape for all
  future desktop-oriented implementations

## Non-Goals

- [ ] no full GPUI component implementation yet
- [ ] no app-specific workstation widgets yet

## Execution Checklist

- [ ] define crate ownership and boundaries for GPUI-facing packages
- [ ] define token-ingestion rules for Rust from generated artifacts rather than
  hand-maintained constants
- [ ] define the first Rust module shape for semantic token families
- [ ] define the first named-theme realization for GPUI Theme structs or
  equivalent helpers
- [ ] define GPUI-specific implementation notes fields in the contract template
- [ ] define where native delta is allowed and how it must be documented
- [ ] define which interaction and layout expectations are parity-critical
- [ ] define how GPUI theme access should work in component render paths
- [ ] define how GPUI-native layout/styling idioms can remain idiomatic without
  becoming canonical contract requirements
- [ ] document which GPUI-specific conveniences must remain outside the
  canonical contract so other desktop implementations stay possible

## Acceptance Criteria

- [ ] GPUI substrate posture is explicit
- [ ] Rust token-binding expectations are explicit
- [ ] GPUI parity policy is explicit enough to guide primitives
- [ ] one-theme-to-many-runtime translation is preserved on the GPUI side
- [ ] GPUI-specific implementation choices are prevented from becoming
  canonical desktop-contract requirements

## Deliverables

- [ ] GPUI substrate policy
- [ ] Rust token-binding baseline

## Evidence Requirements

- [ ] one example token family mapped into a Rust-facing form
- [ ] one example named theme mapped into a GPUI-facing theme/module form

## Next Task

Open `g01.007` and begin the actual shared primitive surface with layout,
surface, and scrolling primitives.
