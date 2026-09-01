# Post-Motion Research Queue

Status: open — motion policy is promoted as g16.034; three downstream tracks
retain evidence or operator gates
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Supersedes: `20260901-080641-post-g16-research-queue.md`

Planning PR #121 promoted the shared motion policy and five-family pilot into
architecture 012, MotionPolicyProvider, the affected component contracts, and
ready card `g16.034`. This note preserves only the unresolved downstream work.
It is not execution authority.

## Ordered Frontier

```text
g16.034 shared motion policy + five-family pilot
        |\
        | \
        v  v
block Slider/RangeSlider   icon feasibility / shimmer evidence gates
```

Block sliders are the next independent component lane after accepted g16.034.
Icon feasibility and AgentSubagent shimmer may be planned after the motion
policy lands. They remain separate evidence batches and do not form one public
API.

## Block Slider And RangeSlider

Approved direction:

- additive appearance; standard and embedded defaults stay unchanged;
- visible label/value content stays separate from accessible names/value text;
- inline content appears only when it fits, with one stable fallback readout;
- full-track RangeSlider targeting selects the nearest thumb, then holds that
  identity for the gesture;
- the visible thumb may be small while its effective pointer/touch target stays
  measurable at the adopted minimum; and
- full vertical admission waits for real native RangeSlider geometry.

Still unresolved before promotion:

- exact public appearance name and contract location;
- formatter, fit threshold, and stable readout placement;
- PageUp/PageDown, RTL direction, overlap tie, and cancellation commit rules;
- forced-color roles, effective target proof, and native per-thumb bounds/value
  text; and
- whether invalid/read-only states remain wrapper-owned.

Evidence: `../research/value-tracks/block-slider-visual-direction.md`.

## Icon Morphing

Run a disposable GPUI 0.2.2 dynamic-path feasibility spike before any public
contract. The likely boundary is a curated semantic pair registry behind a
separate morph primitive; existing `Icon` remains static. Static endpoint swap
or cross-fade is the safe fallback.

The spike must cover dynamic path construction, frame pacing, retained-tree
behavior, stroke/color treatment, interruption, teardown, and provenance. It
must not add arbitrary raw SVG, a runtime Morphicons dependency, or automatic
animation to icon name changes.

Evidence: `../research/value-tracks/icon-morphing.md`.

## AgentSubagent Shimmer

The candidate host is the AgentSubagent running activity line. The effect is
explicitly web-only unless native text-mask support is later proven; native
semantics remain static. AgentSubagent contract/runtime ownership precedes any
effect work.

A later benchmark compares static text, the current background-position
baseline, mask-plus-transform, and supported fallbacks. It must cover agreed
engines, content shapes, node counts, reduced/forced modes, selection/copy,
layout, paint, layers, memory, and frame time. Numeric budgets remain open.

Do not add generic `TextShimmer`, animate arbitrary `Text` or AgentMessage, copy
the public Pen, or claim GPU acceleration without traces.

Evidence: `../research/value-tracks/text-shimmer-effect.md`.

## Promotion Route

1. Merge g16.034 before promoting the block-slider implementation lane.
2. Resolve the block contract choices, then create one additive Slider/
   RangeSlider card.
3. After motion lands, run icon feasibility and settle AgentSubagent ownership
   plus shimmer benchmark budgets as separate planning/evidence lanes.
4. Remove this note when every branch is rejected or promoted into its owning
   canonical plan.

Jetstream remains deferred. No release, consumer adoption, default slider
migration, public icon-morph API, or shimmer implementation is authorized here.
