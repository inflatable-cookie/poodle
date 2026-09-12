# 025 — Preview header control sizing

Status: Queue revision — preview-local ToggleGroup alignment authorized 2026-09-12
Owner: Poodle web previews
Created: 2026-09-12
Governing refs: `../../architecture/003-component-docs-ia-and-implementation-substrates.md`,
`../../../packages/svelte/preview/src/components/DisplayControls.svelte`,
`../../../packages/react/preview/src/gallery/DisplayControls.tsx`
Depends on: none

## Outcome

Make every control in the Svelte and React preview headers use the same fixed
`md` chrome size. Theme, Density, Size, Contrast, and Search align as one row;
changing the specimen Size axis never resizes the header used to control it.

This preview-only repair must close before retained g18.006 resumes so final
acceptance uses the corrected shell.

## Ready-State Rubric

- [x] Both previews render the same five generated shell controls.
- [x] The operator observed mismatched control heights in the current header.
- [x] The header leaves sizing implicit while also controlling specimen size.
- [x] The operator selected one fixed `md` chrome size for all five controls.

## Decisions

- Header chrome is fixed at Poodle `md`; it does not inherit the specimen
  `controlSize` selection.
- Use one coherent size context or existing public size inputs. Do not add
  preview-only CSS heights that bypass component sizing contracts, except for
  the narrow paired-header treatment below.
- ThemeSelect, both ToggleGroups, Slider, and TextInput expose equal 36px visual
  control height and align on their top/bottom edges beneath equal labels.
- **Chatterbox ruling, 2026-09-12:** preserve ToggleGroup's reusable 0.25rem
  item inset globally. Within the two generated preview headers only, neutralize
  that inset so Density and Size paint at the same 36px `md` height as the other
  header controls. This paired local treatment is an explicit exception to the
  no-preview-height rule; it must not change ToggleGroup's public contract or
  leak into catalogue specimens.
- Keep current values, generated-shell ownership, wrapping, responsive search
  growth, keyboard behavior, and accessibility unchanged.
- Mirror the result in Svelte and React. Do not change reusable component APIs.

## UI Design Brief

- **Class:** small shell refinement.
- **Mode:** Operate. One quiet, compact control strip.
- **Geometry:** all five controls use the shared `md` 36px control height;
  labels share a baseline and controls share top/bottom edges.
- **Behavior:** the Size selector changes catalogue specimens only. The header
  remains stable through `xs`–`xl` selections and theme/density changes.
- **Responsive:** retain wrapping and the flexible Search group without
  stretching control height.

## Dispatch manifest

- **State:** ready; serial before retained g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** paired DisplayControls implementations; React
  gallery CSS and Svelte local DisplayControls styles; focused paired preview
  tests; one g18.025 execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch/task state;
  reusable component implementations; g18.006/g18.009; versions, changelog,
  workflows, release/tag, Desktop
- **Worker:** routine paired-web UI worker
- **Excluded:** component API changes; generated shell vocabulary changes;
  broader header redesign; release mutations
- **Escalation:** resolved 2026-09-12. ToggleGroup keeps its reusable inset;
  g18.025 may apply the narrow paired-header exception above.

## Work

1. Plant paired header geometry checks at every specimen Size selection.
2. Bind the complete header control surface to fixed `md` presentation. Apply
   the authorized paired-header ToggleGroup inset neutralization locally.
3. Prove all five visual controls measure 36px and align in both previews.
4. Prove Size, Density, Theme, Contrast, Search, wrapping, and keyboard behavior
   remain functional.
5. Run focused paired preview tests, Chromium/WebKit geometry checks, preview
   builds, docs QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Header controls share one size | one control retains its private/default height | measured 36px visual boxes for all five controls in both previews |
| Header is stable | selecting `xs` or `xl` resizes the controls | stable header rectangles across all five specimen size choices |
| Alignment is visual | wrappers align while painted controls do not | top/bottom edge comparison of rendered controls |
| Components remain authoritative | preview CSS forces arbitrary heights | size-context/prop inspection with no component-internal overrides |
| Behavior survives | normalization breaks a selector or search | paired interaction and keyboard checks |

## Stop conditions

- Stop if consistency requires a new reusable-component API.
- Stop before editing reusable component internals, generated shell semantics,
  release state, versions, workflows, or Desktop.

## Evidence

Inspection on 2026-09-12 found both DisplayControls implementations render the
same ThemeSelect, ToggleGroup, Slider, and TextInput surfaces without binding
one fixed header size. The operator screenshot shows Theme/Search/Contrast and
Density/Size resolving to different visible heights.

PR #257 proved the fixed `md` scope but exposed a pre-existing contract gap:
ToggleGroup deliberately paints each item 0.25rem shorter than its resolved
size. Accepting that inset would leave the reported mismatch intact; changing
the reusable component would broaden a preview-shell repair across every
consumer. Chatterbox therefore authorized the smallest paired preview-local
alignment treatment.

## Next task

After g18.025 closes, accept the corrected preview experience and resume
retained g18.006. g18.009 remains serial after it.
