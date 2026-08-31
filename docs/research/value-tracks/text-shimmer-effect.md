# Poodle-owned GPU-conscious text shimmer

Status: research dossier
Origin: docs/triage/20260831-155153-text-shimmer-effect.md
Primary lead: [Devon Govett's “GPU accelerated text shimmer” Pen](https://codepen.io/editor/devongovett/pen/01a0439c-4b7f-7f44-bf84-205c514ad139)
Research checked: 2026-08-31
Scope: evidence and promotion scope only; no component or API implementation

## Executive summary

The reference Pen is a small, dependency-free web experiment. It keeps the real
text visible and places an inert, transparent duplicate above it. A pseudo-element
draws a gradient highlight in that duplicate's text mask and moves the highlight
with an individual translate. Its reduced-motion branch removes the duplicate,
leaving the ordinary text. The actual source is available as
[HTML](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/index.html)
and
[CSS](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/style.css);
the Pen's JavaScript file is empty.

The performance idea is credible but the phrase “GPU accelerated” is too broad
for a Poodle contract. [Browser animation guidance](https://web.dev/articles/animations-guide)
supports keeping animation to transform/opacity, and [Chrome's rendering
guidance](https://developer.chrome.com/blog/performant-parallaxing/) separately
warns that background-position can cause per-frame repaint. That establishes a
good candidate pipeline, not a result: [CSS Masking](https://www.w3.org/TR/css-masking-1/)
describes a separate compositing operation over a temporary drawing surface;
[layer guidance](https://web.dev/articles/speed-layers) explains that promoted
layers consume memory and require texture uploads. The Pen's own CPU figures,
reported in a [secondary mirror of the author's explanation](https://ngntipkolamrenang.twstalker.com/chrissm79),
are an anecdote without a reproducible device, browser, node count, trace, or
workload. They are not Poodle evidence.

Poodle should not add a generic TextShimmer prop, class, or animated treatment to
Text. Text is semantic and intentionally neutral. Skeleton already owns shape
placeholders; Spinner and Progress already own activity/progress indicators. A
shimmer on arbitrary text would mix meaning, visual decoration, selection, font
layout, and cross-runtime capability in one generic surface.

If there is a confirmed product need, the narrow role is a semantic loading/status
message accompanying an existing busy state: one source-of-truth text node,
parent-owned live-region semantics, opt-in visual treatment, static readable
fallback, and explicit active/paused/reduced-motion behavior. The first plausible
consumers are a future loading-message part of PageLoading and an activity line
such as AgentSubagent; the current streaming AgentMessage body is a poor target
because it already has a deliberately hidden caret and a transcript-level
announcement model.

The research recommendation is therefore: keep the triage note open for the
product/parity decision, reject a standalone generic text effect, and do not open
an implementation card until the operator chooses (a) the semantic consumer and
(b) web-only recipe versus active-cohort native parity. A web prototype, if later
approved, should benchmark mask-plus-transform against Poodle's current
background-position shimmer and a static fallback on the three supported web
engines. A cross-runtime component is not currently feasible without a new native
text-mask/presentation capability; the current poodle-node and GPUI backend have
no such channel.

## Method and source inventory

### Method

Local inspection covered the repository authority chain requested by the triage
note: AGENTS.md, docs/README.md, docs/research/README.md, the originating
triage note, active architecture and contracts, the repo-local Effigy skill,
component source, token schema, Svelte and React previews/tests, shared Rust
contracts/rendering, and GPUI preview/backend code. The local audit was read-only.

The CodePen was inspected through its public Pen metadata and its raw public file
URLs. The in-app browser backend was unavailable on this worker: browser discovery
returned no available browsers on 2026-08-31. No substitute browser automation or
visual trace was run. Raw source retrieval succeeded, so source anatomy is
verified; visual frame timing, paint flashing, layer count, and CPU/power numbers
remain promotion-gate work.

The external evidence is classified as follows:

- **Sourced fact** — directly stated by the linked specification, browser guidance,
  official CodePen text, or raw Pen source.
- **Local observation** — exact Poodle path and line range inspected in this
  worktree.
- **Inference** — a conclusion from source structure or the rendering model; it
  needs a trace or targeted test before becoming a performance contract.
- **Proposed gate** — acceptance criteria for a future implementation, not a
  current Poodle guarantee.

### External source inventory

All sources in this table were checked on 2026-08-31 unless a source's own update
date is stated.

| Source | Evidence used | Authority / limitation |
| --- | --- | --- |
| [Public Pen](https://codepen.io/devongovett/pen/wBJBRGX), supplied [editor lead](https://codepen.io/editor/devongovett/pen/01a0439c-4b7f-7f44-bf84-205c514ad139), and raw [HTML](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/index.html), [CSS](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/style.css), [JS](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/script.js) | Actual DOM, CSS branches, animation properties, absence of JavaScript/dependencies | Primary artifact. The editor UI was not available in the browser backend; the raw public files were fetched directly. |
| [CodePen licensing docs](https://blog.codepen.io/docs/pens/licensing/) and [Terms of Service](https://blog.codepen.io/legal/terms-of-service/) | Public Pen MIT terms, private-Pen distinction, notice requirement, separation of user content from CodePen site | Primary service terms. Legal review remains appropriate for a derivative implementation. |
| [CodePen raw-file documentation](https://blog.codepen.io/docs/live-view/raw-file-urls/) | Public raw-file URL mechanism and codepenusercontent.com hosting | Primary service documentation. |
| [web.dev: high-performance CSS animations](https://web.dev/articles/animations-guide) | Pipeline triage, compositor-oriented transform/opacity, paint profiler, cautious will-change | Google web guidance; page says it was last updated 2020-10-06. |
| [web.dev: rendering performance](https://web.dev/articles/rendering-performance), [Chrome: performant parallaxing](https://developer.chrome.com/blog/performant-parallaxing/), and [web.dev: layers](https://web.dev/articles/speed-layers) | Layout/style/paint/composite stages; background-position repaint risk; layer RAM/VRAM and texture upload cost | Browser-engine guidance, not a guarantee for every property/engine/version. |
| [W3C CSS Masking](https://www.w3.org/TR/css-masking-1/) and [MDN mask-image](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/mask-image) | Masking is a graphical compositing step; alpha/luminance mask behavior; gradients as mask images; support is not proof of correct/performance behavior | W3C specification plus MDN implementation reference. The specification describes the model, not a particular engine's fast path. |
| [MDN mask-clip](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/mask-clip), [MDN CSS nesting](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Nesting), and [MDN feature queries](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Conditional_rules/Using_feature_queries) | Text clipping support and partial-support caution; the Pen's nested CSS; need for explicit capability fallback | MDN reference. @supports detects parseable declarations, not all implementation bugs or partial behavior. |
| [MDN reduced motion](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion), [WCAG 2.2 Pause, Stop, Hide](https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html), and [WCAG 2.3.3](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html) | User preference, persistent motion obligations, preload/essential exception, and interaction-triggered animation guidance | W3C/MDN accessibility guidance. Applicability depends on product placement and duration. |
| [MDN forced colors](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/forced-colors) and [MDN inert](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inert) | Gradients/background images can be suppressed in forced colors; inert removes a duplicate from interaction and the accessibility tree | MDN platform reference; assistive-technology testing is still required. |
| [WCAG 2.2 contrast minimum](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum) and [WAI-ARIA 1.2 status](https://www.w3.org/TR/wai-aria/#status) / [progressbar](https://www.w3.org/TR/wai-aria/#progressbar) | Text contrast thresholds, placeholder-text applicability, status/live semantics, and determinate/indeterminate range semantics | Normative accessibility references. |
| [MDN web-font performance guidance](https://developer.mozilla.org/en-US/blog/optimize-web-performance/) | Fallback and web-font metric differences can cause layout shifts | MDN guidance; the clone-divergence conclusion below is an inference applied to the Pen's two independently formatted text nodes. |
| [Secondary mirror of the author explanation](https://ngntipkolamrenang.twstalker.com/chrissm79) | Attributes the “background-position is CPU intensive; mask plus transform reduced CPU” rationale and approximate numbers to Devon Govett | Secondary mirror only. The original social post was not independently reachable; no quoted number is used as evidence or an acceptance threshold. |

No external source code was copied into this dossier or into the repository.

## Current Poodle audit

### Authority and ownership

The active architecture in docs/architecture/001-poodle-system-shape.md defines
one contract across Svelte, React, shared Rust composition, and GPUI; the active
cohort is Svelte, React, and GPUI, with Jetstream deferred. Observable semantics,
accessibility, token use, and layout intent lead runtime-specific rendering. The
same ownership is repeated in docs/contracts/001-working-rules.md: Svelte is the
reference implementation, web styles/behavior belong in core, native composition
belongs in poodle-render, and every active-cohort component needs a contract,
both web shells, a Rust spec/render path, and a GPUI specimen.

docs/architecture/007-appearance-recipe-contract.md makes recipes the supported
appearance override surface. Recipes are CSS custom-property contracts and may
contain web-only effects, but a web-only effect must not imply cross-runtime
parity. That distinction is central to this track. docs/architecture/006-headless-
core-and-machine-model.md also requires named effects with explicit cleanup when
behavior, rather than a pure CSS treatment, is introduced.

### Existing loading and text semantics

| Surface | Current contract and implementation | Research consequence |
| --- | --- | --- |
| Skeleton | docs/contracts/components/skeleton.md:10-18 owns shape placeholders and optional shimmer, with decorative aria-hidden markup and parent-owned aria-busy/status. Web CSS in packages/core/src/styles/skeleton.css:4-39 animates a three-stop gradient by background-position over 1.6s. Svelte and React render equivalent shape/preset DOM in packages/svelte/components/src/Skeleton.svelte:30-80 and packages/react/components/src/Skeleton.tsx:30-103. | This is the existing non-semantic placeholder treatment. A text shimmer should not become a second way to represent unknown layout. The current background-position path is the correct baseline to measure, not a proven optimized implementation. |
| Spinner | docs/contracts/components/spinner.md:10-15,102-124 owns compact activity indication; it is decorative without a label and status/polite when explicitly labeled. Web CSS has ring/dots/grid loops in packages/core/src/styles/spinner.css; Svelte/React shells expose the same variants. | Existing loading consumers already have an activity signal. A shimmer should not be added merely because a spinner is present. |
| Progress | docs/contracts/components/progress.md:14-18,90-114 owns determinate range semantics and an indeterminate state. Web implementation exposes role=progressbar and the required determinate range fields in packages/svelte/components/src/Progress.svelte:37-51 and packages/react/components/src/Progress.tsx:32-48. | Shimmer must never carry numeric progress or replace Progress. |
| PageLoading | docs/contracts/components/page-loading.md:10-14,17-25,101-120 owns a status/polite loading surface with Spinner, optional Progress, message, and cancel. Web markup is role=status with aria-live=polite in packages/svelte/components/src/PageLoading.svelte:32-68 and packages/react/components/src/PageLoading.tsx:36-63; overlay CSS is fixed with backdrop blur in packages/core/src/styles/page-loading.css:4-42. | This is the strongest existing loading host, but the message is already accompanied by a spinner/progress. An optional appearance must not duplicate status semantics or obscure the message. The current contract does not specify focus trapping/inert content, pause control, reduced motion, or forced-color behavior. |
| Text | docs/contracts/components/text.md:11-15,48-54 owns element choice, tone, size, leading, spacing, and clamp; it adds no ARIA role. Svelte and React are semantic wrappers in packages/svelte/components/src/Text.svelte:5-40 and packages/react/components/src/Text.tsx:5-39. | Do not add a generic shimmer prop. It would turn a semantic text primitive into a loading-state owner and create an API with no native equivalent. |
| AgentMessage / AgentTranscript | docs/contracts/components/agent-message.md:177-189 keeps the message body out of the live region and makes the streaming caret aria-hidden; packages/core/src/styles/agent-message.css:105-129 already makes the caret reduced-motion aware. docs/contracts/components/agent-transcript.md:287-295 places polite announcement at the log level, not on each token. | The body is meaningful content and can be multiline, Markdown, links, and variable-height. A shimmer mask over it risks readability and selection. Preserve the current caret/activity model unless a separate contract explicitly replaces it. |
| AgentSubagent | docs/contracts/components/agent-subagent.md:46-64,128-173 defines a one-line activity while non-terminal, with a spinner only for running; the contract is still draft and the React/native variants are deferred (:277-287). | Its running activity line is the best semantic shape for a future experiment, but it is not a current cross-runtime admission target. |

### Runtime and token audit

| Runtime/layer | Exact local evidence | Current support / gap |
| --- | --- | --- |
| Svelte | packages/svelte/components/src/index.ts exports Text, Skeleton, Spinner, Progress, and PageLoading; corresponding component files import shared core CSS. Preview cases are packages/svelte/preview/src/specimens/SkeletonSpecimen.svelte:6-45, ProgressSpecimen.svelte:7-32, PageLoadingSpecimen.svelte:38-92, and AgentMessageSpecimen.svelte streaming cases. | Svelte is the reference web surface. Existing specimens cover basic loading axes but not wrapping/multiline, bidi, variable fonts, forced colors, offscreen pausing, or selection. |
| React | packages/react/components/src/index.ts exports the same loading/text surface; packages/react/preview/src/gallery/specimen-map.ts:301-339 maps the paired specimens. | React is source-only/experimental but must follow the same contract. Its SkeletonSpecimen.tsx:4-46, ProgressSpecimen.tsx:5-34, and PageLoadingSpecimen.tsx:27-103 have the same coverage limits. |
| Shared Rust | packages/contracts/components/src/skeleton.rs:17-120, page_loading.rs:28-128, and progress.rs:4-118 define portable specs. packages/render/src/page_loading.rs:80-109 composes Spinner, Progress, and message; packages/render/src/text.rs:21-34 creates one native text node with wrapping and an explicitly documented clamp degradation. | Native semantics and composition exist, but there is no text-shimmer spec or renderer-neutral visual channel. PageLoadingSpec has no motion, pause, or focus/inert fields. |
| poodle-node | packages/contracts/node/src/lib.rs:324-328 has static gradient and one NodeAnimation; :796-844 limits animated properties to opacity, rotate, translate X/Y, and scale X/Y. There is no mask, glyph-clip, background-position, or text-highlight channel. | A translated highlight cannot become a text-shaped highlight from this vocabulary alone. Any cross-runtime adoption requires an explicit contract and node capability, not a hidden backend special case. |
| GPUI backend | packages/gpui/node-backend/src/lib.rs:891-913 says opacity is the only generic animated property; SVG rotation is special-cased at :481-527. packages/gpui/node-backend/src/style.rs:211-215 maps only gradient endpoints when more than two stops are supplied. :917-922 records that GPUI 0.2.2 does not map NodeA11y. | Native feasibility is currently low. A mask-to-glyph operation and a generic translated highlight are both missing; accessibility is already a declared GPUI runtime gap. |
| Native Skeleton | packages/render/src/skeleton.rs:1-7 explicitly says a shimmer sweep is not representable and uses an opacity pulse; :56-73,162-168,256-260 implement that pulse. docs/contracts/components/skeleton.md:350-356 still describes the intended native periodic background-position equivalent. | This is a known contract/implementation delta. Do not use it as evidence that a new text shimmer can be ported by reusing current native animation. |
| Tokens | Canonical source is packages/tokens/schema/. Motion primitives are only fast/standard/slow durations in primitives/motion.json:1-27; semantic motion aliases are in semantic/motion.json:1-23. Semantic colors and theme overrides are in semantic/color.json:1-104 and modes/themes/*.json. | No shimmer-specific semantic roles exist. If a future contract needs a highlight/base pair, add token roles only after contrast and theme tests establish a reusable need; do not hardcode the Pen's #777 or black/white colors. |

### Existing tests and demonstrations

The repository's Svelte axe sweep in test/a11y/component-a11y.test.ts:35-72 runs
against the component surface, but test/fixtures/component-props.ts:193 only
declares a special Progress fixture for this family. It does not prove duplicate
text, selection, live-region stability, forced colors, or animation preference
behavior for a future shimmer.

The focused component tests are semantic smoke tests: for example,
packages/svelte/components/test/PageLoading.test.ts:7-61 checks status, visibility,
backdrop, progress, message, and cancel; Spinner.test.ts:7-42 checks decorative
versus announced states; and AgentMessage.test.ts:35-45 checks the hidden
streaming caret. There is no shimmer test in either web runtime.

The visual gate intentionally skips animation-bearing specimens:
test/visual/config.ts:40-49 lists spinner, skeleton, progress, and page-loading.
The baseline log docs/logs/2026-08/08-baseline-refresh-and-two-real-bugs.md:82-103
records measured frame drift from the dot-grid loading animation and retains the
skip list. This is direct local evidence that a future motion effect needs a
deterministic capture/trace strategy before it can enter visual conformance.

The current PageLoading preview timers are also only demonstration controls:
packages/svelte/preview/src/specimens/PageLoadingSpecimen.svelte:6-35 and
packages/react/preview/src/gallery/specimens/PageLoadingSpecimen.tsx:27-60 clear
the interval on the demo's close action but do not provide an unmount cleanup for
the interval/timeout. This is not evidence that the shipped component leaks; it is
evidence that interruption and unmount are not currently exercised by that
specimen.

## Detailed findings

### 1. What the reference actually does

The Pen's HTML has one ordinary text span and a second span with the same literal
content and the inert attribute. The second span is absolute and transparent,
and its pseudo-element owns a narrow light/dark gradient. The CSS uses
contain: strict and overflow: clip on the clone. It has two mask branches:
the author's WebKit-prefixed text-clip path and a Firefox -moz-element(#text)
fallback. The author comments that the Firefox fallback is not GPU accelerated
and can use high CPU. The highlight, rather than the text layout, moves from one
side to the other using the independent translate property. There is no JS
timer, requestAnimationFrame loop, library, or remote asset.

The Pen hides the clone under prefers-reduced-motion: reduce, so the original
text remains. That is a good progressive fallback pattern. It is not a complete
Poodle contract: the demo is one English line, uses system-ui, hardcodes #777
and black/white highlight colors, has no explicit dir, and provides no test
matrix for width changes, fonts, selection, forced colors, offscreen content, or
SSR/hydration.

The two copies are also a migration concern. They must receive identical text,
font properties, width, line-height, whitespace, direction, and font-loading
state to remain aligned. This is an inference from the two independent DOM
formatting contexts. It is reinforced by MDN's font-performance guidance: a
fallback and web font with different dimensions can cause layout shifts
([MDN](https://developer.mozilla.org/en-US/blog/optimize-web-performance/)).
Poodle must not make callers maintain two strings or duplicate rich child trees.

### 2. Rendering pipeline: plausible optimization, not a GPU guarantee

The useful distinction is not “CSS versus GPU”; it is which stage changes and what
surfaces are created.

1. **Layout and geometry.** The Pen's wrapper and original text participate in
   inline layout. The clone is absolutely positioned, so changing its translation
   should not move surrounding content. The duplicate still has to be laid out
   initially and whenever text, width, font, writing mode, or inherited styles
   change. The no-layout conclusion is therefore limited to the animation frame,
   not to the component's full lifecycle.
2. **Style and paint.** The gradient is paint content. The
   [CSS Masking model](https://www.w3.org/TR/css-masking-1/) describes drawing
   the element and its descendants into a temporary
   canvas/buffer and then applying masking before compositing. This explains why
   a mask can be useful while still having an offscreen/raster cost. The W3C model
   also says masking does not affect hit testing, which is helpful for pointer
   behavior but does not solve duplicate selection or accessibility by itself.
3. **Compositing.** [Browser animation guidance](https://web.dev/articles/animations-guide)
   says transforms and opacity are generally favorable compositor candidates.
   The Pen's translate therefore gives the
   browser a plausible opportunity to move an already-painted highlight surface.
   It does not prove that the mask, text clone, or gradient remains entirely off
   the main thread in Blink, WebKit, or Gecko. Mask behavior, layer promotion,
   raster invalidation, and texture upload remain engine- and size-dependent.
4. **Layer memory and upload.** [Layer guidance](https://web.dev/articles/speed-layers)
   explains that a promoted layer can avoid repainting unchanged content, but
   layers consume RAM/VRAM and their textures need uploading. On a
   page with many messages or long multiline text, a “one layer per shimmer” rule
   can cost more than the paint it avoids.
5. **will-change.** The Pen uses will-change: transform. [Official web
   guidance](https://web.dev/articles/animations-guide) recommends using
   will-change sparingly, only after a demonstrated problem,
   and removing it when the change stops. Poodle should not bake permanent
   will-change into every Text or loading message.

The current Poodle Skeleton uses background-position
(packages/core/src/styles/skeleton.css:26-39). [Chrome's rendering
guidance](https://developer.chrome.com/blog/performant-parallaxing/) specifically uses
changing background-position as an example that can repaint affected parts every
frame, while recommending accelerated properties such as transforms and opacity.
That supports measuring the Pen's strategy as a candidate improvement. It does not
prove that every background-position animation is slow in every engine, nor that
every mask-plus-transform animation is cheap.

**Verdict on “GPU optimized”:** use “compositor-oriented candidate” or
“GPU-conscious candidate” in Poodle research. Reserve “GPU accelerated” for a
measured engine/device result with a trace showing the relevant paint/raster and
composite behavior.

### 3. Text correctness: wrapping, fonts, bidi, and localization

- **Wrapping and multiline.** The Pen's duplicate can align only while both copies
  resolve identical width, font, line-height, white-space, letter-spacing,
  ligatures, and content. Absolute positioning does not guarantee identical line
  boxes when the host changes constraints or fonts. A future component must test
  one line, two lines, five lines, long localized strings, narrow containers,
  resize, and content updates while active. It should not promise arbitrary rich
  children until a single-source rendering strategy is proven.
- **Variable fonts.** Variation axes, optical sizing, font-feature settings, and
  late font loading can change glyph metrics and line breaks. [MDN's
  font-performance guidance](https://developer.mozilla.org/en-US/blog/optimize-web-performance/)
  documents the metric and layout-shift risk. The correct design is
  to derive any visual overlay from the same rendered text state and to disable or
  fall back to static text until layout is stable if alignment cannot be proven.
  This is a migration risk for AgentMessage, where a streaming body can change
  every frame.
- **RTL and writing modes.** The Pen's gradient angle and animation endpoints are
  physically left-to-right. It does not read dir or use a logical sweep. It may
  look acceptable in an RTL paragraph by accident, but that is not a contract.
  Poodle should follow the host's dir/writing mode by default and test Arabic,
  Hebrew, mixed bidi text, punctuation, and numerals. A public left/right speed or
  direction knob is unnecessary until there is a product need.
- **Localization.** Do not make the caller provide a second translated string for
  the visual copy. The semantic string must be the only source of truth. If a
  duplicate layer remains the least-cost implementation, the component must own
  the duplicate and keep it noninteractive and hidden from AT.
- **Selection and copy.** The Pen's inert clone is intended to keep the duplicate
  out of focus, selection, pointer interaction, and the accessibility tree. The
  real text remains the selectable copy. Poodle must test selecting across the
  effect, copying a partial and full range, selection highlight contrast, keyboard
  focus on adjacent links/buttons, and touch selection. An implementation that
  makes the real text transparent and exposes only a painted overlay is rejected.

### 4. Accessibility and motion

The effect has no information to announce. It must never be the live region and
must not change the accessible name. The host owns semantics:

- A loading/status host may use the existing role=status/polite pattern, or a
  content region may use aria-busy according to its contract. [WAI-ARIA's status
  role](https://www.w3.org/TR/wai-aria/#status) is a live-region pattern and
  should not be given to each visual layer.
- A determinate host continues to expose [progressbar range
  semantics](https://www.w3.org/TR/wai-aria/#progressbar). An indeterminate host
  omits numeric range attributes. Shimmer is neither state nor progress.
- The semantic text stays in the accessibility tree exactly once. Any implementation
  duplicate must be inert/aria-hidden, nonfocusable, nonselectable, and
  pointer-inert, with tests proving that it cannot become the only readable copy.
  The [inert attribute](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inert)
  formally removes an element from focus and the accessibility tree and blocks
  selection and pointer interaction.
- The real text must remain visible with a static color in unsupported browsers,
  forced colors, print, and reduced motion. [MDN's forced-colors
  guidance](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/forced-colors)
  documents that forced-colors mode
  can suppress non-URL background images/gradients and force author colors to the
  system palette. A transparent-only text treatment therefore fails the fallback
  requirement.
- Poodle's base text must meet [WCAG contrast
  requirements](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum)
  in every supported theme. WCAG's
  4.5:1 normal-text and 3:1 large-text thresholds apply to placeholder text too;
  the animated highlight is not a substitute for that contrast. The Pen's demo
  #777 is approximately 4.48:1 on white, just below the 4.5:1 normal-text
  threshold, which is another reason not to import its colors.
- The Pen's reduced-motion branch is a sound baseline: remove the effect, preserve
  the text. [The reduced-motion media
  feature](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion)
  should also provide a host-controlled off/paused state. A
  reduced-motion user should not need to find a pause control for a purely
  decorative effect.
- [WCAG 2.2's pause/stop/hide
  guidance](https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html)
  requires a pause/stop/hide mechanism for automatically moving content
  that lasts more than five seconds alongside other content unless the movement is
  essential. The preload exception is conditional, not a blanket loader
  exemption. A long-lived activity message should either expose a non-blocking
  host-level pause/hide mechanism or be explicitly scoped as essential preload.
  This decision belongs to the semantic host, not a visual utility.

The existing AgentMessage treatment demonstrates the preferred separation:
the caret is aria-hidden, the message body is not a token-level live region, and
reduced motion removes blinking (docs/contracts/components/agent-message.md:177-
189; packages/core/src/styles/agent-message.css:105-129). A text shimmer should
not weaken that contract.

### 5. Interruption, cancellation, visibility, and lifecycle

The Pen has no JS lifecycle. Removing the clone stops its animation naturally.
That is good for a pure appearance effect, but it does not provide product
semantics:

- active/inactive should be derived from the host's loading/activity state.
- paused should stop motion without changing text, layout, or announcement.
- cancellation is the host's operation callback (PageLoading already has
  onCancel); it is not an AbortSignal owned by a CSS effect.
- unmount, hidden tab, virtualized row removal, and offscreen content must not
  leave timers, observers, or animation work behind. A no-JS CSS path is preferable
  for this reason, but CSS animation being declarative is not proof that offscreen
  work is free.
- If Poodle adds an IntersectionObserver or visibility observer to pause, that is
  a named effect with cleanup under the active architecture. It must be justified
  by measurement; do not add a runtime observer to every text node preemptively.

AgentTranscript already virtualizes variable-height content and moves a visible
slice with translateY (packages/svelte/components/src/AgentTranscript.svelte:
282-303; React equivalent :355-371). A shimmer attached to virtualized message
content would need explicit interaction with windowing: pause when not materialized,
restart when materialized, and preserve the semantic log behavior. This makes the
transcript body a particularly poor first consumer.

### 6. Browser support, no-build delivery, and fallback

[MDN's mask-image reference](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/mask-image)
describes broad support for mask-image but explicitly notes that
some parts vary. mask-clip:text and the prefixed WebKit path need capability
testing, not property-name optimism. The Pen's Firefox fallback is author-labeled
high CPU and should not be promoted as a universal optimized path.

The Pen also uses nested @media, @supports, and &::after rules. [CSS nesting
guidance](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Nesting)
describes browser parsing and broad current-engine availability, but a no-build Poodle
stylesheet should emit flat CSS or prove its minimum browser baseline. [Feature
query guidance](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Conditional_rules/Using_feature_queries)
describes their use for progressive enhancement, but cautions that they test
whether declarations parse, not whether an implementation is bug-free or fully
correct.

Recommended web fallback order for a future experiment:

1. semantic ordinary text, always present;
2. opt-in mask/highlight enhancement only when the complete tested mask path is
   supported;
3. static text when unsupported, forced colors, reduced motion, printing, or
   host-controlled pause/off state applies;
4. no Firefox -moz-element fallback unless a measured target matrix proves its
   CPU cost acceptable. Otherwise it is a static fallback, not a silent capability
   gap.

background-clip:text is not a free alternative. It usually requires making the
real text transparent and painting the text through a background, which raises
selection, forced-color, and fallback risks. It may be a benchmark candidate,
not the default recommendation.

### 7. Themes and visual tokens

The Pen's color-scheme, #777, and black/white highlight are demo values, not
Poodle roles. Poodle themes range from iceberg light values to eclipse dark values
(packages/tokens/schema/modes/themes/iceberg.json and eclipse.json), and semantic
roles live in packages/tokens/schema/semantic/color.json.

A future effect should use a semantic text role as the readable base and a
component-scoped recipe hook only for a measured highlight treatment. The base
copy must remain valid without the hook. Highlight opacity must not reduce the
least-contrasting glyph region below the theme's WCAG target. Forced colors must
revert to system text and suppress decorative gradients rather than overriding the
user palette.

Do not add arbitrary speed, spread, color, or gradient escape hatches to Text. If
an adopted loading contract needs timing, it should select a bounded semantic
motion token; if it needs theme variation, it should use the normal
recipe-to-semantic-token ladder.

### 8. GPUI feasibility and cross-runtime risk

The current native path cannot reproduce the reference mechanism:

- SkeletonSpec has only a boolean animation flag and token helpers
  (packages/contracts/components/src/skeleton.rs:17-120).
- poodle-render currently flattens Skeleton colors and uses an opacity pulse;
  its source says the moving gradient is not representable
  (packages/render/src/skeleton.rs:1-7,30-73).
- NodeStyle can declare a static gradient and NodeAnimation, but AnimProperty
  has no background-position or mask property (packages/contracts/node/src/lib.rs:
  324-328,796-844).
- GPUI's generic animation adapter applies opacity only; its special transform path
  is for SVG rotation (packages/gpui/node-backend/src/lib.rs:481-527,891-913).
- GPUI's gradient adapter keeps only endpoints when a longer stop list arrives
  (packages/gpui/node-backend/src/style.rs:211-215), and GPUI 0.2.2 has no native
  accessibility projection (packages/gpui/node-backend/src/lib.rs:917-922; the
  accepted gap is recorded in docs/contracts/003-native-accessibility.md).

Native options are therefore:

- add a renderer-neutral text-effect/presentation channel that can express a
  glyph-clipped highlight and a stable animation identity, then teach GPUI to
  paint it; or
- define the feature explicitly as a web-only recipe, with no claim that GPUI
  renders the same visual effect, while keeping semantic loading behavior aligned.

The first option is a real architecture project, not a component CSS port. The
second is allowed by the recipe architecture but must be chosen explicitly; it
cannot be smuggled in as a runtime approximation on a supposedly shared visual
contract. Jetstream remains deferred and is not a reason to expand this track.

### 9. Measured evidence still required

No trace was run in this worker because the browser-control backend was absent.
The author's approximate “about 20% CPU” and “under 2% CPU” statements are not
portable evidence. A disposable reproduction should be created only in a later
implementation/benchmark card and should compare:

- static text;
- Poodle's current Skeleton-style background-position gradient;
- mask plus translated highlight;
- any tested background-clip:text alternative;
- static fallback for unsupported/Firefox paths.

The matrix should include Chromium/Blink, Firefox/Gecko, and Safari/WebKit;
desktop and a low-power/mobile target; DPR 1 and 2; light/dark Poodle themes;
one-line, multiline, long localized, RTL/mixed bidi, variable-font, and resize
cases; 1, 10, 50, and 100 simultaneous messages; foreground, offscreen, and hidden
tab states; reduced motion; forced colors; and selection/copy.

Record, after warm-up and over a fixed interval:

- layout shifts and geometry changes;
- main-thread style/layout/paint time and paint invalidations;
- raster/compositor time, dropped frames, frame-time percentiles;
- layer count, layer dimensions, memory/texture allocation and uploads where the
  engine exposes them;
- CPU/package power where the device allows it;
- text sharpness, mask alignment, selection result, accessible-tree result, and
  fallback result.

The web.dev guidance points to DevTools paint flashing and paint profiling for this
kind of diagnosis. A pass must show the claimed pipeline on the target engine, not
just that the CSS property is named transform. Operator-approved budgets are
still needed; the following are proposed gates rather than current promises:

- zero layout/geometry movement caused by the effect;
- no duplicate accessible or copied text;
- no per-frame layout, and no per-frame paint on an engine/path advertised as
  compositor-oriented after the initial warm-up;
- bounded layer/memory growth as node count increases;
- no material frame-time regression against static text at the agreed node counts;
- readable text and contrast in every theme, with static system-color output in
  forced colors and reduced motion;
- deterministic visual captures either freeze the effect by contract or test a
  stable state; no “two captures happened to agree” rule for moving frames.

## Options and tradeoffs

| Option | Benefits | Costs / failure mode | Assessment |
| --- | --- | --- | --- |
| Semantic loading-message composite (LoadingText, or a contract extension to the PageLoading message part) | Meaning and lifecycle are explicit; live-region ownership stays with the host; can constrain copy, motion, and consumer scope; can be specified for all active runtimes | Requires a new contract and native feasibility work; PageLoading already has Spinner/Progress; operator must choose the exact consumer | **Recommended product shape if need is confirmed.** Start with one-line activity/status copy, not arbitrary rich text. |
| Opt-in appearance on Text | Familiar API; easy web adoption; could reuse recipes | Makes a semantic primitive own loading state; duplicate/mask risks affect every text consumer; no natural GPUI channel; invites arbitrary speed/color props | **Reject as public API.** |
| Reusable web utility/class in poodle-core | Zero JS, no-build CSS possible, easy progressive enhancement; can remain a recipe implementation detail | Has no semantics, lifecycle, pause policy, or native parity; downstreams may apply it to links, long prose, or important text | Accept only as an internal implementation of a semantic host, or explicitly web-only recipe with narrow documentation. |
| Direct CodePen-shaped duplicate/mask | Small source; preserves base text; reduced-motion branch is simple | Literal duplicate maintenance; multiline/font/bidi divergence; hardcoded demo colors; mask and will-change costs; not copied into Poodle | Use as prior art and benchmark candidate, not as a port. |
| Background-clip text / transparent real text | Fewer DOM nodes; potentially simpler geometry | Real text can disappear in forced colors/unsupported browsers; selection and antialiasing suffer; fallback is fragile | Benchmark only; not default. |
| Static text plus existing Spinner/Progress | Lowest risk; strongest semantics and native parity; no new browser cost | Less decorative feedback; does not meet a confirmed visual request if one exists | Default fallback and current product recommendation. |

## Recommended Poodle direction

1. **Product role:** do not treat shimmer as a new loading primitive. If adopted,
   make it an opt-in appearance of a semantic, transient loading/status message
   that accompanies a host-owned busy state. The message remains useful without
   animation. The strongest current shape is a short PageLoading message or a
   running AgentSubagent activity line; the latter waits for its draft contract
   and active-cohort wiring.
2. **API boundary:** no TextShimmer component and no Text shimmer prop. A future
   contract may be named LoadingText or may add a narrowly scoped
   messageAppearance to PageLoading; choose one after an operator confirms the
   consumer. Keep AgentMessage body and generic Text out of the first card.
3. **Semantic contract:** one semantic text source, no live-region behavior owned
   by the effect, no numeric progress meaning, no focusability, and no altered
   selection/copy. If a duplicate layer is used internally, the component owns
   aria-hidden/inert/selection/pointer isolation and proves it in tests.
4. **Motion contract:** default auto honors prefers-reduced-motion; explicit
   inactive/paused state renders static text; a host that can run longer than five
   seconds alongside other content must provide pause/stop/hide or document the
   applicable essential-preload posture. No JS timer or AbortSignal is needed
   for a pure CSS appearance; operation cancellation remains the host's callback.
5. **Web strategy:** after measurement, prefer a feature-queried mask/highlight
   enhancement whose moving part is transform-like, with an always-readable base
   text fallback. Use semantic tokens and component recipe hooks. Emit flat CSS in
   the no-build path. Do not ship blanket will-change; scope any hint to active,
   measured instances and remove it when inactive.
6. **Native strategy:** do not claim active-cohort visual parity until a disposable
   GPUI spike proves glyph clipping/mask and stable animation through the
   renderer-neutral node vocabulary. If the operator selects web-only treatment,
   document it as a web recipe and keep native semantics/static presentation
   explicit. Do not silently substitute the existing Skeleton opacity pulse for a
   text shimmer.
7. **Terminology:** use “GPU-conscious” or “compositor-oriented candidate” until
   the benchmark demonstrates the exact browser/device pipeline. Never promote the
   author's CPU percentages as Poodle performance data.

## Explicit non-goals

- No implementation, fork, or copied fragment of the CodePen reference.
- No generic animated Text API or visual treatment for arbitrary prose,
  headings, links, inputs, or transcript bodies.
- No replacement for Skeleton, Spinner, Progress, PageLoading, or the
  existing streaming caret.
- No progress percentage, completion state, retry state, or cancellation semantics
  encoded by shimmer.
- No caller-maintained duplicate strings or duplicate rich child trees.
- No JS animation loop, requestAnimationFrame driver, timer, or dependency for the
  visual effect unless a later benchmark proves a named lifecycle feature is
  necessary.
- No permanent will-change on every text node, unbounded layer promotion, or
  unsupported Firefox fallback presented as optimized.
- No new compatibility aliases, silent fallbacks, generated-token edits, or
  changes to current contracts in this research batch.
- No app-specific DAW/widget behavior and no Jetstream admission work.

## Risks

| Risk | Impact | Mitigation / owner decision |
| --- | --- | --- |
| “GPU optimized” is repeated without tracing | Misleading performance claim; regressions on low-power devices | Benchmark by engine, node count, layer/raster evidence; use conservative terminology. |
| Mask/offscreen surface cost exceeds saved paint | CPU, memory, texture bandwidth, dropped frames | Measure long/multiline text and 1/10/50/100 nodes; cap scope or use static fallback. |
| Duplicate text drifts after font load, resize, or localization | Misaligned highlight, visible ghosting, layout instability | One source of truth; variable-font/font-load/resize/bidi matrix; disable enhancement when alignment is not proven. |
| Transparent text fails forced colors or unsupported browsers | Content becomes unreadable | Keep ordinary text visible; feature-query only the decoration; forced-color and print tests. |
| Motion distracts or blocks reading | Accessibility failure and user harm | Reduced-motion default, active/paused state, host-level pause/stop/hide decision. |
| Shimmer competes with status/progress signals | Ambiguous loading meaning and visual noise | Keep existing Spinner/Progress as semantic indicators; limit effect to short supporting copy. |
| Web/native capability gap | Cross-runtime drift; hidden GPUI approximation | Decide web-only recipe versus new node/GPUI capability before API promotion. |
| Existing Skeleton contract is itself ahead of native support | New work compounds unresolved animation drift | Reconcile the current Skeleton visual delta in a separate contract decision; do not reuse its native pulse as proof. |
| Animation visual tests remain nondeterministic | False passes or skipped coverage | Freeze/static mode for semantic tests; dedicated trace/visual harness with explicit phase control. |
| CodePen licence/derivative ambiguity | Attribution or legal issue | Preserve public-Pen MIT notice if future code is substantially reused; do not copy the reference; legal/operator review. |

## Unresolved operator decisions

1. Is there a confirmed Poodle-owned consumer for animated loading copy, or is the
   request only visual exploration? PageLoading already has Spinner/Progress;
   AgentMessage already has a caret; AgentSubagent is draft.
2. Should the product boundary be a new semantic LoadingText composite or a
   narrow PageLoading message appearance? This changes contract ownership and
   migration scope.
3. Must the visual effect be equivalent in Svelte, React, shared Rust, and GPUI,
   or is an explicitly web-only recipe acceptable under the active recipe
   architecture? A web-only choice must be visible in the contract and docs.
4. For a persistent activity line, what is the product's pause/stop/hide posture
   under WCAG 2.2? Is it essential preload, or must a non-blocking control/site
   mechanism exist?
5. What node-count, device class, CPU/frame-time, layer-memory, and battery budget
   is acceptable? The author figures do not answer this.
6. Are new semantic base/highlight token roles warranted after theme/contrast
   evidence, or should the first version use existing text/recipe roles only?
7. If a future implementation is derivative of the public Pen, who records the MIT
   notice/attribution and approves the legal treatment? The public-Pen terms permit
   reuse under MIT, but CodePen's Terms separately restrict copying the CodePen
   site/service itself.

## Promotion-ready contract, architecture, and card scope

This is a ready-to-scope shape, not approval to create these files in this research
batch.

### Contract scope

Candidate contract: docs/contracts/components/loading-text.md, or a narrowly
bounded amendment to docs/contracts/components/page-loading.md after decision 2.
It should define:

- semantic purpose and allowed consumers: transient activity/status copy, not
  arbitrary content;
- text/children source, host state (active, inactive, paused), and a bounded motion
  policy (auto/reduced/off) with defaults;
- no role by the visual effect; explicit host rules for status, aria-live, and
  aria-busy; exactly one accessible/copyable text value;
- wrapping, multiline, font-loading, variable-font, bidi, localization, resize,
  selection, and focus behavior;
- forced-color, print, unsupported-mask, reduced-motion, offscreen, hidden-tab,
  unmount, and cancellation behavior;
- theme/token/recipe slots and contrast requirements;
- allowed implementation difference: CSS mask/highlight on web only, or a new
  renderer-neutral native presentation channel if parity is required;
- no numeric progress meaning and no generic Text adoption;
- Svelte, React, shared Rust, and GPUI specimens/cases; Jetstream remains deferred.

### Architecture scope

- Keep semantic state and any cleanup in the owning host/core layer. Keep the
  effect's web CSS in packages/core and web shells thin.
- If web-only: make the recipe/feature-query boundary explicit and do not add a
  portable Rust field that promises an effect GPUI cannot paint.
- If cross-runtime: add the smallest renderer-neutral presentation capability for
  glyph-clipped text highlight, animation identity, pause/inactive state, and
  fallback; then implement poodle-render and GPUI interpretation before public
  web admission. The node vocabulary needs more than the current static gradient
  and generic opacity path.
- Keep semantic token source in packages/tokens/schema; do not edit generated
  artifacts directly. Add tokens only with contrast/theme evidence.
- Preserve the existing Underlay adapter boundary; no host should need to know
  Poodle's internal implementation.

### Implementation/benchmark card shape

The eventual card should be split into meaningful batches:

1. **Decision and contract:** settle consumer, web-only/native parity, motion and
   pause posture; write the contract before source changes.
2. **Disposable web benchmark:** implement independent prototypes in a temporary
   harness, without copying the Pen, and capture the full engine/device/content
   matrix. Include the current Skeleton-style baseline and static fallback.
3. **Web contract implementation:** only if the benchmark passes; add core CSS,
   thin Svelte/React shells, paired semantic tests, selection/AT tests, forced-color
   and reduced-motion cases, and specimens that expose wrapping/bidi/font/paused
   states.
4. **Native feasibility or explicit web-only admission:** if parity is required,
   spike node/GPUI glyph clipping and add shared Rust/specimen evidence; if not,
   record the web-only recipe and its static native posture in the contract.
5. **Conformance and closeout:** add deterministic visual handling, trace receipt,
   contrast/theme evidence, git diff --check, the narrow docs selector, and an
   updated triage disposition through the owning planning workflow.

### Proposed promotion gates

Promotion should require all of the following, with operator-approved numeric
budgets recorded in the card:

- one semantic/copyable/accessibility text value in Svelte and React;
- no new axe violations and explicit tests for status/live ownership, duplicate
  suppression, selection/copy, focus adjacency, reduced motion, forced colors,
  print, unsupported mask, unmount, and pause/inactive state;
- stable wrapping and alignment for the agreed multiline, variable-font, resize,
  RTL/localization, theme, and density/size cases;
- trace evidence for the exact engines and node counts that the phrase
  “GPU-conscious” covers, including layer/memory and paint/raster behavior;
- no layout movement and no unbounded layer or texture growth;
- deterministic visual capture policy that does not bless a coincidental animated
  frame;
- native contract/spec/render/GPUI evidence if the public contract implies active
  cohort visual parity, or an explicit web-only recipe decision otherwise;
- no copied reference implementation and a recorded licence/attribution decision.

## Proposed disposition of the originating triage note

**Keep open, promote this dossier as the research result, and mark it
decision-needed rather than implementation-ready.** The technical question has a
useful answer: mask-plus-transform is a credible web experiment and deserves a
benchmark, but “GPU optimized” is not yet verified and current GPUI cannot express
the visual mechanism. Close the triage note only after the operator selects a
semantic consumer and parity posture; otherwise supersede it with a decision note
that explicitly rejects the generic TextShimmer direction.

## Citations

The source inventory above contains the access/check dates and the direct links
used for each consequential external claim. The most important evidence paths are:

- [CodePen raw HTML](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/index.html) and [raw CSS](https://01a0439c-4b7f-7f44-bf84-205c514ad139.codepenusercontent.com/style.css)
- [CodePen licensing](https://blog.codepen.io/docs/pens/licensing/) and [Terms of Service](https://blog.codepen.io/legal/terms-of-service/)
- [web.dev high-performance CSS animations](https://web.dev/articles/animations-guide), [Chrome performant parallaxing](https://developer.chrome.com/blog/performant-parallaxing/), and [web.dev rendering performance](https://web.dev/articles/rendering-performance)
- [W3C CSS Masking](https://www.w3.org/TR/css-masking-1/)
- [MDN mask-image](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/mask-image), [forced colors](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/forced-colors), [reduced motion](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion), and [inert](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inert)
- [WCAG 2.2 Pause, Stop, Hide](https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html), [WCAG 2.2 contrast](https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum), and [WAI-ARIA 1.2](https://www.w3.org/TR/wai-aria/)
