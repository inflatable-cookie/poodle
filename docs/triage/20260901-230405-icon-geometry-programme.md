# Icon Geometry Programme

Status: accepted programme — compiled as `g16.049`–`g16.051`
Disposition: `g16.049` ready; `g16.050`/`g16.051` serially blocked; IG-07
public admission remains held
Captured: 2026-09-01
Owner: Poodle Northstar planning delegate
Promotion owner: Poodle Northstar orchestrator
Programme source: the committed icon-geometry-programme planning handoff

This packet turns the completed native feasibility spike into a bounded
renderer-neutral geometry programme. It records the settled funding and
boundary decisions, then proposes the architecture, evidence, budgets, and
execution sequence the orchestrator can promote. It does not change a
contract, add a node variant, or admit a public IconMorph.

## Settled Decisions

The handoff settles these decisions. They are recorded here, not reopened:

- The geometry programme is funded. The work continues past the static
  fallback because true geometry continuity is valuable enough to investigate.
- Existing Icon remains static. There is no animation prop, automatic morph on
  an icon-name change, or change to its current registry and accessibility
  contract.
- Arbitrary provider IconNodes and raw SVG path input are not the first public
  boundary. A consumer-supplied icon remains static unless a later, separately
  approved adapter gives it an explicit validated geometry contract.
- Pair identity, endpoint selection, and provenance are curated inputs
  projected into generated registries. A valid SVG shape is not by itself a
  Poodle-approved morph pair.
- Architecture 012 owns full/reduced/frozen policy, semantic ownership,
  latest-state-wins interruption, and terminal cleanup. Icon geometry does not
  create a second motion policy.
- Native visual admission requires the accepted [dedicated conformance lab
  boundary](20260901-230407-conformance-lab-architecture.md). The lab is the
  sole native visual transport: its separate internal repository runs one
  operator-approved, non-activating GPUI process per fixture and returns the
  exact-window and provenance evidence. The historical g15.059–g15.061 path
  informs safety invariants only; it is not a second Poodle execution route.
  A headless path-building result is necessary evidence, not visual admission.
- The active cohort remains Svelte, React, shared Rust composition, and GPUI.
  Jetstream remains deferred under the program-wide rule.

No operator question is requested in this lane. If a later card exposes a
choice that is not bounded by this packet or the promoted authorities, it
stops and returns to the orchestrator.

## Evidence And Current Baseline

The two icon dossiers are complementary:

- [Poodle-owned icon morphing](../research/value-tracks/icon-morphing.md) is
  the broader research dossier. It recommends a pure geometry substrate and a
  curated generated pair registry, and records the limits of arbitrary
  topology, providers, fill semantics, SSR, and frame work.
- [Native icon morphing feasibility](../research/value-tracks/icon-morphing-native-feasibility.md)
  is the completed GPUI spike. It proves that GPUI 0.2.2 can build and paint
  dynamic paths in a disposable harness, but does not prove display pacing,
  pixel fidelity, production node transport, or native admission.

The current local contracts narrow the design:

| Surface | Current fact | Planning consequence |
| --- | --- | --- |
| Icon | The [Icon contract](../contracts/components/icon.md) is a static display primitive with five sizes, a fixed 0 0 24 24 viewBox, stroke-oriented defaults, stable accessible/decorative modes, and no internal state. | Keep Icon unchanged. A future morph must be a separate semantic primitive. |
| IconProvider | The [IconProvider contract](../contracts/components/icon-provider.md) is a registry/context boundary. Web provider content is application-owned; native hosts own their registry. | Provider scope must not silently widen the morph input set. |
| IconButton | The [IconButton contract](../contracts/components/icon-button.md) keeps label, pressed, loading, focus, and busy semantics on the button. | A changing glyph is paint only. It must not carry state, naming, focus, or callbacks. |
| Generated source | The current manifest has 92 canonical names and 16 aliases, 108 supported names in total, from lucide-static 1.31.0. | Canonicalize aliases and derive pair data from the existing manifest and asset generator. Do not create a second catalogue. |
| Web icon shape | IconNodes has open string tag names, while the current Svelte and React renderers emit a closed supported tag set. | Validate effective generated geometry, not only the TypeScript shape. |
| Shared node | The [node vocabulary](../../packages/contracts/node/src/lib.rs) carries NodeKind::Icon with a name and resolved size. It has no path geometry field. | Keep the named-asset Icon path and add a distinct resolved geometry capability only if the programme earns it. |
| Native composition | IconSpec and [shared icon rendering](../../packages/render/src/icon.rs) resolve size and colour, then emit a named icon node. | The future geometry node belongs after shared composition and token resolution, not in a GPUI-only escape hatch. |
| Animation | NodeAnimation currently carries scalar and transform properties only. | Do not add SVG d strings or path buffers to AnimProperty. Geometry is content; the clock remains a separate lifecycle channel. |
| GPUI backend | The [GPUI icon branch](../../packages/gpui/node-backend/src/lib.rs) loads assets/icons/name.svg. The spike separately proved PathBuilder and Window::paint_path can build dynamic paths. | A production backend needs a new node-to-path channel and invalidation proof. Re-serializing SVG strings is not the native route. |
| Motion | [Architecture 012](../architecture/012-semantic-motion-policy.md) excludes path drawing from the default property budget and defines explicit full/reduced/frozen and cleanup laws. | Geometry motion is a named role-specific capability; it does not expand the generic animation property list. |

The parity documents contain useful implementation evidence, but generated
source remains authoritative for counts and bytes. The historical count prose
in the IconProvider parity note must not be copied into a new registry without
reconciling it against default-icons.json and the generator.

The spike's strongest result is bounded: a custom GPUI element rebuilt 61
distinct intermediate geometries and exercised stroke and fill path
construction. Its test platform did not present images, its frame requests
were manually driven, and its timings are directional. The result is a
geometry-channel feasibility signal, not a 60 Hz or pixel-equivalence claim.

The accepted [dedicated conformance lab architecture](20260901-230407-conformance-lab-architecture.md)
sets the native visual boundary for this programme. Poodle owns the icon
fixture meaning, geometry policy, comparison criteria, and admission result.
The lab owns the short-lived one-process-per-fixture capture, exact-window
transport, and focus/permission/provenance receipt. The lab is a separate
internal repository with no reverse dependency from Poodle; Poodle's default
QA and CI remain headless. Poodle workers do not run a local
windowed/native-visual selector. If the lab is unavailable, IG-06 stops and
no public IconMorph admission occurs.

## Recommended Boundary

### Semantic pair boundary

A future public primitive may be named IconMorph, but it is not a current
export or contract. Its first input should be a curated pair reference with:

- a stable pair identity;
- two canonical endpoint identities;
- an authored semantic direction or state mapping;
- size and size-role inputs matching Icon;
- an accessible label with Icon's stable outer semantics; and
- the effective MotionPolicy supplied by the host.

It should not accept raw d strings, arbitrary IconNodes, a provider object, a
runtime dependency, or an automatic “morph whenever icon changes” mode.
Changing the pair identity is a new authored geometry choice, not an implicit
retarget across unrelated shapes.

### Geometry substrate

Use a pure framework-free geometry utility as the only place that parses,
normalizes, samples, matches, interpolates, and serializes geometry. It should
have equivalent TypeScript and Rust evidence over the same versioned vectors.
Neither implementation may import Svelte, React, DOM, GPUI, or a host clock.

The utility should expose a small internal vocabulary:

- validated canonical endpoint geometry;
- contour signatures and fixed sampled buffers;
- a deterministic pair plan;
- a sampled frame for interpolation;
- canonical endpoint serialization; and
- explicit failure reasons for unsupported or malformed input.

The geometry plan is data, not a public component state machine. Lifecycle
helpers consume it but do not move semantic state into the path.

### Node boundary

Keep NodeKind::Icon exactly as the static named-asset leaf. Add a separate
resolved NodeKind::IconGeometry-class leaf only after the pure format and
registry cards have passed. The exact enum spelling can be finalized in the
promoted architecture, but the separation is not optional:

- IconGeometry carries a compact, validated normalized geometry frame in the
  canonical coordinate space, with any required endpoint identity or schema
  version already resolved by shared composition.
- It does not carry an arbitrary SVG string, a consumer path, a provider
  registry, or a pair lookup that the backend must interpret.
- NodeStyle continues to carry resolved colour and layout information.
  Existing Node identity fields carry stable owner identity where the backend
  needs it.
- The shared renderer owns pair lookup, token resolution, and construction of
  the frame. GPUI owns path tessellation, paint, invalidation, and platform
  scheduling.
- A path geometry leaf is a reusable rendering capability, but its first
  constructor is restricted to the curated icon programme. A future non-icon
  consumer needs its own authority and evidence.

Do not overload NodeKind::Icon, encode path data in NodeAnimation, or make GPUI
load pair metadata from a hidden global registry. The node should describe the
resolved geometry the backend is allowed to paint.

## Canonical 24×24 Geometry Laws

The first programme scope is the current generated Lucide source:

1. Resolve aliases to their canonical manifest names before pair identity,
   digest, and registry generation. An alias is not a second geometry.
2. Require viewBox 0 0 24 24. Do not silently fit an off-grid provider or
   external icon into the box.
3. Start with the current Icon paint contract: fill none, currentColor,
   uniform stroke width 2, round caps, and round joins. Fill-only, multicolour,
   non-uniform-stroke, and paint-order semantics are outside this scope.
4. Lower only the generated primitive and path-command subset that the
   normalizer can represent deterministically. The first supported family is
   stroke geometry from path, line, polyline, polygon, circle, ellipse, and
   non-rounded rect forms, with supported path commands lowered into one
   internal segment representation.
5. Reject groups, transforms, masks, clips, filters, gradients, unsupported
   elements, malformed numbers, mismatched viewBoxes, rounded-rect semantics
   without a proven lowering, and any input whose effective renderer shape is
   not supported. Rejection is explicit; it is not a silent empty path.
6. Preserve a canonical endpoint representation separately from the cheaper
   sampled flight representation. Endpoint progress must resolve to the
   canonical endpoint, not an approximate last sample.
7. Quantize generated canonical numeric output with one versioned rule
   (recommended initial precision: four decimal places) so TypeScript, Rust,
   SSR, and generated assets do not diverge on floating-point tails.
8. Use a versioned fixed sampling count. Start the production pilot at 64
   arc-length samples per contour, with a maximum of eight contours and 512
   samples per endpoint. The feasibility spike's eight-point samples are
   evidence of the channel, not the production quality setting.

### Contour laws

- Preserve contour count and the open/closed signature in the first public
  registry. Do not split, merge, duplicate, or invent contours to make an
  incompatible pair drawable.
- Keep open-contour endpoints. Closed contours must not duplicate the first
  sample as a final point; closure remains an explicit flag.
- Preserve the 24×24 coordinate frame and the stroke treatment through the
  whole flight. A contour can move; its paint semantics cannot change.
- Keep winding and closure metadata even in the stroke-only first scope.
  Should fill morphing become valuable later, winding, fill rule, holes, and
  open-path fill behavior require a separate contract and evidence gate.
- A pair with unequal contour count or a mismatched closure signature is a
  negative fixture in this programme. The feasibility dossier's menu↔x and
  play↔pause examples remain useful rejection cases even when a general
  algorithm could duplicate geometry.

### Correspondence laws

- Build one plan for both directions. A reverse transition reuses the same
  pair identity and does not author a second, potentially divergent plan.
- Match contours deterministically using length, centroid, bounds, and sampled
  shape cost. Ties resolve by stable source/index order.
- For each candidate, evaluate both traversal directions. For closed contours,
  evaluate cyclic start offsets without duplicating the first point.
- Correspondence may choose ordering, reversal, and closed-loop offset. It must
  not apply an unrecorded global rotation, scale, reflection, or endpoint
  coordinate rewrite.
- Keep a diagnostic residual and quality record for every accepted pair.
  Numeric cost cannot replace human review: reject visible twisting,
  self-crossing, collapse, unintended rotation, contour duplication, or a
  semantic state change that reads as a broken icon.
- The registry accepts only a pair whose two endpoint assets, topology,
  correspondence plan, and intermediate review all pass. A deterministic
  answer is not automatically a good answer.

The recommended initial accepted set is intentionally small. Build an
8–12-pair candidate corpus from current names, covering compatible one-to-one
paths, multiple contours, open/closed cases, directional congruence,
asymmetry, and likely failures. Candidates such as chevron-left↔chevron-right
and plus↔x exercise positive paths; menu↔x, lock↔lock-open, volume-2↔volume-x,
and play↔pause exercise rejection or future-normalizer boundaries. The
candidate count is a test budget, not a promise that every candidate enters
the registry.

## Registry, Provenance, And Generation

Extend the current generated icon pipeline rather than building a second
catalogue. The proposed authoring surface is a small pair manifest alongside
the current icon manifest, for example
packages/core/src/icons/morph-pairs.json. The final path belongs to the
promoted architecture.

Each authored entry should contain:

- a stable pair ID;
- canonical endpoint names and the semantic direction/state label;
- an explicit registry status such as candidate, accepted, or rejected;
- the normalizer and pair-schema versions;
- generated topology and quality diagnostics; and
- a provenance/notice reference for both source assets and derived geometry.

The generator should:

1. read the pinned default-icons.json and resolved lucide-static version;
2. canonicalize aliases and reject missing, duplicate, self, or reversed
   duplicate entries;
3. normalize and validate both endpoints against the 24×24 stroke boundary;
4. derive correspondence and canonical/sample geometry deterministically;
5. emit TypeScript and Rust registry projections from the same source;
6. stamp source version, source-asset digests, normalizer version, schema
   version, topology signature, quality metrics, and derived-geometry digest;
7. carry the existing Lucide/Feather attribution and notice identity; and
8. fail closed on byte drift, orphaned entries, unapproved candidates, stale
   assets, duplicate pair identities, or output changes not explained by the
   input.

The existing icons:build and audit:icons surfaces can grow this responsibility
or a dedicated icon-morph build/audit selector can own it. The important
boundary is one manifest lineage and one deterministic check, not the command
name.

The generated registry may contain normalized endpoint and plan data if the
pilot budget allows it. It must not contain arbitrary consumer paths or
unreviewed raw d strings. Runtime planning may cache immutable endpoint data,
but the cache is an optimization over generated validated inputs, not a
second authority.

Provenance is part of the generated record, not a release note afterthought:

| Record | Required meaning |
| --- | --- |
| Source package/version | The exact icon data source, initially lucide-static 1.31.0. |
| Canonical endpoint names | Alias-resolved names used for lookup and pair identity. |
| Source asset digest | The bytes that were normalized. |
| Normalizer/schema version | The algorithm and data contract that produced the geometry. |
| Topology signature | Contour count, open/closed flags, primitive/paint limits, and sample count. |
| Quality record | Residuals, bounds/centroid diagnostics, rejection reasons, and visual-review state. |
| Derived geometry digest | The generated normalized endpoint/plan bytes used by each runtime. |
| Notice identity | The Lucide/Feather notice to retain with any published derived output. |

The Morphicons code licence does not settle the licence of icon data or
derivative geometry. Do not copy or vendor Morphicons source, tests, examples,
or assets. Publishing normalized Lucide/Feather-derived geometry requires an
explicit provenance and legal review before generated artifacts become a
public package surface.

## Lifecycle And Motion Semantics

Architecture 012 remains the sole lifecycle authority. The geometry role adds
only its rendering-specific outcome:

| Policy | Geometry result |
| --- | --- |
| full | Animate one bounded one-shot geometry transition using the existing interaction duration/easing role. Do not introduce a new spring or token in the pilot. |
| reduced | Commit semantic state immediately and snap to the canonical target. A short opacity continuity effect is possible only if separately justified; path geometry does not run. |
| frozen | Paint the latest canonical endpoint or explicitly controlled frame with no visual clock. Frozen is deterministic evidence policy, not reduced-motion evidence. |

The following laws apply in every active runtime:

- The authored initial endpoint paints immediately. There is no animation from
  an invented placeholder or a default icon the host did not author.
- The stable motion key is semantic owner identity plus the geometry role and
  channel. Tree position, render order, pair name alone, and a global recipe
  name are not enough.
- Repeated activation of the current endpoint is inert. It does not restart
  or allocate a new plan.
- A same-pair retarget snapshots the current sampled geometry, replaces the
  pending target, and does not queue phases. A reversible transition uses the
  remaining-progress duration law from architecture 012.
- A pair-identity change invalidates the old plan. It may interpolate only
  through a newly validated compatible plan; otherwise it paints the authored
  target through the contract's explicit unsupported outcome. It must not
  silently morph arbitrary names.
- Reduced and frozen changes cancel disallowed clocks immediately. Frozen
  leaves no scheduler, rAF, GPUI request, or cleanup handle alive.
- Unmount, abort, owner replacement, and destruction cancel all pending
  callbacks and prevent late DOM writes or native paints. Owner unmount removes
  the visual remnant; a policy abort settles the latest semantic endpoint.
- The display primitive emits no semantic completion callback. Parent controls
  own pressed, expanded, loading, status, and other meaning.
- The web shells keep one stable outer SVG and a deterministic initial path.
  Scheduling begins after mount; browser preference discovery never runs during
  SSR render. The native clock remains backend-owned.

For IconButton, the geometry remains inside the existing glyph slot. The
button retains label, focus, pressed, loading, disabled, busy, tooltip, and
activation behavior while the child path changes. Existing loading-to-spinner
and static fallback behavior is not rewritten by this programme.

## Evidence Plan

Evidence must separate structure, lifecycle, and visual admission. A green
structural suite cannot stand in for a presented native image.

| Layer | Required proof | What it cannot claim |
| --- | --- | --- |
| Geometry format | TypeScript/Rust vectors cover 24×24 normalization, supported commands, canonical quantization, contour signatures, correspondence, both directions, exact endpoints, malformed input, and explicit rejection. | It cannot claim a good visual pair from numeric cost alone. |
| Registry/codegen | Candidate and accepted pair inventory, aliases, source/version/digest checks, normalized output hashes, notice records, deterministic regeneration, orphan/duplicate/stale failure. | It cannot make an unreviewed pair acceptable. |
| Pure lifecycle | Full/reduced/frozen traces cover authored initial state, repeated target inertness, A→B→A interruption, A→B→C latest-state handling, pair replacement, controlled progress when actually needed, policy tightening, cancellation, and teardown. | It cannot prove browser or GPUI frame delivery. |
| Svelte and React structure | Real shells prove stable outer SVG, fixed box, currentColor/stroke treatment, accessible/decorative mode, no layout shift, SSR/client equality, hydration, focus retention, reduced/frozen behavior, and IconButton semantics. | It cannot prove native rendering or assistive technology in GPUI. |
| Shared Rust and node | Specs/render assertions prove pair lookup, resolved size/colour, stable identity, policy filtering, exact endpoint output, and the distinct geometry node. Tests prove existing NodeKind::Icon and IconProvider behavior stay unchanged. | A node assertion is not a presented native visual. |
| GPUI headless | The production backend builds and updates dynamic paths, records the supported geometry channel, preserves stroke/colour, handles invalidation, and leaves zero live clocks or post-teardown paints. Instrument planning, frame work, allocation, and concurrency. | The current test window does not provide display pixels or real frame pacing. |
| Web visual | A bounded browser scene captures canonical start, controlled midpoint, end, reverse, and frozen states. Svelte↔React exactness is checked where shared DOM/CSS makes it meaningful; endpoint geometry and layout are checked separately from pixels. | It cannot substitute for dedicated-lab native proof. |
| Native visual admission | After headless and structural checks pass, Poodle submits the named fixture meaning and geometry expectations to the accepted dedicated conformance lab. The lab runs one short-lived operator-approved GPUI process per fixture, captures only its exact non-activating window, and returns focus/permission/provenance receipts. Poodle applies the comparison criteria and owns the admission result. | It is not a Poodle worker selector, not default QA/CI, and not automatic active-cohort admission. |

The native visual step uses the dedicated lab's manual path:

- Poodle runs the fixture meaning, geometry-policy, and comparison work
  headlessly first, with exact-head review and green evidence required before
  a lab request;
- the lab owns one short-lived GPUI process per fixture, exact-window
  non-activating capture, clean close/exit, and the typed
  focus/permission/provenance receipt;
- the lab must reject application activation, key-window fallback, desktop or
  region capture, and any silent alternate transport;
- the lab's operator approval, WindowServer/Screen Recording limitation, and
  foreground result remain explicit in the retained run envelope;
- the path remains outside Poodle effigy qa, CI, release, and ordinary worker
  validation. Poodle workers do not run a local windowed/native-visual
  selector; and
- Poodle owns the comparison criteria and admission result after reviewing the
  lab evidence.

The g15.059–g15.061 documents remain historical evidence for the safety
invariants: non-activation, exact-window ownership, foreground preservation,
and fail-closed capture. They are not an execution route for this programme.
If the lab is unavailable, or its transport, focus, permission, provenance,
capture stability, endpoint fidelity, or foreground receipt is missing, IG-06
stops. Headless evidence remains valid but is not upgraded to a visual pass.

## Initial Budgets

These are programme admission targets, not measurements already established by
the feasibility spike. The first implementation cards should measure them on
the repository's supported web and GPUI environments.

| Budget | Initial target | Stop or return condition |
| --- | --- | --- |
| Candidate corpus | 8–12 current-manifest candidate pairs; accepted count may be lower. | Do not widen the catalogue to make the first result look complete. |
| Normalized topology | Exactly 24×24; at most 8 contours and 64 samples per contour; at most 512 samples per endpoint. | Any need for topology invention, off-grid fitting, or unbounded input starts a new scope. |
| Generated pair payload | At most 16 KiB of normalized endpoint/plan payload per pair, excluding notices, for the pilot. | Exceeding the cap requires a measured storage/quality decision; do not compress or drop fields silently. |
| Frame work | Zero hot-path allocations after plan creation; p95 geometry update target ≤1 ms per active instance and ≤4 ms for four concurrent instances at a 60 Hz reference frame. | Misses require profiling, a narrower supported shape, or a programme decision before admission. |
| Plan work | p95 cold plan target ≤2 ms for a pilot pair on each active web/native core. | A slower planner cannot be hidden behind mount-time scheduling or a web-only path. |
| Scheduling | One clock per semantic owner/role/channel, at most one frame request per host turn, and no queue of pending phases. | Duplicate clocks, stale callbacks, or queued targets fail lifecycle acceptance. |
| Motion duration | Reuse the existing bounded interaction duration/easing role; stay within the current 120/180/260 ms semantic duration family. | A new spring family or token requires a separate motion decision. |
| Layout and semantics | Zero endpoint layout shift; one stable outer visual root; zero per-frame accessibility/name/focus changes. | Any geometry solution that changes hit testing, naming, or layout is out of scope. |
| Visual repetition | Two agreeing captures for each native fixture/state and stable controlled web captures. | A single successful capture cannot prove deterministic visual output. |
| Provenance | Every accepted pair has complete source, digest, schema, quality, and notice records. | Any incomplete or ambiguous source record blocks generated publication. |

The pilot does not claim that four is a universal concurrency limit. It is the
small workload that makes a first frame-budget result comparable and reviewable.
Higher concurrency, large icon walls, fill geometry, or application-supplied
sets need new measurements and a new budget decision.

## Proposed Staged Cards

The orchestrator should promote these as one dependency chain. They are
planning candidates, not ready implementation cards in this triage branch.

Dependency chain: IG-01 → IG-02 → IG-03 → {IG-04, IG-05} → IG-06 → IG-07

### IG-01 — Canonical geometry format and normalizer

Define the internal segment, contour, sampled-frame, quantization, failure,
and topology shapes. Implement or prove the supported generated Lucide subset
in pure TypeScript and Rust against shared golden vectors. Include positive
and negative fixtures, fixed 24×24 normalization, both traversal directions,
closed-loop offsets, exact endpoints, and the initial topology/payload caps.

No public component, provider widening, generated catalogue, node variant, or
production GPUI route is part of this card.

### IG-02 — Curated pair registry and provenance generator

Author the 8–12 candidate pairs, canonicalize aliases, derive approved
correspondence metadata, emit web/native projections, preserve attribution, and
add deterministic build/audit checks. The output must distinguish candidate,
rejected, and accepted entries. Rejected fixtures remain useful evidence and
do not become runtime fallback pairs.

This card owns source and derived digests, schema/version stamps, quality
diagnostics, notice records, stale/orphan detection, and the generated payload
budget. It does not admit a public API.

### IG-03 — Pure plan, interpolation, and lifecycle

Build the framework-free plan and lifecycle utility over the validated registry.
Prove full/reduced/frozen, stable semantic keys, exact initial/endpoints,
latest-state-wins retargeting, interruption, cancellation, deterministic
frozen output, SSR-safe initial values, and zero late writes. Keep pair
selection separate from component semantics and do not add a second motion
policy.

### IG-04 — Geometry node, shared Rust composition, and GPUI headless route

Add the distinct resolved geometry node only after IG-01 through IG-03. Make
shared Rust resolve pair data, size, colour, stable identity, and policy;
make GPUI consume the resolved frame through PathBuilder/paint_path; and prove
production invalidation, supported stroke paint, teardown, and budgets in a
headless probe.

Keep NodeKind::Icon, IconProvider, and the existing generic animation channels
unchanged. This card is a native capability gate, not native admission.

### IG-05 — Web shells and structural/visual evidence

Build thin Svelte and React shells over the same pure plan. Keep the outer
visual root stable, preserve Icon and IconButton semantics, prove SSR/hydration,
reduced/frozen behavior, focus and layout invariants, and capture controlled
browser endpoint/midpoint/reversal fixtures. Keep the initial route private or
otherwise non-public until IG-06 and the promoted contract gate pass.

### IG-06 — Curated visual review and dedicated conformance-lab native proof

Poodle reviews every accepted candidate's intermediate frames and both
directions, owning the fixture meaning, geometry policy, comparison criteria,
and admission result. After code review and green headless evidence, submit
the named fixture inputs to the accepted dedicated conformance lab. The lab
owns the short-lived one-process-per-fixture capture, exact-window
non-activating transport, clean teardown, and typed focus/permission/provenance
receipt. Capture endpoint, midpoint, reverse, frozen, and teardown fixtures;
record environment, foreground invariance, repeat stability, endpoint fidelity,
and visual rejections in the lab evidence consumed by Poodle.

The dedicated lab is the sole native visual transport. Poodle workers do not
run a local windowed/native-visual selector. If the lab is unavailable, or the
lab cannot return complete transport and provenance evidence, IG-06 stops and
no public IconMorph admission occurs. The g15.059–g15.061 window-capture work
is historical safety evidence only, not a second execution route.

Failure keeps the geometry capability unadmitted. It does not authorize a
web-only public contract or a silent static substitution.

### IG-07 — Separate public contract and curated primitive admission

Only after IG-06 passes should the orchestrator promote a separate IconMorph
contract, shared Rust spec, public web/native implementation, curated
specimens, parity evidence, and generated package surfaces. The contract must
keep Icon static, make unsupported pairs explicit, preserve architecture 012,
and name the GPUI visual approval result. Jetstream remains deferred.

Future pair additions are their own reviewed registry batches. They do not
silently alter an already-approved pair's geometry or semantic identity.

## Review Oracles

| Invariant | Smallest adversarial case | Required result |
| --- | --- | --- |
| Existing Icon stays static | Change an Icon name under full motion policy. | No morph clock, no node-shape change, and no new Icon prop. |
| Provider content is not implicitly morphable | A provider supplies a valid-looking unsupported node tag or off-grid viewBox. | The static provider path remains outside the morph registry, or the future contract returns its explicit validation result. No silent morph. |
| Canonical grid is exact | An endpoint reports 23×24, a transform, or a non-uniform stroke. | Registry rejection with a reason; no implicit fit or paint rewrite. |
| Topology is preserved | menu↔x or play↔pause has unequal contours/closure semantics. | Candidate rejection; no duplicated contour or invented topology in the first registry. |
| Correspondence is deterministic | Reverse a pair or rotate a closed contour's start point. | One stable plan, stable tie-break, exact endpoint output, and no unrecorded endpoint transform. |
| Numeric output is reproducible | Generate TS and Rust artifacts twice on separate runs. | Identical generated bytes/digests under the same source and schema versions. |
| Pair provenance is complete | Change one source asset or Lucide version without changing the pair file. | The audit fails on source/version/digest drift. |
| Current state owns the flight | A→B→A during an active transition. | Rebase from current sampled geometry, latest target wins, no queued phase or visible jump. |
| Pair replacement is bounded | A live pair is replaced by an unrelated pair ID. | Old plan is cancelled; the new authored target is validated or explicitly rejected. No arbitrary name morph. |
| Motion policy is authoritative | Full transition is tightened to reduced and then frozen. | Geometry clock stops, semantic state stays immediate, reduced/frozen paint their contracted endpoints, and no handle remains. |
| Teardown is exact | Unmount during a scheduled web or GPUI frame. | No late DOM write, native paint, callback, or retained scheduler entry. |
| Semantics stay on the owner | A pressed/loading IconButton changes glyph during focus. | Button label, busy/pressed/disabled/focus behavior is unchanged; the path is decorative paint. |
| Native evidence is not overstated | Headless GPUI path construction passes but the dedicated lab capture is unavailable. | Native admission remains blocked; no headless-pixel or active-cohort visual claim is recorded. |
| Dedicated-lab capture is safe | The lab attempts activation, broad capture, or produces disagreeing repeats or incomplete receipts. | Stop the visual gate; do not substitute a silent transport or waive the failure. |
| Public scope remains curated | A request adds raw d or the full provider set to the first public input. | Return to planning; do not widen the pre-v1 contract. |

## Alternatives Not Selected

| Alternative | Disposition |
| --- | --- |
| Add an animation prop or automatic name-change morph to Icon | Rejected. It breaks the static Icon contract and makes provider content an implicit geometry API. |
| Accept arbitrary raw d or IconNodes | Rejected for the first boundary. It requires a much larger parser, topology, security, provenance, SSR, and quality contract. |
| Let arbitrary contour counts duplicate or split geometry | Rejected for the first registry. Deterministic output is not sufficient visual or semantic quality. |
| Put path data into AnimProperty or NodeAnimation keyframes | Rejected. Animation declarations carry scalar channels; geometry is a resolved content payload and must not churn large strings through the generic channel. |
| Make GPUI load pair data from a global registry | Rejected. It hides shared composition authority and makes native behavior diverge from web/Rust evidence. |
| Adopt Morphicons as a runtime dependency | Rejected for the first implementation. It is useful pinned research, but adds web ownership and no GPUI/node solution. |
| Generate precomputed frame assets | Held as a bounded native fallback experiment only. It risks asset explosion and awkward interruption/reversal and is not the default geometry route. |
| Ship a web-only public primitive with a native gap | Rejected by the settled dedicated-lab native admission boundary and active-cohort rule. A private research shell may collect web evidence; public admission waits. |
| Add fill, arbitrary viewBox fitting, transforms, masks, or multicolour morphs | Deferred to separate capability decisions with their own topology, paint, provenance, and visual gates. |

## Explicit Non-Goals

- Editing production source, contracts, architecture, roadmaps, parity
  documents, generated files, or public exports in this planning packet.
- Changing Icon, IconProvider, IconButton, Spinner, or existing default icon
  resolution.
- Admitting a public IconMorph or any public raw geometry input.
- Treating a custom provider set as a morph catalogue.
- Copying or vendoring Morphicons source, tests, examples, or assets.
- Adding a Morphicons runtime dependency or a second icon catalogue.
- Adding a generic animation framework, generic path/canvas/filter/gradient/3D
  system, or a hidden global geometry clock.
- Supporting fill-only, multicolour, transformed, masked, clipped,
  non-uniform-stroke, off-grid, or arbitrary consumer geometry in the first
  programme.
- Claiming native assistive-technology support from GPUI node metadata.
- Running a local windowed/native-visual selector in a Poodle worker, or
  moving the dedicated lab's window capture into default QA, CI, release, or
  publication.
- Admitting Jetstream or creating a per-component Jetstream exception.
- Treating a static endpoint swap or cross-fade as equivalent to a geometry
  morph. They remain the current safe fallback where an existing contract
  permits them.
- Expanding the first pair set because a candidate failed quality review.

## Stop Conditions

Stop the relevant card and return to the orchestrator when:

- the work needs a change to Icon, IconProvider, IconButton, the default
  registry, or a public raw geometry input;
- the normalizer must accept arbitrary provider data, unbounded SVG syntax,
  fill/winding semantics, topology invention, or implicit viewBox fitting;
- the design puts geometry strings or large buffers into NodeAnimation, or
  requires GPUI to own semantic pair lookup;
- TypeScript and Rust cannot share deterministic vectors, endpoint bytes,
  registry metadata, or lifecycle outcomes;
- generated output exceeds the payload/frame/plan budgets without an explicit
  re-budget decision;
- the web or native path allocates per frame, creates duplicate clocks,
  queues stale targets, leaks a handle, writes after teardown, or changes
  layout/hit-testing/accessibility state;
- the GPUI dynamic path route cannot render the resolved frame or cannot
  invalidate and tear down cleanly at the pinned runtime;
- structural evidence passes only by borrowing another runtime's result or
  silently classifying a native capability gap as parity;
- endpoint, stroke, colour, contour, or intermediate visual review fails;
- provenance, source digest, attribution, or derivative-geometry legal review
  is incomplete;
- the dedicated conformance lab is unavailable, cannot consume the named
  fixture, or cannot return a complete exact-window,
  focus/permission/provenance receipt;
- the lab's native proof needs activation, desktop or region capture, a silent
  fallback, disagreeing repeats, or cannot show the intended endpoint in its
  own window;
- a Poodle worker is asked to run a local windowed/native-visual selector;
- a public web result is proposed before dedicated-lab native approval;
- a new card asks for Jetstream, release/publication, workflow edits, or
  downstream consumer changes.

## Proposed Canonical Destinations

This packet is triage input. The orchestrator chooses and promotes the final
canonical split after re-reading current main:

| Meaning | Proposed destination after promotion |
| --- | --- |
| Renderer-neutral geometry ownership, node boundary, paint limits, and relationship to architecture 012 | New architecture record such as docs/architecture/013-icon-geometry-capability.md; architecture 012 remains the lifecycle authority. |
| Public semantic primitive, if and only if admission passes | A separate docs/contracts/components/icon-morph.md; do not edit Icon or IconProvider to add morph inputs. |
| Pure web geometry and lifecycle utility | packages/core icon-geometry modules with paired Rust vectors/logic where the active-cohort contract requires it. |
| Pair authoring, generated registry, provenance, and drift gate | A manifest beside the current icon source plus the existing icon-generation/audit pipeline or a dedicated named extension. |
| Renderer-neutral resolved geometry payload | packages/contracts/node; shared composition and token resolution in packages/render. |
| GPUI dynamic path interpretation and headless proof | packages/gpui/node-backend and focused preview/probe surfaces; no Poodle windowed capture route. |
| Native visual transport and exact-window provenance receipt | The dedicated internal `poodle-conformance-lab` repository defined by the accepted conformance-lab architecture; Poodle consumes its evidence without a reverse dependency. |
| Web shells and browser proof | Svelte/React component and preview surfaces only after the public contract gate. |
| Parity, specimens, and visual receipts | A new icon-morph parity record and bounded fixtures after admission; no established Icon parity cell moves for research. |
| Sequencing and implementation evidence | A promoted geometry programme card with child cards IG-01 through IG-07, plus one execution log per implementation batch. |
| Current open queue | Keep the post-motion research queue's icon item and this packet until the orchestrator accepts and promotes the meaning; remove or split them only in that promotion batch. |

The orchestrator must re-check the packet against current main before promotion.
Merging this triage note would be intake, not architecture approval, contract
readiness, native admission, or implementation authorization.

## Evidence Used

- The committed planning handoff:
  docs/handoffs/20260901-230405-icon-geometry-programme-planning.md
- docs/research/value-tracks/icon-morphing.md
- docs/research/value-tracks/icon-morphing-native-feasibility.md
- docs/triage/20260901-230407-conformance-lab-architecture.md
- docs/architecture/001-poodle-system-shape.md
- docs/architecture/006-headless-core-and-machine-model.md
- docs/architecture/010-native-presentation-construction-context.md
- docs/architecture/012-semantic-motion-policy.md
- docs/contracts/001-working-rules.md
- docs/contracts/003-native-accessibility.md
- docs/contracts/components/icon.md
- docs/contracts/components/icon-provider.md
- docs/contracts/components/icon-button.md
- docs/parity/icon.md
- docs/parity/icon-provider.md
- docs/parity/icon-button.md
- packages/core/src/icons/default-icons.json
- scripts/build-default-icons.ts
- packages/contracts/components/src/icon.rs
- packages/contracts/node/src/lib.rs
- packages/render/src/icon.rs
- packages/gpui/node-backend/src/lib.rs
- docs/triage/20260901-125758-post-motion-research-queue.md
- docs/roadmaps/g15/059-gpui-cratesio-recovery.md
- docs/roadmaps/g15/060-v022-release-candidate.md
- docs/roadmaps/g15/061-v022-release-certification.md
- docs/roadmaps/g15/047-primitive-visual-comparison.md

The g15.059–g15.061 records are retained as historical safety evidence for
non-activation, exact-window ownership, foreground preservation, and
fail-closed capture. They are not a second execution route for IG-06.
