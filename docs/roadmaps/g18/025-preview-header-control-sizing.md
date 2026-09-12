# 025 — Preview header control sizing

Status: complete — merged as `90c40defe86e4841ad248c72200f7333f37c342d` (PR #257) on 2026-09-12
Owner: Poodle web previews
Created: 2026-09-12
Governing refs: `../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../../packages/svelte/preview/src/components/DisplayControls.svelte`,
`../../../packages/react/preview/src/gallery/DisplayControls.tsx`
Depends on: none

## Outcome

Make every control in the Svelte and React preview headers use the same selected
chrome size. Theme, Density, Size, Contrast, and Search align as one row; the
specimen Size axis resizes the full header control set together and defaults to
`sm`.

This preview-only repair must close before retained g18.006 resumes so final
acceptance uses the corrected shell.

## Ready-State Rubric

- [x] Both previews render the same five generated shell controls.
- [x] The operator observed mismatched control heights in the current header.
- [x] The header leaves sizing implicit while also controlling specimen size.
- [x] The operator clarified that all five controls follow the Size selection,
  with `sm` as the initial selection.

## Decisions

- Header chrome follows the Poodle `controlSize` selection; `sm` is the shell
  default, not a permanent override.
- Use one coherent size context or existing public size inputs. Do not add
  preview-only CSS heights that bypass component sizing contracts.
- ThemeSelect, both ToggleGroups, Slider, and TextInput expose the same selected
  visual control height and align on their top/bottom edges beneath equal labels.
- **Operator correction, 2026-09-12:** ToggleGroup follows the shared control
  height directly. Consistent component sizing belongs to the reusable
  contract; no preview-local height exception remains.
- Keep current values, generated-shell ownership, wrapping, responsive search
  growth, keyboard behavior, and accessibility unchanged.
- Mirror the result in Svelte and React. Do not change reusable component APIs.

## UI Design Brief

- **Class:** small shell refinement.
- **Mode:** Operate. One quiet, compact control strip.
- **Geometry:** all five controls use the selected shared size ladder;
  labels share a baseline and controls share top/bottom edges.
- **Behavior:** the Size selector changes the catalogue specimens and all five
  header controls together. Theme and density changes do not alter that size.
- **Responsive:** retain wrapping and the flexible Search group without
  stretching control height.

## Dispatch manifest

- **State:** complete; merged g18.025 alignment repair, then restored selected-size response; serial before
  retained g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; merged by the plugin, never by the worker
- **Owned mutable paths:** paired DisplayControls implementations; shared
  ToggleGroup sizing contract/style; focused paired preview tests; one g18.025
  execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch/task state;
  reusable component implementations; g18.006/g18.009; versions, changelog,
  workflows, release/tag, Desktop
- **Worker:** routine paired-web UI worker
- **Excluded:** component API changes; generated shell vocabulary changes;
  broader header redesign; release mutations
- **Escalation:** resolved 2026-09-12. ToggleGroup uses the shared ladder;
  no paired-header exception is required.

## Work

1. Plant paired header geometry checks at every specimen Size selection.
2. Bind the complete header control surface to the selected size presentation.
3. Prove all five visual controls match each selected ladder stop and align in
   both previews.
4. Prove Size, Density, Theme, Contrast, Search, wrapping, and keyboard behavior
   remain functional.
5. Run focused paired preview tests, Chromium/WebKit geometry checks, preview
   builds, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Header controls share one size | one control retains its private/default height | equal visual boxes for all five controls at each size in both previews |
| Header responds to Size | selecting `xs` or `xl` leaves the controls unchanged | measured xs–xl ladder geometry across all five choices |
| Alignment is visual | wrappers align while painted controls do not | top/bottom edge comparison of rendered controls |
| Components remain authoritative | preview CSS forces arbitrary heights | size-context/prop inspection with shared component sizing |
| Behavior survives | normalization breaks a selector or search | paired interaction and keyboard checks |

## Stop conditions

- Stop if consistency requires a new reusable-component API.
- Stop before editing reusable component internals, generated shell semantics,
  release state, versions, workflows, or Desktop.

## Evidence

Inspection on 2026-09-12 found both DisplayControls implementations render the
same ThemeSelect, ToggleGroup, Slider, and TextInput surfaces without binding
one coherent header size context. The operator screenshot shows
Theme/Search/Contrast and Density/Size resolving to different visible heights.

PR #257 proved the alignment scope but exposed a pre-existing contract gap:
ToggleGroup painted each item 0.25rem shorter than its resolved size. The
operator clarified that the public size ladder must be consistent across
components, so ToggleGroup now uses the full resolved control height and the
preview-local exception is removed.

## Next task

After g18.025 closes, accept the corrected preview experience and resume
retained g18.006. g18.009 remains serial after it.
