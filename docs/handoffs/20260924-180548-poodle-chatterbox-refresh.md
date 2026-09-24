---
title: Poodle Chatterbox continuation
kind: northstar-handoff
status: active
owner: Tom
created: 2026-09-24
updated: 2026-09-24
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260924-180548-poodle-chatterbox-refresh.md
handoff_mode: chatterbox-continuation
chatterbox_mode: conversational-planning
dispatch_authority: chatterbox
tags: [coordination, handoff, chatterbox, planning]
---

## What This Thread Was Doing

This Chatterbox carried Poodle through the end of the web UX acceptance sweep,
the compatible repairs it exposed, and the `0.4.2` npm patch release. It also
handled the producer-side Tabs bridge regression that blocked Longhorn, resumed
that retained consumer lane against the corrected package, and left Poodle's
product runway empty for a fresh planning choice. Since then, g18.039 migrated
the Queue manifest to prospective-merge hooks and closed normally.

Tom asked for this live Chatterbox to move into a fresh Opus 5.5 medium thread
with a thorough continuation brief. This is an ownership transfer, not a new
project refresh, implementation task, or authorization to invent the next
roadmap item.

## Why It Matters

Poodle is the shared component authority for several consumer repositories.
The release and immediate repair work are settled, but g18 remains open with a
large GPUI capability horizon and several deliberately held design or platform
questions. The next Chatterbox should preserve that distinction: current
maintenance must stay truthful while the next product tranche is chosen with
Tom rather than inferred from old backlog volume.

## Current State

- **Source Chatterbox:** `64760e32-6664-4191-a434-66349bd118ef`.
- **Workspace:** `wks_a43cedb45e82cb05`, local checkout
  `/Users/tom/Dev/projects/poodle`. The successor must use this exact workspace;
  do not create a worktree or another workspace.
- **Canonical head at refresh preparation:**
  `622d456fd6a94f115fda79e695fd374df762422e`, equal to `origin/main` before
  this handoff commit. It promotes the bounded g18.040 lifecycle-currentness
  repair. The preceding `304554839` is g18.039's lifecycle terminal record;
  PR #275 merged g18.039 as `c1ee8fdf7` after the v4 Queue-manifest migration.
- **Released baseline:** public npm `@inflatable-cookie/poodle-core@0.4.2` and
  `@inflatable-cookie/poodle-svelte@0.4.2`; annotated `v0.4.2` dereferences to
  `d2438aef7d34df31b958d172c9c162e83063d83c`. The fresh-registry DockRegion
  bridge proof passed 4/4. Release closeout is
  `/Users/tom/Dev/projects/poodle/docs/logs/2026-09/20260914-g18-038-v042-release.md`.
- **Generation state:** g18 is open and `planning_required`. The last canonical
  product repair is g18.038; g18.039 is complete maintenance. g18.040 is a
  ready, documentation-only currentness repair and is not a product successor.
- **Queue attention preflight:** plan
  `5b0724a541bf35e04f69a54cc42095cc53a0d444e910b96e0128ab5f0c13035b`,
  source `64760e32-6664-4191-a434-66349bd118ef`, workspace
  `wks_a43cedb45e82cb05`, exact unfinished task set `[]`. An empty transfer is
  expected and must still be applied and verified after successor creation.
- **Open triage:** six current notes under
  `/Users/tom/Dev/projects/poodle/docs/triage/`: repository settings/web-pair
  extraction; History CS20 and keyboard geometry; citations and nested-menu
  prerequisites; Jetstream admission; three single-consumer Tabs asks; and
  `gpui-unofficial` build/live-tree gates. Treat the notes as unresolved intake,
  not dispatch authority.
- **Active maintenance owned elsewhere:** g18.040 was promoted and dispatched
  while this handoff was being prepared. Queue task
  `345a792c-266c-491a-ad55-d0a303b0bad6` is `working` in workspace
  `wks_7fbfd23085664520` with worker
  `0b3ac785-834c-47b5-999c-4d27ff117b25`. Its origin is
  `d24445ee-22ae-4ea3-a603-abe263ac188f`; it is not routed to this Chatterbox
  and is not part of the transfer set. Do not supervise, duplicate, or edit its
  owned paths.
- **Live conversational state:** there is no unanswered product question and no
  unconsumed general execution approval. Tom's current request authorizes only
  this refresh and successor launch. Earlier release/repair approvals have been
  consumed.
- **Requested successor runtime:** Claude Opus 5.5,
  `claude/claude-opus-5-5`, medium thinking, full-access
  `bypassPermissions`, no fast-mode override.

## Boundaries

- **In scope:** continue as Poodle's operator-facing Chatterbox; inspect current
  authority, reconcile open triage and concurrent planning, explore the next
  bounded g18 direction with Tom, and promote planning only after explicit
  confirmation.
- **Out of scope:** do not act as coordinator, supervise workers, review or
  merge PRs, publish releases, edit consumer repositories, or turn a held
  horizon into executable work without the missing operator decision and
  readiness evidence.
- Preserve Svelte/React semantic parity, shared core ownership, native parity
  contracts, published-package boundaries, and the pre-v1 ban on compatibility
  shims and silent fallbacks.
- Never run local `*-windowed` conformance selectors without explicit operator
  approval. Do not reopen the release-process lane merely because historical
  `0.4.2` publication visibility timed out after successful publication; that
  papercut is already recorded.
- Follow `/Users/tom/Dev/projects/poodle/AGENTS.md`, the repository docs spine,
  and the installed Northstar Chatterbox mode. Preserve unrelated dirty state
  in the shared checkout.

## Important Context

- **Canonical reading order:** start with
  `/Users/tom/Dev/projects/poodle/docs/README.md`,
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/generation-index.md`,
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g18/README.md`, and
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/dispatch.md`. Reload them from
  current `main`; this handoff is context, not planning authority.
- **System boundary:** Poodle owns generalized tokens, primitives and reusable
  composites. Application-specific widgets and translations stay in consumers.
  Underlay applications import published Poodle packages directly; Poodle does
  not carry consumer-named adapters.
- **Accepted web direction:** Slider/RangeSlider share private primitives but
  remain distinct public controls; block is the default presentation. Rich text
  uses TipTap JSON over the ProseMirror document schema. Renderer/editor size
  follows `UiPresentation`. Tooltip focus-open is keyboard-modality only.
- **Recent compatible fixes:** `role="slider"` is an ancestor-drag boundary;
  MenuSurface rows expose exact `item.label` accessible names; Svelte Tabs
  forwards `crossWindowSourceBridge` into TabsItem. These shipped in `0.4.2`.
- **Release discipline:** use Effigy selectors and bounded evidence. Do not
  stack broad boards. A frozen npm candidate gets one artifact gate; registry
  delay after immutable publication is not permission to republish.
- **Current product horizon:** g18's completion rule is capability-level
  disposition for every portable GPUI row. Future functional repair tranches,
  GPUI visual expansion and keyboard-origin focus remain planning horizons.
  A2 platform accessibility remains held on a buildable `gpui-apple` and live
  non-activating tree proof. Jetstream remains separately held.
- **Current maintenance seam:** g18.039's terminal lifecycle projection is
  canonical, but some narrative surfaces still say `ready maintenance` or omit
  its completion. g18.040 owns the bounded audited-currentness repair. Let its
  existing Queue lane finish; a later administrative notice may update the
  runway, but this successor does not coordinate the task.
- **Open choice:** after maintenance currentness is reconciled, Tom has not yet
  selected between shaping the first remaining GPUI functional repair tranche,
  revisiting one of the explicit triage gates, or another bounded Poodle-owned
  consumer finding. Ask rather than infer.

## Suggested Next Move

Remain read-only until the source sends `Ownership transfer complete`. Then
reload `main` and confirm the exact shared-checkout state. Treat g18.040 as an
active Queue lane owned elsewhere; do not edit, dispatch, or supervise it.

Once repository ownership is clear, give Tom a short state recap and ask one
useful planning question: whether he wants to choose the next Poodle product
frontier while g18.040 runs, or wait for its maintenance closeout first. If he
chooses product planning, offer the smallest credible options from current
authority rather than a generic backlog dump.

## Completion Protocol

This is a Chatterbox continuation. Until the explicit follow-up arrives, stay
read-only: do not promote planning, make direct changes, rule on Queue
escalations, or send coordinator direction. The source retains ownership until
same-workspace creation, exact empty-set Queue transfer, and the completion
message are verified.

After `Ownership transfer complete`, take over normal Chatterbox authority in
this workspace. Keep triage mutable, require Tom's confirmation before
canonical promotion, commit and push planning only on the integration branch,
and use Queue/coordinator channels only within their documented authority.
There is no active Queue attention route to inherit at handoff time. The next
durable Chatterbox outcome should be an operator-selected, readiness-complete
next g18 product task or a deliberate pause—not implementation started from
this handoff alone. g18.040 remains with its existing origin and Queue lane.
