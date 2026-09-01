# AgentSubagent Running Activity Shimmer — Decision Packet

Status: proposal for orchestrator review; implementation held
Captured: 2026-09-01
Owner: Poodle Northstar orchestrator
Source handoff: `docs/handoffs/20260901-230404-agent-subagent-shimmer-planning.md`
Promotion authority: orchestrator
Scope: AgentSubagent contract/runtime ownership, then a bounded web-only benchmark

## Decision Summary

The semantic host is `AgentSubagent`'s running activity line. The effect is a
web-only visual treatment unless native text-mask support is proved in a later,
separate architecture lane. Native AgentSubagent semantics and presentation
remain static.

This packet sequences the prerequisite contract reconciliation and the
benchmark. It does not implement either surface, promote a contract, add a
public API, or claim that a transform is GPU-accelerated. The following are
already settled by the handoff:

- AgentSubagent owns the running activity-line presentation.
- Shimmer is not a generic `TextShimmer`, `Text` prop, `AgentMessage` treatment,
  copied Pen, or unsupported GPU claim.
- Architecture 012 owns `full`, `reduced`, and `frozen` behavior.
- The benchmark compares the ordinary static line, the current
  `background-position` approach, a mask-plus-transform candidate, and a
  tested `background-clip: text` alternative only when it has complete support.
- A static readable line is always the fallback.

The candidate lifecycle is fixed: when an item enters an eligible `running`
epoch with a non-empty `activityLine` under effective `full` policy, one
non-looping 2.0-second sweep starts after the first committed frame. A
replacement of `activityLine` while that same eligible running epoch continues
updates the semantic value immediately but does not restart or queue the sweep.
Any loss of eligibility cancels remaining visual work immediately; a later
transition out of `running` and back into `running` creates a new epoch. This
is private presentation: no public pause/stop/hide prop is added and the
decoration never mutates semantic status.

## AgentSubagent Prerequisite

### Current state

`AgentSubagent` already has the relevant implementation and evidence paths:

- The draft semantic contract is
  [`docs/contracts/components/agent-subagent.md`](../contracts/components/agent-subagent.md).
- Shared status vocabulary and terminal/spinner rules live in
  [`packages/core/src/agent-subagent.ts`](../../packages/core/src/agent-subagent.ts).
- Svelte and React shells, shared web styles, the Rust spec/renderer, and a GPUI
  specimen are present. The live evidence row is in
  [`docs/roadmaps/g16/parity-evidence-ledger.md`](../roadmaps/g16/parity-evidence-ledger.md).
- `AgentTranscript` renders the group inside its `role="log"`
  / `aria-live="polite"` viewport and forwards child navigation. Its contract
  is [`docs/contracts/components/agent-transcript.md`](../contracts/components/agent-transcript.md).
- The current contract still says `draft` and describes React/native variants
  as deferred even though the active implementation/evidence paths now exist.
  That stale state is the prerequisite gap, not a reason to add shimmer code.

### Ownership map

| Concern | Owner | Boundary to preserve |
| --- | --- | --- |
| Child identity, provider status, activity text, localization, persistence, and lifecycle | The consuming host/provider | Supplies one `AgentSubagentItem`; owns transport, data updates, navigation, and any operation-level cancel/stop. AgentSubagent remains observation-only; operation cancellation is not a visual pause API. |
| Status vocabulary, terminal mapping, running-spinner rule, and activity/summary projection | `AgentSubagent` contract plus `poodle-core` | `unknown` stays “Unknown”; only `running` spins; `activityLine` is non-terminal content and `summary` is terminal content. The component does not infer status. |
| Transcript placement, grouping, virtualization, scroll anchoring, live-region ownership, and `onOpenChild` forwarding | `AgentTranscript` | The transcript owns `role="log"` and polite announcement behavior. The activity line does not become its own live region and is not announced once per update. |
| Web rendering and selection-safe enhancement | `poodle-core` styles with thin Svelte/React shells | The real text node stays readable and copyable. Any visual duplicate is internal, inert, hidden from assistive technology, pointer-inert, and non-selectable. No caller supplies a second string or rich child tree. Epoch and cancellation bookkeeping stay private; no consumer pause/stop/hide prop is added. |
| Native semantic composition and static visual fallback | `poodle-render`, `poodle-node`, and GPUI | Native receives the same item meaning and motion-policy input but no text-mask promise. GPUI stays static for this effect; Jetstream remains deferred. |
| Effective motion policy | Host integration and Architecture 012 | The host supplies `full`, `reduced`, or `frozen`; no component performs ambient preference discovery. Descendants cannot re-enable restricted motion. |
| Benchmark harness and traces | A disposable web-only fixture | Keep it outside the published package graph and do not create a permanent conformance authority from this packet. |

### Required prerequisite batch

Before a benchmark result can support promotion, a separate contract-first
batch should reconcile `agent-subagent.md` with the live shape and record:

1. AgentSubagent's contract status and active runtime ownership. The current
   Svelte, React, shared Rust, and GPUI paths are evidence; Jetstream remains a
   program-level deferred backend.
2. The exact source and lifecycle of `activityLine`: the host supplies the
   string, the component presents it while the status is non-terminal, and a
   terminal status presents `summary` instead. A status update changes
   semantics immediately.
3. The motion role `agent-subagent/activity-line-shimmer` as an implementation
   role, not a generic public effect. It is eligible only for a non-empty line
   while the item is `running`, under effective `full` policy, on a supported
   rendered/on-screen path. Entering that eligible epoch starts exactly one
   non-looping 2.0-second sweep after the first committed frame. Non-empty
   `activityLine` replacements in the same epoch update semantics immediately
   without restarting or queueing a sweep; after the sweep, the same epoch is
   static.
4. The Architecture 012 mode table, including first committed frame, latest
   state wins, immediate cancellation on teardown or loss of eligibility, and
   zero-clock `frozen` behavior. The composed Spinner continues to follow its
   own 012 rules. The decoration never mutates the host's status.
5. Selection, copy, accessible-tree, live-region, focus, forced-colors, print,
   unsupported-mask, and font/resize rules below.
6. The intentional web-only visual delta and the native static posture. No
   portable Rust field may imply that GPUI paints a glyph mask.

The prerequisite is complete only when the contract no longer contradicts the
current runtime evidence, the ownership map is canonical, and the benchmark
can use the existing semantic input without inventing a new prop. This packet
does not make that promotion itself.

### Decoration lifecycle

An eligible running epoch is a private, monotonic identity for one contiguous
interval in which all of these hold:

- the host-supplied status is `running`;
- `activityLine` is non-empty;
- effective motion policy is `full`;
- the tested web enhancement path is supported and is not a fallback, forced-
  colors, or print path; and
- the item is rendered/materialized and on screen under the existing
  inactivity/offscreen policy.

The host supplies status, text, policy, and existing render/visibility
lifecycle. It does not receive an epoch handle or a visual pause API, and the
effect never writes or changes semantic status. After the first committed
frame of an eligible epoch, the selected web candidate runs exactly one
2.0-second animation with one iteration, then settles to static text for the
rest of that epoch. A non-empty `activityLine` replacement updates the source
semantics and overlay alignment immediately while preserving the current
epoch, deadline, and sweep; it never restarts or queues visual work. Once the
sweep has ended, replacements still do not start another sweep.

Leaving `running`, becoming empty or otherwise ineligible, changing to
`reduced` or `frozen`, entering forced colors or print, losing candidate
support or selecting the static fallback, becoming inactive/offscreen or
unmaterialized, and unmounting all cancel remaining visual work immediately
and leave the readable source text static. A later re-entry into eligibility
gets a fresh epoch; in particular, leaving `running` and later re-entering
`running` permits exactly one new sweep. A line replacement that keeps the
same eligible epoch does not. Offscreen/inactivity cancellation uses the
existing host render/visibility boundary or private internal observation; it
does not add a consumer-facing prop.

## Host Semantics And Fallbacks

The effect is decoration around existing meaning. It never owns status,
progress, announcements, focus, selection, or operation cancellation; it owns
only cleanup of its private visual work.

| Condition | Semantic output | Visual output |
| --- | --- | --- |
| `status="running"` with a non-empty `activityLine`, effective policy `full`, supported path, and rendered/on-screen item | The one host-supplied line remains the only semantic text value. | Start exactly one non-looping 2.0-second sweep after the first committed frame. The existing Spinner remains the activity indicator and follows Architecture 012. |
| Same eligible running epoch receives a non-empty `activityLine` replacement | The replacement is immediate and remains the one semantic text value. | Preserve the current sweep and deadline; do not restart or queue. After the sweep, stay static until eligibility ends. |
| `running` with an empty line | Status and any existing spinner remain as defined by the contract. | Ineligible: cancel any remaining sweep and stay static; never create an empty shimmer surface. |
| `pending`, `waiting`, or `unknown` | The status word and non-terminal activity line remain immediate. `unknown` is literally “Unknown”. | Ineligible: cancel any remaining sweep and stay static. Spinner remains absent because only `running` spins. |
| `completed`, `failed`, `interrupted`, or `shutdown` | `summary` replaces the activity line when supplied. | Ineligible: cancel any remaining sweep; static summary and no spinner. |
| Effective policy `reduced` | Status, line, summary, focus, and announcements update normally. | Cancel any remaining sweep and stay static; Spinner follows 012's reduced static treatment. |
| Effective policy `frozen` | Same semantics as `full`; frozen is deterministic evidence policy, not reduced-motion evidence. | Cancel any remaining sweep; canonical static frame with no visual clock or late animation work. |
| Forced colors, print, unsupported/incomplete mask path, or selected static fallback | Same text and host semantics. | Ineligible: cancel any remaining sweep. Use ordinary visible system/forced-color text; disable mask, gradients, duplicate overlay, and decorative color treatment. |
| Item becomes inactive/offscreen/unmaterialized, leaves the rendered window, or component unmounts | No semantic disappearance beyond the existing host contract. | Cancel remaining visual work immediately and leave readable text static. No timer, observer, rAF, or animation handle may outlive the owner. |
| Status leaves `running` and later re-enters `running` with an eligible non-empty line | Host status semantics remain authoritative. | The old epoch cannot resume. Allocate a fresh epoch and permit exactly one new 2.0-second sweep after its first committed frame. |

The existing host status and render/visibility lifecycle are the only external
inputs. The effect exposes no public pause/stop/hide prop and never mutates
semantic status; a consumer does not need to change status merely to stop
decoration. The fixed two-second, non-looping bound is the complete visual
lifetime.

### Selection, copy, and accessibility rules

- The ordinary text remains visible in every mode and browser. Making the real
  text transparent and leaving only a painted overlay is rejected.
- The semantic text exists in the accessibility tree exactly once. The
  component root remains a plain group inside the transcript log; the shimmer
  never receives `role="status"`, `aria-live`, or a progress role.
- The transcript continues to own polite announcements. Running-line updates
  do not become token-by-token announcements, and shimmer completion emits no
  semantic callback.
- A same-epoch line replacement changes semantic text immediately but does not
  create another visual phase; cancellation and completion are private visual
  cleanup, not status updates or consumer callbacks.
- The existing spinner remains decorative (`aria-hidden`). Status words remain
  textual, so color is never the only status channel.
- If the web candidate needs a duplicate to paint a mask, AgentSubagent owns
  the duplicate. It must be `inert`, `aria-hidden`, non-focusable,
  pointer-inert, and excluded from selection; tests must prove that partial and
  full copy return the source string once.
- The benchmark must exercise keyboard and touch selection, selection highlight
  contrast, adjacent-link/button focus, the accessible tree, and live-region
  stability. The visual effect cannot change focus order or the `onOpenChild`
  / disclosure behavior.
- Base text must meet the repository's normal contrast requirements in light
  and dark themes. The animated highlight is not a contrast substitute.

## Sequencing And Serial Edges

The ordering is deliberately narrow:

```text
g16.034 / Architecture 012 landed
          |
          v
AgentSubagent contract + runtime ownership reconciliation
          |
          v
disposable web benchmark and evidence capture
          |
          v
threshold verdict: per-engine enhancement or static fallback
          |
          v
future web implementation card, only if the verdict passes
```

Serial edges:

| Edge | Why it is serial |
| --- | --- |
| `g16.034` → ownership prerequisite | Architecture 012 is the motion authority. It is already merged; no second policy is allowed. |
| Ownership prerequisite → benchmark | The benchmark must know the exact semantic input, status lifecycle, transcript live-region boundary, and native static posture before measuring a visual treatment. |
| Static control/baseline → moving candidates | Every claim needs the ordinary text control and the current `background-position` baseline in the same environment and run shape. |
| Benchmark → any public web implementation | A passing trace and accessibility/fallback result is a promotion gate, not an implementation detail. A failed result stops at static fallback. |
| Web enhancement → native visual parity | There is no such edge in this lane. If active-cohort visual parity is later required, stop and open a new renderer-capability architecture lane before changing the native contract. |
| This packet → canonical promotion | The packet is intake. The orchestrator re-reads it against current `main`, promotes the settled contract/roadmap meaning separately, and then decides readiness. |

The packet is safe to write beside unrelated same-repository work because it
touches only its named triage path. Any future canonical promotion or
implementation must rebase against the then-current `main` and re-check the
post-motion queue; no conclusion here depends on a stale branch head.

## Bounded Web Benchmark

### Scope and candidate engines

Run a disposable harness that mounts the real AgentSubagent semantic shape in
the Svelte and React web shells for integration checks, while keeping the scale
fixture minimal enough to attribute browser work to the effect. Do not add a
package export, permanent selector, generated corpus, or public recipe from the
benchmark card.

Compare these candidates under the same text, cadence, dimensions, and node
counts. Every moving candidate uses the same finite lifecycle: one
non-looping 2.0-second sweep per eligible running epoch, followed by static
text. The ordinary static control has no sweep; no benchmark candidate may
loop indefinitely.

| Candidate | Purpose | Promotion posture |
| --- | --- | --- |
| Ordinary static text | Control for semantic and browser cost | Always-valid fallback. |
| AgentSubagent-line `background-position` gradient | Current Skeleton-style repaint baseline adapted to the line | Measurement baseline, not an optimized claim. |
| Masked text with a translated highlight | Main compositor-oriented candidate | Enable only on an engine/device/content path that passes every gate. |
| `background-clip: text` alternative | Only if the complete engine path preserves readable fallback, selection, forced colors, and alignment | Benchmark-only; never make the source text transparent by default. |

The mask candidate must be authored independently. Do not copy the public Pen,
its CSS, its fallback, or its colors. “Compositor-oriented” is the strongest
term available until a trace proves the exact pipeline on a named engine and
device.

Recommended benchmark environments and counts are explicit recommendations for
orchestrator review; they are not current product promises:

- Engines: current pinned Chromium/Blink, Firefox/Gecko, and Safari/WebKit
  builds. Record exact build identifiers in the manifest rather than relying on
  the word “latest”.
- Devices: one ordinary desktop class and one low-power/mobile class, each at
  DPR 1 and DPR 2 where the device exposes both conditions. Use 60 Hz as the
  primary comparable frame target; record a 120 Hz run separately when
  available.
- Viewports: fixed 320 CSS px and 640 CSS px content widths. The effect must
  not change the host's measured width, height, or block position.
- Node counts: `N = 1, 10, 50, 100` simultaneously running activity lines,
  each with a stable distinct item id. These are AgentSubagent activity-line
  hosts, not arbitrary Text nodes.
- Themes and axes: Iceberg and Eclipse, default density/size for the scale
  run, with compact and comfortable density as correctness smoke cases. The
  benchmark records theme, density, size, viewport, DPR, and refresh rate.

### Content and lifecycle matrix

Use fixed sanitized strings with stable content ids. The matrix is bounded as
follows:

| Case | Content or event | Coverage |
| --- | --- | --- |
| C1 | Short one-line activity, matching the existing “Checking vectors” shape | Full N=1/10/50/100 performance scale. |
| C2 | Long one-line activity near the 320 px and 640 px width limits | N=1/10/50/100; alignment and paint pressure. |
| C3 | Two-line wrap at 320 px | N=1/10/50/100; duplicate line-box alignment. |
| C4 | Five-line wrap at 320 px | N=1/10/50; multiline and layer/memory stress. |
| C5 | Long localized strings, including a metric-expanding translation and CJK text | N=1/10; font, wrapping, and copy correctness. |
| C6 | Arabic/Hebrew plus Latin punctuation and numerals | N=1/10; bidi direction and highlight alignment. |
| C7 | Stable variable-font text before and after font readiness | N=1/10; no stale overlay after font metrics settle. |
| C8 | Activity-line replacement during the sweep and 320→640 px resize while active | N=10; semantic text and geometry update immediately, with exactly one sweep in the same epoch and no restart or queue. |
| C9 | Mid-sweep loss of eligibility (`running` → non-running/empty, reduced/frozen, forced colors/print, unsupported/static fallback, inactivity/offscreen, or unmount), followed by a later `running` re-entry | N=10; remaining work cancels immediately, the old epoch does not resume, and the new eligible epoch gets exactly one new sweep. |

Every candidate gets the correctness cases at N=1 and the scale cases in C1–C3.
Run C8 and C9 at N=1 for event/lifecycle correctness and at N=10 for
fan-out. The N=50/100 runs stay limited to C1–C3 so the benchmark remains
bounded. Each scale run covers foreground, an inactivity/offscreen policy
transition, hidden-tab diagnostics, terminal transition, unmount, `reduced`,
`frozen`, forced colors, unsupported-mask, and selection/copy conditions as
applicable. A hidden or offscreen trace is evidence about lifecycle/throttling,
not permission to claim that CSS work is free.

Recommended capture shape:

- Use a 0.5-second static pre-roll, enter one eligible running epoch, wait for
  its first committed frame, capture one 2.0-second non-looping sweep, and keep
  a 0.5-second static post-roll. Repeat each candidate/environment/count cell
  three times, summarize the median, and require every repetition to stay
  inside hard gates.
- In the lifecycle traces, replace `activityLine` during the sweep and verify
  that the same epoch has one sweep with no restart or queued sweep. In a
  separate trace, leave `running`, exercise cancellation, and re-enter
  `running`; verify that only the new epoch gets one sweep. Do not add a
  consumer pause/stop/hide prop or have the effect mutate status to drive these
  traces.
- Keep the existing 1.6-second Skeleton timing as a separately named lineage
  baseline; neither timing becomes a public token in this lane.
- Capture browser performance traces, frame-time samples, geometry snapshots,
  paint invalidation evidence, layer information, and exposed texture/memory
  data. Capture DOM/accessibility-tree snapshots and selection/copy results for
  every content family.
- Use `frozen` for deterministic semantic and endpoint screenshots. Full-mode
  moving captures require an explicit phase/time anchor for the one sweep; two
  coincident frames are not evidence.

## Recommended Budgets

All numeric values in this section are delegate recommendations for review.
They make the future benchmark falsifiable without pretending that the current
repository has measured them.

| Budget | Recommendation |
| --- | --- |
| Lifecycle | Each eligible running epoch gets exactly one non-looping 2.0-second sweep after its first committed frame, with one terminal end/cancel outcome. A same-epoch non-empty `activityLine` replacement creates no new start or queue; every loss of eligibility cancels remaining visual work, and only a fresh epoch may start again. |
| Layout geometry | Effect-caused x/y/width/height change is zero. Instrumentation may use a 0.25 CSS-px comparison tolerance for fractional reporting; any raw movement beyond that fails. |
| Per-frame layout | Zero effect-attributed layout or geometry recalculation after setup. One initial layout/paint for the real text and any measured enhancement setup is allowed. |
| Paint | A path called compositor-oriented has zero recurring effect-attributed main-thread paint invalidation in steady-state frames after setup. Any recurring paint keeps that engine on the static fallback or labels the path paint-bound. |
| Main-thread work | At N=100, p95 style+layout+paint time is no more than static control +2.0 ms and no more than 4.0 ms absolute on the primary 60 Hz device. Record style, layout, paint, and raster separately. |
| Layers | At most one additional effect-related composited layer per active line plus four fixed harness surfaces: static-layer-count + N + 4 is the ceiling. No layer count may grow after setup for a fixed N. |
| Texture/layer memory | Additional layer/texture memory is no more than 16 MiB at N=50 and 32 MiB at N=100 at DPR 2; use half those caps at DPR 1. If an engine cannot expose a trustworthy measurement, record it as unavailable and do not make a memory or GPU claim. |
| Frame time | Primary 60 Hz p95 ≤16.7 ms, p99 ≤33.4 ms, and dropped frames ≤1% on desktop / ≤2% on low-power-mobile. Candidate regression against static is ≤2.0 ms p95 and ≤1 percentage point dropped frames. A 120 Hz run uses p95 ≤8.3 ms and p99 ≤16.7 ms as a separate recommendation. |
| Text alignment | Overlay and source line boxes, glyph baseline, and highlight bounds remain within 0.5 CSS px with no visible ghosting across resize, font readiness, wrapping, localization, and bidi cases. |
| Text contrast | Base and highlighted text meet at least 4.5:1 for normal text and 3:1 for large text in both themes. Forced colors use system roles and no decorative gradient. |
| Selection/copy/AT | 100% of fixed content cases return the exact source string once through selection/copy and expose one semantic text value in the accessibility tree. Any duplicate, truncation, or focus change is a hard failure. |

CPU and package-power readings should be retained as diagnostic evidence, but
they are not a gate until the chosen device pair exposes comparable readings.
The benchmark must not repeat the Pen author's approximate CPU figures as Poodle
data.

## Promotion Thresholds

Promotion is a conjunction, not an average score:

1. **Contract gate.** The AgentSubagent contract is reconciled and approved for
   the current runtime ownership. The web-only visual delta, native static
   posture, motion modes, one-shot epoch lifecycle, cancellation paths, and
   host announcement boundary are written there before implementation.
2. **Semantic gate.** Svelte and React preserve one host-supplied text value,
   existing status/summary rules, disclosure/open-child behavior, transcript
   live-region ownership, exact selection/copy, focus adjacency, and no new axe
   violations. Same-epoch `activityLine` replacements update semantics without
   restarting or queuing visual work, and the effect never mutates status.
   Native meaning remains explicit and static.
3. **Fallback gate.** Reduced, frozen, forced-color, print, unsupported-mask,
   inactive, terminal, hidden, offscreen, and unmounted cases all cancel any
   remaining sweep and resolve to a readable static line without a stranded
   effect handle. A browser that fails the complete mask path uses the static
   fallback; `@supports` alone is not enough.
4. **Lifecycle gate.** An eligible running epoch starts one non-looping
   2.0-second sweep after its first committed frame and never loops. Every
   ineligible, motion, fallback, forced-color/print, inactivity/offscreen, and
   unmount transition cancels remaining visual work immediately. A same-epoch
   line replacement does not restart or queue; leaving and later re-entering
   `running` creates a fresh epoch with one new sweep. No public pause/stop/hide
   prop is admitted, and the effect never changes semantic status.
5. **Visual/layout gate.** Geometry, wrapping, font readiness, localization,
   bidi, contrast, and text sharpness meet the budgets for every content case
   on every engine where enhancement is enabled.
6. **Performance gate.** The candidate meets every absolute layer, memory,
   main-thread, paint, and frame budget at N=1/10/50/100 during the single
   measured sweep per epoch. It must also improve the background-position
   baseline by at least 25% in p95 main-thread style+layout+paint cost at both
   N=50 and N=100 on the low-power/mobile target, or demonstrate zero
   steady-state paint while staying within all other budgets. If the baseline
   is already within the static budget and the candidate does not produce a
   meaningful measured win, keep static text.
7. **Engine gate.** Enable the enhancement only for an engine/device path that
   passes the complete tested matrix. The recommended package default is static
   until all three named web engines have a passing path; a partial per-engine
   enhancement may be considered only with explicit capability checks and a
   documented static fallback for every other path.
8. **Claim gate.** Use “web-only”, “GPU-conscious”, or “compositor-oriented
   candidate” as appropriate to the evidence. Never publish “GPU accelerated”
   without a trace that proves the claimed engine/device pipeline.

Any hard-gate failure rejects the moving enhancement for that path. The safe
result is an ordinary static activity line, not a slower or less readable
fallback. A future implementation card may be opened only after the
orchestrator accepts the evidence and promotion map.

## Evidence Retention

The future benchmark record should retain:

- an immutable manifest containing the Poodle commit, package versions, browser
  build ids, OS/device class, viewport, DPR, refresh rate, theme, density/size,
  motion/forced-color state, candidate id, content ids, node count, epoch
  scenario, first-commit/sweep/pre-roll/post-roll durations, lifecycle event
  timeline, cancellation reason, and repetition number;
- SHA-256 digests for each raw trace, screenshot, DOM/accessibility snapshot,
  selection result, and layer/memory report;
- a compact per-cell summary with median/p95 values, pass/fail for every budget,
  engine-specific fallback decisions, and the exact reason for any rejection;
- sanitized raw traces, screenshots, action/selection traces, and accessibility
  snapshots outside canonical source through the verdict plus a 90-day audit
  window, then deletion; and
- the compact manifest, digests, threshold table, reviewer notes, and final
  verdict as the durable record in the future owning log/card.

Use synthetic strings and an isolated environment. Do not capture credentials,
customer data, ambient sessions, or private product transcripts. Full-mode
traces are performance evidence; frozen screenshots are deterministic endpoint
evidence. Neither is an active-cohort visual-parity receipt.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| AgentSubagent remains the semantic host | Apply the effect to generic `Text` or `AgentMessage` | The benchmark fixture has only AgentSubagent running activity lines; no generic export or arbitrary-text route is introduced. |
| There is one source of truth and no replay | Replace the activity string after the sweep starts, then replace it again | Source and overlay stay aligned from the one item value; semantic text updates immediately; the same epoch has one sweep with no restart or queue; copy and accessibility expose one value. |
| Motion policy is honored | Switch `full → reduced → frozen` while the line is moving | Enhancement cancels immediately, disallowed motion drops, frozen schedules no clock, and the latest semantic line remains immediate. |
| One sweep belongs to one eligible epoch | Enter `running` with a non-empty line, remain running through completion, and replace the line mid-sweep | The first committed frame starts exactly one non-looping 2.0-second sweep; it ends once, stays static, and does not replay while the epoch continues. |
| Status and eligibility own cancellation | Change `running → waiting`, empty the line, go offscreen/inactive, select fallback, or unmount during a sample | Remaining visual work cancels immediately, the source stays readable/static, and no timer/observer/rAF/animation handle or late callback remains. |
| Re-entry creates a fresh epoch | Leave `running`, then later re-enter `running` with a non-empty line | The old epoch cannot resume; the new epoch gets exactly one new sweep after its first committed frame. |
| Consumers do not control decoration | Look for pause/stop/hide props or have the effect attempt a status update | No public visual pause API exists, no consumer must mutate status to stop decoration, and status remains host-owned. |
| Reflow does not move the host | Resize, load a variable font, wrap to five lines, or replace text | Host geometry is unchanged; the overlay either realigns within tolerance or falls back to ordinary text. |
| Forced and unsupported paths stay readable | Disable mask support or enable forced colors | No mask/gradient/duplicate is used; system text remains selectable and accessible. |
| Scale stays bounded | Run N=100 at DPR 2 on the low-power device | Layer count, texture memory, main-thread work, frame p95/p99, and dropped frames stay under the adopted budgets. |
| Native boundary stays honest | Ask GPUI or Jetstream to render the web mask | Native semantics/static output remain explicit; any parity request stops at a new renderer-capability decision. |

## Stop Conditions

Stop the future benchmark or implementation lane and return to the orchestrator
when any of these occurs:

- the AgentSubagent contract remains draft/contradictory, or the host cannot
  supply one stable activity-line source and lifecycle;
- the implementation loops, starts more than one sweep in an eligible epoch,
  restarts or queues on an `activityLine` replacement, resumes an old epoch
  after cancellation, or fails to create a fresh one after leaving and
  re-entering `running`;
- the effect needs a generic Text/AgentMessage API, caller-maintained duplicate
  content, product-specific vocabulary, a new semantic status, or a second
  live-region owner;
- alignment fails for multiline, font readiness, resize, localization, bidi,
  or variable-font cases, or the real text becomes transparent/unselectable;
- any ineligible, reduced/frozen, forced-color/print, unsupported/fallback,
  inactivity/offscreen, status-transition, or unmount path does not cancel
  remaining visual work immediately and leave readable static text;
- the proposed consumer surface includes a pause/stop/hide prop, requires the
  consumer to mutate semantic status merely to stop decoration, or lets the
  effect mutate status itself;
- any hard layout, paint, layer, memory, frame, contrast, copy, focus, or
  accessibility budget fails;
- the mask path is merely property-name-supported but not trace-supported, or
  the evidence cannot identify the claimed compositor/raster behavior;
- the candidate does not produce the recommended measured win over the
  background-position baseline and static text is the better result;
- native text-mask support becomes necessary, which is a new architecture and
  renderer-capability lane, not a fallback to an unsupported GPUI claim;
- the work expands into Skeleton, Spinner, PageLoading, AgentMessage body,
  icon morphing, block sliders, Jetstream admission, a permanent harness, or a
  release/consumer change.

## Proposed Canonical Destinations

These are promotion recommendations, not writes authorized by this packet:

- reconcile and promote the narrow contract meaning in
  `docs/contracts/components/agent-subagent.md`;
- create one bounded roadmap card for the disposable web benchmark and its
  evidence receipt, without creating a generic effect API or permanent
  conformance plane;
- record the accepted benchmark verdict in the owning execution log and close
  or split the post-motion triage queue only after promotion; and
- keep any accepted web CSS/recipe implementation in the AgentSubagent-owned
  web surface, with native static behavior and the engine-specific fallback
  documented in the contract.

No canonical contract, architecture, roadmap, log, research dossier, token,
package, or implementation file is changed by this PR.

## Promotion Boundary

The orchestrator reviews this packet as planning intake. Merge does not promote
the contract, make a card ready, or authorize a benchmark implementation. The
orchestrator must reconcile the packet with current `main`, promote the
settled ownership and web-only boundary into canonical planning surfaces, and
then decide whether the numeric recommendations are adopted, revised, or
rejected. Only a later accepted benchmark can authorize a bounded web
implementation. Native semantics remain static unless a separately approved
native text-mask capability proves otherwise.
