# Value Track: Poodle-owned icon morphing

Status: research complete; promotion-ready for an operator architecture decision
Created: 2026-08-31
Checked: 2026-08-31
Track: Poodle-owned icon morphing
Origin: [`docs/triage/20260831-155152-icon-morphing-research.md`](../../triage/20260831-155152-icon-morphing-research.md)
Primary lead (live/mutable): [Morphicons “How it works”](https://www.morphicons.com/#how)

This is point-in-time research, not a component contract or an implementation
plan. The current contract remains authoritative. Do not treat this dossier as
permission to widen `Icon`, add a dependency, or admit Jetstream.

## Evidence labels

- **[VF] Verified fact** — read from the cited repository, official standard,
  official documentation, or upstream source.
- **[SAC] Source-author claim** — a performance, compatibility, or product claim
  made by Morphicons or another project about its own implementation. It is not
  an independent benchmark.
- **[WI] Worker inference** — a conclusion drawn from the cited facts and the
  current Poodle contracts.

## Executive summary

Morphicons is a substantive reference implementation, not a simple SVG
`d`-string tween. Its published core parses path data, lowers supported SVG
primitives to cubic paths, resamples by arc length, matches subpaths, resolves
orientation and closed-loop offsets, aligns with 2D Procrustes transforms, and
interpolates in polar coordinates. Its DOM driver then serializes a sampled
polyline during flight, snaps to an exact canonical endpoint, shares one
`requestAnimationFrame` scheduler, and re-plans from the current intermediate
shape when interrupted. These facts are visible in the [official core source
files](https://github.com/guillermolg00/morphicons/tree/38d2a7221633a453eeafebd872ee3649b9274b22/src/core) and the
[DOM driver](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts).

The “any icon” claim has material boundaries. The implementation is for
stroke-oriented icons on a common grid. It accepts an `IconNode` or raw `d`, but
the current normalizer handles seven primitive families and rejects groups,
transforms, unsupported elements, and fill-only geometry. Unequal subpath counts
are made drawable by duplicating a matched subpath; that is deterministic, but
not a guarantee of semantic or visual quality. [VF] from the
[normalizer](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/normalize.ts),
[planner](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/plan.ts),
and the project’s [compatibility notes](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/README.md#icon-library-compatibility).

Poodle’s Lucide base is unusually favourable for a constrained morph: the
current contract fixes a 24×24 viewBox, `fill="none"`, `currentColor`, uniform
stroke width, round caps, and round joins. But Poodle’s public icon input is
broader than that effective renderer support: `IconNode` attributes are open,
while the Svelte and React renderers currently emit only seven SVG tag types.
Custom provider content therefore cannot be assumed morphable. [VF] from the
[Icon contract](../../contracts/components/icon.md#L106-L127),
[core icon types](../../../packages/core/src/icons/types.ts#L1-L17), and the
[Svelte](../../../packages/svelte/components/src/Icon.svelte#L39-L70) and
[React](../../../packages/react/components/src/Icon.tsx#L17-L50) renderers.

Recommendation: keep `Icon` static and out of scope for animation. If Poodle
wants true geometry morphing, promote a separate, opt-in `IconMorph`-type
primitive backed by a pure, framework-free plan/interpolation utility and a
curated, generated morph-pair registry. Do not make arbitrary raw SVG morphing
the first public contract. The registry gives Poodle a reviewable quality
boundary, preserves the existing Lucide generation/licence pipeline, and leaves
room for Svelte, React, and shared Rust composition to consume one semantic
contract. This is [WI], contingent on the unresolved operator decisions below.

The decisive blocker is native feasibility. The current shared node only names
an icon asset; GPUI’s backend loads `assets/icons/<name>.svg` and presently has
generic opacity plus SVG rotation animation, not mutable path geometry. A real
active-cohort morph therefore needs a new renderer-neutral geometry capability
and a GPUI dynamic-path proof before it can be called complete. Jetstream stays
deferred under the repository rules. [VF] from
[`NodeKind::Icon`](../../../packages/contracts/node/src/lib.rs#L80-L90), the
[GPUI icon branch](../../../packages/gpui/node-backend/src/lib.rs#L431-L440),
[GPUI SVG animation path](../../../packages/gpui/node-backend/src/lib.rs#L481-L527),
and the [active-cohort rule](../../contracts/001-working-rules.md#L64-L70).

## Method and source inventory

### Method

1. Read the repository instructions and research routing before inspecting
   implementation: [`AGENTS.md`](../../../AGENTS.md),
   [`docs/README.md`](../../README.md),
   [`docs/research/README.md`](../README.md), and the originating
   [triage note](../../triage/20260831-155152-icon-morphing-research.md).
2. Read the active system, token, headless-core, native-presentation, working-
   rules, and native-accessibility documents, then audited the icon contracts,
   source, tests, previews, generated assets, and Rust node/backend path.
3. Inspected Morphicons’ official site, repository README, package metadata,
   licence, changelog, core source, bindings, and tests. Source and asset text
   was not copied into Poodle.
4. Checked the relevant SVG, accessibility, reduced-motion, browser hydration,
   animation-frame, Lucide, and GPUI primary/official sources. External URLs in
   this dossier were accessed and checked on 2026-08-31 unless a different date
   is stated.
   The inspected Morphicons release is tag `v1.7.1` at commit
   `38d2a7221633a453eeafebd872ee3649b9274b22`; all release/code citations below
   are pinned to that revision. The live product page is intentionally mutable.
5. Separated facts, source-author claims, and worker inferences. No code spike
   or generated artifact was required to establish the recommendation.

### External source inventory

| ID | Direct source | Use | Checked |
|---|---|---|---|
| M1 | [Morphicons live/mutable technique page](https://www.morphicons.com/#how) | Maintainer’s public overview, examples, stated constraints, size and runtime claims, and attribution links. | 2026-08-31 |
| M2 | [Morphicons README at v1.7.1](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/README.md) | Public API, supported formats, lifecycle, reduced motion, SSR, compatibility, and architecture claims. | 2026-08-31 |
| M3 | [Morphicons core source at v1.7.1](https://github.com/guillermolg00/morphicons/tree/38d2a7221633a453eeafebd872ee3649b9274b22/src/core) and direct files for [parse](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/parse.ts), [normalize](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/normalize.ts), [resample](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/resample.ts), [plan](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/plan.ts), [interpolate](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/interpolate.ts), [serialize](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/serialize.ts), and [spring](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/spring.ts) | Actual algorithm, topology handling, numeric choices, output format, and bounds. | 2026-08-31 |
| M4 | Morphicons [DOM driver](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts), [controller](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/controller.ts), [React binding](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/react/index.tsx), and [Svelte binding](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/svelte/MorphIcon.svelte) | Per-frame mutation, cancellation, interruption, SSR, and binding lifecycle. | 2026-08-31 |
| M5 | Morphicons [package metadata](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/package.json), [MIT licence](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/LICENSE), and [changelog](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/CHANGELOG.md) | Current published version, exports, optional peers, size gates, licence, and release history. | 2026-08-31; package/changelog report 1.7.1 on 2026-08-28 |
| S1 | Lucide [licence](https://github.com/lucide-icons/lucide/blob/main/LICENSE), [icon data guide](https://github.com/lucide-icons/lucide/blob/main/docs/guide/packages/icons.md), [package metadata](https://github.com/lucide-icons/lucide/blob/main/packages/lucide/package.json), and [contributing pipeline](https://github.com/lucide-icons/lucide/blob/main/CONTRIBUTING.md) | Authoritative icon data shape, tree-shaking model, source-to-generated flow, and ISC/Feather-derived notices. | 2026-08-31 |
| S2 | W3C [SVG paths](https://www.w3.org/TR/SVG2/paths.html) and [painting](https://www.w3.org/TR/SVG2/painting.html) | Native path interpolation, subpaths, closepath, fill, and stroke semantics. | 2026-08-31 |
| S3 | MDN [fill-rule](https://developer.mozilla.org/en-US/docs/Web/CSS/fill-rule), [fill](https://developer.mozilla.org/docs/Web/CSS/fill), [preserveAspectRatio](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/preserveAspectRatio), [SVG in HTML](https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_in_HTML), [SVG title](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title), [ARIA img](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/img_role), [prefers-reduced-motion](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion), and [requestAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame) | Browser geometry, naming, user preference, and frame scheduling details. | 2026-08-31 |
| S4 | W3C WAI [Animation from Interactions](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html) | Accessibility guidance for disabling or reducing non-essential interaction animation. | 2026-08-31 |
| S5 | React [hydrateRoot](https://react.dev/reference/react-dom/client/hydrateRoot) | Server/client initial-output equality and hydration mismatch constraints. | 2026-08-31 |
| N1 | GPUI 0.2.2 [animation source](https://docs.rs/gpui/0.2.2/src/gpui/elements/animation.rs.html) and [Transformation API](https://docs.rs/gpui/0.2.2/gpui/struct.Transformation.html) | Official native animation and SVG transform capability reference. | 2026-08-31 |

Morphicons’ repository and package are MIT-licensed according to the
[upstream licence](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/LICENSE)
and `package.json`. That licence covers Morphicons code. It does not settle the
licence or attribution obligations of icon data fed into it. Poodle’s current
Lucide/Feather notices remain the relevant local record in
[`THIRD_PARTY_NOTICES.md`](../../../THIRD_PARTY_NOTICES.md#L1-L16) and
[`packages/render/assets/icons/LICENSE.txt`](../../../packages/render/assets/icons/LICENSE.txt#L1-L35).
Any derived normalized-pair artifact needs an explicit provenance and legal
review decision; this dossier makes no licence conclusion about derivative
geometry. [VF/WI]

## Current Poodle audit

### Authority and ownership

Poodle’s system shape requires one semantic contract across web and native. The
active web pair is shared core plus Svelte/React shells; the native path is
shared `poodle-render` composition into `poodle-node` and a backend. Svelte,
React, shared Rust composition, and GPUI are the active completion cohort;
Jetstream is deferred. [VF, `001 Poodle System Shape`](../../architecture/001-poodle-system-shape.md#L47-L88)
and [working rules](../../contracts/001-working-rules.md#L51-L83).

Poodle owns reusable primitives and composites. Applications own workflow and
domain vocabulary; DAW-specific widgets remain outside Poodle. [VF,
working rules](../../contracts/001-working-rules.md#L128-L135). A general icon
morph can fit Poodle only as a reusable visual primitive. A product-specific
transition does not become a Poodle component merely because it uses an icon.

### Existing `Icon` contract and implementations

The current `Icon` is deliberately static. Its contract covers five sizes,
semantic size roles, registry resolution, accessible/decorative modes, and
currentColor inheritance; “animated icons” is explicitly out of scope. [VF,
[`icon.md`](../../contracts/components/icon.md#L6-L15)]. It has no internal
state and emits no events. [VF, [`icon.md`](../../contracts/components/icon.md#L40-L70)].

The public web inputs are `IconNodes | string | null`, deprecated `name`,
explicit size, `sizeRole` defaulting to `chrome`, and `ariaLabel`. [VF,
[`icon.md`](../../contracts/components/icon.md#L28-L38)]. The Svelte and React
shells emit a stable 24×24 SVG with `fill="none"`, `stroke="currentColor"`,
stroke width 2, round caps, and round joins. Without a label the root is
presentational and hidden; with a label it is an `img`. [VF,
[Svelte `Icon.svelte`](../../../packages/svelte/components/src/Icon.svelte#L39-L70),
[React `Icon.tsx`](../../../packages/react/components/src/Icon.tsx#L32-L50)].

`IconProvider` is a registry/context boundary, not an animation boundary. It
merges application content over Poodle’s scoped default Lucide set on the web;
the GPUI implementation is a child passthrough with a host-owned registry and
does not provide nested scoped inheritance. [VF,
[`icon-provider.md`](../../contracts/components/icon-provider.md#L6-L15),
[`icon-provider.md`](../../contracts/components/icon-provider.md#L105-L138)].

### Icon data and generated pipeline

The committed manifest currently contains 92 canonical names and 16 aliases,
108 supported names in total, pinned to `lucide-static` 1.31.0. [VF,
[`default-icons.json`](../../../packages/core/src/icons/default-icons.json#L1-L115)].
The generator verifies the resolved Lucide version, sorted names and aliases,
alias targets, and catalogue membership. It emits one TypeScript module and one
Rust SVG asset per supported name, plus generated indexes; `--check` detects
both byte drift and stale files. [VF,
[`build-default-icons.ts`](../../../scripts/build-default-icons.ts#L19-L68),
[`build-default-icons.ts`](../../../scripts/build-default-icons.ts#L90-L205)].

The generated asset contract is a strong morphing input boundary: Poodle owns a
fixed manifest, one source version, one 24×24 SVG representation, and explicit
licence headers. A morph-pair registry should extend this pipeline rather than
introduce a second icon catalogue. [WI]

Core `IconNodes` are Lucide-shaped tuples with open string tag names and string,
number, or undefined attributes. Resolution supports direct nodes, provider
sets, default names, aliases, and a default error glyph for unknown names. [VF,
[`types.ts`](../../../packages/core/src/icons/types.ts#L1-L17),
[`icons/index.ts`](../../../packages/core/src/icons/index.ts#L1-L37)]. The actual
Svelte and React renderers whitelist `path`, `circle`, `rect`, `line`,
`polyline`, `polygon`, and `ellipse`. [VF, renderer links above.] This is a
real constraint for arbitrary providers: a type-level custom node can be
accepted while an unsupported rendered tag is ignored. A future morph input
must validate effective geometry, not only the TypeScript shape. [WI]

### Existing motion, accessibility, and tests

The current icon CSS provides display, fixed size variables, vertical alignment,
and flex-shrink only; it has no icon morph or transition rule. [VF,
[`icon.css`](../../../packages/core/src/styles/icon.css#L1-L30)]. `IconButton`
requires its own accessible label, keeps focus/pressed/loading semantics on the
button, and replaces its glyph with the shared `Spinner` while loading. [VF,
[`icon-button.md`](../../contracts/components/icon-button.md#L135-L175)]. This
means a future icon transition must not carry or announce the button state by
itself.

The core icon tests cover the 108-name set, alias identity, direct/provider/
default lookup priority, selected-set deduplication, and unknown-name failure.
[VF, [`icons.test.ts`](../../../packages/core/test/icons.test.ts#L1-L83)].
`Spinner` is the only nearby animated icon use: the shared Rust recipe emits a
named `Node::icon("spinner", ...)` with a rotate animation, while grid/dot
variants animate opacity or remain static. [VF,
[`spinner.rs`](../../../packages/render/src/spinner.rs#L1-L45)]. This is a
useful precedent for stable animation keys and backend-owned clocks, but it is
not evidence that arbitrary path geometry is supported.

One separate parity issue surfaced during the audit: the web `Icon` contract
documents `sizeRole` default `chrome`, while `IconSpec::new` initializes
`SemanticControlSizeRole::Control` and an explicit `Md` size. [VF,
[`icon.md`](../../contracts/components/icon.md#L28-L38),
[`icon.rs`](../../../packages/contracts/components/src/icon.rs#L40-L70)].
This dossier does not repair or reinterpret that pre-existing mismatch. It
should be resolved by the owner of the icon contract before a new icon-bearing
contract relies on native default sizing. [WI]

### Shared Rust and GPUI path

`IconSpec` carries a name, size, optional accessible label, and size role. The
shared renderer resolves the size token, applies the native icon colour token,
creates `Node::icon(name, size)`, and carries a label into `poodle-node`.
[VF, [`icon.rs`](../../../packages/contracts/components/src/icon.rs#L40-L114)
and [`render/src/icon.rs`](../../../packages/render/src/icon.rs#L1-L29)].

`poodle-node` declares `NodeKind::Icon { name, size }`. Its animation vocabulary
currently contains opacity, rotation, translation, and scale; no path geometry,
path buffer, or SVG `d` channel exists. The node module explicitly leaves
animation driving and icon rasterisation to the backend. [VF,
[`node/lib.rs`](../../../packages/contracts/node/src/lib.rs#L10-L13),
[`node/lib.rs`](../../../packages/contracts/node/src/lib.rs#L80-L90),
[`node/lib.rs`](../../../packages/contracts/node/src/lib.rs#L796-L865),
[`node/lib.rs`](../../../packages/contracts/node/src/lib.rs#L976-L984)].

The GPUI backend loads a named SVG file from `assets/icons/`. For SVG leaves it
can apply GPUI’s `Transformation::rotate`; generic elements get opacity
animation. The backend comments that other transform channels remain
unavailable on GPUI 0.2.2, and it does not map `NodeA11y` because GPUI exposes no
accessibility attributes. [VF,
[`node-backend/src/lib.rs`](../../../packages/gpui/node-backend/src/lib.rs#L431-L440),
[`node-backend/src/lib.rs`](../../../packages/gpui/node-backend/src/lib.rs#L481-L527),
[`node-backend/src/lib.rs`](../../../packages/gpui/node-backend/src/lib.rs#L891-L922)].
The preview pins `gpui = "0.2.2"`. [VF,
[`Cargo.toml`](../../../packages/gpui/preview/Cargo.toml#L37-L52)].

Native feasibility is therefore an architectural question, not a missing prop.
A GPUI spike would need to prove an efficient mutable path-builder/paint path or
another dynamic SVG geometry route. Precomputed frame assets would avoid that
backend capability but introduce asset size, loading, and quality costs. A
web-only implementation would be a capability gap in the active cohort, not an
accepted parity result. [WI, grounded in the active-cohort rule above.] The
Jetstream backend is intentionally not an admission target for this research;
its current status remains the program-level deferral.

## Detailed findings

### 1. Morphicons’ actual technique

The following is a high-level description of the inspected implementation; no
Morphicons source or asset is copied here.

| Stage | Observed technique | Poodle implication |
|---|---|---|
| Input | Structural `IconNode` tuples or a raw path `d`; the input type is DOM-free. | A pure core could consume Poodle’s generated node data without coupling to Svelte or React. Public Poodle inputs should still be narrower than the internal structural type. |
| Parse | Absolute subpaths are reconstructed from `M/L/H/V/C/S/Q/T/A/Z`; relative commands, shorthand reflection, repeated pairs, packed arc flags, and scientific notation are handled. | A parser is real domain logic. It needs malformed-input errors and bounded work, not a string interpolation helper. [VF, [parser](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/parse.ts)] |
| Normalize | Lines and quadratics become cubics; arcs use SVG endpoint-to-centre conversion and ≤90° cubic slices; circles, ellipses, rectangles, polylines, and polygons become cubic paths. | Normalization must preserve closure and stroke topology, and must reject unsupported fill/transform/group semantics. [VF, [normalizer](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/normalize.ts)] |
| Resample | Cubic paths are sampled at equal arc-length intervals, default `N=64`; segment boundaries, endpoints, and detected corners are anchored. Closed loops avoid duplicating the first sample and leave circular start-point freedom for matching. | Fixed sample counts make interpolation possible, but create approximation error and a quality/CPU tradeoff. [VF, [resampler](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/resample.ts)] |
| Match | Subpaths are paired using centroid plus length cost. Equal counts use exhaustive permutations up to eight subpaths, then greedy matching; unequal counts use a bounded surjection and duplicate a nearest shape where necessary. | “Any pair” includes deterministic compromises. Registry metadata should record pair quality and reject visually bad matches rather than hide all compromises. [VF, [planner](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/plan.ts)] |
| Align | Both traversal directions and closed-loop circular offsets are tried. A closed-form 2D Procrustes similarity chooses rotation/scale; a minimal-rotation tie-break resolves symmetric cases. A global-hybrid path keeps congruent multi-subpath icons coherent. | Rotation should be derived, not hand-authored for every pair. A quality review still matters for semantically unrelated shapes. [VF, [planner](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/plan.ts)] |
| Interpolate | Scale is log-linear, angle is linear, centroids are translated, and residual points are blended in the aligned frame. Exact endpoints are supported; spring overshoot extrapolates progress. | A lower-level utility can separate geometry planning from visual component bindings. [VF, [interpolator](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/interpolate.ts)] |
| Serialize | In flight, sampled points become an `M/L` polyline with closure flags. At rest, the target’s canonical cubic `d` is restored; canonical values are quantized to four decimals for cross-engine stability. | SSR and rest output need a canonical representation distinct from the cheap frame representation. [VF, [serializer](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/serialize.ts)] |
| Animate | A semi-implicit Euler spring uses fixed 1/240-second substeps, capped per frame; named presets and custom stiffness/damping are supported. | A spring is a policy choice, not a geometry requirement. Poodle should decide whether tokens own duration/easing or a spring family. [VF, [spring](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/spring.ts)] |

The public site and README call the system zero-dependency and universal, and
the site states approximately 6.5 KB, under 1 ms planning, and one shared rAF.
Those are [SAC] claims, not independent measurements. The published package
does provide concrete anti-regression size gates: 7 KB gzip for core, 7.5 KB for
core plus DOM, and 8.5 KB for the React entry. [SAC/VF, [live/mutable product page](https://www.morphicons.com/#how)
and [v1.7.1 package size limits](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/package.json)].

### 2. SVG topology, winding, stroke, fill, and viewBox

Native SVG path interpolation is not a general fallback. The SVG specification
requires compatible path command structures for smooth interpolation; a
different command sequence or count cannot be meaningfully interpolated as the
same native path and falls back to discrete behavior. [VF, W3C [SVG paths](https://www.w3.org/TR/SVG2/paths.html)].
That is why a morph system needs normalization and resampling before it can claim
arbitrary pairs.

Subpaths and closure are observable geometry. SVG supports multiple subpaths in
one `d`; `Z` closes a subpath and can produce different joins and caps from an
explicit final line. [VF, W3C [paths](https://www.w3.org/TR/SVG2/paths.html)].
Morphicons preserves a closed flag and appends `Z` when both the plan and target
allow it. A closed-to-open transition is intentionally allowed to open the
loop. [VF, [Morphicons serializer](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/serialize.ts)
and [planner](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/plan.ts)].

Winding is a fill concern, not just point order. `nonzero` and `evenodd` use
different winding rules, and fill paints an open subpath as if it were closed.
[VF, MDN [fill-rule](https://developer.mozilla.org/en-US/docs/Web/CSS/fill-rule),
[fill](https://developer.mozilla.org/docs/Web/CSS/fill), and W3C [painting](https://www.w3.org/TR/SVG2/painting.html)].
Reversing or duplicating contours can therefore change a filled icon’s interior,
holes, and visual weight even when its sampled centreline looks plausible.
Morphicons’ own compatibility scan is explicit about stroke-based geometry and
rejects fill-only cases. [VF, [README compatibility section](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/README.md#icon-library-compatibility)].

`viewBox` establishes the SVG’s logical coordinate system, and
`preserveAspectRatio` controls how it maps into the viewport. [VF, MDN [SVG in
HTML](https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_in_HTML) and
[preserveAspectRatio](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/preserveAspectRatio)].
Morphicons’ `fitIcon` re-grids off-24 icon packs; Poodle’s default Lucide path
is already fixed at 24×24. A public arbitrary input would still need to define
whether it accepts only 24×24, performs fitting, or rejects mismatched boxes.

Poodle’s current defaults simplify the first registry: Lucide assets are
uniform, stroke-based, `fill="none"`, and round-capped/joined. That does not
make all pairs good. Different subpath counts, highly asymmetric shapes,
near-zero contours, holes, and semantic changes can still produce poor
correspondence. [WI]

### 3. Arbitrary pairs versus curated pairs

Morphicons deliberately handles unequal subpath counts through surjective
matching and duplicate “cell division.” It is a useful general-purpose
fallback, but it means an arbitrary pair always gets an answer even when the
answer should be rejected for product quality. [VF, [planner](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/plan.ts)].

Poodle has a finite, generated default icon set and semantic consumers such as
`IconButton`, `Menu`, and loading controls. A curated registry can choose pairs
whose state relationship is meaningful, validate both directions, record source
names and licence provenance, and allow human review of intermediate frames.
The cost is registry maintenance and the fact that an unlisted pair remains a
static swap. [WI]

The practical boundary is therefore:

- Internal geometry code may support broader structural inputs for testing and
  future adapters.
- The first public Poodle component should accept a semantic pair key or a
  validated pair reference, not arbitrary raw SVG strings.
- A custom `IconProvider` should not silently make arbitrary morphing available.
  Provider content must pass the same geometry, viewBox, primitive, stroke, and
  licence checks, or it remains static.

### 4. Reversibility, interruption, and cancellation

The Morphicons DOM driver exposes `morphTo`, `set`, `seek`, `progress`, a live
reduced-motion policy, and `destroy`. It keeps one global ticker set and cancels
the rAF when no ticker remains. [VF, [DOM driver](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts)].

When a target changes during flight, it snapshots the current sampled buffers,
builds a new plan from that intermediate shape, preserves the spring velocity,
and starts the new progress at zero. `set` stops the ticker and snaps to a
canonical target. Destroy makes later calls no-ops and unregisters the ticker.
[VF, [DOM driver lifecycle](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts#L214-L299)].
This is the right class of behavior for rapid toggles: no visible jump merely
because the user clicked again. It is not a guarantee that a reverse spring is
the exact time-reversal of the prior flight; it is a new plan from the current
shape with retained velocity. [WI]

The controller separates uncontrolled target changes, controlled frozen
`from`/`to`/`progress`, and imperative calls. Controlled mode wins while both
endpoints exist; leaving it invalidates the old pair and re-bases on re-entry.
[VF, [controller](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/controller.ts#L65-L220)].
Poodle should preserve this distinction if it exposes scrubbing, but should not
add all three modes to a first contract without an actual use case.

### 5. Accessible naming, state, focus, and reduced motion

SVG’s accessible name comes from the outer SVG role/label or a first-child
`title`; a decorative visual should not be announced. [VF, MDN [SVG title](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/title),
[ARIA img](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/img_role),
and W3C [SVG accessibility](https://www.w3.org/TR/SVG/access)]. Poodle already
puts naming on `Icon` and on the parent control. A morph should keep one stable
outer element and must not emit announcements per frame. If the semantic state
changes from “closed” to “open,” the parent control’s `aria-expanded`, pressed,
or other state remains the authority; the path is decoration. [WI]

An icon-only `IconButton` must retain its required label, keyboard focus, focus
ring, pressed semantics, disabled state, loading semantics, and tooltip behavior
while its child geometry changes. [VF, [`IconButton` accessibility](../../contracts/components/icon-button.md#L135-L175)].
Do not move the accessible name to the animated path or make the path focusable.

`prefers-reduced-motion` lets an application detect a user/device request to
reduce non-essential motion; WAI’s interaction-animation guidance recommends a
way to disable or reduce such animation. [VF, MDN [prefers-reduced-motion](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion)
and W3C WAI [SC 2.3.3 guidance](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html)].
Morphicons currently defaults to `reducedMotion="never"`, which ignores the OS
setting, and offers `"user"` and `"always"` as opt-in alternatives. [VF/SAC,
[README reduced-motion section](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/README.md#reduced-motion-all-five-bindings)
and [driver policy](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts#L31-L40)].
That is an explicit product choice, not a browser requirement. Poodle should
default to honoring the user preference or provide a contract-level policy that
does so by default; a short icon transition is not a sufficient reason to
silently override it. [WI]

The safe reduced-motion result is an immediate canonical target snap. It should
still preserve the same name, state, dimensions, and focus. A live preference
change should govern the next transition; it need not retroactively alter the
current path unless the contract says so. [WI]

### 6. SSR, hydration, and browser timing

Morphicons computes a canonical initial `d` with its pure core, emits it from
SSR, then creates the driver on mount/hydration. The React binding intentionally
keeps that initial string stable and mutates the path outside the virtual DOM;
the Svelte binding follows the same pattern. [VF/SAC, [React source](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/react/index.tsx#L1-L110)
and [Svelte source](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/svelte/MorphIcon.svelte#L1-L87)].

React’s official hydration contract requires the initial client output to match
the server output; browser-only values and deliberate differences must be
deferred to an effect. [VF, React [hydrateRoot](https://react.dev/reference/react-dom/client/hydrateRoot)].
Morphicons’ canonical serializer quantizes to four decimals because arc-to-cubic
trigonometry can differ by a final floating-point ulp across JS engines. [VF,
[serializer](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/serialize.ts#L29-L53)
and [changelog entry](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/CHANGELOG.md#L47-L50)].

For Poodle, a future morph must render a deterministic endpoint or controlled
frozen shape synchronously on server and client, avoid `matchMedia` during
render, and start scheduling only after mount. Any generated canonical geometry
must have a defined float-quantization rule. This avoids hydration warnings,
first-paint flashes, and path replacement that changes layout. [WI]

`requestAnimationFrame` is one-shot, typically follows the display refresh rate,
and is paused or throttled in background contexts; callbacks should use the
provided timestamp and cancel work when no longer needed. [VF, MDN
[requestAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)].
Morphicons clamps large `dt` values and owns one scheduler. Poodle’s web core
should make scheduler ownership and teardown explicit. GPUI’s clock is backend
owned, so the same pure plan should not assume browser globals. [WI]

### 7. Bundle, runtime, and frame cost

Morphicons’ source uses typed arrays for samples and preallocated output buffers;
its object-identity caches retain samples and rest-to-target plans for reusable
`IconNode` objects, while intermediate interruption plans are not cached. [VF,
[DOM driver caches](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts#L101-L149)
and [interpolator](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/core/interpolate.ts#L14-L46)].
Each animated SVG frame still performs a `d` attribute write and string
serialization. That keeps the virtual DOM out of the hot path but remains
main-thread DOM work and can trigger SVG style/paint. [VF, [driver render loop](https://github.com/guillermolg00/morphicons/blob/38d2a7221633a453eeafebd872ee3649b9274b22/src/dom/index.ts#L178-L212); WI about browser cost].

The package’s published size limits and one-rAF design are good engineering
guardrails, but Poodle should measure its own generated registry, number of
concurrent morphs, plan time, frame time, allocations, and SSR output. A
curated registry can precompute or serialize normalized plans at build time;
that trades package/assets size for lower mount-time planning. [WI]

### 8. Native GPUI feasibility

GPUI’s official 0.2.2 animation source provides a retained animation wrapper
with an `Instant`-based clock and frame requests; its `Transformation` type
covers scale, translation, and rotation. [VF, N1 sources above]. Poodle already
uses SVG rotation for the ring spinner, which proves the backend can animate a
supported property. It does not prove that a named SVG asset can accept a new
`d` string or a per-frame path object.

The minimum native feasibility spike is a dynamic path paint that can consume a
shared sampled geometry without rebuilding an entire asset catalogue every
frame. It must check lifetime, retained-tree rebuild behavior, frame pacing,
colour/stroke parity, and cancellation on node removal. If that route is not
available on GPUI 0.2.2, Poodle has three honest choices: defer true morphing,
ship a documented static fallback for a future contract, or use a separate
native-only precomputed technique. None is equivalent to quietly omitting the
feature from GPUI. [WI, based on local node/backend contracts and the active-
cohort rule.]

GPUI’s current accessibility gap is separately documented: metadata reaches the
node but GPUI 0.2.2 has no accessibility attributes/tree to receive it. [VF,
[`003-native-accessibility.md`](../../contracts/003-native-accessibility.md#L8-L45)].
The visual animation must not worsen that gap or be used as a reason to move
semantics into geometry. Jetstream’s AccessKit position is not a reason to
admit Jetstream for this track; its program-level deferral remains in force.

## Options and tradeoffs

| Option | Fit with current Poodle | Benefits | Costs and failure modes | Disposition |
|---|---|---|---|---|
| No-build static endpoint swap | High | No new geometry, dependency, node, SSR, or native capability. Keeps `Icon` contract intact. A parent could optionally cross-fade two static icons where a product contract allows it. | Not a true shape morph; cross-fade can look like disappearance and may not preserve perceived continuity. CSS/native behavior would still need a separate motion decision. | Keep as the current baseline and safe fallback. |
| Generic public primitive over arbitrary `IconNodes`/raw `d` | Low initially | Maximum flexibility; closely resembles Morphicons’ structural core; useful for experiments and custom packs. | Requires parser/normalizer/topology/viewBox/fill/stroke validation, quality policy, malformed-input errors, SSR determinism, performance limits, and a GPUI geometry solution. “Any icon” produces valid but often poor matches. Public raw SVG also expands licence and security/provenance review. | Do not make this the first public contract. |
| Curated morph-pair registry | Medium-high | Semantic quality boundary; finite review surface; build-time validation; tree-shakeable pair data; can be emitted to web and native formats from the existing manifest; easy to test interruption and reverse transitions. | Registry curation and visual QA cost; unlisted pairs remain static; pair metadata and licences can drift; generated geometry increases artifacts. | Recommended public direction, subject to native and operator decisions. |
| Lower-level pure motion/geometry utility | High as substrate | Framework-free, testable, reusable by Svelte and React; can be consumed by Rust port; separates planning from DOM/GPUI scheduling; supports controlled seek and interruption without forcing a component API. | Still needs input constraints, numeric determinism, cache policy, and a rendering target. A utility alone does not solve semantics, registry quality, or GPUI path drawing. | Recommended internal substrate before a public component. |
| Adopt Morphicons as a third-party dependency | Medium for web only | Mature reference implementation; MIT source licence; React/Svelte bindings, SSR, tests, reduced-motion policy, and package size gates already exist. | Adds external ownership and release coupling; uses a different data/registry model; its default ignores OS reduced motion; no GPUI/poodle-node integration; Poodle cannot claim one active-cohort contract from web bindings alone. The package’s MIT code licence does not settle licences of supplied icon data. | Do not adopt now. Use as evidence and possible future comparison point. |
| Precomputed frame/asset set (“no-build” at runtime) | Low-medium | Could render in GPUI without mutable SVG geometry; deterministic runtime and simple lookup. | Asset explosion, coarse stepping, memory/loading cost, poor arbitrary-pair coverage, and awkward interruption/reversal. It moves cost from CPU to repository/package size. | Keep as a native fallback experiment only, not the default design. |

The lower-level utility and curated registry are complementary, not competing:
the utility owns geometry and lifecycle mechanics; the registry owns which pairs
Poodle promises. [WI]

## Recommended Poodle direction

### Recommendation

1. Keep the existing `Icon`, `IconProvider`, `IconButton`, and `Spinner`
   contracts unchanged. Do not add an animation prop, automatic morph-on-name
   change, or raw `d` escape hatch to `Icon`. This respects the current
   static-foundation contract and avoids a pre-v1 breaking migration. [WI]
2. Promote a future separate `IconMorph`-type display primitive only after an
   operator decision that true geometry morphing is valuable enough to justify
   native geometry work. The first portable input should be a curated semantic
   pair key or validated pair reference, not arbitrary provider geometry. [WI]
3. Build the capability around a pure core utility: validated input → normalized
   geometry → fixed sampling → correspondence/plan → interpolation → canonical
   serialization. Web bindings should be thin Svelte/React shells; shared Rust
   should own the renderer-neutral declaration; GPUI should only interpret a
   proven dynamic geometry capability. This follows Poodle’s headless-core and
   shared-render boundaries. [WI, [`006 Headless Core`](../../architecture/006-headless-core-and-machine-model.md#L9-L28)
   and [`003 Component Substrates`](../../architecture/003-component-docs-ia-and-implementation-substrates.md#L8-L50)].
4. Generate pair metadata/artifacts from the same pinned Lucide manifest and
   build gate. Include source names, canonical endpoint identity, geometry
   constraints, sample/normalization version, quality metrics or approval state,
   and licence/provenance records. Do not copy Morphicons source or assets; an
   implementation can independently use the researched techniques. [WI]
5. Default reduced motion to the user’s preference, with an explicit test/screenshot
   override and an immediate endpoint snap. Keep accessible naming and control
   state on the stable outer component/parent. [WI]
6. Treat interruption and cancellation as contract behavior: rebase from the
   current visual sample or use a deterministic snap, stop work on unmount, and
   ensure repeated updates cannot leave a stale scheduler or stale pair plan.
   [WI]
7. Run the bounded spike below before promoting a component contract or roadmap
   card. If GPUI cannot render mutable path geometry at the target version, stop
   short of claiming active-cohort completion and record the capability gap with
   its reason. Jetstream remains deferred. [WI]

### Bounded research spike before promotion

The next technical spike should be disposable or isolated until its evidence is
accepted. It should use current Poodle data and no Morphicons dependency.

- Select 8–12 meaningful current-manifest pairs covering one-to-one paths,
  multiple subpaths, open/closed contours, congruent rotation, asymmetric
  shapes, and at least one likely poor pair. Include examples such as menu/X,
  plus/check, play/pause, lock/lock-open, volume-2/volume-x, and chevron or
  directional pairs only where the current manifest contains both endpoints.
- Compare normalized, sampled, aligned, and serialized output in a pure core;
  record correspondence residuals, closed flags, endpoint error, plan time,
  frame time, allocation behavior, and visual review notes. Test both directions.
- Exercise rapid A→B→A and A→B→C retargeting, controlled progress/seek,
  explicit `set`, destroy/unmount, zero/one concurrent instances, and a delayed
  first icon. Assert no stale frame writes after cancellation.
- Render the same cases through Svelte and React with stable outer semantics;
  test server output against the first client output, reduced-motion preference,
  explicit always-snap mode, focus retention, and icon-button naming/state.
- Prove or disprove a GPUI dynamic path route at pinned GPUI 0.2.2. Measure
  native frame pacing and teardown. Do not run windowed conformance selectors
  without operator approval.
- Run a provenance/licence review for every source icon and any derived
  normalized artifact. Preserve current Lucide/Feather notices.

## Explicit non-goals

- No change to the current static `Icon` contract or automatic morphing whenever
  an icon name/provider value changes.
- No implementation, generated pair files, new public API, token schema change,
  architecture edit, triage edit, index edit, roadmap card, or component ship in
  this research task.
- No promise that arbitrary raw SVG, arbitrary `IconNodes`, groups, transforms,
  masks, clips, fill-only icons, multicolour icons, non-uniform strokes, or
  mismatched viewBoxes can morph.
- No copying or vendoring of Morphicons source, tests, examples, or assets; no
  Morphicons runtime dependency in the proposed first step.
- No inference that Morphicons’ “universal,” size, or speed claims are Poodle
  guarantees.
- No new GPUI accessibility tree or workaround for the existing GPUI upstream
  limitation.
- No Jetstream admission or per-component Jetstream exception.
- No full Lucide catalogue expansion, lazy icon loading, or app-specific DAW
  widget ownership.
- No assumption that a static cross-fade is equivalent to a geometry morph; it is
  only the current safe fallback.

## Risks

| Risk | Consequence | Mitigation / gate |
|---|---|---|
| Correspondence artefacts | Shapes can twist, collapse, duplicate, or rotate through an unintended orientation. | Curated pairs, residual/quality thresholds, visual review, both-direction and interruption tests. |
| Topology and winding | Holes, closure, fill rule, caps, and joins can change apparent meaning. | Stroke-only first scope; preserve closed flags; reject fill semantics; explicit viewBox/stroke contract. |
| Custom provider mismatch | A provider can supply unsupported tags, fill geometry, transforms, or off-grid data. | Validate effective nodes; static fallback or clear error; no silent morph promise for provider content. |
| Generated drift | Pair data and endpoint assets can diverge across web/native or Lucide versions. | One manifest, pinned source version, deterministic codegen, stale/orphan-aware `--check`, provenance headers. |
| Licence ambiguity | Derived geometry may have obligations different from Morphicons code. | Keep upstream notices; record source and transformations; require operator/legal policy before publishing derived artifacts. |
| Runtime/frame budget | Per-frame `d` writes and serialization compete with application work; many concurrent morphs can jank. | Shared scheduler/clock, preallocated buffers, bounded concurrency, benchmarks, and explicit size/frame budgets. |
| SSR/hydration | Server/client float differences or browser-only preference reads cause warnings or first-paint changes. | Canonical quantization, synchronous stable initial endpoint, post-mount scheduling, SSR tests. |
| Rapid updates | Stale plans, jumps, orphaned rAFs, or post-unmount writes can leave wrong geometry. | Snapshot/replan semantics, `set` cancellation, destroy tests, stable identity/cache rules. |
| Reduced-motion policy | Ignoring a user preference can cause discomfort; always snapping can remove useful state communication. | Honor user preference by default; expose explicit policy; keep state/name independent of motion. |
| GPUI capability | Web can ship while native cannot draw a mutable path, violating active-cohort parity. | Native feasibility gate before contract promotion; otherwise document capability absence and defer the feature. |
| API lock-in | A broad raw-geometry API becomes hard to narrow before v1. | Separate primitive, curated references first, no aliases/fallbacks, contract before implementation. |
| Quality maintenance | Pair additions become aesthetic or semantic regressions. | Pair owner, review checklist, representative specimens, regression snapshots, and removal/update policy. |

## Unresolved operator decisions

1. Is true geometry morphing a Poodle-owned capability, or is a static endpoint
   swap/cross-fade sufficient for the intended product surfaces?
2. Must the active cohort have a real GPUI geometry morph before any web contract
   is promoted, or may a web-first research/preview phase carry a declared
   native capability gap? The working rules currently treat such a gap as debt,
   not parity.
3. Which semantic pairs are valuable enough to own, and who approves visual
   quality, reversibility, and pair removal?
4. Should the motion policy use semantic duration/easing tokens, a spring preset,
   or both? What is the default reduced-motion behavior and test override?
5. Should public inputs be canonical Poodle pair keys only, or should validated
   custom `IconNodes` be supported later through an explicit adapter?
6. Does pair geometry belong in generated TypeScript/Rust artifacts, in a runtime
   plan cache, or in a hybrid with build-time normalized data and runtime
   interpolation?
7. Is a new renderer-neutral `MorphIcon`/path-geometry node acceptable, or should
   the feature wait for a GPUI upstream capability? Do not overload static
   `NodeKind::Icon` or encode path strings inside the generic property animation
   channel without an architecture decision.
8. What licence/provenance policy governs normalized geometry derived from Lucide,
   Feather-derived icons, and any future application icon sources?

## Promotion-ready contract, architecture, and card scope

This is a handoff boundary, not approval to edit those documents in this
research branch.

### Contract scope

Create a new contract only after the operator decisions and native spike.

- Name: tentative `IconMorph`; keep it separate from `Icon`.
- Purpose: an opt-in display primitive for a validated semantic pair.
- Inputs: pair key/reference, current endpoint or state, size, size role,
  accessible label, and an explicit motion policy. Add controlled progress only
  if a real gesture/scrubbing use case exists.
- States: exact endpoint A, exact endpoint B, in-flight, reduced-motion snap,
  interrupted/rebased, and cancelled/unmounted lifecycle. Do not expose
  intermediate geometry as accessible state.
- Events: none for a display-only primitive. Parent controls semantic state and
  callbacks; an imperative lower-level utility may expose `set`, `seek`,
  `morphTo`, and `destroy` if the operator chooses it.
- Accessibility: stable outer role/name behavior matching `Icon`; decorative by
  default; no focus target; no per-frame announcements; parent owns button/menu
  state and label.
- Layout: same size-role and fixed-box behavior as the static icon; no layout
  shifts between endpoints.
- SSR: deterministic endpoint/frozen progress output; runtime begins after mount;
  canonical float serialization is part of the contract.
- Explicit limits: initially stroke-oriented, 24×24, supported primitive set,
  approved pair registry, no automatic arbitrary-provider guarantee.

### Architecture scope

- `packages/core` owns framework-free geometry types, validation, plan building,
  interpolation, canonical serialization, caches, and lifecycle policy that is
  genuinely portable. It must not import Svelte, React, DOM, or GPUI.
- A deterministic build step consumes the pinned icon manifest plus authored pair
  metadata. It emits a registry and any normalized endpoint/pair data needed by
  web and Rust. It records source version, pair version, constraints, quality
  diagnostics, and licence/provenance. `audit:icons` or a dedicated check must
  fail on source/pair/artifact drift.
- Svelte and React expose thin shells over the same core contract and shared CSS;
  they do not each implement matching or spring logic.
- `poodle-specs` gets a renderer-neutral spec only after the contract is
  approved. `poodle-render` owns composition and token resolution.
- `poodle-node` gets a distinct geometry-capable declaration if needed. Keep
  static `NodeKind::Icon` as the named-asset path. Do not put large per-frame
  path strings into generic `AnimProperty`; the node/backend boundary says the
  backend owns driving and rasterisation.
- GPUI receives a backend branch only after the dynamic-path proof. If it cannot
  meet the observable result, the contract must record the limitation and the
  feature must not be reported complete. Jetstream remains deferred.
- Add a semantic icon-morph motion token only if the motion decision establishes
  a reusable meaning. Add it in the canonical DTCG schema and regenerate all
  targets; do not hardcode a value in a component.

### Future implementation cards

1. Geometry format, parser/normalizer, fixed sampling, topology validator, and
   deterministic numeric rules.
2. Pair authoring schema, curated registry, quality diagnostics, provenance and
   licence notices, deterministic code generation, and drift gate.
3. Pure core plan/interpolation/lifecycle utility with interruption, cancellation,
   reduced motion, and SSR vectors.
4. Svelte and React bindings plus shared CSS, stable accessible semantics, and
   hydration/reduced-motion tests.
5. Shared Rust spec/render composition and a GPUI dynamic path feasibility,
   backend, lifecycle, and performance card. Do not split a web card from its
   native admission decision if the contract claims active-cohort completion.
6. Curated specimens, browser/native test fixtures, pair regression snapshots,
   benchmarks, and documentation.

## Proposed disposition of the originating triage note

**Research complete. Keep the triage note open as a promotion-ready decision
record; promote it to an architecture decision candidate only after the operator
answers the native-feasibility, pair-registry, motion-policy, input-boundary,
and licence questions above. Do not create an implementation card yet.**

The current safe disposition is “hold for architecture decision,” with static
`Icon` behavior unchanged and Jetstream still deferred. [WI]

## Citations and local audit index

Primary external evidence is inventoried above. The local audit depends on these
authorities and implementation paths:

- [`docs/architecture/001-poodle-system-shape.md`](../../architecture/001-poodle-system-shape.md)
- [`docs/architecture/002-token-system-and-package-layout.md`](../../architecture/002-token-system-and-package-layout.md)
- [`docs/architecture/003-component-docs-ia-and-implementation-substrates.md`](../../architecture/003-component-docs-ia-and-implementation-substrates.md)
- [`docs/architecture/006-headless-core-and-machine-model.md`](../../architecture/006-headless-core-and-machine-model.md)
- [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md)
- [`docs/contracts/003-native-accessibility.md`](../../contracts/003-native-accessibility.md)
- [`docs/contracts/components/icon.md`](../../contracts/components/icon.md)
- [`docs/contracts/components/icon-provider.md`](../../contracts/components/icon-provider.md)
- [`docs/contracts/components/icon-button.md`](../../contracts/components/icon-button.md)
- [`packages/core/src/icons/default-icons.json`](../../../packages/core/src/icons/default-icons.json)
- [`scripts/build-default-icons.ts`](../../../scripts/build-default-icons.ts)
- [`packages/contracts/node/src/lib.rs`](../../../packages/contracts/node/src/lib.rs)
- [`packages/gpui/node-backend/src/lib.rs`](../../../packages/gpui/node-backend/src/lib.rs)
- [`packages/core/test/icons.test.ts`](../../../packages/core/test/icons.test.ts)
