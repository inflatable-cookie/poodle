# 024 — Slider-family layout and vertical repair

Status: ready for Queue dispatch — operator approved 2026-09-12
Owner: Poodle cross-runtime components
Created: 2026-09-12
Governing refs: `../../contracts/components/slider.md`,
`../../contracts/components/range-slider.md`,
`../../architecture/012-feedback-motion-and-state-change.md`
Depends on: g18.022 complete and merged

## Outcome

Repair the post-g18.022 Slider-family UX defects before `0.4.0`: block Slider
and RangeSlider align with other controls on the shared `xs`–`xl` size axis,
44×44 hit targets stop adding layout whitespace, default visible numbers are
step-aware rather than binary-float strings, and vertical block controls render
as usable native-axis controls rather than clipped fragments.

This task is independent of g18.023 and may run beside it. Both must merge
before retained g18.006 resumes.

## Ready-State Rubric

- [x] g18.022 is merged and the block-first public API is settled.
- [x] The operator observed block controls misaligned with adjacent Poodle
  controls because Slider uses a private capsule ladder and a layout-bearing
  44px hit envelope.
- [x] The block size specimens do vary, but their 28/30/32/36/40px capsule
  ladder does not match the shared 24/28/36/44/52px control ladder.
- [x] The operator observed raw floating-point tails in visible values.
- [x] The operator observed vertical block specimens with clipped values,
  collapsed text, and unusable geometry.
- [x] Slider and RangeSlider share the same sizing, formatting, hit-target, and
  orientation obligations.

## Decisions

- The block capsule consumes the shared `--poodle-size-control-height` axis:
  24px `xs`, 28px `sm`, 36px `md`, 44px `lg`, and 52px `xl`. Do not maintain a
  second Slider-only height ladder.
- The root's measured cross-axis layout size equals the visible capsule size.
  A 44×44 effective target may overflow that box, but it must not contribute
  margin, padding, minimum size, row height, or baseline drift.
- Density and size remain independent. Density must not inflate block-axis
  layout or mask the selected size. Same-size Slider, RangeSlider, Button,
  Input, and Select controls align in an ordinary flex or grid row.
- Scale internal block metrics with the shared size: text, inset, corner radius,
  and visible handle remain proportionate. The handle's effective target stays
  at least 44×44 without forcing the painted handle or capsule to 44px.
- The default visible formatter rounds the normalized step-snapped value to the
  decimal precision implied by `min` and a finite positive `step`, emits the
  shortest ordinary decimal, trims insignificant trailing zeroes, and converts
  negative zero to `0`. It must never show binary floating-point tails. The
  existing web `formatVisibleValue` callback remains the authoritative override;
  native specs continue to receive resolved strings.
- Apply the same default law independently to both RangeSlider endpoints. Do
  not change value math, callback payloads, accessible numeric values, or
  consumer formatting APIs.
- Vertical block is native axis geometry, never a rotated horizontal capsule.
  Its root has host-owned length and shared-size cross-axis width. Track, fill,
  handles, hit targets, focus rings, and clipping share one coordinate system.
- Keep all vertical text upright and inside the capsule. Slider value stays at
  physical top and label centered. RangeSlider upper value stays top, label
  centered, and lower value bottom. Fill/handle crossover changes text
  foreground only; it never clips, shifts, rotates, or collapses text.
- Preserve horizontal/vertical keyboard meaning, pointer normalization,
  polarity, RTL, disabled state, forced colours, and change/commit behavior.
- Repair Svelte, React, shared Rust composition, and GPUI together wherever the
  public contract or emitted visible strings are affected. No runtime earns
  parity from source shape alone.

## UI Design Brief

- **Class:** release-blocking refinement of a settled component family.
- **Mode:** Operate. Sliders sit cleanly beside other workbench controls and
  communicate values without decorative whitespace or numerical noise.
- **Alignment:** the visible capsule is the component's layout box. In a mixed
  same-size row, its top and bottom edges align with Button/Input/Select edges.
- **Scale:** use the shared five-step control ladder. Differences must be
  visible and coherent, with `md` matching the ordinary control height.
- **Vertical form:** a tall rounded-square rail with upright anchored content,
  a legible fill boundary, small handles, unclipped focus, and no horizontal
  remnants. Host height remains explicit; size changes rail width and internal
  metrics.
- **Numbers:** calm, short operational readouts such as `0.85`, never
  `0.8500000000000001`. Custom units and precision remain consumer-owned through
  the existing formatter.
- **Accessibility:** retain measurable 44×44 targets, visible keyboard focus,
  slider semantics, orientation, value bounds/current value, and forced-colour
  legibility without making the accessibility envelope visible as spacing.

## Dispatch manifest

- **State:** ready; parallel-safe with g18.023; serial before retained g18.006
- **Completion:** one open non-draft PR at a clean pushed head with exact-head
  independent review; never merge
- **Owned mutable paths:** Slider/RangeSlider contracts; shared TypeScript and
  Rust slider value/visual helpers; Svelte and React Slider/RangeSlider wrappers
  and styles; shared Rust composition and GPUI slider renderers; paired slider
  specimens and focused geometry/interaction/a11y/parity tests; generated
  component docs only where those surfaces require regeneration; one g18.024
  execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch/task state;
  g18.023; g18.006/g18.009 state; versions, changelog, workflows, release/tag,
  Desktop, unrelated controls and editor work
- **Worker:** high-reasoning cross-runtime UI worker comfortable with CSS
  geometry, pointer hit testing, native-axis layouts, numeric precision, GPUI,
  and real-browser measurement
- **Excluded:** a new formatter prop; a Slider-only public size axis; API aliases;
  value-machine changes beyond canonical display serialization; release
  mutations; windowed selectors
- **Escalation:** Chatterbox if maintaining a layout-neutral 44×44 target is
  impossible in an active runtime, the shared size ladder cannot apply without
  a public API change, or vertical content cannot remain stable under the
  existing fit law

## Work

1. Plant exact geometry failures from the merged specimens: mixed-control row
   alignment, all five shared heights, density independence, and hit rectangles
   that do not affect the root layout box.
2. Replace private block capsule heights with the shared control-size axis and
   scale internal text/inset/handle metrics coherently in all active runtimes.
3. Rebuild the block hit layer so its 44×44 rectangle remains measurable and
   interactive while staying out of layout and unclipped at host boundaries.
4. Add one shared default display serializer and bind both Slider families to
   it. Prove fractional steps, non-zero fractional minima, integer steps,
   trailing-zero trimming, negative zero, and custom-formatter precedence.
5. Reproduce and repair vertical block Slider and RangeSlider across Svelte,
   React, shared Rust, and GPUI. Bind native-axis paint, hit testing, focus, text
   anchors, polarity, two-thumb ordering, and host-owned length.
6. Exercise pointer and keyboard interaction at vertical min/mid/max, bipolar
   below/at/above center, overlapping range thumbs, disabled, RTL, all sizes,
   both densities, forced colours, and text/fill crossover.
7. Update paired specimens to put each Slider family beside same-size reference
   controls and show representative fractional vertical values.
8. Run focused component/core/GPUI tests, paired Chromium/WebKit geometry and
   interaction probes, package and preview builds, accessibility checks, docs
   QA, and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Shared size means shared geometry | Slider uses another ladder or all roots remain 44px | measured 24/28/36/44/52px capsules aligned beside same-size reference controls |
| Hit targets do not become spacing | xs capsule sits inside a 44px layout row or density adds top/bottom whitespace | root/capsule rectangles plus independent ≥44×44 hit rectangles at every size/density |
| Default values are human decimals | a snapped `0.85` renders a binary tail | shared serializer vectors and rendered single/range values in every active runtime |
| Consumer formatting still wins | default rounding rewrites units or chosen precision | callback precedence and native resolved-string proof |
| Vertical is a complete rendition | text clips, handles float outside the rail, or pointer mapping uses horizontal coordinates | paired browser screenshots/geometry/interaction plus GPUI headless render-state proof |
| Anchors never move | fill/handle crossover shifts or hides a label/value | min/mid/max and overlap measurements on stable nodes |
| Runtime parity is real | web looks fixed while GPUI retains old sizes/strings/geometry | shared contract vectors and runtime-specific mounted/render proof |
| Release stays closed | candidate starts before the repair and palettes merge | retained g18.006 remains paused until g18.023 and g18.024 close |

## Stop conditions

- Stop if the repair requires a new public formatting or sizing API.
- Stop if a 44×44 effective target cannot remain interactive without changing
  surrounding layout; return per-runtime geometry instead of reducing it.
- Stop before editing g18.023, release state, versions, changelog, workflows,
  Desktop, or unrelated components.

## Evidence

Post-merge inspection on 2026-09-12 found:

- shared controls use 24/28/36/44/52px for `xs` through `xl`;
- block Slider and RangeSlider use 28/30/32/36/40px capsule heights;
- both roots take `max(block height, 44px)` and density adds block-axis padding,
  so the accessibility envelope participates in ordinary layout;
- current default visible text is `String(value)` after floating-point step
  arithmetic; and
- the vertical block specimen visibly clips and displaces its upright values
  and labels instead of presenting a complete rail.

## Next task

After g18.023 and g18.024 merge, accept the repaired specimens and resume the
same retained g18.006 release-candidate task. g18.009 remains serial after it.
