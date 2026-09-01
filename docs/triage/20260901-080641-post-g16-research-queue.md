# Post-g16 Research Queue

Status: open — operator-reviewed queue; promotion and implementation remain
orchestrator-owned
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Source: `docs/handoffs/20260901-080641-post-g16-research-queue.md`
Scope: four named value-track dossiers and their shared promotion dependencies

This is a planning packet, not execution authority. The four research dossiers
remain read-only. No component, contract, architecture, roadmap, release,
consumer, or Jetstream change is authorized by this note.

## Operator-confirmed direction

- The first post-g16 outcome is a shared, host-level motion policy.
- The first motion lane uses the five-family pilot: disclosure;
  transient notification; Tabs selection indicator; discrete state
  (`Checkbox` plus the same-slot `IconButton`/state-swap pattern); and
  loading/reveal (`Skeleton`/`Spinner`).
- The transitions catalogue, icon morphing, and shimmer dossiers are grouped
  under that motion-policy initiative. Block Slider/RangeSlider remains a
  separate component lane.
- Block sliders are the next independent component bet after the motion lane.
  The approved direction is additive: separate visual content from ARIA
  naming/value text, use inline content only when it fits with a stable
  fallback readout, target the nearest RangeSlider thumb and hold it for the
  gesture, and wait for native RangeSlider vertical geometry before full
  vertical admission. The exact public appearance name remains a gate.
- Icon morphing enters as a feasibility spike first. The likely public boundary
  is a curated pair registry behind a separate morph primitive; the existing
  `Icon` stays static unless a later decision changes its contract.
- The shimmer candidate host is the `AgentSubagent` activity line. Its visual
  treatment is explicitly web-only for now; native semantics stay static and
  no active-cohort visual-parity claim is made.
- `g16.028` remains the ordered first gate. This queue cannot displace,
  modify, or dispatch work against the active drag closeout.
- Jetstream remains program-deferred.

## Dependency model

```text
g16.028 drag closeout
        |
        v
host-level motion policy + five-family pilot
        |\
        | \
        v  v
block-slider lane   icon feasibility / shimmer host gates
        |
        v
later component/API promotion only after each track's evidence gate
```

The block lane is independent of the motion implementation and is deliberately
the next component bet, but it still waits for the current g16 runway to close.
Icon feasibility and shimmer benchmarking are downstream gates of the grouped
motion initiative. Their relative order is not material: both may be planned
after the policy boundary is settled, with no public API admission until their
own evidence is accepted.

## Evidence synthesis

### Shared motion foundation — transitions.dev catalogue

The pinned catalogue audit reconciles 43 cards: 32 free and 11 Pro, with 43
data records, 43 generated detail directories, and 32 public free references.
It is evidence for semantic lifecycle policy, not a Poodle recipe catalogue.

The strongest Poodle matches are disclosure, transient notification, Tabs
selection, Checkbox/state change, and loading/reveal. The audit found the
current cross-runtime gaps that make a shared policy useful: Svelte has
disclosure/Drawer transitions where React is structural; native ToastStack,
Skeleton, and Spinner already declare motion while web loading styles lack one
policy; and `poodle-node` does not represent width/height, blur, path draw, or
arbitrary easing.

The recommended policy boundary is:

- semantic intent and phase (`enter`, `exit`, `update`, `loop`), with semantic
  state and ARIA value updated immediately;
- host-level full, reduced, and deterministic frozen-capture modes;
- stable identity across rebuilds, explicit reverse/retarget/abort/unmount
  behavior, and cleanup for timers, rAF, WAAPI, pointer capture, and native
  clocks;
- a default cross-runtime property budget of opacity, translation, scale, and
  rotation; layout, blur, path drawing, gradients, filters, canvas, and 3D
  effects require an explicit rationale and static/reduced fallback;
- shared web-core behavior/styles, shared Rust composition, thin web shells,
  and GPUI interpretation. Jetstream stays deferred.

The pilot must not add Dialog intermediate states. It should prove lifecycle,
reduced mode, interruption, stable identity, final state, and semantic
ownership across the five selected families before named recipe follow-ons.

### Independent block Slider/RangeSlider direction

The block dossier treats the reference as a track-dominant presentation over
the existing value laws, not an image to copy. The safe direction is an opt-in
appearance with current standard and embedded defaults unchanged.

The approved queue baseline preserves one Slider value law and two
independently focusable RangeSlider controls. It keeps lower/upper identity and
no-crossing behavior stable, makes visible label/value content distinct from
`ariaLabel`/`valueText`, and uses a deterministic fit ladder rather than
clipping or overflowing content. Full-track nearest-thumb targeting followed
by gesture ownership is the recommended interaction. A visible handle remains
smaller than its effective pointer/touch target, which must be measurable at
the adopted minimum.

The dossier also records material admission limits: current Rust RangeSlider
does not project all per-thumb semantic bounds/value text, native vertical
scrub geometry is deferred, GPUI accessibility remains a documented blocker,
and web Svelte/React cancellation/commit paths are not identical. These are
promotion gates, not CSS polish.

### Curated icon morphing

The icon dossier confirms that true morphing needs parsing, normalization,
resampling, topology/correspondence, deterministic serialization, and
interruption/reversal policy. “Any icon” is not a quality boundary: arbitrary
provider geometry can contain unsupported tags, fills, transforms, mismatched
viewBoxes, or poor correspondence.

Poodle's current Lucide manifest and fixed 24x24 stroke-oriented assets are a
useful source boundary. The recommended shape is a pure framework-free
geometry/lifecycle utility plus a generated, curated semantic pair registry.
The public input should be a pair key/reference, not arbitrary raw SVG or a
new animation prop on `Icon`.

The hard gate is GPUI feasibility. `poodle-node` currently names static icon
assets and supports opacity/transform animation, not mutable path geometry.
The spike must prove a dynamic path route at pinned GPUI 0.2.2, including
frame pacing, retained-tree behavior, color/stroke treatment, interruption,
and teardown, before an active-cohort morph contract can be considered.
Static endpoint swap/cross-fade remains the safe fallback.

### Semantic shimmer candidate

The shimmer dossier rejects a generic `TextShimmer` component, `Text` prop, or
arbitrary animated prose treatment. The Pen demonstrates a credible
mask-plus-translated-highlight experiment, but “GPU accelerated” is not a
Poodle fact without engine/device traces. Duplicate text also creates wrapping,
font, bidi, localization, selection, forced-color, and accessibility risks.

The operator-selected candidate is the `AgentSubagent` running activity line.
That host is draft and its React/native variants are not current active-cohort
admission surfaces, so host contract/runtime readiness comes first. The effect
may be a narrow web-only recipe: one semantic/copyable text source, host-owned
status/live-region behavior, explicit active/paused/reduced/off state, readable
static fallback, and no numeric progress meaning. Native must retain explicit
static semantics rather than silently substituting Skeleton pulse.

The later benchmark must compare static text, the current background-position
baseline, mask-plus-transform, and supported fallbacks across agreed engines,
content shapes, node counts, reduced/forced modes, selection/copy, and layout,
paint, raster, layer, memory, and frame-time evidence. Numeric budgets remain
open.

## Recommended promotion order

1. **Finish `g16.028`.** Keep the drag closeout sole owner of its active files,
   certification evidence, and current runway. Do not create a competing
   motion or visual worker while it is active.
2. **Promote the shared motion policy and five-family pilot.** Start with one
   architecture/contract decision. Settle host-level policy propagation,
   semantic roles, full/reduced/frozen behavior, stable identity,
   interruption/cancellation, final-state rules, the native property subset,
   and the pilot's evidence shape before adding named effects.
3. **Promote block Slider/RangeSlider next as the independent component lane.**
   Translate the approved baseline into Slider and RangeSlider contracts and
   an additive implementation plan. Keep the exact appearance name, remaining
   keyboard/direction choices, vertical native gate, and default migration out
   of implicit implementation scope.
4. **Run the icon feasibility spike and shimmer host/benchmark gates under the
   motion umbrella.** They are separate evidence batches, not one public API:
   icon must clear dynamic-path/native and curated-pair gates; shimmer must
   clear AgentSubagent host ownership, web-only boundary, accessibility, and
   performance gates. Do not open either implementation card from catalogue
   resemblance alone.
5. **Only after accepted evidence, promote canonical implementation cards.**
   The orchestrator re-reads this packet against current `main`, resolves any
   g16.028 drift, chooses canonical destinations, and applies normal readiness
   gates. Merge of this packet is intake, not feature promotion.

## Shared decisions and independent decisions

### Shared across motion tracks

- Host-level policy propagation through web core and `RenderContext`.
- Full, reduced, and frozen-capture modes, with the default when a host cannot
  report preference still to be specified in the architecture decision.
- Semantic state commits immediately; motion never owns ARIA, status text,
  progress, focus, or correctness.
- Stable identity, retarget/reverse rules, cancellation, unmount cleanup, and
  deterministic final output.
- A bounded token/property vocabulary and explicit native approximation rules.
- Theme, forced-color, reduced-motion, accessibility, and deterministic-capture
  evidence. GPUI limitations stay named; Jetstream stays deferred.

### Independent block decisions

- Exact public appearance name and contract location.
- Visible label/value fields, formatter ownership, fit threshold, and stable
  readout placement.
- PageUp/PageDown, RTL direction semantics, overlap tie behavior, and
  cancellation commit policy.
- Forced-color role ownership, effective 44x44 target proof, and native
  vertical RangeSlider geometry.
- Whether invalid/read-only states remain wrapper-owned.
- No default replacement without a separate consumer/migration record.

### Independent icon decisions

- Whether true geometry morphing is worth the native capability cost versus a
  static swap/cross-fade.
- Pair registry ownership, pair approval/removal, quality thresholds,
  direction/reversibility, and provenance/licence rules.
- Public pair-key boundary versus a future explicitly validated custom adapter.
- Generated geometry versus runtime plan cache, or a bounded hybrid.
- GPUI dynamic path capability and the shape of any new renderer-neutral node.

### Independent shimmer decisions

- Exact AgentSubagent contract/runtime prerequisite and activity-line lifecycle.
- Pause/stop/hide behavior for persistent activity and the host's live-region
  cadence.
- Benchmark node counts, device classes, engine coverage, frame/layer/memory
  budgets, and acceptable fallback thresholds.
- Whether new highlight/base token roles are justified after contrast evidence.
- Attribution/legal handling if a future implementation is derivative of the
  public Pen. No Pen source is copied by this queue.

## Promotion gates by track

### Motion policy and five-family pilot

Promote only when the architecture/contract record specifies:

- the host-level input and propagation path across web core, Svelte, React,
  shared Rust, and GPUI;
- semantic roles and phase state for each pilot family;
- full, reduced, and frozen-capture outcomes, including default behavior when
  preference is unavailable;
- stable keys, reverse/retarget, repeated activation, abort, pointer cancel,
  unmount, delayed cleanup, and final semantic/visual state;
- the supported native property/easing subset and each approximation;
- focus, live-region, label, keyboard/touch, and GPUI accessibility boundaries;
- token/build/drift validation and deterministic test shape.

Pilot acceptance must prove disclosure reversal, keyed toast enter/exit and
no-focus-theft behavior, measured Tabs indicator first-layout/resize rules,
immediate Checkbox/state semantics, and loading/reveal static/reduced/unmount
behavior. Dialog presence remains outside this lane until its own contract
changes.

### Block Slider/RangeSlider

Before a contract or implementation card is ready, record:

- exact additive appearance name; current standard/embedded defaults remain
  behaviorally and visually stable;
- visible content channels separate from accessible names/value text;
- deterministic inline-fit and stable fallback readout behavior;
- two-thumb identity, overlap layering, nearest-thumb tie rule, and effective
  target geometry;
- pointerup, pointercancel, lost capture, teardown, stale pointer, and
  disabled terminal semantics with exact callback/commit counts;
- PageUp/PageDown, RTL, forced colors, reduced motion, contrast, and vertical
  admission decisions;
- Rust per-thumb semantic bounds/value-text projection and native vertical
  geometry, with the GPUI accessibility limitation named honestly;
- focused core/web/Rust/GPUI evidence and a performance probe before any
  visual or default-migration claim.

### Icon morphing

Before a public contract or API card is ready, require:

- a disposable GPUI 0.2.2 dynamic-path feasibility result or an explicit
  decision to retain static swap/cross-fade;
- curated meaningful pairs, both-direction visual review, topology/stroke/
  viewBox constraints, deterministic endpoint serialization, and rejection of
  poor/unsupported input;
- provenance/licence records for every source and derived artifact;
- pure-core traces for plan cost, interpolation, interruption, reversal,
  reduced motion, controlled seek if justified, and destroy/unmount;
- stable accessible outer semantics with no per-frame announcement;
- Svelte/React hydration and reduced-motion evidence plus shared Rust/node
  evidence if active-cohort parity remains the contract;
- explicit node/backend boundary. Do not overload static `NodeKind::Icon` or
  put path strings into generic property animation.

### AgentSubagent shimmer candidate

Before any effect implementation is admitted, require:

- an accepted AgentSubagent activity-line contract and runtime ownership;
- one semantic, selectable, copyable text source and host-owned status/live
  semantics; no duplicate accessible or copied text;
- active, paused, reduced, forced-color, print, unsupported-mask, hidden,
  offscreen, resize, unmount, and cancellation behavior;
- explicit web-only recipe wording and static native presentation, unless a
  later native capability decision replaces it;
- benchmark evidence against static text and current shimmer approaches on
  the agreed browser/device/content/node-count matrix;
- no layout movement, unbounded layer/memory growth, or material frame-time
  regression at the agreed budgets;
- a token/contrast decision and any Pen attribution/legal record.

## Rejected, deferred, and grouped work

- Do not split the four dossiers into four competing first bets. The motion
  dossiers share policy/lifecycle decisions; block sliders remain independent.
- Do not copy, import, or repackage the 43-card Transitions.dev catalogue or
  Pro source. Decorative hover, confetti, goo, gradient branding, image/canvas
  effects, private-reasoning presentation, and marketing transitions remain
  product-owned or rejected.
- Do not add modal/dialog intermediate phases from the catalogue without a
  separate Dialog contract decision.
- Do not replace the standard Slider/RangeSlider default. Do not use a
  recipe-only change to hide content, fit, hit-target, or native-parity
  decisions.
- Do not add arbitrary raw SVG or automatic name-change animation to `Icon`;
  do not adopt Morphicons as a runtime dependency. Static swap/cross-fade is
  the safe icon fallback.
- Do not add `TextShimmer`, an animated `Text` prop, a generic AgentMessage
  shimmer, duplicate caller-maintained strings, or a Skeleton substitute.
- Do not call the shimmer candidate “GPU accelerated” without measured
  engine/device evidence. Use conservative candidate language.
- Do not admit Jetstream, claim GPUI assistive-technology parity, or turn
  frozen visual capture into user reduced-motion evidence.
- Do not implement, promote, mark ready, launch workers, edit canonical
  contracts/architecture/roadmaps/logs, mutate packages/releases/workflows,
  or change downstream consumers in this planning lane.

## Suggested canonical destinations

These are promotion destinations, not edits authorized by this packet.

1. **Motion:** one cross-runtime semantic motion architecture/contract
   decision, followed by a meaningful pilot card. Update the existing motion
   and appearance authorities only after the policy, pilot states, and native
   boundaries are settled.
2. **Block sliders:** Slider and RangeSlider component contracts, shared
   appearance-recipe mapping, then an active-generation roadmap card for the
   additive implementation/evidence lane. Keep default migration in a later
   explicit record.
3. **Icon morphing:** a new separate morph primitive contract and geometry/
   registry architecture record only after the native spike, provenance policy,
   and pair ownership are accepted. Keep static `Icon` authority unchanged.
4. **Shimmer:** the selected AgentSubagent activity contract and an explicit
   web-recipe/benchmark evidence record. If that host cannot own the need,
   return to planning for a different semantic consumer; do not fall through
   to generic `Text`.

## Non-goals

- No implementation or public API change.
- No default Slider/RangeSlider replacement or compatibility alias.
- No generic animated text/icon API, arbitrary SVG geometry promise, or
  third-party transition dependency.
- No copied code, assets, screenshots, catalogue recipes, Pro source, or Pen
  fragment.
- No new conformance authority, exhaustive specimen surface, or release/
  publication/adoption work.
- No Dialog presence change, page-transition programme, broad visual sweep,
  native accessibility claim, Jetstream admission, or downstream migration.
- No change to the ordered g16 runway or overlap with `g16.028`.

## Disposition

The four dossiers remain the evidence record. Keep their originating triage
notes unchanged until the orchestrator promotes or explicitly closes them.
Keep this packet open through planning-PR review and merge. After merge, the
orchestrator must reconcile it with current `main`, resolve any drift, choose
canonical destinations, and promote only settled meaning. This packet itself
does not make any track ready for implementation.

Validation for this planning batch is `effigy docs:lint` plus an exact diff
scope check. The final PR must contain only this triage file.
