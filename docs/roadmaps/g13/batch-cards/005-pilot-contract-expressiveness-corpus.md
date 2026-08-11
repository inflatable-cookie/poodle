# 005 Pilot Contract Expressiveness Corpus

Status: merged
Milestone: `g13.002` research precursor
Owner: Poodle core
Branch: `thread/g13-pilot-expressiveness-corpus` (commit `2f8dc5db`, merged
`bb3f79ef`)
Review: `docs/logs/2026-08/11-g13-b001-b005-review-and-merge.md`
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
(`IR-02`–`IR-11`), `docs/contracts/components/button.md`,
`docs/contracts/components/range-slider.md`,
`docs/contracts/components/text-input.md`

## Goal

Extract the complete semantic vocabulary the pilot IR must express. Produce a
contract-derived stress corpus, not an IR schema or implementation design.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents or parallel research tasks. Read sources directly.
- Component contracts are semantic authority. Current implementation and tests
  provide evidence and may reveal contradictions, but do not silently override
  contracts.
- Do not edit contracts, specs, architecture, roadmaps, dispatch state,
  implementation, tests, registries, generated artifacts, or manifests.
- Write only the corpus, batch log, and valid PAPERCUTS entries.
- Do not propose Rust types, JSON shapes, macros, compiler APIs, or crate
  placement.
- Commit and push the worker branch. Do not merge.

## Scope

### In scope

- `docs/roadmaps/g13/pilot-expressiveness-corpus.md`
- `docs/logs/2026-08/11-g13-pilot-expressiveness-corpus.md`
- Button, RangeSlider, and TextInput contracts plus their current Svelte,
  React, core-machine/style, Rust spec/headless/render, GPUI, Jetstream,
  specimen, interaction, accessibility, recipe, and visual evidence.
- Current Svelte/React/GPUI/Jetstream preview shell controls for theme, size,
  density, contrast, navigation, search, and specimen tabs.

### Out of scope

- Repository-wide authority inventory and docs-baseline repair owned by
  `g13-b001`.
- Any schema, codegen, component, test, specimen, or preview change.
- Deciding whether an implementation/contract contradiction changes the
  contract; report it as a stop finding.

## Steps

1. Define stable requirement IDs: `CROSS-*`, `BTN-*`, `RNG-*`, `TXT-*`, and
   `SHELL-*`.
2. Extract every required prop/default/type, controlled state, event/effect,
   slot/content rule, anatomy part, conditional/repeated node, state-derived
   attribute, keyboard command, accessibility fact, token/recipe reference,
   size/density/orientation axis, and VisualState field.
3. Classify each requirement without designing representation:
   - shared declarative definition
   - generated target artifact
   - adapter capability
   - conformance vector
   - candidate explicit runtime extension
4. For every requirement, cite the contract and at least one current evidence
   path. Name missing evidence rather than inferring it.
5. Add negative cases: behavior that must not move into drawing, generated
   lifecycle, `poodle-node` web authoring, or arbitrary Rust execution.
6. Add contradiction and unknown registers. Stop on a semantic contradiction;
   continue on missing non-authoritative evidence.
7. Record exact counts by component/category and validation exit states.

## Acceptance Criteria

- [x] The corpus covers all three pilot components and all four preview shells.
- [x] Every requirement has a stable ID, classification, contract citation,
  and evidence path or explicit evidence gap.
- [x] Controlled state, environment capabilities, accessibility, recipes,
  axes, VisualState, and event timing are not collapsed into generic rows.
- [x] Negative cases directly exercise IR-03–IR-06.
- [x] No representation/schema recommendation appears.
- [x] `git diff --check` passes and the batch log records command exit states
  and requirement counts.

## Stop Conditions

- A component contract and live implementation disagree on public semantics,
  event timing, accessibility, or runtime ownership.
- A required pilot contract is missing or structurally incomplete.
- Completing the corpus requires inventing a schema or changing code.

Stop with requirement IDs, exact paths, and the smallest unresolved semantic
question.

## Promotion Target

The reviewed corpus becomes the acceptance input for the g13.002 schema card;
it does not become architecture by itself.
