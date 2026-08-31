# Value Track: Transitions.dev Catalogue Audit

Status: complete (evidence documented; implementation deferred)
Created: 2026-08-31
Updated: 2026-08-31
Origin: [`docs/triage/20260820-205249-transitions-dev-motion-learning.md`](../../triage/20260820-205249-transitions-dev-motion-learning.md)
Primary leads: [transitions.dev](https://transitions.dev/) and [Jakubantalik/transitions.dev](https://github.com/Jakubantalik/transitions.dev)

## Evidence labels

- **[VF] verified fact** — observed in the pinned upstream checkout, a Poodle
  source file, or an external authority linked at the claim.
- **[SC] source claim** — guidance or intent stated by Transitions.dev or an
  external authority, not an independent Poodle conclusion.
- **[WI] worker inference** — a Poodle interpretation, recommendation, or risk
  derived from the cited facts.

This dossier is research only. It copies no upstream recipe into Poodle and
does not change triage, indexes, contracts, architecture, specs, roadmaps,
package source, or generated artifacts.

## Executive summary

The captured Transitions.dev catalogue contains **43 cards: 32 free and 11
Pro**. The inventory below is exhaustive for the captured site source, not a
sample. The pinned source commit is
[`0f8540f9a7c200211d4bcc149d787382fa4567fa`](https://github.com/Jakubantalik/transitions.dev/commit/0f8540f9a7c200211d4bcc149d787382fa4567fa),
committed on 2026-08-31. The upstream data file has 43 records, the generated
site has 43 detail directories, and the public skill has 32 free reference
files. [VF]

The useful Poodle contribution is a small semantic motion contract, not a
43-item animation catalogue. The strongest matches are disclosure, transient
notification, tab selection, checkbox/icon state change, and loading/reveal.
They map to existing Poodle semantics and expose current parity gaps:

- Svelte has a panel slide for `Accordion` and `Collapsible`; React currently
  has structural presence only. `Drawer` has a Svelte-only edge/fade transition
  while React has none. [VF]
- Native `ToastStack`, `Skeleton`, and `Spinner` declare animations in shared
  Rust render output, while the web ToastStack has no corresponding presence
  animation and web loading loops have no general reduced-motion guard. [VF]
- Poodle has three primitive motion durations and two semantic duration roles,
  but no canonical distance, blur, stagger, phase, cancellation, or reduced
  motion policy contract. [VF]
- The native `NodeAnimation` vocabulary supports opacity, rotation, translation,
  scale, four named easings, and loop modes, but not width/height, blur, path
  drawing, or arbitrary cubic-bezier values. [VF]

**Recommended direction [WI]:** promote semantic intent and lifecycle rules
through the existing web-core/native-render architecture. Reuse the current
token baseline first; add only the missing roles demonstrated by the pilot.
Make reduced motion, semantic timing, stable identity, interruption, and final
state part of the contract. Keep decorative effects, marketing transitions,
private-reasoning presentation, image-specific effects, and Pro-only source
out of Poodle.

**Representative pilot [WI]:**

1. `Accordion` + `Collapsible` disclosure.
2. `ToastStack` plus the banner/stack replacement behavior.
3. `Tabs` sliding indicator.
4. `Checkbox` check draw plus the same-slot `IconButton`/state swap pattern.
5. `Skeleton` reveal and `Spinner` reduced-motion behavior.

This set covers layout, transient lifecycle, measured geometry, discrete state
change, and continuous loading without admitting decorative animation as a
component contract.

## Method and source inventory

### Capture and completeness

Checked 2026-08-31 in the worker worktree, Europe/London. The upstream checkout
was a shallow clone of the public repository at the commit above. The capture
used these sources, in descending authority for inventory:

| Source | Use | Finding |
|---|---|---|
| [`index.html`](https://github.com/Jakubantalik/transitions.dev/blob/0f8540f9a7c200211d4bcc149d787382fa4567fa/index.html) at the pinned commit | Rendered card list, site order, category attributes, Pro flags, public inline demos | 43 `article[data-proto-card]` cards; `data-seq` values 1–43; category attributes are filter labels, not mutually exclusive taxonomy. [VF] |
| [`scripts/transitions-data.json`](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) at the pinned commit | Machine-readable names, slugs, tier, CSS and React payloads | 43 records; 32 free records have CSS, 27 have optional React, and five AI-oriented free records have no React payload; all 11 Pro records have empty CSS/React fields. [VF] |
| [`scripts/build-transition-pages.py`](https://github.com/Jakubantalik/transitions.dev/blob/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/build-transition-pages.py) | Proves how static pages are derived | It says the data file is scraped from the running app, free snippets are exact detail-page output, and Pro pages contain demo/pitch but no source. [SC/VF] |
| [`build/extract.mjs`](https://github.com/Jakubantalik/transitions.dev/blob/0f8540f9a7c200211d4bcc149d787382fa4567fa/build/extract.mjs) and [`build/templates/`](https://github.com/Jakubantalik/transitions.dev/tree/0f8540f9a7c200211d4bcc149d787382fa4567fa/build/templates) | Proves the portable reference build | The extractor parses the inline `PROTO_TEMPLATES` and uses a curated 32-item `ORDER`; it emits `SKILL.md`, `_root.css`, and the 32 free reference files. [VF] |
| [`skills/transitions-dev/`](https://github.com/Jakubantalik/transitions.dev/tree/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev) | Free mechanism, timing, prerequisites, and reduced-motion source | `SKILL.md`, `_root.css`, and 32 numbered references are present. Every free reference includes a `prefers-reduced-motion` guard. [VF] |
| [`transitions/`](https://github.com/Jakubantalik/transitions.dev/tree/0f8540f9a7c200211d4bcc149d787382fa4567fa/transitions) | Generated per-transition pages | 43 detail directories are present, one for every data record. [VF] |
| [live site](https://transitions.dev/) | Operator-facing current catalogue | Live site was checked on 2026-08-31. This dossier uses the pinned checkout for reproducibility if the dynamic site changes. [VF] |

The local reconciliation was:

```text
data records       43
generated pages    43
free references    32
free cards         32
Pro cards          11
```

The root upstream [`README.md`](https://github.com/Jakubantalik/transitions.dev/blob/0f8540f9a7c200211d4bcc149d787382fa4567fa/README.md)
still describes an older 12/18-item surface. That is documentation drift,
not an inventory omission: the index, data, generated pages, and extractor
are internally reconcilable at this capture. [VF]

The site exposes these filter values: `all`, `essential`, `ai`, `effects`,
`texts`, and `pro`. The card attributes are overlapping labels. Counts in the
captured index are `essential=20`, `texts=3`, `effects=11`, `texts ai=2`,
`effects ai=1`, `ai=5`, and `ai effects=1`. The counts must not be summed to
derive 43. [VF]

### External evidence and licence boundary

The accessibility and implementation evidence checked on 2026-08-31:

- [WCAG 2.2 Understanding 2.3.3, Animation from Interactions](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html)
  says non-essential motion triggered by interaction must be disableable, with
  an exception where the motion is essential to the functionality or the
  information being conveyed. It also distinguishes interaction-triggered
  animation from automatically moving content. [SC]
- [MDN `prefers-reduced-motion`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/%40media/prefers-reduced-motion)
  documents the OS/user preference exposed to CSS and notes that scaling and
  panning large objects can trigger discomfort. [SC]
- [MDN accessibility media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Media_queries/Using_for_accessibility)
  recommends removing or reducing non-essential movement while recognising
  that reduced motion does not mean every animation must disappear. [SC]
- [CSS Transitions Level 2](https://www.w3.org/TR/css-transitions-2/) defines
  interruption/reversal behaviour and notes that removing an element from
  rendering cancels its transitions. [SC]
- [MDN Web Animations API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API),
  [MDN `Animation.cancel()`](https://developer.mozilla.org/en-US/docs/Web/API/Animation/cancel),
  and [MDN using the Web Animations API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Animations_API/Using_the_Web_Animations_API)
  establish that script-driven animations need explicit lifecycle control;
  `cancel()` aborts playback and clears the keyframe effect. [SC]
- [web.dev animation performance guidance](https://web.dev/articles/animations-guide)
  and [web.dev animations and performance](https://web.dev/articles/animations-and-performance)
  recommend transform/opacity where possible and warn that geometry changes
  such as width/height can trigger layout. [SC]
- [MDN `will-change`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/will-change)
  warns that persistent or excessive layer promotion consumes resources and
  should be used sparingly. [SC]

The pinned upstream repository has no root `LICENSE`, `LICENCE`, `COPYING`, or
`NOTICE` file in its tracked tree. Its authoritative legal text is the
[Terms & License page](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/terms.html)
(page says “Last updated July 2026”, checked 2026-08-31). It grants use of
free or Pro snippets in unlimited personal and commercial projects, permits
modification and shipping to product users, and prohibits repackaging,
reselling, or publishing the collection or a substantial part of it as a
competing library, template pack, or component kit. The [CLI README](https://github.com/Jakubantalik/transitions.dev/blob/0f8540f9a7c200211d4bcc149d787382fa4567fa/cli/README.md)
and [`cli/package.json`](https://github.com/Jakubantalik/transitions.dev/blob/0f8540f9a7c200211d4bcc149d787382fa4567fa/cli/package.json)
identify the CLI package as MIT while stating that installed transitions are
covered by the Transitions.dev licence. [VF]

**Licence conclusion [WI]:** this dossier may analyse public behaviour and
cite the sources. It does not copy snippets, generated CSS, Pro source, or a
substantial collection into Poodle. Any future source reuse needs an explicit
licence review; the CLI's MIT label is not evidence that the recipes are MIT.

## Current Poodle audit

### Contract and token state

| Area | Current evidence | Research consequence |
|---|---|---|
| Canonical primitive motion tokens | [`packages/tokens/schema/primitives/motion.json:1-27`](../../../packages/tokens/schema/primitives/motion.json) defines `fast=120ms`, `standard=180ms`, `slow=260ms`, `easing.standard=cubic-bezier(0.2,0,0,1)`, and `easing.emphasized=cubic-bezier(0.16,1,0.3,1)`. | Poodle already has a compact baseline. Do not import the upstream scale wholesale. [VF/WI] |
| Semantic motion tokens | [`packages/tokens/schema/semantic/motion.json:1-23`](../../../packages/tokens/schema/semantic/motion.json) aliases `duration.interaction` to standard and `duration.overlay` to fast, with semantic easing aliases. | There is no semantic duration for enter/exit/loading/status, no distance/blur/stagger role, and no preference policy. [VF] |
| Generated token surfaces | CSS: [`packages/tokens/artifacts/css/poodle-tokens.css:57-61,115-118`](../../../packages/tokens/artifacts/css/poodle-tokens.css) and [`packages/core/src/tokens/generated/css/poodle-tokens.css:57-61,115-118`](../../../packages/core/src/tokens/generated/css/poodle-tokens.css); TS: [`packages/tokens/artifacts/ts/index.ts:399-403,457-460,552-555`](../../../packages/tokens/artifacts/ts/index.ts); Rust: [`packages/tokens/artifacts/rust/semantic.rs:31-34`](../../../packages/tokens/artifacts/rust/semantic.rs). | These are generated surfaces, not research edit targets. A promoted token change must use the documented generator and drift checks. [VF] |
| Contract authority | [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md) requires one observable contract across Svelte, React, shared Rust composition, and GPUI; exact platform mechanisms can differ, but semantics, states, behaviour, accessibility, and token usage must align. [`docs/architecture/001-poodle-system-shape.md`](../../architecture/001-poodle-system-shape.md) and [`docs/architecture/006-headless-core-and-machine-model.md`](../../architecture/006-headless-core-and-machine-model.md) put shared behaviour in core and native output in `poodle-render`/`poodle-node`. | A transition is promotable only as a semantic state/lifecycle rule with runtime renderings, not as a Svelte recipe. Jetstream remains deferred per the working rules. [VF] |
| Dialog | [`docs/contracts/components/dialog.md`](../../contracts/components/dialog.md) explicitly has closed/open states, full unmount, and **no intermediate opening/closing animation states**. | The site modal recipe is not a current Dialog match. Adding it would be a contract decision, not a style tweak. [VF/WI] |
| Popover | [`docs/contracts/components/popover.md`](../../contracts/components/popover.md) keeps the surface unmounted when closed and says presence animation is a possible future concern. | The menu/dropdown recipe is a strong research input for a future presence contract, not current permission to add intermediate states. [VF/WI] |
| Drawer | [`docs/contracts/components/drawer.md:78-129`](../../contracts/components/drawer.md) documents closed/open behaviour and says a small closed/opening/open/closing machine is appropriate, while its shared modal machine remains closed/open. | Drawer is a candidate for explicit motion semantics, but focus, modality, body scroll lock, and cancellation must be resolved together. [VF/WI] |

### Web implementation and runtime audit

| Runtime/surface | Current behaviour | Exact local evidence and gap |
|---|---|---|
| Shared web disclosure styles | Indicator transforms use the interaction token in `Accordion` and `Collapsible`; panel presence is not a shared CSS contract. | [`packages/core/src/styles/accordion.css:72-82`](../../../packages/core/src/styles/accordion.css), [`packages/core/src/styles/collapsible.css:74-84`](../../../packages/core/src/styles/collapsible.css). [VF] |
| Shared web loading styles | `Skeleton` shimmers at 1.6s linear; `Spinner` has ring/dots/grid loops; `Progress` has a 180ms determinate transform and a 1.2s indeterminate loop. | [`packages/core/src/styles/skeleton.css:26-39`](../../../packages/core/src/styles/skeleton.css), [`packages/core/src/styles/spinner.css:62-105`](../../../packages/core/src/styles/spinner.css), [`packages/core/src/styles/progress.css:23-46`](../../../packages/core/src/styles/progress.css). These core styles have no local `prefers-reduced-motion` guard. [VF] |
| Svelte | `Accordion` and `Collapsible` use Svelte `slide` at 180ms; `Drawer` uses a 200ms backdrop fade and edge slide. Dialog, Popover, ToastHost, Tooltip, and Tabs use structural/timer behaviour without corresponding visual presence/indicator motion. | [`packages/svelte/components/src/Accordion.svelte:9,126`](../../../packages/svelte/components/src/Accordion.svelte), [`packages/svelte/components/src/Collapsible.svelte:9,120-126`](../../../packages/svelte/components/src/Collapsible.svelte), [`packages/svelte/components/src/Drawer.svelte:14-16,60-61,84-99,122-130,199-220`](../../../packages/svelte/components/src/Drawer.svelte), plus [`Dialog.svelte`](../../../packages/svelte/components/src/Dialog.svelte), [`Popover.svelte`](../../../packages/svelte/components/src/Popover.svelte), [`ToastHost.svelte`](../../../packages/svelte/components/src/ToastHost.svelte), [`Tooltip.svelte`](../../../packages/svelte/components/src/Tooltip.svelte), and [`Tabs.svelte`](../../../packages/svelte/components/src/Tabs.svelte). The Svelte transition path calls `element.animate`; the local [`PAPERCUTS.md`](../../../PAPERCUTS.md) records the happy-dom limitation. [VF] |
| React | `Dialog`, `Popover`, `Drawer`, `Accordion`, and `Collapsible` conditionally render/unmount with no visual presence transition. Toast timers and Tooltip/Tabs interaction behaviour exist, but no shared motion layer is attached. | [`packages/react/components/src/Dialog.tsx:107-226`](../../../packages/react/components/src/Dialog.tsx), [`Popover.tsx:98-197`](../../../packages/react/components/src/Popover.tsx), [`Drawer.tsx:70-123`](../../../packages/react/components/src/Drawer.tsx), [`Collapsible.tsx:48-94`](../../../packages/react/components/src/Collapsible.tsx), [`ToastHost.tsx:43-65`](../../../packages/react/components/src/ToastHost.tsx). React imports the shared core CSS, so the Svelte-only panel transitions are a parity gap. [VF/WI] |
| Reduced motion | Only `packages/core/src/styles/split-view.css:140-143` and `packages/core/src/styles/agent-message.css:125-128` contain product `prefers-reduced-motion` rules found by the audit. | Dialog/popover/drawer/disclosure/toast/tooltip/loading/tab component styles do not yet have one unified preference policy. [VF] |
| Shared native node contract | [`packages/contracts/node/src/lib.rs:46-70`](../../../packages/contracts/node/src/lib.rs) gives nodes stable identity used for animation clocks; [`:327-328`](../../../packages/contracts/node/src/lib.rs) exposes `NodeStyle.animation`. [`:796-865`](../../../packages/contracts/node/src/lib.rs) supports opacity, rotate, translate X/Y, scale X/Y; Linear/EaseIn/EaseOut/EaseInOut; Once/Loop/PingPong. | Stable keys are a good basis for interruption and immediate-mode rebuilds. Width/height, blur, stroke draw, arbitrary cubic curves, and a first-class reduced-motion policy are not in this vocabulary. [VF/WI] |
| Native render | Native `ToastStack` enters with fade + 0.5rem rise for 0.18s; `Skeleton` pulses at 1.6s when animated; `Spinner` uses 0.8s ring and 1.2s grid loops. Other audited Dialog, Popover, Drawer, disclosure, Tooltip, Progress, and Tabs renderers have no animation declarations. | Native motion is already shared before GPUI, but it does not yet mirror web presence/indicator behaviour. [`packages/render/src/toast_stack.rs:79-102,287-288`](../../../packages/render/src/toast_stack.rs), [`skeleton.rs:56-73,162-167,258`](../../../packages/render/src/skeleton.rs), [`spinner.rs:37-44,119-141`](../../../packages/render/src/spinner.rs). [VF] |
| GPUI and visual harness | GPUI capture clears node animation declarations because stock GPUI 0.2.2 lacks the fork-only reduce-motion API. Web and native visual harnesses zero timing or skip animated/media fixtures for determinism. | [`packages/gpui/preview/src/bin/window_capture/fixture_capture.rs:510-523`](../../../packages/gpui/preview/src/bin/window_capture/fixture_capture.rs), [`test/visual/capture.ts:14-23`](../../../test/visual/capture.ts), [`test/visual/config.ts:35-49`](../../../test/visual/config.ts), [`test/native-visual/config.ts:83-103`](../../../test/native-visual/config.ts). Deterministic capture is not a substitute for behavioural reduced-motion tests. [VF/WI] |
| Native accessibility | [`docs/contracts/003-native-accessibility.md`](../../contracts/003-native-accessibility.md) records that GPUI 0.2.2 has no accessibility API while Jetstream/AccessKit is deferred. | A motion pilot must preserve semantic state and interaction reachability without claiming uniform native assistive-technology parity that the current adapter cannot provide. [VF/WI] |

## Detailed findings

### 1. The site is a useful pattern source, not a Poodle taxonomy

The free skill groups transitions by visible UI target and tells the user to
choose the closest element first, use the lower-overhead option when there is
a tie, and reveal without guessing when there is no clear match. It also
publishes reusable variables and a reduced-motion guard rather than a runtime
contract. See [`skills/transitions-dev/SKILL.md`](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/SKILL.md).
That is a good pattern-selection heuristic. [SC]

The cards mix semantic state feedback (`Checkbox check`, `Error state shake`,
`Tooltip open/close`) with pointer decoration (`3D tilt`, `Like button`,
`Confetti burst`), marketing (`Get Pro button`), implementation-specific AI
presentation (`Reasoning stream`), and image/canvas effects. [VF]

**Poodle inference [WI]:** classify by semantic state and component ownership,
not by visual resemblance. A transition belongs in Poodle only when the
observable state change is reusable, accessible, and meaningful across the
active Svelte/React/shared-Rust/GPUI cohort.

### 2. Upstream motion scale versus Poodle scale

The upstream free skill defines these shared roles:

| Role | Upstream value | Observed use |
|---|---:|---|
| `--duration-stagger` | 40ms | Per-item text/stack staggering |
| `--duration-micro` | 80ms | Path delay, shake segment, large stagger |
| `--duration-quick` | 150ms | Close, text swap, tooltip appearance |
| `--duration-fast` | 250ms | Open, icon swap, tabs, page slide |
| `--duration-medium` | 350ms | Panel/toast close or toast open family |
| `--duration-slow` | 400ms | Panel open, skeleton reveal, clear phase |
| `--duration-very-slow` | 500ms | Success, badge, text reveal emphasis |

It also uses `smooth-out=cubic-bezier(0.22,1,0.36,1)`, `ease-in-out`,
`ease-out`, `linear`, a light bounce
`cubic-bezier(0.34,1.36,0.64,1)`, and a strong avatar-return bounce
`cubic-bezier(0.34,3.85,0.64,1)`. Distances cluster at 4, 6, 8, 12, and
30px; scales cluster at 0.96, 0.97, 0.98, and 0.99; blur is usually 2–3px,
with larger success emphasis. These values are verified in the upstream
[`SKILL.md`](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/SKILL.md)
and [`_root.css`](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/_root.css).

Poodle's current 120/180/260ms baseline overlaps the useful short range, but
the systems are not interchangeable. The upstream recipes also use 1–2s
loops, 1.4s reel motion, 2s text shimmer, 2s thinking holds, 840ms reasoning
steps, and custom per-card values. [VF]

**Recommendation [WI]:** keep Poodle's existing token names as the source of
truth. If the pilot proves a gap, add semantic roles such as disclosure enter,
transient enter/exit, status update, and loop policy in the canonical token
schema. Do not expose upstream variable names or adopt a decorative bounce
scale merely because it is present in the catalogue.

### 3. Choreography principles worth promoting

The catalogue yields a small set of reusable principles [WI], grounded in the
recipe sources:

- Commit the semantic state immediately; animation communicates the change but
  must not be the source of truth for `aria-expanded`, `aria-selected`,
  `aria-checked`, status text, or toast content.
- Keep the trigger or anchor stable when the surface opens. The badge leaves
  the bell still; menus use an origin; tabs measure the selected indicator;
  disclosure uses a clipped inner panel. [VF]
- Enter can be slightly longer than exit for transient surfaces: the recipes
  use menu/modal 250/150ms, toast 350/250ms, and panel 400/350ms. This is a
  candidate heuristic, not a Poodle law. [VF]
- Prefer short transform/opacity changes. Treat width/height, blur, filters,
  gradient paint, canvas, and per-frame pointer work as explicit costed
  mechanisms, not default choreography. [WI]
- Use stagger only to explain order or hierarchy. A 40ms text stagger can
  support reading order; an indefinite shimmer or fan-out needs a semantic
  reason or should remain product-owned. [WI]
- Keep focus, dismissal, live-region announcement, and final semantic content
  independent of visual completion. `Dialog`, `ToastStack`, `Tooltip`, and
  form validation contracts make that separation particularly important. [WI]

### 4. Reduced motion is a lifecycle policy, not a CSS afterthought

All 32 free references include a `prefers-reduced-motion: reduce` rule, but the
behaviour is not uniform: some rules remove transitions, some force the final
state, some hide shimmer, and some only alter the visual layer. Pro source is
not published in the repository, so its portable reduced-motion behaviour
cannot be audited from source. [VF]

Poodle currently has no cross-runtime policy that reaches core web styles,
Svelte/React orchestration, `RenderContext`, and GPUI. [VF] The pilot should
define at least:

1. **Full mode:** the component may use the contracted visual phase.
2. **Reduced mode:** remove non-essential travel, blur, bounce, and looping;
   preserve the final state and any essential progress or status information.
3. **No-motion/frozen test mode:** deterministic capture may freeze clocks, but
   it must not be mistaken for user preference behaviour.

The policy needs to cover JS timers, `requestAnimationFrame`, Svelte WAAPI
transitions, CSS loops, and native `NodeAnimation`. Checking only the CSS media
query would leave `input-clear-with-dissolve`, text replay, status intervals,
and canvas work running. [WI]

### 5. Interruption and cancellation are the main adoption risk

CSS transitions can retarget a value when a state reverses, but removal from
rendering cancels the transition. The upstream recipes add timers, forced
reflow, DOM rebuilding, `requestAnimationFrame`, pointer capture, and delayed
unmount in different combinations. [VF]

The promoted contract should require every non-trivial phase to specify:

- stable identity/key across immediate-mode rebuilds;
- what happens on reverse, retarget, repeated trigger, `pointercancel`, abort,
  unmount, and host-owned state replacement;
- cancellation of timers, rAF loops, WAAPI animations, pointer capture, and
  delayed cleanup;
- whether content remains mounted until exit completes or disappears
  immediately; and
- the final semantic and visual state after cancellation.

The highest-risk free recipes are `text-states-swap`, `menu-dropdown`,
`modal-open-close`, `success-check`, `error-state-shake`,
`input-clear-with-dissolve`, `skeleton-loader-and-reveal`, `tabs-sliding`,
`texts-reveal`, `thinking-states`, `streaming-text`, and `banner-stacking`.
They all need orchestration or cleanup beyond a single reversible CSS rule.
The native node key facility is a useful starting point, not proof that web
and native cancellation already match. [WI]

### 6. Performance and rendering mechanism

The catalogue's cheapest patterns are same-slot opacity/transform changes and
simple CSS transitions. Its costlier patterns include layout geometry
(`card-resize`), filters/blur (`panel-reveal`, `skeleton-loader-and-reveal`,
`input-clear-with-dissolve`), gradient text, per-frame pointer updates,
multi-element reels, SVG filters, and Pro canvas pixel effects. [VF]

For Poodle, transform/opacity should be the default primitive surface. Width /
height, blur, path drawing, gradient paint, and canvas should require an
explicit component-level rationale and a reduced/performance fallback. This
follows the cited web.dev and MDN guidance and the fact that native
`NodeAnimation` cannot currently represent most of those properties. [WI]

### 7. Accessibility and semantic fit

- `Tooltip open/close`, `Tabs sliding`, `Checkbox check`, `Toggle`, and
  `Accordion` have direct semantic state anchors. Motion should supplement
  their ARIA state, never delay it.
- `Toast open/close` and `Banner stacking` must preserve live-region posture,
  action/dismiss reachability, and no focus theft. The current `ToastStack`
  contract already says transient notifications announce without stealing
  focus. [`docs/contracts/components/toast-stack.md:111-138`](../../contracts/components/toast-stack.md)
  [VF]
- `Error state shake` can be a useful optional validation cue, but shake must
  not be the only error signal and should be disabled/replaced in reduced mode.
  Large or repeated movement has a vestibular risk under the W3C/MDN guidance.
  [WI]
- `Learn more hover`, `Avatar group hover`, `3D tilt`, `Like button` particles,
  confetti, gradient text, and promotional effects are not reliable keyboard
  or assistive-technology communication. They are not generic Poodle motion
  contracts. [WI]
- `Reasoning stream` must not be used to expose private chain-of-thought. Its
  visual transcript loop is semantically mismatched to a generalized UI
  primitive even if an application has a user-visible progress log. [WI]

## Exhaustive catalogue audit

The rows use the current rendered DOM/display order. The row number is this
dossier's ordinal; it is not the site's `data-seq` value because the current
DOM inserts Pro cards with sequence values 28–43 between the original free
cards. Category values are copied from the captured `data-cat` attribute; `Pro` is copied from the card/data record. Free
mechanism and reduced-motion details are from the corresponding pinned public
reference. Pro rows cite the public detail page and pinned card/data, but the
portable source is intentionally absent per the upstream build script.

| # | Name | Category / tier | Observable behaviour | Intended interaction or state change | Mechanism and prerequisites | Reduced motion | Interruption / cancellation concern | Direct source |
|---:|---|---|---|---|---|---|---|---|
| 1 | Card resize | `essential` / free | Container width or height smoothly changes. | Compact ↔ expanded card or row layout. | CSS `width`/`height` transition; state/class toggle; explicit dimensions; `will-change` is used in the recipe. | Free reference disables the transition. | CSS can retarget geometry; layout work remains; unmount removes the effect. | [free ref 01](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/01-card-resize.md) |
| 2 | Number pop-in | `texts` / free | Each digit enters with blur, slide, opacity, and stagger. | Updated counter, price, balance, or metric. | Per-digit spans, CSS keyframes, DOM replacement, forced reflow to replay. | Guard leaves the final text without motion. | Rebuild/replay must cancel stale target text and pending sequencing; per-digit DOM is required. | [free ref 02](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/02-number-pop-in.md) |
| 3 | Notification badge | `essential` / free | Badge slides diagonally onto a still trigger; dot pops independently. | New notification/count on a bell, inbox, or button. | Absolutely positioned badge under a `position: relative` trigger; CSS transform/opacity/scale; state attribute. | Guard removes slide/pop motion. | Repeated count changes must not leave stale badge state; CSS reversal is straightforward. | [free ref 03](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/03-notification-badge.md) |
| 4 | Text states swap | `essential` / free | Old text exits upward blurred; new text enters from below. | `Processing…` → `Done`, or `Save` → `Saved`. | JS swaps `textContent` after a delay, inserts an enter state, forces reflow, then releases CSS transition. | Guard makes the swap immediate. | Timer must be cancelled or made stale-safe when a new state arrives; React sample guards busy state. | [free ref 04](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/04-text-states-swap.md) |
| 5 | Menu dropdown | `essential` / free | Anchored surface grows from its origin with opacity/scale; close is shorter. | Closed ↔ open menu, dropdown, or popover. | Positioned surface, `data-origin`, CSS state classes, `.is-closing`, delayed unmount/cleanup. | Guard removes transition; state still opens/closes. | Clear close timeout when reopened; coordinate focus, dismissal, and surface presence. | [free ref 05](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/05-menu-dropdown.md) |
| 6 | Confetti burst | `effects` / Pro | Canvas particles burst, fall, and disappear. | Celebrate a completed action. | Canvas, `requestAnimationFrame`, particle physics, decorative layer. Pro portable source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Stop rAF and clear canvas on completion, abort, reduced mode, and unmount. | [site detail](https://transitions.dev/transitions/confetti-burst/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 7 | Modal open/close | `essential` / free | Centered surface scales/fades in and scales/fades out. | Closed ↔ modal overlay open. | CSS opacity/transform with open/closing classes and delayed cleanup; overlay, focus, and ARIA are external prerequisites. | Guard removes transition. | Clear close timeout; avoid removing focus trap/backdrop at the wrong phase. Poodle Dialog currently prohibits intermediate phases. | [free ref 06](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/06-modal.md) |
| 8 | Panel reveal | `essential` / free | Panel translates, fades, and cross-blurs into a region. | Closed ↔ inline detail/expansion panel. | CSS transform/opacity/filter; optional clipping container; state attribute. | Guard removes visual transition. | Reversal can retarget CSS; conditional unmount cuts the exit and may lose focus. | [free ref 07](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/07-panel-reveal.md) |
| 9 | Gooey plus menu | `effects` / Pro | Circular plus trigger morphs into a gooey action fan/panel. | Closed action launcher ↔ expanded actions. | SVG goo filter, positioned buttons, opacity/blur/rotation/scale, JS state. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Synchronise trigger, action hit areas, filter, and panel state when toggled rapidly. | [site detail](https://transitions.dev/transitions/gooey-plus-menu/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 10 | Page side-by-side | `essential` / free | Outgoing and incoming pages slide horizontally with opacity/filter. | List ↔ detail or step 1 ↔ step 2 navigation. | Stacked/fixed page regions, CSS transforms/opacity/filter, navigation state outside the recipe. | Guard removes transition. | Navigation, URL, focus, scroll restoration, and rapid route replacement need cancellation semantics. | [free ref 08](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/08-page-side-by-side.md) |
| 11 | Icon swap | `essential` / free | Two same-slot icons cross-fade with blur and scale. | Binary icon state: menu/close, sun/moon, play/pause, expand/collapse. | Both icons remain stacked in one grid cell; CSS opacity/filter/scale; equal slot geometry. | Guard removes transition. | CSS can retarget rapid state changes; accessible label/hiding must not depend on visual opacity. | [free ref 09](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/09-icon-swap.md) |
| 12 | Success check | `essential` / free | Fade, rotate, Y-settle, and optional SVG stroke draw. | Pending/unknown ↔ completed/success. | JS measures `getTotalLength()`, resets dash state, forces reflow, and starts CSS animation; success semantics are external. | Guard forces final visible state and completed stroke. | Replay needs reset; path/timer cleanup and persistent success visibility must be explicit. | [free ref 10](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/10-success-check.md) |
| 13 | Avatar group hover | `effects` / free | Hovered avatar lifts/scales; neighbours offset by distance and spring back. | Pointer hover inspection of an avatar group. | Sibling indexing, pointer hover, inline custom properties, transform timing; recipe intentionally leaves keyboard/touch resting. | Guard disables transform motion. | Recompute on pointer leave/re-entry; pointer and touch semantics cannot make hover motion the only cue. | [free ref 11](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/11-avatar-group-hover.md) |
| 14 | Card stack hover | `effects` / Pro | Stacked cards fan out, rotate, scale, and spring on hover/open. | Pointer hover or explicit open of a card stack. | Keyed cards, CSS custom positions/rotations/transforms, hover or open state. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Pointer exit/re-entry and reduced mode need a deterministic collapsed state. | [site detail](https://transitions.dev/transitions/card-stack-hover/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 15 | Error state shake | `essential` / free | Input border enters a short shake and validation message reveals. | Invalid submission or field validation failure. | Separate error/shaking classes, CSS keyframes, message reveal, optional hold timer/typing cancellation. | Guard removes transform animation; error state/message remain. | Clear/restart the hold timer; repeated validation can replay without stale revert. Never make shaking the only error cue. | [free ref 12](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/12-error-state-shake.md) |
| 16 | Input clear with dissolve | `effects` / free | Mirrored text flies/blurs away, placeholder enters, and a radial glow dissolves. | Clear an input while retaining focus. | Duplicated mirror/placeholder/glow, pointer handling, JS `requestAnimationFrame`, gradients/filter. | CSS guard hides glow, but JS must also skip animation work to fully honour reduce. | Cancel rAF and pending phases on rapid clear, replacement, abort, and unmount; high paint/CPU cost. | [free ref 13](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/13-input-clear-dissolve.md) |
| 17 | Skeleton loader and reveal | `essential` / free | Skeleton pulses, then real content cross-fades/de-blurs into the same slot. | Loading ↔ content available. | Layered skeleton/content, CSS pulse/reveal, JS state/reset/reflow/timeout; stable geometry. | Guard disables pulse/transition and leaves final content. | Cancel reveal/reset timers; unmount must not reveal stale content. | [free ref 14](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/14-skeleton-reveal.md) |
| 18 | Texts reveal | `texts` / free | Multiple lines enter with translate, blur, opacity, and 40ms stagger. | Content block entrance. | Stable line elements, class/reflow, delayed JS sequencing. | Guard makes lines immediately visible. | Cancel delayed line work on hide/replay; stable order and final visibility matter. | [free ref 18](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/18-texts-reveal.md) |
| 19 | Tabs sliding | `essential` / free | Selected indicator slides and resizes to the active tab. | `aria-selected` tab change. | JS reads `offsetLeft`/`offsetWidth`, writes transform/width; first position and resize skip transition; tablist semantics required. | Guard makes indicator jump to measured final position. | Rapid clicks/resize can race measurements; state and indicator identity must stay aligned. | [free ref 16](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/16-tabs-sliding.md) |
| 20 | Drag & drop with physics | `effects` / Pro | Draggable image translates/rotates/scales, lands, fades, and may displace/smoke. | Pointer drag, target hover, drop, return, or reject. | Pointer events, `touch-action`, CSS transforms, SVG displacement, JS drag state. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Handle pointer cancellation, failed drop, unmount, and retarget; physics must not obscure drop semantics. | [site detail](https://transitions.dev/transitions/drag-drop-with-physics/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 21 | Shimmer text | `texts ai` / free | Duplicated text has an infinite gradient sweep. | Waiting/active streaming or status emphasis. | Duplicate `data-text`, gradient `background-clip:text`, 2s linear keyframe. | Guard hides/stops the shimmer. | Pause loop on unmount/state completion; duplicate text must not be announced twice. | [free ref 15](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/15-shimmer-text.md) |
| 22 | Organic shimmer | `effects ai` / Pro | Wavy gradient shimmer with edge glow and blur. | Active/playing visual status. | Multiple gradients, blur/filter, animated background layers, play state. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Stop animation on completion/reduced/unmount; high paint cost and no general semantic need. | [site detail](https://transitions.dev/transitions/organic-shimmer/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 23 | Tooltip open/close | `essential` / free | Tooltip fades/scales in after a short delay and exits quickly. | Hover/focus target ↔ explanatory tooltip. | Wrapper hover/focus, `aria-describedby`, `role=tooltip`, CSS opacity/scale; no JS timer in the recipe. | Guard removes transition; delay/semantics remain. | CSS retargets; focus/hover leave and hit-area semantics matter more than the effect. | [free ref 17](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/17-tooltip.md) |
| 24 | 3D tilt | `effects` / free | Card rotates in 3D with pointer-tracked glare and resets on leave. | Pointer inspection/hover affordance. | Outer pointer tracker, inner card transform, custom properties, perspective, pointer capture/cancel. | Guard flattens/removes motion. | Handle pointerup/cancel/leave and touch; no semantic state may depend on tilt. | [free ref 19](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/19-card-tilt.md) |
| 25 | Dropdown menu morph | `essential` / free | Circular trigger grows into a rounded panel; plus rotates and content cross-fades/slides. | Trigger ↔ expanded action menu. | Fixed open dimensions, overflow-hidden anchor, CSS radius/size/opacity/transform, JS state and `aria-expanded`. | Guard disables transitions. | Hardcoded dimensions and timer/state synchronisation make rapid toggle and responsive sizing risky. | [free ref 20](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/20-plus-menu-morph.md) |
| 26 | Accordion | `essential` / free | Panel opens via grid row 0fr↔1fr; chevron rotates. | Disclosure trigger `aria-expanded=false/true`. | Two-element panel with overflow-hidden inner; CSS grid/transform; no height measurement. | Guard removes transition. | CSS reverses naturally; conditional unmount would cut exit; trigger/panel ownership must persist. | [free ref 21](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/21-accordion.md) |
| 27 | Toast open/close | `essential` / free | Toast rises with opacity/scale/blur and exits faster. | Notification enters/leaves a transient stack. | CSS state, transient surface, host-owned auto-dismiss/action/dismiss semantics. | Guard removes transition. | Queue identity, remove-before-exit, timer ownership, focus, and live-region behaviour need a contract. | [free ref 22](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/22-toast.md) |
| 28 | Like button | `effects` / free | Heart fills, overshoots, and emits particles on like. | Boolean like/favourite action. | CSS fill/pop plus JS particle elements and burst class/timer. | Guard disables burst/pop. | Retoggle and unmount must clear particles/timers; particles are decorative and not the state signal. | [free ref 23](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/23-like-button.md) |
| 29 | Image open tilt | `effects` / Pro | Image zooms, bends/tilts in 3D, and can use a displacement warp. | Image closed ↔ opened/focused. | Image plus canvas/software warp, perspective, border-radius and CSS transforms. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Cancel pixel work and restore image/canvas on close, abort, unmount; image-specific and expensive. | [site detail](https://transitions.dev/transitions/image-open-tilt/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 30 | Learn more hover | `essential` / free | Chevron translates slightly; SVG arms spread on hover. | Pointer hover affordance for a link/action. | Pure CSS SVG transforms; hover-only source; inline link/button prerequisite. | Guard removes movement. | Reversible CSS; keyboard and touch remain at rest, so it cannot carry meaning alone. | [free ref 24](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/24-learn-more-hover.md) |
| 31 | Checkbox check | `essential` / free | Box fills, then check path draws; uncheck reverses. | `aria-checked=false` ↔ `true`. | Checkbox semantics plus SVG path dash offset and CSS transitions; path length calibration. | Guard jumps to the checked/final state. | CSS reverses mid-draw; accessible value must update immediately and path length must not pre-reveal. | [free ref 25](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/25-checkbox-check.md) |
| 32 | Spinner to check morph | `ai` / Pro | Spinner ring crosses into a completed check with blur/scale/bob. | Processing ↔ success. | Ring/check SVG, state classes, stroke/opacity/blur/scale choreography. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | State replacement must cancel loop and settle success; composition of Spinner and success status needs ownership. | [site detail](https://transitions.dev/transitions/spinner-to-check-morph/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 33 | Spinning counter | `texts` / free | Reel of digits spins through several turns with per-column stagger and blur. | Numeric value update with emphasis. | JS-built digit strips, measured cell height, 1.4s transforms, optional SVG blur. | Guard stops motion/filter and shows final value. | Retarget/cancel long reel and stale target; many nodes/filter work make it a poor default. | [free ref 26](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/26-spinning-counter.md) |
| 34 | Toggle | `essential` / free | Switch thumb travels with a double-bounce; track colours cross-fade. | Switch off ↔ on. | `role=switch`/checked state, data attribute, transform keyframe, initialisation gate. | Guard removes keyframe motion. | Rapid toggles can restart animation against host state; initial mount must not animate from a false default. | [free ref 27](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/27-toggle.md) |
| 35 | Pro gradient text | `texts ai` / Pro | Text continuously shifts through animated gradients/hue. | Decorative emphasis or AI/product branding. | Inline SVG/radial gradients, background clip, hue/background-position loops. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Stop loop on unmount/reduced; no semantic state is carried by the gradient. | [site detail](https://transitions.dev/transitions/pro-gradient-text/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 36 | Delete with smoky dissolve | `effects` / Pro | Image dissolves through smoke/displacement and scale/opacity. | Delete/remove or respawn an image. | Canvas pixel processing plus CSS opacity/scale/filter; image/delete state. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Abort pixel work and restore/remove deterministically; destructive visual is not a generic deletion contract. | [site detail](https://transitions.dev/transitions/delete-with-smoky-dissolve/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 37 | Thinking states | `ai` / free | Status text holds, swaps vertically, and shimmers while working. | Working/waiting state sequence in an agent or job. | `role=status`, hidden width sizer, interval/timers, text swap, CSS shimmer. | Stops transition and shimmer while still updating status text. | Cancel interval/timers on job completion, pause, replacement, and unmount; status announcements need cadence control. | [free ref 28](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/28-thinking-states.md) |
| 38 | Reasoning stream | `ai` / free | Two-line transcript scrolls upward through a clipped/masked viewport. | Animated reasoning/progress transcript. | Fixed line-height, cloned text, mask fade, JS loop and 500ms transform. | Guard stops transform and leaves the current/final view. | Cancel loop on pause/unmount; never use this to expose private chain-of-thought. | [free ref 29](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/29-reasoning-stream.md) |
| 39 | Streaming text | `ai` / free | Incoming words fade/blur into a growing text stream at 60ms gaps. | Incremental output while content is delivered. | JS wraps words in spans, clears/replays, and schedules per-word transitions. | Guard makes all words immediately visible. | Abort/unmount must cancel pending word timers; text segmentation and duplicate-span accessibility need care. | [free ref 30](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-dev/30-streaming-text.md) |
| 40 | Matrix dot loader | `ai` / free | 4×4 dots pulse with phase/delay patterns. | Waiting/loading indicator. | JS creates 16 dots; CSS delay table and perpetual pulse; decorative geometry. | Guard stops animation. | Pause/clear loop on unmount and completion; many moving dots add cost without more status meaning than Spinner. | [free ref 31](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/31-matrix-loader.md) |
| 41 | Banner stacking | `essential` / free | Sonner-style stack rises, layers depth/scale/opacity, ejects the fourth, and spreads on hover. | Queue of transient banners/notifications. | Stable IDs, JS depth/reflow/timeout, CSS transforms/blur/scale, max-three queue. | Guard removes transition/transform/filter. | Queue removal, close timers, hover geometry, pointer cancellation, and externally owned item state must agree. | [free ref 32](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/skills/transitions-dev/32-banner-stacking.md) |
| 42 | Image generation placeholder | `ai effects` / Pro | Dotted placeholder pulses and image cross-fades/de-blurs in. | Image generation/loading ↔ image available. | Placeholder dots, image layers, CSS opacity/blur, state script. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Abort stale image load/animation and preserve alt/status semantics; image generation is app-specific. | [site detail](https://transitions.dev/transitions/image-generation-placeholder/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |
| 43 | Get Pro button | `effects` / Pro | CTA text has looping animated gradient/SVG background. | Promotional purchase/upgrade action. | Gradient/SVG background, hue and position loops. Pro source is unavailable. | Demo-level behaviour is not a portable contract; audit source is unavailable. | Stop loop on unmount/reduced; promotional styling is product-owned, not a primitive. | [site detail](https://transitions.dev/transitions/get-pro-button/), [pinned data](https://raw.githubusercontent.com/Jakubantalik/transitions.dev/0f8540f9a7c200211d4bcc149d787382fa4567fa/scripts/transitions-data.json) |

## Catalogue-to-Poodle matrix

Disposition is deliberately stricter than visual similarity:

- **Semantic match [WI]** — an existing Poodle semantic component can own the
  state and the motion could be a shared contract.
- **Possible match [WI]** — the state is reusable, but the current component
  contract, ownership, or runtime support is insufficient.
- **Explicit non-match [WI]** — decorative, marketing, private/app-specific,
  image-specific, or contradicted by a current Poodle contract.

`P0` means representative pilot; `P1` means a likely follow-on; `P2` means
defer for a future contract or product-owned implementation; `—` means reject
from the generalized Poodle motion catalogue.

| # / transition | Disposition | Candidate Poodle component(s) | Rationale and constraints | Runtime-parity implication | Priority |
|---:|---|---|---|---|---:|
| 1 Card resize | Possible match | `Card`; future layout-state primitive | Layout change is reusable, but width/height causes geometry work and the card contract does not currently define animated intermediate layout. | Web can transition geometry; native `NodeAnimation` cannot. Keep product-owned until a layout animation contract exists. | P2 |
| 2 Number pop-in | Possible match | Future value-change primitive; `MetricTile` if promoted | Value updates are meaningful, but per-digit DOM/reflow and long stagger are too specific for a base primitive. | React/Svelte can share DOM orchestration; native needs a text/value animation model absent today. | P2 |
| 3 Notification badge | Possible match | `StatusIndicator`; future `Badge` | Notification signal is semantic, but Poodle needs a documented badge/count contract and live-label rules. | Same-slot badge can be CSS/native opacity/scale; announce count independently. | P2 |
| 4 Text states swap | Semantic match | `StatusIndicator`, `AgentMessage`, status-bearing controls | State text swap is reusable when text is a status, not arbitrary copy. | Shared core should own stale-safe swap; Svelte/React/native must update semantic text immediately. | P1 |
| 5 Menu dropdown | Semantic match | `Menu`, `Popover`, `Select` | Direct overlay presence match, subject to focus, dismissal, origin, and delayed-unmount rules. | Current Popover/Menu surfaces are structural; add one shared presence contract or keep all runtimes instant. | P1 |
| 6 Confetti burst | Explicit non-match | None | Decorative celebration; not required to understand or complete an action. | Canvas has no shared native equivalent and adds reduced-motion/performance risk. | — |
| 7 Modal open/close | Explicit non-match | `Dialog` only if its contract is deliberately changed | Site visual is close, but current Dialog explicitly forbids intermediate opening/closing states and full-unmounts. | Do not create Svelte/React/native divergence by adding the recipe. | — |
| 8 Panel reveal | Possible match | `Drawer`, `Collapsible` | Meaningful reveal, but Drawer modality/focus and Collapsible panel ownership need separate contracts. | Svelte already has a divergent slide; React/native need a shared state/phase decision. | P1 |
| 9 Gooey plus menu | Explicit non-match | None; perhaps future `Menu` appearance recipe | Goo/filter is decorative and its fan geometry is not generalized menu semantics. | SVG filter/blur has no native NodeAnimation representation. | — |
| 10 Page side-by-side | Possible match | Future navigation transition; no current component | Navigation state is meaningful, but page/URL/focus/scroll lifecycle belongs to owning product shell. | No component-level cross-runtime target; keep outside Poodle until architecture assigns ownership. | P2 |
| 11 Icon swap | Semantic match | `IconButton`, `Switch`, disclosure indicators | Same-slot state change is common and can be a small shared primitive with accessible hiding rules. | Opacity/scale maps to web and native; avoid relying on blur/path morph. | P1 |
| 12 Success check | Possible match | `StatusIndicator`; future completion composite | Success feedback is semantic, but path drawing and celebration are optional appearance layers. | Core status must work without SVG path animation; native can use opacity/scale only. | P1 |
| 13 Avatar group hover | Explicit non-match | None; `Avatar` remains static | Pointer-only fan/lift is decorative, not a generalized avatar semantic. | Keyboard/touch and GPUI parity make it a poor contract. | — |
| 14 Card stack hover | Explicit non-match | None | Hover fan is decorative and the Pro source is unavailable; no Poodle card-stack semantic exists. | Pointer/3D/spring support is not shared in NodeAnimation. | — |
| 15 Error state shake | Semantic match | `TextInput`, `Field`, validation message | Validation failure is semantic. Shake must be optional, non-exclusive, reduced-safe, and not repeated aggressively. | Shared error state first; web/native may use a restrained translation or no motion. | P1 |
| 16 Input clear with dissolve | Explicit non-match | `TextInput` uses ordinary clear action | Clear is semantic, but mirror/glow/rAF/filter is decorative and expensive. | Keep clear/focus parity; no need to match dissolve across runtimes. | — |
| 17 Skeleton loader and reveal | Semantic match | `Skeleton`, content-loading composite | Loading→content and stable geometry are directly reusable; current `Skeleton` already owns a loop/static mode. | Add shared reduced policy and reveal semantics; native has pulse but web lacks guard. | P0 |
| 18 Texts reveal | Possible match | `AgentMessage`, `EmptyState`, future content entrance recipe | Entrance can be useful, but arbitrary page copy should not become a Poodle choreography. | Needs shared sequencing and cancellation; otherwise product-owned. | P2 |
| 19 Tabs sliding | Semantic match | `Tabs`, `SegmentedControl` | Selected indicator is a direct semantic state and current Tabs already measures geometry. | Define shared indicator phase; native needs a measured layout/translate representation. | P0 |
| 20 Drag & drop with physics | Possible match | `DragDrop` | Drag/drop semantics are in active [`docs/architecture/011-drag-and-drop-substrate.md`](../../architecture/011-drag-and-drop-substrate.md), but physics/smoke is decorative. | Contract can align pointer/keyboard/drop states; keep physics out until native gesture/render support is specified. | P2 |
| 21 Shimmer text | Possible match | `StatusIndicator`, `AgentMessage` | Streaming/working status can be semantic, but infinite shimmer should be optional and not duplicate announcements. | Web can gradient-shimmer; native should provide a static/opacity alternative. | P1 |
| 22 Organic shimmer | Explicit non-match | None | Decorative gradient/filter effect with no necessary state information and Pro source unavailable. | No shared native property model; high paint cost. | — |
| 23 Tooltip open/close | Semantic match | `Tooltip` | Direct component match; delay, focus/hover, hit area, and unmount rules matter more than effect. | Web/native adapters already differ for tooltip chrome; contract should define semantic timing, not blur. | P1 |
| 24 3D tilt | Explicit non-match | None | Pointer inspection/glare is decorative and not an accessible state change. | No cross-runtime pointer/3D/glare parity requirement. | — |
| 25 Dropdown menu morph | Possible match | `Menu`, `Popover` | Open action surface is meaningful, but morphing trigger into panel imposes fixed geometry and unusual focus ownership. | Use ordinary menu presence first; treat morph as product recipe if ever allowed. | P2 |
| 26 Accordion | Semantic match | `Accordion`, `Collapsible` | Direct disclosure match; grid-row approach avoids measurement and is browser-safe. | Strong pilot: Svelte panel slide, React structural, native structural currently diverge. | P0 |
| 27 Toast open/close | Semantic match | `ToastStack`, `ToastHost` | Direct transient-notification match; current contract already defines live-region and no-focus-theft behaviour. | Strong pilot: native has enter animation while web has none; define entry/exit/remove lifecycle once. | P0 |
| 28 Like button | Explicit non-match | None; `Button`/`IconButton` stays semantic | Like/favourite workflow is product-specific; particle burst is decorative. | State and accessible pressed value can be shared without burst. | — |
| 29 Image open tilt | Explicit non-match | None; future product image viewer owns it | Image warp/tilt is content-specific and canvas-heavy. | No native pixel-warp contract; avoid cross-runtime obligation. | — |
| 30 Learn more hover | Explicit non-match | `Link`/`Button` without required motion | Hover affordance is optional and keyboard/touch resting; not a meaningful state transition. | Keep focus styles and label semantics shared; motion can remain product-owned. | — |
| 31 Checkbox check | Semantic match | `Checkbox` | Direct boolean state; path draw is optional appearance, fill/final state is required. | Strong pilot: shared semantic update, web stroke/fill where available, native simpler opacity/scale fallback. | P0 |
| 32 Spinner to check morph | Possible match | `Spinner` + `StatusIndicator`; future loading-result composite | Processing→success is meaningful, but ownership between loading control and result status is undefined. | Define composite only after Spinner/status lifecycle; native can cross-fade rather than morph paths. | P2 |
| 33 Spinning counter | Possible match | Future value-change primitive; `MetricTile` | Metric update can be meaningful, but three-turn reel is emphasis and expensive. | Use short text/value update across runtimes; no requirement to reproduce reel/filter. | P2 |
| 34 Toggle | Semantic match | `Switch` | Direct switch state; current recipe's initialisation gate and ARIA model are useful. | Strong near-term match; native can use translate/colour without bounce. | P1 |
| 35 Pro gradient text | Explicit non-match | None | Branding/decorative loop; no semantic state and Pro source unavailable. | No cross-runtime need or native gradient animation contract. | — |
| 36 Delete with smoky dissolve | Explicit non-match | None; product-owned deletion feedback | Destructive image effect is app/content-specific and canvas-heavy. | Shared deletion semantics must not depend on pixel effect. | — |
| 37 Thinking states | Possible match | `StatusIndicator`, `AgentMessage` | User-visible working status can be generalized, but AI cadence/copy is product-owned. | Define status update and reduced policy; do not standardize text list or shimmer. | P1 |
| 38 Reasoning stream | Explicit non-match | None | Transcript loop is AI/app-specific and risks exposing private reasoning. | No Poodle semantic contract; use a product-owned progress log if appropriate. | — |
| 39 Streaming text | Possible match | `AgentMessage` | User-visible streaming is meaningful, but segmentation/timers belong to content delivery and accessibility. | Shared message semantics can render incremental content; optional visual reveal must not block/duplicate text. | P2 |
| 40 Matrix dot loader | Explicit non-match | `Spinner` is the semantic alternative | Decorative loader variant adds 16 moving elements without a stronger status contract. | Use existing Spinner's shared native/web loop and reduced policy. | — |
| 41 Banner stacking | Semantic match | `ToastStack`, `ToastHost` | Same transient queue semantics as Toast; depth/hover spread is an optional recipe. | Pilot with simple fade/translate stack first; native stable IDs already support keyed enter. | P0 |
| 42 Image generation placeholder | Possible match | `Skeleton` or image-loading composite | Loading placeholder is reusable, but generation-specific copy/state and image alt handling belong to product. | Reuse stable-geometry Skeleton contract; no Pro dots/blur requirement. | P2 |
| 43 Get Pro button | Explicit non-match | `Button` without animated branding | Marketing CTA and gradient loop are product-owned. | Button semantics/tokens remain shared; no motion parity obligation. | — |

## Recommended Poodle direction

### Promote intent and lifecycle, not named recipes

Create one cross-cutting motion contract only if the operator accepts the
following direction [WI]:

1. **Semantic intent.** Components select a small role such as disclosure,
   overlay presence, transient notification, selection indicator, state swap,
   status reveal, or loading loop. The role is not a public alias for an
   upstream recipe and does not expose arbitrary CSS snippets.
2. **Phase.** The contract distinguishes enter, exit, update, and loop. The
   semantic state and ARIA value update immediately; a visual phase is
   cancellable and cannot be required for correctness.
3. **Motion policy.** A host-level policy reaches web CSS/JS and native
   `RenderContext`/`NodeAnimation`: full, reduced, and deterministic frozen
   capture are distinct modes. Reduced mode removes non-essential movement and
   looping while retaining essential information.
4. **Stable identity.** Every repeated or native-rendered item has a stable key
   so a rebuild can continue, reverse, or cancel the same phase. The existing
   node animation key is the native precedent.
5. **Interruption.** Each role specifies retarget/reverse, repeated activation,
   abort, pointer cancellation, unmount, delayed cleanup, and final-state
   behaviour. Timers/rAF/WAAPI/native clocks cannot outlive their owner.
6. **Property budget.** The default cross-runtime subset is opacity, translation,
   scale, and rotation. Layout, blur, path drawing, gradients, filters, canvas,
   and 3D pointer effects are opt-in appearance layers with a static/reduced
   fallback.
7. **Runtime shape.** Shared web core owns state/effects and shared styles;
   Svelte and React remain thin adapters; shared Rust composition emits the
   native node/animation description; GPUI interprets it. Jetstream remains
   deferred. This follows [`docs/architecture/001-poodle-system-shape.md`](../../architecture/001-poodle-system-shape.md),
   [`docs/architecture/003-component-docs-ia-and-implementation-substrates.md`](../../architecture/003-component-docs-ia-and-implementation-substrates.md),
   [`docs/architecture/006-headless-core-and-machine-model.md`](../../architecture/006-headless-core-and-machine-model.md),
   and [`docs/architecture/010-native-presentation-construction-context.md`](../../architecture/010-native-presentation-construction-context.md).

### Pilot scope

The pilot should be five semantic families, with no Pro source dependency:

| Pilot | Components | What it proves | Required evidence |
|---|---|---|---|
| Disclosure | `Accordion`, `Collapsible` | Layout/open state, indicator, reversal, conditional content | Same semantic state and final layout in Svelte/React/native; reduced mode; rapid open→close. |
| Transient notification | `ToastStack`, `ToastHost` | Stable queue identity, enter/exit/remove, live-region/no-focus-theft | Host-owned item removal, timeout cancellation, stack reorder, native stable keys, reduced mode. |
| Selection indicator | `Tabs` | Measured geometry and first-layout/resize rules | Same selected state and indicator destination; resize/rapid selection; no motion on first paint/reduced mode. |
| Discrete state | `Checkbox`, `IconButton`/state swap | Boolean/status update with optional draw/cross-fade | ARIA state immediate; interrupted reversal; native fallback without path/blur. |
| Loading/reveal | `Skeleton`, `Spinner` | Static/animated loop and content reveal | `animated=false`, reduced preference, unmount, completion, capture freeze; no duplicate status announcements. |

The pilot should not include Dialog until its explicit contract changes, and it
should not include the Pro cards. `Tooltip`, `Switch`, validation error, and a
status-text swap can follow once the policy and cancellation shape are proven.

## Explicit non-goals

- Reimplementing or importing the 43-card Transitions.dev library.
- Copying free or Pro CSS/React recipes, generated `_root.css`, or a
  substantial collection under a different name.
- Shipping a generic `transitions.dev` compatibility layer, aliases, or silent
  fallbacks. Poodle is pre-v1 and its working rules require an operator decision
  before a breaking migration.
- Adding modal/dialog intermediate states without revising
  `docs/contracts/components/dialog.md`.
- Treating hover decoration, confetti, particles, gradient branding, 3D tilt,
  goo filters, image warp, smoky dissolve, or marketing CTAs as generalized
  Poodle primitives.
- Standardizing AI copy, private reasoning presentation, image-generation
  choreography, or product navigation/page transitions in Poodle.
- Solving Jetstream accessibility or animation support in this value track.
- Editing package source, component contracts, architecture, tokens, demos,
  tests, generated artifacts, triage, indexes, roadmaps, or release/CI files.

## Risks

| Risk | Evidence | Mitigation if promoted |
|---|---|---|
| Runtime drift | Svelte slides disclosure/Drawer; React is structural; native ToastStack animates while web ToastStack does not. [VF] | Contract semantic phases first; cross-runtime focused tests before visual polish. |
| Reduced-motion incompleteness | Most audited core loading styles have no guard; JS/rAF recipes can keep work running after CSS disables paint. [VF] | Host policy covers CSS, orchestration, and native output; test final state and no ongoing work. |
| Vestibular/attention cost | W3C/MDN guidance; shake, tilt, bounce, blur, and looping patterns in catalogue. [SC/WI] | Reduce/remove non-essential movement; never make motion the only error/status signal; allow component opt-out. |
| Layout/paint cost | Width/height, blur/filter, gradients, reels, pointer rAF, and canvas appear in the catalogue. [VF] | Property budget, static fallback, performance checks, no permanent `will-change`. |
| Cancellation bugs | Timers, forced reflow, pointer capture, rAF, delayed unmount, and stable identities vary by recipe. [VF] | Make lifecycle cases contract tests, not implementation notes. |
| Token overfitting | Upstream has many recipe-specific values beyond Poodle's compact token system. [VF] | Add semantic roles only after pilot evidence; do not expose vendor variables. |
| Native capability gap | `NodeAnimation` lacks width/height, blur, path draw, and arbitrary cubic easing; GPUI capture freezes animations. [VF] | Choose a common subset and document approximations; keep Jetstream deferred. |
| Accessibility mapping gap | GPUI 0.2.2 has no accessibility API under the current contract. [VF] | Preserve native node semantics and document what the current adapter can/cannot expose. |
| Licence boundary | Terms allow product use but prohibit repackaging the collection; Pro source is not in the public checkout. [VF] | Cite/analyse only; obtain licence review before any source reuse. |
| Upstream drift | README, data, index, extractor, and Pro access have different freshness/visibility. [VF] | Record commit/date on every refresh and reconcile cards/data/pages/references again. |
| Test false confidence | Visual harness freezes/skips motion for deterministic screenshots. [VF] | Add state/reduced/interruption tests; use visual capture only for static endpoints and explicit motion fixtures. |

## Unresolved operator decisions

1. Should motion preference be a host-level input on web/core and
   `RenderContext`, or should each component receive a narrower policy? What is
   the default when the host cannot report a preference?
2. Which semantic roles belong in the token schema after the pilot: separate
   enter/exit durations, distance roles, stagger, blur, or only existing
   duration/easing aliases?
3. Should `Drawer` gain a real closed/opening/open/closing contract, and should
   `Popover`/`Menu` gain delayed unmount? The Dialog contract must remain a
   separate explicit decision.
4. Is a sliding Tabs indicator a normative behaviour or an appearance recipe?
   The answer affects whether measurement belongs in shared core or remains
   web-only.
5. Is `ToastStack` entry/exit motion required across all active runtimes, or is
   native's current fade/rise an allowed implementation difference?
6. Is restrained validation motion approved for `TextInput`/`Field`, with a
   component-level opt-out for vestibular sensitivity?
7. What native approximation is acceptable when a web recipe uses blur, SVG
   path draw, gradients, or layout interpolation?
8. Should visible streaming status be promoted as a generic `AgentMessage`
   capability, or remain product-owned while Poodle only supplies static
   status/loading primitives?
9. After operator review, should the originating triage note be closed as
   “research complete / ready for promotion” with one architecture card, or
   kept open for a separate motion accessibility decision?

## Promotion-ready contract / architecture / card scope

This is a handoff scope, not an implementation request.

### Proposed architecture card

**Title:** Cross-runtime semantic motion policy and lifecycle contract.

**In scope:**

- Add a documented semantic motion vocabulary to the existing contract layer;
- define full/reduced/frozen policy propagation across web core, Svelte, React,
  shared Rust render, and GPUI;
- define phase, stable identity, reversal, cancellation, delayed unmount, and
  final-state rules;
- map the pilot to existing token roles, adding only evidence-backed semantic
  token gaps;
- implement the pilot in shared web core / styles and shared native render,
  with thin Svelte/React adapters and GPUI interpretation;
- add contract/state tests for normal, reduced, interrupted, aborted, and
  unmounted paths.

**Out of scope:** Pro source, decorative effects, Dialog presence changes,
Jetstream, page transitions, image/canvas effects, compatibility aliases, and
generated-artifact hand edits.

### Promotion gates

The card is ready to promote only when the operator resolves the nine
decisions above and the implementation plan can state:

- the exact public contract and whether motion is a prop, context, or host
  policy;
- the semantic state trace for each pilot component;
- the reduced-motion final state and cancellation rule for every phase;
- the supported native property/easing subset and documented approximation;
- Svelte/React parity evidence plus shared Rust/GPUI node evidence;
- accessibility evidence for focus, live regions, labels, keyboard/touch, and
  native mapping limits;
- performance evidence for layout/paint-heavy candidates; and
- token/build/drift validation using the repo's approved Effigy selectors.

### Suggested test shape

Use focused tests, not a broad visual suite for every iteration:

- core transition traces for open, close, reverse, repeated trigger, abort,
  unmount, reduced, and frozen modes;
- Svelte and React behavioural tests against the same semantic trace;
- Rust render assertions for node identity, animation key, properties, loop,
  and final static output;
- browser reduced-motion tests for CSS and JS work, including no stale timers;
- GPUI/native visual captures only for deterministic final states plus a small
  explicit animation receipt test;
- visual diff checks after semantics and timing are stable.

## Proposed disposition of the originating triage note

**Research complete; ready for operator review and promotion planning.** Keep
the triage note itself unchanged in this worker. After the operator resolves
the decisions above, close or archive it with this dossier as the evidence
record and open one cross-cutting motion architecture/card follow-up. Do not
promote individual site cards automatically.
