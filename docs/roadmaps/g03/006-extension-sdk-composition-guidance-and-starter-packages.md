# g03.006 Extension SDK, Composition Guidance, And Starter Packages

Status: completed
Owner: Poodle Core
Updated: 2026-03-12
Depends on: g03.001, g03.003
Primary repos: `poodle`

## Goals

- [x] define extension-facing documentation and package posture
- [x] define starter guidance for downstream consumers
- [x] define how app-specific systems should compose from Poodle safely

## Execution Checklist

- [x] define extension-facing docs and package posture
- [x] define starter guidance for downstream consumers
- [x] define safe composition rules for app-specific systems

## Completed Work

- froze the normative baseline in `docs/specs/039-extension-sdk-composition-guidance-and-starter-package-baseline.md`
- defined the current extension SDK as package APIs, contracts, bridge adapters, and documentation rather than a new framework layer
- defined the allowed starter-package shapes for direct Svelte, bridge-mediated, and workstation-oriented consumers
- made wrapper, adapter, branding, and host-integration ownership explicit so downstream repos do not infer their own boundary from preview-only examples

## Acceptance Criteria

- [x] extension guidance is explicit
- [x] starter-package posture is explicit

## Next Task

Open `g03.007` and harden the Underlay bridge with a zero-leak adoption proof,
using the new extension-SDK and starter-package baseline rather than inventing
bridge posture ad hoc.
