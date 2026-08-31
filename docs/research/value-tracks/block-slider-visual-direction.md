# Value Track: Block Slider And RangeSlider Visual Direction

Status: complete (evidence documented; promotion requires operator decisions)
Created: 2026-08-31
Updated: 2026-08-31
Origin: `docs/triage/20260831-155151-block-slider-visual-direction.md`
Scope: block-style `Slider` and `RangeSlider`; Jetstream remains deferred

This is a point-in-time research dossier. It proposes a bounded Poodle direction; it does not change a contract, add an API, or implement a component.

Evidence labels used below:

- **[LF] Local fact** — observed in this worktree at the cited path.
- **[SF] Source fact** — stated by an external standard or library source.
- **[WI] Worker inference** — Poodle-specific synthesis or recommendation from those facts.

## Executive Summary

The reference is a useful visual direction: a track-dominant rounded capsule with an active segment carrying a short label (`Blur`) and a remainder carrying a formatted value (`67 px`). The durable Poodle pattern should be “block appearance,” not “copy this image.” The capsule is a presentation layer over the existing scalar and two-thumb value laws.

**Recommendation: add an opt-in block appearance and keep the current default unchanged.** A block appearance should:

- preserve the existing `Slider` value law, step, bounds, live-change, commit, orientation, and disabled semantics;
- preserve `RangeSlider` as two independently focusable slider controls with a stable lower/upper identity and the `lower <= upper` invariant;
- use one shared normalized axis geometry for Svelte, React, shared Rust composition, and GPUI;
- make the track and selected segment the visual subject, with inline label/value content only when it fits;
- move content to a stable readout or focus/drag tooltip when the inline segment is too small, never clip or overwrite it;
- keep both RangeSlider handles visible and independently targetable even when the selected window is zero-width or narrow;
- distinguish a small visible thumb from an effective pointer/touch target of at least 44×44 CSS px, matching the current Poodle size-and-density rule;
- treat pointer release, cancellation, and lost capture as one idempotent gesture-terminal path;
- provide native/system-color fallbacks for forced colors and no motion dependency for value feedback;
- admit horizontal web first for visual acceptance, then vertical across the active Svelte/React/GPUI cohort only after the existing native RangeSlider vertical gap is closed.

The block treatment should not become the default yet. Existing Slider consumers and raw embedded `.poodle-slider` visuals rely on today’s structure, while RangeSlider has public preview and package coverage despite no current consumer in the release roster. A replacement default would create layout, visual, and interaction migration at once. An additive appearance isolates that risk and follows the repository’s recipe policy.

The strongest first interaction model is **full-track targeting to the nearest thumb at pointer-down, then hold that thumb for the gesture**. Do not add whole-window dragging, thumb swapping, or minimum-distance policy to this visual track. For equal or overlapping thumbs, keep lower/upper focus order and value identity stable, choose a deterministic tie, and raise the focused/active thumb visually without hiding the other semantic control.

## Method And Source Inventory

### Method

Access/check date for all external sources below: **2026-08-31**. The local audit read the repository authority chain before source inspection:

- `AGENTS.md`, `docs/README.md`, `docs/research/README.md`, and the originating triage note;
- architecture: `docs/architecture/001-poodle-system-shape.md`, `002-token-system-and-package-layout.md`, `006-headless-core-and-machine-model.md`, `007-appearance-recipe-contract.md`, `008-audio-control-family.md`, `010-native-presentation-construction-context.md`, and `product-guardrails.md`;
- contracts/specs: `docs/contracts/001-working-rules.md`, `docs/contracts/components/slider.md`, `docs/contracts/components/range-slider.md`, `docs/contracts/components/size-and-density.md`, `docs/contracts/003-native-accessibility.md`, and the relevant accessibility, contrast, performance, migration, and GPUI baseline specs;
- current TypeScript/Rust source, CSS, recipes, tokens, Svelte/React/GPUI specimens, focused tests, parity records, and consumer references;
- the supplied image, which was available at `/var/folders/ng/5rcw8k5s24j50my4x2rqf8sm0000gn/T/paseo-attachments-a7zsvQ/e8de4aad9aeaf14dc032dc2bf9ca74f9c4040a29a5ffbd7ed4d5e7393611c9c3.png`. It was inspected as a visual reference only and was not copied into the repository.

External research prioritized normative platform/accessibility sources, then mature component-library precedent. No third-party code or asset is copied by this dossier.

### Normative and platform sources

| Source | Evidence used | Licence/access record |
| --- | --- | --- |
| [WAI-ARIA APG Slider Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/slider/) | Slider keyboard, value properties, accessible naming, orientation, and the warning that touch assistive technology support for custom sliders varies. | W3C document; checked 2026-08-31. W3C’s [Software and Document License](https://www.w3.org/copyright/software-license-2023/) was checked. |
| [WAI-ARIA APG Multi-Thumb Slider Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/slider-multithumb/) and [example](https://www.w3.org/WAI/ARIA/apg/patterns/slider-multithumb/examples/slider-multithumb/) | Each thumb is a distinct slider; tab order remains stable; a thumb cannot cross its sibling in the example; dependent bounds are exposed; the example keeps a current value adjacent to the focused thumb for magnification users. | W3C document; checked 2026-08-31. W3C document licence checked. |
| [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria/) | `slider` role is a range input with value/bounds semantics; horizontal is the default orientation; `aria-valuetext` is the human-readable value channel. | W3C Recommendation; checked 2026-08-31. W3C document licence checked. |
| [WCAG 2.2](https://www.w3.org/TR/WCAG22/), [non-text contrast](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast), [target size minimum](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html), [focus appearance](https://www.w3.org/WAI/WCAG22/Understanding/focus-appearance.html) | 4.5:1 normal text / 3:1 large text; 3:1 non-text component and focus indicators; 24×24 AA target baseline; focus must remain visible and distinguishable. | W3C Recommendation/Understanding documents; checked 2026-08-31. W3C document licence checked. |
| [Pointer Events Level 3](https://www.w3.org/TR/pointerevents3/) | Unified mouse/pen/touch events, `pointercancel`, `lostpointercapture`, pointer capture, and `touch-action` behavior. | W3C Recommendation; checked 2026-08-31. W3C document licence checked. |
| [CSS Color Adjustment Level 1](https://drafts.csswg.org/css-color-adjust-1/) and [CSS Color 4](https://drafts.csswg.org/css-color-4/) | `forced-colors`, `forced-color-adjust`, and system-color pairings; author/system color mixing is not a guaranteed contrast strategy. | CSS Working Group drafts; checked 2026-08-31. W3C document licence checked. |
| [Media Queries Level 5](https://drafts.csswg.org/mediaqueries-5/#prefers-reduced-motion) | `prefers-reduced-motion: reduce` is the platform signal for reducing non-essential motion. | CSS Working Group draft; checked 2026-08-31. W3C document licence checked. |
| [WHATWG HTML range state](https://html.spec.whatwg.org/multipage/input.html) | Native range inputs provide range semantics and established `input`/`change` event behavior. | HTML Standard checked 2026-08-31. Its [acknowledgements/licence text](https://html.spec.whatwg.org/multipage/acknowledgements.html) states CC BY 4.0 for the document and BSD 3-Clause for incorporated source portions. |

### High-quality precedent

| Precedent | Evidence used | Licence/access record |
| --- | --- | --- |
| [Adobe Spectrum Slider anatomy](https://spectrum.adobe.com/page/slider/) and [Spectrum Web Components Slider API](https://opensource.adobe.com/spectrum-web-components/components/slider/api/) | Label, track, handle, fill, and value are explicit anatomy; range handles remain individually named; formatted values and focus/hover value display are treated as first-class concerns; tick variants are separate. | Documentation checked 2026-08-31. No Spectrum code copied. |
| [Adobe React Aria Slider](https://react-aria.adobe.com/Slider) and [useSlider](https://react-aria.adobe.com/Slider/useSlider) | One or more values, `onChange` during interaction, `onChangeEnd` at completion, per-thumb labels, orientation, formatting, and explicit focused/hovered/dragging render states. | Documentation checked 2026-08-31. The project [Apache 2.0 licence](https://raw.githubusercontent.com/adobe/react-spectrum/main/LICENSE) was checked; no code copied. |
| [Radix Slider](https://www.radix-ui.com/primitives/docs/components/slider) | Multiple thumbs, min-distance configuration, track interaction, orientation, RTL support, keyboard support, and form-native inputs. | Documentation checked 2026-08-31. [MIT licence](https://raw.githubusercontent.com/radix-ui/primitives/main/LICENSE) checked; no code copied. |
| [MUI Slider](https://mui.com/material-ui/react-slider/) and [API](https://mui.com/material-ui/api/slider/) | Range arrays, labelled marks, value-label display/formatting, track variants, thumb slots, and explicit range constraints. | Documentation checked 2026-08-31. [MIT licence](https://raw.githubusercontent.com/mui/material-ui/master/LICENSE) checked; no code copied. |

The precedents are evidence of viable design choices, not Poodle authorities. Poodle’s architecture, component contracts, and active-runtime admission rules remain authoritative.

## Current Poodle Audit

### Authority and architecture

**[LF]** `docs/architecture/001-poodle-system-shape.md` defines one semantic Poodle contract across Svelte, React, shared Rust composition, and GPUI. Web shells are thin adapters over shared core behavior/styles; native composition flows through `poodle-render` and `poodle-node`; Jetstream is deferred. The same document puts semantics/state/behavior before visual parity.

**[LF]** `docs/architecture/006-headless-core-and-machine-model.md` puts transition laws in framework-free machines and pointer capture, focus, keyboard translation, ARIA, and other runtime details in adapters. `docs/architecture/008-audio-control-family.md` repeats that Slider/RangeSlider geometry and gesture laws are shared, while DOM/native adapters own event translation. Its gesture rule requires one begin/end lifecycle, equivalent release/cancel termination, and no stranded or duplicate terminal event.

**[LF]** `docs/architecture/007-appearance-recipe-contract.md` allows component-scoped recipe variables as the supported override surface. Recipes are additive and must be mapped to existing semantic tokens; structural brand expression belongs in wrappers. A recipe cannot by itself introduce visible semantic content, collision policy, or a different interaction surface. That is why block appearance needs a contract-level decision in addition to token hooks.

**[LF]** `docs/contracts/001-working-rules.md` requires Svelte, React, shared Rust spec/renderer, and GPUI specimen coverage for the active cohort and says contract changes precede observable behavior changes. `docs/contracts/003-native-accessibility.md` records that GPUI 0.2.2 has no usable accessibility tree/API in the measured preview; native accessibility evidence is therefore a documented blocker, not a parity claim.

### Existing Slider contract and source

**[LF]** `docs/contracts/components/slider.md` currently defines:

- one continuous or stepped value with bounds, step, linear/native or embedded audio law, unipolar/bipolar polarity, optional center, horizontal/vertical orientation, disabled state, `ariaLabel`, `valueText`, size, size role, and density;
- standard and embedded variants; standard uses native range semantics, embedded uses a custom role and the shared control machine;
- live value change plus one release commit;
- arrows, Home, and End; PageUp/PageDown remain browser-owned/optional rather than strict parity requirements;
- a visible track/fill/focus/thumb treatment with no marks or value-label wrapper in the current contract;
- 44×44 minimum interaction intent through the size-and-density contract, while the visual size ladder remains smaller (`xs` through `xl` thumb diameters from 0.75rem to 1.25rem).

The exact current recipe hooks are listed in `docs/contracts/components/slider.md` and generated inventory `packages/svelte/preview/artifacts/recipe-inventory.json:1957-1968`: track fill, fill, negative fill, track border, center fill, control fill, thumb fill, control shadow, focus ring, and focus control shadow.

**[LF]** `packages/core/src/slider.ts:19-167` normalizes Slider values and emits change/commit effects. `packages/core/src/slider.ts:171-343` does the same for RangeSlider and its embedded control machine. The embedded machine exposes `PointerBegin`, `PointerMove`, and `PointerEnd`; it has no distinct cancellation or lost-capture event. RangeSlider chooses the nearer thumb on begin, holds the chosen thumb, clamps it against its sibling, and does not swap.

**[LF]** `packages/core/src/styles/slider.css:4-172` and `packages/core/src/styles/range-slider.css:4-227` render a rounded track, fill segments, optional bipolar center, and native/custom controls. The current CSS has no `forced-colors` or `prefers-reduced-motion` branch and no component transition/animation. Embedded roots use `touch-action: none` and the density selectors add vertical padding. The root min-height is 1.25–1.75rem by size, not 44px; the CSS does not itself establish a 44×44 per-thumb hit wrapper.

### Existing RangeSlider contract and source

**[LF]** `docs/contracts/components/range-slider.md` defines two slider controls under a group:

- lower and upper values with a preserved `lower <= upper` invariant;
- nearest-thumb track targeting, then gesture ownership held by the thumb chosen at press time;
- independent lower/upper names, values, bounds, and value text; two Tab stops;
- standard native two-input and embedded custom-control variants;
- unipolar and bipolar selected-window fill geometry;
- the same size/density, orientation, disabled, and commit model as Slider;
- marks, tick labels, and the future value-label wrapper out of scope.

The exact current recipe hooks are listed in `docs/contracts/components/range-slider.md` and `packages/svelte/preview/artifacts/recipe-inventory.json:1728-1740`: track fill, fill, negative fill, track border, center fill, control fill, control track fill, thumb fill, thumb shadow, focus ring, and focus thumb shadow.

**[LF]** The current web RangeSlider source has two important deltas:

- Svelte `packages/svelte/components/src/RangeSlider.svelte:110-174` and React `packages/react/components/src/RangeSlider.tsx:131-206` handle pointer cancellation but do not handle `lostpointercapture`. Both standard native inputs expose the full `min`/`safeMax`; the contract text describes dynamically clamping the lower input’s maximum to the upper value and the upper input’s minimum to the lower value. The shared machine clamps callback values, but the DOM constraint/value announcement is not the same as the documented contract.
- `packages/core/src/styles/range-slider.css:93-108` gives embedded controls a 0.75rem geometry and focus shadow but no explicit thumb fill or border. The rule needs a visual verification pass before it can be treated as a stable block foundation. The Rust renderer supplies a separate native node treatment, so this is a web/native divergence risk.

### Runtime implementations

| Runtime | Current evidence | Block-direction consequence |
| --- | --- | --- |
| Svelte | `packages/svelte/components/src/Slider.svelte:99-156` and `RangeSlider.svelte:110-175`; standard Slider is native, embedded Slider is custom, Range standard is two native inputs, Range embedded is two custom roles. Focused tests are `packages/svelte/components/test/Slider.test.ts` and `RangeSlider.test.ts`. | Native semantics are the preferred web baseline. A block appearance can use a visual layer over native inputs, but must preserve the two-input/two-thumb accessibility model. Add tests for pointer cancellation, lost capture, hit regions, and text collision. |
| React | `packages/react/components/src/Slider.tsx:97-164` and `RangeSlider.tsx:111-208`; Range standard commits from mouse/key/touch-end paths rather than the same native `change` path as Svelte. Focused tests are `packages/react/components/test/Slider.test.tsx` and `RangeSlider.test.tsx`. | Block behavior must remove this commit-path drift or document it before claiming shared parity. Keep render-time formatting and controlled updates bounded during live drags. |
| Shared core | `packages/core/src/slider.ts:123-343` contains scalar/range value laws, normalized geometry, nearest-thumb selection, sibling clamping, and one commit effect. | Reuse the laws. Add only the missing interaction state needed for cancellation/active identity and a serializable presentation geometry; do not fork block-specific value math. |
| Shared Rust/native renderer | `packages/render/src/slider.rs:58-123,307-385` maps orientation, fill geometry, a focusable Slider node, and an overlay scrub axis. `packages/render/src/range_slider.rs:137-197` creates two identified thumb nodes. | Block composition can be built from the same finite track/fill/thumb nodes, but native RangeSlider’s semantic bounds/value-text projection and vertical axis need closure first. |
| GPUI | `packages/gpui/preview/src/node_compat.rs:7223-7331` wraps the renderer. `packages/gpui/preview/src/specimens/range_slider.rs:177-289` includes narrow/full/low/high and vertical specimens. | A specimen does not prove native vertical or assistive-technology parity. GPUI must receive the same two semantic controls and hit geometry; its current AX blocker stays explicit. |
| Jetstream | Deferred in `docs/architecture/001-poodle-system-shape.md` and the working rules. | No Jetstream admission, implementation, or parity claim belongs in this direction. |

### Native gaps that constrain admission

**[LF]** `packages/render/src/range_slider.rs:175-180` assigns each native thumb a role, label, and value, and only adds `aria-orientation` for the embedded branch. It does not project the spec’s per-thumb min/max/value-text fields at this point. `packages/render/src/range_slider.rs:479-500` deliberately installs a horizontal full-width scrub overlay and records that vertical RangeSlider layout is deferred. A vertical spec can therefore exist in the preview without being a working vertical native control.

**[LF]** `packages/render/src/slider.rs:99-123` is a stronger precedent: it projects Slider role, label, value, bounds, value text, orientation, focusability, tab index, and keyboard handler, and `packages/render/src/slider.rs:65-69` has an orientation-specific scrub axis. The RangeSlider renderer needs equivalent per-thumb semantic projection before block visual parity can be promoted.

**[LF]** `docs/contracts/003-native-accessibility.md` and `packages/gpui/native-accessibility-proof.json` record GPUI’s measured 0.2.2 accessibility limitation. The block work can improve node metadata and focus/hit behavior, but it cannot honestly claim working native assistive technology until the upstream/backend event in that contract is resolved.

### Specimens, tests, parity, and consumers

**[LF]** Svelte and React specimens cover default, stepped, disabled, embedded unipolar/bipolar, five sizes, and densities. Range specimens additionally cover vertical, and GPUI Range specimens include narrow `[45,55]`, full `[0,100]`, low `[0,25]`, and high `[75,100]` positions. Relevant files are:

- `packages/svelte/preview/src/specimens/SliderSpecimen.svelte` and `RangeSliderSpecimen.svelte`;
- `packages/react/preview/src/gallery/specimens/SliderSpecimen.tsx` and `RangeSliderSpecimen.tsx`;
- `packages/gpui/preview/src/specimens/slider.rs` and `range_slider.rs`.

**[LF]** Existing focused tests cover normalization, snapping, keyboard arrows/Home/End, embedded pointer traces, no-crossing, live change, one commit, disabled state, and some ARIA attributes. They do not cover the block-specific risks: inline-content collision, equal-thumb layering, 44×44 effective targets, `lostpointercapture`, focus-visible/hover/pressed render states, forced colors, reduced motion, contrast across themes, or native RangeSlider vertical behavior. The Svelte Range test is mostly standard-input coverage; the React test has more scrub tracing but the two runtimes’ standard commit paths differ.

**[LF]** `docs/roadmaps/g16/parity-evidence-ledger.md:138` records Slider with focused web tests and a mounted GPUI regression, but native accessibility remains manual and a GPUI visual comparison fixture is missing. Its RangeSlider row records the same evidence shape and the existing mounted scrub regression. The ledger is evidence status, not a visual-admission decision.

**[LF]** Current consumer references found in this tree:

- Svelte `packages/svelte/preview/src/components/DisplayControls.svelte:126-137` and React `packages/react/preview/src/gallery/DisplayControls.tsx:110-121` use Slider for neutral contrast with `ariaLabel` and formatted `valueText`;
- Svelte `packages/svelte/components/src/ColorPicker.svelte:430-462` and React `packages/react/components/src/ColorPicker.tsx:436-464` use Slider for Hue and Opacity;
- Svelte `packages/svelte/components/src/audio/ModMatrixVisual.svelte:6-14` and React `packages/react/components/src/audio/ModMatrixVisual.tsx:4-11` use the raw `.poodle-slider` visual class for an aria-hidden embedded matrix cell, not the public Slider shell;
- `docs/roadmaps/g15/release-baseline-roster.md:127-134` lists no current RangeSlider consumer and lists `acowtancy` and `loophole-legacy` for Slider. Those named products are not present as source in this worktree, so the roster is an inventory signal, not a complete downstream compile audit.

Changing the default `.poodle-slider` structure or dimensions would therefore affect preview controls, color controls, raw audio visuals, public specimens, and external consumers. An explicit appearance selector limits that blast radius.

## Detailed Findings

### 1. The visual model should be a semantic track with finite layers

**[SF]** Spectrum’s slider anatomy separates label, track, handle, fill, and value; React Aria exposes track, fill, thumb, output, and interaction states as separate pieces. This is a better precedent for Poodle than treating the reference as a single background image. See [Spectrum anatomy](https://spectrum.adobe.com/page/slider/) and [React Aria Slider](https://react-aria.adobe.com/Slider).

**[WI]** The block anatomy should be:

```text
Slider:      [ selected segment: label             ][ remainder: value ]
RangeSlider: [ remainder ][ lower handle | selected window | upper handle ][ remainder ]
```

The visual has one track, one or two selected-fill segments, text layers, and an optional readout/tooltip layer. The behavior still has one Slider control or two RangeSlider controls. Text must not become a separate pointer target or obscure the track geometry.

The reference’s black selected segment is a visual cue, not a token requirement. Use semantic/recipe roles for selected fill, remainder, label text, value text, handle, border, and focus. Theme-specific black/grey literals would conflict with the token architecture and fail on some themes.

### 2. Geometry must be normalized once and interpreted by every runtime

**[LF]** Core and Rust already publish normalized positions and fill spans. Slider has value/center/fill-start/fill-span; RangeSlider has lower/upper/center and negative/positive spans. Svelte and React derive positions from the root bounding box; Rust uses a full-width scrub overlay for the horizontal axis.

**[WI]** Promote a block presentation geometry, after operator approval, with these invariants:

1. Normalize values through the existing law and bounds to `[0,1]`.
2. Map the physical track between the effective interaction extents, not blindly between visible capsule edges. The visible thumb may overhang the track, while the hit wrapper remains at least 44px.
3. Use the same direction convention in all runtimes: horizontal minimum at the inline-start numeric origin and maximum at the inline-end numeric origin; vertical minimum at the bottom and maximum at the top.
4. Treat zero-width fills as real states. Do not manufacture a one-pixel selected region that implies a value change.
5. Keep content layout separate from value geometry. Text-fit thresholds are presentation decisions and must not change the value law.

This gives Svelte, React, Rust, and GPUI the same normalized positions while allowing native controls and GPUI nodes to use their own layout primitives.

### 3. Inline label/value placement needs a deterministic collision policy

The image works because the label is short, the selected segment is wide enough, and the value sits in the remainder. At value 0, near 0, near 100, or a narrow RangeSlider window, that assumption fails.

**[SF]** The APG multi-thumb example keeps the current value adjacent to the focused thumb for magnification users, and Spectrum treats value output as part of slider anatomy. See the [APG example](https://www.w3.org/WAI/ARIA/apg/patterns/slider-multithumb/examples/slider-multithumb/) and [Spectrum API](https://opensource.adobe.com/spectrum-web-components/components/slider/api/).

**[WI] Recommended fit ladder:**

- At a measured/defined minimum inline width, show the Slider label in the selected segment and the formatted value in the remainder.
- If either segment cannot fit its content after insets, collapse the block to one stable external readout associated with the control. Do not ellipsize a numeric value into ambiguity, let text paint outside the capsule, or allow label/value layers to overlap.
- While a thumb is focused or dragged, a small attached readout/tooltip may supplement the stable readout. It must not be the only value channel.
- For RangeSlider, show lower and upper values next to their handles only when both labels fit. Otherwise show one combined range output and retain per-thumb `aria-valuetext` and focus-specific supplemental output.

The recommended fallback is a single stable readout rather than a per-value layout that jumps independently. It is easier to test, localize, and mirror in GPUI. Whether that readout is rendered outside or directly above the capsule is an operator API/layout decision.

Do not infer visible label text from `ariaLabel`. `ariaLabel` is the accessible name; block appearance should have an explicit visible label/value content contract. Reusing `valueText` as visible output is plausible for formatted values, but it must be decided explicitly because some current consumers supply `valueText` only for assistive technology.

### 4. RangeSlider needs two controls even when the visual looks like one block

**[SF]** The WAI-ARIA multi-thumb pattern requires distinct slider semantics, independent focus, stable tab order, and dependent bounds. [Radix](https://www.radix-ui.com/primitives/docs/components/slider) and [React Aria](https://react-aria.adobe.com/Slider) also model range values as multiple handles rather than one anonymous range widget.

**[WI]** Block RangeSlider rules should be:

- lower and upper remain separate semantic controls, named “minimum” and “maximum” or their explicit equivalents;
- tab order remains lower then upper, even after the upper value is visually left of the lower value in an RTL or transformed layout;
- dragging a handle clamps at its sibling; no swap in the first block appearance;
- at pointer-down, choose the nearer thumb; at an exact tie, use a deterministic rule (current core chooses lower on `<=`), then hold the chosen thumb for the gesture;
- the active/focused thumb receives a higher visual layer and focus ring, but the other thumb remains visible and targetable;
- equal values render a deliberate overlap state: two semantic hit targets, a visible active handle/ring, and a stable keyboard route. Do not rely on two identical pixels to communicate two controls;
- whole-window dragging is a separate interaction. It should remain a non-goal until a separate contract defines how pointer capture, keyboard focus, and accessibility describe moving both values together.

The current contract and Rust renderer differ in historical comments about whether only thumbs or the full track are draggable. The bounded recommendation for block appearance is full-track nearest-thumb targeting because it is already the documented contract and is implementable in all active runtimes. The selected window itself is not a drag target in this phase.

### 5. Small visible thumb, large effective target

**[SF]** WCAG 2.2 AA target size is 24×24 CSS px with exceptions; the enhanced criterion is 44×44. Poodle’s `docs/contracts/components/size-and-density.md` adopts 44×44 as the minimum for controls and says a smaller visible area must compensate. See [Target Size Minimum](https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html) and [Target Size Enhanced](https://www.w3.org/WAI/WCAG22/Understanding/target-size-enhanced.html).

**[LF]** Current visual thumbs are 0.75–1.25rem and current Slider/RangeSlider roots are 1.25–1.75rem at the size ladder. The current styles do not establish an independently measurable 44×44 thumb wrapper. Density padding improves some roots but is not a per-thumb guarantee.

**[WI]** The block contract should separate:

- visible handle diameter, controlled by size and recipe;
- pointer/touch hit wrapper, at least 44×44 CSS px;
- keyboard focus box, which must visibly identify the focused thumb without requiring a large visible circle.

For web, the wrapper can be a transparent native-input/control layer over the visual handle. For Rust/GPUI, it can be a larger interaction node or overlay with a smaller painted child. The track remains a broad target, so the wrapper must have clear z-order and deterministic tie behavior when two targets overlap.

Do not meet the target only with a tooltip or by increasing the whole capsule’s height. The thumb’s effective target and the two thumbs’ overlap behavior need direct tests.

### 6. Pointer, touch, interruption, and cancellation

**[SF]** Pointer Events provides pointer capture for a continuing gesture, `pointercancel` when the stream is suppressed, and `lostpointercapture` when capture ends. `touch-action` controls whether the browser may pan/zoom the surface; decisions are made at gesture start. See [Pointer Events 3](https://www.w3.org/TR/pointerevents3/).

**[LF]** Svelte and React call `setPointerCapture`, handle `pointerup` and `pointercancel`, and do not listen for `lostpointercapture` (`Slider.svelte:106-118`, `RangeSlider.svelte:116-122`, React equivalents). Embedded CSS sets `touch-action: none` on the root. The core machines have no cancellation-specific event.

**[WI] Recommended interaction contract:**

- one primary pointer gesture starts on the effective track/handle target and sends normalized positions to the shared machine;
- capture the active pointer on the stable root/interaction layer;
- `pointerup`, `pointercancel`, `lostpointercapture`, teardown, and stale-pointer cleanup all enter one idempotent terminal path;
- the terminal path clears active state exactly once and never emits duplicate commit effects;
- cancellation terminates the current accepted gesture and commits the last accepted pair once, matching the current architecture’s release/cancel terminal rule and the absence of a rollback callback. A future transactional/revert model would need a separate contract; it must not be inferred from pointer cancellation;
- disabled controls reject begin and move and produce no value mutation;
- for horizontal web controls, prefer `touch-action: pan-y` when vertical page scrolling must remain available; for vertical controls, prefer `pan-x`. Reserve `none` for a documented surface that owns both axes. The current unconditional `none` is a migration risk for embedded controls in scrollable layouts;
- pointer movement updates only normalized geometry and live value effects. Formatting, tooltip placement, and expensive layout should be cached or throttled to the host’s rendering model, not rebuilt as a new semantic tree for every event.

This is intentionally more explicit than “mouseup/touchend.” It covers pen, touch interruption, browser gesture takeover, DOM removal, and native backend capture loss.

### 7. Keyboard and touch-assistive technology semantics

**[SF]** The APG slider pattern specifies Right/Up to increase, Left/Down to decrease, Home to minimum, End to maximum, and optional PageUp/PageDown. It also warns that custom slider touch interaction varies across assistive technologies and recommends testing actual platform combinations. Native HTML range inputs supply stronger browser semantics. See [APG Slider](https://www.w3.org/WAI/ARIA/apg/patterns/slider/), [ARIA 1.2](https://www.w3.org/TR/wai-aria/), and [HTML range](https://html.spec.whatwg.org/multipage/input.html).

**[LF]** Current Poodle maps Left/Down to decrement and Right/Up to increment across orientations, with Home/End. The custom embedded handlers do not implement PageUp/PageDown. The standard Svelte and React paths are not identical for RangeSlider commit timing.

**[WI]** Keep the current key meaning for the additive block mode. It is simple, matches APG value semantics, and avoids depending on the user’s physical orientation. Before promotion, decide whether block custom controls add a common PageUp/PageDown step. The bounded recommendation is one tenth of the usable range, rounded to the declared step and never smaller than one step, with PageUp increasing and PageDown decreasing. If the operator declines that addition, document PageUp/PageDown as optional in every runtime rather than allowing accidental browser/runtime differences.

The web block renderer should retain native range inputs where possible. If custom roles are needed for the visual/content layer, it must still expose role, name, min, max, now, value text, orientation, disabled state, focus, and keyboard behavior per thumb. A visible block label is not a replacement for per-thumb names.

### 8. Orientation and direction

**[SF]** ARIA’s slider pattern treats orientation as a semantic property for custom sliders; the default is horizontal. CSS and native controls can represent vertical ranges, but transforming a horizontal input is a rendering technique, not proof that native/runtime semantics and pointer coordinates agree. See [ARIA 1.2](https://www.w3.org/TR/wai-aria/) and [APG Slider](https://www.w3.org/WAI/ARIA/apg/patterns/slider/).

**[LF]** Web CSS has vertical Slider and RangeSlider rules. Rust Slider has a vertical scrub axis; Rust RangeSlider explicitly defers vertical geometry and samples a horizontal overlay. GPUI Range specimens include a vertical case, but the renderer remains horizontal.

**[WI]** Block horizontal and vertical geometry should use one logical axis descriptor:

- horizontal: x position, inline label/value fallback;
- vertical: y position with minimum at the bottom, maximum at the top, and a side/adjacent readout that does not rotate text;
- no CSS transform as the only native implementation for an admitted cross-runtime vertical RangeSlider;
- no silent fallback from requested vertical to horizontal.

RTL is unresolved. The current public contracts have orientation but no `dir`/writing-direction input. Do not invent RTL semantics in the block renderer. Decide whether Poodle’s value-increase key mapping remains fixed or whether physical Left/Right follows direction, then add it to the shared contract and test matrix.

### 9. Marks, ticks, and tooltips

**[LF]** Marks and value labels are explicitly out of scope in `docs/contracts/components/slider.md` and `range-slider.md`. Current specimens do not expose marks or tooltips.

**[SF]** MUI and Spectrum demonstrate that marks, tick labels, formatting, and value-label display are separate, configurable features rather than an unavoidable part of the base slider. See [MUI Slider](https://mui.com/material-ui/react-slider/) and [Spectrum Slider API](https://opensource.adobe.com/spectrum-web-components/components/slider/api/).

**[WI]** Do not add marks to the first block scope. If a later track adds them, marks should be data derived from the same min/max/step axis, with a density/label-overlap rule and no requirement that every mark label be visible. Marks must not steal pointer ownership from the slider.

Tooltips/readouts are useful for a small visible thumb, but should be supplemental:

- show the active thumb’s formatted value on focus-visible and during drag;
- show on hover only when the platform reports hover capability, never as the only value output;
- for RangeSlider, identify lower/upper in the tooltip or show a combined range when the handles overlap;
- keep the value in `aria-valuetext` or an explicit stable output even when the tooltip is suppressed, delayed, or unavailable to touch/keyboard users;
- do not animate the value itself.

### 10. State coverage

The current contracts name default, focus-lower/focus-upper, active, and disabled. Block appearance should make those states visually testable without inventing unrelated component states.

| State | Slider | RangeSlider | Block treatment |
| --- | --- | --- | --- |
| Resting | one value | lower/upper pair | selected/remainder contrast, readable content, visible handle(s) where applicable |
| Hover | optional pointer affordance | per-handle/track affordance | supplemental only; never the sole indication |
| Focus-visible | one control | exactly one of two controls | external or adjacent focus indicator with 3:1 non-text contrast; active thumb raised when overlapping |
| Pressed/dragging | active gesture | active thumb identity | stable active state, no text jump, one live stream and one terminal effect |
| Disabled | no mutation | neither thumb mutates | visibly disabled but still structurally understandable; no pointer capture/focus entry |
| Invalid/read-only/indeterminate | not current props | not current props | separate field/control contract; do not smuggle these into block appearance |

**[SF]** WCAG requires non-text contrast for visual component/state indicators and visible focus. See [non-text contrast](https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast) and [focus appearance](https://www.w3.org/WAI/WCAG22/Understanding/focus-appearance.html). **[WI]** The active fill, remainder boundary, handle border, and focus indicator each need an explicit state/contrast assertion; color alone must not carry lower-versus-upper identity or active-versus-disabled meaning.

Current `data-state="active"` is pointer-oriented. Keyboard adjustment can change the value without entering that state. A block contract should define whether keyboard adjustment uses the same pressed styling or only focus styling; do not infer it from current CSS.

### 11. Forced colors and contrast

**[SF]** In forced colors, user agents may replace author colors with system colors. CSS Color Adjustment and CSS Color 4 recommend system color keywords and warn that mixing author colors with system colors is not a general contrast guarantee. See [CSS Color Adjustment](https://drafts.csswg.org/css-color-adjust-1/) and [CSS Color 4](https://drafts.csswg.org/css-color-4/).

**[LF]** The current Slider and RangeSlider styles have no forced-colors branch. They use author color tokens, semi-transparent `color-mix`, shadows, and a 32% focus-ring mix. This is a gap to validate before adding a high-information text-bearing block treatment.

**[LF] Local token spot check.** I calculated WCAG relative-luminance ratios from the authored schema under `packages/tokens/schema/primitives/color.json`, `semantic/color.json`, and all 12 `modes/themes/*.json` files. The result is a risk signal, not a complete rendered-theme audit:

- the current light neutral default border `#75869b` against the light surface `#dbe5ef` is about **2.92:1**, below the 3:1 non-text threshold;
- the muted/tertiary text pair has the same roughly **2.92:1** ratio on that surface, below 4.5:1 normal text;
- inverse text on theme accent fill is as low as **3.62:1** in `clay` and **3.82:1** in `meadow`, below 4.5:1 normal text;
- a full-opacity focus color can exceed 3:1 against its theme canvas, but the current semi-transparent focus mix must be tested after compositing, not assumed to inherit that ratio.

**[WI]** Block recipes should use explicit foreground/background role pairs and a generated or focused rendered contrast matrix. Do not hard-code black active fill plus white label as the universal solution. A viable default can use a neutral selected fill in some themes and an accent selected fill in others if the pair remains legible. Forced-colors defaults should map selected/remainder/focus/handle to system colors such as `Highlight`, `HighlightText`, `Canvas`, `CanvasText`, `ButtonText`, and `GrayText`, with no `forced-color-adjust: none` unless every part is deliberately remapped.

### 12. Reduced motion

**[LF]** Current component CSS has no Slider/RangeSlider transition or animation. That is a sound baseline for value feedback.

**[SF]** `prefers-reduced-motion: reduce` is the platform preference for reduced motion. See [Media Queries Level 5](https://drafts.csswg.org/mediaqueries-5/#prefers-reduced-motion).

**[WI]** The block appearance should remain motion-free by default. If a future tooltip, active fill, or focus treatment animates, disable non-essential transform/opacity animation under `prefers-reduced-motion: reduce`; never delay live value feedback or focus visibility. Native GPUI should use the same static default, with system motion preference mapping treated as an explicit native decision.

### 13. Tokens and appearance recipes

**[LF]** The current recipe inventory has 10 Slider hooks and 11 RangeSlider hooks. It does not have block-specific label/value/handle roles. The architecture permits additive component-scoped recipe hooks but warns against exposing internal metrics as public override surface.

**[WI]** Add no token source or generated artifact in this research. If the direction is promoted, first decide whether one shared block role map can serve both components or whether Slider and RangeSlider need distinct hooks. The likely stable roles are:

| Role | Why it is stable | Metric that must stay internal |
| --- | --- | --- |
| selected/active fill | visual meaning of the chosen value/window | exact split threshold or segment flex formula |
| remainder/track fill | visual meaning of available range | pixel width and fit breakpoint |
| inline label text | content-bearing foreground | font-measurement algorithm |
| inline value text | formatted value foreground | formatter cache and truncation width |
| handle fill/border | control affordance and overlap | hit-wrapper diameter |
| focus ring | keyboard state | browser/native focus implementation |
| hover/pressed overlay | pointer state | pointer sampling cadence |
| forced-colors mapping | platform fallback | UA-specific system palette quirks |

Recipe defaults should resolve through semantic token roles. The block API should not expose a consumer-facing `black`, `grey`, or fixed text-fit threshold. Any new recipe hook must be demonstrated in both web and native mappings or be explicitly documented as web-only.

### 14. Performance and rendering cost

**[LF]** The current core path computes a bounded normalized state and a small effect list. The web paths update controlled values and CSS custom properties; the Rust renderer builds a finite node tree containing track/fill/thumb/overlay nodes. `docs/specs/038-performance-render-cost-and-memory-hardening-baseline.md` requires avoiding repeated full-surface recompute and making no unmeasured benchmark claims.

**[WI]** A block implementation should preserve that shape:

- one track and a bounded number of fill/text/handle nodes;
- CSS custom properties or equivalent style fields for position/span updates;
- no per-pointer event DOM insertion/removal;
- no uncached locale/formatting work in the pointer-move hot path;
- one optional tooltip/readout node reused through the gesture;
- Rust/GPUI live updates that do not allocate a new semantic subtree per move;
- a focused performance probe measuring pointer-move render work, DOM/node count, and controlled React rerender behavior before any performance claim is made.

The visual’s large capsule should not become a large canvas or bitmap. It is a small finite composition.

### 15. Migration and consumer risk

**[WI]** Additive appearance is the safest path because:

- existing standard Slider and RangeSlider geometry stays intact;
- existing `ariaLabel`/`valueText` consumers do not gain visible content unexpectedly;
- raw `.poodle-slider` audio visuals do not inherit a new text/handle anatomy;
- preview snapshots and current public examples retain their current default;
- downstream products can opt in one surface at a time after visual and accessibility review.

Replacement default is not safe without a separate migration plan. The block treatment changes minimum height, text layout, hit layering, focus visuals, value-label assumptions, and possibly pointer capture. A recipe-only change would avoid an API change but cannot promise cross-runtime anatomy or collision behavior. A consumer wrapper would reduce core risk but fork the shared contract and undermine Poodle’s cross-runtime direction.

Pre-v1 policy allows deliberate breaking changes but does not allow silent compatibility shims or fallbacks. If the operator selects a different public prop name or changes the existing default, the contract and migration record must name affected source paths and downstream review. This dossier recommends no alias and no silent vertical fallback.

## Options And Tradeoffs

### Appearance admission

| Option | Benefit | Cost/risk | Disposition |
| --- | --- | --- | --- |
| Replace current default with block | Fast visual convergence; one canonical screenshot direction | Breaks current consumers, preview assumptions, embedded raw visuals, focus/hit layout, and possibly native parity; forces migration before evidence | Reject for now |
| Add explicit block appearance | Preserves default and permits staged web/native evidence; matches additive recipe policy | Adds a public appearance decision and a second visual mode; requires API naming and specimen matrix | **Recommend** |
| Recipe-only styling | Minimal API surface; useful for color/shape experiments | Cannot define inline content, fallback/readout, two-thumb collision, hit layering, or native composition | Use only for token polish inside an approved appearance |
| Consumer-owned wrapper | Fast experiment for one app | Duplicates semantics/geometry and violates the shared contract goal if promoted as Poodle behavior | Disposable prototype only |

### Text fit and value display

| Option | Benefit | Cost | Disposition |
| --- | --- | --- | --- |
| Clip/ellipsis inside segments | Keeps screenshot silhouette | Loses exact values, fails localization and narrow ranges | Reject |
| Let text overflow capsule | Preserves content | Overlap/stacking and hit-testing become unpredictable | Reject |
| Always external readout | Strongest readability and accessibility | Less faithful to the reference; more vertical/side layout | Valid fallback, especially for narrow layouts |
| Inline when it fits, otherwise one stable external readout | Preserves the visual direction while remaining deterministic | Requires fit measurement or a documented width heuristic | **Recommend** |
| Focus/drag tooltip only | Low resting footprint | Hover/touch/keyboard and magnification users can miss it; not sufficient as value channel | Supplemental only |

### Range interaction

| Option | Benefit | Cost | Disposition |
| --- | --- | --- | --- |
| Thumb-only drag | Avoids accidental changes to the wrong bound | Track feels inert; inconsistent with current documented nearest-thumb behavior | Not first block policy |
| Full-track nearest-thumb, then hold | Fast and discoverable; shared Rust overlay can mirror it; preserves thumb identity | Requires overlap/tie and z-order rules | **Recommend** |
| Drag selected window | Useful for range editing | Adds paired movement, keyboard/focus semantics, and a new accessibility story | Separate future contract |
| Swap thumbs on crossing | Familiar in some libraries | Changes lower/upper identity and tab/announcement stability | Reject for current Poodle invariant |

## Recommended Poodle Direction

This is the bounded direction to take to operator review. It is not yet a public contract.

### Proposed block behavior

1. **Opt-in appearance.** Add a named appearance mode for Slider and RangeSlider after the operator chooses the public name. Keep the current standard and embedded defaults unchanged. Prefer a semantic appearance name such as `block`; do not overload the existing `variant` values without a contract decision.
2. **Shared value laws.** Reuse the existing core normalization, snapping, polarity, center, and RangeSlider no-crossing machine. The block renderer consumes normalized geometry; it does not rederive value math.
3. **Slider anatomy.** Render one rounded capsule with selected and remainder segments, a visible label slot, a formatted value slot, a focusable effective control, and an optional supplemental readout. The label/value content is explicit and does not replace `ariaLabel`.
4. **RangeSlider anatomy.** Render the selected window between lower/upper positions, two visible-but-subordinate handles, per-thumb focus/hit layers, and lower/upper content when it fits. When it does not, use one combined stable readout plus focused-thumb supplemental output. Keep two semantic controls.
5. **Interaction.** Full track chooses the nearest thumb on begin; the chosen thumb remains active until terminal cleanup. No swap, whole-window drag, min-distance, marks, or more than two thumbs in the first scope.
6. **Overlap.** Exact ties choose lower by the current deterministic rule unless the operator selects last-focused. Focus/active thumb is raised; both controls remain discoverable via Tab and keyboard.
7. **Targeting.** Visible thumb size follows the existing size ladder; effective handle target is at least 44×44 CSS px. Track hit area and handle hit area are separate in geometry and z-order.
8. **Keyboard.** Preserve Left/Down decrement, Right/Up increment, Home/End, stable lower-then-upper Tab order, per-thumb names/bounds/value text, and orientation semantics. Decide PageUp/PageDown before promoting custom block controls; recommended common page is one tenth of the usable range rounded to step.
9. **Pointer cancellation.** Add one shared terminal policy for release, cancel, lost capture, and teardown. Recommended outcome is one final commit for the last accepted value, no duplicate, no rollback. Add rollback only as a separate transaction contract.
10. **Orientation.** Support the same logical geometry for horizontal and vertical. Do not admit native vertical RangeSlider until Rust/GPUI geometry, scrub axis, semantics, and mounted evidence are complete. Do not silently render a requested vertical control horizontally.
11. **States.** Cover resting, optional hover, focus-visible per control, pressed/dragging, and disabled. Keep invalid/read-only/indeterminate out of this appearance contract.
12. **Colors and motion.** Use semantic recipe roles and theme-aware foreground/background pairs. Add forced-color system mappings and a contrast matrix. Keep value feedback static by default; reduce any future non-essential animation under `prefers-reduced-motion: reduce`.
13. **Runtime posture.** Svelte and React can use native range semantics behind a shared visual/content layer. Rust/GPUI must build the same finite anatomy and per-thumb semantic nodes. Jetstream remains deferred.

### Proposed contract shape, pending naming

The future contract should explicitly decide fields equivalent to:

- appearance mode;
- visible single label and single value content;
- lower and upper visible value content for RangeSlider, plus combined fallback behavior;
- optional readout/tooltip policy, if it is a component prop rather than a wrapper concern;
- shared geometry/state semantics for focus, active thumb, overlap, and orientation.

Do not silently treat current `ariaLabel`, `valueText`, `lowerValueText`, or `upperValueText` as visible content. Some current consumers use these exclusively for assistive technology. A deliberate API can offer a convenience mapping later, but the semantic and visual channels must remain distinct.

## Explicit Non-Goals

- Pixel imitation, screenshot matching, or importing the supplied image as an asset.
- Implementing a shipped component, public API, CSS, token, renderer, test, generated artifact, or consumer migration in this research task.
- Replacing the current Slider/RangeSlider default.
- Adding marks, tick labels, a mark data model, or a tooltip-only value contract to the first block scope.
- Adding whole-window drag, thumb swapping, configurable minimum distance, editable hot text, or more than two RangeSlider thumbs.
- Inventing read-only, invalid, or indeterminate states without a governing field/control contract.
- Making custom tooltips the only value delivery path.
- Claiming GPUI assistive-technology parity while the recorded GPUI accessibility blocker remains.
- Admitting Jetstream to this visual cohort.
- Running or requiring `*-windowed` conformance selectors locally.
- Claiming a performance improvement without a focused measurement.

## Risks

| Risk | Evidence/impact | Required mitigation |
| --- | --- | --- |
| Segment text collision | Reference assumes wide segments; endpoint and narrow values invalidate that assumption | Stable fit ladder; endpoint/narrow/range-overlap specimens; localization strings |
| Two-thumb disappearance | Equal or near-equal positions can paint one pixel/handle | Separate semantic targets, active z-order, focus/overlap state, direct pointer/keyboard tests |
| Small visual target | Current size ladder is below 44px and no per-thumb wrapper is explicit | Measure effective target, not just root bounds; test touch/pointer coordinates at every size/density |
| Web event drift | Svelte/React Range commit paths differ; no lost capture | Shared lifecycle trace across runtimes; add pointercancel/lostpointercapture cases |
| Scroll interference | Current embedded root uses `touch-action: none` | Choose axis-specific `touch-action` per host; test nested scroll surfaces |
| Native vertical gap | Rust RangeSlider scrub is explicitly horizontal/deferred | Close native geometry and semantics before vertical admission; no fallback |
| Native AX blocker | GPUI 0.2.2 lacks proven AX API/tree | Carry node metadata and document evidence as manual/blocker; do not overclaim |
| Contrast failure | Local spot check finds 2.92:1 border/tertiary and 3.62:1 minimum inverse/accent pair | Rendered theme matrix, forced-colors check, dedicated foreground/background roles |
| Forced-color override | Author `color-mix`/shadow can disappear or lose contrast | System-color branch; do not rely on author colors in forced colors |
| Motion preference | Future tooltip/active polish could animate | Static default; explicit reduced-motion branch and manual check |
| Token sprawl | Block adds text/state roles to an existing 10/11-hook inventory | Add only stable demonstrated roles; keep geometry thresholds internal |
| Consumer breakage | DisplayControls, ColorPicker, raw audio visual, public previews, and external roster names use current surfaces | Additive opt-in; downstream visual/a11y review before any default change |
| Performance regression | Controlled React and native retained trees can rerender/rebuild during live drag | Bounded node tree, cached formatting, pointer-move probe |
| Localization and formatting | `67 px`-style output can be longer, bidi-sensitive, or unavailable | Explicit visible content/readout contract; test long values, units, and RTL before API freeze |

## Unresolved Operator Decisions

These are decisions, not gaps this worker should silently resolve.

1. **Public name:** `appearance="block"`, a new variant, or a wrapper-only experiment? Recommendation: an explicit additive appearance; operator chooses the exact name.
2. **Visible content API:** separate `label`/`displayValue` fields, or a documented mapping from existing value-text props? Recommendation: separate visual content from ARIA naming/value text.
3. **Fit fallback:** external stable readout, above-track readout, or another bounded wrapper? Recommendation: collapse to one stable readout when either segment cannot fit.
4. **Range pointer policy:** full track nearest-thumb or thumb-only? Recommendation: full track nearest-thumb, then hold; no window drag initially.
5. **Exact overlap tie:** current lower-on-`<=` or last-focused? Recommendation: retain current lower tie for deterministic cross-runtime behavior unless product evidence favors last-focused.
6. **PageUp/PageDown:** common 10%-of-range behavior for custom block controls or optional/browser-owned only? Recommendation: make one common snapped page step part of the block contract if cross-runtime keyboard parity is required.
7. **RTL:** should Left/Right follow numeric increase/decrease or physical direction? The current API has no direction prop; decide before advertising RTL.
8. **Vertical admission:** block web vertical can be prototyped, but should full block promotion wait for native RangeSlider vertical? Recommendation: yes, for the active cohort; no silent runtime-specific fallback.
9. **Forced-color ownership:** web CSS system mappings only, or an equivalent native palette role in shared recipes? Recommendation: semantic roles in shared contract, platform mapping per renderer.
10. **Cancellation commit:** commit last accepted value on cancel/lost capture, or introduce rollback? Recommendation: one final commit under current API; rollback is separate scope.
11. **Read-only/invalid state:** remain wrapper-owned or become Slider/RangeSlider props? Recommendation: remain out of block scope until a governing field contract exists.
12. **Default migration:** no replacement decision now. If replacement is later proposed, require an explicit downstream inventory, migration record, visual/a11y evidence, and operator approval.

## Promotion-Ready Contract, Architecture, And Card Scope

The following is enough to shape promotion work without changing those documents in this research task.

### Promotion gate

Promote only after the operator records decisions 1–10 above, confirms additive appearance, and accepts that GPUI native AX evidence remains a named blocker until the current contract changes. Keep the originating triage note open until that decision is recorded.

### Card 1 — Contract and architecture translation

Update only after approval:

- Slider and RangeSlider contracts with appearance name, visible content channels, fit fallback, overlap/tie, target geometry, cancellation terminal semantics, keyboard Page policy, orientation/direction, and state table;
- shared appearance recipe contract with stable block roles and web-only forced-color/motion selectors called out;
- shared Rust spec/headless shape for appearance and serialized block geometry;
- explicit cross-runtime admission and GPUI evidence boundary.

Acceptance: no unresolved behavior is hidden in CSS or a runtime adapter; standard/embedded defaults remain byte/behavior compatible.

### Card 2 — Core geometry and gesture law

Implement later in the shared core only:

- normalized axis geometry for Slider and RangeSlider;
- lower/upper active identity, tie rule, no-crossing, and overlap state;
- pointer begin/move/end/cancel/lost-capture-compatible terminal semantics;
- one commit effect per accepted gesture terminal;
- optional common PageUp/PageDown law if approved.

Acceptance: core traces cover zero/mid/full, narrow/equal ranges, pointer interruption, stale pointer, disabled, keyboard, bipolar center, and no duplicate effects.

### Card 3 — Web Svelte and React block renderer

- Preserve native range inputs where possible for standard semantics.
- Add explicit effective hit wrappers and per-thumb layering.
- Align Svelte/React Range commit and cancellation traces.
- Implement inline fit ladder, stable fallback output, focus/drag supplemental readout, orientation, and axis-specific `touch-action`.
- Add forced-colors and reduced-motion CSS checks.

Acceptance: Svelte and React produce equivalent semantic inputs, state transitions, live/commit effects, focus order, hit behavior, and block geometry for the active matrix.

### Card 4 — Rust renderer and GPUI

- Add block node composition from finite track/fill/text/handle/readout nodes.
- Project per-thumb role, label, min, max, now, value text, orientation, disabled, focus, and tab identity.
- Close native RangeSlider vertical geometry and scrub axis.
- Match effective hit geometry and overlap z-order without CSS-only assumptions.
- Record GPUI AX status as manual/blocker until `docs/contracts/003-native-accessibility.md` permits stronger evidence.

Acceptance: headless and mounted tests prove behavior; visual evidence proves geometry; no `*-windowed` selector is required locally.

### Card 5 — Tokens and recipes

- Add only approved stable block roles to authored token/recipe sources.
- Regenerate through the repository token process; never hand-edit generated files.
- Map all active themes, sizes, and densities.
- Add forced-colors system mappings and rendered contrast matrix.

Acceptance: semantic foreground/background pairs pass text and non-text contrast checks in all themes, including active/focus/disabled/overlap states.

### Card 6 — Specimen and evidence matrix

Both web previews and GPUI specimens should show:

- Slider at 0, near 0, 50, near 100, and 100;
- RangeSlider `[0,0]`, narrow/equal, narrow non-equal, `[0,25]`, `[45,55]`, `[75,100]`, and `[0,100]`;
- standard and embedded, unipolar and bipolar, all sizes, all densities, horizontal and vertical;
- resting, hover where supported, focus-lower/focus-upper, dragging, disabled, forced colors, reduced motion, long/localized values, and missing optional visible content;
- pointer track click, handle drag, touch scroll interaction, pointercancel, lost capture, keyboard, overlap, and disabled rejection;
- Svelte/React focused tests, core traces, Rust renderer tests, GPUI headless mounted regressions, and manual native accessibility record.

Acceptance: parity evidence states exactly which cells are automated, mounted, visual/manual, or blocked; no browser axe result is presented as GPUI assistive-technology proof.

### Card 7 — Consumer opt-in and migration

- Opt in one preview surface and one real internal consumer after evidence passes.
- Audit `DisplayControls`, ColorPicker, raw ModMatrix visual selectors, and named downstream roster products.
- Do not change the default in this card.
- If a later default replacement is proposed, create a separate migration record with affected paths, screenshots, accessibility review, and rollback plan.

## Proposed Disposition Of Originating Triage

**Keep `docs/triage/20260831-155151-block-slider-visual-direction.md` open, with disposition `research complete — awaiting operator decisions and promotion`.** The visual direction is bounded and the strongest interaction model is recommended, but API naming, visible content channels, fit fallback, PageUp/PageDown, RTL, cancellation commit, vertical native admission, and default replacement are still operator decisions. After those decisions, the next move is the contract/architecture translation card above; no triage closure or default migration should happen before that promotion gate.

## Citations And Licence Notes

The consequential external rules are cited at the point of use above. The normative sources were checked on 2026-08-31 and their W3C/WHATWG licence text was inspected. The precedent licence records checked on that date are:

- [W3C Software and Document License](https://www.w3.org/copyright/software-license-2023/);
- [WHATWG HTML acknowledgements and licence](https://html.spec.whatwg.org/multipage/acknowledgements.html);
- [Radix MIT licence](https://raw.githubusercontent.com/radix-ui/primitives/main/LICENSE);
- [MUI MIT licence](https://raw.githubusercontent.com/mui/material-ui/master/LICENSE);
- [React Spectrum Apache 2.0 licence](https://raw.githubusercontent.com/adobe/react-spectrum/main/LICENSE).

No code, screenshots, icons, or other third-party assets were copied into Poodle. The image supplied with the triage request informed only the described capsule anatomy.
