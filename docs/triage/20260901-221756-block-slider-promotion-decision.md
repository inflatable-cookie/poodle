# Block Slider And RangeSlider Promotion Decision Proposal

Status: accepted decision — promoted as `g16.046`
Disposition: additive horizontal block appearance; vertical RangeSlider held
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Scope: additive block appearance for `Slider` and `RangeSlider`
Promotion authority: orchestrator after operator acceptance; merge is intake only

This packet proposes exact resolutions for the choices named by
`docs/handoffs/20260901-221756-block-slider-promotion-planning.md`. It is not
contract, roadmap, implementation, or merge authority.

## Settled Decisions Preserved

- The appearance is additive. Current standard and embedded defaults remain.
- Visible label/value content is separate from accessible names/value text.
- Inline content appears only when it fits. One stable fallback readout owns
  the narrow case.
- RangeSlider chooses the nearest thumb at pointer-down and holds it for the
  gesture.
- Visible thumbs may be small. Each effective target remains measurable at
  Poodle's adopted minimum.
- Full vertical admission waits for real native RangeSlider geometry.
- Architecture 012 and g16.034 remain motion authority.
- Jetstream remains deferred. No default migration is in scope.

## Delegate Authority

No operator conversation occurred in this delegate thread. The handoff said to
use the dossier recommendation where no contrary operator preference exists.
That instruction authorizes an exact delegate proposal; it does not make the
proposal an operator decision.

Every exact name and law below is a delegate recommendation. The operator must
accept, revise, or reject the proposed surface before canonical promotion. PR
merge is intake for that review, not acceptance of the API or behavior.

## Proposed Contract Surface

### Appearance

Use one shared public type and field on both components:

```ts
type SliderAppearance = "track" | "block";

appearance?: SliderAppearance; // default: "track"
```

- `appearance="block"` is the public opt-in name.
- `track` names today's visual treatment. It is not a compatibility alias.
- The same `SliderAppearance` type applies to Slider and RangeSlider.
- `appearance` is orthogonal to `variant="standard" | "embedded"`.
- Omitted `appearance` preserves current structure, behavior, and styling.
- Do not add `block` to `variant`; variant continues to select the interaction
  substrate.

### Visible Content Inputs

Use explicit visual channels. Never derive them from `ariaLabel`, `valueText`,
`lowerValueText`, or `upperValueText`.

```ts
// Slider
visibleLabel?: string | null;
formatVisibleValue?: (value: number) => string;

// RangeSlider
visibleLabel?: string | null;
formatVisibleValue?: (value: number, thumb: "lower" | "upper") => string;
formatVisibleRange?: (lower: number, upper: number) => string;
```

Formatter law:

- Inputs are the current normalized, bounds-guarded, step-snapped values.
  Raw pointer coordinates and unsnapped values never reach a formatter.
- Slider calls `formatVisibleValue(value)`.
- RangeSlider calls `formatVisibleValue` once for each thumb. The second
  argument is stable value identity, not visual position.
- RangeSlider calls `formatVisibleRange(lower, upper)` for the combined
  fallback readout.
- Without `formatVisibleValue`, default endpoint text is `String(value)`.
- Without `formatVisibleRange`, default range text joins the two resolved
  endpoint strings with `" – "`.
- An empty formatter result or empty `visibleLabel` means that item is absent.
- Formatter results are cached by accepted value and formatter identity. Text
  measurement may not invoke them repeatedly during one pointer sample.

Rust specs carry resolved data rather than closures:

- Slider: `visible_label`, `visible_value_text`.
- RangeSlider: `visible_label`, `lower_visible_value_text`,
  `upper_visible_value_text`, `range_visible_value_text`.

This is the same accepted web-formatter/native-resolved-string pattern used by
AgentQuestion, ChangedFiles, and ToolCallGroup. The semantic inputs and output
meaning remain shared.

### Inline Anatomy And Fit

The block anatomy assigns content to fixed regions:

- Slider: label in the selected segment; value in the remainder.
- RangeSlider: label in the selected window; lower value in the leading
  remainder; upper value in the trailing remainder.
- Empty items do not reserve a region.
- Text is paint only. It cannot become a pointer target or change value
  geometry.

Fit is all-or-nothing. Every non-empty item must fit its assigned region or all
inline text moves to the fallback readout.

For each region, in logical pixels:

```text
available = floor(unoccluded region span - 2 * content inset)
required  = ceil(shaped inline text advance)
fits      = available >= required
```

- `content inset` is one shared block recipe metric resolved for the current
  size. It is not a public prop or recipe hook.
- `unoccluded region span` subtracts visible handle paint, overlap layering,
  and the focus-ring clearance owned by that region.
- Measurement uses the resolved font, weight, locale-shaped string, and actual
  writing direction.
- Equality fits. No ellipsis, clipping, overflow, or partial inline mode.
- Fit recomputes only when value text, label, size, direction, font metrics, or
  capsule geometry changes. Hover and focus alone do not change it.
- The fallback node sits outside the measured capsule, so its presence cannot
  feed back into the fit calculation.

Fallback placement is fixed:

- one non-interactive line immediately after the capsule in block flow;
- full capsule inline width, aligned to inline-end;
- Slider content: optional label plus the resolved value;
- RangeSlider content: optional label plus the resolved combined range;
- unrotated in vertical layouts after vertical admission;
- hidden from the accessibility tree because slider semantics retain the
  independent accessible name and value-text channels.

The line exists only in fallback mode. It never follows a thumb. A focus/drag
tooltip may be planned later as supplemental paint, but is not required by this
appearance and cannot replace the stable line.

### Keyboard And Direction

Add one explicit shared input to both components:

```ts
direction?: "ltr" | "rtl"; // default: "ltr"
```

- The explicit default preserves current behavior and gives native specs the
  same input as web. Ambient web `dir` is not cross-runtime contract authority.
- Web reflects the field on the component root. Rust specs carry the same enum.
  Direction is component input, not a new ambient `RenderContext` value.
- Horizontal minimum is logical inline-start; maximum is logical inline-end.
  RTL therefore mirrors track geometry and pointer normalization.
- Vertical minimum remains physical bottom; maximum remains physical top.
- Key meaning is numeric in every orientation and direction: Left/Down
  decrement; Right/Up increment; PageDown decrements; PageUp increments.
- Home selects the focused thumb's lowest permitted value. End selects its
  highest permitted value. Dependent RangeSlider bounds still apply.

Page keys keep the current component law for both appearances in the first
block visual card: browser-owned/optional, not strict cross-runtime parity.
`appearance` cannot branch keyboard behavior.

A separate all-appearance Slider/RangeSlider behavior migration is recommended.
If the operator accepts it, let `span = safeMax - min`:

```text
rawPage = span / 10
pageStep = step > 0
  ? max(step, round(rawPage / step) * step)
  : rawPage
```

- `round` is nearest integer; an exact positive half rounds upward.
- The existing normalization and sibling clamp apply after adding or
  subtracting `pageStep`.
- Page amount uses the declared full span, not the current distance to the
  sibling.
- The migration applies identically to `track` and `block`, standard and
  embedded variants, and every active runtime.
- It is outside the first block visual card and cannot be conditioned on
  `appearance`.
- Until operator acceptance and canonical promotion, the formula is a delegate
  recommendation. Current paging remains unchanged.

### Range Ownership And Exact Overlap

- Nearest-thumb comparison uses normalized distance at pointer-down.
- An exact distance tie chooses lower (`lowerDistance <= upperDistance`).
- Last focus, paint order, direction, and pointer history do not change the
  tie.
- The chosen thumb keeps the gesture until terminal cleanup and clamps at its
  sibling. It never swaps identity.
- At equal values, lower and upper remain distinct focus stops and effective
  targets. Pointer-down on the shared exact coordinate chooses lower.
- The focused or active thumb paints above its sibling. If neither is focused
  or active, lower owns the tie paint layer. The other semantic control remains
  keyboard reachable.

### Release, Cancellation, And Lost Capture

Use one idempotent pointer terminal for release, `pointercancel`, lost capture,
stale-pointer cleanup, disablement during a gesture, and adapter teardown:

- an accepted begin opens at most one gesture;
- the first terminal closes it, clears pointer/active-thumb state, and emits
  one `onValueCommit` with the last accepted value or pair;
- cancellation does not roll back;
- a terminal does not emit a new live-change effect;
- later terminals for the same gesture are inert;
- a terminal with no open gesture is inert;
- terminal delivery drains after any already-returned live-change batch, so
  teardown from inside a callback cannot reorder or lose the commit.

Rollback would require a separate transactional value contract and callback.
It is not part of Slider or RangeSlider block appearance.

### Forced Colours

Define shared semantic roles. Map them per renderer:

| Block role | Web forced-colour mapping | Native/high-contrast mapping |
| --- | --- | --- |
| remainder fill / text | `Canvas` / `CanvasText` | canvas / canvas-text |
| selected fill / text | `Highlight` / `HighlightText` | selection / selection-text |
| handle fill / border | `ButtonFace` / `ButtonText` | control / control-border |
| focus indicator | `Highlight` | focus-highlight |
| disabled text / border | `GrayText` | disabled-content |

- Selected, remainder, label, value, handle, border, focus, and disabled roles
  are shared contract meaning. CSS system keywords and native palette lookup
  remain adapter mechanisms.
- Do not use `forced-color-adjust: none` unless every descendant role is
  explicitly remapped and the focused matrix proves the result.
- Do not depend on shadows, transparency, `color-mix`, or author/system colour
  mixing for boundaries in forced colours. Use explicit solid borders where
  regions otherwise merge.
- Text needs 4.5:1 unless it qualifies as large text. Control boundaries and
  focus indicators need 3:1 against adjacent colours.
- Architecture 012 still owns motion. Block value feedback is static in all
  policies; no new motion role is created.

### Effective Target Evidence

Each Slider control and each RangeSlider thumb owns a measurable effective
target of at least 44×44 logical pixels at every size and density.

Required proof:

- Web: resolved interaction-layer rectangles plus coordinate hit tests at the
  centre and four inset corners. A root rectangle or visual thumb pseudo-box is
  insufficient.
- Shared Rust: serialized interaction-node bounds for each stable thumb ID.
- GPUI: mounted pointer dispatch at the same centre/corner coordinates, proving
  the intended control receives the event.
- Range overlap: prove both target rectangles still exist; pointer tie chooses
  lower; direct focus plus keyboard adjusts upper without moving lower.
- Test horizontal LTR, horizontal RTL, and vertical only after native vertical
  admission. Cover `xs` through `xl` and every density.

Targets may geometrically overlap. Overlap does not waive stable focus identity,
tie behavior, or direct keyboard access.

### Native Per-Thumb Prerequisites

Block RangeSlider cannot claim active-cohort admission until every native thumb
node carries:

- stable lower/upper identity and stable lower-then-upper tab order;
- slider role, name, current value, orientation, disabled state, focusability,
  and value text;
- dependent bounds: lower `min=min`, `max=upper`; upper `min=lower`,
  `max=safeMax`;
- separate 44×44 interaction bounds and active/focus paint state;
- the shared direction input for horizontal geometry; and
- real vertical axis geometry and pointer normalization before vertical is
  advertised.

Headless node assertions prove metadata, not assistive-technology projection.
The GPUI 0.2.2 accessibility blocker in
`docs/contracts/003-native-accessibility.md` stays named. No packet or later
card may report native AT parity from metadata alone.

### Invalid And Read-Only Ownership

Invalid and read-only remain wrapper/Field-owned. They do not enter Slider,
RangeSlider, `SliderAppearance`, block VisualState, or the recipe role map.
Indeterminate remains out of scope too.

- A wrapper may render validation copy and compose disabled behavior under its
  own contract.
- It may not stamp undocumented slider states or reinterpret `disabled` as
  read-only.
- Adding any of these states later needs a governing field/control contract,
  callbacks, semantics, and active-cohort evidence.

## Recommendations

- Promote the appearance, content, fit, direction, interaction, forced-colour,
  and evidence proposals together after operator acceptance. Partial promotion
  would leave runtime adapters choosing public behavior.
- Reuse the current normalized Slider/RangeSlider machines. Extend their
  terminal events; do not fork value math for block rendering.
- Keep PageUp/PageDown unchanged in the first block visual card. If the operator
  accepts strict paging, promote and sequence it as a separate all-appearance
  behavior migration.
- Keep fit metrics internal. Expose semantic recipe colours, not consumer
  thresholds or hit-wrapper dimensions.
- Keep the first block card horizontal for active-cohort acceptance. Carry the
  vertical law in the contract, but gate its advertised support on native
  RangeSlider completion.
- Treat GPUI accessibility metadata as prerequisite evidence and the current
  backend limitation as an explicit blocker, not an accepted parity delta.

## Alternatives Not Selected

| Alternative | Reason |
| --- | --- |
| Put `block` in `variant` | Mixes visual appearance with native/embedded interaction substrate. |
| Separate Slider and RangeSlider appearance fields | Creates needless public divergence for one shared visual contract. |
| Reuse ARIA label/value text as visible copy | Breaks the settled separation and changes current consumers visibly. |
| Consumer-supplied resolved visible strings only on web | Makes controlled drag formatting host churn and diverges from existing formatter conventions. |
| Clip, ellipsize, overflow, or fit items independently | Can hide exact values or make the layout jump between thumbs. |
| Always external readout | Valid but discards the approved inline direction when space exists. |
| Thumb-following fallback tooltip | Unstable under overlap and insufficient for keyboard, touch, and magnification users. |
| Last-focused overlap tie | Adds hidden history to pointer ownership and diverges from the current core law. |
| Block-only strict PageUp/PageDown | Makes a presentation field change keyboard semantics and creates two behavioral contracts. |
| Defer PageUp/PageDown without a migration proposal | Preserves current runtime drift; valid if the operator rejects the separate all-appearance migration. |
| Physical-direction arrow keys in RTL | Makes key meaning depend on layout direction and conflicts with current numeric mapping. |
| Roll back on cancel | Requires transaction state and a new callback contract. |
| Web-only forced-colour CSS | Leaves shared native roles undefined and weakens semantic parity. |
| Add invalid/read-only props now | Expands a visual promotion into an unplanned field-state contract. |

## Explicit Non-Goals

- Implementing code, tokens, generated artifacts, specimens, or tests.
- Promoting contracts, architecture, specs, roadmaps, or a ready card.
- Replacing current Slider or RangeSlider defaults.
- Migrating any preview, internal consumer, or downstream product.
- Marks, ticks, editable hot text, whole-window dragging, thumb swapping,
  minimum-distance policy, or more than two thumbs.
- Tooltip-only output or a required tooltip in the first appearance.
- Changing PageUp/PageDown behavior in the first block visual card.
- Vertical active-cohort admission before native RangeSlider geometry exists.
- Jetstream implementation, evidence, or parity claims.
- Claiming native assistive-technology support from node metadata.
- A new motion role or performance claim.

## Required Review Oracles

| Invariant | Smallest adversarial counterexample | Expected failure or stop | Required proof |
| --- | --- | --- | --- |
| Omitted appearance preserves today | One existing Slider with no new props changes DOM/node anatomy or callback trace | Stop promotion | Before/after focused snapshots plus existing trace parity for both variants and components |
| One shared appearance field | RangeSlider exposes a different field or value name | Contract drift failure | Web prop/type audit and Rust spec enum audit |
| Visual and accessible channels stay separate | `ariaLabel="Gain"` appears visibly with no `visibleLabel` | Focused test failure | Svelte, React, Rust node assertions |
| Formatter inputs are accepted values | Step `10`, raw pointer value `67`, formatter receives `67` | Core/adapter test failure | Formatter spy receives snapped value only |
| Fit is all-or-nothing | Slider label fits but value misses by one logical pixel | Inline text must be absent; fallback line present | Boundary tests at `required-1`, `required`, `required+1` |
| Fallback placement is stable | Range lower and upper overlap at 50 and readout follows the focused thumb | Visual/layout failure | Geometry assertion and representative captures |
| RTL geometry and numeric keys agree | RTL Slider at 40 receives ArrowRight | Value becomes 41 and thumb moves toward logical inline-end | Paired web/core/native traces plus geometry assertion |
| Appearance cannot change paging | Same Slider props, only `track` changes to `block`, then PageUp | Both appearances produce the same current runtime result | Paired appearance trace before first visual-card acceptance |
| Separate paging migration formula is exact | `min=0`, `max=100`, `step=6` | PageUp adds 12 in both appearances, not browser 10 or step×10 | Shared machine trace in all active runtimes after separate operator acceptance |
| Exact pointer tie chooses lower | `[50,50]`, press exactly at 50 after upper was focused | Lower owns gesture; focus history is ignored | Core trace and mounted web/GPUI pointer proof |
| Gesture ownership never swaps | Lower chosen at 40, dragged beyond upper at 60 | Lower clamps at 60; upper unchanged | Shared core and adapter trace |
| Terminal is idempotent | `pointercancel` followed by `lostpointercapture` | One commit total, no rollback, no second change | Ordered effect trace in Svelte, React, Rust, GPUI |
| Teardown preserves effect order | Host removes control inside live-change callback | Live change drains before one commit | Framework-free trace plus mounted adapter case |
| Each effective target is 44×44 | `xs` RangeSlider at equal values | Both target bounds remain at least 44×44 | Web geometry/hit probe, Rust bounds assertion, GPUI mounted probe |
| Forced colours retain roles | Selected and remainder both resolve to `Canvas` | Stop visual acceptance | Forced-colour role dump plus focused rendered contrast check |
| Native dependent bounds are exact | Range `[20,80]` | Lower max is 80; upper min is 20 | Per-thumb node metadata assertion |
| Native AX is not overclaimed | Node metadata tests pass under GPUI 0.2.2 | Evidence must remain manual/blocked | Ledger wording check against contract 003 |
| Invalid/read-only stay wrapper-owned | Block props or VisualState gain `readOnly` | Scope/drift failure | Public surface audit and contract diff review |
| Vertical stays gated | GPUI specimen renders a vertical label over horizontal scrub geometry | Stop admission | Native axis, pointer, semantics, and mounted evidence before support claim |
| Jetstream stays deferred | A block Jetstream specimen or passing parity cell appears | Scope failure | Changed-file audit and parity-ledger review |

## Proposed Canonical Destinations

| Meaning | Destination after packet acceptance |
| --- | --- |
| Shared appearance, visible inputs, fit/readout, direction, keyboard, target, and state laws | `docs/contracts/components/slider.md`, `docs/contracts/components/range-slider.md` |
| Stable semantic recipe roles and internal-metric boundary | `docs/architecture/007-appearance-recipe-contract.md` plus the two component contracts |
| Shared terminal law reference | Reuse `docs/architecture/008-audio-control-family.md`; project it into both component contracts |
| Motion statement | Reference `docs/architecture/012-semantic-motion-policy.md`; no architecture change expected |
| Effective-target minimum | Reference `docs/contracts/components/size-and-density.md`; add component-specific proof rows only |
| GPUI evidence boundary | `docs/contracts/003-native-accessibility.md` and later parity evidence ledger updates |
| First visual implementation sequencing, scope, stops, and validation | One additive Slider/RangeSlider block card after operator acceptance and canonical promotion; no paging change |
| Optional strict PageUp/PageDown behavior | A separate all-appearance Slider/RangeSlider behavior-migration card after operator acceptance and canonical promotion |

The orchestrator chooses the final promotion split. No implementation card is
ready from this packet alone.

## Unresolved Questions

Operator acceptance or revision of the exact delegate proposal remains open.
That gate includes the public names, formatter fields, fit law, direction,
overlap tie, terminal law, forced-colour map, effective-target evidence, native
prerequisites, and wrapper-owned state boundary.

The operator must also accept the separate all-appearance PageUp/PageDown
migration or keep current paging unchanged. The first block visual card never
changes paging. Merge is intake for these decisions, not acceptance. Promotion
may also expose integration drift against newer `main`; the orchestrator owns
that reconciliation after the operator gate.

## Evidence Used

- `docs/triage/20260901-125758-post-motion-research-queue.md`
- `docs/research/value-tracks/block-slider-visual-direction.md`
- `docs/architecture/006-headless-core-and-machine-model.md`
- `docs/architecture/007-appearance-recipe-contract.md`
- `docs/architecture/008-audio-control-family.md`
- `docs/architecture/010-native-presentation-construction-context.md`
- `docs/architecture/012-semantic-motion-policy.md`
- `docs/contracts/001-working-rules.md`
- `docs/contracts/003-native-accessibility.md`
- `docs/contracts/components/slider.md`
- `docs/contracts/components/range-slider.md`
- `docs/contracts/components/size-and-density.md`
- current Svelte, React, shared core, Rust renderer, and GPUI Slider/
  RangeSlider source and focused evidence described by the dossier
