# 025 Audio Cross-runtime Parity

Status: complete
Owner: Poodle core
Created: 2026-08-10
Depends on: `docs/architecture/008-audio-control-family.md`,
`docs/roadmaps/g12/024-audio-control-follow-ons.md`

## Boundary Change

The maintainer expanded the completed Svelte audio family to require full
Svelte, React, GPUI, and Jetstream coverage. Card g12.024 remains the
historical Phase 1/2 delivery record; this card owns the new runtime-parity
baseline. This work was briefly filed as `g13.001`; it is consolidated into
g12 because it continued the same active delivery program.

Asset skins, host parameter binding, and Phase 3 components remain out of
scope.

## Contract Repair

- [x] Replace Svelte-only notes and deltas in all nine audio contracts.
- [x] Add normative React, GPUI, and Jetstream notes.
- [x] Add named specimen definitions and runtime parity checklists.

## React

- [x] Port all nine thin shells over `poodle-core` machines and shared styles.
- [x] Preserve the VisualState-only renderer seam.
- [x] Add standalone specimen pages, exports, interaction/a11y tests, and
  parity-report coverage.

## Native Shared Layer

- [x] Add Rust audio laws, formatting, machines/feed integration, and golden
  tests to `poodle-headless`.
- [x] Add renderer-neutral specs and serializable VisualState equivalents to
  `poodle-specs`.
- [x] Add token-themed VisualState-only node builders to `poodle-render`.

## GPUI And Jetstream

- [x] Register every audio component in both catalogues; keep the direct
  adapter manifests clean while the family uses the shared node renderer.
- [x] Add full standalone specimen pages in both previews.
- [x] Route interaction, accessibility, and hit testing through adapters or
  host state rather than draw functions.
- [x] Add native render, adapter, accessibility, and preview coverage.

## Acceptance

- [x] Every audio component has one canonical contract and four standalone
  specimen pages.
- [x] Every renderer consumes a serializable VisualState without machine-state
  reads.
- [x] Web and native law/format/ballistics golden values agree.
- [x] Audio-scoped React, GPUI, Jetstream, parity, accessibility, docs, and
  drift gates pass. Repository-wide exceptions are recorded below.
- [x] Runtime-specific visual or accessibility limits are named in contracts;
  no preview silently substitutes a placeholder.
- [x] Every component supports inherited or explicit `xs`–`xl` size and
  compact/default/comfortable density across Svelte, React, GPUI, and
  Jetstream, with full matrices on every specimen page.

## Validation Exceptions

- The deterministic Jetstream gate is exact for all nine updated pages. The
  focused GPUI refresh captured the first three pages, then macOS denied three
  consecutive window screenshots and the gate stopped as designed. GPUI
  build, shared-render tests, and semantic coverage are green; rerun the nine
  focused captures in an unlocked Screen Recording session. The older global
  GPUI sweep also retains its roughly `0.34%` raster offset across 135 older
  specimens.
- The global Svelte surface audit reaches 158/159 exports; only the unrelated
  `AgentSubagent` usage-doc gap remains.
- Direct React `tsc` reaches only the three pre-existing `AgentChatStatus`
  errors. Vite build, family tests, axe, and anatomy parity are green.
