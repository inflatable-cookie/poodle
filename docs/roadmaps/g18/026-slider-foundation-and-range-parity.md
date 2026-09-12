# 026 — Slider foundation and RangeSlider parity

Status: ready — operator-approved pre-`0.4.0` repair
Owner: Poodle cross-runtime components
Created: 2026-09-12
Governing refs: `../../contracts/components/slider.md`,
`../../contracts/components/range-slider.md`,
`../../architecture/012-feedback-motion-and-state-change.md`
Depends on: g18.024 and g18.025 complete

## Outcome

Remove the implementation split that let Slider repairs miss RangeSlider.
Keep `Slider` and `RangeSlider` as separate public components and contracts,
but make both compose one private Slider-family rendering foundation. Port
every accepted post-g18.024 Slider behavior to RangeSlider before `0.4.0`.

## Ready-State Rubric

- [x] The operator selected separate public components over one union API.
- [x] Core already shares normalization, visual-state and layout helpers while
  retaining distinct one-thumb and two-thumb machines.
- [x] Svelte, React and CSS duplicate track, fill, handle, axis, hit-target and
  pointer presentation machinery.
- [x] Recent Slider-only fixes prove that duplication is causing release drift.
- [x] The intended block/embedded, size, orientation, polarity and text laws
  are already approved in the two component contracts.

## Decisions

- Preserve the public `Slider` and `RangeSlider` names, value types, props,
  callbacks and contracts. Do not introduce `number | [number, number]`, a
  public thumb-count switch, inheritance, aliases or compatibility wrappers.
- Keep distinct state machines. Slider owns one value and one slider focus
  stop. RangeSlider owns an ordered pair, active-thumb selection, non-crossing
  bounds and two independently named slider focus stops inside a group.
- Introduce one private family foundation for control-size metrics, axis and
  direction mapping, capsule/track geometry, fill segments, center markers,
  handles, text clipping, collision docking and layout-neutral 44×44 targets.
- Share the visual handle/marker implementation once. RangeSlider composes it
  twice and adds only two-thumb modifiers.
- Consolidate common CSS into one canonical foundation. Slider and RangeSlider
  style files retain only genuine single/range differences.
- Port all accepted Slider refinements to RangeSlider: shared size response,
  no layout margin from hit targets, sensible fixed-precision visible values,
  reachable extrema when step does not divide the domain, bounded inset line
  handles, horizontal label/value docking, vertical native-axis geometry and
  bipolar center-relative fill.
- Preserve Svelte/React semantic parity and shared Rust/GPUI output. Rust may
  keep its existing shared composition shape when it already satisfies the
  family law; proof, not forced file churn, is required.
- This task changes no public API. Stop and return any required public break to
  Chatterbox so g18.027 can classify it before release.

## UI Design Brief

- **Class:** release-blocking component-family refinement and internal
  consolidation.
- **Mode:** Operate. Slider and RangeSlider should look and respond as one
  family while preserving their different value semantics.
- **Geometry:** the same size, axis, capsule, handle, focus and hit-target
  primitives drive both controls. RangeSlider renders two instances of the
  same handle primitive.
- **Text:** horizontal RangeSlider keeps lower at logical start, label centered
  and upper at logical end. Vertical keeps upper top, label centered and lower
  bottom. Fill and handle crossover changes paint only.
- **States:** prove every size, block/embedded, horizontal/vertical,
  unipolar/bipolar, extrema, uneven step, equality, overlap, RTL, disabled,
  keyboard focus, pointer drag, Eclipse/Iceberg and forced colours.
- **Non-goal:** no visual redesign and no generalized multi-thumb component.

## Dispatch manifest

- **State:** ready for Queue submission after explicit execution authorization
- **Completion:** one clean pushed PR with exact-head independent review and
  queue-owned merge/closeout
- **Owned mutable paths:** Slider/RangeSlider contracts; shared TypeScript
  Slider core; private web foundation; Svelte/React Slider and RangeSlider
  shells and styles; shared Rust/GPUI Slider-family render paths where proof
  finds drift; focused paired tests, specimens, generated component docs and
  one execution log
- **Reserved closeout surfaces:** g18 README/index/dispatch/task state;
  g18.027; retained g18.006/g18.009 state; versions, changelog, release notes,
  workflows, publication and Desktop
- **Worker:** complex cross-runtime component worker comfortable with Svelte,
  React, CSS geometry, Rust render composition and accessibility
- **Excluded:** public API changes; multi-thumb generalization; unrelated
  controls; release mutations; Desktop; Jetstream; windowed selectors
- **Escalation:** Chatterbox for any public break, unresolved semantic mismatch,
  or foundation shape that cannot preserve both components' ARIA models

## Work

1. Plant paired laws showing the current Slider-only refinements missing from
   RangeSlider and proving common renderer behavior is not independently
   reimplemented.
2. Extract shared private geometry, pointer and visual helpers without changing
   either public component surface.
3. Consolidate the common CSS foundation and reduce the component styles to
   genuine one-thumb/two-thumb modifiers.
4. Port the accepted Slider behavior matrix to RangeSlider in Svelte and React.
5. Verify the shared Rust/GPUI renderers against the same contract and repair
   only observed drift.
6. Update paired specimens and generated docs so the family comparison is
   visible at all sizes and orientations.
7. Run focused core/component/native checks, paired Chromium/WebKit geometry
   and interaction probes, accessibility and forced-colour checks, package and
   preview builds, docs QA and `git diff --check`.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Public APIs remain clear | one union component or compatibility wrapper appears | declaration and packed-type comparison for separate scalar/pair components |
| Visual mechanics are shared | RangeSlider keeps a copied handle, axis or size implementation | source-bound foundation tests plus narrow component modifiers |
| Slider fixes reach RangeSlider | unequal sizes, raw decimals, unreachable max or broken vertical geometry remains | paired live matrix across both controls/frameworks/browsers |
| State machines stay truthful | shared renderer collapses two focus stops or permits thumb crossing | keyboard, pointer, ARIA and callback machine tests |
| Bipolar geometry is centered | RangeSlider or Slider fill still grows from an edge | positive/negative center-relative segment assertions in both axes |
| Native parity is real | web is repaired while Rust/GPUI emits stale geometry | headless render-state and focused native tests |
| Release stays closed | g18.006 resumes before this merge and g18.027 audit | retained candidate remains blocked |

## Stop conditions

- Stop before adding, removing or renaming a public prop, type or export.
- Stop if consolidation would weaken either component's accessibility or
  controlled/uncontrolled behavior.
- Stop before release, workflow, Desktop, Jetstream or windowed mutations.

## Next task

After merge and operator acceptance, run g18.027 against the resulting main
head. Retained g18.006 remains blocked until that audit closes with every
public break classified.
