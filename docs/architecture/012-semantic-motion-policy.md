# 012 Semantic Motion Policy

Status: active
Accepted: 2026-09-01
Owner: Poodle core
Depends on: `001-poodle-system-shape.md`,
`010-native-presentation-construction-context.md`,
`../contracts/001-working-rules.md`,
`../contracts/components/motion-policy-provider.md`
Decision source: planning intake PR #121, merged as `7f718dd42`

## Decision

Poodle owns one explicit effective motion policy across web core, Svelte,
React, shared Rust composition, and GPUI:

```text
MotionPolicy = full | reduced | frozen
```

Platform integration resolves the user's system preference at the host edge.
Poodle consumes the effective value; components do not perform ambient media,
OS, or backend preference discovery. A missing preference resolves to `full`.
Capture and test hosts select `frozen` explicitly.

The policy is host-level and restriction-only. A descendant may reduce or
freeze motion but cannot re-enable motion suppressed above it:

```text
full < reduced < frozen
effective = max(ancestor, child)
```

Components may choose a stricter static treatment in any mode. They do not get
a general override that forces motion above the effective host policy.

## Runtime Boundary

Web exposes a dedicated `MotionPolicyProvider` beside
`UiPresentationProvider`. The provider supplies the effective policy through
Svelte and React context and a stable inherited style hook for shared web
styles. Presentation-only scopes do not reset motion.

Shared Rust carries the same effective value in `RenderContext`, beside theme,
size scale, and density. A native motion provider is a construction boundary:
it derives a restricted child context and returns the built child unchanged.
No provider metadata is added to resolved `Node` trees.

```text
host preference / capture policy
              |
        effective policy
          /           \
 web context/CSS     RenderContext
   /       \              |
Svelte    React       poodle-render -> Node -> GPUI
```

## Mode Laws

Semantic and accessibility state always updates immediately. Motion never owns
ARIA, labels, status text, progress meaning, focus, correctness, or semantic
timers.

- `full` permits the role's accepted one-shot or looping visual treatment.
- `reduced` stops continuous loops and removes translation, scale, rotation,
  blur, bounce, and layout travel. A short non-looping opacity transition is
  allowed only where it preserves continuity without delaying presence,
  removal, focus, or announcement.
- `frozen` schedules no visual clock. One-shot roles paint the latest settled
  endpoint; looping roles paint a named readable static frame.

Toast expiry, loading state, and other product lifecycle remain with their
existing semantic owners. A capture harness may control time separately from
the frozen visual policy.

## Identity And Interruption

A motion instance is keyed by semantic owner identity plus role and channel.
Tree position, render order, and a global recipe name are not identity. Keys
survive immediate-mode rebuilds while their semantic owner survives; siblings
must not share a clock accidentally.

Visual phases follow latest-state-wins:

- repeated activation of the current target is inert and does not restart;
- a reversible binary phase continues from its sampled progress toward the new
  endpoint, with duration proportional to the remaining progress;
- a multi-target update retargets from current rendered geometry to the latest
  measured target; phases do not queue;
- an exit remnant may remain as inert paint only, but leaves accessibility,
  focus order, hit testing, and live-region ownership immediately;
- abort, owner unmount, or a switch to `frozen` cancels every clock and cleanup
  handle;
- abort and `frozen` settle the latest semantic endpoint; owner unmount removes
  the remnant;
- tightening from `full` to `reduced` drops disallowed properties immediately
  and may finish only an allowed opacity phase; and
- visual completion never fires semantic callbacks.

Authored initial state paints its endpoint. It does not animate from a default
or placeholder the host never authored. Full-mode loading loops may begin
after the first committed frame.

## Property Budget And Native Approximation

The default renderer-neutral property budget is opacity, translation, scale,
and rotation. Layout, blur, path drawing, gradients, filters, canvas, and 3D
effects need a separate role-specific decision and a static or reduced
fallback.

GPUI 0.2.2 realizes a smaller subset: generic opacity and SVG rotation.
Translation and scale may use a named bounded opacity approximation, or the
static endpoint when opacity would mislead. Every approximation is recorded by
role and remains an active-cohort visual capability gap. It is never reported
as exact visual parity.

Disclosure height interpolation is the first pilot's one named layout-motion
exception. Full web mode may animate the clipped block axis; reduced and
frozen paint final layout immediately. Native uses the named opacity/static
approximation until a bounded layout capability exists.

## First Pilot Roles

| Family | Full | Reduced | Frozen |
| --- | --- | --- | --- |
| Accordion / Collapsible | clipped-height reveal and indicator rotation | final layout and paint immediately | final layout; no clock |
| ToastStack | keyed opacity/translation enter and exit | short opacity enter and exit | latest endpoint; no clock |
| Tabs underline | measured moving underline on semantic selection | measured endpoint immediately | measured endpoint; no clock |
| Checkbox / same-slot IconButton state | bounded state crossfade or transform within the property budget | short opacity crossfade | latest endpoint; no clock |
| Skeleton / Spinner | Skeleton 1.6s opacity pulse; Spinner rotation or opacity phasing | readable static frame | canonical static frame; no clock |

The moving Tabs indicator applies only to `activeEdge="underline"`. It is one
paint-only child of the tablist measured from the selected tab. First layout,
orientation changes, resize, font reflow, and overflow-mode changes snap to the
remeasured endpoint. Only semantic selection against stable geometry animates.

Skeleton's full-mode loop is normalized to a 1.6s opacity pulse. Spinner keeps
ring rotation and dot/grid opacity variants. `Skeleton.animated=false` is
static in every policy. Loading-to-content reveal remains host-owned and may
use one allowed opacity replacement.

## Semantic Boundaries

- Toasts enter their live region immediately and do not reannounce on reorder,
  retarget, policy change, or visual completion.
- A dismissed toast leaves live-region and accessibility ownership
  immediately. Any visual remnant is `aria-hidden`, inert, unfocusable, and
  excluded from hit testing.
- When a dismissed toast owns focus, ToastStack moves focus synchronously to
  the equivalent control on the next item, then previous item, then the still-
  connected element from which focus entered the stack. If none exists,
  ordinary host focus order resumes.
- Danger-toast assertive posture is mode-independent.
- Skeleton remains decorative. Spinner's optional status role and label remain
  present when its animation stops.
- Motion never steals focus or delays keyboard behavior.

## Evidence Boundary

The policy is proved in layers rather than by a full family × mode × runtime
Cartesian board:

1. Paired framework-free TypeScript and Rust traces prove defaults,
   restriction-only nesting, identity, initial endpoints, interruption,
   reduced-property filtering, frozen clocks, and cleanup.
2. Svelte and React mount the same family traces through real components.
3. Rust render assertions prove policy propagation, stable keys, declared
   properties, endpoints, loops, and frozen removal.
4. Headless GPUI probes prove supported channels and name every approximation.
5. A small browser probe owns measured disclosure, Tabs geometry, toast focus,
   and live-region claims.
6. Deterministic captures compare only static endpoints and canonical frozen
   frames. They are not reduced-motion or lifecycle evidence.

This does not create a portable case corpus, generated component interface,
exhaustive specimen surface, or permanent conformance authority.

## Ownership

- Host integrations own system-preference discovery and explicit capture mode.
- `poodle-core` owns the framework-free policy laws and web context helpers.
- Svelte and React own thin provider shells and component projection.
- `poodle-render::RenderContext` owns native construction-time propagation.
- `poodle-node` owns resolved animation declarations, not policy discovery.
- GPUI owns supported channels and explicit approximations.
- Components own semantic state, timers, focus, announcements, and accepted
  role-specific visuals.
- Jetstream remains deferred.

## Rejected Alternatives

- Extending `UiPresentationProvider`: rejected because presentation scopes may
  replace size and density, while motion is a restriction that must survive
  those scopes.
- Ambient runtime discovery: rejected because it cannot give web orchestration,
  Rust composition, GPUI, and deterministic capture one explicit input.
- Nearest-provider-wins: rejected because a subtree could re-enable suppressed
  motion.
- Root-only policy: rejected because bounded stricter and frozen scopes are
  useful.
- Per-component reduced rules: rejected because they weaken host
  predictability and cross-runtime evidence.
- Exact GPUI property parity as a prerequisite: rejected because it would turn
  the policy pilot into a renderer-capability programme. Gaps stay explicit.

## Migration Boundary

The provider, `RenderContext` field, paired laws, and five-family component
changes land atomically in the first implementation card. Before v1.0 there is
no compatibility alias, silent default shim, or second motion-policy path.
Existing hosts without a preference signal retain `full`; capture hosts must
select `frozen` explicitly.
