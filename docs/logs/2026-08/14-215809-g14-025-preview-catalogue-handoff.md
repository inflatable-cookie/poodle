---
title: Preview catalogue taxonomy handoff
status: active
owner: Poodle previews
updated: 2026-08-14
tags: [coordination, handoff, preview, catalogue]
---

## What This Thread Was Doing

The orchestrator reviewed the component navigation used by all four Poodle
previews after the catalogue grew to 174 entries. It found that one anatomy
`tag` was serving as both classification and navigation, splitting related
agent, model, audio, and lifecycle suites across unrelated headings. It froze
a replacement taxonomy and wrote one complete worker roadmap.

## Why It Matters

The current sidebar makes discovery harder as Poodle grows, and its duplicated
Rust registries allow navigation metadata to drift between runtimes. New
components should enter one predictable catalogue authority and appear in an
obvious family everywhere without four manual category edits.

## Current State

- Done so far: the repository and preview registries were audited; the
  section/family/kind model, family order, motivating assignments, active
  runtime cohort, navigation behavior, generator boundary, and validation
  rules are fixed in g14.025.
- Still open: neutral manifest, complete classification, codegen, runtime
  overlays, Svelte/React/GPUI navigation, generated Jetstream metadata, tests,
  screenshots, selectors, and implementation log.
- Active spec lane: none. This is preview tooling and information architecture,
  not a public component contract or component-spec change.
- Canonical refs:
  - `/Users/tom/Dev/projects/poodle/docs/architecture/003-component-docs-ia-and-implementation-substrates.md`
  - `/Users/tom/Dev/projects/poodle/docs/specs/025-parity-automation-and-harness-boundary.md`
  - `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/README.md`
- Remaining continuation envelope: exactly g14.025 in one dedicated worktree
  and PR.
- Lane budget / pause signal: finish the roadmap, open the PR, then stop for
  orchestrator review. Do not begin specimen, public-component, curated
  collection, or Jetstream-shell follow-up work.
- Key files:
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/025-preview-catalogue-taxonomy-and-generated-navigation.md`
  - `/Users/tom/Dev/projects/poodle/packages/svelte/preview/src/component-registry.ts`
  - `/Users/tom/Dev/projects/poodle/packages/react/preview/src/gallery/registry.ts`
  - `/Users/tom/Dev/projects/poodle/packages/gpui/preview/src/component_registry.rs`
  - `/Users/tom/Dev/projects/poodle/packages/jetstream/preview/src/component_registry.rs`

## Boundaries

- Execute the complete writable scope in g14.025 and obey its stop conditions.
- Do not change component APIs, behavior, contracts, specimens, recipes,
  stable routes, conformance, native backends, or release workflows.
- Svelte, React, and GPUI are the active navigation cohort. Generate
  Jetstream's metadata, but do not work on its shell or run paired Jetstream
  QA.
- Rebase before inventory and again before final review if another component
  lane changes a registry. Preserve and classify newly landed entries.
- All automated validation is headless. Never run a windowed/native visual
  selector or either native preview application.
- Follow `/Users/tom/Dev/projects/poodle/AGENTS.md`.

## Important Context

- Planning lineage: current Svelte metadata feeds React, while GPUI and
  Jetstream retain large Rust copies. GPUI also defaults unknown slugs to
  `Workstation`; g14.025 removes that silent failure mode.
- Spec-to-canonical relationship: architecture 003 keeps contracts normative
  and previews evidential. The new manifest owns preview discovery metadata
  only; it must not become a second component contract.
- Decisions and preferences: keep related AI-agent and model-management
  components together; favor a simple, attractive catalogue; reuse existing
  shell/components; make future additions predictable; keep Jetstream out of
  the ordinary loop.
- The existing `packages/svelte/preview/src/catalog.ts` taxonomy describes
  documentation/adoption suites. It is not this component catalogue and stays
  separate.
- The old `Workstation` heading was a valid historical preview category. This
  card replaces its overloaded navigation role; it does not restore the
  retired workstation package/spec tier or change product architecture.
- Open tension: some primitives fit more than one use case. The primary family
  must answer where an adopter will look first. Record ambiguous decisions in
  the implementation log instead of adding vague buckets or duplicate sidebar
  entries.

## Suggested Next Move

Open
`/Users/tom/Dev/projects/poodle/docs/roadmaps/g14/025-preview-catalogue-taxonomy-and-generated-navigation.md`
in the fresh worktree. Rebase and inventory current entries, then complete
Batch A through the failing `catalogue:check` gate before changing navigation.
Continue through the card only while its fixed taxonomy and boundaries hold.

## Completion Protocol

1. Complete every accepted g14.025 checkbox or stop with exact evidence.
2. Add one August implementation log with before/after counts, full ambiguity
   register, generator outputs, active-runtime evidence, screenshots, deferred
   Jetstream status, and residual risk.
3. Run the card's headless Effigy validation. Do not run `qa`, paired
   Jetstream, conformance-windowed, native visual, or preview-run selectors.
4. Open one PR from the dedicated worktree. Do not change roadmap or dispatch
   status.
5. Report the PR URL, commits, generated files, classification count,
   failed/waived checks, screenshots, and any stop-condition question.
6. Stop. The orchestrator reviews classification and preview behavior before
   deciding whether any secondary-collection follow-up exists.
