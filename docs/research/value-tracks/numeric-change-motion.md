# Value Track: Numeric Change Motion

Status: complete — research only; promotion deferred
Updated: 2026-09-01
Origin: [g16.040 — Numeric Change Motion Research](../../roadmaps/g16/040-numeric-change-motion-research.md)
Scope: display-only numeric updates; editing controls are excluded

## Decision

Numeric change motion is **not admitted as a Poodle semantic motion role** in
g16.040.

The recommendation is **recipe-only guidance with static behavior as the
Poodle default**:

- Keep the current consumers' latest formatted value immediately visible.
- Permit a product-owned, optional full-mode recipe only when a named consumer
  has a meaningful, bounded update cadence and the host wants continuity or
  attention on that update.
- Keep reduced and frozen behavior static: the latest formatted value replaces
  the old value immediately, with no digit staging, layout travel, blur, or
  live clock.
- Keep formatting with the existing Poodle consumer/core owner. A motion
  recipe must not introduce a generic number formatter or infer trend meaning
  from a numeric delta.
- Do not add a `numeric-update` role, component, dependency, formatter API, or
  consumer change from this research card.

The evidence supports a future narrow role only if an operator approves a
semantic consumer and an active-cohort fallback. Neither gate is present in
the current card. `g16.034` and the shared [semantic motion policy](../../architecture/012-semantic-motion-policy.md)
are accepted prerequisites; they do not establish a numeric consumer by
themselves.

If a consumer is later approved, `MetricTile` is the strongest candidate for a
first pilot because a dashboard snapshot can change at a bounded cadence and
already has an explicit trend/sparkline context. `ValueReadout` needs proof
that its updates are discrete rather than continuous control feedback.
`ListCardCounter` is a poor candidate: it is compact secondary metadata, can be
linked, and has no update or announcement semantics.

## Evidence labels and method

- **[VF] Verified finding** — read from the pinned source or the current Poodle
  repository.
- **[SC] Source claim** — stated by an upstream project or its public docs.
- **[WI] Worker inference** — a conclusion from the evidence and Poodle's
  current contracts.

This dossier reuses the numeric rows in the [Transitions.dev catalogue
audit](transitions-dev-catalogue.md); it does not repeat that catalogue.
NumberFlow and Calligraph were inspected from durable upstream snapshots on
2026-09-01:

| Source | Snapshot | Primary evidence | License boundary |
| --- | --- | --- | --- |
| [NumberFlow](https://number-flow.barvian.me/) | `0.6.2`, commit [`a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a`](https://github.com/barvian/number-flow/tree/a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a) | [`lite.ts`](https://github.com/barvian/number-flow/blob/a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a/packages/number-flow/src/lite.ts), [`formatter.ts`](https://github.com/barvian/number-flow/blob/a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a/packages/number-flow/src/formatter.ts), [`ssr.ts`](https://github.com/barvian/number-flow/blob/a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a/packages/number-flow/src/ssr.ts), [React wrapper](https://github.com/barvian/number-flow/blob/a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a/packages/react/src/NumberFlow.tsx) | [MIT](https://github.com/barvian/number-flow/blob/a7b78f5d4d4b1e2cd4cb53d09d7158461bad493a/LICENSE.md) |
| [Calligraph](https://calligraph.raphaelsalaja.com/) | package `1.4.1`, untagged `main` commit [`c99497a440e280e8471f084db1173197ba0e255c`](https://github.com/raphaelsalaja/calligraph/tree/c99497a440e280e8471f084db1173197ba0e255c) | [`index.tsx`](https://github.com/raphaelsalaja/calligraph/blob/c99497a440e280e8471f084db1173197ba0e255c/packages/calligraph/src/index.tsx), [`number.tsx`](https://github.com/raphaelsalaja/calligraph/blob/c99497a440e280e8471f084db1173197ba0e255c/packages/calligraph/src/number.tsx), [`reconcile.ts`](https://github.com/raphaelsalaja/calligraph/blob/c99497a440e280e8471f084db1173197ba0e255c/packages/calligraph/src/reconcile.ts), [`package.json`](https://github.com/raphaelsalaja/calligraph/blob/c99497a440e280e8471f084db1173197ba0e255c/packages/calligraph/package.json) | [MIT](https://github.com/raphaelsalaja/calligraph/blob/c99497a440e280e8471f084db1173197ba0e255c/packages/calligraph/LICENSE) |
| Transitions.dev | pinned audit commit [`0f8540f9a7c200211d4bcc149d787382fa4567fa`](https://github.com/Jakubantalik/transitions.dev/tree/0f8540f9a7c200211d4bcc149d787382fa4567fa) | [existing Poodle audit](transitions-dev-catalogue.md) | Recipes are analysis-only under the source terms; no source is copied. |

The upstream MIT licenses permit reuse subject to their notices. This card
does not copy upstream source or recommend a dependency. That is a separate
admission decision and is unnecessary for the current static/recipe-only
disposition. [VF]

## Poodle boundary

Poodle's architecture requires semantic inputs, state, behavior,
accessibility, and token usage to align across Svelte, React, shared Rust
composition, and GPUI; runtime mechanisms may differ only inside that
contract. The web pair shares core behavior, and native composition flows
through `poodle-render` into renderer-neutral `poodle-node` output. See
[Poodle System Shape](../../architecture/001-poodle-system-shape.md) and
[the working rules](../../contracts/001-working-rules.md). [VF]

Architecture 012 establishes the relevant laws:

- `full`, `reduced`, and `frozen` are host-resolved effective policies;
  descendants can only become more restrictive.
- Semantic and accessibility state updates immediately. Motion does not own
  labels, announcements, correctness, focus, or timers.
- Reduced mode removes translation, scale, rotation, blur, bounce, and layout
  travel. A short opacity effect is allowed only for an explicitly admitted
  role.
- Frozen mode schedules no clock and paints the latest settled endpoint.
- Motion identity is semantic owner + role + channel. Updates are
  latest-state-wins; repeated targets are inert; visual completion never fires
  semantic callbacks.
- The default renderer-neutral property budget is opacity, translation, scale,
  and rotation. Layout, blur, filters, masks, and other mechanisms need a
  role-specific decision and a static/reduced fallback.

The current core has no numeric motion role in `MOTION_ROLE`, and the current
native node animation vocabulary has generic opacity, rotation, translation,
and scale channels but no numeric-part or text-segmentation model. See
[`packages/core/src/motion-policy.ts`](../../../packages/core/src/motion-policy.ts)
and [`packages/contracts/node/src/lib.rs`](../../../packages/contracts/node/src/lib.rs).
[VF]

This makes the distinction important:

| Layer | Current decision |
| --- | --- |
| Semantics | The formatted value is the state. It must update immediately. |
| Poodle role | No numeric role is admitted by this card. |
| Optional recipe | Product-owned guidance may use a short full-mode value-change effect after a future consumer decision. |
| Reduced/frozen | Latest formatted text immediately; no per-digit visual state. |
| Native fallback | Static endpoint is the only already-valid cross-cohort fallback. A GPUI opacity approximation would need a separate role-specific admission. |

## Existing consumers

### Contract matrix

| Consumer | Current input and owner | Existing semantics | Numeric-motion fit | Disposition |
| --- | --- | --- | --- | --- |
| `ValueReadout` | Numeric `value`, audio range/law, and `AudioValueFormat`. Core owns `formatAudioValue` and `valueReadoutVisualState`. | Read-only `<output>`; only enabled/disabled states; no events; the visual child is `aria-hidden` so the formatted value is announced once. | Strongest formatting boundary, but audio control feedback can be high-frequency. Per-digit motion risks hiding the latest control value and has no semantic delta contract. | Keep static. Reconsider only with evidence of discrete, meaningful updates. |
| `MetricTile` | `value: string` is already formatted by the caller; optional caller-provided `trend`, `trendLabel`, and sparkline data. | Non-interactive display composite; no internal state; no live region; root accessible name is `label: value`; trend and sparkline are separate context. | Best future candidate for sparse dashboard snapshots. Current string API cannot safely infer numeric segments, locale, unit, or direction. | Keep static. Candidate for a future consumer-specific pilot after operator approval. |
| `ListCardCounter` | `count: number` is rendered directly beside an icon; optional link/tooltip. | Compact secondary metadata; static `<span>` or native `<a>`; no live behavior; linked counters preserve keyboard/link semantics. | Low value. A changing count is not necessarily an event users need emphasized, and width changes can disturb a dense list footer. | Keep static; do not pilot. |

The source and contract evidence is direct:

- [`ValueReadout` contract](../../contracts/components/value-readout.md)
  makes formatting, serializable visual output, and the one accessible
  announcement core-owned. The Svelte and React shells call the same formatter
  and pass formatted text to an `aria-hidden` visual child: [`ValueReadout.svelte`](../../../packages/svelte/components/src/ValueReadout.svelte), [`ValueReadout.tsx`](../../../packages/react/components/src/ValueReadout.tsx), and [`ValueVisual`](../../../packages/svelte/components/src/audio/ValueVisual.svelte). [VF]
- [`MetricTile` contract](../../contracts/components/metric-tile.md) defines a
  styled-only composite with externally driven strings, explicit trend state,
  no live region, and no internal state. Both shells put the supplied string
  directly in the value element: [`MetricTile.svelte`](../../../packages/svelte/components/src/MetricTile.svelte) and [`MetricTile.tsx`](../../../packages/react/components/src/MetricTile.tsx). [VF]
- [`ListCardCounter` contract](../../contracts/components/list-card-counter.md)
  defines a compact static/linkable count and explicitly requires tabular
  numerals. The Svelte and React shells render `count` directly: [`ListCardCounter.svelte`](../../../packages/svelte/components/src/ListCardCounter.svelte) and [`ListCardCounter.tsx`](../../../packages/react/components/src/ListCardCounter.tsx). [VF]

### Representative consumers

The inspected Svelte, React, and GPUI specimens are catalog examples, not
stateful update demonstrations:

- `MetricTileSpecimen` uses values such as `85`, `94%`, `2,847`, `$14.2k`,
  `1,204`, and `4.2 GB`, with trend/sparkline variants but no update loop:
  [`Svelte`](../../../packages/svelte/preview/src/specimens/MetricTileSpecimen.svelte),
  [`React`](../../../packages/react/preview/src/gallery/specimens/MetricTileSpecimen.tsx),
  [`GPUI`](../../../packages/gpui/preview/src/specimens/metric_tile_specimen.rs).
- `ListCardSpecimen` uses static counts such as `18`, `6`, `24`, `8`, `3`, and
  `42` in list-card footers:
  [`Svelte`](../../../packages/svelte/preview/src/specimens/ListCardSpecimen.svelte),
  [`React`](../../../packages/react/preview/src/gallery/specimens/ListCardSpecimen.tsx),
  [`GPUI`](../../../packages/gpui/preview/src/specimens/list_card.rs).
- `ValueReadoutSpecimen` exercises number, dB, Hz/kHz, percent, ratio,
  milliseconds, note-name, semitone, negative, boundary, disabled, size, and
  density cases, but does not demonstrate a changing value:
  [`Svelte`](../../../packages/svelte/preview/src/specimens/ValueReadoutSpecimen.svelte),
  [`React`](../../../packages/react/preview/src/gallery/specimens/ValueReadoutSpecimen.tsx),
  [`native audio specimens`](../../../packages/render/src/audio_specimens.rs).

No inspected representative surface establishes the missing semantic facts:
what update is important, how often it happens, whether the user should be
alerted, and who owns the update's accessible announcement. [VF][WI]

## NumberFlow findings

### What it proves

NumberFlow is the stronger reference for a segmented numeric renderer. Its
formatter calls `Intl.NumberFormat.formatToParts`, preserves integer/fraction
and symbol sections, accepts `prefix`/`suffix`, and keys integer parts from the
right for layout continuity. Its custom element maintains separate numeric and
symbol sections, per-digit strips, and inert old parts. [VF]

Its public docs also expose `locales`, `format`, per-channel timings, `trend`,
`digits`, `isolate`, `animated`, `respectMotionPreference`, and grouped update
lifecycle. These are source claims confirmed in the pinned React wrapper and
lite implementation where applicable. [SC][VF]

This gives useful implementation evidence for a future recipe:

- Keep the formatted result as the source of truth and distinguish digits from
  signs, grouping, decimal marks, prefixes, and suffixes.
- Keep identity by semantic part rather than by tree position. Numeric columns
  can be keyed from the least-significant side, while symbols are retained as
  separate parts.
- Measure before and after the update when horizontal continuity is explicitly
  accepted; do not make width measurement a hidden default.
- Treat a repeated target as inert and suppress stale visual-finish reporting
  when a newer value arrives.
- Set the semantic label immediately and keep old visual parts inert.

### Where it does not fit Poodle

- The renderer is browser-specific: custom elements, shadow DOM, CSS registered
  properties, `CSS.supports`, `ElementInternals`, masks, `offsetWidth`, and
  `getBoundingClientRect` are all part of the implementation. [VF]
- Its `Intl` input is not the Poodle `AudioValueFormat` vocabulary. Poodle
  supports dB, Hz/kHz, explicit kHz, percent, ratio, milliseconds/seconds, MIDI
  notes, semitones, and other audio-specific parsing/display rules. Moving that
  ownership would break the current contract. [VF]
- The official docs exclude scientific/engineering notation and currently call
  out non-Latin digits and RTL limitations. The pinned source also parses digit
  characters into numbers, so it cannot be treated as a general locale-neutral
  segment model. [SC][VF]
- Its width continuity uses horizontal translation and scale/mask techniques.
  Width itself is not in Poodle's default motion property budget, and masks are
  not a neutral native primitive. [VF][WI]
- `respectMotionPreference` defaults to true but can be disabled. Poodle's
  restriction-only policy must not expose a child opt-out that re-enables motion
  under a reduced host policy. [VF][WI]
- The lite implementation samples reduced preference while processing an
  update and gates animation on visibility and document visibility. The React
  package has a reactive preference hook. This is useful web behavior, but it
  is not Poodle's host-resolved `MotionPolicy` and does not define frozen/capture
  behavior. [VF]
- `animated=false` finishes in-flight animations and suppresses future visual
  work; that is not by itself the Poodle frozen law of settling the latest
  semantic endpoint with no clock. [VF][WI]
- The source uses explicit pre/post measurement and accumulated browser
  animations to handle interruption. It is evidence that rapid retargeting is
  solvable, not a portable contract for Svelte, shared Rust, and GPUI. [VF][WI]

### Accessibility, forced colors, and capture

NumberFlow's SSR output exposes one `role="img"` with an `aria-label`, and its
browser path uses `ElementInternals` with the same role/label where supported;
old visual parts receive `inert`. This is a credible single-value accessibility
pattern. It is not a reason to replace Poodle's existing `output`, metric-root,
or link semantics with `role="img"`. [VF]

The pinned styles rely on masks, registered custom properties, and blend/filter
effects. No forced-colors rule or capture/frozen mode appears in the inspected
source. A future Poodle recipe would need a plain-text endpoint test under
forced colors and explicit frozen capture, rather than inheriting those
mechanisms. [VF][WI]

## Calligraph findings

### What it proves

Calligraph is a compact React/Motion recipe library with `text`, `number`, and
`slots` variants. It reconciles grapheme sequences, preserves common character
keys, and uses Motion presence transitions for entering and exiting characters.
The number variant applies vertical movement, opacity, blur, and scale; the
slots variant builds 0–9 columns and spins them by a direction derived from the
formatted string. [VF]

The number implementation makes the visual recipe concrete:

- `String(children ?? "")` is the input; the package does not own `Intl` or
  locale formatting.
- Prefix characters are identified only before the first ASCII digit.
- Direction is inferred by stripping non-`0-9.-` characters and parsing the
  result. This is a visual convenience, not a safe semantic trend model.
- `AnimatePresence` and `layout="position"` preserve/reflow character slots.
- `autoSize` defaults to true and animates wrapper width through
  `ResizeObserver` and Motion.

This is useful recipe evidence for preformatted text and common-character
continuity. It also confirms why a named Poodle role cannot simply be “use a
number animation package.” [VF]

### Where it does not fit Poodle

- It is React-only and has a peer dependency on Motion. There is no Svelte,
  shared Rust, or GPUI implementation path in the pinned package. [VF]
- The pinned source exposes no Calligraph-level `MotionPolicy`,
  `respectMotionPreference`, reduced-motion branch, frozen/capture mode, or
  forced-colors rule. Motion may provide host features independently, but this
  component does not wire the Poodle policy contract. [VF][WI]
- The changing character nodes are `aria-hidden`. The package does not add a
  single equivalent accessible value or live-region strategy; a wrapper must
  supply and own the semantic label. That is incomplete for a reusable Poodle
  composite without additional contract work. [VF]
- `onComplete` is a visual animation callback. It cannot become the source of
  a value announcement, state commit, or other semantic effect. [VF][WI]
- Grapheme and ASCII parsing do not provide the locale/format-part model needed
  for Poodle's signed, grouped, unit-bearing, and audio-specific values. [VF]
- Default auto-sizing and `layout="position"` make layout travel part of the
  recipe. Poodle requires a separate layout-motion decision and a static/reduced
  fallback. [VF][WI]
- `AnimatePresence` handles React presence, but the public component does not
  state Poodle's stable semantic owner/channel identity, latest-state-wins
  retargeting, abort cleanup, or policy tightening behavior. [VF][WI]

Calligraph is therefore a recipe reference, not a Poodle dependency or role
authority. Its MIT license is noted in the source table; no source is copied.

## Transitions.dev evidence reused

The existing [Transitions.dev audit](transitions-dev-catalogue.md) already
covers the relevant mechanisms and failure modes:

- **Number pop-in** uses per-digit spans, blur/slide/opacity, stagger, DOM
  replacement, and forced reflow. Reduced mode leaves final text. Rebuild and
  replay must cancel stale targets and sequencing.
- **Spinning counter** uses JS-built digit strips, measured cell height,
  transforms, a long reel, and optional blur. Reduced mode stops the motion and
  paints the final value; many nodes and filters make it a poor default.
- The catalogue matrix marks both as possible future matches for a value-change
  primitive or `MetricTile`, but says per-digit/reflow choreography is too
  specific and that native text/value animation is absent.

The audit's existing recommendation remains the governing one: promote
semantic intent and lifecycle, not named recipes. Numeric evidence narrows the
future candidate; it does not reopen architecture 012. [VF]

## Numeric design questions

### Does the motion carry semantic value?

Sometimes, but not by itself.

It can preserve continuity when all of these are true:

- the user already understands the stable metric owner;
- the update is a meaningful snapshot or discrete event, not every control tick;
- the host wants the new value noticed without changing its semantic meaning;
- the formatted endpoint remains immediately available to assistive technology;
- trend, severity, validation, and notification meaning come from explicit
  consumer state, not from animation direction.

Otherwise the motion is decorative emphasis. A rolling digit, a blur, or a
vertical direction does not tell the user whether a change is good, bad,
validated, urgent, or actionable. In particular, NumberFlow's and
Calligraph's inferred direction must not replace `MetricTile`'s explicit
`trend`/`trendLabel`. [SC][VF][WI]

### How should formatting survive an update?

The formatter must commit the latest formatted value first. A future recipe
may segment that already-owned value, but it must not parse arbitrary text into
a new semantic number model.

The atomic display identity includes:

- sign and sign placement;
- prefix/suffix and units;
- grouping and decimal marks;
- integer and fraction digits;
- the selected format and locale.

If segmentation is later admitted, stable keys should follow semantic parts and
least-significant numeric positions, not DOM index. A format or locale change
that reorders symbols, changes digit systems, changes grouping, or changes
units should rebuild the visual representation from the latest endpoint. It
must not leave an old prefix, sign, separator, or fractional precision on
screen while a new value is being animated. [VF][WI]

The current consumer split matters:

- `ValueReadout` already has a core formatter and a serializable formatted
  text, so it is the only named consumer with a natural formatting owner.
- `MetricTile` intentionally accepts an already-formatted string. A generic
  motion layer cannot assume that `$14.2k`, `4.2 GB`, `1.8s`, and `94%` share a
  numeric parse or locale policy.
- `ListCardCounter` owns only a raw count display today. Localized formatting
  would be a contract change before it could be a motion input.

### What about width and tabular numerals?

Tabular numerals stabilize repeated columns but do not stabilize sign,
grouping, unit, locale, or format changes. `ValueReadout` has a minimum width
and `tabular-nums`; `ListCardCounter` explicitly requires `tabular-nums`.
`MetricTile` accepts arbitrary strings and its current value style does not
establish a numeric feature contract. [VF]

NumberFlow's measured width/scale and Calligraph's default auto-size are
useful web techniques, but neither should become an invisible Poodle layout
effect. Width animation moves surrounding geometry or compresses text and is
outside the default property budget. A future role would need an explicit
width policy:

- same-slot values may retain their allocated box;
- changed width may snap to the endpoint;
- a reserved width or bounded layout exception needs consumer evidence;
- reduced and frozen always settle width immediately.

### What should trend mean?

Trend remains an explicit consumer input. `MetricTile` already separates
`trend`, `trendLabel`, and the display `value`; that separation is correct.
Numeric delta may choose a visual direction inside a future recipe, but it must
not infer status color, announcement, progress, validation, or business
meaning. A caller-provided `flat` trend must not be overridden by a parsed
numeric difference. [VF][WI]

### What happens on rapid retargeting?

Any future admitted recipe must follow architecture 012:

1. Update the semantic value, formatted text, accessible name, and derived
   consumer state immediately.
2. Keep one stable key based on semantic owner + role + channel.
3. Retarget to the latest value; do not queue stale intermediate values.
4. Reverse from sampled rendered progress only when the role explicitly admits
   reversal; otherwise settle to the latest endpoint.
5. Treat repeated current targets as inert.
6. Cancel or settle all visual clocks on abort, unmount, policy tightening, or
   frozen capture. Remove old visual remnants without removing the semantic
   owner.
7. Never use visual completion to fire the value update or an announcement.

NumberFlow supplies useful evidence for measurement, accumulated animation,
and stale completion suppression. Calligraph supplies presence reconciliation
but no equivalent Poodle lifecycle contract. Neither is sufficient authority
for a cross-runtime role. [VF][WI]

### What is the reduced/frozen behavior?

The future recipe guidance is intentionally strict:

| Effective policy | Numeric behavior |
| --- | --- |
| `full` | Optional short, one-shot emphasis only after role admission. No loop, long reel, mandatory stagger, blur, or implicit width animation. The latest formatted value and accessible semantics are already committed. |
| `reduced` | Paint the latest formatted value immediately. Do not use per-digit translation, scale, rotation, blur, bounce, or layout travel. Opacity is not assumed to be allowed because numeric change is not an admitted role. |
| `frozen` | No clock, staged presence, or pending visual target. Paint the latest formatted endpoint immediately for deterministic capture and tests. |

Initial render always paints its endpoint. A host changing from full to
reduced/frozen must not leave an old digit reel or stale symbol visible. [VF]

### Can one accessible value survive per-digit segmentation?

Yes in principle, but it must be explicit:

- Keep one semantic value/label owned by the current consumer.
- Mark all visual digit/symbol clones `aria-hidden` or `inert` as appropriate.
- Do not create a live region or announce each digit.
- Preserve the existing element semantics: `output` for `ValueReadout`, the
  metric root's accessible name for `MetricTile`, and `span`/`a` plus tooltip
  semantics for `ListCardCounter`.
- Update labels and visible text before any optional visual effect.

NumberFlow's single labelled `role="img"` is evidence that a separate visual
tree can work. Calligraph's hidden character nodes without a generated
accessible value show the failure mode. Neither changes the Poodle contracts.
[VF][WI]

### What about locale, forced colors, and capture?

- **Locale and format:** preserve Poodle's current formatter ownership. A
  future segmented model must test negative values, signs, prefixes/suffixes,
  grouping, decimals, units, locale changes, and unsupported digit systems.
  NumberFlow's documented RTL/non-Latin limitations and Calligraph's string
  parser are reasons to fall back to the latest static text, not reasons to
  narrow Poodle's contract silently. [SC][VF]
- **Forced colors/high contrast:** neither pinned upstream source declares a
  forced-colors treatment. The static endpoint must remain readable without
  masks, blend modes, blur, or opacity-only contrast. Add a forced-colors check
  before any role admission. [VF][WI]
- **Capture:** visual capture must select `frozen` and compare the latest
  endpoint. An animated screenshot is not reduced-motion evidence and must not
  be used as cross-runtime parity proof. [VF]

### What is explicitly out of scope?

`NumberInput`, `DragNumberField`, keyboard entry, pointer dragging, caret
movement, commit/cancel behavior, validation, and edit previews are excluded.
Numeric change motion must not be inferred as an editing affordance. The
existing editing contracts remain authoritative. [VF]

## Active-cohort feasibility

| Runtime/layer | Feasible evidence today | Blocking issue for a Poodle role |
| --- | --- | --- |
| Shared web core | Core already owns formatted audio text, visual state, motion policy helpers, and latest-state lifecycle machinery. | No numeric part model or admitted numeric role. Adding one would be an API/contract change outside g16.040. |
| Svelte | A browser recipe could split preformatted text or wrap a web component. Current Svelte shells are thin over shared behavior. | A Svelte-only DOM choreography would break the web-pair contract. Calligraph has no Svelte path; NumberFlow would add a browser/custom-element dependency. |
| React | NumberFlow has a React wrapper; Calligraph is React-native. A React specimen could prove a browser effect. | A React-only implementation is not Poodle parity. It would also need to preserve `ValueReadout`/`MetricTile` semantics and core formatting. |
| Shared Rust / `poodle-render` | Rust can emit the current formatted text and carries the effective `MotionPolicy` through `RenderContext`. | `poodle-node` has text nodes and generic animation channels, not keyed numeric segments, text measurement, or glyph layout. A new portable value model is out of scope. |
| GPUI | Architecture 012 permits named static or opacity approximations where the backend cannot realize a property. | GPUI 0.2.2 cannot express per-digit text motion or width/layout continuity through the current node vocabulary. Its current accessibility limitation also forbids claiming equivalent AT behavior. The safe fallback is static endpoint. |
| Jetstream | Deferred by the working rules. | It is not an active-cohort admission target for this card and must not be used to hide a Poodle gap. |
| Capture/test | Existing policy and harness can select frozen/static output. | Capture can verify endpoints only; it cannot prove reduced behavior, retargeting, cleanup, or accessibility. |

The common semantic fallback is therefore clear: every active renderer must be
able to paint the latest formatted value immediately, with no stale visual
remnant. Exact per-digit visuals are optional runtime mechanics only after a
future role defines their value and acceptable approximation. [VF][WI]

## Promotion gates and unresolved decisions

### Gates

| Gate | State |
| --- | --- |
| Accepted `g16.034` motion policy | Met; architecture 012 is the shared policy basis. |
| Operator-approved semantic consumer | Not met. No current specimen establishes a meaningful update cadence or notice requirement. |
| Explicit active-cohort fallback | Defined here as latest formatted static endpoint; not yet accepted as a role-specific promotion decision. |
| Formatter and locale ownership | Current ownership is clear; no generic numeric formatter is approved. |
| Accessible single-value behavior | Current consumer contracts are clear; no segmented implementation has been admitted. |
| Performance/interruption evidence | Upstream provides useful web techniques, but no Poodle cross-runtime proof exists. |

Before any future promotion, the operator must answer:

- Which product surface changes, and at what cadence?
- Why is the change worth noticing when trend, sparkline, or ordinary text
  already communicates it?
- Is the first pilot `MetricTile`, a discrete `ValueReadout` surface, or
  another consumer with a revised contract?
- Does full-mode motion use only a short bounded effect, and what is the
  budget for layout measurement and extra DOM/native nodes?
- Is static latest-endpoint behavior accepted as the GPUI and reduced/frozen
  fallback?
- How are locale/format changes, rapid retargets, unmount, visibility changes,
  forced colors, SSR/hydration, and capture handled?
- What evidence proves one accessible value rather than repeated digit
  announcements?

### Required future test shape

If a role is later admitted, its contract and tests must cover the named
consumer rather than a generic demo:

- initial endpoint with no animation;
- one update, repeated target, rapid retarget, reversal, abort, unmount, and
  owner replacement;
- immediate formatted text and accessible name at every visual phase;
- reduced and frozen policies with no disallowed property or live clock;
- sign, prefix/suffix, grouping, decimal precision, units, negative/boundary
  values, locale/format changes, and width changes;
- tabular numeral behavior where the consumer contract requires it;
- forced-colors/high-contrast readability;
- Svelte/React pair evidence and shared Rust/GPUI static or named
  approximation evidence;
- deterministic capture of the frozen endpoint only.

NumberInput and DragNumberField must remain absent from this matrix.

## Final disposition

Close g16.040 as a completed research dossier with no promotion. The durable
Poodle position is:

> Numeric change can be a useful product-owned continuity cue for a named,
> bounded display update. It is not a general semantic role today. Poodle
> consumers remain static, formatting remains consumer/core-owned, and reduced
> or frozen output is the latest formatted value immediately.

The orchestrator owns any later roadmap/card/front-door/triage or
`PAPERCUTS.md` integration. This worker changes only this dossier.
