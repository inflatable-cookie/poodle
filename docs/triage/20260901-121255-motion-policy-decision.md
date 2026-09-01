# Shared Motion Policy Decision

Status: ready for planning PR review — intake only; no promotion or execution authority
Captured: 2026-09-01
Updated: 2026-09-01
Owner: Poodle Northstar orchestrator
Scope: host-level motion policy and the bounded five-family pilot
Promotion authority: Poodle Northstar orchestrator after planning PR review

This packet records planning decisions. It does not change architecture,
contracts, components, tokens, roadmaps, runtime admission, or execution
authority.

## Settled Direction

Operator-confirmed before this delegate:

- Motion policy is host-level across web core and native `RenderContext`.
- The policy has `full`, `reduced`, and deterministic `frozen` capture modes.
- Semantic and accessibility state update immediately. Motion never owns ARIA,
  labels, status text, progress meaning, focus, or correctness.
- Stable identity, reverse or retarget behaviour, abort and unmount cleanup,
  and the final visual state belong to the policy.
- The default cross-runtime property budget is opacity, translation, scale,
  and rotation. Layout, blur, path drawing, gradients, filters, canvas, and 3D
  effects need separate rationale and a static or reduced fallback.
- The pilot covers five families:
  - `Accordion` and `Collapsible` disclosure;
  - `ToastStack` transient notifications;
  - the `Tabs` selection indicator;
  - `Checkbox` plus same-slot `IconButton` or state swap;
  - `Skeleton` and `Spinner` loading or reveal.
- Dialog presence is outside the lane. Jetstream remains deferred.

## Repository Evidence

- `UiPresentationProvider` already propagates host defaults through Svelte and
  React context. Native composition carries the same kind of explicit,
  construction-time inheritance through `RenderContext`; nested scopes replace
  inherited defaults and explicit component values win.
- `RenderContext` currently carries theme, size scale, and density. Motion can
  follow this ownership boundary without adding ambient global state or motion
  metadata to resolved `Node` trees.
- `NodeAnimation` declares stable keys, one-shot or looping clocks, opacity,
  rotation, translation, and scale properties, plus four named easings.
- The stock GPUI 0.2.2 backend does not realize that whole abstract budget:
  generic styled elements animate opacity; unwrapped SVG leaves can animate
  rotation; generic translation and scale are unavailable. `PingPong` is
  approximated as `Loop`.
- Current deterministic GPUI capture removes `NodeAnimation` declarations. It
  therefore schedules no clock, but leaves the authored initial frame. That is
  a harness workaround, not yet a general frozen-mode contract.
- Current runtime behaviour is inconsistent: Svelte disclosure slides while
  React and native disclosure are structural; native `ToastStack` enters with
  opacity and translation while web toast presence is structural; web and
  native loading loops have no shared host preference policy.
- Current tokens provide 120ms, 180ms, and 260ms durations plus standard and
  emphasized easing. The pilot should reuse them unless evidence proves a
  missing semantic role.
- The active pilot contracts have not drifted since the dispatch base except
  for the already-landed `g16.028` Tabs contract work represented in this
  planning branch.

## Operator Decisions

### Missing preference

Use `full` when a normal host cannot report a user preference. Treat missing
preference as no preference signal, not as an inferred request to reduce.
Require capture and test hosts to select `frozen` explicitly.

Why:

- matches the web platform's no-preference/default cascade;
- keeps existing visual behaviour stable for hosts without preference wiring;
- avoids silently treating an integration omission as user intent;
- preserves a clear validation failure when a host that can report preference
  fails to propagate it.

Rejected: default unknown hosts to `reduced`. It would silently change
component behaviour in every host without preference integration and hide
missing host wiring.

### Reduced versus frozen

The modes split as follows:

- `reduced`: semantic state remains immediate; stop continuous loops; remove
  translation, scale, rotation, blur, bounce, and layout travel; allow only a
  short, non-looping opacity transition when it preserves continuity and does
  not delay presence, removal, focus, or announcement;
- `frozen`: semantic state remains immediate; schedule no visual clock; render
  the settled endpoint for one-shot phases and a role-defined, readable static
  frame for loops;
- neither mode changes semantic timers by itself. Toast expiry, loading state,
  and other product lifecycle remain with their existing owners. A capture
  harness may control time separately.

This keeps reduced motion as a user policy and frozen output as a deterministic
evidence policy. It also exposes the current GPUI capture defect: clearing a
one-shot declaration can leave its initial rather than settled frame.

Rejected alternatives:

- make reduced and frozen identical static-output modes; simpler, but loses a
  useful user-facing continuity option and conflates accessibility with test
  determinism;
- let every role define reduced behaviour independently; flexible, but weakens
  host-level predictability and makes parity evidence harder to falsify.

### Public host shape

Use a dedicated motion policy context/provider beside
`UiPresentationProvider`, backed by one shared `MotionPolicy` value with
`full`, `reduced`, and `frozen` variants. Platform integration resolves the
system preference to that effective value; Poodle consumes the value rather
than owning an ambient preference lookup. An unavailable signal resolves to
the settled `full` default.

On web, the provider supplies the value to Svelte and React descendants and a
stable inherited style hook for shared core CSS. On native, `RenderContext`
carries the same effective value beside theme, size scale, and density. A
presentation-only nested scope preserves motion unchanged.

Why keep it separate from `UiPresentationProvider`:

- size and density are presentation defaults that a composite may replace;
- motion policy represents host or user constraint and must survive those
  internal presentation scopes;
- adding a defaulted motion field to every existing presentation scope risks
  resetting a reduced ancestor to `full`;
- a dedicated provider is additive and keeps preference discovery outside
  component behaviour.

Rejected alternatives:

- extend `UiPresentationProvider`; fewer public providers, but it couples
  independent inheritance laws and makes existing internal scopes risky;
- let each runtime discover system preference ambiently; convenient on web,
  but does not give web orchestration, Rust composition, GPUI, and deterministic
  capture one explicit policy input.

### Inheritance and component overrides

Use restriction-only nesting: `full < reduced < frozen`, and the effective
descendant policy is the more restrictive ancestor or child value. A nested
scope can reduce or freeze motion but cannot re-enable motion suppressed by an
ancestor.

Components receive no new general motion override in the pilot. Existing
semantic controls such as `Skeleton.animated=false` remain stricter and win.
Components may decline motion in any mode; they may not force motion above the
effective host policy.

Rejected alternatives:

- nearest provider wins; familiar context semantics, but a subtree could
  violate user reduced-motion or a capture freeze;
- root-only policy with no nesting; simplest, but prevents bounded frozen
  evidence and local stricter scopes without rebuilding a separate host.

### Motion identity

Key a motion instance by semantic owner identity plus motion role and channel,
not by tree position, generated render order, or a global recipe name. The key
survives immediate-mode rebuilds while that semantic owner survives. Two
siblings cannot share a clock accidentally.

For keyed collections such as toasts, authored item identity owns enter, update,
and exit continuity. Reusing a retired key before delayed visual cleanup ends
reverses or retargets the same visual remnant; a genuinely new semantic item
needs a new key.

Rejected alternatives:

- restart on every rebuild; simple, but immediate-mode native trees and React
  rerenders can prevent one-shot motion from completing;
- let every component invent identity rules; flexible, but makes the shared
  lifecycle policy and cross-runtime evidence non-falsifiable.

### Interruption and terminal behaviour

Use latest-state-wins with no queued visual phases:

- repeated activation of the current target is a no-op and does not restart;
- a reversible binary phase continues from its current sampled progress toward
  the new endpoint, with duration proportional to the remaining progress;
- a multi-target update such as a Tabs indicator retargets from its current
  rendered geometry to the latest measured target using the role's bounded
  update duration;
- an exit may retain an inert paint-only remnant until its visual phase ends,
  but it leaves accessibility, focus order, hit testing, and live-region
  ownership immediately;
- abort, owner unmount, or a switch to `frozen` cancels every clock and cleanup
  handle. Abort and `frozen` settle to the latest semantic endpoint; owner
  unmount removes the remnant;
- a switch from `full` to `reduced` drops disallowed properties immediately and
  may finish only an allowed short opacity phase;
- motion completion never fires semantic callbacks or owns component state.

Rejected alternatives:

- restart a full-duration phase from the current sample; smooth but makes rapid
  reversal arbitrarily slow;
- snap to the old endpoint before starting the new phase; deterministic but
  visibly jumps and misrepresents the latest semantic state.

## Pilot Decisions And Resolved Gap

The current Tabs contract does not contain the measured sliding indicator
assumed by the research queue's pilot oracle. `activeEdge="underline"` is a
static border on each tab item; selection swaps which border is coloured. The
web implementations measure only overflow, not indicator geometry. Shared Rust
renders the same per-item border. The contract's stale implementation-freedom
line about “indicator animation internals” has no corresponding anatomy,
state, style, or runtime mechanism.

This is not post-dispatch mainline drift: the current contract predates this
delegate and is the state named by the handoff. It is a planning gap between
the research recommendation and current component authority. The packet cannot
quietly claim first-layout, resize, or measured retarget evidence without an
operator decision to add that observable behaviour.

### Tabs pilot boundary

Keep Tabs in the pilot and promote one moving indicator only for
`activeEdge="underline"`. It is a single paint-only child of the tablist,
measured from the selected tab. First layout and frozen mode paint the selected
endpoint without motion; resize and rapid selection retarget the same semantic
owner key. Outline and fill treatments stay on their current per-item paths.

This is an observable contract change but does not require a new public prop.
The later canonical promotion must remove the stale implementation-freedom line
and specify anatomy, measurement, first-paint, orientation, overflow, and
fallback behaviour.

Rejected alternatives:

- keep current static edges and reduce the Tabs pilot to an opacity crossfade;
  bounded, but drops the measured-geometry oracle the operator-reviewed queue
  selected;
- make measured sliding web-only; preserves the web effect, but creates a
  deliberate active-cohort visual capability gap at the policy's first pilot.

### Native approximation limit

Allow a bounded semantic approximation in the first pilot. Every runtime must
match semantic timing, identity, interruption, cleanup, and static endpoints.
GPUI full mode may replace unsupported generic translation or scale with the
same bounded opacity phase used by reduced mode, or use the static endpoint
when opacity would mislead. SVG rotation remains available. Layout, blur, path,
gradient, filter, canvas, and 3D interpolation remain unavailable.

Every approximation is named per pilot role and remains a visible active-cohort
capability gap under the working rules. It cannot be reported as full visual
parity. A later backend capability may close it; the policy does not add a
silent fallback.

Rejected alternatives:

- require exact full-mode property parity before the pilot can land; strongest
  parity posture, but turns the policy pilot into a GPUI rendering-capability
  programme;
- restrict the whole cross-runtime policy to generic opacity plus SVG rotation;
  exact today, but contradicts the already-settled translation and scale
  property budget and weakens the selected pilot.

### Disclosure geometry

Keep disclosure height interpolation as the pilot's one explicit layout-motion
exception. It is intrinsic to understanding content reveal and is already
normative in the Accordion and Collapsible contracts. Full web mode may animate
the clipped block axis plus paint; reduced and frozen jump to final layout.
Native may use the settled opacity/static approximation until a bounded layout
capability exists.

The exception stays role-specific. It does not add width, height, or arbitrary
layout animation to the default motion property vocabulary.

Rejected alternatives:

- replace disclosure height motion with paint-only opacity and translation;
  keeps the default budget pure but breaks the current component contracts and
  makes collapsing geometry jump before the visual exit;
- make disclosure instant in every mode; exact across the cohort today, but
  removes the pilot's only meaningful conditional-layout lifecycle.

### Loading loops

Normalize `Skeleton` full mode to a 1.6s opacity pulse across the active cohort,
replacing the web gradient-position shimmer. Keep Spinner's semantic variants:
ring rotation and dot/grid opacity phasing. This uses only the settled property
budget and leaves richer shimmer behind its separate evidence gate.

In reduced and frozen modes, both components render a role-defined readable
static frame and schedule no loop. `Skeleton.animated=false` stays static in
all modes. A loading-to-content reveal is host-owned evidence: the parent's
semantic loading state changes immediately, while the visual replacement may
use the policy's allowed one-shot opacity. The pilot does not add content or
completion ownership to Skeleton or Spinner.

Rejected alternatives:

- preserve web Skeleton shimmer and native opacity pulse as an explicit visual
  delta; smaller migration, but the first policy pilot does not normalize its
  clearest current divergence;
- make Skeleton static even in full mode; simplest, but discards its current
  animated contract rather than repairing it.

### First committed frame

Paint authored initial state at its settled endpoint. Do not synthesize an
enter, update, or selection transition from a component's default or placeholder
state on first mount. This covers default-open disclosure, preloaded toasts,
initial Tabs selection, checked Checkbox, seeded IconButton state, and initially
visible content.

Continuous loading loops may start after the first committed frame in `full`.
They remain static in `reduced` and `frozen`. A new semantic owner added after
the baseline frame may run its contracted enter phase.

Rejected alternatives:

- animate all initial state; visually lively, but invents transitions from
  states the host never authored and makes hydration/capture unstable;
- let each component decide; preserves current quirks but weakens the shared
  lifecycle oracle.

### Reduced outcomes by role

Use opacity selectively rather than automatically:

- disclosure: immediate final layout and paint; no reduced phase;
- ToastStack: short opacity enter/exit; translation removed;
- Tabs underline: jump to the measured endpoint; no reduced phase;
- Checkbox and same-slot IconButton/state swap: short opacity crossfade with
  immediate checked, pressed, label, and busy state;
- Skeleton and Spinner: readable static frame; no loop;
- host-owned loading-to-content reveal: short opacity replacement is allowed.

This spends the reduced-mode allowance where opacity preserves continuity
without implying movement or delaying navigational geometry.

Rejected alternatives:

- give every one-shot role an opacity phase; uniform, but adds attention motion
  to disclosure and selection where immediate spatial truth is clearer;
- make reduced fully static; simplest, but rejects the already-settled bounded
  opacity allowance rather than applying it deliberately.

## Settled Semantic Boundaries

- Motion policy does not pause, start, restart, or extend semantic timers.
  Toast expiry remains host-owned and is measured from semantic insertion, not
  visual completion. Capture harnesses may control time separately from
  `frozen` visual policy.
- A toast joins its live region immediately when inserted. Enter motion never
  delays announcement. Reorder, retarget, policy changes, and visual completion
  do not reannounce it.
- A dismissed toast leaves live-region and accessibility ownership immediately.
  Any retained exit remnant is `aria-hidden`, inert, unfocusable, and excluded
  from hit testing.
- Danger-toast assertive posture remains semantic and mode-independent.
- Skeleton remains decorative and hidden from accessibility. Spinner's optional
  status role and label remain present in every policy mode; stopping its loop
  does not remove loading meaning.
- Motion never steals focus, delays focus movement, or makes visual completion
  a prerequisite for keyboard behavior.

### Focused toast removal

When a dismissed toast owns focus, move focus synchronously to the equivalent
control on the next surviving toast, else the previous toast, else restore the
focusable element from which focus entered the stack when it still exists.
Only then make the exit remnant inert. If no target remains, use ordinary host
focus order rather than focusing the visual remnant or stack chrome.

This is ToastStack behavior, not generic motion behavior. The pilot surfaces
the current contract gap and the later promotion must place the rule in the
component contract.

Rejected alternatives:

- leave focus to platform behavior after DOM/node removal; smallest change,
  but often drops focus to the document root and differs by runtime;
- add a host focus callback; explicit, but widens the public API for a behavior
  the reusable stack can resolve from its own ordered items.

### Tabs resize

On first measurement, orientation change, container resize, font reflow, or an
overflow-mode switch, remeasure and paint the selected underline endpoint
without motion. Animate only a semantic selection change made against stable
geometry. A resize during selection motion cancels that phase and snaps to the
remeasured latest selected endpoint.

This prevents window or font layout from looking like a user selection and
keeps capture deterministic.

Rejected alternatives:

- retarget through every resize; visually continuous, but can chase layout and
  turns environmental change into authored motion;
- keep the old geometry until the next selection; avoids resize work, but
  leaves a visibly incorrect indicator.

## Operator Decisions — Evidence And Promotion

### Pilot oracle and falsifiable evidence matrix

Use a layered minimum rather than a full family × mode × runtime Cartesian
board.

#### Shared policy laws

Run the same authored traces through framework-free TypeScript and shared Rust:

- missing preference resolves to `full`;
- nesting can only move `full` → `reduced` → `frozen`;
- initial owners paint settled endpoints;
- semantic-owner keys continue across rebuilds and never collide across
  siblings;
- repeated targets do not restart; binary reversal is proportional; latest
  multi-target update wins with no queue;
- `reduced` emits only allowed opacity phases; `frozen` emits no clocks and
  resolves one-shot endpoints or canonical loop frames;
- abort, policy tightening, delayed exit, and unmount leave no timer, rAF,
  WAAPI, pointer-capture, or native-clock owner behind.

#### Five family oracles

| Family | Smallest falsification set |
| --- | --- |
| Disclosure | settled default-open frame; open → close reversal; immediate expanded state and focus exclusion; full clipped-height exception; reduced/frozen final layout; unmount cleanup |
| ToastStack | preloaded items do not enter; keyed add, reorder, dismiss, exit reversal, and re-add; immediate live ownership; no focus theft; focused removal fallback; reduced opacity; frozen endpoint; semantic expiry unchanged |
| Tabs | initial underline snaps; A → B → C retarget; horizontal and vertical endpoints; resize, font/orientation, and overflow changes snap; reduced/frozen snap; selected/focus semantics immediate |
| Discrete state | Checkbox checked/mixed endpoints and interrupted reversal; same-slot IconButton pressed/loading swap; labels, checked/pressed/busy state immediate; reduced opacity; frozen endpoint; no path/blur dependency |
| Loading/reveal | Skeleton full opacity pulse and `animated=false`; Spinner ring plus one opacity-phased variant; reduced/frozen static frames with no scheduled loop; host-owned loading → content opacity; semantics and unmount cleanup |

#### Runtime receipts

- Svelte and React run the same family traces against real components and
  shared core policy.
- Rust render assertions prove effective policy, stable keys, properties,
  endpoints, loop declarations, and removed declarations in frozen mode.
- Headless GPUI probes prove generic opacity and SVG rotation, record every
  translation/scale approximation, and fail if an unsupported property is
  silently treated as parity.
- A small headless browser probe owns real disclosure geometry, Tabs
  measurement/resize, toast focus, and no-live-region reannouncement claims.
- Deterministic web/native visual capture checks only static endpoints and
  canonical frozen loop frames. It does not stand in for reduced-motion or
  lifecycle evidence.

No new portable case corpus, generated interface, exhaustive specimen tab, or
permanent conformance authority is introduced.

Rejected evidence shapes:

- exhaustive mode × family × runtime fixtures; stronger breadth, but repeats
  shared laws and recreates the rejected broad conformance cost;
- one representative component per family with no paired runtime receipts;
  cheaper, but cannot falsify the current Svelte/React/native divergences.

### Promotion split

Promote in two serial records:

1. one cross-runtime semantic motion architecture/contract decision covering
   `MotionPolicyProvider`, web/native propagation, policy laws, property budget,
   approximations, and evidence rules;
2. one bounded five-family pilot card updating the affected component
   contracts and delivering the evidence matrix above.

Proposed canonical destinations:

- new semantic motion architecture record, linked from system shape and native
  construction context;
- new `MotionPolicyProvider` component contract; explicit separation from
  `UiPresentationProvider`;
- focused updates to Accordion, Collapsible, ToastStack, Tabs, Checkbox,
  IconButton, Skeleton, and Spinner contracts;
- current token schema reused unchanged at promotion time unless implementation
  evidence proves one missing semantic role;
- one new post-g16 roadmap card only after architecture/contract promotion and
  readiness review.

Rejected alternatives:

- one card per family; easier local ownership, but duplicates policy substrate
  and lets early families land before the cross-family oracle exists;
- one monolithic architecture-plus-implementation card; fewer records, but
  hides the policy gate and makes review of public semantics depend on code.

## Unresolved Questions

None inside this delegate's decision boundary. The orchestrator still must
reconcile this intake against current `main`, choose final canonical filenames
and roadmap numbering, and run readiness before execution. Exact implementation
mechanisms remain worker-owned only after that promotion.

## Non-goals

- No named transition catalogue or arbitrary effect API.
- No icon morphing, shimmer, block-slider, Dialog, or page-transition design.
- No implementation choice, compatibility surface, release, consumer adoption,
  Jetstream admission, or active-cohort parity claim.
- No permanent conformance authority or exhaustive specimen matrix.

## Promotion Route

Accepted merge is intake, not promotion. The orchestrator reconciles this
packet with current `main`, promotes one cross-runtime motion architecture or
contract decision, runs readiness, then promotes one bounded pilot card. Only
that canonical sequence may dispatch implementation.
