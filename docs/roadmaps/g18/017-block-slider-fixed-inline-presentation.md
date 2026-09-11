# 017 — Block Slider fixed inline presentation

Status: ready — operator-confirmed cross-runtime visual repair
Owner: Poodle Slider family
Created: 2026-09-11
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/slider.md`,
`../../contracts/components/range-slider.md`,
`../../specs/026-appearance-recipes-and-downstream-override-strategy.md`,
`../../../packages/core/src/slider.ts`,
`../../../packages/core/src/styles/slider.css`,
`../../../packages/core/src/styles/range-slider.css`,
`../../../packages/contracts/headless/src/slider.rs`,
`../../../packages/render/src/slider.rs`,
`../../../packages/render/src/range_slider.rs`
Depends on: none

## Outcome

Correct the opt-in block Slider presentation across active runtimes. Block
Slider and block RangeSlider use rounded-square control corners rather than
pill ends. A single block Slider keeps its visible label and value at stable
logical-edge positions while the thumb moves; text crossed by the selected-fill
boundary changes foreground brightness instead of jumping outside the track.

This is a contract migration for an additive pre-v1 appearance. Preserve value,
pointer, keyboard, accessibility and callback semantics.

## Ready-State Rubric

- [x] Web block Slider and RangeSlider capsules use `999px`; Rust rendering
  resolves `radius.pill` for the same family surface.
- [x] Single Slider places the label inside the selected-width node and value
  inside the remainder-width node.
- [x] The shared fit law depends on the current selected/remainder spans, so
  ordinary thumb motion can eject both strings into an external fallback.
- [x] The operator rejected pill ends and value-dependent external text.
- [x] Fixed duplicated text layers clipped by the selected-fill boundary can
  preserve positions while using selected/remainder foreground roles.
- [x] `radius.control` is the established cross-runtime rounded-square token.
- [x] The task is independent of active editor files and release mutations.

## Decisions

- Block Slider and block RangeSlider capsules resolve `radius.control` in CSS,
  shared Rust rendering and GPUI. The small visible thumbs remain circular.
- Single Slider label stays pinned to logical start and value stays pinned to
  logical end. Their geometry does not depend on the current value.
- Paint one stable text layout through two clipped foreground layers: selected
  text over selected fill and remainder text over remainder fill. Crossing a
  glyph changes only its painted foreground; it does not move, disappear or
  create an external row.
- If existing GPUI/node primitives cannot express that clipped paint without a
  new substrate, the operator-authorized fallback is one combined in-track
  label/value group on the larger side of the thumb, with a fixed midpoint tie
  rule. Apply the same fallback across runtimes and record why the preferred
  split-colour path was unavailable. Never restore the external row.
- Keep the visible thumb above both text layers. Text remains pointer-inert.
- Replace the single-Slider segment-fit fallback with whole-track collision
  handling. If label and numeric value cannot coexist at the available width,
  preserve the value inside the track and suppress the optional visible label;
  `ariaLabel` continues to name the control. Never move either string below or
  beside the component because of thumb position.
- Keep exact numeric value text untruncated. A long optional label may be
  suppressed only by the deterministic whole-track collision rule.
- Direction uses logical start/end and mirrored clipping. The same value and
  label positions must remain stable in LTR and RTL.
- RangeSlider receives the rounded-square family shape only in this task. Its
  three-region/two-thumb inline placement remains unchanged and must not be
  silently redesigned without separate operator evidence.
- Preserve forced-colour role pairs. Selected and remainder clipped layers map
  to `HighlightText` and `CanvasText` respectively.

## Dispatch manifest

- **State:** ready for immediate Queue dispatch in parallel with g18.013,
  g18.015 and g18.016; serial before held g18.011 and retained g18.006; g18.009
  remains held
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** Slider/RangeSlider contracts only for the approved
  shape and single-Slider text migration; shared TS/Rust block layout helpers;
  Svelte/React Slider render structure; web Slider/RangeSlider styles; shared
  Rust/GPUI Slider and RangeSlider rendering; focused component, headless,
  mounted and visual/browser tests; paired preview specimens if needed; one
  g18.017 execution log
- **Reserved closeout surfaces:** standard/embedded track appearance; public
  prop/type expansion; RangeSlider inline-placement redesign; unrelated tokens
  or components; g18 README/index/dispatch; g18.006/g18.009 and g18.011–g18.016
  task/workspace/PR state; workflows; release/tag/publication; Desktop
- **Worker:** high-reasoning cross-runtime component worker comfortable with
  CSS logical clipping, text measurement, Rust node composition, GPUI mounted
  proof, forced colours and visual regression evidence
- **Excluded:** new props; RangeSlider text redesign; motion; tooltips/marks;
  vertical block admission; release work; workflow changes; Desktop
- **Escalation:** Chatterbox for a required new token or public API, inability
  to express either approved in-track treatment, unavoidable numeric
  truncation, RangeSlider text-policy expansion, or any release/workflow mutation

## Work

1. Plant Svelte and React browser evidence at low/mid/high values showing the
   current label/value disappearance and external fallback movement. Bind the
   current pill radius separately.
2. Update Slider and RangeSlider block capsule radius from the pill role to
   `radius.control` across web CSS and shared Rust rendering. Prove the visible
   thumb remains circular.
3. Replace the single-Slider per-segment fit law with stable whole-track text
   placement and deterministic collision priority. Keep the exact numeric value
   visible inside the track.
4. Build paired clipped text layers in Svelte and React. Prove the same glyph
   coordinates before, during and after the fill boundary crosses each string,
   with selected/remainder foreground roles changing at the boundary.
5. Express the same stable placement and split-colour paint in shared Rust/GPUI
   composition. If the existing node substrate cannot clip it faithfully, use
   the approved cross-runtime larger-side fallback and bind its midpoint rule.
6. Cover LTR/RTL, all sizes, representative themes, forced colours, disabled,
   focus and active drag. Prove hit target, pointer capture, keyboard value law,
   ARIA values and one-commit terminal behavior are unchanged.
7. Cover narrow/localized labels: value remains exact and in-track, optional
   label suppression is stable across value changes, and no fallback sibling is
   rendered.
8. Run focused TS/Rust/headless/mounted/browser/visual checks, paired preview
   builds, contrast/accessibility checks, Effigy docs QA and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Family corners are rounded-square | web changes but GPUI or RangeSlider stays pill | CSS plus rendered Svelte/React/GPUI radius identity |
| Thumb stays circular | capsule migration squares the handle | independent capsule/thumb radius assertions |
| Text position is stable | label/value nodes resize with selected segments | equal glyph bounds at low/mid/high values |
| Crossover changes foreground only | glyph vanishes or jumps as fill crosses it | pixel/DOM paint proof immediately before/at/after crossover |
| Approved fallback stays internal | runtime limitation restores the external row | recorded substrate proof plus cross-runtime larger-side/tie journey |
| Value remains in-track | fit failure creates `.fallback` below the slider | narrow-width journey with exact numeric text inside capsule |
| Optional label degrades first | numeric value is truncated or hidden | collision case preserves full value and suppresses label |
| RTL is logical | clipping remains physically left-to-right | mirrored journey with stable logical anchors and correct colours |
| Forced colours remain legible | duplicate layers use author colours under HC | `HighlightText`/`CanvasText` mapping proof |
| Behavior is unchanged | new overlays steal pointer or alter commits | hit, drag, keyboard, cancellation and ARIA regression traces |
| Range scope stays bounded | task silently rewrites two-thumb label policy | radius-only RangeSlider diff and retained existing layout laws |
| Sweep remains gated | g18.011 begins with known visual churn | merged repair before Queue hold release |

## Stop conditions

- Stop if neither the preferred clipped paint nor the approved larger-side
  in-track fallback is expressible without a substrate addition.
- Stop if exact numeric value text cannot remain visible inside the configured
  track width under either approved treatment.
- Stop before adding public props, redesigning RangeSlider text placement,
  admitting vertical block appearance, or changing release/workflow state.

## Evidence

Operator screenshot on 2026-09-11 shows pill-ended block sliders and a value
label displaced below the track while the other example keeps content inline.
Source inspection confirms `999px`/`radius.pill` capsule styling and a
value-dependent all-or-nothing fit law that renders an external fallback when
either selected or remainder segment cannot hold its assigned string.

## Next task

After this task and g18.013/g18.014/g18.018/g18.019 close, Queue dependencies
dispatch g18.011 for the complete four-surface editor acceptance sweep. Keep
g18.006 paused; g18.009 waits on it.
