---
title: Poodle orchestrator handoff
kind: northstar-handoff
status: active
owner: Poodle core
created: 2026-08-31
updated: 2026-08-31
handoff_path: /Users/tom/Dev/projects/poodle/docs/handoffs/20260831-141135-poodle-orchestrator.md
tags: [coordination, handoff, orchestrator, g16]
---

## What This Thread Was Doing

This thread has been Poodle's long-running Northstar orchestrator. It took the
project from the failed g13/g14 consolidated-conformance experiments through a
release-first g15, the corrected v0.2.2 release and consumer adoption, and then
into g16's evidence-led parity repairs.

The current branch of that story is dependable drag-and-drop. The semantic
kernel, web custom-surface controller, EditableList migration, Tree migration,
and Rust/GPUI substrate are now merged. PR #108 was the last piece: it needed
four serious review rounds before its GPUI lifecycle and evidence were sound.
It merged on 2026-08-31 as `7a39f3c6d143784838e3e5cae4f05d9331c08f85`.

## Why It Matters

Poodle exists to stop component behavior drifting between Svelte, React, and
GPUI while keeping one web substrate and one Rust substrate. Drag-and-drop is a
high-risk version of that problem: apps repeatedly get cleanup, touch,
keyboard, nested targets, cross-window transfer, and drag-out subtly wrong.
The programme is building one reusable lifecycle without reviving the failed
component IR or specimen-snapshot approaches.

The broader release goal remains a complete, dependable Svelte reference with
paired React and GPUI behavior. Jetstream stays deferred until its backend is
admitted deliberately.

## Current State

Here is the short version of where things stand:

- **Done:** g16.021–g16.025 are merged. PR #108 closed the renderer-neutral
  Node registrations, shared Rust projection, public GPUI controller, mounted
  native evidence, and stock-GPUI capability matrix. The parity ledger remains
  52 mounted / 122 missing.
- **Still open:** g16.026 must define the exact paired TypeScript/Rust
  cross-window host-bridge API and a window-owned GPUI provider-unmount seam
  before it is safe to dispatch.
- **Active spec lane:**
  `/Users/tom/Dev/projects/poodle/docs/specs/069-dependable-drag-and-drop-substrate.md`.
- **Current batch card:**
  `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/026-drag-drop-cross-window-bridge-and-dock-region.md`.
  It is planned, not worker-ready yet.
- **Canonical refs:**
  `/Users/tom/Dev/projects/poodle/docs/architecture/011-drag-and-drop-substrate.md`,
  `/Users/tom/Dev/projects/poodle/docs/contracts/001-working-rules.md`, and the
  component contracts named by g16.026.
- **Remaining continuation envelope:** g16.026, then g16.027, then g16.028,
  serially. Do not start a later drag card before its dependency lands.
- **Lane budget / pause signal:** there is no second implementation worker
  ready from the component-continuation lane. EditableLabel remains
  decision-blocked; accessibility, visual comparison, motion learning,
  Longhorn conformance lab, and Jetstream admission are programme choices, not
  background work to dispatch casually.
- **Key files:**
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/README.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/generation-index.md`
  - `/Users/tom/Dev/projects/poodle/docs/logs/2026-08/20260831-g16-025-drag-drop-rust-gpui-substrate.md`
  - `/Users/tom/Dev/projects/poodle/docs/triage/20260828-221415-drag-drop-public-migration-boundary.md`
  - `/Users/tom/Dev/projects/poodle/docs/triage/20260830-180816-tabs-drag-host-bridge-sequencing.md`
  - `/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/component-continuation-runway.md`

## Boundaries

Please keep the next pass within these boundaries:

- **In scope:** maintain the g16 runway on `main`, finish the g16.026 public
  API/readiness decision, dispatch one bounded worker handoff, review its PR on
  GitHub, and merge only after the operator explicitly says to merge.
- **Out of scope:** Poodle must not import Longhorn, Loophole, Electron, Tauri,
  shell IPC, filesystem paths, window topology, or durable layout authority.
  Do not start file drag-out from g16.027 early. Do not reopen the component IR,
  machine-pinning, or specimen-snapshot conformance experiments.
- **Native boundary:** stay on crates.io GPUI 0.2.2. Do not add a GPUI fork or
  an OS input backend. Mouse and keyboard plus the in-window capture-equivalent
  route are certified; pen, touch, and device-originated cancel remain honest
  unsupported debt.
- **Migration boundary:** Tabs and DockRegion move together. Delete the old
  DOM-shaped Tabs reorder helpers and DockRegion external-drag/global-session
  surface only after the replacement passes. Do not add aliases, shims, dual
  controllers, or silent fallbacks.
- **Validation boundary:** use headless Effigy selectors. Never run a
  `*-windowed`, native-visual, focus-taking conformance, or Jetstream preview/QA
  selector without explicit operator approval.
- **Repo constraints:** follow
  `/Users/tom/Dev/projects/poodle/AGENTS.md` and the canonical
  architecture/contracts named above. The orchestrator owns the primary
  `main` checkout; workers use separate non-`main` worktrees and never merge.

## Important Context

- **Planning lineage:** g13's Rust-authored component IR and g14's executable
  conformance plane were rejected after cost and coverage evidence. g15 made a
  practical v0.2.x release from contract, runtime, test, preview, and consumer
  evidence. g16 repairs named semantic and mounted gaps from one 175-component
  ledger rather than inventing another authority.
- **How the plan fits the system:** architecture 011 owns the stable
  drag-and-drop boundary; spec 069 owns the active normative shape; g16.021–028
  own delivery. Svelte remains the reference. Svelte/React share core behavior
  and styles; Rust runtimes share `poodle-render` and `poodle-node`; GPUI is the
  active native backend; Jetstream consumes renderer-neutral maintenance only.
- **Decisions and preferences:** specimen Examples pages are human-facing and
  curated, never exhaustive conformance matrices. Cross-window transfer is
  capability-based: the host owns leases, geometry, authorization, mutation,
  rollback, and recovery; Poodle carries opaque identity and presentation.
  Cross-window and drag-out were required from the start, as was touch on web.
- **PR loop:** workers push PRs and report through the operator. Review
  independently, post every blocking finding and the final verdict on the PR
  before summarizing in chat, and do not confuse a worker's green report with
  independent review. PR #108 demonstrated why this matters.
- **Open tension:** the attempted g16.025 provider-unmount fix was correctly
  reverted. Its registry was thread-global, so one window could falsely cancel
  another window's controller; it also did not prove GPUI's own active drag and
  preview stopped. g16.026 must prove both. A preview-only root hook is not an
  integration contract; every GPUI consumer must be able to wire the result.
- **Other deferred ideas:** motion learning is recorded at
  `/Users/tom/Dev/projects/poodle/docs/triage/20260820-205249-transitions-dev-motion-learning.md`;
  the Longhorn-controlled conformance lab is recorded at
  `/Users/tom/Dev/projects/poodle/docs/triage/20260821-165500-longhorn-conformance-lab.md`.
  Neither is currently promoted for execution.
- **Repo health:** `effigy qa` passed for PR #108. `effigy doctor` is not a
  green entry gate today: it reports existing generated-in-source, god-file,
  and stale-suppression scan debt under
  `/Users/tom/Dev/projects/poodle/.effigy/reports/doctor/`. Do not misattribute
  those structural findings to the next card.

## Suggested Next Move

Start by checking `main` and reading g16.026 beside the landed public web and
GPUI controllers. Map the smallest paired host-bridge vocabulary that preserves
spec 069's prepare → arm → start → terminal ordering and carries only opaque
host authority. In the same planning pass, settle how a controller is owned by
one GPUI window so provider unmount cancels the right session and stops GPUI's
native drag without a thread-global sweep.

Update g16.026 with the exact names, ownership boundary, adversarial tests, and
review oracle before calling it ready. If that exposes a meaningful public API
fork rather than a naming/detail choice, talk it through with the operator
instead of choosing quietly. Once the card is genuinely ready, create and push
one Northstar worker handoff under
`/Users/tom/Dev/projects/poodle/docs/handoffs/` and give the operator its
absolute path.

## Completion Protocol

Keep using the established orchestrator loop:

1. Keep the primary checkout on clean, current `main`; planning and runway
   maintenance happen there.
2. Make a roadmap worker-ready before dispatch. The worker handoff is an
   overlay, not a second specification, and its frontmatter must activate the
   Northstar worker/PR loop.
3. Commit and push each worker handoff to `main`, then give the operator only
   its absolute path. The worker verifies the committed handoff from its own
   `HEAD`, works in a dedicated non-`main` worktree, pushes a PR, and never
   merges.
4. Review the PR independently against the card's review oracle and focused
   adversarial cases. Put the verdict and every requested change on GitHub
   before the chat summary.
5. Merge only after explicit operator authorization. Then reconcile `main`,
   close the card/log/front doors honestly, and promote only the next landed
   dependency.
6. For the current lane, g16.026 is next; g16.027 and g16.028 are the remaining
   serial envelope. EditableLabel and the other programme choices remain
   paused unless the operator deliberately switches lanes.

If g16.026 cannot preserve the opaque host boundary, needs a GPUI fork, needs
focus-taking automation, or requires sibling mutation, stop and bring the
decision back. Do not make the card look ready by weakening its proof.
